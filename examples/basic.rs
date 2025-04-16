fn main() {
    use bi18n::{I18n, Key, set, i18n_t, Locale};
    
    let mut map = ahash::AHashMap::<Key, String>::default();
    map.insert("test.hi".into(), "Hi".into());

    let i18n = I18n::new("en".into(), map);

    set(i18n);

    // println!("{}", t("test.hi"));
    let key = Key::from("Hi");
    let x = i18n_t!(key);
}
