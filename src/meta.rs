use std::sync::RwLock;

/// Version string
static VERSION: RwLock<&str> = RwLock::new("");

// Name string
static NAME: RwLock<&str> = RwLock::new("");

/// Get version
pub fn version() -> &'static str {
    *VERSION.read().unwrap()
}

/// Get name
pub fn name() -> &'static str {
    *NAME.read().unwrap()
}

/// Get version string from a Cargo.toml
pub fn init_version(cargo_toml: &'static str) -> &'static str {
    // modifiy VERSION and return
    let version = search(cargo_toml, "version=").unwrap_or("0.0.0");
    *VERSION.write().unwrap() = version;
    version
}

/// Get name string from a Cargo.toml
pub fn init_name(cargo_toml: &'static str) -> &'static str {
    let name = search(cargo_toml, "name=").unwrap_or("kern");
    *NAME.write().unwrap() = name;
    name
}

/// Search value of key (key must end with =) in cargo_toml
pub fn search(cargo_toml: &'static str, key: &str) -> Option<&'static str> {
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
            .split('\n')
            .next_back()
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
