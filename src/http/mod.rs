//! Lightweight HTTP library

pub mod server;

use std::io::{Read, Write};

use crate::meta::{init_name, init_version, name as get_name, version as get_version};

const CARGO_TOML: &str = include_str!("../../Cargo.toml");

/// Get kern version string
pub fn version() -> &'static str {
    match get_version() {
        "" => init_version(CARGO_TOML),
        version => version,
    }
}

/// Get kern name string
pub fn name() -> &'static str {
    match get_name() {
        "" => init_name(CARGO_TOML),
        name => name,
    }
}

pub trait ReadWrite: Read + Write {}

impl<T: Read + Write> ReadWrite for T {}

// TODO add tests
