use std::num::NonZeroUsize;
use std::thread::available_parallelism;
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
    pub threads: HttpThreads,
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
            threads: HttpThreads::SPAWN(available_parallelism().unwrap_or(NonZeroUsize::MIN).get()),
        }
    }

    pub fn max_header_size(mut self, max_header_size: usize) -> Self {
        self.max_header_size = max_header_size;
        self
    }

    pub fn max_body_size(mut self, max_body_size: usize) -> Self {
        self.max_body_size = max_body_size;
        self
    }

    pub fn header_buffer(mut self, header_buffer: usize) -> Self {
        self.header_buffer = header_buffer;
        self
    }

    pub fn body_buffer(mut self, body_buffer: usize) -> Self {
        self.body_buffer = body_buffer;
        self
    }

    pub fn header_read_attempts(mut self, header_read_attempts: usize) -> Self {
        self.header_read_attempts = header_read_attempts;
        self
    }

    pub fn body_read_attempts(mut self, body_read_attempts: usize) -> Self {
        self.body_read_attempts = body_read_attempts;
        self
    }

    pub fn read_timeout(mut self, read_timeout: Option<Duration>) -> Self {
        self.read_timeout = read_timeout;
        self
    }

    pub fn write_timeout(mut self, write_timeout: Option<Duration>) -> Self {
        self.write_timeout = write_timeout;
        self
    }

    pub fn threads(mut self, threads: HttpThreads) -> Self {
        self.threads = threads;
        self
    }

    pub fn threads_num(mut self, threads_num: usize) -> Self {
        use HttpThreads::{CONSTANT, SPAWN};
        match self.threads {
            SPAWN(ref mut num) => *num = threads_num,
            CONSTANT(ref mut num) => *num = threads_num,
        };
        self
    }
}

/// Configuration for HTTP threads
#[derive(Clone, Debug)]
pub enum HttpThreads {
    /// Spawns all N threads at start
    /// No new thread is spawned for an incoming request
    CONSTANT(usize),

    /// Spawns N threads at start to accept new connections
    /// A new thread is then spawned for each incoming request
    SPAWN(usize),
}
