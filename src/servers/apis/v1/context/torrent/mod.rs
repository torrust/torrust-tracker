//! Torrents API context.
//!
//! This API context is responsible for handling all the requests related to
//! the torrents data stored by the tracker.
//!
//! # Endpoints
//!
//! - [Get a torrent](#get-a-torrent)
//! - [List torrents](#list-torrents)
//!
//! # Get a torrent
//!
//! `GET /torrent/:info_hash`
//!
//! Returns all the information about a torrent.
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
//! curl "http://127.0.0.1:1212/api/v1/torrent/5452869be36f9f3350ccee6b4544e7e76caaadab?token=MyAccessToken"
//! ```
//!
//! **Example response** `200`
//!
//! ```json
//! {
//!     "info_hash": "5452869be36f9f3350ccee6b4544e7e76caaadab",
//!     "seeders": 1,
//!     "completed": 0,
//!     "leechers": 0,
//!     "peers": [
//!       {
//!         "peer_id": {
//!           "id": "0x2d7142343431302d2a64465a3844484944704579",
//!           "client": "qBittorrent"
//!         },
//!         "peer_addr": "192.168.1.88:17548",
//!         "updated": 1680082693001,
//!         "updated_milliseconds_ago": 1680082693001,
//!         "uploaded": 0,
//!         "downloaded": 0,
//!         "left": 0,
//!         "event": "None"
//!       }
//!     ]
//! }
//! ```
//!
//! **Not Found response** `200`
//!
//! This response is returned when the tracker does not have the torrent.
//!
//! ```json
//! "torrent not known"
//! ```
//!
//! **Resource**
//!
//! Refer to the API [`Torrent`](crate::servers::apis::v1::context::torrent::resources::torrent::Torrent)
//! resource for more information about the response attributes.
//!
//! # List torrents
//!
//! `GET /torrents`
//!
//! Returns basic information (no peer list) for all torrents.
//!
//! **Query parameters**
//!
//! The endpoint supports pagination.
//!
//! Name | Type | Description | Required | Example
//! ---|---|---|---|---
//! `offset` | positive integer | The page number, starting at 0 | No | `1`
//! `limit` | positive integer | Page size. The number of results per page | No | `10`
//!
//! **Example request**
//!
//! ```bash
//! curl "http://127.0.0.1:1212/api/v1/torrents?token=MyAccessToken&offset=1&limit=1"
//! ```
//!
//! **Example response** `200`
//!
//! ```json
//! [
//!     {
//!       "info_hash": "5452869be36f9f3350ccee6b4544e7e76caaadab",
//!       "seeders": 1,
//!       "completed": 0,
//!       "leechers": 0,
//!       "peers": null
//!     }
//! ]
//! ```
//!
//! **Resource**
//!
//! Refer to the API [`ListItem`](crate::servers::apis::v1::context::torrent::resources::torrent::ListItem)
//! resource for more information about the attributes for a single item in the
//! response.
//!
//! > **NOTICE**: this endpoint does not include the `peers` list.
pub mod handlers;
pub mod resources;
pub mod responses;
pub mod routes;
