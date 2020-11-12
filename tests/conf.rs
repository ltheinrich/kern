use kern::Config;

#[test]
fn config() {
    // generate config
    let config = Config::from("Hallo=Du\nDas=Ist ein kleiner Test\nint=604\nbool=true\n\n");

    // check value
    assert_eq!(config.value("Hallo", "falsch"), "Du");
    assert_eq!(config.value("Das", "falsch"), "Ist ein kleiner Test");
    assert_ne!(config.value("Hallo", "falsch"), "Diesmal falsch");

    // check get
    assert_eq!(config.get("int", 0), 604);
    assert_eq!(config.get("bool", false), true);

    // check exists
    assert_eq!(config.exists("Hallo"), true);
    assert_eq!(config.exists("Das"), true);
    assert_eq!(config.exists("Hall"), false);
    assert_eq!(config.exists(""), false);
}

#[test]
fn read() {
    // read Cargo.toml as config
    let mut buf = String::new();
    let config = Config::read("Cargo.toml", &mut buf).unwrap();

    // check license and authors
    assert_eq!("\"ISC\"", config.value("license", "falsch"));
    assert_eq!(
        "[\"Lennart Heinrich <lennart@ltheinrich.de>\"]",
        config.value("authors", "falsch")
    );
}
