extern crate kern;

use kern::conf::Config;

#[test]
fn get() {
    let config = Config::from("Hallo=Du\nDas=Ist ein kleiner Test\n\n");
    assert_eq!("Du", config.get("Hallo").unwrap());
    assert_eq!("Ist ein kleiner Test", config.get("Das").unwrap());
    assert_ne!("Diesmal falsch", config.get("Hallo").unwrap());
}

#[test]
fn exists() {
    let config = Config::from("Hallo=Du\nDas=Ist ein kleiner Test\n\n");
    assert_eq!(true, config.exists("Hallo"));
    assert_eq!(true, config.exists("Das"));
    assert_eq!(false, config.exists("Hall"));
    assert_eq!(false, config.exists(""));
}

#[test]
fn equals() {
    let config = Config::from("Hallo=Du\nDas=Ist ein kleiner Test\n\n");
    assert_eq!(true, config.equals("Hallo", "Du"));
    assert_eq!(false, config.equals("Hallo", "Falscher Eintrag"));
    assert_eq!(false, config.equals("Hall", "Gibt es nicht"));
    assert_eq!(true, config.equals("Das", "Ist ein kleiner Test"));
}

#[test]
fn fill() {
    let config = Config::read("Cargo.toml")
        .unwrap()
        .fill("Hallo=Du\nDas=Ist ein kleiner Test\nname=Lennart\n\n");
    assert_eq!("\"kern\"", config.get("name").unwrap());
    assert_eq!(
        "[\"Lennart Heinrich <lennart@ltheinrich.de>\"]",
        config.get("authors").unwrap()
    );
    assert_eq!("Du", config.get("Hallo").unwrap());
    assert_eq!("Ist ein kleiner Test", config.get("Das").unwrap());
    assert_eq!(false, config.exists(""));
}

#[test]
fn read() {
    let config = Config::read("Cargo.toml").unwrap();
    assert_eq!("\"kern\"", config.get("name").unwrap());
    assert_eq!(
        "[\"Lennart Heinrich <lennart@ltheinrich.de>\"]",
        config.get("authors").unwrap()
    );
}

#[test]
fn from() {
    let config = Config::from("Hallo=Du\nDas=Ist ein kleiner Test\n\n");
    assert_eq!("Du", config.get("Hallo").unwrap());
    assert_eq!("Ist ein kleiner Test", config.get("Das").unwrap());
    assert_eq!(false, config.exists(""));
}
