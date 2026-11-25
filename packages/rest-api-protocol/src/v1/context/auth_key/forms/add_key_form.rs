//! Form for adding a new authentication key.
//!
//! This is the input DTO for the `POST /api/v1/keys` endpoint.
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::{DefaultOnNull, serde_as};

/// This type contains the info needed to add a new tracker key.
///
/// You can upload a pre-generated key or let the app to generate a new one.
/// You can also set an expiration date or leave it empty (`None`) if you want
/// to create a permanent key that does not expire.
#[serde_as]
#[derive(Serialize, Deserialize)]
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

/// The uploaded key is a secret, so `Debug` redacts it.
impl fmt::Debug for AddKeyForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddKeyForm")
            .field("opt_key", &self.opt_key.as_ref().map(|_| "[REDACTED]"))
            .field("opt_seconds_valid", &self.opt_seconds_valid)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::AddKeyForm;

    #[test]
    fn it_should_not_print_the_uploaded_key_in_its_debug_output() {
        let secret = "TestPeerKeyInTheAddKeyForm000001";
        let form = AddKeyForm {
            opt_key: Some(secret.to_string()),
            opt_seconds_valid: Some(60),
        };

        let debug = format!("{form:?}");

        assert!(!debug.contains(secret), "the Debug output holds the test secret");
        assert!(
            debug.contains("opt_seconds_valid"),
            "the Debug output should still show the other field: {debug}"
        );
    }
}
