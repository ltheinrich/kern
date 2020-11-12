//! General library for Rust

mod cli;
mod conf;
mod error;

pub mod byte;
pub mod http;
pub mod meta;

pub use cli::Command;
pub use conf::Config;
pub use error::Fail;
