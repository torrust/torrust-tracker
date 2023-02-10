use derive_more::{Deref, Display, From, Into};
use thiserror::Error;

#[derive(Error, Debug, Display, From, Into, Deref)]
pub struct IoError {
    pub(crate) repr: std::io::Error,
}

impl std::hash::Hash for IoError {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.kind().hash(state);
    }
}

impl Eq for IoError {}

impl PartialEq for IoError {
    fn eq(&self, other: &Self) -> bool {
        self.repr.kind() == other.repr.kind()
    }
}

impl PartialOrd for IoError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.repr.kind().partial_cmp(&other.repr.kind())
    }
}

#[derive(Error, Debug, Display, From, Into, Deref)]
pub struct SerdeJsonError {
    pub(crate) repr: serde_json::Error,
}

impl std::hash::Hash for SerdeJsonError {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.to_string().hash(state);
    }
}

impl Eq for SerdeJsonError {}

impl PartialEq for SerdeJsonError {
    fn eq(&self, other: &Self) -> bool {
        self.repr.to_string() == other.repr.to_string()
    }
}

impl PartialOrd for SerdeJsonError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.repr.to_string().partial_cmp(&other.repr.to_string())
    }
}

#[derive(Error, Clone, Debug, Eq, Display, From, Into, Deref)]
pub struct TomlDeError {
    pub(crate) repr: toml::de::Error,
}

impl PartialEq for TomlDeError {
    fn eq(&self, other: &Self) -> bool {
        self.repr.to_string() == other.repr.to_string()
    }
}

impl std::hash::Hash for TomlDeError {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.repr.to_string().hash(state);
    }
}
