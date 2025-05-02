//! Swarm Data Structures.
//!
//! This module defines the primary data structures used to store and manage
//! swarm data within the tracker. In `BitTorrent` terminology, a "swarm" is
//! the collection of peers that are sharing or downloading a given torrent.
//!
//! There are two main types of data stored:
//!
//! - **Torrent Entry** (`Entry`): Contains all the information the tracker
//!   stores for a single torrent, including the list of peers currently in the
//!   swarm. This data is crucial for peers to locate each other and initiate
//!   downloads.
//!
//! - **Swarm Metadata** (`SwarmMetadata`): Contains aggregate data derived from
//!   all torrent entries. This metadata is split into:
//!   - **Active Peers Data:** Metrics related to the peers that are currently
//!     active in the swarm.
//!   - **Historical Data:** Metrics collected since the tracker started, such
//!     as the total number of completed downloads.
//!
//! ## Metrics Collected
//!
//! The tracker collects and aggregates the following metrics:
//!
//! - The total number of peers that have completed downloading the torrent
//!   since the tracker began collecting metrics.
//! - The number of completed downloads from peers that remain active (i.e., seeders).
//! - The number of active peers that have not completed downloading the torrent (i.e., leechers).
//!
//! This information is used both to inform peers about available connections
//! and to provide overall swarm statistics.
//!
//! This module re-exports core types from the torrent repository crate to
//! simplify integration.
//!
//! ## Internal Data Structures
//!
//! The [`torrent`](crate::torrent) module contains all the data structures
//! stored by the tracker except for peers.
//!
//! We can represent the data stored in memory internally by the tracker with
//! this JSON object:
//!
//! ```json
//! {
//!     "c1277613db1d28709b034a017ab2cae4be07ae10": {
//!         "completed": 0,
//!         "peers": {
//!             "-qB00000000000000001": {
//!                 "peer_id": "-qB00000000000000001",
//!                 "peer_addr": "2.137.87.41:1754",
//!                 "updated": 1672419840,
//!                 "uploaded": 120,
//!                 "downloaded": 60,
//!                 "left": 60,
//!                 "event": "started"
//!             },
//!             "-qB00000000000000002": {
//!                 "peer_id": "-qB00000000000000002",
//!                 "peer_addr": "23.17.287.141:2345",
//!                 "updated": 1679415984,
//!                 "uploaded": 80,
//!                 "downloaded": 20,
//!                 "left": 40,
//!                 "event": "started"
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! The tracker maintains an indexed-by-info-hash list of torrents. For each
//! torrent, it stores a torrent `Entry`. The torrent entry has two attributes:
//!
//! - `completed`: which is hte number of peers that have completed downloading
//!   the torrent file/s. As they have completed downloading, they have a full
//!   version of the torrent data, and they can provide the full data to other
//!   peers. That's why they are also known as "seeders".
//! - `peers`: an indexed and orderer list of peer for the torrent. Each peer
//!   contains the data received from the peer in the `announce` request.
//!
//! The [`crate::torrent`] module not only contains the original data obtained
//! from peer via `announce` requests, it also contains aggregate data that can
//! be derived from the original data. For example:
//!
//! ```rust,no_run
//! pub struct SwarmMetadata {
//!     pub complete: u32,   // The number of active peers that have completed downloading (seeders)
//!     pub downloaded: u32, // The number of peers that have ever completed downloading
//!     pub incomplete: u32, // The number of active peers that have not completed downloading (leechers)
//! }
//! ```
//!
//! > **NOTICE**: that `complete` or `completed` peers are the peers that have
//! > completed downloading, but only the active ones are considered "seeders".
//!
//! `SwarmMetadata` struct follows name conventions for `scrape` responses. See
//! [BEP 48](https://www.bittorrent.org/beps/bep_0048.html), while `SwarmMetadata`
//! is used for the rest of cases.
//!
//! ## Peers
//!
//! A `Peer` is the struct used by the tracker to keep peers data:
//!
//! ```rust,no_run
//! use std::net::SocketAddr;
//! use aquatic_udp_protocol::PeerId;
//! use torrust_tracker_primitives::DurationSinceUnixEpoch;
//! use aquatic_udp_protocol::NumberOfBytes;
//! use aquatic_udp_protocol::AnnounceEvent;
//!
//! pub struct Peer {
//!     pub peer_id: PeerId,                 // The peer ID
//!     pub peer_addr: SocketAddr,           // Peer socket address
//!     pub updated: DurationSinceUnixEpoch, // Last time (timestamp) when the peer was updated
//!     pub uploaded: NumberOfBytes,         // Number of bytes the peer has uploaded so far
//!     pub downloaded: NumberOfBytes,       // Number of bytes the peer has downloaded so far   
//!     pub left: NumberOfBytes,             // The number of bytes this peer still has to download
//!     pub event: AnnounceEvent,            // The event the peer has announced: `started`, `completed`, `stopped`
//! }
//! ```
//!
//! Notice that most of the attributes are obtained from the `announce` request.
//! For example, an HTTP announce request would contain the following `GET` parameters:
//!
//! <http://0.0.0.0:7070/announce?info_hash=%81%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_addr=2.137.87.41&downloaded=0&uploaded=0&peer_id=-qB00000000000000001&port=17548&left=0&event=completed&compact=0>
//!
//! The `Tracker` keeps an in-memory ordered data structure with all the torrents and a list of peers for each torrent, together with some swarm metrics.
//!
//! We can represent the data stored in memory with this JSON object:
//!
//! ```json
//! {
//!     "c1277613db1d28709b034a017ab2cae4be07ae10": {
//!         "completed": 0,
//!         "peers": {
//!             "-qB00000000000000001": {
//!                 "peer_id": "-qB00000000000000001",
//!                 "peer_addr": "2.137.87.41:1754",
//!                 "updated": 1672419840,
//!                 "uploaded": 120,
//!                 "downloaded": 60,
//!                 "left": 60,
//!                 "event": "started"
//!             },
//!             "-qB00000000000000002": {
//!                 "peer_id": "-qB00000000000000002",
//!                 "peer_addr": "23.17.287.141:2345",
//!                 "updated": 1679415984,
//!                 "uploaded": 80,
//!                 "downloaded": 20,
//!                 "left": 40,
//!                 "event": "started"
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! That JSON object does not exist, it's only a representation of the `Tracker` torrents data.
//!
//! `c1277613db1d28709b034a017ab2cae4be07ae10` is the torrent infohash and `completed` contains the number of peers
//! that have a full version of the torrent data, also known as seeders.
//!
//! Refer to [`peer`](torrust_tracker_primitives::peer) for more information about peers.
pub mod manager;
pub mod repository;
pub mod services;
