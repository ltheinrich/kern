//! General library for Rust

mod error;

pub mod cli;
pub mod conf;

pub use cli::Command;
pub use conf::Config;
pub use error::Fail;

// Version string
static mut VERSION: &str = "";

/// Get version (unsafe, but should be safe unless VERSION is being modified)
pub fn version() -> &'static str {
    unsafe { VERSION }
}

/// Get version string from a Cargo.toml (unsafe, modifies VERSION)
pub fn init_version(cargo_toml: &'static str) -> &str {
    // split by "
    let blocks: Vec<&str> = cargo_toml.split('"').collect();

    // get fourth string
    let version_string = match blocks.get(3) {
        Some(version_string) => {
            // check if contains two dots
            if version_string.split('.').count() == 3 {
                // return correct version
                version_string
            } else {
                // check first version
                check_version(&blocks, 0)
            }
        }
        None => check_version(&blocks, 0),
    };

    // modifiy VERSION and return
    unsafe {
        VERSION = version_string;
    }
    version()
}

// Return version string else check next
fn check_version(blocks: &[&'static str], index: usize) -> &'static str {
    // get string at index
    match blocks.get(index) {
        Some(version_string) => {
            // check if contains two dots
            if version_string.split('.').count() == 3 {
                // return correct version
                version_string
            } else {
                // check next version
                check_version(blocks, index + 1)
            }
        }
        None => "0.0.0",
    }
}
