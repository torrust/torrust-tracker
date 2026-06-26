//! Use-case service for tracker statistics API operations.
//!
//! Orchestrates calls to the [`StatsQueryPort`] to retrieve tracker metrics.
use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::{LabeledStats, Stats};

use crate::ports::stats::StatsQueryPort;

/// Use-case service for stats-related API operations.
///
/// Delegates to a [`StatsQueryPort`] implementation (tracker adapter).
pub struct StatsApiService {
    query_port: Box<dyn StatsQueryPort>,
}

impl StatsApiService {
    /// Creates a new service backed by the given port implementation.
    #[must_use]
    pub fn new(query_port: Box<dyn StatsQueryPort>) -> Self {
        Self { query_port }
    }

    /// Returns the global tracker statistics.
    pub async fn get_stats(&self) -> Stats {
        self.query_port.get_stats().await
    }

    /// Returns extended labeled metrics from all tracker subsystems.
    pub async fn get_labeled_stats(&self) -> LabeledStats {
        self.query_port.get_labeled_stats().await
    }
}
