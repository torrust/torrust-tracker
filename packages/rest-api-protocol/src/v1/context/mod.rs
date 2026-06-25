//! API resources (DTOs) for the v1 REST API contract, organized by context.
//!
//! Each submodule corresponds to an API context. Resources for each context
//! live under its `resources/` subdirectory.
pub mod health_check;
pub mod torrent;
pub mod whitelist;
