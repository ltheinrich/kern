use core::str;
use std::collections::HashMap;
use std::io::Write;

use crate::Result;
use crate::http::common::HttpMethod;

use super::request::{connect, end_headers, send_content_length, send_header, send_main_header};
use super::response::{HttpResponse, read_all};

#[cfg(feature = "tls")]
use {
    rustls::{ClientConfig, RootCertStore},
    std::sync::Arc,
    webpki_roots::TLS_SERVER_ROOTS,
};

#[derive(Debug)]
pub struct HttpClient {
    headers: HashMap<String, Vec<String>>,
    query: HashMap<String, String>,
    #[cfg(feature = "tls")]
    config: Arc<ClientConfig>,
}

impl HttpClient {
    pub fn new() -> Self {
        let headers = HashMap::new();
        let query = HashMap::new();
        #[cfg(feature = "tls")]
        let config: Arc<ClientConfig> = ClientConfig::builder()
            .with_root_certificates(RootCertStore {
                roots: TLS_SERVER_ROOTS.into(),
            })
            .with_no_client_auth()
            .into();

        Self {
            headers,
            query,
            #[cfg(feature = "tls")]
            config,
        }
    }

    pub fn with_header(mut self, name: impl ToString, value: impl ToString) -> Self {
        let name = name.to_string();
        let value = value.to_string();

        if let Some(header) = self.headers.get_mut(&name) {
            header.push(value);
        } else {
            self.headers.insert(name, vec![value]);
        }

        self
    }

    pub fn set_header(&mut self, name: impl ToString, value: impl ToString) {
        self.headers
            .insert(name.to_string(), vec![value.to_string()]);
    }

    pub fn remove_header(&mut self, name: impl AsRef<str>) {
        self.headers.remove(name.as_ref());
    }

    pub fn headers(&self) -> &HashMap<String, Vec<String>> {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut HashMap<String, Vec<String>> {
        &mut self.headers
    }

    pub fn with_query(mut self, name: impl ToString, value: impl ToString) -> Self {
        self.query.insert(name.to_string(), value.to_string());
        self
    }

    pub fn remove_query(&mut self, name: impl AsRef<str>) {
        self.query.remove(name.as_ref());
    }

    pub fn query(&self) -> &HashMap<String, String> {
        &self.query
    }

    pub fn query_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.query
    }

    pub fn get(&self, url: impl AsRef<str>) -> Result<HttpResponse> {
        self.request(HttpMethod::Get, url, None)
    }

    pub fn post(&self, url: impl AsRef<str>, body: impl AsRef<[u8]>) -> Result<HttpResponse> {
        self.request(HttpMethod::Get, url, Some(body.as_ref()))
    }

    pub fn put(&self, url: impl AsRef<str>, body: impl AsRef<[u8]>) -> Result<HttpResponse> {
        self.request(HttpMethod::Put, url, Some(body.as_ref()))
    }

    pub fn patch(&self, url: impl AsRef<str>, body: impl AsRef<[u8]>) -> Result<HttpResponse> {
        self.request(HttpMethod::Put, url, Some(body.as_ref()))
    }

    pub fn head(&self, url: impl AsRef<str>) -> Result<HttpResponse> {
        self.request(HttpMethod::Head, url, None)
    }

    pub fn options(&self, url: impl AsRef<str>) -> Result<HttpResponse> {
        self.request(HttpMethod::Options, url, None)
    }

    pub fn delete(&self, url: impl AsRef<str>) -> Result<HttpResponse> {
        self.request(HttpMethod::Delete, url, None)
    }

    pub fn trace(&self, url: impl AsRef<str>) -> Result<HttpResponse> {
        self.request(HttpMethod::Trace, url, None)
    }

    pub fn request(
        &self,
        method: HttpMethod,
        url: impl AsRef<str>,
        body: Option<&[u8]>,
    ) -> Result<HttpResponse> {
        let url = url.as_ref().try_into()?;
        let mut stream = connect(
            #[cfg(feature = "tls")]
            self.config.clone(),
            &url,
        )?;

        send_main_header(&mut stream, method, &url, &self.query)?;

        if let Some(body) = body {
            send_content_length(&mut stream, body)?;
            self.send_headers(&mut stream)?;
            stream.write_all(body)?;
        } else {
            self.send_headers(&mut stream)?;
        }

        read_all(&mut stream)
    }

    fn send_headers(&self, stream: &mut impl Write) -> Result<()> {
        for (name, values) in &self.headers {
            for value in values {
                send_header(stream, name, value)?;
            }
        }
        end_headers(stream)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
