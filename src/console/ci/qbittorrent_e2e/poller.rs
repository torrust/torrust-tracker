use std::time::{Duration, Instant};

use tokio::time::sleep;

use super::types::{Deadline, PollInterval};

pub(super) struct Poller {
    deadline: Instant,
    interval: Duration,
}

impl Poller {
    pub(super) fn new(timeout: Deadline, interval: PollInterval) -> Self {
        Self {
            deadline: Instant::now() + timeout.as_duration(),
            interval: interval.as_duration(),
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
