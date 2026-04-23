//! Reusable scenario steps for qBittorrent E2E flows.
//!
//! Each file contains one explicit step so available actions are discoverable in the IDE tree.

mod add_torrent_file_to_client;
mod build_payload_fixture;
mod build_torrent_fixture;
mod wait_until_client_can_login;
mod wait_until_client_has_any_torrent;

pub(super) use add_torrent_file_to_client::add_torrent_file_to_client;
pub(super) use build_payload_fixture::build_payload_fixture;
pub(super) use build_torrent_fixture::build_torrent_fixture;
pub(super) use wait_until_client_can_login::{wait_until_client_can_login, LoginReadinessSettings};
pub(super) use wait_until_client_has_any_torrent::wait_until_client_has_any_torrent;
