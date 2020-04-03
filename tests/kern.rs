use kern::Fail;
use kern::{init_name, init_version};

#[test]
fn fail() {
    // create new Fail from string
    let other_err = Fail::new("Das ist ein Fehler");
    let err_1 = Fail::new(other_err);
    let err_2: Result<(), Fail> = Fail::from("Das ist auch ein Fehler");

    // check if error message is correct
    assert_eq!(err_1.err_msg(), "Das ist ein Fehler");
    assert_eq!(err_2.unwrap_err().err_msg(), "Das ist auch ein Fehler");
}

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
fn name() {
    // parse name and check
    let n = init_name("name = \"kern\"");
    assert_eq!(n, "kern");

    // parse name and check
    let n = init_name("version = \"1.1.4\"\nname = \"kern\"");
    assert_eq!(n, "kern");
}
