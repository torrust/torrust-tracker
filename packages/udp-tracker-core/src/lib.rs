pub mod connection_cookie;
pub mod container;
pub mod crypto;
pub mod event;
pub mod services;
pub mod statistics;

use torrust_tracker_clock::clock;

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

#[macro_use]
extern crate lazy_static;

/// The maximum number of connection id errors per ip. Clients will be banned if
/// they exceed this limit.
pub const MAX_CONNECTION_ID_ERRORS_PER_IP: u32 = 10;

pub const UDP_TRACKER_LOG_TARGET: &str = "UDP TRACKER";

/// It initializes the static values.
#[instrument(skip())]
pub fn initialize_static() {
    // Initialize the Ephemeral Instance Random Seed
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_SEED);

    // Initialize the Ephemeral Instance Random Cipher
    lazy_static::initialize(&ephemeral_instance_keys::RANDOM_CIPHER_BLOWFISH);

    // Initialize the Zeroed Cipher
    lazy_static::initialize(&ephemeral_instance_keys::ZEROED_TEST_CIPHER_BLOWFISH);
}

#[cfg(test)]
pub(crate) mod tests {
    use bittorrent_primitives::info_hash::InfoHash;

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
