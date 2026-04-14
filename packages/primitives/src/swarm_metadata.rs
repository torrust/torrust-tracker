use std::ops::AddAssign;

use derive_more::Constructor;

use crate::NumberOfDownloads;

/// Swarm statistics for one torrent.
///
/// Swarm metadata dictionary in the scrape response.
///
/// See [BEP 48: Tracker Protocol Extension: Scrape](https://www.bittorrent.org/beps/bep_0048.html)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Constructor)]
pub struct SwarmMetadata {
    /// (i.e `completed`): The number of peers that have ever completed
    /// downloading a given torrent.
    ///
    /// This uses `u64` because it is persisted as a cumulative counter and can
    /// grow independently from the active peer counts below.
    pub downloaded: NumberOfDownloads,

    /// (i.e `seeders`): The number of active peers that have completed
    /// downloading (seeders) a given torrent.
    ///
    /// Active peer counts stay bounded by the swarm population, so `u32`
    /// remains sufficient here.
    pub complete: u32,

    /// (i.e `leechers`): The number of active peers that have not completed
    /// downloading (leechers) a given torrent.
    ///
    /// Active peer counts stay bounded by the swarm population, so `u32`
    /// remains sufficient here.
    pub incomplete: u32,
}

impl SwarmMetadata {
    #[must_use]
    pub fn zeroed() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn downloads(&self) -> NumberOfDownloads {
        self.downloaded
    }

    #[must_use]
    pub fn seeders(&self) -> u32 {
        self.complete
    }

    #[must_use]
    pub fn leechers(&self) -> u32 {
        self.incomplete
    }
}

/// Structure that holds aggregate swarm metadata.
///
/// Metrics are aggregate values for all active torrents/swarms.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct AggregateActiveSwarmMetadata {
    /// Total number of peers that have ever completed downloading.
    pub total_downloaded: u64,

    /// Total number of seeders.
    pub total_complete: u64,

    /// Total number of leechers.
    pub total_incomplete: u64,

    /// Total number of torrents.
    pub total_torrents: u64,
}

impl AddAssign for AggregateActiveSwarmMetadata {
    fn add_assign(&mut self, rhs: Self) {
        self.total_complete += rhs.total_complete;
        self.total_downloaded += rhs.total_downloaded;
        self.total_incomplete += rhs.total_incomplete;
        self.total_torrents += rhs.total_torrents;
    }
}
