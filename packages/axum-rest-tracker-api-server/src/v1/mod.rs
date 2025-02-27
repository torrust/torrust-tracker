//! The API version `v1`.
//!
//! The API is organized in the following contexts:
//!
//! Context | Description | Version
//! ---|---|---
//! `Stats` | Tracker statistics | [`v1`](crate::v1::context::stats)
//! `Torrents` | Torrents | [`v1`](crate::v1::context::torrent)
//! `Whitelist` | Torrents whitelist | [`v1`](crate::v1::context::whitelist)
//! `Authentication keys` | Authentication keys | [`v1`](crate::v1::context::auth_key)
//!
//! > **NOTICE**:
//! - The authentication keys are only used by the HTTP tracker.
//! - The whitelist is only used when the tracker is running in `listed` or
//!   `private_listed` mode.
//!
//! Refer to the [authentication middleware](crate::v1::middlewares::auth)
//! for more information about the authentication process.
pub mod context;
pub mod middlewares;
pub mod responses;
pub mod routes;
