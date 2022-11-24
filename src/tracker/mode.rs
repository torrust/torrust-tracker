use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tracker {
    // Will track every new info hash and serve every peer.
    #[serde(rename = "public")]
    Public,

    // Will only track whitelisted info hashes.
    #[serde(rename = "listed")]
    Listed,

    // Will only serve authenticated peers
    #[serde(rename = "private")]
    Private,

    // Will only track whitelisted info hashes and serve authenticated peers
    #[serde(rename = "private_listed")]
    PrivateListed,
}
