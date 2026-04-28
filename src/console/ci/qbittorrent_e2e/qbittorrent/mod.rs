//! Staged feature module for qBittorrent-specific internals.
//!
//! During the migration this module re-exports symbols from legacy files so
//! call sites can switch imports incrementally.

mod client;
mod config_builder;
mod credentials;
mod torrent;

/// Default port on which the qBittorrent `WebUI` listens.
///
/// Used both when writing the per-client config file ([`QbittorrentConfigBuilder`])
/// and when connecting to the container's `WebUI` ([`QbittorrentClient`]).
/// Keeping it here ensures both sides always agree on the same value.
pub(super) const QBITTORRENT_WEBUI_PORT: u16 = 8080;

pub(super) use client::QbittorrentClient;
pub(super) use config_builder::QbittorrentConfigBuilder;
pub(super) use credentials::QbittorrentCredentials;
// These re-exports are staged ahead of use: they will be consumed once
// additional scenario steps reference `TorrentState` / `TorrentProgress`
// directly. Tracked: <https://github.com/torrust/torrust-tracker/issues/1706>.
#[expect(unused_imports, reason = "staged migration re-export; see #1706")]
pub(super) use torrent::{TorrentInfo, TorrentProgress, TorrentState};
