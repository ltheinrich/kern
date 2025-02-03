//! HTTP response

use std::collections::HashMap;
use std::convert::AsRef;

/// Additional response data
#[derive(Clone, Debug)]
pub struct ResponseData<'a> {
    pub status: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
}

impl Default for ResponseData<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ResponseData<'a> {
    /// Create new with default values
    pub fn new() -> Self {
        Self {
            status: "200 OK",
            headers: HashMap::new(),
        }
    }

    /// Wraps in Option for respond
    pub fn build(self) -> Option<Self> {
        Some(self)
    }

    /// Change status
    pub fn status(mut self, status: &'a str) -> Self {
        self.status = status;
        self
    }

    /// Add header
    pub fn header(mut self, key: &'a str, value: &'a str) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn continue100() -> Self {
        ResponseData::new().status("100 Continue")
    }

    pub fn switching_protocols() -> Self {
        ResponseData::new().status("101 Switching Protocols")
    }

    pub fn early_hints() -> Self {
        ResponseData::new().status("103 Early Hints")
    }

    pub fn ok() -> Self {
        ResponseData::new()
    }

    pub fn created() -> Self {
        ResponseData::new().status("201 Created")
    }

    pub fn accepted() -> Self {
        ResponseData::new().status("202 Accepted")
    }

    pub fn non_authoritative_information() -> Self {
        ResponseData::new().status("203 Non-Authoritative Information")
    }

    pub fn no_content() -> Self {
        ResponseData::new().status("204 No Content")
    }

    pub fn reset_content() -> Self {
        ResponseData::new().status("205 Reset Content")
    }

    pub fn partial_content() -> Self {
        ResponseData::new().status("206 Partial Content")
    }

    pub fn multi_status() -> Self {
        ResponseData::new().status("207 Multi-Status")
    }

    pub fn already_reported() -> Self {
        ResponseData::new().status("208 Already Reported")
    }

    pub fn im_used() -> Self {
        ResponseData::new().status("226 IM Used")
    }

    pub fn multiple_choices() -> Self {
        ResponseData::new().status("300 Multiple Choices")
    }

    pub fn moved_permanently() -> Self {
        ResponseData::new().status("301 Moved Permanently")
    }

    pub fn found() -> Self {
        ResponseData::new().status("302 Found")
    }

    pub fn see_other() -> Self {
        ResponseData::new().status("303 See Other")
    }

    pub fn not_modified() -> Self {
        ResponseData::new().status("304 Not Modified")
    }

    pub fn temporary_redirect() -> Self {
        ResponseData::new().status("307 Temporary Redirect")
    }

    pub fn permanent_redirect() -> Self {
        ResponseData::new().status("308 Permanent Redirect")
    }

    pub fn bad_request() -> Self {
        ResponseData::new().status("400 Bad Request")
    }

    pub fn unauthorized() -> Self {
        ResponseData::new().status("401 Unauthorized")
    }

    pub fn payment_required() -> Self {
        ResponseData::new().status("402 Payment Required")
    }

    pub fn forbidden() -> Self {
        ResponseData::new().status("403 Forbidden")
    }

    pub fn not_found() -> Self {
        ResponseData::new().status("404 Not Found")
    }

    pub fn method_not_allowed() -> Self {
        ResponseData::new().status("405 Method Not Allowed")
    }

    pub fn not_acceptable() -> Self {
        ResponseData::new().status("406 Not Acceptable")
    }

    pub fn proxy_authentication_required() -> Self {
        ResponseData::new().status("407 Proxy Authentication Required")
    }

    pub fn request_timeout() -> Self {
        ResponseData::new().status("408 Request Timeout")
    }

    pub fn conflict() -> Self {
        ResponseData::new().status("409 Conflict")
    }

    pub fn gone() -> Self {
        ResponseData::new().status("410 Gone")
    }

    pub fn length_required() -> Self {
        ResponseData::new().status("411 Length Required")
    }

    pub fn precondition_failed() -> Self {
        ResponseData::new().status("412 Precondition Failed")
    }

    pub fn content_too_large() -> Self {
        ResponseData::new().status("413 Content Too Large")
    }

    pub fn uri_too_long() -> Self {
        ResponseData::new().status("414 URI Too Long")
    }

    pub fn unsupported_media_type() -> Self {
        ResponseData::new().status("415 Unsupported Media Type")
    }

    pub fn range_not_satisfiable() -> Self {
        ResponseData::new().status("416 Range Not Satisfiable")
    }

    pub fn expectation_failed() -> Self {
        ResponseData::new().status("417 Expectation Failed")
    }

    pub fn im_a_teapot() -> Self {
        ResponseData::new().status("418 I'm a teapot")
    }

    pub fn misdirected_request() -> Self {
        ResponseData::new().status("421 Misdirected Request")
    }

    pub fn unprocessable_content() -> Self {
        ResponseData::new().status("422 Unprocessable Content")
    }

    pub fn locked() -> Self {
        ResponseData::new().status("423 Locked")
    }

    pub fn failed_dependency() -> Self {
        ResponseData::new().status("424 Failed Dependency")
    }

    pub fn too_early() -> Self {
        ResponseData::new().status("425 Too Early")
    }

    pub fn upgrade_required() -> Self {
        ResponseData::new().status("426 Upgrade Required")
    }

    pub fn precondition_required() -> Self {
        ResponseData::new().status("428 Precondition Required")
    }

    pub fn too_many_requests() -> Self {
        ResponseData::new().status("429 Too Many Requests")
    }

    pub fn request_header_fields_too_large() -> Self {
        ResponseData::new().status("431 Request Header Fields Too Large")
    }

    pub fn unavailable_for_legal_reasons() -> Self {
        ResponseData::new().status("451 Unavailable For Legal Reasons")
    }

    pub fn internal_server_error() -> Self {
        ResponseData::new().status("500 Internal Server Error")
    }

    pub fn not_implemented() -> Self {
        ResponseData::new().status("501 Not Implemented")
    }

    pub fn bad_gateway() -> Self {
        ResponseData::new().status("502 Bad Gateway")
    }

    pub fn service_unavailable() -> Self {
        ResponseData::new().status("503 Service Unavailable")
    }

    pub fn gateway_timeout() -> Self {
        ResponseData::new().status("504 Gateway Timeout")
    }

    pub fn http_version_not_supported() -> Self {
        ResponseData::new().status("505 HTTP Version Not Supported")
    }

    pub fn variant_also_negotiates() -> Self {
        ResponseData::new().status("506 Variant Also Negotiates")
    }

    pub fn insufficient_storage() -> Self {
        ResponseData::new().status("507 Insufficient Storage")
    }

    pub fn loop_detected() -> Self {
        ResponseData::new().status("508 Loop Detected")
    }

    pub fn not_extended() -> Self {
        ResponseData::new().status("510 Not Extended")
    }

    pub fn network_authentication_required() -> Self {
        ResponseData::new().status("511 Network Authentication Required")
    }
}

/// Create HTTP response
pub fn respond(
    content: impl AsRef<[u8]>,
    content_type: impl AsRef<str>,
    data: Option<ResponseData>,
) -> Vec<u8> {
    // convert content to &[u8]
    let content = content.as_ref();

    // additional response data
    let data = match data {
        Some(data) => data,
        None => ResponseData::new(),
    };
    let status = data.status;
    let mut headers = String::new();
    data.headers.iter().for_each(|(k, v)| {
        headers.push_str("\r\n");
        headers.push_str(k);
        headers.push_str(": ");
        headers.push_str(v);
    });

    // create response
    let mut response = Vec::new();
    let header = format!(
        "HTTP/1.1 {}\r\nserver: ltheinrich.de/kern\r\ncontent-type: {}; charset=utf-8{}",
        status,
        content_type.as_ref(),
        headers
    );
    response.extend_from_slice(header.as_bytes());

    // write content
    response.append(&mut set_content_length(content.len()));
    response.extend_from_slice(content);
    response.extend_from_slice(b"\r\n");

    // return
    response
}

/// create content-length header bytes
fn set_content_length(content_length: usize) -> Vec<u8> {
    let mut header = Vec::new();
    header.extend_from_slice(b"\r\n");
    header.extend_from_slice(b"content-length: ");
    header.extend_from_slice((content_length + 2).to_string().as_bytes());
    header.extend_from_slice(b"\r\n\r\n");
    header
}

/// Create HTTP redirect response
pub fn redirect(url: impl AsRef<str>) -> Vec<u8> {
    // as ref
    let url = url.as_ref();

    // set location
    let mut headers = HashMap::new();
    headers.insert("location", url);

    // create response data
    let data = ResponseData::see_other();

    // create and return response
    respond(
        format!("<html><head><title>Moved</title></head><body><h1>Moved</h1><p><a href=\"{url}\">{url}</a></p></body></html>"),
        "text/html",
        Some(data)
        )
}
