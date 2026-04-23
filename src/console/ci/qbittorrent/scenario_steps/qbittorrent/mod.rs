//! qBittorrent client interaction steps for E2E scenarios.
//!
//! Each file contains one explicit step so available actions are discoverable in the IDE tree.

mod add_torrent_file_to_client;
mod login_client;
mod wait_until_client_has_any_torrent;
mod wait_until_download_completes;
mod wait_until_temporary_password_appears_in_logs;

pub(in super::super) use add_torrent_file_to_client::add_torrent_file_to_client;
pub(in super::super) use login_client::login_client;
pub(in super::super) use wait_until_client_has_any_torrent::wait_until_client_has_any_torrent;
pub(in super::super) use wait_until_download_completes::wait_until_download_completes;
pub(in super::super) use wait_until_temporary_password_appears_in_logs::wait_until_temporary_password_appears_in_logs;
