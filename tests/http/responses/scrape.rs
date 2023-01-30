use std::collections::HashMap;
use std::str;

use serde::{self, Deserialize, Serialize};
use serde_bencode::value::Value;

use crate::http::bencode::ByteArray20;

#[derive(Debug, PartialEq)]
pub struct Response {
    pub files: HashMap<ByteArray20, File>,
}

impl Response {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let scrape_response: DeserializedResponse = serde_bencode::from_bytes(bytes).unwrap();
        Self::from(scrape_response)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct File {
    pub complete: i64,
    pub downloaded: i64,
    pub incomplete: i64,
}

impl From<DeserializedResponse> for Response {
    fn from(scrape_response: DeserializedResponse) -> Self {
        // todo:
        // - Use `try_from` trait instead of `from`.
        // - Improve error messages.
        // - Extract parser function out of the trait.
        // - Extract parser for each nested element.
        // - Extract function to instantiate [u8; 20] from Vec<u8>.
        let mut files: HashMap<ByteArray20, File> = HashMap::new();

        match scrape_response.files {
            Value::Dict(dict) => {
                for file_element in dict {
                    let info_hash_byte_vec = file_element.0;
                    let file_value = file_element.1;

                    let file = match &file_value {
                        Value::Dict(dict) => {
                            let mut file = File {
                                complete: 0,
                                downloaded: 0,
                                incomplete: 0,
                            };

                            for file_field in dict {
                                let value = match file_field.1 {
                                    Value::Int(number) => *number,
                                    _ => panic!("Error parsing bencoded scrape response. Invalid value. Expected <i64>"),
                                };

                                if file_field.0 == b"complete" {
                                    file.complete = value;
                                } else if file_field.0 == b"downloaded" {
                                    file.downloaded = value;
                                } else if file_field.0 == b"incomplete" {
                                    file.incomplete = value;
                                } else {
                                    panic!("Error parsing bencoded scrape response. Invalid <File> field");
                                }
                            }

                            file
                        }
                        _ => panic!("Error parsing bencoded scrape response. Invalid value. Expected <Value::Dict>"),
                    };

                    // Clone Vec<u8> into [u8; 20]
                    let mut info_hash_byte_array: [u8; 20] = Default::default();
                    info_hash_byte_array.clone_from_slice(info_hash_byte_vec.as_slice());

                    files.insert(info_hash_byte_array, file);
                }
            }
            _ => panic!("Error parsing bencoded scrape response. Invalid value. Expected <Value::Dict>"),
        }

        Self { files }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DeserializedResponse {
    pub files: Value,
}
