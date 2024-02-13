use std::ops::AddAssign;

/// Structure that holds general `Tracker` torrents metrics.
///
/// Metrics are aggregate values for all torrents.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct TorrentsMetrics {
    /// Total number of seeders for all torrents
    pub seeders: u64,
    /// Total number of peers that have ever completed downloading for all torrents.
    pub completed: u64,
    /// Total number of leechers for all torrents.
    pub leechers: u64,
    /// Total number of torrents.
    pub torrents: u64,
}

impl AddAssign for TorrentsMetrics {
    fn add_assign(&mut self, rhs: Self) {
        self.seeders += rhs.seeders;
        self.completed += rhs.completed;
        self.leechers += rhs.leechers;
        self.torrents += rhs.torrents;
    }
}
