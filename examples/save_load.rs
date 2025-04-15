fn save(i18n: bi18n::I18n) {
    use {std::fs::OpenOptions, std::io::Write as _};

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("./examples/i18n-{}", i18n.locale()))
        .unwrap();

    file.write_all(serde_json::to_string(&i18n).unwrap().as_bytes())
        .unwrap()
}

fn load() -> bi18n::I18n {
    use {std::fs::OpenOptions, std::io::Read as _};

    let mut file = OpenOptions::new()
        .read(true)
        .open("./examples/i18n-en")
        .unwrap();

    let mut buffer = String::new();
    let _read = file.read_to_string(&mut buffer).unwrap();

    serde_json::from_str(&buffer).unwrap()
}

fn main() {
    use bi18n::{I18n, Key};

    let mut map = ahash::AHashMap::<Key, String>::default();
    map.insert("test.hi".into(), "Hi".into());

    let i18n = I18n::new("en".into(), map);

    let clone = i18n.clone();

    save(i18n);

    let loaded = load();

    assert_eq!(clone, loaded);
    println!("Equal !");
    println!("{loaded:?}");
}
