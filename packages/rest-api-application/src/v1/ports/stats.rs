//! Port trait for querying tracker statistics.
//!
//! Defines the boundary between the application layer and the
//! tracker-internal statistics aggregation. Implementations
//! live in the runtime adapter package.
use async_trait::async_trait;
use torrust_tracker_rest_api_protocol::v1::context::stats::resources::stats::{LabeledStats, Stats};

/// Port for querying tracker statistics.
///
/// Implementations of this trait aggregate data from all tracker-internal
/// repositories and services into protocol-level DTOs.
// `async_trait` applies `#[must_use]` to generated futures. Nightly Clippy also treats those
// futures as must-use and reports the macro expansion as redundant.
#[allow(clippy::double_must_use)]
#[async_trait]
pub trait StatsQueryPort: Send + Sync {
    /// Returns the global tracker statistics (unlabeled).
    async fn get_stats(&self) -> Stats;

    /// Returns extended labeled metrics from all tracker subsystems.
    async fn get_labeled_stats(&self) -> LabeledStats;
}
