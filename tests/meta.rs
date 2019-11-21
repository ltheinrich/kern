extern crate kern;

use kern::meta::version;

// test version
#[test]
fn test_version() {
    // get version
    let v = version("version = \"1.0.0\"");

    // check if contains two dots
    assert_eq!(v.split('.').collect::<String>().len(), 3);

    // check if 1.0.0
    assert_eq!(v, "1.0.0");
}
