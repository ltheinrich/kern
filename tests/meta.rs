use kern::meta::{init_name, init_version};

#[test]
fn test_version() {
    // parse version and check
    let v = init_version("version = \"1.0.0\"");
    assert_eq!(v, "1.0.0");

    // parse version and check
    let v = init_version("version = \"1.0.0-beta.1\"");
    assert_eq!(v, "1.0.0-beta.1");

    // parse version and check
    let v = init_version("kern = \"1.1.2\"\nversion = \"1.0.0-alpha.1\"");
    assert_eq!(v, "1.0.0-alpha.1");
}

#[test]
fn test_name() {
    // parse name and check
    let n = init_name("name = \"kern\"");
    assert_eq!(n, "kern");

    // parse name and check
    let n = init_name("version = \"1.1.4\"\nname = \"kern\"");
    assert_eq!(n, "kern");
}
