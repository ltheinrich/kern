use std::time::Duration;

/// HTTP server settings
#[derive(Clone, Debug)]
pub struct HttpSettings {
    pub max_header_size: usize,
    pub max_body_size: usize,
    pub header_buffer: usize,
    pub body_buffer: usize,
    pub header_read_attempts: usize,
    pub body_read_attempts: usize,
    pub read_timeout: Option<Duration>,
    pub write_timeout: Option<Duration>,
}

impl Default for HttpSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpSettings {
    /// Create new HttpSettings with default values
    pub fn new() -> Self {
        Self {
            max_header_size: 8192,
            max_body_size: 10_485_760,
            header_buffer: 8192,
            body_buffer: 8192,
            header_read_attempts: 3,
            body_read_attempts: 3,
            read_timeout: Some(Duration::from_secs(10)),
            write_timeout: Some(Duration::from_secs(10)),
        }
    }
}
