use std::error::Error;
use std::io::{Read, Write};

use crate::Fail;

pub trait ReadWrite: Read + Write {}

impl<T: Read + Write> ReadWrite for T {}

/// HTTP request method (GET or POST)
#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Connect,
    Options,
    Trace,
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        use HttpMethod::*;
        match self {
            Get => "GET",
            Post => "POST",
            Put => "PUT",
            Delete => "DELETE",
            Head => "HEAD",
            Connect => "CONNECT",
            Options => "OPTIONS",
            Trace => "TRACE",
        }
    }
}

impl TryFrom<&str> for HttpMethod {
    type Error = Box<dyn Error>;

    fn try_from(item: &str) -> Result<Self, Self::Error> {
        match item {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "HEAD" => Ok(HttpMethod::Head),
            "CONNECT" => Ok(HttpMethod::Connect),
            "OPTIONS" => Ok(HttpMethod::Options),
            "TRACE" => Ok(HttpMethod::Trace),
            _ => Fail::from("Invalid method in header"),
        }
    }
}
