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
                // Buffer is full, remove the eldest tasks.
                if let Some(eldest_task) = self.rb.try_pop() {
                    // After popping there is one free place.
                    if eldest_task.is_finished() {
                        self.rb
                            .try_push(abort_handle)
                            .expect("it should have place now for a new active request.");
                        return;
                    }

                    // Give it a second chance to finish.
                    tokio::task::yield_now().await;

                    if !eldest_task.is_finished() {
                        // Log unfinished task.
                        tracing::debug!(
                            target: UDP_TRACKER_LOG_TARGET,
                            local_addr,
                            "Udp::run_udp_server::loop (got unfinished task)"
                        );

                        eldest_task.abort();

                        tracing::warn!(
                            target: UDP_TRACKER_LOG_TARGET,
                            local_addr,
                            "Udp::run_udp_server::loop aborting request: (no finished tasks)"
                        );
                    }
                }
            }
        };
    }

    /// # Panics
    ///
    /// Will panic if it can't re-insert a removed unfinished tasks
    pub fn clean_finished(&mut self) {
        // We clean unfinished tasks until we found a finished one.

        let mut unfinished_task = None;

        for removed_abort_handle in self.rb.pop_iter() {
            if removed_abort_handle.is_finished() {
                continue;
            }

            // Save the unfinished task for re-entry.
            unfinished_task = Some(removed_abort_handle);
            
            break;
        }

        // Reinsert the unfinished task if any.
        if let Some(h) = unfinished_task {
            self.rb.try_push(h).expect("it was previously inserted");
        }
    }
}
