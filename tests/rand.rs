extern crate kern;

use kern::rand::{rand_byte, rand_bytes};

#[test]
fn test_byte() {
    rand_byte().unwrap();
}

#[test]
fn test_bytes() {
    assert_eq!(rand_bytes(10).unwrap().len(), 10);
}
