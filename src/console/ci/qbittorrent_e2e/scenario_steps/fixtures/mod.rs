//! Fixture builders for qBittorrent E2E scenarios.
//!
//! Each file contains one builder so available fixtures are discoverable in the IDE tree.

mod build_payload_fixture;
mod build_torrent_fixture;

pub(in super::super) use build_payload_fixture::build_payload_fixture;
pub(in super::super) use build_torrent_fixture::build_torrent_fixture;
