#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Locale(String);

impl From<String> for Locale {
    fn from(value: String) -> Self {
        Locale(value)
    }
}

impl From<&String> for Locale {
    fn from(value: &String) -> Self {
        Locale(value.clone())
    }
}

impl From<&str> for Locale {
    fn from(value: &str) -> Self {
        Locale(value.to_string())
    }
}

impl std::fmt::Display for Locale{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

