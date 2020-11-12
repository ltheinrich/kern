/// Version string
static mut VERSION: &str = "";

// Name string
static mut NAME: &str = "";

/// Get version (unsafe, but should be safe unless VERSION is being modified)
pub fn version() -> &'static str {
    unsafe { VERSION }
}

/// Get name (unsafe, but should be safe unless NAME is being modified)
pub fn name() -> &'static str {
    unsafe { NAME }
}

/// Get version string from a Cargo.toml (unsafe, modifies VERSION)
pub fn init_version(cargo_toml: &'static str) -> &str {
    // modifiy VERSION and return
    unsafe {
        VERSION = search(cargo_toml, "version=").unwrap_or("0.0.0");
    }
    version()
}

/// Get name string from a Cargo.toml (unsafe, modifies NAME)
pub fn init_name(cargo_toml: &'static str) -> &str {
    unsafe {
        NAME = search(cargo_toml, "name=").unwrap_or("kern");
    }
    name()
}

/// Search value of key (key must end with =) in cargo_toml
fn search(cargo_toml: &'static str, key: &str) -> Option<&'static str> {
    // split by "
    let blocks: Vec<&str> = cargo_toml.split('"').collect();

    // iterate through blocks
    let mut value_string = None;
    for (i, &block) in blocks.iter().enumerate() {
        // first is never key
        if i == 0 {
            continue;
        }

        // get cleaned/trimmed previous block
        let previous_block = blocks[i - 1]
            .splitn(2, '\n')
            .nth(1)
            .unwrap_or(blocks[i - 1])
            .replace(' ', "");

        // check if previous block was key
        if previous_block == key {
            // set value and break
            value_string = Some(block);
            break;
        }
    }

    // return
    value_string
}
