//! Peer ID types.
//!
//! **Deprecated**: import from [`torrust_peer_id`] instead.
//! This module is kept for backwards compatibility and will be removed in a
//! future release. Removal is tracked as a follow-up cleanup subissue of EPIC
//! [#1669](https://github.com/torrust/torrust-tracker/issues/1669).

#[deprecated(
    since = "3.0.0-develop",
    note = "import peer ID types from `torrust_peer_id` crate instead; \
            this module will be removed in a future release (see EPIC #1669)"
)]
pub use torrust_peer_id::{PeerClient, PeerId};
