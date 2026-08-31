use axum::Json;
use axum::extract::State;
use torrust_server_lib::registar::Registar;
use torrust_tracker_primitives::RuntimeServiceMetadata;
use tracing::{Level, instrument};

use super::resources::{CheckReport, Report};
use super::responses;

/// Endpoint for container health check.
///
/// Creates a vector [`CheckReport`] from the input set of [`CheckJob`], and then builds a report from the results.
///
#[instrument(skip(registar), ret(level = Level::DEBUG))]
pub(crate) async fn health_check_handler(State(registar): State<Registar<RuntimeServiceMetadata>>) -> Json<Report> {
    let mut checks: Vec<_> = registar
        .services()
        .await
        .into_iter()
        .filter_map(|service| {
            service.spawn_check().map(|health_check| {
                (
                    service.service_binding().clone(),
                    service.metadata().service_role().as_str().to_string(),
                    service.metadata().public_url().map(str::to_string),
                    health_check,
                )
            })
        })
        .collect();

    // if we do not have any checks, lets return a `none` result.
    if checks.is_empty() {
        return responses::none();
    }

    let jobs = checks
        .drain(..)
        .map(|(service_binding, service_type, public_url, health_check)| {
            tokio::spawn(async move {
                CheckReport {
                    service_binding: service_binding.url(),
                    binding: service_binding.bind_address(),
                    info: health_check.info,
                    service_type,
                    public_url,
                    result: health_check
                        .job
                        .await
                        .expect("it should be able to join into the checking function"),
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
