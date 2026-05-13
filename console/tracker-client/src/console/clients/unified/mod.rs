//! Unified tracker-client command implementation.
//!
//! This module is the migration target for the mechanical copy-port in issue #1771.
//! It is intentionally isolated from legacy `http`, `udp`, and `checker` app entry points:
//! - New behavior and tests should be added here.
//! - Legacy binaries stay frozen except startup deprecation warnings.
//! - Once legacy binaries are removed, this module can be flattened in a dedicated cleanup.
pub mod app;
pub mod check;
pub mod http;
pub mod udp;
