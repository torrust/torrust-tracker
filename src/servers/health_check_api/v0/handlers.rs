use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use tokio::task::JoinSet;

use super::resources::{CheckReport, Report};
use super::responses;
use crate::servers::registar::{HeathCheckFuture, Registration, Registry};

/// Endpoint for container health check.
///
/// Creates a vector [`CheckReport`] from the input set of [`CheckJob`], and then builds a report from the results.
///
pub(crate) async fn handle_health_check(State(registry): State<Arc<Registry>>) -> Json<Report> {
    #[allow(unused_assignments)]
    let mut checks = JoinSet::new();

    {
        let db = registry.lock().expect("it should get a lock");

        let mut tasks: Vec<HeathCheckFuture<'_>> = db.values().map(Registration::check_task).collect();

        let aborts = tasks.drain(..).map(|c| checks.spawn(c)).collect::<Vec<_>>();

        // if we do not have any checks, lets return a `none` result.
        if aborts.is_empty() {
            return responses::none();
        }
    }

    let mut results = Vec::default();

    while let Some(r) = checks.join_next().await {
        let result = r.expect("it should join task");

        results.push(CheckReport::from(result));
    }

    if results.iter().any(CheckReport::fail) {
        responses::error("health check failed".to_string(), results)
    } else {
        responses::ok(results)
    }
}
