use ringbuf::traits::{Consumer, Observer, Producer};
use ringbuf::StaticRb;
use tokio::task::AbortHandle;

use crate::servers::udp::UDP_TRACKER_LOG_TARGET;

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
        let dbg = format!("capacity: {}, left: {left:?}, right: {right:?}", &self.rb.capacity());
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
    /// If the buffer is full, this method attempts to make space by:
    ///
    /// 1. Removing finished tasks.
    /// 2. Removing the oldest unfinished task if no finished tasks are found.
    ///
    /// # Panics
    ///
    /// This method will panic if it cannot make space for adding a new handle.
    ///
    /// # Arguments
    ///
    /// * `abort_handle` - The `AbortHandle` for the UDP request processor task.
    /// * `local_addr` - A string slice representing the local address for logging.
    pub async fn force_push(&mut self, abort_handle: AbortHandle, local_addr: &str) {
        // Attempt to add the new handle to the buffer.
        match self.rb.try_push(abort_handle) {
            Ok(()) => {
                // Successfully added the task, no further action needed.
            }
            Err(abort_handle) => {
                // Buffer is full, attempt to make space.

                let mut finished: u64 = 0;
                let mut unfinished_task = None;

                for removed_abort_handle in self.rb.pop_iter() {
                    // We found a finished tasks ... increase the counter and
                    // continue searching for more and ...
                    if removed_abort_handle.is_finished() {
                        finished += 1;
                        continue;
                    }

                    // The current removed tasks is not finished.

                    // Give it a second chance to finish.
                    tokio::task::yield_now().await;

                    // Recheck if it finished ... increase the counter and
                    // continue searching for more and ...
                    if removed_abort_handle.is_finished() {
                        finished += 1;
                        continue;
                    }

                    // At this point we found a "definitive" unfinished task.

                    // Log unfinished task.
                    tracing::debug!(
                        target: UDP_TRACKER_LOG_TARGET,
                        local_addr,
                        removed_count = finished,
                        "Udp::run_udp_server::loop (got unfinished task)"
                    );

                    // If no finished tasks were found, abort the current
                    // unfinished task.
                    if finished == 0 {
                        // We make place aborting this task.
                        removed_abort_handle.abort();

                        tracing::warn!(
                            target: UDP_TRACKER_LOG_TARGET,
                            local_addr,
                            "Udp::run_udp_server::loop aborting request: (no finished tasks)"
                        );

                        break;
                    }

                    // At this point we found at least one finished task, but the
                    // current one is not finished and it was removed from the
                    // buffer, so we need to re-insert in in the buffer.

                    // Save the unfinished task for re-entry.
                    unfinished_task = Some(removed_abort_handle);
                }

                // After this point there can't be a race condition because only
                // one thread owns the active buffer. There is no way for the
                // buffer to be full again. That means the "expects" should
                // never happen.

                // Reinsert the unfinished task if any.
                if let Some(h) = unfinished_task {
                    self.rb.try_push(h).expect("it was previously inserted");
                }

                // Insert the new task, ensuring there's space.
                if !abort_handle.is_finished() {
                    self.rb
                        .try_push(abort_handle)
                        .expect("it should remove at least one element.");
                }
            }
        };
    }
}
