use bittorrent_primitives::info_hash::InfoHash;
use rand::prelude::*;

pub fn invalid_info_hashes() -> Vec<String> {
    [
        "0".to_string(),
        "-1".to_string(),
        "1.1".to_string(),
        "INVALID INFOHASH".to_string(),
        "9c38422213e30bff212b30c360d26f9a0213642".to_string(), // 39-char length instead of 40. DevSkim: ignore DS173237
        "9c38422213e30bff212b30c360d26f9a0213642&".to_string(), // Invalid char
    ]
    .to_vec()
}

/// Returns a random info hash.
pub fn random_info_hash() -> InfoHash {
    let mut rng = rand::rng();
    let random_bytes: [u8; 20] = rng.random();

    InfoHash::from_bytes(&random_bytes)
}
