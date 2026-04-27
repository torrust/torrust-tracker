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
mod verify_payload_integrity;

pub(super) use fixtures::{build_payload_fixture, build_torrent_fixture};
pub(super) use qbittorrent::{
    add_torrent_file_to_client, login_client, wait_until_client_has_any_torrent, wait_until_download_completes,
};
pub(super) use verify_payload_integrity::verify_payload_integrity;
