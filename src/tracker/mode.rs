use derive_more::Display;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Display)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    // Will track every new info hash and serve every peer.
    Public,

    // Will only track whitelisted info hashes.
    #[default]
    Listed,

    // Will only serve authenticated peers
    Private,

    // Will only track whitelisted info hashes and serve authenticated peers
    PrivateListed,
}
