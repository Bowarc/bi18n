mod key;
mod locale;
pub use key::Key;
pub use locale::Locale;
mod i18n;
pub use i18n::I18n;

use {ahash::AHashMap, parking_lot::RwLock, std::sync::Arc};

lazy_static::lazy_static! {
    static ref CURRENT: Arc<RwLock<Option<I18n>>> = Arc::new(RwLock::new(None));
    static ref DICTIONARY: Arc<RwLock<AHashMap<Locale, I18n>>> = Arc::new(RwLock::new(AHashMap::default()));
}

/// Convenience macro for i18n* functions
/// /!\ Theses functions can panic on error, if you're unsure, use the try_* functions
#[macro_export]
macro_rules! i18n {
    ($key:expr) => {
        bi18n::i18n_current($key)
    };
    ($key:expr, $locale:expr) => {
        bi18n::i18n_with_locale($key, $locale)
    };
}

/// Convenience macro for try_i18n* functions
#[macro_export]
macro_rules! try_i18n {
    ($key:expr) => {
        bi18n::try_i18n_current($key)
    };
    ($key:expr, $locale:expr) => {
        bi18n::try_i18n_with_locale($key, $locale)
    };
}

/// Fetches the translation corresponding to the given key in the currently saved i18n
/// Panics if not found, use try_i18n_current if you're not sure
pub fn i18n_current(key: impl Into<Key>) -> String {
    try_i18n_current(key).unwrap()
}

/// Tries to fetch the translation corresponding to the given key in the currently saved i18n
pub fn try_i18n_current(key: impl Into<Key>) -> Option<String> {
    CURRENT.read().as_ref()?.try_get(&key.into()).cloned()
}

/// Fetches the translation corresponding to the given key and given locale in all saved i18n (current & saved)
/// Panics if not found, use try_i18n_with_locale if you're not sure
pub fn i18n_with_locale(key: impl Into<Key>, locale: impl Into<Locale>) -> String {
    try_i18n_with_locale(key, locale).unwrap()
}

/// Tries to fetch the translation corresponding to the given key and given locale in all saved i18n (current & saved)
pub fn try_i18n_with_locale(key: impl Into<Key>, locale: impl Into<Locale>) -> Option<String> {
    let locale = locale.into();
    let key = key.into();

    if let Some(ref current) = *CURRENT.read() {
        if current.locale() == &locale {
            return current.try_get(&key).cloned();
        }
    }

    DICTIONARY
        .read()
        .iter()
        .filter(|(l, _)| *l == &locale)
        .flat_map(|(_, i)| i.try_get(&key))
        .next()
        .cloned()
}

// Sets the current i18n, saving the current one if any
pub fn set(i18n: I18n) {
    if let Some(old) = CURRENT.write().take() {
        DICTIONARY.write().insert(old.locale().clone(), old);
    }

    *CURRENT.write() = Some(i18n)
}

/// Sets the current i18n if empty, else saves it
pub fn add(i18n: I18n) {
    if CURRENT.read().is_none() {
        *CURRENT.write() = Some(i18n);
        return;
    }

    DICTIONARY.write().insert(i18n.locale().clone(), i18n);
}

/// Removes the current i18n and all saved ones
pub fn free() {
    *CURRENT.write() = None;
    DICTIONARY.write().clear();
}

/// Tries to fetch the locale of the current i18n
pub fn try_get_locale() -> Option<Locale> {
    Some(CURRENT.read().as_ref()?.locale().clone())
}

pub struct LocaleNotFound;

/// Tries to swap the current i18n with a saved one corresponding to the given Locale
pub fn set_language(locale: impl Into<Locale>) -> Result<(), LocaleNotFound> {
    let locale = locale.into();
    if let Some(ref current) = *CURRENT.read() {
        if current.locale() == &locale {
            return Ok(());
        }
    }

    if let Some(new) = DICTIONARY.write().remove(&locale) {
        set(new)
    }

    Ok(())
}
