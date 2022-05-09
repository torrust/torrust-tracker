use serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum TrackerMode {
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
