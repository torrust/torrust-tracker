//! Reusable scenario steps for qBittorrent E2E flows.
//!
//! Steps are grouped by subject:
//! - `fixtures` — test data builders (payload, torrent metadata)
//! - `qbittorrent` — qBittorrent client interaction steps
//!
//! Each leaf file contains one explicit step so available actions are discoverable in the IDE tree.

mod fixtures;
mod qbittorrent;

pub(super) use fixtures::{build_payload_fixture, build_torrent_fixture};
pub(super) use qbittorrent::{
    add_torrent_file_to_client, login_client, wait_until_client_has_any_torrent, wait_until_download_completes,
};
