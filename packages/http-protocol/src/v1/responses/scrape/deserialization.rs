//! `Scrape` response deserialization for the HTTP tracker.
//!
//! Types for deserializing scrape responses from an HTTP tracker.
use std::collections::HashMap;
use std::str;

use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_bencode::value::Value;
use thiserror::Error;
use torrust_info_hash::InfoHash;

#[derive(Debug, PartialEq, Eq, Default, Deserialize)]
pub struct Response {
    pub files: HashMap<InfoHash, File>,
}

impl Response {
    #[must_use]
    pub fn with_one_file(info_hash: InfoHash, file: File) -> Self {
        let mut files: HashMap<InfoHash, File> = HashMap::new();
        files.insert(info_hash, file);
        Self { files }
    }

    /// # Errors
    ///
    /// Will return an error if the deserialized bencoded response cannot be converted into a valid response.
    pub fn try_from_bencoded(bytes: &[u8]) -> Result<Self, BencodeParseError> {
        let scrape_response: DeserializedResponse =
            serde_bencode::from_bytes(bytes).map_err(|source| BencodeParseError::DeserializationError { source })?;
        Self::try_from(scrape_response)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct File {
    pub complete: i64,
    pub downloaded: i64,
    pub incomplete: i64,
}

impl File {
    #[must_use]
    pub fn zeroed() -> Self {
        Self::default()
    }
}

impl TryFrom<DeserializedResponse> for Response {
    type Error = BencodeParseError;

    fn try_from(scrape_response: DeserializedResponse) -> Result<Self, Self::Error> {
        parse_bencoded_response(&scrape_response.files)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DeserializedResponse {
    pub files: Value,
}

impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.files.len()))?;
        for (key, value) in &self.files {
            let hex_key = hex::encode(key.bytes());
            map.serialize_entry(&hex_key, value)?;
        }
        map.end()
    }
}

#[derive(Default)]
pub struct ResponseBuilder {
    response: Response,
}

impl ResponseBuilder {
    #[must_use]
    pub fn add_file(mut self, info_hash: InfoHash, file: File) -> Self {
        self.response.files.insert(info_hash, file);
        self
    }

    #[must_use]
    pub fn build(self) -> Response {
        self.response
    }
}

#[derive(Debug, Error)]
pub enum BencodeParseError {
    #[error("failed to deserialize bencoded scrape response: {source}")]
    DeserializationError { source: serde_bencode::Error },

    #[error("invalid value: expected dictionary, got: {value:?}")]
    InvalidValueExpectedDict { value: Value },

    #[error("invalid value: expected integer, got: {value:?}")]
    InvalidValueExpectedInt { value: Value },

    #[error("invalid file field in scrape response: {value:?}")]
    InvalidFileField { value: Value },

    #[error("missing required scrape file field: {field_name}")]
    MissingFileField { field_name: String },
}

/// It parses a bencoded scrape response into a `Response` struct.
fn parse_bencoded_response(value: &Value) -> Result<Response, BencodeParseError> {
    let mut files: HashMap<InfoHash, File> = HashMap::new();

    match value {
        Value::Dict(dict) => {
            for file_element in dict {
                let info_hash_bytes = file_element.0;
                let file_value = file_element.1;

                let file = parse_bencoded_file(file_value)?;

                let info_hash = InfoHash::from(info_hash_bytes.as_slice());

                files.insert(info_hash, file);
            }
        }
        _ => return Err(BencodeParseError::InvalidValueExpectedDict { value: value.clone() }),
    }

    Ok(Response { files })
}

/// It parses a bencoded dictionary into a `File` struct.
fn parse_bencoded_file(value: &Value) -> Result<File, BencodeParseError> {
    let file = match &value {
        Value::Dict(dict) => {
            let mut complete = None;
            let mut downloaded = None;
            let mut incomplete = None;

            for file_field in dict {
                let field_name = file_field.0;

                let field_value = match file_field.1 {
                    Value::Int(number) => Ok(*number),
                    _ => Err(BencodeParseError::InvalidValueExpectedInt {
                        value: file_field.1.clone(),
                    }),
                }?;

                if field_name == b"complete" {
                    complete = Some(field_value);
                } else if field_name == b"downloaded" {
                    downloaded = Some(field_value);
                } else if field_name == b"incomplete" {
                    incomplete = Some(field_value);
                } else {
                    return Err(BencodeParseError::InvalidFileField {
                        value: file_field.1.clone(),
                    });
                }
            }

            File {
                complete: complete.ok_or_else(|| BencodeParseError::MissingFileField {
                    field_name: "complete".to_string(),
                })?,
                downloaded: downloaded.ok_or_else(|| BencodeParseError::MissingFileField {
                    field_name: "downloaded".to_string(),
                })?,
                incomplete: incomplete.ok_or_else(|| BencodeParseError::MissingFileField {
                    field_name: "incomplete".to_string(),
                })?,
            }
        }
        _ => return Err(BencodeParseError::InvalidValueExpectedDict { value: value.clone() }),
    };

    Ok(file)
}
