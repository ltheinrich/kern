use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};
use std::thread::available_parallelism;

#[cfg(feature = "tls")]
use rustls::ServerConfig;

use crate::Result;

use super::{respond, ErrorHandler, Handler, HttpServer, HttpSettings, ResponseData};

/// Builder for HttpServer
#[derive(Clone, Debug)]
pub struct HttpServerBuilder<S: Send + Sync + 'static> {
    addr: String,
    settings: HttpSettings,
    handler: Handler<S>,
    error_handler: ErrorHandler<S>,
    threads: usize,
    #[cfg(feature = "tls")]
    tls_config: Option<ServerConfig>,
}

impl<S: Send + Sync + 'static> Default for HttpServerBuilder<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Send + Sync + 'static> HttpServerBuilder<S> {
    /// Create new HttpServerBuilder with defaults
    pub fn new() -> Self {
        Self {
            addr: "localhost:8080".to_string(),
            threads: available_parallelism().unwrap_or(NonZeroUsize::MIN).get(),
            settings: HttpSettings::default(),
            handler: |_, _| unimplemented!(),
            error_handler: |err, _| {
                respond(
                    err.to_string(),
                    "text/plain",
                    ResponseData::internal_server_error().build(),
                )
            },
            #[cfg(feature = "tls")]
            tls_config: None,
        }
    }

    /// Set TcpListener address
    pub fn addr(mut self, addr: impl ToString) -> Self {
        self.addr = addr.to_string();
        self
    }

    /// Set HttpSettings
    pub fn settings(mut self, settings: HttpSettings) -> Self {
        self.settings = settings;
        self
    }

    #[cfg(feature = "tls")]
    /// Set TLS configuration (Option)
    /// TLS enabled when Some(ServerConfig)
    /// TLS disabled when None
    pub fn tls(mut self, tls_config: Option<ServerConfig>) -> Self {
        self.tls_config = tls_config;
        self
    }

    #[cfg(feature = "tls")]
    /// Set TLS configuration and enable TLS
    pub fn tls_on(self, tls_config: ServerConfig) -> Self {
        self.tls(Some(tls_config))
    }

    #[cfg(feature = "tls")]
    /// Remove TLS configuration and disable TLS
    pub fn tls_off(self) -> Self {
        self.tls(None)
    }

    /// Set request handler
    pub fn handler(mut self, handler: Handler<S>) -> Self {
        self.handler = handler;
        self
    }

    /// Set error handler
    pub fn error_handler(mut self, error_handler: ErrorHandler<S>) -> Self {
        self.error_handler = error_handler;
        self
    }

    /// Set thread count
    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    /// Build HttpServer
    pub fn build(self, shared: Arc<RwLock<S>>) -> Result<Arc<HttpServer<S>>> {
        HttpServer::new(
            self.addr,
            Arc::new(self.settings),
            self.handler,
            self.error_handler,
            shared,
            self.threads,
            #[cfg(feature = "tls")]
            self.tls_config.map(Arc::new),
        )
    }
}
