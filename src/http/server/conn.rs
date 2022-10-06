//! HTTP connection handling

use crate::http::server::{respond, Handler, HttpRequest, HttpSettings, ResponseData, Stream};
use crate::http::{name, version};
use crate::{Fail, Result};

use rustls::{ServerConfig, ServerConnection, Stream as RustlsStream};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;
use std::{
    io::prelude::{Read, Write},
    net::SocketAddr,
};

/// Accept connections
pub fn accept_connections<T: Send + Sync + 'static>(
    listener: Arc<RwLock<TcpListener>>,
    http_settings: Arc<HttpSettings>,
    tls_config: Arc<ServerConfig>,
    handler: Handler<T>,
    shared: Arc<RwLock<T>>,
) {
    loop {
        // accept connection
        if let Ok((stream, address)) = listener.read().unwrap().accept() {
            // clones
            let http_settings = http_settings.clone();
            let tls_config = tls_config.clone();
            let shared = shared.clone();

            // spawn new thread
            thread::spawn(move || {
                // handle connection
                handle_connection(stream, address, &http_settings, tls_config, handler, shared)
                    .ok();
            });
        }
    }
}

/// Handle connection
pub fn handle_connection<T: Send + Sync + 'static>(
    mut stream: TcpStream,
    address: SocketAddr,
    http_settings: &HttpSettings,
    tls_config: Arc<ServerConfig>,
    handler: Handler<T>,
    shared: Arc<RwLock<T>>,
) -> Result<()> {
    // set timeouts
    stream.set_read_timeout(http_settings.read_timeout)?;
    stream.set_write_timeout(http_settings.write_timeout)?;

    // create TLS connection
    let mut session = ServerConnection::new(tls_config)
        .or_else(|_| Fail::from("could not initialize server connection"))?;
    let mut stream = RustlsStream::new(&mut session, &mut stream);

    // read header
    let response = match read_header(&mut stream, http_settings) {
        Ok((header, rest)) => {
            // parse HTTP request and process
            let http_request =
                HttpRequest::from(&header, rest, &mut stream, address, http_settings);
            match handler(http_request, shared) {
                Ok(response) => response,
                Err(err) => respond(
                    format!("<!DOCTYPE html><html><head><title>{0}</title></head><body><h3>HTTP server error</h3><p>{0}</p><hr><address>{1} v{2}</address></body></html>", err, name(), version()),
                    "text/html",
                    Some(ResponseData::new().set_status("400 Bad Request"))),
            }
        }
        Err(err) => {
            if err.to_string() == "received corrupt message" {
                return Fail::from("Not a TLS connection");
            }
            respond(
            format!("<!DOCTYPE html><html><head><title>{0}</title></head><body><h3>HTTP server error</h3><p>{0}</p><hr><address>{1} v{2}</address></body></html>", err, name(), version()),
            "text/html",
            Some(ResponseData::new().set_status("400 Bad Request")),
        )
        }
    };

    // respond
    stream.write_all(&response)?;
    stream.flush()?;

    // done
    Ok(())
}

/// Read until \r\n\r\n
fn read_header(stream: &mut Stream, http_settings: &HttpSettings) -> Result<(String, Vec<u8>)> {
    // initialize vectors
    let mut header = Vec::new();
    let mut rest = Vec::new();
    let mut buf = vec![0u8; http_settings.header_buffer];

    // read continously
    let mut read_fails = 0;
    'l: loop {
        // read from stream and check max header size
        let length = stream.read(&mut buf)?;
        if header.len() + length > http_settings.max_header_size {
            return Fail::from("Max header size exceeded");
        }

        // only use actually read data
        let buf = &buf[0..length];

        // iterate through bytes
        'f: for (i, &b) in buf.iter().enumerate() {
            // check if byte is \r
            if b == b'\r' {
                // check if necessary to read 3 more bytes
                if buf.len() < i + 4 {
                    // read 3 more bytes
                    let mut buf_temp = vec![0u8; i + 4 - buf.len()];
                    stream.read_exact(&mut buf_temp)?;

                    // combine buffers and compare bytes
                    let mut buf2 = [buf, &buf_temp].concat();
                    let header_end =
                        buf2[i + 1] == b'\n' && buf2[i + 2] == b'\r' && buf2[i + 3] == b'\n';

                    // add buffer to header and check if header end reached
                    header.append(&mut buf2);
                    if header_end {
                        // header end reached
                        break 'l;
                    } else {
                        // not yet, read more
                        break 'f;
                    }
                // can read 3 more bytes, so compare
                } else if buf[i + 1] == b'\n' && buf[i + 2] == b'\r' && buf[i + 3] == b'\n' {
                    // split into header and rest
                    let (split1, split2) = buf.split_at(i + 4);
                    header.extend_from_slice(split1);
                    rest.extend_from_slice(split2);

                    // header end reached
                    break 'l;
                }
            }

            // last byte reached, but end not reached yet
            if buf.len() == i + 1 {
                // add buffer to header
                header.extend_from_slice(buf);
            }
        }

        // check if didn't read fully
        if length < http_settings.header_buffer {
            read_fails += 1;

            // failed too often
            if read_fails > http_settings.header_read_attempts {
                return Fail::from("Read header failed too often");
            }
        }
    }

    // return header as string and rest
    Ok((
        match String::from_utf8(header) {
            Ok(header) => header,
            Err(err) => return Fail::from(err),
        },
        rest,
    ))
}
