use ringbuf::traits::{Consumer, Observer, Producer};
use ringbuf::StaticRb;
use tokio::task::AbortHandle;

use crate::servers::udp::UDP_TRACKER_LOG_TARGET;

/// Ring-Buffer of Active Requests
#[derive(Default)]
pub struct ActiveRequests {
    rb: StaticRb<AbortHandle, 50>, // the number of requests we handle at the same time.
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
    /// It inserts the abort handle for the UDP request processor tasks.
    ///
    /// If there is no room for the new task, it tries to make place:
    ///
    /// - Firstly, removing finished tasks.
    /// - Secondly, removing the oldest unfinished tasks.
    ///
    /// # Panics
    ///
    /// Will panics if it can't make space for the new handle.
    pub async fn force_push(&mut self, abort_handle: AbortHandle, local_addr: &str) {
        // fill buffer with requests
        let Err(abort_handle) = self.rb.try_push(abort_handle) else {
            return;
        };

        let mut finished: u64 = 0;
        let mut unfinished_task = None;

        // buffer is full.. lets make some space.
        for h in self.rb.pop_iter() {
            // remove some finished tasks
            if h.is_finished() {
                finished += 1;
                continue;
            }

            // task is unfinished.. give it another chance.
            tokio::task::yield_now().await;

            // if now finished, we continue.
            if h.is_finished() {
                finished += 1;
                continue;
            }

            tracing::debug!(target: UDP_TRACKER_LOG_TARGET, local_addr, removed_count = finished, "Udp::run_udp_server::loop (got unfinished task)");

            if finished == 0 {
                // we have _no_ finished tasks.. will abort the unfinished task to make space...
                h.abort();

                tracing::warn!(target: UDP_TRACKER_LOG_TARGET, local_addr, "Udp::run_udp_server::loop aborting request: (no finished tasks)");

                break;
            }

            // we have space, return unfinished task for re-entry.
            unfinished_task = Some(h);
        }

        // re-insert the previous unfinished task.
        if let Some(h) = unfinished_task {
            self.rb.try_push(h).expect("it was previously inserted");
        }

        // insert the new task.
        if !abort_handle.is_finished() {
            self.rb
                .try_push(abort_handle)
                .expect("it should remove at least one element.");
        }
    }
}
