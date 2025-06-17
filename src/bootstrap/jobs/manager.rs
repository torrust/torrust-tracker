use std::time::Duration;

use tokio::task::JoinHandle;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

/// Represents a named background job.
#[derive(Debug)]
pub struct Job {
    name: String,
    handle: JoinHandle<()>,
}

impl Job {
    pub fn new<N: Into<String>>(name: N, handle: JoinHandle<()>) -> Self {
        Self {
            name: name.into(),
            handle,
        }
    }
}

/// Manages multiple background jobs.
#[derive(Debug, Default)]
pub struct JobManager {
    jobs: Vec<Job>,
    cancellation_token: CancellationToken,
}

impl JobManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            cancellation_token: CancellationToken::new(),
        }
    }

    pub fn push<N: Into<String>>(&mut self, name: N, handle: JoinHandle<()>) {
        self.jobs.push(Job::new(name, handle));
    }

    pub fn push_opt<N: Into<String>>(&mut self, name: N, handle: Option<JoinHandle<()>>) {
        if let Some(handle) = handle {
            self.push(name, handle);
        }
    }

    #[must_use]
    pub fn new_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.clone()
    }

    /// Cancels all jobs using the shared cancellation token.
    ///
    /// Notice that this does not cancel the jobs immediately, but rather
    /// signals them to stop. The jobs themselves must handle the cancellation
    /// token appropriately.
    ///
    /// Notice jobs might be pushed into the manager without a cancellation
    /// token, so this method will not cancel those jobs. Some tasks might
    /// decide to listen for CTRL+c signal directly, or implement their own
    /// cancellation logic.
    pub fn cancel(&self) {
        self.cancellation_token.cancel();
    }

    /// Waits sequentially for all jobs to complete, with a graceful timeout per
    /// job.
    pub async fn wait_for_all(mut self, grace_period: Duration) {
        for job in self.jobs.drain(..) {
            let name = job.name.clone();

            info!(job = %name, "Waiting for job to finish (timeout of {} seconds) ...", grace_period.as_secs());

            if let Ok(result) = timeout(grace_period, job.handle).await {
                if let Err(e) = result {
                    warn!(job = %name, "Job return an error: {:?}", e);
                } else {
                    info!(job = %name, "Job completed gracefully");
                }
            } else {
                warn!(job = %name, "Job did not complete in time");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::time::Duration;

    use super::*;

    #[tokio::test]
    async fn it_should_wait_for_all_jobs_to_finish() {
        let mut manager = JobManager::new();

        manager.push("job1", tokio::spawn(async {}));
        manager.push("job2", tokio::spawn(async {}));

        manager.wait_for_all(Duration::from_secs(1)).await;
    }

    #[tokio::test]
    async fn it_should_log_when_a_job_panics() {
        let mut manager = JobManager::new();

        manager.push(
            "panic_job",
            tokio::spawn(async {
                panic!("expected panic");
            }),
        );

        manager.wait_for_all(Duration::from_secs(1)).await;
    }
}
