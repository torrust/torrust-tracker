//! The GitHub CLI boundary: the only place the poller talks to GitHub.

use std::ffi::OsString;
use std::io::Read;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::error::Error;

const CHILD_POLL_INTERVAL: Duration = Duration::from_millis(25);

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

        let status = self.wait_for(&mut child)?;

        Ok((status, join(stdout), join(stderr)))
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

/// Reads a child pipe to the end on its own thread, so a large response never blocks the child.
fn drain(mut pipe: impl Read + Send + 'static) -> JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut buffer = Vec::new();
        drop(pipe.read_to_end(&mut buffer));
        buffer
    })
}

fn join(handle: Option<JoinHandle<Vec<u8>>>) -> Vec<u8> {
    handle.and_then(|handle| handle.join().ok()).unwrap_or_default()
}
