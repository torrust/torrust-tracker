//! Tracker operation mode types.
//!
//! This module contains the [`PrivateMode`] struct, which holds
//! configuration options that apply when the tracker operates in private mode.
use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};

/// Configuration that applies when the tracker is operating in private mode.
// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(
    clippy::redundant_field_names,
    reason = "derive_more::Constructor emits field initializers on this MSRV-compatible version"
)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, Constructor, Display)]
pub struct PrivateMode {
    /// A flag to check the expiration date of peer keys.
    ///
    /// When true (the default), a key that is not permanent is rejected once
    /// its expiration date has passed. When false, the expiration date is
    /// ignored and the key is accepted even if it has expired.
    #[serde(default = "PrivateMode::default_check_keys_expiration")]
    pub check_keys_expiration: bool,
}

impl Default for PrivateMode {
    fn default() -> Self {
        Self {
            check_keys_expiration: Self::default_check_keys_expiration(),
        }
    }
}

impl PrivateMode {
    const fn default_check_keys_expiration() -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::mode::PrivateMode;

    #[test]
    fn it_should_check_the_keys_expiration_by_default() {
        let mode = PrivateMode::default();

        assert_eq!(mode, PrivateMode::new(true));
    }

    #[test]
    fn it_should_check_the_keys_expiration_when_the_json_does_not_say_otherwise() {
        let mode: PrivateMode = serde_json::from_str("{}").expect("an empty object should deserialize");

        assert_eq!(mode, PrivateMode::new(true));
    }

    #[test]
    fn it_should_keep_an_explicit_choice_not_to_check_the_keys_expiration() {
        let mode: PrivateMode = serde_json::from_str(r#"{"check_keys_expiration": false}"#).expect("the flag should deserialize");

        assert_eq!(mode, PrivateMode::new(false));
    }

    #[test]
    fn it_should_serialize_to_the_configuration_key() {
        let mode = PrivateMode::new(false);

        let json = serde_json::to_string(&mode).expect("it should serialize");

        assert_eq!(json, r#"{"check_keys_expiration":false}"#);
    }
}
