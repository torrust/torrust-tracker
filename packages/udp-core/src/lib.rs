pub mod connection_cookie;
pub mod container;
pub mod crypto;
pub mod event;
pub mod peer_builder;
pub mod services;
pub mod statistics;

use torrust_clock::clock;

/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;

use crypto::ephemeral_instance_keys;
use tracing::instrument;

pub const UDP_TRACKER_LOG_TARGET: &str = "UDP TRACKER";

/// Controls whether the UDP tracker validates the connection ID supplied by
/// clients in announce and scrape requests.
///
/// This mirrors [`torrust_tracker_configuration::v3_0_0::udp_tracker_server::ConnectionIdValidationPolicy`]
/// but lives in `udp-core` so that the service layer does not need to depend on
/// the configuration crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionIdValidationPolicy {
    /// Preserve all existing connection ID validation. This is the secure default.
    #[default]
    Strict,
    /// Skip connection ID validation for announce and scrape requests.
    /// Cookie-error metrics are still emitted; IP-ban counters are not incremented.
    Disabled,
}

/// It initializes the static values.
#[instrument(skip())]
pub fn initialize_static() {
    // Initialize the Ephemeral Instance Random Seed
    std::sync::LazyLock::force(&ephemeral_instance_keys::RANDOM_SEED);

    // Initialize the Ephemeral Instance Random Cipher
    std::sync::LazyLock::force(&ephemeral_instance_keys::RANDOM_CIPHER_BLOWFISH);

    // Initialize the Zeroed Cipher
    std::sync::LazyLock::force(&ephemeral_instance_keys::ZEROED_TEST_CIPHER_BLOWFISH);
}

#[cfg(test)]
pub(crate) mod tests {
    use torrust_info_hash::InfoHash;

    /// # Panics
    ///
    /// Will panic if the string representation of the info hash is not a valid info hash.
    #[must_use]
    pub fn sample_info_hash() -> InfoHash {
        "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0" // DevSkim: ignore DS173237
            .parse::<InfoHash>()
            .expect("String should be a valid info hash")
    }
}
