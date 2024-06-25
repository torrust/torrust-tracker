use std::collections::HashMap;
use std::fmt::Write;
use std::str;

use axum::body::Bytes;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_bencode::value::Value;
use torrust_tracker_primitives::info_hash::InfoHash;

use super::{BencodeParseError, Scrape};
use crate::shared::bit_torrent::tracker::http::ByteArray20;

#[derive(Debug, PartialEq, Eq, Default, Deserialize, Clone)]
pub(super) struct Response {
    pub files: HashMap<ByteArray20, File>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct File {
    pub complete: i64,   // The number of active peers that have completed downloading
    pub downloaded: i64, // The number of peers that have ever completed downloading
    pub incomplete: i64, // The number of active peers that have not completed downloading
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

// Custom serialization for Response
impl Serialize for Response {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.files.len()))?;
        for (key, value) in &self.files {
            // Convert ByteArray20 key to hex string
            let hex_key = byte_array_to_hex_string(key);
            map.serialize_entry(&hex_key, value)?;
        }
        map.end()
    }
}

// Helper function to convert ByteArray20 to hex string
fn byte_array_to_hex_string(byte_array: &ByteArray20) -> String {
    let mut hex_string = String::with_capacity(byte_array.len() * 2);
    for byte in byte_array {
        write!(hex_string, "{byte:02x}").expect("Writing to string should never fail");
    }
    hex_string
}

pub type OneFile = (ByteArray20, File);

#[derive(Default)]
pub struct ResponseBuilder {
    response: Response,
}

impl From<OneFile> for ResponseBuilder {
    fn from((infohash, file): OneFile) -> Self {
        let mut files: HashMap<ByteArray20, File> = HashMap::new();
        files.insert(infohash, file);

        Self {
            response: Response { files },
        }
    }
}

impl TryFrom<&Bytes> for ResponseBuilder {
    type Error = BencodeParseError;

    /// # Errors
    ///
    /// Will return an error if the deserialized bencoded response can't not be converted into a valid response.
    fn try_from(value: &Bytes) -> Result<Self, Self::Error> {
        let scrape_response: DeserializedResponse =
            serde_bencode::from_bytes(value).map_err(|e| BencodeParseError::ParseSerdeBencodeError {
                data: value.to_vec(),
                err: e.into(),
            })?;

        Ok(Self {
            response: Response::try_from(scrape_response)?,
        })
    }
}

impl ResponseBuilder {
    #[must_use]
    pub fn add_file(mut self, (infohash, file): OneFile) -> Self {
        self.response.files.insert(infohash, file);
        self
    }

    #[must_use]
    pub fn build(self) -> Scrape {
        self.response.into()
    }
}

/// It parses a bencoded scrape response into a `Response` struct.
///
/// For example:
///
/// ```text
/// d5:filesd20:xxxxxxxxxxxxxxxxxxxxd8:completei11e10:downloadedi13772e10:incompletei19e
/// 20:yyyyyyyyyyyyyyyyyyyyd8:completei21e10:downloadedi206e10:incompletei20eee
/// ```
///
/// Response (JSON encoded for readability):
///
/// ```text
/// {
///   'files': {
///     'xxxxxxxxxxxxxxxxxxxx': {'complete': 11, 'downloaded': 13772, 'incomplete': 19},
///     'yyyyyyyyyyyyyyyyyyyy': {'complete': 21, 'downloaded': 206, 'incomplete': 20}
///   }
/// }
fn parse_bencoded_response(value: &Value) -> Result<Response, BencodeParseError> {
    let mut files: HashMap<ByteArray20, File> = HashMap::new();

    match value {
        Value::Dict(dict) => {
            for file_element in dict {
                let info_hash_byte_vec = file_element.0;
                let file_value = file_element.1;

                let file = parse_bencoded_file(file_value)?;

                files.insert(
                    InfoHash::try_from(info_hash_byte_vec.clone())
                        .map_err(|_| BencodeParseError::InvalidFileField {
                            value: Value::Bytes(info_hash_byte_vec.clone()),
                        })?
                        .bytes(),
                    file,
                );
            }
        }
        _ => return Err(BencodeParseError::InvalidValueExpectedDict { value: value.clone() }),
    }

    Ok(Response { files })
}

/// It parses a bencoded dictionary into a `File` struct.
///
/// For example:
///
///
/// ```text
/// d8:completei11e10:downloadedi13772e10:incompletei19ee
/// ```
///
/// into:
///
/// ```text
/// File {
///     complete: 11,
///     downloaded: 13772,
///     incomplete: 19,
/// }
/// ```
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

            if complete.is_none() {
                return Err(BencodeParseError::MissingFileField {
                    field_name: "complete".to_string(),
                });
            }

            if downloaded.is_none() {
                return Err(BencodeParseError::MissingFileField {
                    field_name: "downloaded".to_string(),
                });
            }

            if incomplete.is_none() {
                return Err(BencodeParseError::MissingFileField {
                    field_name: "incomplete".to_string(),
                });
            }

            File {
                complete: complete.unwrap(),
                downloaded: downloaded.unwrap(),
                incomplete: incomplete.unwrap(),
            }
        }
        _ => return Err(BencodeParseError::InvalidValueExpectedDict { value: value.clone() }),
    };

    Ok(file)
}
