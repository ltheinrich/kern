use core::str;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

use crate::http::common::{HttpMethod, ReadWrite};
use crate::{Fail, Result};

use super::url::{url_encode, Url};

#[cfg(feature = "tls")]
use {
    rustls::{ClientConfig, ClientConnection, StreamOwned},
    std::sync::Arc,
};

pub fn send_content_length(stream: &mut impl Write, body: &[u8]) -> Result<()> {
    send_header(stream, "Content-Length", &body.len().to_string()).or_else(Fail::from)
}

pub fn end_headers(stream: &mut impl Write) -> Result<()> {
    stream.write_all("\r\n".as_bytes()).or_else(Fail::from)
}

pub fn send_main_header(
    stream: &mut impl Write,
    method: HttpMethod,
    url: &Url,
    query: &HashMap<String, String>,
) -> Result<()> {
    stream.write_all(method.as_str().as_bytes())?;
    stream.write_all(" /".as_bytes())?;
    stream.write_all(url_encode(url.path, true).as_bytes())?;

    for (i, (name, value)) in query.iter().enumerate() {
        stream.write_all(&[if i == 0 { b'?' } else { b'&' }])?;
        stream.write_all(url_encode(name, false).as_bytes())?;
        stream.write_all(b"=")?;
        stream.write_all(url_encode(value, false).as_bytes())?;
    }

    stream.write_all(" HTTP/1.1\r\nHost: ".as_bytes())?;
    stream.write_all(url.server_name.as_bytes())?;
    stream.write_all("\r\n".as_bytes())?;
    Ok(())
}

pub fn send_header(stream: &mut impl Write, name: &str, value: &str) -> Result<()> {
    stream.write_all(name.as_bytes())?;
    stream.write_all(": ".as_bytes())?;
    stream.write_all(value.as_bytes())?;
    stream.write_all("\r\n".as_bytes()).or_else(Fail::from)
}

pub fn connect(
    #[cfg(feature = "tls")] config: Arc<ClientConfig>,
    url: &Url,
) -> Result<Box<dyn ReadWrite>> {
    let stream = TcpStream::connect(&url.addr)?;

    let stream: Box<dyn ReadWrite> = if url.secure {
        #[cfg(not(feature = "tls"))]
        return Fail::from("tls feature not enabled");

        #[cfg(feature = "tls")]
        {
            let server_name = url.server_name.to_string().try_into()?;
            let conn = ClientConnection::new(config, server_name).or_else(Fail::from)?;
            Box::new(StreamOwned::new(conn, stream))
        }
    } else {
        Box::new(stream)
    };

    Ok(stream)
}
