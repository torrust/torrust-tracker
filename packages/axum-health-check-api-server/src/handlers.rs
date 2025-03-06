use std::collections::VecDeque;

use axum::extract::State;
use axum::Json;
use torrust_server_lib::registar::{ServiceHealthCheckJob, ServiceRegistration, ServiceRegistry};
use tracing::{instrument, Level};

use super::resources::{CheckReport, Report};
use super::responses;

/// Endpoint for container health check.
///
/// Creates a vector [`CheckReport`] from the input set of [`CheckJob`], and then builds a report from the results.
///
#[instrument(skip(register), ret(level = Level::DEBUG))]
pub(crate) async fn health_check_handler(State(register): State<ServiceRegistry>) -> Json<Report> {
    #[allow(unused_assignments)]
    let mut checks: VecDeque<ServiceHealthCheckJob> = VecDeque::new();

    {
        let mutex = register.lock();

        checks = mutex.await.values().map(ServiceRegistration::spawn_check).collect();
    }

    // if we do not have any checks, lets return a `none` result.
    if checks.is_empty() {
        return responses::none();
    }

    let jobs = checks.drain(..).map(|c| {
        tokio::spawn(async move {
            CheckReport {
                binding: c.binding,
                info: c.info.clone(),
                result: c.job.await.expect("it should be able to join into the checking function"),
            }
        })
    });

    let results: Vec<CheckReport> = futures::future::join_all(jobs)
        .await
        .drain(..)
        .map(|r| r.expect("it should be able to connect to the job"))
        .collect();

    if results.iter().any(CheckReport::fail) {
        responses::error("health check failed".to_string(), results)
    } else {
        responses::ok(results)
    }
}
