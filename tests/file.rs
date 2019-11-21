extern crate kern;

use kern::file::FileDir;
use std::fs;

// test directory listing
#[test]
fn filedir_list_dir() {
    // read and list project directory
    let project_dir = fs::read_dir("./").unwrap();
    let filedir = FileDir::list_dir(project_dir).unwrap();

    // check if LICENSE exists in project directory
    let license_exists = filedir.iter().any(|entry| match entry {
        // not a file
        FileDir::Dir(_, _, _) => false,
        // is a file, check name
        FileDir::File(file_name, _) => file_name == "./LICENSE",
    });

    // should exist
    assert_eq!(license_exists, true);
}
