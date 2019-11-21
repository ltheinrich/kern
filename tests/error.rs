extern crate kern;

use kern::Error;

// test new error creation
#[test]
fn error_new() {
    // create new error from another error
    let other_err = Error::new("Das ist ein Fehler");
    let err_1 = Error::new(other_err);
    let err_2 = Error::new("Das ist auch ein Fehler");

    // check if error message is correct
    assert_eq!(err_1.to_string().as_str(), "Das ist ein Fehler");
    assert_eq!(err_2.to_string().as_str(), "Das ist auch ein Fehler");
}

// test new error result creation
#[test]
fn error_from() {
    // create new error result from another error
    let other_err = Error::new("Das ist ein Fehler");
    let err_1: Result<(), Error> = Error::from(other_err);
    let err_2: Result<(), Error> = Error::from("Das ist auch ein Fehler");

    // check if error message is correct
    assert_eq!(
        err_1.unwrap_err().to_string().as_str(),
        "Das ist ein Fehler"
    );
    assert_eq!(
        err_2.unwrap_err().to_string().as_str(),
        "Das ist auch ein Fehler"
    );
}
