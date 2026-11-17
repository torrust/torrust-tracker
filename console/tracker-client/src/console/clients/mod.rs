//! Console clients.
//!
//! `unified` contains the single-binary implementation (`tracker_client`).
//! The legacy `http`, `udp` and `checker` modules are the libraries the removed
//! `http_tracker_client`, `udp_tracker_client` and `tracker_checker` binaries used;
//! `tracker_client` still reuses their checks and types.
pub mod checker;
pub mod http;
pub mod udp;
pub mod unified;
