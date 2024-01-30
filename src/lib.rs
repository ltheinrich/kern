//! General library for Rust

mod cli;
mod conf;
mod error;

pub mod byte;
pub mod data;
pub mod http;
pub mod meta;
pub mod string;

pub use cli::{CliBuilder, Command};
pub use conf::Config;
pub use error::{Error, Fail, Result};
