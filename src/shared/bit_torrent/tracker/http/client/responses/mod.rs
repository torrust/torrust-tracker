use std::sync::Arc;

use derive_more::From;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod announce;
pub mod error;
pub mod scrape;

#[derive(Serialize, Deserialize, Debug, From, PartialEq, Eq, Clone)]
pub struct Announce {
    #[serde(flatten)]
    response: announce::Response,
}

#[derive(Serialize, Deserialize, Debug, From, PartialEq, Eq, Clone)]
pub struct Scrape {
    #[serde(flatten)]
    response: scrape::Response,
}

#[derive(Debug, Error, Clone)]
pub enum BencodeParseError {
    #[error("Invalid Value in Dictionary: {value:?}")]
    InvalidValueExpectedDict { value: serde_bencode::value::Value },
    #[error("Invalid Value in Integer: {value:?}")]
    InvalidValueExpectedInt { value: serde_bencode::value::Value },
    #[error("Invalid File Field: {value:?}")]
    InvalidFileField { value: serde_bencode::value::Value },
    #[error("Missing File Field: {field_name}")]
    MissingFileField { field_name: String },
    #[error("Failed to deserialize the serde bencoded response data with the error: \"{err:?}\"")]
    ParseSerdeBencodeError { data: Vec<u8>, err: Arc<serde_bencode::Error> },
}
