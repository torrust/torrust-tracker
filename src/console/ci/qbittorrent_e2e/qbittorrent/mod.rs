//! Staged feature module for qBittorrent-specific internals.
//!
//! During the migration this module re-exports symbols from legacy files so
//! call sites can switch imports incrementally.

mod client;
mod config_builder;
mod credentials;
mod torrent;

pub(super) use client::QbittorrentClient;
pub(super) use config_builder::QbittorrentConfigBuilder;
pub(super) use credentials::QbittorrentCredentials;
#[expect(unused_imports, reason = "staged migration re-export")]
pub(super) use torrent::{TorrentInfo, TorrentProgress, TorrentState};
