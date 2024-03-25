use std::ops::AddAssign;

/// Structure that holds general `Tracker` torrents metrics.
///
/// Metrics are aggregate values for all torrents.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct TorrentsMetrics {
    /// Total number of seeders for all torrents
    pub complete: u64,
    /// Total number of peers that have ever completed downloading for all torrents.
    pub downloaded: u64,
    /// Total number of leechers for all torrents.
    pub incomplete: u64,
    /// Total number of torrents.
    pub torrents: u64,
}

impl AddAssign for TorrentsMetrics {
    fn add_assign(&mut self, rhs: Self) {
        self.complete += rhs.complete;
        self.downloaded += rhs.downloaded;
        self.incomplete += rhs.incomplete;
        self.torrents += rhs.torrents;
    }
}
