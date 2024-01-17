use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Error {
    #[serde(rename = "failure reason")]
    pub failure_reason: String,
}
