//! HTTP request parsing

use crate::byte::{split, splitn};
use crate::http::server::{HttpSettings, Stream};
use crate::{Fail, Result};

use std::io::prelude::Read;
use std::{collections::HashMap, net::SocketAddr};

/// HTTP request method (GET or POST)
#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
}

/// HTTP request structure
#[derive(Debug)]
pub struct HttpRequest<'a> {
    method: HttpMethod,
    url: &'a str,
    headers: HashMap<String, &'a str>,
    get: HashMap<String, &'a str>,
    post: HashMap<String, Vec<u8>>,
    body: Vec<u8>,
    ip: String,
}

impl<'a> HttpRequest<'a> {
    /// Get HTTP request method
    pub fn method(&self) -> &HttpMethod {
        // return HTTP request method
        &self.method
    }

    /// Get URL
    pub fn url(&self) -> &str {
        // return URL
        self.url
    }

    /// Get headers map
    pub fn headers(&self) -> &HashMap<String, &str> {
        // return headers map
        &self.headers
    }

    /// Get GET parameters
    pub fn get(&self) -> &HashMap<String, &str> {
        // return GET parameters map
        &self.get
    }

    /// Get POST parameters
    pub fn post(&self) -> &HashMap<String, Vec<u8>> {
        // return POST parameters map
        &self.post
    }

    /// Get POST parameters
    pub fn post_utf8(&self) -> HashMap<String, String> {
        // init map and iterate through byte map
        let mut post_utf8 = HashMap::new();
        for (k, v) in &self.post {
            // parse and insert
            post_utf8.insert(k.to_string(), String::from_utf8_lossy(v).to_string());
        }

        // return new UTF-8 POST parameters map
        post_utf8
    }

    /// Get body
    pub fn body(&self) -> &[u8] {
        // return body string
        &self.body
    }

    /// Get IP address
    pub fn ip(&self) -> &str {
        // return IP address string
        &self.ip
    }

    /// Parse HTTP request
    pub fn from(
        raw_header: &'a str,
        mut raw_body: Vec<u8>,
        stream: &mut Stream,
        address: SocketAddr,
        http_settings: &HttpSettings,
    ) -> Result<Self> {
        // split header
        let mut header = raw_header.lines();
        let mut reqln = header
            .next()
            .ok_or_else(|| Fail::new("Empty header"))?
            .split(' ');

        // parse method
        let method = if reqln
            .next()
            .ok_or_else(|| Fail::new("No method in header"))?
            == "POST"
        {
            HttpMethod::Post
        } else {
            HttpMethod::Get
        };

        // parse url and split raw get parameters
        let mut get_raw = "";
        let url = if let Some(full_url) = reqln.next() {
            let mut split_url = full_url.splitn(2, '?');
            let url = split_url
                .next()
                .ok_or_else(|| Fail::new("No URL in header"))?;
            if let Some(params) = split_url.next() {
                get_raw = params;
            }
            url
        } else {
            "/"
        };

        // parse headers
        let mut headers = HashMap::new();
        header.for_each(|hl| {
            let mut hls = hl.splitn(2, ':');
            if let (Some(key), Some(value)) = (hls.next(), hls.next()) {
                headers.insert(key.trim().to_lowercase(), value.trim());
            }
        });

        // get content length
        let buf_len = if let Some(buf_len) = headers.get("Content-Length") {
            Some(buf_len)
        } else {
            headers.get("content-length")
        };

        // read rest of body
        let mut body = Vec::new();
        if let Some(buf_len) = buf_len {
            // parse buffer length
            let con_len = buf_len
                .parse::<usize>()
                .ok()
                .ok_or_else(|| Fail::new("Content-Length is not of type usize"))?;

            // check if body size is ok.
            if con_len > http_settings.max_body_size {
                return Fail::from("Max body size exceeded");
            }

            // read body
            let mut read_fails = 0;
            while raw_body.len() < con_len {
                // read next buffer
                let mut rest_body = vec![0u8; http_settings.body_buffer];
                let length = stream
                    .read(&mut rest_body)
                    .ok()
                    .ok_or_else(|| Fail::new("Stream broken"))?;
                rest_body.truncate(length);
                raw_body.append(&mut rest_body);

                // check if didn't read fully
                if length < http_settings.body_buffer {
                    read_fails += 1;

                    // failed too often
                    if read_fails > http_settings.body_read_attempts {
                        return Fail::from("Read body failed too often");
                    }
                }
            }

            body = raw_body;
        }

        // parse GET and POST parameters
        let get = parse_parameters(get_raw, |v| v)?;
        let post = parse_post(&headers, &body).unwrap_or_default();

        // ip: x-real-ip if socket ip is loopback else socket ip
        let ip = match headers.get("x-real-ip") {
            Some(x_real_ip) if address.ip().is_loopback() => x_real_ip.to_string(),
            _ => address.ip().to_string(),
        };

        // return request
        Ok(Self {
            method,
            url,
            headers,
            get,
            post,
            body,
            ip,
        })
    }
}

/// Parse POST parameters to map
fn parse_post(headers: &HashMap<String, &str>, body: &[u8]) -> Result<HashMap<String, Vec<u8>>> {
    match headers.get("content-type") {
        Some(&content_type_header) => {
            let mut content_type_header = content_type_header.split(';').map(|s| s.trim());
            let mut content_type = None;
            let boundary = content_type_header.find_map(|s| {
                if s.starts_with("boundary=") {
                    return s.split('=').nth(1);
                } else if content_type.is_none() {
                    content_type = Some(s);
                }
                None
            });
            match content_type {
                Some(content_type) => {
                    if content_type == "multipart/form-data" {
                        parse_post_upload(
                            body,
                            boundary.ok_or_else(|| Fail::new("post upload, but no boundary"))?,
                        )
                    } else {
                        parse_parameters(&String::from_utf8(body.to_vec())?, |v| {
                            v.as_bytes().to_vec()
                        })
                    }
                }
                None => parse_parameters(&String::from_utf8(body.to_vec())?, |v| {
                    v.as_bytes().to_vec()
                }),
            }
        }
        None => parse_parameters(&String::from_utf8(body.to_vec())?, |v| {
            v.as_bytes().to_vec()
        }),
    }
}

/// Parse POST upload to map
fn parse_post_upload(body: &[u8], boundary: &str) -> Result<HashMap<String, Vec<u8>>> {
    // parameters map
    let mut params = HashMap::new();

    // split body into sections
    let mut sections = split(&body, &format!("--{boundary}\r\n"));
    sections.remove(0);
    for mut section in sections {
        // check if last section
        let last_sep = format!("--{boundary}--\r\n");
        if section.ends_with(last_sep.as_bytes()) {
            // remove ending seperator from last section
            section = &section[..(section.len() - last_sep.len() - 2)];
        }
        // split lines (max 3)
        let lines = splitn(3, &section, b"\r\n");

        // parse name
        let name = String::from_utf8_lossy(lines[0])
            .split(';')
            .map(|s| s.trim())
            .find_map(|s| {
                if s.starts_with("name=") {
                    let name = s.split('=').nth(1)?;
                    Some(name[1..(name.len() - 1)].to_lowercase())
                } else {
                    None
                }
            })
            .ok_or_else(|| Fail::new("missing name in post body section"))?;

        // get value
        let data_section = lines
            .get(2)
            .ok_or_else(|| Fail::new("broken section in post body"))?;
        let data_lines = splitn(2, data_section, b"\r\n");
        let next_data_line = data_lines
            .first()
            .ok_or_else(|| Fail::new("broken section in post body"))?;
        let value = if let Some(file_data_line) = data_lines.get(1) {
            if next_data_line.is_empty() {
                file_data_line.to_vec()
            } else if file_data_line.is_empty() {
                next_data_line.to_vec()
            } else {
                [&next_data_line[..], &b"\r\n"[..], &file_data_line[..]]
                    .concat()
                    .to_vec()
            }
        } else {
            next_data_line.to_vec()
        };

        // insert into map
        params.insert(name, value);
    }

    // return parameters map
    Ok(params)
}

/// Parse GET parameters to map
fn parse_parameters<'a, V>(
    raw: &'a str,
    process_value: fn(&'a str) -> V,
) -> Result<HashMap<String, V>> {
    // parameters map
    let mut params = HashMap::new();

    // split parameters by ampersand
    for p in raw.split('&') {
        // split key and value and add to map
        let mut ps = p.splitn(2, '=');
        params.insert(
            ps.next()
                .ok_or_else(|| Fail::new("broken x-www-form-urlencoded parameters"))?
                .trim()
                .to_lowercase(), // trimmed key
            // correct value type
            process_value(if let Some(value) = ps.next() {
                value.trim() // trimmed value
            } else {
                "" // no value, is option
            }),
        );
    }

    // return parameters map
    Ok(params)
}
