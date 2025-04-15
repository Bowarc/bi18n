mod key;
mod locale;
pub use key::Key;
pub use locale::Locale;
mod i18n;
pub use i18n::I18n;

lazy_static::lazy_static! {
    static ref I18N: std::sync::Arc<parking_lot::RwLock<Option<I18n>>> = std::sync::Arc::new(parking_lot::RwLock::new(None));
}

pub fn t(key: impl Into<Key>) -> String {
    try_t(key).unwrap()
}

pub fn try_t(key: impl Into<Key>) -> Option<String> {
    I18N.read().as_ref()?.try_get(key.into()).cloned()
}

pub fn set(i18n: I18n) {
    *I18N.write() = Some(i18n)
}

pub fn free() {
    *I18N.write() = None
}

pub fn try_get_locale() -> Option<Locale> {
    Some(I18N.read().as_ref()?.locale().clone())
}
