use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull};

/// This type contains the info needed to add a new tracker key.
///
/// You can upload a pre-generated key or let the app to generate a new one.
/// You can also set an expiration date or leave it empty (`None`) if you want
/// to create permanent key that does not expire.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct AddKeyForm {
    /// The pre-generated key. Use `None` (null in json) to generate a random key.
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(rename = "key")]
    pub opt_key: Option<String>,

    /// How long the key will be valid in seconds. Use `None` (null in json) for
    /// permanent keys.
    #[serde_as(deserialize_as = "DefaultOnNull")]
    #[serde(rename = "seconds_valid")]
    pub opt_seconds_valid: Option<u64>,
}
