use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Error {
    #[serde(rename = "failure reason")]
    pub failure_reason: String,
}
