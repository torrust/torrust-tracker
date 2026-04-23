//! Reusable scenario steps for qBittorrent E2E flows.
//!
//! Each file contains one explicit step so available actions are discoverable in the IDE tree.

mod add_torrent_file_to_client;
mod add_torrent_file_to_leecher;
mod build_payload_fixture;
mod build_torrent_fixture;
mod login_client;
mod wait_until_client_has_any_torrent;
mod wait_until_download_completes;
mod wait_until_temporary_password_appears_in_logs;

pub(super) use add_torrent_file_to_client::add_torrent_file_to_client;
pub(super) use add_torrent_file_to_leecher::add_torrent_file_to_leecher;
pub(super) use build_payload_fixture::build_payload_fixture;
pub(super) use build_torrent_fixture::build_torrent_fixture;
pub(super) use login_client::login_client;
pub(super) use wait_until_client_has_any_torrent::wait_until_client_has_any_torrent;
pub(super) use wait_until_download_completes::wait_until_download_completes;
pub(super) use wait_until_temporary_password_appears_in_logs::wait_until_temporary_password_appears_in_logs;
