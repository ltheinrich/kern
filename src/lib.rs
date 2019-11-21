//! General library for Rust

mod error;

pub mod cli;
pub mod conf;
pub mod file;
pub mod meta;
pub mod net;
pub mod rand;

pub use error::Error;
