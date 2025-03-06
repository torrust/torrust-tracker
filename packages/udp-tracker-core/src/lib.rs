pub mod connection_cookie;
pub mod container;
pub mod crypto;
pub mod services;
pub mod statistics;

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
