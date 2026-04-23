use std::time::{Duration, Instant};

use tokio::time::sleep;

pub(super) struct Poller {
    deadline: Instant,
    interval: Duration,
}

impl Poller {
    pub(super) fn new(timeout: Duration, interval: Duration) -> Self {
        Self {
            deadline: Instant::now() + timeout,
            interval,
        }
    }

    pub(super) async fn retry_or_timeout<M>(&self, timeout_message: M) -> anyhow::Result<()>
    where
        M: FnOnce() -> String,
    {
        if Instant::now() >= self.deadline {
            anyhow::bail!(timeout_message());
        }

        sleep(self.interval).await;

        Ok(())
    }
}
