//! HTTP server

mod builder;
mod request;
mod response;
#[allow(clippy::module_inception)]
mod server;
mod settings;
#[cfg(feature = "tls")]
mod tls;

pub use builder::*;
pub use request::*;
pub use response::*;
pub use server::*;
pub use settings::*;
#[cfg(feature = "tls")]
pub use tls::*;

use crate::{Error, Result};

use std::sync::{Arc, RwLock};

/// Handler function
pub type Handler<S> = fn(HttpRequest, Arc<RwLock<S>>) -> Result<Vec<u8>>;

/// ErrorHandler function
pub type ErrorHandler<S> = fn(Error, Arc<RwLock<S>>) -> Vec<u8>;
