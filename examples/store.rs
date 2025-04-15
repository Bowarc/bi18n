fn main() {
    use bi18n::{I18n, Key, set, t};

    let mut map = ahash::AHashMap::<Key, String>::default();
    map.insert("test.hi".into(), "Hi".into());

    let i18n = I18n::new("en".into(), map);

    set(i18n);

    println!("{}", t("test.hi"));
}
