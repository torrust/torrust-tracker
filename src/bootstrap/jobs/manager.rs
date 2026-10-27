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
//! A component can be spawned with [`StopBefore`] edges naming the components
//! that must not be asked to stop until it has completed. [`JobManager::cancel`]
//! asks to stop only the components that wait for nobody, and
//! [`JobManager::wait_for_all`] asks each of the others as soon as the last
//! component it waits for has completed. The manager knows nothing about what
//! the edges mean: bootstrap declares them.
//!
//! ADR: `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md`
//!
//! issue: #1586
//! issue: #2415
//! issue: #2416

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
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

/// Process exit code of a shutdown in which every component completed.
pub const EXIT_CODE_GRACEFUL_SHUTDOWN: u8 = 0;

/// Process exit code of a shutdown in which a component failed, panicked, or was aborted.
pub const EXIT_CODE_INCOMPLETE_SHUTDOWN: u8 = 1;

/// Maps the outcomes of a shutdown to the process exit code (Q3 of
/// `docs/features/shutdown-process/questions.md`).
#[must_use]
pub const fn shutdown_exit_code(_outcomes: &[JobOutcome]) -> u8 {
    EXIT_CODE_GRACEFUL_SHUTDOWN
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
///
/// Ids of one manager are ordered by reservation. An id also names its
/// manager, so another manager can tell that it did not issue it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComponentId {
    manager: u64,
    index: u64,
}

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
///
/// Whatever the error, the runner is dropped without being polled and the
/// reservation is released: the manager holds no edge and no token for the
/// rejected component.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum SpawnError {
    #[error(
        "Component '{component}' was reserved by another `JobManager`, so this manager cannot spawn it. Spawn it with the manager whose `reserve` issued the reservation."
    )]
    ForeignReservation { component: String },

    #[error(
        "Component '{component}' cannot stop before {target:?}, because this manager has no such component. The id belongs to another `JobManager`, was never issued, or its reservation was dropped without being spawned. Pass only ids of components reserved and kept by this manager."
    )]
    UnknownStopBeforeTarget { component: String, target: ComponentId },

    #[error(
        "Component '{component}' cannot stop before {target:?}, because it was not reserved before '{component}'. A component may only stop before components reserved earlier, which keeps the stop order free of cycles. Reserve the component that stops last first."
    )]
    StopBeforeTargetNotEarlier { component: String, target: ComponentId },

    #[error(
        "Component '{component}' cannot be given stop-before edges, because shutdown has already been requested and the components it would stop before may already be cancelled. Register every component and its edges before calling `cancel`."
    )]
    ShutdownRequested { component: String },
}

/// The components that a spawned component stops before.
///
/// Each target is cancelled only after the spawned component has completed,
/// once shutdown has been requested. A target is the id of a component
/// reserved earlier in the same manager. Targets that no component waits for
/// are cancelled by [`JobManager::cancel`], as before.
///
/// ```no_run
/// # use torrust_tracker_lib::bootstrap::jobs::manager::{ComponentCompletion, JobManager, StopBefore};
/// let mut manager = JobManager::new();
/// let consumer = manager.reserve("consumer");
/// let consumer_id = consumer.id();
/// let _ = manager.spawn(consumer, async { Ok(ComponentCompletion::Completed) });
/// let producer = manager.reserve("producer");
/// let _ = manager.spawn_with(producer, StopBefore(&[consumer_id]), async { Ok(ComponentCompletion::Completed) });
/// ```
#[derive(Debug, Clone, Copy)]
pub struct StopBefore<'a>(pub &'a [ComponentId]);

impl StopBefore<'_> {
    /// No edges: the component waits for nobody and is waited for by nobody.
    pub const NONE: Self = StopBefore(&[]);
}

#[derive(Debug, PartialEq, Eq)]
enum RegistrationState {
    Reserved,
    Spawned,
    Finished,
}

#[derive(Debug)]
struct RegisteredComponent {
    name: String,
    cancellation_token: CancellationToken,
    state: RegistrationState,
    /// Components that must finish before this one is cancelled by the ordered shutdown.
    unfinished_predecessors: BTreeSet<ComponentId>,
}

/// What the manager decided for its components when shutdown was requested.
#[derive(Debug, Default, PartialEq, Eq)]
struct ShutdownPlan {
    cancelled: Vec<String>,
    waiting: Vec<WaitingComponent>,
}

/// A component that is not asked to stop until the components it waits for have completed.
#[derive(Debug, PartialEq, Eq)]
pub struct WaitingComponent {
    pub name: String,
    pub waiting_for: Vec<String>,
}

/// The component tokens a manager has issued, shared with its reservations so
/// a dropped reservation can release itself.
#[derive(Debug)]
struct ComponentRegistry {
    manager_id: u64,
    next_id: u64,
    cancelled: bool,
    components: BTreeMap<ComponentId, RegisteredComponent>,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        static NEXT_MANAGER_ID: AtomicU64 = AtomicU64::new(0);

        Self {
            manager_id: NEXT_MANAGER_ID.fetch_add(1, Ordering::Relaxed),
            next_id: 0,
            cancelled: false,
            components: BTreeMap::new(),
        }
    }
}

impl ComponentRegistry {
    fn reserve(&mut self, name: &str) -> (ComponentId, CancellationToken) {
        let id = ComponentId {
            manager: self.manager_id,
            index: self.next_id,
        };
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
                name: name.to_owned(),
                cancellation_token: cancellation_token.clone(),
                state: RegistrationState::Reserved,
                unfinished_predecessors: BTreeSet::new(),
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

    /// Records that `id` stops before every target, or records nothing.
    fn add_stop_before(&mut self, id: ComponentId, targets: &[ComponentId]) -> Result<(), SpawnError> {
        if targets.is_empty() {
            return Ok(());
        }

        let component = self.component_name(id);
        for &target in targets {
            if target.manager != self.manager_id || !self.components.contains_key(&target) {
                return Err(SpawnError::UnknownStopBeforeTarget { component, target });
            }
            if target >= id {
                return Err(SpawnError::StopBeforeTargetNotEarlier { component, target });
            }
        }
        if self.cancelled {
            return Err(SpawnError::ShutdownRequested { component });
        }

        for target in targets {
            if let Some(successor) = self.components.get_mut(target) {
                successor.unfinished_predecessors.insert(id);
            }
        }

        Ok(())
    }

    fn component_name(&self, id: ComponentId) -> String {
        self.components
            .get(&id)
            .map_or_else(String::new, |component| component.name.clone())
    }

    /// Requests the ordered shutdown: cancels every component that waits for
    /// nobody, and reports which components wait for which.
    ///
    /// Cancelling a token only wakes the tasks waiting on it; none of them can
    /// re-enter this registry while the lock is held. Returns `None` when
    /// shutdown was already requested.
    fn request_shutdown(&mut self) -> Option<ShutdownPlan> {
        if self.cancelled {
            return None;
        }
        self.cancelled = true;

        let mut plan = ShutdownPlan::default();
        for component in self.components.values() {
            if component.unfinished_predecessors.is_empty() {
                component.cancellation_token.cancel();
                plan.cancelled.push(component.name.clone());
            } else {
                plan.waiting.push(WaitingComponent {
                    name: component.name.clone(),
                    waiting_for: self.names_of(&component.unfinished_predecessors),
                });
            }
        }

        Some(plan)
    }

    /// Records that a component completed, and, once shutdown was requested,
    /// cancels the components that were waiting only for it and the others
    /// that have completed. Returns the names it cancelled.
    fn mark_finished(&mut self, id: ComponentId) -> Vec<String> {
        if let Some(component) = self.components.get_mut(&id) {
            component.state = RegistrationState::Finished;
        }

        let mut cancelled = Vec::new();
        for component in self.components.values_mut() {
            if component.unfinished_predecessors.remove(&id)
                && component.unfinished_predecessors.is_empty()
                && self.cancelled
                && component.state != RegistrationState::Finished
            {
                component.cancellation_token.cancel();
                cancelled.push(component.name.clone());
            }
        }

        cancelled
    }

    /// The spawned components that have not completed and still wait for others.
    fn waiting_components(&self) -> Vec<WaitingComponent> {
        self.components
            .values()
            .filter(|component| component.state == RegistrationState::Spawned && !component.unfinished_predecessors.is_empty())
            .map(|component| WaitingComponent {
                name: component.name.clone(),
                waiting_for: self.names_of(&component.unfinished_predecessors),
            })
            .collect()
    }

    fn names_of(&self, ids: &BTreeSet<ComponentId>) -> Vec<String> {
        ids.iter().map(|id| self.component_name(*id)).collect()
    }
}

/// Locks the registry even if a thread panicked while holding the lock: every
/// critical section leaves it consistent, and a reservation's drop must not
/// panic.
fn lock(registry: &Mutex<ComponentRegistry>) -> MutexGuard<'_, ComponentRegistry> {
    registry.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The component a running task belongs to, kept to name its outcome and to
/// release the components that wait for it.
#[derive(Debug)]
struct JobIdentity {
    id: ComponentId,
    name: String,
}

/// Manages direct, named top-level component tasks.
#[derive(Debug, Default)]
pub struct JobManager {
    jobs: JoinSet<ComponentResult>,
    job_names: HashMap<Id, JobIdentity>,
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
        let name = name.into();
        let (id, cancellation_token) = lock(&self.registry).reserve(&name);

        Reservation {
            id,
            name,
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
        self.spawn_with(reservation, StopBefore::NONE, future)
    }

    /// Spawns a reserved top-level component that stops before the `stop_before` components.
    ///
    /// Once shutdown is requested, each target is cancelled only after this
    /// component has completed (in any state, including failure or panic) and
    /// after every other component that stops before it has completed too.
    ///
    /// # Errors
    ///
    /// Returns the errors of [`Self::spawn`], and also:
    ///
    /// - [`SpawnError::UnknownStopBeforeTarget`] when a target is not a component of this manager.
    /// - [`SpawnError::StopBeforeTargetNotEarlier`] when a target is this component or was reserved after it.
    /// - [`SpawnError::ShutdownRequested`] when [`Self::cancel`] was already called.
    pub fn spawn_with<F>(&mut self, reservation: Reservation, stop_before: StopBefore<'_>, future: F) -> Result<(), SpawnError>
    where
        F: Future<Output = ComponentResult> + Send + 'static,
    {
        if !Arc::ptr_eq(&reservation.registry, &self.registry) {
            return Err(SpawnError::ForeignReservation {
                component: reservation.name.clone(),
            });
        }

        {
            let mut registry = lock(&self.registry);
            registry.add_stop_before(reservation.id, stop_before.0)?;
            registry.mark_spawned(reservation.id);
        }
        let abort_handle = self.jobs.spawn(future);
        self.job_names.insert(
            abort_handle.id(),
            JobIdentity {
                id: reservation.id,
                name: reservation.name.clone(),
            },
        );

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

    /// Requests the ordered shutdown.
    ///
    /// It cancels the legacy root token and the token of every reserved or
    /// spawned component that does not wait for another component. A
    /// component that must wait for others (see [`StopBefore`]) is cancelled by
    /// [`Self::wait_for_all`] once the last of them has completed; the logs name
    /// what each one waits for. A reservation issued afterwards gets a
    /// cancelled token. Calling it again has no further effect.
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
        let plan = lock(&self.registry).request_shutdown();
        if let Some(plan) = plan {
            log_shutdown_plan(&plan);
        }
    }

    /// The spawned components that wait for other components before they are asked to stop,
    /// with the names of the components they wait for, in reservation order.
    ///
    /// It reflects the edges declared with [`StopBefore`], minus the components that have
    /// completed.
    #[must_use]
    pub fn waiting_components(&self) -> Vec<WaitingComponent> {
        lock(&self.registry).waiting_components()
    }

    /// Waits for all component tasks under one shared deadline.
    ///
    /// Each time a component completes, in any state, and shutdown was
    /// requested, it cancels the components that were waiting only for it.
    /// Completion alone never cancels anything before [`Self::cancel`].
    ///
    /// The deadline also applies to the narrowly scoped legacy compatibility
    /// jobs. When it expires, every remaining task is aborted and joined before
    /// this method returns, including components that were never asked to stop
    /// because the components they wait for had not completed. Components own
    /// the cleanup of their own children before completing.
    pub async fn wait_for_all(mut self, grace_period: Duration) -> Vec<JobOutcome> {
        let mut outcomes = Vec::new();
        let deadline = Instant::now() + grace_period;
        info!(
            timeout_seconds = grace_period.as_secs(),
            "Waiting for job to finish under the shared shutdown deadline"
        );
        let completed_before_deadline = self.collect_jobs_before_deadline(deadline, &mut outcomes).await;

        if !completed_before_deadline {
            log_components_never_asked_to_stop(&self.waiting_components());
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
                            let (outcome, finished) = outcome(result, &mut self.job_names, false);
                            if let Some(finished) = finished {
                                let released = lock(&self.registry).mark_finished(finished);
                                log_released_components(&outcome.name, &released);
                            }
                            outcomes.push(outcome);
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
            outcomes.push(outcome(result, &mut self.job_names, true).0);
        }

        while let Some(mut job) = self.legacy_jobs.pop() {
            let result = (&mut job.handle).await;
            outcomes.push(legacy_outcome(job, result, true));
        }
    }
}

fn outcome(
    result: Result<(Id, ComponentResult), JoinError>,
    job_names: &mut HashMap<Id, JobIdentity>,
    aborted_after_deadline: bool,
) -> (JobOutcome, Option<ComponentId>) {
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
    let (name, component_id) = job_names.remove(&id).map_or_else(
        || (format!("unknown-job-{id}"), None),
        |identity| (identity.name, Some(identity.id)),
    );
    log_outcome(&name, &status);
    (JobOutcome { name, status }, component_id)
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

fn log_shutdown_plan(plan: &ShutdownPlan) {
    for name in &plan.cancelled {
        info!(job = %name, "Requested the job to stop: it waits for no other job");
    }
    for waiting in &plan.waiting {
        info!(
            job = %waiting.name,
            waiting_for = %waiting.waiting_for.join(", "),
            "Job will be asked to stop after the jobs it waits for have completed"
        );
    }
}

fn log_released_components(completed_job: &str, cancelled: &[String]) {
    for name in cancelled {
        info!(
            job = %name,
            last_completed_job = %completed_job,
            "Requested the job to stop: every job it waited for has completed"
        );
    }
}

fn log_components_never_asked_to_stop(waiting: &[WaitingComponent]) {
    for component in waiting {
        warn!(
            job = %component.name,
            waiting_for = %component.waiting_for.join(", "),
            "Job was never asked to stop because the jobs it waits for had not completed"
        );
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
    use tokio::time::{Duration, sleep, timeout};
    use tokio_util::sync::CancellationToken;
    use torrust_server_lib::signals::Halted;

    use super::{
        ComponentCompletion, ComponentError, ComponentId, ComponentRegistry, ComponentResult, EXIT_CODE_GRACEFUL_SHUTDOWN,
        EXIT_CODE_INCOMPLETE_SHUTDOWN, JobManager, JobOutcome, JobStatus, Reservation, ShutdownPlan, SpawnError, StopBefore,
        WaitingComponent, lock, shutdown_exit_code,
    };

    fn named_outcome(name: &str, status: JobStatus) -> JobOutcome {
        JobOutcome {
            name: name.to_owned(),
            status,
        }
    }

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

    // Stop order (issue #2416).

    /// Spawns a producer that stops before `consumers` and completes only when the returned sender is used.
    ///
    /// It ignores its own token, like a server that is still draining its requests after shutdown began.
    fn spawn_producer_released_by_the_test(
        manager: &mut JobManager,
        name: &str,
        consumers: &[ComponentId],
    ) -> oneshot::Sender<()> {
        let reservation = manager.reserve(name);
        let (release, released) = oneshot::channel();
        manager
            .spawn_with(reservation, StopBefore(consumers), async move {
                let _ = released.await;
                Ok(ComponentCompletion::Completed)
            })
            .expect("the consumer was reserved by this manager before the producer");
        release
    }

    /// Lets every runnable task run: with paused time the clock only advances once all of them are idle.
    async fn let_the_manager_react() {
        sleep(Duration::from_millis(1)).await;
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_cancel_a_consumer_only_after_every_producer_that_stops_before_it_has_completed() {
        // Arrange
        // A consumer waits for its own stop request. Two producers stop before it, and each keeps
        // running until the test releases it, like servers still answering requests during shutdown.
        let mut manager = JobManager::new();
        let consumer = manager.reserve("consumer");
        let consumer_id = consumer.id();
        let consumer_token = consumer.cancellation_token();
        spawn_component_stopping_on_its_own_token(&mut manager, consumer);
        let release_first = spawn_producer_released_by_the_test(&mut manager, "first-producer", &[consumer_id]);
        let release_second = spawn_producer_released_by_the_test(&mut manager, "second-producer", &[consumer_id]);

        // Act
        // Shutdown starts while both producers run. Then the first producer completes, and then the second.
        manager.cancel();
        let shutdown = tokio::spawn(manager.wait_for_all(Duration::from_secs(5)));
        let_the_manager_react().await;
        let cancelled_while_both_producers_run = consumer_token.is_cancelled();
        release_first.send(()).expect("the first producer is waiting for its release");
        let_the_manager_react().await;
        let cancelled_while_one_producer_runs = consumer_token.is_cancelled();
        release_second
            .send(())
            .expect("the second producer is waiting for its release");
        let_the_manager_react().await;
        let cancelled_after_both_producers_completed = consumer_token.is_cancelled();
        let outcomes = shutdown.await.expect("the shutdown task must not panic");

        // Assert
        // The consumer is cancelled only once both producers have completed, and it stops last.
        assert!(
            !cancelled_while_both_producers_run,
            "the consumer must keep running while both producers run, after cancel()"
        );
        assert!(
            !cancelled_while_one_producer_runs,
            "the consumer must keep running while one of its two producers still runs"
        );
        assert!(
            cancelled_after_both_producers_completed,
            "the consumer must be cancelled once both producers have completed"
        );
        assert_eq!(
            outcomes,
            vec![
                JobOutcome {
                    name: "first-producer".to_owned(),
                    status: JobStatus::Completed,
                },
                JobOutcome {
                    name: "second-producer".to_owned(),
                    status: JobStatus::Completed,
                },
                JobOutcome {
                    name: "consumer".to_owned(),
                    status: JobStatus::Cancelled,
                },
            ]
        );
    }

    /// Spawns a consumer that runs until its own token is cancelled, and returns its id and token.
    fn spawn_consumer(manager: &mut JobManager, name: &str) -> (ComponentId, CancellationToken) {
        let reservation = manager.reserve(name);
        let id = reservation.id();
        let token = reservation.cancellation_token();
        spawn_component_stopping_on_its_own_token(manager, reservation);
        (id, token)
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_cancel_a_component_that_waits_for_nobody_at_once_while_another_waits_for_its_producer() {
        // Arrange
        // A consumer has a producer that keeps running. An unrelated component has no edges.
        let mut manager = JobManager::new();
        let (consumer_id, consumer_token) = spawn_consumer(&mut manager, "consumer");
        let (_unrelated_id, unrelated_token) = spawn_consumer(&mut manager, "unrelated");
        let _release_producer = spawn_producer_released_by_the_test(&mut manager, "producer", &[consumer_id]);

        // Act
        manager.cancel();

        // Assert
        // The ordered shutdown does not delay a component that nobody stops before.
        assert!(
            unrelated_token.is_cancelled(),
            "cancel() must cancel a component that waits for nobody at once"
        );
        assert!(
            !consumer_token.is_cancelled(),
            "cancel() must not cancel the consumer while its producer runs"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_cancel_each_consumer_when_the_producers_that_stop_before_it_have_completed() {
        // Arrange
        // Producer P stops before consumers A and B. Producer Q stops before B only, and keeps running.
        let mut manager = JobManager::new();
        let (a_id, a_token) = spawn_consumer(&mut manager, "consumer-a");
        let (b_id, b_token) = spawn_consumer(&mut manager, "consumer-b");
        let release_p = spawn_producer_released_by_the_test(&mut manager, "producer-p", &[a_id, b_id]);
        let _release_q = spawn_producer_released_by_the_test(&mut manager, "producer-q", &[b_id]);
        manager.cancel();
        let _shutdown = tokio::spawn(manager.wait_for_all(Duration::from_secs(5)));

        // Act
        release_p.send(()).expect("producer P is waiting for its release");
        let_the_manager_react().await;

        // Assert
        // A waited only for P, so it is cancelled. B still waits for Q.
        assert!(
            a_token.is_cancelled(),
            "a consumer must be cancelled when every producer that stops before it has completed, even while other producers run"
        );
        assert!(
            !b_token.is_cancelled(),
            "a consumer must not be cancelled while one of its producers still runs"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_not_cancel_a_consumer_when_its_producers_complete_before_shutdown_is_requested() {
        // Arrange
        // A consumer waits for a producer that completes while no shutdown was requested.
        let mut manager = JobManager::new();
        let (consumer_id, consumer_token) = spawn_consumer(&mut manager, "consumer");
        let release_producer = spawn_producer_released_by_the_test(&mut manager, "producer", &[consumer_id]);
        let shutdown = tokio::spawn(manager.wait_for_all(Duration::from_secs(5)));

        // Act
        release_producer.send(()).expect("the producer is waiting for its release");
        let_the_manager_react().await;
        let cancelled_by_the_completion = consumer_token.is_cancelled();
        let outcomes = shutdown.await.expect("the shutdown task must not panic");

        // Assert
        // Only a requested shutdown asks components to stop: the consumer runs until the deadline aborts it.
        assert!(
            !cancelled_by_the_completion,
            "a producer completing must not cancel a consumer before cancel() was called"
        );
        assert_eq!(
            outcomes,
            vec![
                JobOutcome {
                    name: "producer".to_owned(),
                    status: JobStatus::Completed,
                },
                JobOutcome {
                    name: "consumer".to_owned(),
                    status: JobStatus::Aborted,
                },
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_cancel_a_consumer_whose_producer_completed_before_shutdown_was_requested() {
        // Arrange
        // The producer completes on its own, and the manager has not yet observed it when cancel() is called.
        let mut manager = JobManager::new();
        let (consumer_id, _consumer_token) = spawn_consumer(&mut manager, "consumer");
        let release_producer = spawn_producer_released_by_the_test(&mut manager, "producer", &[consumer_id]);
        release_producer.send(()).expect("the producer is waiting for its release");
        let_the_manager_react().await;

        // Act
        manager.cancel();
        let outcomes = manager.wait_for_all(Duration::from_secs(5)).await;

        // Assert
        // The finished producer does not hold its consumer back: it stops cooperatively, not by abort.
        assert_eq!(
            outcomes,
            vec![
                JobOutcome {
                    name: "producer".to_owned(),
                    status: JobStatus::Completed,
                },
                JobOutcome {
                    name: "consumer".to_owned(),
                    status: JobStatus::Cancelled,
                },
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_cancel_a_consumer_when_its_producer_fails_and_keep_the_failure_visible() {
        // Arrange
        let mut manager = JobManager::new();
        let (consumer_id, _consumer_token) = spawn_consumer(&mut manager, "consumer");
        let reservation = manager.reserve("failing-producer");
        manager
            .spawn_with(reservation, StopBefore(&[consumer_id]), async {
                Err(ComponentError::new("socket lost"))
            })
            .expect("the consumer was reserved before the producer");

        // Act
        manager.cancel();
        let outcomes = manager.wait_for_all(Duration::from_secs(5)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![
                JobOutcome {
                    name: "failing-producer".to_owned(),
                    status: JobStatus::Failed(ComponentError::new("socket lost")),
                },
                JobOutcome {
                    name: "consumer".to_owned(),
                    status: JobStatus::Cancelled,
                },
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_cancel_a_consumer_when_its_producer_panics_and_keep_the_panic_visible() {
        // Arrange
        let mut manager = JobManager::new();
        let (consumer_id, _consumer_token) = spawn_consumer(&mut manager, "consumer");
        let reservation = manager.reserve("panicking-producer");
        manager
            .spawn_with(reservation, StopBefore(&[consumer_id]), async { panic!("producer defect") })
            .expect("the consumer was reserved before the producer");

        // Act
        manager.cancel();
        let outcomes = manager.wait_for_all(Duration::from_secs(5)).await;

        // Assert
        assert_eq!(
            outcomes,
            vec![
                JobOutcome {
                    name: "panicking-producer".to_owned(),
                    status: JobStatus::Panicked("producer defect".to_owned()),
                },
                JobOutcome {
                    name: "consumer".to_owned(),
                    status: JobStatus::Cancelled,
                },
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_abort_and_name_a_consumer_that_was_never_asked_to_stop_when_the_deadline_expires() {
        // Arrange
        // The producer never completes, so the consumer is never cancelled.
        let mut manager = JobManager::new();
        let (consumer_id, consumer_token) = spawn_consumer(&mut manager, "consumer");
        let _release_producer = spawn_producer_released_by_the_test(&mut manager, "producer", &[consumer_id]);
        manager.cancel();

        // Act
        let outcomes = manager.wait_for_all(Duration::from_secs(5)).await;

        // Assert
        // The shared deadline aborts and joins both components, and each outcome keeps its name.
        assert!(
            !consumer_token.is_cancelled(),
            "the consumer must not be cancelled while its producer has not completed"
        );
        assert_eq!(outcomes.len(), 2, "both components must be joined");
        assert!(
            outcomes.iter().all(|outcome| outcome.status == JobStatus::Aborted),
            "both components must be reported as aborted: {outcomes:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_abort_a_consumer_busy_in_a_handler_at_the_shared_deadline_even_when_it_was_cancelled_late() {
        // Arrange
        // The consumer ignores its stop request, like a listener stuck in a handler. Its producer completes
        // 2 seconds after shutdown began, so the consumer is cancelled with 3 of the 5 seconds left.
        let mut manager = JobManager::new();
        let consumer = manager.reserve("busy-consumer");
        let consumer_id = consumer.id();
        manager
            .spawn(consumer, std::future::pending::<ComponentResult>())
            .expect("the reservation was issued by this manager");
        let release_producer = spawn_producer_released_by_the_test(&mut manager, "producer", &[consumer_id]);
        let started = tokio::time::Instant::now();

        // Act
        manager.cancel();
        let shutdown = tokio::spawn(manager.wait_for_all(Duration::from_secs(5)));
        sleep(Duration::from_secs(2)).await;
        release_producer.send(()).expect("the producer is waiting for its release");
        let outcomes = shutdown.await.expect("the shutdown task must not panic");

        // Assert
        // The deadline runs from the start of the wait, not from the consumer's cancellation, and the
        // busy consumer is aborted, joined and named.
        assert_eq!(
            started.elapsed(),
            Duration::from_secs(5),
            "the shared deadline must not restart"
        );
        assert_eq!(
            outcomes,
            vec![
                JobOutcome {
                    name: "producer".to_owned(),
                    status: JobStatus::Completed,
                },
                JobOutcome {
                    name: "busy-consumer".to_owned(),
                    status: JobStatus::Aborted,
                },
            ]
        );
    }

    #[test]
    fn it_should_reject_a_stop_before_target_issued_by_another_manager() {
        // Arrange
        let mut other_manager = JobManager::new();
        let foreign = other_manager.reserve("foreign-component");
        let foreign_id = foreign.id();
        let mut manager = JobManager::new();
        let producer = manager.reserve("producer");

        // Act
        let result = manager.spawn_with(producer, StopBefore(&[foreign_id]), async {
            Ok(ComponentCompletion::Completed)
        });

        // Assert
        assert_eq!(
            result,
            Err(SpawnError::UnknownStopBeforeTarget {
                component: "producer".to_owned(),
                target: foreign_id,
            })
        );
    }

    #[test]
    fn it_should_reject_a_stop_before_target_whose_reservation_was_dropped() {
        // Arrange
        let mut manager = JobManager::new();
        let dropped = manager.reserve("dropped-consumer");
        let dropped_id = dropped.id();
        drop(dropped);
        let producer = manager.reserve("producer");

        // Act
        let result = manager.spawn_with(producer, StopBefore(&[dropped_id]), async {
            Ok(ComponentCompletion::Completed)
        });

        // Assert
        assert_eq!(
            result,
            Err(SpawnError::UnknownStopBeforeTarget {
                component: "producer".to_owned(),
                target: dropped_id,
            })
        );
    }

    #[test]
    fn it_should_reject_a_stop_before_target_reserved_after_the_component() {
        // Arrange
        let mut manager = JobManager::new();
        let producer = manager.reserve("producer");
        let later = manager.reserve("later-component");
        let later_id = later.id();

        // Act
        let result = manager.spawn_with(producer, StopBefore(&[later_id]), async {
            Ok(ComponentCompletion::Completed)
        });

        // Assert
        assert_eq!(
            result,
            Err(SpawnError::StopBeforeTargetNotEarlier {
                component: "producer".to_owned(),
                target: later_id,
            })
        );
    }

    #[test]
    fn it_should_reject_a_component_that_stops_before_itself() {
        // Arrange
        let mut manager = JobManager::new();
        let producer = manager.reserve("producer");
        let own_id = producer.id();

        // Act
        let result = manager.spawn_with(producer, StopBefore(&[own_id]), async { Ok(ComponentCompletion::Completed) });

        // Assert
        assert_eq!(
            result,
            Err(SpawnError::StopBeforeTargetNotEarlier {
                component: "producer".to_owned(),
                target: own_id,
            })
        );
    }

    #[tokio::test]
    async fn it_should_reject_stop_before_edges_after_shutdown_was_requested() {
        // Arrange
        let mut manager = JobManager::new();
        let (consumer_id, _consumer_token) = spawn_consumer(&mut manager, "consumer");
        manager.cancel();
        let late_producer = manager.reserve("late-producer");

        // Act
        let result = manager.spawn_with(late_producer, StopBefore(&[consumer_id]), async {
            Ok(ComponentCompletion::Completed)
        });

        // Assert
        assert_eq!(
            result,
            Err(SpawnError::ShutdownRequested {
                component: "late-producer".to_owned(),
            })
        );
    }

    #[tokio::test]
    async fn it_should_leave_no_edge_and_no_running_component_when_a_spawn_with_edges_is_rejected() {
        // Arrange
        // The first target is valid and the second was issued by another manager.
        let mut other_manager = JobManager::new();
        let foreign_id = other_manager.reserve("foreign-component").id();
        let mut manager = JobManager::new();
        let (consumer_id, consumer_token) = spawn_consumer(&mut manager, "consumer");
        let producer = manager.reserve("rejected-producer");
        let producer_token = producer.cancellation_token();
        let (ran_sender, ran_receiver) = oneshot::channel::<()>();

        // Act
        let result = manager.spawn_with(producer, StopBefore(&[consumer_id, foreign_id]), async move {
            let _ = ran_sender.send(());
            Ok(ComponentCompletion::Completed)
        });
        manager.cancel();
        let consumer_cancelled_without_the_rejected_producer = consumer_token.is_cancelled();

        // Assert
        // A rejected registration is atomic: the valid target keeps no edge, the runner never runs,
        // and the reservation is released.
        assert!(result.is_err(), "the foreign target must reject the registration");
        assert!(
            consumer_cancelled_without_the_rejected_producer,
            "a rejected spawn must not leave an edge that holds the valid target back"
        );
        assert!(
            ran_receiver.await.is_err(),
            "the runner of a rejected spawn must be dropped without running"
        );
        assert!(
            !producer_token.is_cancelled(),
            "a rejected reservation must be released, so cancel() does not reach its token"
        );
    }

    #[tokio::test]
    async fn it_should_report_which_components_the_shutdown_cancels_and_which_wait() {
        // Arrange
        let mut manager = JobManager::new();
        let consumer = manager.reserve("consumer");
        let consumer_id = consumer.id();
        let unrelated = manager.reserve("unrelated");
        let first = manager.reserve("first-producer");
        let second = manager.reserve("second-producer");
        manager
            .spawn(consumer, async { Ok(ComponentCompletion::Completed) })
            .expect("the reservation was issued by this manager");
        manager
            .spawn(unrelated, async { Ok(ComponentCompletion::Completed) })
            .expect("the reservation was issued by this manager");
        manager
            .spawn_with(first, StopBefore(&[consumer_id]), async {
                Ok(ComponentCompletion::Completed)
            })
            .expect("the consumer was reserved before the producer");
        manager
            .spawn_with(second, StopBefore(&[consumer_id]), async {
                Ok(ComponentCompletion::Completed)
            })
            .expect("the consumer was reserved before the producer");

        // Act
        let plan = lock(&manager.registry).request_shutdown();

        // Assert
        assert_eq!(
            plan,
            Some(ShutdownPlan {
                cancelled: vec![
                    "unrelated".to_owned(),
                    "first-producer".to_owned(),
                    "second-producer".to_owned()
                ],
                waiting: vec![WaitingComponent {
                    name: "consumer".to_owned(),
                    waiting_for: vec!["first-producer".to_owned(), "second-producer".to_owned()],
                }],
            })
        );
        assert_eq!(
            lock(&manager.registry).request_shutdown(),
            None,
            "a second request must not report the same decisions again"
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
        let (_id, _token) = registry.reserve("component");
    }

    #[test]
    fn it_should_exit_with_the_graceful_code_when_there_are_no_outcomes() {
        // Arrange
        let outcomes: Vec<JobOutcome> = Vec::new();

        // Act
        let exit_code = shutdown_exit_code(&outcomes);

        // Assert
        assert_eq!(exit_code, EXIT_CODE_GRACEFUL_SHUTDOWN, "no component means nothing failed");
    }

    #[test]
    fn it_should_exit_with_the_graceful_code_when_every_component_completed_or_was_cancelled() {
        // Arrange
        let outcomes = vec![
            named_outcome("http_api", JobStatus::Completed),
            named_outcome("udp_instance_0", JobStatus::Cancelled),
        ];

        // Act
        let exit_code = shutdown_exit_code(&outcomes);

        // Assert
        assert_eq!(
            exit_code, EXIT_CODE_GRACEFUL_SHUTDOWN,
            "Completed and Cancelled are graceful outcomes"
        );
    }

    #[test]
    fn it_should_exit_with_the_incomplete_code_when_a_component_failed() {
        // Arrange
        let outcomes = vec![
            named_outcome("http_api", JobStatus::Completed),
            named_outcome(
                "http_instance_0",
                JobStatus::Failed(ComponentError::new("graceful drain timed out")),
            ),
        ];

        // Act
        let exit_code = shutdown_exit_code(&outcomes);

        // Assert
        assert_eq!(
            exit_code, EXIT_CODE_INCOMPLETE_SHUTDOWN,
            "a Failed outcome (for example a reached drain budget) must not exit with the graceful code"
        );
    }

    #[test]
    fn it_should_exit_with_the_incomplete_code_when_a_component_panicked() {
        // Arrange
        let outcomes = vec![
            named_outcome("http_api", JobStatus::Completed),
            named_outcome("torrent_cleanup", JobStatus::Panicked("boom".to_owned())),
        ];

        // Act
        let exit_code = shutdown_exit_code(&outcomes);

        // Assert
        assert_eq!(
            exit_code, EXIT_CODE_INCOMPLETE_SHUTDOWN,
            "a Panicked outcome must not exit with the graceful code"
        );
    }

    #[test]
    fn it_should_exit_with_the_incomplete_code_when_a_component_was_aborted_at_the_deadline() {
        // Arrange
        let outcomes = vec![
            named_outcome("http_api", JobStatus::Completed),
            named_outcome("peers_inactivity_update", JobStatus::Aborted),
        ];

        // Act
        let exit_code = shutdown_exit_code(&outcomes);

        // Assert
        assert_eq!(
            exit_code, EXIT_CODE_INCOMPLETE_SHUTDOWN,
            "an Aborted outcome (the supervisor deadline expired) must not exit with the graceful code"
        );
    }
}
