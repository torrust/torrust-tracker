//! Shared supervisor of the token-aware server components.
//!
//! The HTTP tracker, REST API and health-check API components each own a server task and its drain
//! controller. [`supervise_token_aware_server`] races the component's cancellation token against the
//! server task, joins both owned children and maps the result to a [`ComponentResult`].
//!
//! Join order, on the cancelled and on the unexpected-stop branch: join the server task, always join
//! the drain controller, and only then map the errors. A failed server join therefore never leaves the
//! controller unjoined (dropped, hence aborted), and the drain outcome is not lost.
//!
//! The component name appears only in the error messages.

// issue: #1488
// ADR: docs/adrs/20260902074438_adopt_supervised_cancellation_tree_for_shutdown.md
use tokio_util::sync::CancellationToken;
use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;

use crate::bootstrap::jobs::manager::{ComponentCompletion, ComponentError, ComponentResult, TokenAwareServerTask};

/// Supervises a token-aware server component until it stops.
///
/// `runtime_error` turns the value the server task returned into its runtime error, if any; a server
/// that returns no result passes `|_| Ok(())`.
///
/// # Errors
///
/// Returns a [`ComponentError`] when a child fails to join, the server returns a runtime error, or the
/// graceful drain timed out.
pub async fn supervise_token_aware_server<T, E>(
    component: &str,
    mut server_task: TokenAwareServerTask<T, GracefulShutdownOutcome>,
    cancellation_token: CancellationToken,
    runtime_error: impl FnOnce(T) -> Result<(), E>,
) -> ComponentResult
where
    E: std::fmt::Display,
{
    tokio::select! {
        biased;
        () = cancellation_token.cancelled() => {
            let server_result = server_task.join().await;
            let drain_outcome = server_task.join_shutdown_controller().await;
            finish(component, server_result, drain_outcome, runtime_error, Stop::Cancelled)
        }
        server_result = server_task.join() => {
            cancellation_token.cancel();
            let drain_outcome = server_task.join_shutdown_controller().await;
            finish(component, server_result, drain_outcome, runtime_error, Stop::Unexpected)
        }
    }
}

#[derive(Clone, Copy)]
enum Stop {
    Cancelled,
    Unexpected,
}

/// Maps the joined results, once both children are joined.
fn finish<T, E>(
    component: &str,
    server_result: Result<T, tokio::task::JoinError>,
    drain_outcome: Result<GracefulShutdownOutcome, tokio::task::JoinError>,
    runtime_error: impl FnOnce(T) -> Result<(), E>,
    stop: Stop,
) -> ComponentResult
where
    E: std::fmt::Display,
{
    let drain_outcome =
        drain_outcome.map_err(|error| ComponentError::new(format!("{component} drain controller failed: {error}")))?;
    let (join_context, runtime_context) = match stop {
        Stop::Cancelled => (
            "server task join failed while stopping",
            "server runtime failed while stopping",
        ),
        Stop::Unexpected => ("server task join failed", "server runtime returned an error"),
    };
    let server_value = server_result.map_err(|error| ComponentError::new(format!("{component} {join_context}: {error}")))?;
    runtime_error(server_value).map_err(|error| ComponentError::new(format!("{component} {runtime_context}: {error}")))?;
    match drain_outcome {
        GracefulShutdownOutcome::Drained => Ok(match stop {
            Stop::Cancelled => ComponentCompletion::Cancelled,
            Stop::Unexpected => ComponentCompletion::Completed,
        }),
        GracefulShutdownOutcome::TimedOut => Err(ComponentError::new(format!("{component} graceful drain timed out"))),
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::sync::oneshot;
    use tokio::task::JoinHandle;
    use tokio_util::sync::CancellationToken;
    use torrust_tracker_axum_server::signals::GracefulShutdownOutcome;

    use super::supervise_token_aware_server;
    use crate::bootstrap::jobs::manager::{ComponentCompletion, ComponentResult, TokenAwareServerTask};

    const TEST_DEADLINE: Duration = Duration::from_secs(5);

    type ServerResult = Result<(), &'static str>;

    /// A case name, the server task, whether the token is cancelled first, the drain outcome, and the expected result.
    type Case = (
        &'static str,
        JoinHandle<ServerResult>,
        bool,
        GracefulShutdownOutcome,
        Result<ComponentCompletion, &'static str>,
    );

    /// A drain controller that reports `outcome` once released; `joined` is signalled when it finishes.
    fn drain_controller(
        cancellation_token: CancellationToken,
        outcome: GracefulShutdownOutcome,
    ) -> (JoinHandle<GracefulShutdownOutcome>, oneshot::Receiver<()>) {
        let (joined_sender, joined) = oneshot::channel();
        let handle = tokio::spawn(async move {
            cancellation_token.cancelled().await;
            let _ = joined_sender.send(());
            outcome
        });
        (handle, joined)
    }

    async fn run(
        server_task: JoinHandle<ServerResult>,
        cancel_first: bool,
        outcome: GracefulShutdownOutcome,
    ) -> (ComponentResult, oneshot::Receiver<()>) {
        let cancellation_token = CancellationToken::new();
        let (controller, joined) = drain_controller(cancellation_token.clone(), outcome);
        if cancel_first {
            cancellation_token.cancel();
        }
        let result = tokio::time::timeout(
            TEST_DEADLINE,
            supervise_token_aware_server(
                "test component",
                TokenAwareServerTask::new(server_task, controller),
                cancellation_token,
                |result: ServerResult| result,
            ),
        )
        .await
        .expect("the component should report within the test deadline");
        (result, joined)
    }

    #[tokio::test]
    async fn it_should_map_the_join_and_error_cases_after_joining_both_children() {
        // Arrange
        let drained = || GracefulShutdownOutcome::Drained;
        let timed_out = || GracefulShutdownOutcome::TimedOut;
        let cases: Vec<Case> = vec![
            (
                "cancelled and drained",
                tokio::spawn(async { Ok(()) }),
                true,
                drained(),
                Ok(ComponentCompletion::Cancelled),
            ),
            (
                "stopped independently and drained",
                tokio::spawn(async { Ok(()) }),
                false,
                drained(),
                Ok(ComponentCompletion::Completed),
            ),
            (
                "cancelled and drain timed out",
                tokio::spawn(async { Ok(()) }),
                true,
                timed_out(),
                Err("test component graceful drain timed out"),
            ),
            (
                "cancelled with a runtime error",
                tokio::spawn(async { Err("boom") }),
                true,
                drained(),
                Err("test component server runtime failed while stopping: boom"),
            ),
            (
                "stopped independently with a runtime error",
                tokio::spawn(async { Err("boom") }),
                false,
                drained(),
                Err("test component server runtime returned an error: boom"),
            ),
            (
                "cancelled with a server join error",
                tokio::spawn(async { panic!("server panic") }),
                true,
                drained(),
                Err("test component server task join failed while stopping"),
            ),
            (
                "stopped independently with a server join error",
                tokio::spawn(async { panic!("server panic") }),
                false,
                drained(),
                Err("test component server task join failed:"),
            ),
        ];

        for (name, server_task, cancel_first, outcome, expected) in cases {
            // Act
            let (result, joined) = run(server_task, cancel_first, outcome).await;

            // Assert
            match expected {
                Ok(completion) => assert_eq!(result, Ok(completion), "{name}"),
                Err(message) => {
                    let error = result.expect_err(name);
                    assert!(error.to_string().contains(message), "{name}: {error}");
                }
            }
            // The controller finished (was joined, not aborted) in every case, including the server-join errors.
            assert!(
                joined.await.is_ok(),
                "{name}: the drain controller must be joined, not dropped"
            );
        }
    }

    #[tokio::test]
    async fn it_should_report_a_drain_controller_join_error_before_a_server_error() {
        // Arrange
        let cancellation_token = CancellationToken::new();
        let controller: JoinHandle<GracefulShutdownOutcome> = tokio::spawn(async { panic!("controller panic") });
        let server_task: JoinHandle<ServerResult> = tokio::spawn(async { Err("boom") });
        cancellation_token.cancel();

        // Act
        let result = supervise_token_aware_server(
            "test component",
            TokenAwareServerTask::new(server_task, controller),
            cancellation_token,
            |result: ServerResult| result,
        )
        .await;

        // Assert
        let error = result.expect_err("a controller panic should fail the component");
        assert!(error.to_string().contains("test component drain controller failed"));
    }
}
