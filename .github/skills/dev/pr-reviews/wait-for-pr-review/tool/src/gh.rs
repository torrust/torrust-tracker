//! The GitHub CLI boundary: the only place the poller talks to GitHub.

use std::ffi::OsString;
use std::io::Read;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::error::Error;

const CHILD_POLL_INTERVAL: Duration = Duration::from_millis(25);

/// How long the readers of a killed child get to finish. A killed child normally closes its pipes at
/// once, so they finish and are reclaimed. A grandchild that inherited a pipe can keep it open, and
/// waiting for that without limit would defeat the deadline, so the wait is bounded.
const RECLAIM_GRACE: Duration = Duration::from_millis(200);

/// Read access to the GitHub REST API. Tests fake this boundary.
pub trait Gh {
    /// Fails unless the GitHub CLI holds a valid login.
    ///
    /// # Errors
    ///
    /// Returns an error when `gh` cannot start or is not authenticated.
    fn check_authenticated(&self) -> Result<(), Error>;

    /// Returns the JSON document at `endpoint`.
    ///
    /// # Errors
    ///
    /// Returns an error when the request fails or the response is not JSON.
    fn get(&self, endpoint: &str) -> Result<Value, Error>;

    /// Returns every item of the paginated JSON array at `endpoint`, across all pages.
    ///
    /// # Errors
    ///
    /// Returns an error when a request fails or a response is not an array of JSON arrays.
    fn get_all(&self, endpoint: &str) -> Result<Vec<Value>, Error>;
}

/// The installed GitHub CLI. The poller holds no token; `gh` owns the authentication.
#[derive(Debug)]
pub struct GhCli {
    program: OsString,
    interrupted: Arc<AtomicBool>,
    deadline: Instant,
}

impl GhCli {
    /// A CLI handle that kills its running `gh` child as soon as `interrupted` is set or `deadline`
    /// passes, and refuses to start a child after `deadline`.
    #[must_use]
    pub fn new(program: impl Into<OsString>, interrupted: Arc<AtomicBool>, deadline: Instant) -> Self {
        Self {
            program: program.into(),
            interrupted,
            deadline,
        }
    }

    fn run(&self, arguments: &[&str]) -> Result<(ExitStatus, Vec<u8>, Vec<u8>), Error> {
        if Instant::now() >= self.deadline {
            return Err(Error::DeadlineExceeded);
        }

        let mut child = Command::new(&self.program)
            .args(arguments)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| Error::GhUnavailable {
                program: self.program.to_string_lossy().into_owned(),
                reason: error.to_string(),
            })?;

        let stdout = child.stdout.take().map(drain);
        let stderr = child.stderr.take().map(drain);

        let outcome = self.wait_for(&mut child);

        let status = match outcome {
            Ok(status) => status,
            Err(failure) => {
                // The child was killed. Reclaim what the readers finished, but never wait for a
                // descendant that still holds a pipe: the grace is the only bound here.
                let reclaim_deadline = Instant::now() + RECLAIM_GRACE;
                drop(collect(stdout, reclaim_deadline, &AtomicBool::new(false)));
                drop(collect(stderr, reclaim_deadline, &AtomicBool::new(false)));
                return Err(failure);
            }
        };

        // The child exited, but a descendant may still hold its pipes. Output that is not complete
        // when the budget ends or a signal arrives is a failure, never partial data.
        let stdout = collect(stdout, self.deadline, &self.interrupted)?;
        let stderr = collect(stderr, self.deadline, &self.interrupted)?;

        Ok((status, stdout, stderr))
    }

    /// Waits for `child`, killing and reaping it when a signal arrives or the deadline passes, so a
    /// stalled `gh` call never outlives the wait and no `gh` process is orphaned.
    fn wait_for(&self, child: &mut Child) -> Result<ExitStatus, Error> {
        loop {
            if let Some(status) = child.try_wait().map_err(|error| self.unavailable(&error))? {
                return Ok(status);
            }

            let failure = if self.interrupted.load(Ordering::Relaxed) {
                Error::Interrupted
            } else if Instant::now() >= self.deadline {
                Error::DeadlineExceeded
            } else {
                thread::sleep(CHILD_POLL_INTERVAL);
                continue;
            };

            drop(child.kill());
            drop(child.wait());
            return Err(failure);
        }
    }

    fn unavailable(&self, error: &std::io::Error) -> Error {
        Error::GhUnavailable {
            program: self.program.to_string_lossy().into_owned(),
            reason: error.to_string(),
        }
    }

    fn api(&self, endpoint: &str, extra: &[&str]) -> Result<Value, Error> {
        let mut arguments = vec!["api"];
        arguments.extend_from_slice(extra);
        arguments.push(endpoint);

        let (status, stdout, stderr) = self.run(&arguments)?;

        if !status.success() {
            return Err(Error::GhApi {
                endpoint: endpoint.to_owned(),
                message: String::from_utf8_lossy(&stderr).trim().to_owned(),
            });
        }

        serde_json::from_slice(&stdout).map_err(|error| Error::MalformedResponse {
            endpoint: endpoint.to_owned(),
            reason: error.to_string(),
        })
    }
}

impl Gh for GhCli {
    fn check_authenticated(&self) -> Result<(), Error> {
        let (status, stdout, stderr) = self.run(&["auth", "status"])?;

        if status.success() {
            Ok(())
        } else {
            let detail = [stderr, stdout]
                .iter()
                .map(|bytes| String::from_utf8_lossy(bytes).trim().to_owned())
                .find(|text| !text.is_empty())
                .unwrap_or_default();
            Err(Error::GhNotAuthenticated(detail))
        }
    }

    fn get(&self, endpoint: &str) -> Result<Value, Error> {
        self.api(endpoint, &[])
    }

    fn get_all(&self, endpoint: &str) -> Result<Vec<Value>, Error> {
        let pages = self.api(endpoint, &["--paginate", "--slurp"])?;

        let malformed = |reason: &str| Error::MalformedResponse {
            endpoint: endpoint.to_owned(),
            reason: reason.to_owned(),
        };

        pages
            .as_array()
            .ok_or_else(|| malformed("expected an array of pages"))?
            .iter()
            .map(|page| page.as_array().ok_or_else(|| malformed("expected every page to be an array")))
            .collect::<Result<Vec<_>, _>>()
            .map(|pages| pages.into_iter().flatten().cloned().collect())
    }
}

/// Reads a child pipe to the end on its own thread, so a large response never blocks the child, and
/// sends what it read when the pipe closes.
fn drain(mut pipe: impl Read + Send + 'static) -> Receiver<Vec<u8>> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut buffer = Vec::new();
        drop(pipe.read_to_end(&mut buffer));
        drop(sender.send(buffer));
    });

    receiver
}

/// What a reader read, waiting in slices of at most `CHILD_POLL_INTERVAL` until the pipe closes.
///
/// A reader that has not finished when `deadline` passes or `interrupted` is set fails with
/// `DeadlineExceeded` or `Interrupted`, and is left to finish on its own: a descendant that inherited
/// the pipe can keep it open long after the child exited.
fn collect(reader: Option<Receiver<Vec<u8>>>, deadline: Instant, interrupted: &AtomicBool) -> Result<Vec<u8>, Error> {
    let Some(reader) = reader else {
        return Ok(Vec::new());
    };

    loop {
        let slice = deadline.saturating_duration_since(Instant::now()).min(CHILD_POLL_INTERVAL);

        match reader.recv_timeout(slice) {
            Ok(bytes) => return Ok(bytes),
            Err(RecvTimeoutError::Disconnected) => return Ok(Vec::new()),
            Err(RecvTimeoutError::Timeout) => {}
        }

        if interrupted.load(Ordering::Relaxed) {
            return Err(Error::Interrupted);
        }
        if Instant::now() >= deadline {
            return Err(Error::DeadlineExceeded);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, mpsc};
    use std::thread;
    use std::time::{Duration, Instant};

    use super::collect;
    use crate::error::Error;

    fn far() -> Instant {
        Instant::now() + Duration::from_secs(60)
    }

    #[test]
    fn it_should_return_what_a_reader_sent_before_the_deadline() {
        let (sender, receiver) = mpsc::channel();
        sender.send(b"output".to_vec()).unwrap();

        let bytes = collect(Some(receiver), far(), &AtomicBool::new(false));

        assert_eq!(bytes.unwrap(), b"output");
    }

    #[test]
    fn it_should_return_what_a_reader_sent_even_when_the_deadline_has_already_passed() {
        let (sender, receiver) = mpsc::channel();
        sender.send(b"output".to_vec()).unwrap();

        let bytes = collect(Some(receiver), Instant::now(), &AtomicBool::new(false));

        assert_eq!(bytes.unwrap(), b"output");
    }

    #[test]
    fn it_should_fail_at_the_deadline_for_a_reader_whose_pipe_stays_open() {
        // The sender stays alive, like a reader whose pipe a descendant keeps open.
        let (_sender, receiver) = mpsc::channel::<Vec<u8>>();
        let started = Instant::now();

        let result = collect(Some(receiver), started + Duration::from_millis(50), &AtomicBool::new(false));

        assert!(matches!(result, Err(Error::DeadlineExceeded)));
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "the wait must be bounded by the deadline"
        );
    }

    #[test]
    fn it_should_fail_promptly_when_interrupted_while_a_reader_stays_silent() {
        let (_sender, receiver) = mpsc::channel::<Vec<u8>>();
        let interrupted = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&interrupted);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            flag.store(true, Ordering::Relaxed);
        });
        let started = Instant::now();

        let result = collect(Some(receiver), far(), &interrupted);

        assert!(matches!(result, Err(Error::Interrupted)));
        assert!(started.elapsed() < Duration::from_secs(5), "the signal must end the wait");
    }

    #[test]
    fn it_should_return_what_a_reader_sends_while_the_wait_is_in_progress() {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(60));
            sender.send(b"late".to_vec()).unwrap();
        });

        let bytes = collect(Some(receiver), far(), &AtomicBool::new(false));

        assert_eq!(bytes.unwrap(), b"late");
    }

    #[test]
    fn it_should_return_nothing_for_a_pipe_that_does_not_exist() {
        assert_eq!(collect(None, Instant::now(), &AtomicBool::new(false)).unwrap(), b"");
    }
}
