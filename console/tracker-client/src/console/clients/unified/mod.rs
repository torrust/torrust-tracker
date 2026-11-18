//! Unified tracker-client command implementation.
//!
//! This module is the only command entry point of the crate: the legacy `http`, `udp` and
//! `checker` app entry points were removed with their binaries (issues #2765 and #2768).
//! Everything the binary writes goes through [`output`].
//!
//! Sub-modules are kept as flat files (no per-action nesting). See the design decision in
//! `docs/issues/open/1771-merge-clients-into-unified-tracker-client-cli.md`.
pub mod app;
pub mod check;
pub mod http;
pub mod output;
pub mod scrape;
pub mod udp;
