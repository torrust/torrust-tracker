use ringbuf::StaticRb;
use ringbuf::traits::{Consumer, Observer, Producer};
use tokio::task::AbortHandle;
use torrust_tracker_udp_core::UDP_TRACKER_LOG_TARGET;

// ADR: packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md
/// A ring buffer for managing active UDP request abort handles.
///
/// The `ActiveRequests` struct maintains a fixed-size ring buffer of abort
/// handles for UDP request processor tasks. It ensures that at most 50 requests
/// are handled concurrently, and provides mechanisms to handle buffer overflow
/// by removing finished or oldest unfinished tasks.
#[derive(Default)]
pub struct ActiveRequests {
    rb: StaticRb<AbortHandle, 50>, // The number of requests handled simultaneously.
}

impl std::fmt::Debug for ActiveRequests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (left, right) = &self.rb.as_slices();
        let dbg = format!("capacity: {}, left: {left:?}, right: {right:?}", self.rb.capacity());
        f.debug_struct("ActiveRequests").field("rb", &dbg).finish()
    }
}

impl Drop for ActiveRequests {
    fn drop(&mut self) {
        for h in self.rb.pop_iter() {
            if !h.is_finished() {
                h.abort();
            }
        }
    }
}

impl ActiveRequests {
    /// Inserts an abort handle for a UDP request processor task.
    ///
    /// If the buffer is full, this method traverses handles from oldest to newest. It:
    ///
    /// 1. Removes completed handles encountered before the first still-active handle.
    /// 2. Gives that oldest active task one scheduler yield to finish.
    /// 3. Aborts that task when no earlier completed handle created capacity; otherwise it
    ///    continues the bounded traversal. It retains at most one subsequently encountered active
    ///    handle for re-entry.
    ///
    /// It intentionally does not scan all newer handles before selecting this eviction. See the
    /// module ADR for the request-hot-path performance rationale.
    ///
    /// Returns `true` if a task was removed, `false` otherwise.
    ///
    /// # Panics
    ///
    /// This method will panic if it cannot make space for adding a new handle.
    ///
    /// # Arguments
    ///
    /// * `abort_handle` - The `AbortHandle` for the UDP request processor task.
    /// * `local_addr` - A string slice representing the local address for logging.
    pub async fn force_push(&mut self, new_task: AbortHandle, local_addr: &str) -> bool {
        // Attempt to add the new handle to the buffer.
        match self.rb.try_push(new_task) {
            Ok(()) => {
                // Successfully added the task, no further action needed.
                false
            }
            Err(new_task) => {
                // Buffer is full, attempt to make space.

                let mut finished: u64 = 0;
                let mut unfinished_task = None;
                let mut old_task_aborted = false;

                for old_task in self.rb.pop_iter() {
                    // A completed task before the first still-active task frees capacity.
                    if old_task.is_finished() {
                        finished += 1;
                        continue;
                    }

                    // Give the oldest still-active task one opportunity to finish.
                    tokio::task::yield_now().await;

                    // If it completed while yielded, it also frees capacity.
                    if old_task.is_finished() {
                        finished += 1;
                        continue;
                    }

                    // This is the first task that remains active after yielding.
                    tracing::debug!(
                        target: UDP_TRACKER_LOG_TARGET,
                        local_addr,
                        removed_count = finished,
                        "Udp::run_udp_server::loop (got unfinished task)"
                    );

                    // No older completed task created capacity, so evict this oldest active task.
                    if finished == 0 {
                        // We make place aborting this task.
                        old_task.abort();
                        old_task_aborted = true;

                        tracing::warn!(
                            target: UDP_TRACKER_LOG_TARGET,
                            local_addr,
                            "Udp::run_udp_server::loop aborting request: (no finished tasks)"
                        );

                        break;
                    }

                    // Earlier completed tasks created capacity; retain this active task for re-entry.
                    unfinished_task = Some(old_task);
                }

                // After this point there can't be a race condition because only
                // one thread owns the active buffer. There is no way for the
                // buffer to be full again. That means the "expects" should
                // never happen.

                // Reinsert the active task that followed at least one completed task, if any.
                if let Some(h) = unfinished_task {
                    self.rb.try_push(h).expect("it was previously inserted");
                }

                // Insert the new task.
                //
                // Earlier completed tasks, or one oldest active task eviction, made capacity.
                if !new_task.is_finished() {
                    self.rb.try_push(new_task).expect("it should have space for this new task.");
                }

                old_task_aborted
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::oneshot;
    use tokio::task::JoinHandle;

    use ringbuf::traits::Producer;

    use super::ActiveRequests;

    struct PendingTask {
        completion_sender: oneshot::Sender<()>,
        join_handle: JoinHandle<()>,
    }

    impl PendingTask {
        fn new() -> Self {
            let (completion_sender, completion_receiver) = oneshot::channel::<()>();
            let join_handle = tokio::spawn(async move {
                drop(completion_receiver.await);
            });

            Self {
                completion_sender,
                join_handle,
            }
        }

        fn abort_handle(&self) -> tokio::task::AbortHandle {
            self.join_handle.abort_handle()
        }

        async fn assert_was_aborted(self, message: &str) {
            self.join_handle.await.expect_err(message);
        }
    }

    struct FullBufferWithPendingTasks {
        active_requests: ActiveRequests,
        oldest_task: Option<PendingTask>,
        retained_tasks: Vec<PendingTask>,
        new_task: PendingTask,
    }

    impl FullBufferWithPendingTasks {
        fn new() -> Self {
            let mut active_requests = ActiveRequests::default();
            let oldest_task = PendingTask::new();
            active_requests
                .rb
                .try_push(oldest_task.abort_handle())
                .expect("an empty request buffer should accept the oldest task");

            let mut retained_tasks = Vec::with_capacity(49);
            for _ in 0..49 {
                let task = PendingTask::new();
                active_requests
                    .rb
                    .try_push(task.abort_handle())
                    .expect("a request buffer with available capacity should accept the task");
                retained_tasks.push(task);
            }

            let new_task = PendingTask::new();

            Self {
                active_requests,
                oldest_task: Some(oldest_task),
                retained_tasks,
                new_task,
            }
        }

        fn new_task_abort_handle(&self) -> tokio::task::AbortHandle {
            self.new_task.abort_handle()
        }

        async fn assert_oldest_task_was_aborted(&mut self) {
            self.oldest_task
                .take()
                .expect("scenario should retain the oldest task")
                .assert_was_aborted("oldest pending task should be evicted when capacity is exhausted")
                .await;
        }

        async fn abort_and_join_retained_tasks(self) {
            drop(self.active_requests);

            for task in self.retained_tasks {
                task.assert_was_aborted("retained task should be aborted during test cleanup")
                    .await;
            }
            self.new_task
                .assert_was_aborted("new task should be aborted during test cleanup")
                .await;
        }
    }

    #[tokio::test]
    async fn it_should_not_evict_a_pending_task_when_the_buffer_has_available_capacity() {
        // Arrange
        let task = PendingTask::new();
        let mut active_requests = ActiveRequests::default();

        // Act
        let task_was_evicted = active_requests.force_push(task.abort_handle(), "127.0.0.1:6969").await;

        // Assert
        assert!(!task_was_evicted);
        assert!(!task.join_handle.is_finished());

        drop(task.completion_sender);
        task.join_handle
            .await
            .expect("pending task should complete after test cleanup");
    }

    #[tokio::test]
    async fn it_should_evict_the_oldest_pending_task_when_the_buffer_is_full() {
        // Arrange
        let mut scenario = FullBufferWithPendingTasks::new();

        // Act
        let task_was_evicted = scenario
            .active_requests
            .force_push(scenario.new_task_abort_handle(), "127.0.0.1:6969")
            .await;

        // Assert
        assert!(task_was_evicted);
        scenario.assert_oldest_task_was_aborted().await;
        scenario.abort_and_join_retained_tasks().await;
    }
}
