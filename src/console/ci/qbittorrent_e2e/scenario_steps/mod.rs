//! Reusable scenario steps for qBittorrent E2E flows.
//!
//! Steps are grouped by subject:
//! - `fixtures` — test data builders (payload, torrent metadata)
//! - `qbittorrent` — qBittorrent client interaction steps
//! - `verify_payload_integrity` — assert that a downloaded file matches the original payload
//!
//! Each leaf file contains one explicit step so available actions are discoverable in the IDE tree.

mod fixtures;
mod qbittorrent;
mod tracker;
mod verify_payload_integrity;

pub(super) use fixtures::{build_payload_fixture, build_torrent_fixture};
pub(super) use qbittorrent::{
    add_torrent_file_to_client, ensure_torrent_is_absent, login_client, wait_until_download_completes,
    wait_until_torrent_appears_in_client,
};
pub(super) use tracker::verify_tracker_swarm;
pub(super) use verify_payload_integrity::verify_payload_integrity;
