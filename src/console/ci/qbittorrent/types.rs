//! Small domain types shared across the `qBittorrent` E2E module.
//!
//! Most types here follow the newtype pattern: a thin wrapper around a primitive
//! that gives the value a precise, self-documenting type at every call site.
use std::fmt;
use std::ops::Deref;
use std::path::Path;
use std::time::Duration;

/// A file name (base name only, no path separators).
///
/// Wraps a [`String`] and provides [`Deref`] to `str` so values can be used
/// directly wherever `&str` is expected, and [`AsRef<Path>`] so they can be
/// passed to [`Path::join`].
#[derive(Debug, Clone)]
pub(crate) struct FileName(String);

impl FileName {
    /// Creates a new [`FileName`] from any value that converts into a [`String`].
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl Deref for FileName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for FileName {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for FileName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for FileName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// An absolute path inside a Docker container (e.g. `"/downloads"`).
///
/// Distinct from host [`PathBuf`]s: a `ContainerPath` is always a
/// Linux-style absolute path that exists only within the container
/// file-system, never on the host.
///
/// [`PathBuf`]: std::path::PathBuf
#[derive(Debug, Clone)]
pub(crate) struct ContainerPath(String);

impl ContainerPath {
    /// Creates a new [`ContainerPath`] from any value that converts into a [`String`].
    pub(crate) fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }
}

impl Deref for ContainerPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ContainerPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ContainerPath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ContainerPath {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// A polling-loop deadline expressed as a [`Duration`] measured from the moment
/// the loop starts.
///
/// Wraps a [`Duration`] representing the *maximum time* a polling loop may wait
/// before giving up.  Keeping it distinct from [`PollInterval`] turns an
/// accidental swap into a compile error instead of a silent logic bug.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Deadline(Duration);

impl Deadline {
    /// Creates a new [`Deadline`] from a [`Duration`].
    pub(crate) fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the underlying [`Duration`].
    pub(crate) fn as_duration(&self) -> Duration {
        self.0
    }
}

/// The sleep duration between successive retries in a polling loop.
///
/// Wraps a [`Duration`].  Distinct from [`Deadline`] so that the two cannot
/// be accidentally swapped at a call site.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PollInterval(Duration);

impl PollInterval {
    /// Creates a new [`PollInterval`] from a [`Duration`].
    pub(crate) fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the underlying [`Duration`].
    pub(crate) fn as_duration(&self) -> Duration {
        self.0
    }
}
