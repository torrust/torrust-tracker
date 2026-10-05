//! Fake trackers for `tracker_client` integration tests.
//!
//! Each fake keeps only the first N info hashes of a scrape, like a tracker that
//! truncates silently, so tests can check client behavior for any tracker limit
//! without depending on the tracker servers.
mod http;
mod udp;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub use http::FakeHttpTracker;
pub use udp::FakeUdpTracker;

/// How often a fake tracker checks whether it should stop.
const POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Runs a fake tracker loop in a thread and stops it when dropped.
struct ServerThread {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl ServerThread {
    fn spawn(serve: impl FnOnce(&AtomicBool) + Send + 'static) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let handle = thread::spawn({
            let stop = stop.clone();
            move || serve(&stop)
        });

        Self {
            stop,
            handle: Some(handle),
        }
    }
}

impl Drop for ServerThread {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);

        if let Some(handle) = self.handle.take() {
            let joined = handle.join();

            if !thread::panicking() {
                assert!(joined.is_ok(), "fake tracker thread panicked");
            }
        }
    }
}
