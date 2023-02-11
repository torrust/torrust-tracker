use std::collections::BTreeMap;

pub mod auth_key;
pub mod peer;
pub mod stats;
pub mod torrent;

pub type ApiTokens = BTreeMap<String, String>;
