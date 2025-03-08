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

/// Handler function
pub type Handler = fn(HttpRequest) -> Result<Vec<u8>>;

/// ErrorHandler function
pub type ErrorHandler = fn(Error) -> Vec<u8>;
