use aquatic_udp_protocol::TransactionId;
use bittorrent_primitives::info_hash::InfoHash;
use rand::prelude::*;

/// Returns a random info hash.
pub fn random_info_hash() -> InfoHash {
    let mut rng = rand::rng();
    let random_bytes: [u8; 20] = rng.random();

    InfoHash::from_bytes(&random_bytes)
}

/// Returns a random transaction id.
pub fn random_transaction_id() -> TransactionId {
    let random_value = rand::rng().random();
    TransactionId::new(random_value)
}
