//! Application-level background-component supervision.
//!
//! `JobManager` owns only direct components registered during application
//! bootstrap. A component is registered in two steps: [`JobManager::reserve`]
//! issues its [`Reservation`], which carries the component's own cancellation
//! token, and [`JobManager::spawn`] starts the runner built with that token.
//! Each component, rather than this supervisor, owns and completes or
//! deliberately aborts its nested tasks. Shutdown requests cooperative
//! cancellation, waits under one shared deadline, then aborts and joins any
//! remaining direct components.
//!
//! issue: #1586
//! issue: #2415

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
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

/// Owns a token-aware server task and its cancellation drain controller.
///
/// Dropping the outer component runner aborts both children, preventing either
/// task from outliving its component owner.
#[derive(Debug)]
pub struct TokenAwareServerTask<T, U> {
    task: JoinHandle<T>,
    shutdown_controller: JoinHandle<U>,
}

impl<T, U> TokenAwareServerTask<T, U> {
    #[must_use]
    pub const fn new(task: JoinHandle<T>, shutdown_controller: JoinHandle<U>) -> Self {
        Self {
            task,
            shutdown_controller,
        }
    }

    /// Joins the nested server task.
    ///
    /// # Errors
    ///
    /// Returns the nested task's join error when it panics or is aborted.
    pub async fn join(&mut self) -> Result<T, JoinError> {
        (&mut self.task).await
    }

    /// Joins the nested cancellation drain controller.
    ///
    /// # Errors
    ///
    /// Returns the controller's join error when it panics or is aborted.
    pub async fn join_shutdown_controller(&mut self) -> Result<U, JoinError> {
        (&mut self.shutdown_controller).await
    }
}

impl<T, U> Drop for TokenAwareServerTask<T, U> {
    fn drop(&mut self) {
        self.task.abort();
        self.shutdown_controller.abort();
    }
}

/// Owns a component's single spawned task until it has been joined.
///
/// Dropping the outer component runner aborts the task, preventing it from
/// outliving its component owner.
#[derive(Debug)]
pub struct OwnedTask<T> {
    task: JoinHandle<T>,
}

impl<T> OwnedTask<T> {
    #[must_use]
    pub const fn new(task: JoinHandle<T>) -> Self {
        Self { task }
    }

    /// Joins the owned task, consuming the owner so the task is joined at most once.
    ///
    /// The owner lives inside the returned future, so dropping that future
    /// before the task finishes still aborts the task.
    ///
    /// # Errors
    ///
    /// Returns the task's join error when it panics or is aborted.
    pub async fn join(mut self) -> Result<T, JoinError> {
        (&mut self.task).await
    }
}

impl<T> Drop for OwnedTask<T> {
    fn drop(&mut self) {
        self.task.abort();
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

/// Identifies a top-level component reserved in one [`JobManager`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComponentId(u64);

/// The right to spawn one top-level component in the [`JobManager`] that
/// issued it.
///
/// It carries the component's id and its own cancellation token, which is not
/// derived from any other token. Build the component's runner with
/// [`Reservation::cancellation_token`], then pass the reservation to
/// [`JobManager::spawn`]. Dropping a reservation without spawning it releases
/// it: the manager forgets its token and no other component is affected.
///
/// A reservation is spawned at most once, because spawning consumes it:
///
/// ```compile_fail,E0382
/// # use torrust_tracker_lib::bootstrap::jobs::manager::{ComponentCompletion, JobManager};
/// let mut manager = JobManager::new();
/// let reservation = manager.reserve("component");
/// let _ = manager.spawn(reservation, async { Ok(ComponentCompletion::Completed) });
/// let _ = manager.spawn(reservation, async { Ok(ComponentCompletion::Completed) });
/// ```
///
/// Reserve again to register another component:
///
/// ```no_run
/// # use torrust_tracker_lib::bootstrap::jobs::manager::{ComponentCompletion, JobManager};
/// let mut manager = JobManager::new();
/// let first = manager.reserve("first-component");
/// let _ = manager.spawn(first, async { Ok(ComponentCompletion::Completed) });
/// let second = manager.reserve("second-component");
/// let _ = manager.spawn(second, async { Ok(ComponentCompletion::Completed) });
/// ```
#[derive(Debug)]
pub struct Reservation {
    id: ComponentId,
    name: String,
    cancellation_token: CancellationToken,
    registry: Arc<Mutex<ComponentRegistry>>,
}

impl Reservation {
    /// The id of the reserved component.
    #[must_use]
    pub const fn id(&self) -> ComponentId {
        self.id
    }

    /// The component's own cancellation token.
    ///
    /// It is cancelled by [`JobManager::cancel`], including when the manager
    /// was cancelled before the reservation was issued or before it is spawned.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }
}

impl Drop for Reservation {
    fn drop(&mut self) {
        lock(&self.registry).release_if_reserved(self.id);
    }
}

/// Errors returned when a reserved component cannot be spawned.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum SpawnError {
    #[error(
        "Component '{component}' was reserved by another `JobManager`, so this manager cannot spawn it. Spawn it with the manager whose `reserve` issued the reservation."
    )]
    ForeignReservation { component: String },
}

#[derive(Debug, PartialEq, Eq)]
enum RegistrationState {
    Reserved,
    Spawned,
}

#[derive(Debug)]
struct RegisteredComponent {
    cancellation_token: CancellationToken,
    state: RegistrationState,
}

/// The component tokens a manager has issued, shared with its reservations so
/// a dropped reservation can release itself.
#[derive(Debug, Default)]
struct ComponentRegistry {
    next_id: u64,
    cancelled: bool,
    components: HashMap<ComponentId, RegisteredComponent>,
}

impl ComponentRegistry {
    fn reserve(&mut self) -> (ComponentId, CancellationToken) {
        let id = ComponentId(self.next_id);
        // Unreachable in practice: bootstrap reserves a handful of components.
        // Failing loudly is safer than wrapping, which would reuse an id and
        // overwrite another component's entry.
        self.next_id = self
            .next_id
            .checked_add(1)
            .expect("the job manager has issued every possible component id");

        let cancellation_token = CancellationToken::new();
        if self.cancelled {
            cancellation_token.cancel();
        }

        self.components.insert(
            id,
            RegisteredComponent {
                cancellation_token: cancellation_token.clone(),
                state: RegistrationState::Reserved,
            },
        );

        (id, cancellation_token)
    }

    fn mark_spawned(&mut self, id: ComponentId) {
        // A reservation is removed only by its own `Drop`, which runs after
        // this call, and `spawn` has already checked it belongs here.
        let component = self.components.get_mut(&id);
        debug_assert!(component.is_some(), "a live reservation must be registered in its manager");
        if let Some(component) = component {
            component.state = RegistrationState::Spawned;
        }
    }

    fn release_if_reserved(&mut self, id: ComponentId) {
        if self
            .components
            .get(&id)
            .is_some_and(|component| component.state == RegistrationState::Reserved)
        {
            self.components.remove(&id);
        }
    }

    // Cancelling a token only wakes the tasks waiting on it; none of them can
    // re-enter this registry while the lock is held.
    fn cancel_all(&mut self) {
        self.cancelled = true;
        for component in self.components.values() {
            component.cancellation_token.cancel();
        }
    }
}

/// Locks the registry even if a thread panicked while holding the lock: every
/// critical section leaves it consistent, and a reservation's drop must not
/// panic.
fn lock(registry: &Mutex<ComponentRegistry>) -> MutexGuard<'_, ComponentRegistry> {
    registry.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Manages direct, named top-level component tasks.
#[derive(Debug, Default)]
pub struct JobManager {
    jobs: JoinSet<ComponentResult>,
    job_names: HashMap<Id, String>,
    legacy_jobs: Vec<LegacyJob>,
    registry: Arc<Mutex<ComponentRegistry>>,
    cancellation_token: CancellationToken,
}

impl JobManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            jobs: JoinSet::new(),
            job_names: HashMap::new(),
            legacy_jobs: Vec::new(),
            registry: Arc::new(Mutex::new(ComponentRegistry::default())),
            cancellation_token: CancellationToken::new(),
        }
    }

    /// Reserves a top-level component and issues its own cancellation token.
    ///
    /// The token is not derived from the legacy root token or from any other
    /// component's token. If the manager has already been cancelled, the token
    /// is issued cancelled.
    pub fn reserve<N>(&mut self, name: N) -> Reservation
    where
        N: Into<String>,
    {
        let (id, cancellation_token) = lock(&self.registry).reserve();

        Reservation {
            id,
            name: name.into(),
            cancellation_token,
            registry: Arc::clone(&self.registry),
        }
    }

    /// Spawns a reserved top-level component task under this manager's ownership.
    ///
    /// The future must be built with the reservation's cancellation token, and
    /// must explicitly report independent completion, cooperative cancellation,
    /// or component failure. A task panic and deadline abort are supervisor
    /// outcomes and are recorded separately.
    ///
    /// # Errors
    ///
    /// Returns [`SpawnError::ForeignReservation`] when another manager issued
    /// the reservation. The future is dropped without being polled, and the
    /// reservation is released from the manager that issued it.
    pub fn spawn<F>(&mut self, reservation: Reservation, future: F) -> Result<(), SpawnError>
    where
        F: Future<Output = ComponentResult> + Send + 'static,
    {
        if !Arc::ptr_eq(&reservation.registry, &self.registry) {
            return Err(SpawnError::ForeignReservation {
                component: reservation.name.clone(),
            });
        }

        lock(&self.registry).mark_spawned(reservation.id);
        let abort_handle = self.jobs.spawn(future);
        self.job_names.insert(abort_handle.id(), reservation.name.clone());

        // Consumed so it cannot be spawned twice. Its drop finds the component
        // spawned and keeps its token registered for `cancel()`.
        drop(reservation);

        Ok(())
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

    /// Returns a clone of the legacy root token.
    ///
    /// Only jobs registered with [`Self::register_legacy`] use it. Components
    /// spawned with [`Self::spawn`] use the token of their [`Reservation`].
    #[must_use]
    pub fn legacy_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }

    /// Signals cooperative cancellation to every component.
    ///
    /// It cancels the legacy root token and the token of every reserved or
    /// spawned component. A reservation issued afterwards gets a cancelled
    /// token. Calling it again has no further effect.
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
        lock(&self.registry).cancel_all();
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
    use std::future::Future;

    use tokio::sync::oneshot;
    use tokio::time::{Duration, timeout};
    use torrust_server_lib::signals::Halted;

    use super::{
        ComponentCompletion, ComponentError, ComponentRegistry, ComponentResult, JobManager, JobOutcome, JobStatus, Reservation,
        SpawnError,
    };

    /// Reserves a component and spawns it with a runner that does not use its token.
    fn reserve_and_spawn<F>(manager: &mut JobManager, name: &str, future: F)
    where
        F: Future<Output = ComponentResult> + Send + 'static,
    {
        let reservation = manager.reserve(name);
        manager
            .spawn(reservation, future)
            .expect("the reservation was issued by this manager");
    }

    #[tokio::test]
    async fn it_should_report_jobs_in_completion_order() {
        // Arrange
        let mut manager = JobManager::new();
        let (second_finished_sender, second_finished_receiver) = oneshot::channel();
        reserve_and_spawn(&mut manager, "first", async move {
            second_finished_receiver
                .await
                .expect("second task reports its completion before the first task completes");
            Ok(ComponentCompletion::Completed)
        });
        reserve_and_spawn(&mut manager, "second", async move {
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
        reserve_and_spawn(&mut manager, "panic-job", async {
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
        reserve_and_spawn(&mut manager, "first", async move {
            let _ = first_receiver.await;
            Ok(ComponentCompletion::Completed)
        });
        reserve_and_spawn(&mut manager, "second", async move {
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
        reserve_and_spawn(&mut manager, "server-component", async move {
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
        reserve_and_spawn(&mut manager, "server-component", async move {
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
        reserve_and_spawn(&mut manager, "running-server-component", async move {
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
        reserve_and_spawn(&mut manager, "direct-job", async move {
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
        reserve_and_spawn(&mut manager, "failing-job", async {
            Err(ComponentError::new("database connection lost"))
        });

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

    // Reservation lifecycle (issue #2415).

    /// Spawns a component that runs until its own reservation token is cancelled.
    fn spawn_component_stopping_on_its_own_token(manager: &mut JobManager, reservation: Reservation) {
        let cancellation_token = reservation.cancellation_token();
        manager
            .spawn(reservation, async move {
                cancellation_token.cancelled().await;
                Ok(ComponentCompletion::Cancelled)
            })
            .expect("the reservation was issued by this manager");
    }

    #[tokio::test]
    async fn it_should_cancel_a_spawned_components_own_token_when_the_manager_is_cancelled() {
        // Arrange
        let mut manager = JobManager::new();
        let reservation = manager.reserve("cooperative-component");
        spawn_component_stopping_on_its_own_token(&mut manager, reservation);

        // Act
        manager.cancel();
        let outcomes = manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "cooperative-component".to_owned(),
                status: JobStatus::Cancelled,
            }]
        );
    }

    #[test]
    fn it_should_not_cancel_a_reserved_component_when_only_the_legacy_root_token_is_cancelled() {
        // Arrange
        let mut manager = JobManager::new();
        let reservation = manager.reserve("component");

        // Act
        manager.legacy_cancellation_token().cancel();

        // Assert
        assert!(
            !reservation.cancellation_token().is_cancelled(),
            "a component token must not be a child of the legacy root token"
        );
    }

    #[test]
    fn it_should_issue_a_cancelled_token_when_a_component_is_reserved_after_the_manager_is_cancelled() {
        // Arrange
        let mut manager = JobManager::new();
        manager.cancel();

        // Act
        let reservation = manager.reserve("late-component");

        // Assert
        assert!(
            reservation.cancellation_token().is_cancelled(),
            "a reservation issued after cancel() must carry a cancelled token"
        );
    }

    #[tokio::test]
    async fn it_should_cancel_a_reserved_component_that_is_spawned_after_the_manager_is_cancelled() {
        // Arrange
        let mut manager = JobManager::new();
        let reservation = manager.reserve("component-spawned-after-cancel");

        // Act
        manager.cancel();
        let token_cancelled_before_spawn = reservation.cancellation_token().is_cancelled();
        spawn_component_stopping_on_its_own_token(&mut manager, reservation);
        let outcomes = manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        assert!(
            token_cancelled_before_spawn,
            "cancel() between reserve and spawn must cancel the reserved token"
        );
        assert_eq!(
            outcomes,
            vec![JobOutcome {
                name: "component-spawned-after-cancel".to_owned(),
                status: JobStatus::Cancelled,
            }]
        );
    }

    #[tokio::test]
    async fn it_should_stop_components_and_legacy_jobs_once_when_the_manager_is_cancelled_twice() {
        // Arrange
        let mut manager = JobManager::new();
        let reservation = manager.reserve("component");
        spawn_component_stopping_on_its_own_token(&mut manager, reservation);
        let legacy_token = manager.legacy_cancellation_token();
        manager.register_legacy(
            "legacy-job",
            tokio::spawn(async move {
                legacy_token.cancelled().await;
            }),
        );

        // Act
        manager.cancel();
        manager.cancel();
        let mut outcomes = manager.wait_for_all(Duration::from_secs(1)).await;

        // Assert
        outcomes.sort_by(|left, right| left.name.cmp(&right.name));
        assert_eq!(
            outcomes,
            vec![
                JobOutcome {
                    name: "component".to_owned(),
                    status: JobStatus::Cancelled,
                },
                JobOutcome {
                    name: "legacy-job".to_owned(),
                    status: JobStatus::Completed,
                },
            ]
        );
    }

    #[test]
    fn it_should_release_a_dropped_reservation_without_affecting_other_components() {
        // Arrange
        let mut manager = JobManager::new();
        let dropped_reservation = manager.reserve("dropped-component");
        let kept_reservation = manager.reserve("kept-component");
        let dropped_token = dropped_reservation.cancellation_token();

        // Act
        drop(dropped_reservation);
        let kept_token_cancelled_by_the_drop = kept_reservation.cancellation_token().is_cancelled();
        manager.cancel();

        // Assert
        assert!(
            !kept_token_cancelled_by_the_drop,
            "dropping one reservation must not cancel another component"
        );
        assert!(
            !dropped_token.is_cancelled(),
            "a released reservation's token must no longer be held by the manager"
        );
        assert!(
            kept_reservation.cancellation_token().is_cancelled(),
            "releasing one reservation must not stop the manager from cancelling the others"
        );
    }

    #[tokio::test]
    async fn it_should_reject_a_reservation_issued_by_another_manager() {
        // Arrange
        let mut issuing_manager = JobManager::new();
        let mut other_manager = JobManager::new();
        let reservation = issuing_manager.reserve("foreign-component");
        let foreign_token = reservation.cancellation_token();
        let (ran_sender, ran_receiver) = oneshot::channel::<()>();

        // Act
        let result = other_manager.spawn(reservation, async move {
            let _ = ran_sender.send(());
            Ok(ComponentCompletion::Completed)
        });
        issuing_manager.cancel();

        // Assert
        assert_eq!(
            result,
            Err(SpawnError::ForeignReservation {
                component: "foreign-component".to_owned(),
            })
        );
        assert!(
            ran_receiver.await.is_err(),
            "the runner of a rejected reservation must be dropped without running"
        );
        assert!(
            !foreign_token.is_cancelled(),
            "a rejected reservation must be released by the manager that issued it"
        );
        assert_eq!(
            other_manager.wait_for_all(Duration::from_secs(1)).await,
            Vec::<JobOutcome>::new(),
            "the manager that rejected the reservation must not supervise its runner"
        );
    }

    #[test]
    #[should_panic(expected = "the job manager has issued every possible component id")]
    fn it_should_refuse_to_issue_a_reused_component_id_when_the_id_space_is_exhausted() {
        // Arrange
        let mut registry = ComponentRegistry {
            next_id: u64::MAX,
            ..ComponentRegistry::default()
        };

        // Act
        let (_id, _token) = registry.reserve();
    }
}
