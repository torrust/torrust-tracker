//! API is organized in resource groups called contexts.
//!
//! Each context is a module that contains the API endpoints related to a
//! specific resource group.
pub mod auth_key;
pub mod stats;
pub mod torrent;
pub mod whitelist;
