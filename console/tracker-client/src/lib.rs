// The `console/` library modules contain CLI output logic (print!/println!/eprintln!)
// shared by all binary targets. Each binary already allows the print lints individually.
// We keep the crate-level allow because the printing lives in library code that the
// binaries call, not in the binaries themselves.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::time::Duration;

pub mod console;

pub(crate) const DEFAULT_NETWORK_TIMEOUT: Duration = Duration::from_secs(5);
