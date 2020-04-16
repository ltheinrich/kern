use kern::byte::{scan, split, splitn};

#[test]
fn test_splitn() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6];
    let s = splitn(2, &v, &[2, 3]);

    assert_eq!(s.len(), 2);
    assert_eq!(s[0].len(), 2);
    assert_eq!(s[1].len(), 13);
}

#[test]
fn test_split() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6];
    let s = split(&v, &[2, 3]);

    assert_eq!(s.len(), 3);
    assert_eq!(s[0].len(), 2);
    assert_eq!(s[1].len(), 8);
    assert_eq!(s[2].len(), 3);
}

#[test]
fn test_scan() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    assert_eq!(scan(&v, &[5, 6, 7]).unwrap(), 5);

    v.reverse();
    assert_eq!(scan(&v, &[7, 6, 5]).unwrap(), 4);
}
