//! Copyright (c) 2020-2023 Joakim Frostegård and The Torrust Developers
//!
//! Distributed under Apache 2.0 license

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Announce events. Described on  the
/// [BEP 3. The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
#[derive(Hash, Clone, Copy, Debug, Display, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnounceEvent {
    /// The peer has started downloading the torrent.
    #[display(fmt = "started")]
    Started,
    /// The peer has ceased downloading the torrent.
    #[display(fmt = "stopped")]
    Stopped,
    /// The peer has completed downloading the torrent.
    #[display(fmt = "completed")]
    Completed,
    /// This is one of the announcements done at regular intervals.
    #[display(fmt = "")]
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
