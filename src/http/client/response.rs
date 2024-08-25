use core::str;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

use crate::{Fail, Result};

#[derive(Clone, Debug)]
pub struct HttpResponse {
    headers: HashMap<String, Vec<String>>,
    status: u16,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn headers(&self) -> &HashMap<String, Vec<String>> {
        &self.headers
    }

    pub fn header_first(&self, name: impl AsRef<str>) -> Option<&str> {
        self.headers
            .get(name.as_ref())
            .and_then(|values| values.first().map(|first| first.as_str()))
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn body_text(&self) -> Result<&str> {
        str::from_utf8(&self.body).or_else(Fail::from)
    }
}

pub fn read_all(stream: &mut impl Read) -> Result<HttpResponse> {
    let mut reader = BufReader::new(stream);
    let (headers, status) = read_headers(&mut reader)?;

    let content_length: Option<usize> = headers
        .get("content-length")
        .and_then(|values| values.first())
        .and_then(|content_length| content_length.parse().ok());
    let mut body;
    if let Some(content_length) = content_length {
        body = vec![0u8; content_length];
        reader.read_exact(&mut body)?;
    } else {
        let mut buf = String::new();
        while reader.read_line(&mut buf)? != 2 {}
        body = buf.as_bytes().to_owned();
    }

    Ok(HttpResponse {
        headers,
        status,
        body,
    })
}

fn read_headers(
    reader: &mut BufReader<&mut impl Read>,
) -> Result<(HashMap<String, Vec<String>>, u16)> {
    let mut raw_header = String::new();
    while reader.read_line(&mut raw_header)? > 2 {}

    let mut headers: HashMap<String, Vec<String>> = HashMap::new();
    let status: u16 = raw_header
        .get(9..12)
        .and_then(|status| status.parse().ok())
        .ok_or_else(|| Fail::new("status code not u16"))?;

    let mut lines = 0;
    let mut duplicate_headers = 0;
    raw_header
        .split("\r\n")
        .filter_map(|line| {
            lines += 1;
            line.split_once(':')
        })
        .for_each(|(key, value)| {
            let key = key.trim().to_lowercase();
            let value = value.trim().to_lowercase();
            if let Some(values) = headers.get_mut(&key) {
                duplicate_headers += 1;
                values.push(value);
            } else {
                headers.insert(key, vec![value]);
            }
        });

    if headers.len() < lines - duplicate_headers - 3 {
        Fail::from("invalid header")?;
    }

    Ok((headers, status))
}
