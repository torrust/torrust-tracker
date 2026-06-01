//! Tracker operation mode types.
//!
//! This module contains the [`PrivateMode`] struct, which holds
//! configuration options that apply when the tracker operates in private mode.
use derive_more::{Constructor, Display};
use serde::{Deserialize, Serialize};

/// Configuration that applies when the tracker is operating in private mode.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy, Constructor, Display)]
pub struct PrivateMode {
    /// A flag to disable expiration date for peer keys.
    ///
    /// When true, if the keys is not permanent the expiration date will be
    /// ignored. The key will be accepted even if it has expired.
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
    fn default_check_keys_expiration() -> bool {
        true
    }
}
