//! Application-level background-component supervision.
//!
//! `JobManager` owns only direct components registered during application
//! bootstrap. New components must use [`JobManager::spawn`]; each component,
//! rather than this supervisor, owns and completes or deliberately aborts its
//! nested tasks. Shutdown requests cooperative cancellation, waits under one
//! shared deadline, then aborts and joins any remaining direct components.
//!
//! issue: #1586

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::sync::oneshot::Sender;
use tokio::task::{Id, JoinError, JoinHandle, JoinSet};
use tokio::time::{Instant, timeout_at};
use tokio_util::sync::CancellationToken;
use torrust_server_lib::signals::Halted;
use tracing::{info, warn};

/// A component's explicit terminal result.
///
/// Component runners must return this result instead of inferring their state
/// from the supervisor's cancellation token.
pub type ComponentResult = Result<ComponentCompletion, ComponentError>;

/// A normal terminal state reported by a component runner.
#[derive(Debug, Eq, PartialEq)]
pub enum ComponentCompletion {
    Completed,
    Cancelled,
}

/// Context supplied by a component that stopped unsuccessfully.
#[derive(Debug, Eq, PartialEq)]
pub struct ComponentError {
    message: String,
}

impl ComponentError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl std::fmt::Display for ComponentError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ComponentError {}

/// The terminal state of a named top-level component task.
#[derive(Debug, Eq, PartialEq)]
pub enum JobStatus {
    Completed,
    Cancelled,
    Failed(ComponentError),
    Panicked(String),
    Aborted,
}

/// The named terminal result of a top-level component task.
#[derive(Debug, Eq, PartialEq)]
pub struct JobOutcome {
    pub name: String,
    pub status: JobStatus,
}

/// Delivers a normal halt signal when its component is dropped.
#[derive(Debug)]
pub struct HaltSignal {
    sender: Option<Sender<Halted>>,
}

impl HaltSignal {
    #[must_use]
    pub const fn new(sender: Sender<Halted>) -> Self {
        Self { sender: Some(sender) }
    }

    /// Signals normal shutdown. It is harmless when shutdown was already signalled.
    ///
    /// # Errors
    ///
    /// Returns the unsent signal when the receiver has already been dropped.
    pub fn signal_shutdown(&mut self) -> Result<(), Halted> {
        self.sender.take().map_or(Ok(()), |sender| sender.send(Halted::Normal))
    }
}

impl Drop for HaltSignal {
    fn drop(&mut self) {
        let _ = self.signal_shutdown();
    }
}

/// Owns a component's spawned server task until it has been joined.
///
/// Dropping the outer component runner can occur while its `select!` branch is
/// awaiting this task, for example when `JobManager` escalates at its deadline.
/// The drop implementation sends the normal halt signal and aborts the child,
/// so the child cannot continue detached from its component owner.
#[derive(Debug)]
pub struct NestedServerTask<T> {
    halt_signal: HaltSignal,
    task: JoinHandle<T>,
    shutdown_controller: Option<JoinHandle<()>>,
}

impl<T> NestedServerTask<T> {
    #[must_use]
    pub const fn new(halt_task: Sender<Halted>, task: JoinHandle<T>) -> Self {
        Self {
            halt_signal: HaltSignal::new(halt_task),
            task,
            shutdown_controller: None,
        }
    }

    /// Creates an owner that also supervises a server's shutdown controller.
    #[must_use]
    pub const fn with_shutdown_controller(
        halt_task: Sender<Halted>,
        task: JoinHandle<T>,
        shutdown_controller: JoinHandle<()>,
    ) -> Self {
        Self {
            halt_signal: HaltSignal::new(halt_task),
            task,
            shutdown_controller: Some(shutdown_controller),
        }
    }

    /// Signals normal shutdown. It is harmless when shutdown was already signalled.
    ///
    /// # Errors
    ///
    /// Returns the unsent signal when the receiver has already been dropped.
    pub fn signal_shutdown(&mut self) -> Result<(), Halted> {
        self.halt_signal.signal_shutdown()
    }

    /// Joins the nested server task.
    ///
    /// # Errors
    ///
    /// Returns the nested task's join error when it panics or is aborted.
    pub async fn join(&mut self) -> Result<T, JoinError> {
        (&mut self.task).await
    }

    /// Joins the shutdown controller after normal halt has been signalled.
    ///
    /// # Errors
    ///
    /// Returns the controller's join error when it panics or is aborted.
    pub async fn join_shutdown_controller(&mut self) -> Result<(), JoinError> {
        if let Some(shutdown_controller) = &mut self.shutdown_controller {
            shutdown_controller.await
        } else {
            Ok(())
        }
    }
}

impl<T> Drop for NestedServerTask<T> {
    fn drop(&mut self) {
        let _ = self.signal_shutdown();
        self.task.abort();
        if let Some(shutdown_controller) = &self.shutdown_controller {
            shutdown_controller.abort();
        }
    }
}

// issue: #1488
// Transitional compatibility boundary. Do not register new components here.
// SI-5 must migrate peers inactivity update to `JobManager::spawn`; UDP IP-ban
// cleanup has its own periodic-job migration.
/// A pre-spawned job retained for a narrow legacy compatibility boundary.
///
/// `JoinSet` cannot adopt this handle without a wrapper task, so this type is
/// deliberately not part of the direct-component task set.
#[derive(Debug)]
struct LegacyJob {
    name: String,
    handle: JoinHandle<()>,
}

/// Manages direct, named top-level component tasks.
#[derive(Debug, Default)]
pub struct JobManager {
    jobs: JoinSet<ComponentResult>,
    job_names: HashMap<Id, String>,
    legacy_jobs: Vec<LegacyJob>,
    cancellation_token: CancellationToken,
}

impl JobManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            jobs: JoinSet::new(),
            job_names: HashMap::new(),
            legacy_jobs: Vec::new(),
            cancellation_token: CancellationToken::new(),
        }
    }

    /// Spawns a direct top-level component task under this manager's ownership.
    ///
    /// The future must explicitly report independent completion, cooperative
    /// cancellation, or component failure. A task panic and deadline abort are
    /// supervisor outcomes and are recorded separately.
    pub fn spawn<N, F>(&mut self, name: N, future: F)
    where
        N: Into<String>,
        F: Future<Output = ComponentResult> + Send + 'static,
    {
        let abort_handle = self.jobs.spawn(future);
        self.job_names.insert(abort_handle.id(), name.into());
    }

    // issue: #1488
    /// Registers an existing periodic job without transferring it to `JoinSet`.
    ///
    /// This narrow compatibility API preserves pre-existing launch and
    /// cancellation semantics for periodic jobs that own a `JoinHandle` before
    /// registration. The manager still joins and escalates these jobs; callers
    /// must not drop the handle. New components must use [`Self::spawn`].
    pub fn register_legacy<N>(&mut self, name: N, handle: JoinHandle<()>)
    where
        N: Into<String>,
    {
        self.legacy_jobs.push(LegacyJob {
            name: name.into(),
            handle,
        });
    }

    #[must_use]
    pub fn new_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }

    /// Signals cooperative cancellation to every component.
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    /// Waits for all component tasks under one shared deadline.
    ///
    /// The deadline also applies to the narrowly scoped legacy compatibility
    /// jobs. When it expires, every remaining task is aborted and joined before
    /// this method returns. Components own the cleanup of their own children
    /// before completing.
    pub async fn wait_for_all(mut self, grace_period: Duration) -> Vec<JobOutcome> {
        let mut outcomes = Vec::new();
        let deadline = Instant::now() + grace_period;
        info!(
            timeout_seconds = grace_period.as_secs(),
            "Waiting for job to finish under the shared shutdown deadline"
        );
        let completed_before_deadline = self.collect_jobs_before_deadline(deadline, &mut outcomes).await;

        if !completed_before_deadline {
            warn!(
                timeout_seconds = grace_period.as_secs(),
                "Job shutdown deadline expired; aborting remaining jobs"
            );
            self.abort_and_drain_remaining_jobs(&mut outcomes).await;
        }

        outcomes
    }

    async fn collect_jobs_before_deadline(&mut self, deadline: Instant, outcomes: &mut Vec<JobOutcome>) -> bool {
        timeout_at(deadline, async {
            while !self.jobs.is_empty() || !self.legacy_jobs.is_empty() {
                tokio::select! {
                    result = self.jobs.join_next_with_id(), if !self.jobs.is_empty() => {
                        if let Some(result) = result {
                            outcomes.push(outcome(result, &mut self.job_names, false));
                        }
                    }
                    (index, result) = wait_for_legacy_completion(&mut self.legacy_jobs), if !self.legacy_jobs.is_empty() => {
                        let job = self.legacy_jobs.swap_remove(index);
                        outcomes.push(legacy_outcome(job, result, false));
                    }
                }
            }
        })
        .await
        .is_ok()
    }

    async fn abort_and_drain_remaining_jobs(&mut self, outcomes: &mut Vec<JobOutcome>) {
        self.jobs.abort_all();
        for job in &self.legacy_jobs {
            job.handle.abort();
        }

        while let Some(result) = self.jobs.join_next_with_id().await {
            outcomes.push(outcome(result, &mut self.job_names, true));
        }

        while let Some(mut job) = self.legacy_jobs.pop() {
            let result = (&mut job.handle).await;
            outcomes.push(legacy_outcome(job, result, true));
        }
    }
}

fn outcome(
    result: Result<(Id, ComponentResult), JoinError>,
    job_names: &mut HashMap<Id, String>,
    aborted_after_deadline: bool,
) -> JobOutcome {
    let (id, status) = match result {
        Ok((id, Ok(ComponentCompletion::Completed))) => (id, JobStatus::Completed),
        Ok((id, Ok(ComponentCompletion::Cancelled))) => (id, JobStatus::Cancelled),
        Ok((id, Err(error))) => (id, JobStatus::Failed(error)),
        Err(error) => {
            let id = error.id();
            let status = join_error_status(error, aborted_after_deadline);
            (id, status)
        }
    };
    let name = job_names.remove(&id).unwrap_or_else(|| format!("unknown-job-{id}"));
    log_outcome(&name, &status);
    JobOutcome { name, status }
}

async fn wait_for_legacy_completion(legacy_jobs: &mut [LegacyJob]) -> (usize, Result<(), JoinError>) {
    std::future::poll_fn(|context: &mut Context<'_>| {
        for (index, job) in legacy_jobs.iter_mut().enumerate() {
            if let Poll::Ready(result) = Pin::new(&mut job.handle).poll(context) {
                return Poll::Ready((index, result));
            }
        }

        Poll::Pending
    })
    .await
}

fn legacy_outcome(job: LegacyJob, result: Result<(), JoinError>, aborted_after_deadline: bool) -> JobOutcome {
    let status = result.map_or_else(
        |error| join_error_status(error, aborted_after_deadline),
        |()| JobStatus::Completed,
    );
    log_outcome(&job.name, &status);
    JobOutcome { name: job.name, status }
}

fn join_error_status(error: JoinError, aborted_after_deadline: bool) -> JobStatus {
    if aborted_after_deadline && error.is_cancelled() {
        JobStatus::Aborted
    } else if error.is_panic() {
        let payload = error.into_panic();
        JobStatus::Panicked(panic_message(payload.as_ref()))
    } else {
        JobStatus::Aborted
    }
}

fn log_outcome(name: &str, status: &JobStatus) {
    match status {
        JobStatus::Completed => log_completed(name),
        JobStatus::Cancelled => log_cancelled(name),
        JobStatus::Failed(error) => log_failed(name, error),
        JobStatus::Panicked(message) => log_panicked(name, message),
        JobStatus::Aborted => log_aborted(name),
    }
}

fn log_completed(name: &str) {
    info!(job = %name, "Job completed");
}

fn log_cancelled(name: &str) {
    info!(job = %name, "Job completed after cooperative cancellation");
}

fn log_failed(name: &str, error: &ComponentError) {
    warn!(job = %name, %error, "Job reported a failure");
}

fn log_panicked(name: &str, message: &str) {
    warn!(job = %name, %message, "Job panicked");
}

fn log_aborted(name: &str) {
    warn!(job = %name, "Job was aborted after the shutdown deadline");
}

fn panic_message(payload: &(dyn std::any::Any + Send + 'static)) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&'static str>().map(|message| (*message).to_owned()))
        .unwrap_or_else(|| "non-string panic payload".to_owned())
}

#[cfg(test)]
mod tests {
    use tokio::sync::oneshot;
    use tokio::time::{Duration, timeout};
    use torrust_server_lib::signals::Halted;

    use super::{ComponentCompletion, ComponentError, JobManager, JobOutcome, JobStatus};

    #[tokio::test]
    async fn it_should_report_jobs_in_completion_order() {
        // Arrange
        let mut manager = JobManager::new();
        let (second_finished_sender, second_finished_receiver) = oneshot::channel();
        manager.spawn("first", async move {
            second_finished_receiver
                .await
                .expect("second task reports its completion before the first task completes");
            Ok(ComponentCompletion::Completed)
        });
        manager.spawn("second", async move {
            second_finished_sender
                .send(())
                .expect("first task is waiting for the second task");
            Ok(ComponentCompletion::Completed)
        });

        // Act
        let outcomes = manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![
                JobOutcome {
                    name: "second".to_owned(),
                    status: JobStatus::Completed,
                },
                JobOutcome {
                    name: "first".to_owned(),
                    status: JobStatus::Completed,
                },
            ]
        );
    }

    #[tokio::test]
    async fn it_should_report_a_named_panic() {
        // Arrange
        let mut manager = JobManager::new();
        manager.spawn("panic-job", async {
            panic!("expected panic");
        });

        // Act
        let outcomes = manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "panic-job".to_owned(),
                status: JobStatus::Panicked("expected panic".to_owned()),
            }]
        );
    }

    #[tokio::test]
    async fn it_should_apply_one_deadline_to_concurrently_blocked_jobs_and_join_them_after_aborting() {
        // Arrange
        let mut manager = JobManager::new();
        let (first_sender, first_receiver) = oneshot::channel::<()>();
        let (second_sender, second_receiver) = oneshot::channel::<()>();
        manager.spawn("first", async move {
            let _ = first_receiver.await;
            Ok(ComponentCompletion::Completed)
        });
        manager.spawn("second", async move {
            let _ = second_receiver.await;
            Ok(ComponentCompletion::Completed)
        });

        // Act
        let outcomes = timeout(Duration::from_secs(1), manager.wait_for_all(Duration::ZERO))
            .await
            .expect("manager must join aborted jobs without detaching them");

        // Assert
        assert_eq!(outcomes.len(), 2);
        assert!(outcomes.iter().all(|outcome| outcome.status == JobStatus::Aborted));
        assert!(first_sender.send(()).is_err());
        assert!(second_sender.send(()).is_err());
    }

    #[tokio::test]
    async fn it_should_report_cooperatively_cancelled_jobs() {
        // Arrange
        let mut manager = JobManager::new();
        let cancellation_token = manager.new_cancellation_token();
        manager.spawn("cooperative-job", async move {
            cancellation_token.cancelled().await;
            Ok(ComponentCompletion::Cancelled)
        });

        // Act
        manager.cancel();
        let outcomes = manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "cooperative-job".to_owned(),
                status: JobStatus::Cancelled,
            }]
        );
    }

    #[tokio::test]
    async fn it_should_abort_and_join_a_legacy_pre_spawned_job_after_the_deadline() {
        // Arrange
        let mut manager = JobManager::new();
        let (completion_sender, completion_receiver) = oneshot::channel::<()>();
        manager.register_legacy(
            "legacy-job",
            tokio::spawn(async move {
                let _ = completion_receiver.await;
            }),
        );

        // Act
        let outcomes = manager.wait_for_all(Duration::ZERO).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "legacy-job".to_owned(),
                status: JobStatus::Aborted,
            }]
        );
        assert!(completion_sender.send(()).is_err());
    }

    #[tokio::test]
    async fn it_should_abort_a_nested_server_task_when_the_component_is_aborted_after_the_deadline() {
        // Arrange
        let mut manager = JobManager::new();
        let (child_completion_sender, child_completion_receiver) = oneshot::channel::<()>();
        let (halt_sender, halt_receiver) = oneshot::channel();
        let (server_ready_sender, server_ready_receiver) = oneshot::channel();
        manager.spawn("server-component", async move {
            let mut server = super::NestedServerTask::new(
                halt_sender,
                tokio::spawn(async move {
                    let _ = child_completion_receiver.await;
                }),
            );
            server_ready_sender
                .send(())
                .expect("the test waits until the nested server owner is constructed");
            server
                .join()
                .await
                .map_err(|error| ComponentError::new(format!("server task failed: {error}")))?;
            Ok(ComponentCompletion::Completed)
        });

        // Act
        server_ready_receiver
            .await
            .expect("the nested server owner must be constructed before the deadline starts");
        let outcomes = manager.wait_for_all(Duration::ZERO).await;

        // Assert
        assert_eq!(outcomes[0].status, JobStatus::Aborted);
        assert!(
            child_completion_sender.send(()).is_err(),
            "the nested task receiver must be dropped after the outer task abort"
        );
        assert!(matches!(halt_receiver.await, Ok(Halted::Normal)));
    }

    #[tokio::test]
    async fn it_should_abort_a_nested_shutdown_controller_when_the_component_is_aborted_after_the_deadline() {
        // Arrange
        let mut manager = JobManager::new();
        let (server_completion_sender, server_completion_receiver) = oneshot::channel::<()>();
        let (controller_completion_sender, controller_completion_receiver) = oneshot::channel::<()>();
        let (halt_sender, halt_receiver) = oneshot::channel();
        let (server_ready_sender, server_ready_receiver) = oneshot::channel();
        manager.spawn("server-component", async move {
            let mut server = super::NestedServerTask::with_shutdown_controller(
                halt_sender,
                tokio::spawn(async move {
                    let _ = server_completion_receiver.await;
                }),
                tokio::spawn(async move {
                    let _ = controller_completion_receiver.await;
                }),
            );
            server_ready_sender
                .send(())
                .expect("the test waits until the nested server owner is constructed");
            server
                .join()
                .await
                .map_err(|error| ComponentError::new(format!("server task failed: {error}")))?;
            Ok(ComponentCompletion::Completed)
        });

        // Act
        server_ready_receiver
            .await
            .expect("the nested server owner must be constructed before the deadline starts");
        let outcomes = manager.wait_for_all(Duration::ZERO).await;

        // Assert
        assert_eq!(outcomes[0].status, JobStatus::Aborted);
        assert!(matches!(halt_receiver.await, Ok(Halted::Normal)));
        assert!(
            server_completion_sender.send(()).is_err(),
            "the nested server receiver must be dropped after the outer task abort"
        );
        assert!(
            controller_completion_sender.send(()).is_err(),
            "the shutdown controller receiver must be dropped after the outer task abort"
        );
    }

    #[tokio::test]
    async fn it_should_signal_and_drop_an_uncooperative_running_future_when_the_component_is_aborted_after_the_deadline() {
        // Arrange
        let mut manager = JobManager::new();
        let (running_completion_sender, running_completion_receiver) = oneshot::channel::<()>();
        let (halt_sender, halt_receiver) = oneshot::channel();
        let (halt_signal_ready_sender, halt_signal_ready_receiver) = oneshot::channel();
        manager.spawn("running-server-component", async move {
            let _halt_signal = super::HaltSignal::new(halt_sender);
            halt_signal_ready_sender
                .send(())
                .expect("the test waits until the halt signal guard is constructed");
            let _ = running_completion_receiver.await;
            Ok(ComponentCompletion::Completed)
        });

        // Act
        halt_signal_ready_receiver
            .await
            .expect("the halt signal guard must be constructed before the deadline starts");
        let outcomes = manager.wait_for_all(Duration::ZERO).await;

        // Assert
        assert_eq!(outcomes[0].status, JobStatus::Aborted);
        assert!(matches!(halt_receiver.await, Ok(Halted::Normal)));
        assert!(
            running_completion_sender.send(()).is_err(),
            "the running future receiver must be dropped after the outer task abort"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_apply_the_same_deadline_to_direct_and_legacy_jobs() {
        // Arrange
        let mut manager = JobManager::new();
        let grace_period = Duration::from_secs(1);
        let (direct_completion_sender, direct_completion_receiver) = oneshot::channel::<()>();
        let (first_legacy_completion_sender, first_legacy_completion_receiver) = oneshot::channel::<()>();
        let (second_legacy_completion_sender, second_legacy_completion_receiver) = oneshot::channel::<()>();
        manager.spawn("direct-job", async move {
            let _ = direct_completion_receiver.await;
            Ok(ComponentCompletion::Completed)
        });
        manager.register_legacy(
            "first-legacy-job",
            tokio::spawn(async move {
                let _ = first_legacy_completion_receiver.await;
            }),
        );
        manager.register_legacy(
            "second-legacy-job",
            tokio::spawn(async move {
                let _ = second_legacy_completion_receiver.await;
            }),
        );

        // Act
        let started_at = tokio::time::Instant::now();
        let outcomes = manager.wait_for_all(grace_period).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![
                JobOutcome {
                    name: "direct-job".to_owned(),
                    status: JobStatus::Aborted,
                },
                JobOutcome {
                    name: "second-legacy-job".to_owned(),
                    status: JobStatus::Aborted,
                },
                JobOutcome {
                    name: "first-legacy-job".to_owned(),
                    status: JobStatus::Aborted,
                },
            ]
        );
        assert_eq!(tokio::time::Instant::now() - started_at, grace_period);
        assert!(direct_completion_sender.send(()).is_err());
        assert!(first_legacy_completion_sender.send(()).is_err());
        assert!(second_legacy_completion_sender.send(()).is_err());
    }

    #[tokio::test]
    async fn it_should_report_a_named_component_failure() {
        // Arrange
        let mut manager = JobManager::new();
        manager.spawn("failing-job", async { Err(ComponentError::new("database connection lost")) });

        // Act
        let outcomes = manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "failing-job".to_owned(),
                status: JobStatus::Failed(ComponentError::new("database connection lost")),
            }]
        );
    }
}
