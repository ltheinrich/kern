use kern::string::{is_alphanumeric, is_alphanumeric_char};

#[test]
fn test_is_alphanumeric_char() {
    let accept = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let decline = "äöü !\"§$%&/()=?{[]}\\+#*',.-;:_<>|^°áàéèíìóòúù";

    for a in accept.chars() {
        assert!(is_alphanumeric_char(a));
    }
    for d in decline.chars() {
        assert!(!is_alphanumeric_char(d));
    }
}

#[test]
fn test_is_alphanumeric() {
    let accept = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let decline = "äöü !\"§$%&/()=?{[]}\\+#*',.-;:_<>|^°áàéèíìóòúù";

    assert!(is_alphanumeric(accept));
    assert!(!is_alphanumeric(decline));
    assert!(!is_alphanumeric(format!("{accept}{decline}")));
}
