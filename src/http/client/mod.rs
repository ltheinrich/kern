//! HTTP client
//! Work in progress

#[allow(clippy::module_inception)]
mod client;
mod request;
mod response;
mod url;

pub use client::*;
pub use response::HttpResponse;
