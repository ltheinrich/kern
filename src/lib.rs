//! General library for Rust

mod error;

pub mod cli;
pub mod conf;

pub use cli::Command;
pub use conf::Config;
pub use error::Fail;

/// Version string
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
            let mut split = version_string.split('-');
            if split.clone().count() <= 2 && split.next().unwrap().split('.').count() == 3 {
                // check if it is actually version
                if blocks[2].contains("version") {
                    // return correct version
                    version_string
                } else {
                    check_version(&blocks, 0)
                }
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

/// Return version string else check next
fn check_version(blocks: &[&'static str], index: usize) -> &'static str {
    // get string at index
    match blocks.get(index) {
        Some(version_string) => {
            // check if contains two dots
            let mut split = version_string.split('-');
            if split.clone().count() <= 2 && split.next().unwrap().split('.').count() == 3 {
                // check if it is actually version
                match blocks.get(index - 1) {
                    Some(previous) => {
                        if previous.contains("version") {
                            // return correct version
                            version_string
                        } else {
                            check_version(&blocks, index + 1)
                        }
                    }
                    None => check_version(&blocks, index + 1),
                }
            } else {
                // check next version
                check_version(blocks, index + 1)
            }
        }
        None => "0.0.0",
    }
}
