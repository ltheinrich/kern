use std::sync::Arc;

#[cfg(feature = "tls")]
use rustls::ServerConfig;

use crate::Result;

use super::{respond, ErrorHandler, Handler, HttpServer, HttpSettings, ResponseData};

/// Builder for HttpServer
#[derive(Clone, Debug)]
pub struct HttpServerBuilder {
    addr: String,
    settings: HttpSettings,
    handler: Handler,
    error_handler: ErrorHandler,
    #[cfg(feature = "tls")]
    tls_config: Option<fn() -> Arc<ServerConfig>>,
}

impl Default for HttpServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpServerBuilder {
    /// Create new HttpServerBuilder with defaults
    pub fn new() -> Self {
        Self {
            addr: "localhost:8080".to_string(),
            settings: HttpSettings::default(),
            handler: |_| unimplemented!(),
            error_handler: |err| {
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
    pub fn tls(mut self, tls_config: Option<fn() -> Arc<ServerConfig>>) -> Self {
        self.tls_config = tls_config;
        self
    }

    #[cfg(feature = "tls")]
    /// Set TLS configuration and enable TLS
    pub fn tls_on(self, tls_config: fn() -> Arc<ServerConfig>) -> Self {
        self.tls(Some(tls_config))
    }

    #[cfg(feature = "tls")]
    /// Remove TLS configuration and disable TLS
    pub fn tls_off(self) -> Self {
        self.tls(None)
    }

    /// Set request handler
    pub fn handler(mut self, handler: Handler) -> Self {
        self.handler = handler;
        self
    }

    /// Set error handler
    pub fn error_handler(mut self, error_handler: ErrorHandler) -> Self {
        self.error_handler = error_handler;
        self
    }

    /// Build HttpServer
    pub fn build(self) -> Result<Arc<HttpServer>> {
        HttpServer::new(
            self.addr,
            Arc::new(self.settings),
            self.handler,
            self.error_handler,
            #[cfg(feature = "tls")]
            self.tls_config,
        )
    }
}
