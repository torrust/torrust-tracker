//! Authentication keys API context.
//!
//! Authentication keys are used to authenticate HTTP tracker `announce` and
//! `scrape` requests.
//!
//! When the tracker is running in `private` mode, the authentication keys are
//! required to announce and scrape torrents.
//!
//! A sample `announce` request **without** authentication key:
//!
//! <http://0.0.0.0:7070/announce?info_hash=12345678901234567890&peer_id=ABCDEFGHIJKLMNOPQRST&ip=255.255.255.255&port=6881&downloaded=1234&left=98765&event=stopped>
//!
//! A sample `announce` request **with** authentication key:
//!
//! <http://0.0.0.0:7070/announce/YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ?info_hash=12345678901234567890&peer_id=ABCDEFGHIJKLMNOPQRST&ip=255.255.255.255&port=6881&downloaded=1234&left=98765&event=stopped>
//!
//! # Endpoints
//!
//! - [Generate a new authentication key](#generate-a-new-authentication-key)
//! - [Delete an authentication key](#delete-an-authentication-key)
//! - [Reload authentication keys](#reload-authentication-keys)
//!
//! # Generate a new authentication key
//!
//! `POST /keys`
//!
//! It generates a new authentication key or upload a pre-generated key.
//!
//! **POST parameters**
//!
//! Name | Type | Description | Required | Example
//! ---|---|---|---|---
//! `key` | 32-char string (0-9, a-z, A-Z) or `null` | The optional pre-generated key. | Yes | `Xc1L4PbQJSFGlrgSRZl8wxSFAuMa21z7` or `null`
//! `seconds_valid` | positive integer or `null` | The number of seconds the key will be valid. | Yes | `3600` or `null`
//!
//! > **NOTICE**: the `key` and `seconds_valid` fields are optional. If `key` is not provided the tracker
//! > will generated a random one. If `seconds_valid` field is not provided the key will be permanent. You can use the `null` value.
//!
//! **Example request**
//!
//! ```bash
//! curl -X POST http://localhost:1212/api/v1/keys?token=MyAccessToken \
//!      -H "Content-Type: application/json" \
//!      -d '{
//!            "key": "xqD6NWH9TcKrOCwDmqcdH5hF5RrbL0A6",
//!            "seconds_valid": 7200
//!          }'
//! ```
//!
//! **Example response** `200`
//!
//! ```json
//! {
//!     "key": "xqD6NWH9TcKrOCwDmqcdH5hF5RrbL0A6",
//!     "valid_until": 1680009900,
//!     "expiry_time": "2023-03-28 13:25:00.058085050 UTC"
//! }
//! ```
//!
//! > **NOTICE**: `valid_until` and `expiry_time` represent the same time.
//! > `valid_until` is the number of seconds since the Unix epoch
//! > ([timestamp](https://en.wikipedia.org/wiki/Timestamp)), while `expiry_time`
//! > is the human-readable time ([ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html)).
//!
//! **Resource**
//!
//! Refer to the API [`AuthKey`](crate::v1::context::auth_key::resources::AuthKey)
//! resource for more information about the response attributes.
//!
//! # Delete an authentication key
//!
//! `DELETE /key/:key`
//!
//! It deletes a previously generated authentication key.
//!
//! **Path parameters**
//!
//! Name | Type | Description | Required | Example
//! ---|---|---|---|---
//! `key` | 40-char string | The `key` to remove. | Yes | `xqD6NWH9TcKrOCwDmqcdH5hF5RrbL0A6`
//!
//! **Example request**
//!
//! ```bash
//! curl -X DELETE "http://127.0.0.1:1212/api/v1/key/xqD6NWH9TcKrOCwDmqcdH5hF5RrbL0A6?token=MyAccessToken"
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
//! It you try to delete a non-existent key, the response will be an error with
//! a `500` status code.
//!
//! **Example error response** `500`
//!
//! ```text
//! Unhandled rejection: Err { reason: "failed to delete key: Failed to remove record from Sqlite3 database, error-code: 0, src/tracker/databases/sqlite.rs:267:27" }
//! ```
//!
//! > **NOTICE**: a `500` status code will be returned and the body is not a
//! > valid JSON. It's a text body containing the serialized-to-display error
//! > message.
//!
//! # Reload authentication keys
//!
//! `GET /keys/reload`
//!
//! The tracker persists the authentication keys in a database. This endpoint
//! reloads the keys from the database.
//!
//! **Example request**
//!
//! ```bash
//! curl "http://127.0.0.1:1212/api/v1/keys/reload?token=MyAccessToken"
//! ```
//!
//! **Example response** `200`
//!
//! ```json
//! {
//!     "status": "ok"
//! }
//! ```
pub mod forms;
pub mod handlers;
pub mod resources;
pub mod responses;
pub mod routes;
