/// Get version string from a Cargo.toml
pub fn version<'a>(cargo_toml: &'a str) -> &'a str {
    // split by "
    let blocks: Vec<&'a str> = cargo_toml.split('"').collect();

    // get fourth string
    match blocks.get(3) {
        Some(version) => {
            // check if contains two dots
            if version.split('.').count() == 3 {
                // return correct version
                version
            } else {
                // check first version
                check_version(&blocks, 0)
            }
        }
        None => check_version(&blocks, 0),
    }
}

// Return version string else check next
fn check_version<'a>(blocks: &[&'a str], index: usize) -> &'a str {
    // get string at index
    match blocks.get(index) {
        Some(version) => {
            // check if contains two dots
            if version.split('.').count() == 3 {
                // return correct version
                version
            } else {
                // check next version
                check_version(blocks, index + 1)
            }
        }
        None => "0.0.0",
    }
}
