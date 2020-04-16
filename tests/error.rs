use kern::Fail;

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
