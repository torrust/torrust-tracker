//! Whitelist API context.
//!
//! This API context is responsible for handling all the requests related to
//! the torrent whitelist.
//!
//! A torrent whitelist is a list of Info Hashes that are allowed to be tracked
//! by the tracker. This is useful when you want to limit the torrents that are
//! tracked by the tracker.
//!
//! Common tracker requests like `announce` and `scrape` are limited to the
//! torrents in the whitelist. The whitelist can be updated using the API.
//!
//! > **NOTICE**: the whitelist is only used when the tracker is configured to
//! in `listed` or `private_listed` modes. Refer to the
//! [configuration crate documentation](https://docs.rs/torrust-tracker-configuration)
//! to know how to enable the those modes.
//!
//! > **NOTICE**: if the tracker is not running in `listed` or `private_listed`
//! modes the requests to the whitelist API will be ignored.
//!
//! # Endpoints
//!
//! - [Add a torrent to the whitelist](#add-a-torrent-to-the-whitelist)
//! - [Remove a torrent from the whitelist](#remove-a-torrent-from-the-whitelist)
//! - [Reload the whitelist](#reload-the-whitelist)
//!
//! # Add a torrent to the whitelist
//!
//! `POST /whitelist/:info_hash`
//!
//! It adds a torrent infohash to the whitelist.
//!
//! **Path parameters**
//!
//! Name | Type | Description | Required | Example
//! ---|---|---|---|---
//! `info_hash` | 40-char string | The Info Hash v1 | Yes | `5452869be36f9f3350ccee6b4544e7e76caaadab`
//!
//! **Example request**
//!
//! ```bash
//! curl -X POST "http://127.0.0.1:1212/api/v1/whitelist/5452869be36f9f3350ccee6b4544e7e76caaadab?token=MyAccessToken"
//! ```
//!
//! **Example response** `200`
//!
//! ```json
//! {
//!     "status": "ok"
//! }
//! ```
//!
//! # Remove a torrent from the whitelist
//!
//! `DELETE /whitelist/:info_hash`
//!
//! It removes a torrent infohash to the whitelist.
//!
//! **Path parameters**
//!
//! Name | Type | Description | Required | Example
//! ---|---|---|---|---
//! `info_hash` | 40-char string | The Info Hash v1 | Yes | `5452869be36f9f3350ccee6b4544e7e76caaadab`
//!
//! **Example request**
//!
//! ```bash
//! curl -X DELETE "http://127.0.0.1:1212/api/v1/whitelist/5452869be36f9f3350ccee6b4544e7e76caaadab?token=MyAccessToken"
//! ```
//!
//! **Example response** `200`
//!
//! ```json
//! {
//!     "status": "ok"
//! }
//! ```
//!
//! # Reload the whitelist
//!
//! It reloads the whitelist from the database.
//!
//! **Example request**
//!
//! ```bash
//! curl "http://127.0.0.1:1212/api/v1/whitelist/reload?token=MyAccessToken"
//! ```
//!
//! **Example response** `200`
//!
//! ```json
//! {
//!     "status": "ok"
//! }
//! ```
pub mod handlers;
pub mod responses;
pub mod routes;
