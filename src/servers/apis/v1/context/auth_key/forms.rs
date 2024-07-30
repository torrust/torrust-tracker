use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddKeyForm {
    #[serde(rename = "key")]
    pub opt_key: Option<String>,
    pub seconds_valid: u64,
}
