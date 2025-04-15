#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct I18n {
    locale: crate::Locale,
    map: ahash::AHashMap<crate::Key, String>,
}

impl I18n {
    pub fn new(locale: crate::Locale, map: ahash::AHashMap<crate::Key, String>) -> Self{
        Self{locale, map}
    }

    pub fn get(&self, key: crate::Key) -> &String {
        self.try_get(key).unwrap()
    }
    pub fn try_get(&self, key: crate::Key) -> Option<&String> {
        self.map.get(&key)
    }

    pub fn locale(&self) -> &crate::Locale {
        &self.locale
    }
}
