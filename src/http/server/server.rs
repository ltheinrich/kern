#[cfg(not(feature = "tls"))]
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{spawn, JoinHandle};

#[cfg(feature = "tls")]
use rustls::{ServerConfig, ServerConnection, Stream as RustlsStream};

use crate::http::ReadWrite;
use crate::{Fail, Result};

use super::{ErrorHandler, Handler, HttpRequest, HttpSettings};

/// Processes incoming HTTP connections
#[derive(Debug)]
pub struct HttpServer<S: Send + Sync + 'static> {
    listener: Arc<RwLock<TcpListener>>,
    settings: Arc<HttpSettings>,
    handler: Handler<S>,
    error_handler: ErrorHandler<S>,
    shared: Arc<RwLock<S>>,
    threads: RwLock<Vec<JoinHandle<()>>>,
    #[cfg(feature = "tls")]
    tls_config: Option<Arc<ServerConfig>>,
}

impl<S: Send + Sync + 'static> HttpServer<S> {
    /// Create new HttpServer and listen
    pub fn new(
        addr: String,
        settings: Arc<HttpSettings>,
        handler: Handler<S>,
        error_handler: ErrorHandler<S>,
        shared: Arc<RwLock<S>>,
        threads: usize,
        #[cfg(feature = "tls")] tls_config: Option<Arc<ServerConfig>>,
    ) -> Result<Arc<Self>> {
        let listener = TcpListener::bind(addr)?;
        let server = Self {
            listener: Arc::new(RwLock::new(listener)),
            settings,
            handler,
            error_handler,
            shared,
            threads: RwLock::default(),
            #[cfg(feature = "tls")]
            tls_config,
        };
        let server = Arc::new(server);

        (0..threads).for_each(|_| {
            let server_clone = server.clone();
            server
                .threads_mut()
                .unwrap()
                .push(spawn(move || accept(server_clone)));
        });
        Ok(server)
    }

    /// Get HttpSettings
    pub fn settings(&self) -> &HttpSettings {
        &self.settings
    }

    #[cfg(feature = "tls")]
    /// Get TLS configuration
    pub fn tls_config(&self) -> Option<&ServerConfig> {
        match &self.tls_config {
            Some(tls_config) => Some(tls_config),
            None => None,
        }
    }

    /// Read access to shared
    pub fn shared(&self) -> Result<RwLockReadGuard<S>> {
        self.shared.read().or_else(Fail::from)
    }

    /// Write access to shared
    pub fn shared_mut(&self) -> Result<RwLockWriteGuard<S>> {
        self.shared.write().or_else(Fail::from)
    }

    /// Read access to threads
    pub fn threads(&self) -> Result<RwLockReadGuard<Vec<JoinHandle<()>>>> {
        self.threads.read().or_else(Fail::from)
    }

    /// Write access to shared
    pub fn threads_mut(&self) -> Result<RwLockWriteGuard<Vec<JoinHandle<()>>>> {
        self.threads.write().or_else(Fail::from)
    }

    /// Block on join of a Thread's JoinHandle
    pub fn block(&self) -> Result<()> {
        while let Some(thread) = self.threads_mut()?.pop() {
            thread
                .join()
                .or_else(|_| Fail::from("listener thread crashed"))?;
        }
        Ok(())
    }
}

/// Reads header and create HttpRequest to pass to Handler
fn process_request<S: Send + Sync + 'static>(
    stream: &mut impl ReadWrite,
    address: SocketAddr,
    settings: &HttpSettings,
    shared: Arc<RwLock<S>>,
    handler: Handler<S>,
) -> Result<Vec<u8>> {
    let (raw_header, partial_body) = read_header(stream, settings)?;
    let request = HttpRequest::from(&raw_header, partial_body, stream, address, settings)?;
    handler(request, shared)
}

/// Accept connections
fn accept<S: Send + Sync + 'static>(server: Arc<HttpServer<S>>) {
    loop {
        // accept connection
        if let Ok((mut stream, address)) = server.listener.read().unwrap().accept() {
            // clones
            let server = server.clone();

            // spawn new thread
            spawn(move || {
                // set timeouts
                stream
                    .set_read_timeout(server.settings.read_timeout)
                    .unwrap();
                stream
                    .set_write_timeout(server.settings.write_timeout)
                    .unwrap();

                // create TLS connection
                #[cfg(feature = "tls")]
                let mut session;
                #[cfg(feature = "tls")]
                let mut stream: Box<dyn ReadWrite> = match server.tls_config.clone() {
                    Some(tls_config) => {
                        session = ServerConnection::new(tls_config)
                            .or_else(|_| Fail::from("could not initialize server connection"))
                            .unwrap();
                        Box::new(RustlsStream::new(&mut session, &mut stream))
                    }
                    None => Box::new(stream),
                };

                // process request
                let response = match process_request(
                    &mut stream,
                    address,
                    &server.settings,
                    server.shared.clone(),
                    server.handler,
                ) {
                    Ok(response) => response,
                    Err(err) => (server.error_handler)(err, server.shared.clone()),
                };

                // respond
                stream.write_all(&response).unwrap();
                stream.flush().unwrap();
            });
        }
    }
}

/// Read until \r\n\r\n
fn read_header(
    stream: &mut impl ReadWrite,
    http_settings: &HttpSettings,
) -> Result<(String, Vec<u8>)> {
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
