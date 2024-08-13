//! Copyright (c) 2020-2023 Joakim Frostegård and The Torrust Developers
//!
//! Distributed under Apache 2.0 license

use serde::{Deserialize, Serialize};

/// Announce events. Described on  the
/// [BEP 3. The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
#[derive(Hash, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnounceEvent {
    /// The peer has started downloading the torrent.
    Started,
    /// The peer has ceased downloading the torrent.
    Stopped,
    /// The peer has completed downloading the torrent.
    Completed,
    /// This is one of the announcements done at regular intervals.
    None,
}

impl AnnounceEvent {
    #[inline]
    #[must_use]
    pub fn from_i32(i: i32) -> Self {
        match i {
            1 => Self::Completed,
            2 => Self::Started,
            3 => Self::Stopped,
            _ => Self::None,
        }
    }

    #[inline]
    #[must_use]
    pub fn to_i32(&self) -> i32 {
        match self {
            AnnounceEvent::None => 0,
            AnnounceEvent::Completed => 1,
            AnnounceEvent::Started => 2,
            AnnounceEvent::Stopped => 3,
        }
    }
}
