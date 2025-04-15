#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Key(String);

impl From<String> for Key {
    fn from(value: String) -> Self {
        Key(value)
    }
}

impl From<&String> for Key {
    fn from(value: &String) -> Self {
        Key(value.clone())
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Key(value.to_string())
    }
}
