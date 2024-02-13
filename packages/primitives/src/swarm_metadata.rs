use derive_more::Constructor;

/// Swarm statistics for one torrent.
/// Swarm metadata dictionary in the scrape response.
///
/// See [BEP 48: Tracker Protocol Extension: Scrape](https://www.bittorrent.org/beps/bep_0048.html)
#[derive(Copy, Clone, Debug, PartialEq, Default, Constructor)]
pub struct SwarmMetadata {
    /// (i.e `completed`): The number of peers that have ever completed downloading
    pub downloaded: u32, //
    /// (i.e `seeders`): The number of active peers that have completed downloading (seeders)
    pub complete: u32, //seeders
    /// (i.e `leechers`): The number of active peers that have not completed downloading (leechers)
    pub incomplete: u32,
}

impl SwarmMetadata {
    #[must_use]
    pub fn zeroed() -> Self {
        Self::default()
    }
}
