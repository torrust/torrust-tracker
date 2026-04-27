//! qBittorrent client interaction steps for E2E scenarios.
//!
//! Each file contains one explicit step so available actions are discoverable in the IDE tree.

mod add_torrent_file_to_client;
mod ensure_torrent_is_absent;
mod login_client;
mod wait_until_download_completes;
mod wait_until_torrent_appears_in_client;

pub(in super::super) use add_torrent_file_to_client::add_torrent_file_to_client;
pub(in super::super) use ensure_torrent_is_absent::ensure_torrent_is_absent;
pub(in super::super) use login_client::login_client;
pub(in super::super) use wait_until_download_completes::wait_until_download_completes;
pub(in super::super) use wait_until_torrent_appears_in_client::wait_until_torrent_appears_in_client;
