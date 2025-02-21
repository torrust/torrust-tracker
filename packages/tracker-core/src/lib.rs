//! The core `bittorrent-tracker-core` crate contains the generic `BitTorrent`
//! tracker logic which is independent of the delivery layer.
//!
//! It contains the tracker services and their dependencies. It's a domain layer
//!  which does not specify how the end user should connect to the `Tracker`.
//!
//! Typically this crate is intended to be used by higher components like:
//!
//! - A UDP tracker
//! - A HTTP tracker
//! - A tracker REST API
//!
//! ```text
//!   Delivery layer  |   Domain layer
//! -----------------------------------
//!     HTTP tracker  |
//!      UDP tracker  |-> Core tracker
//! Tracker REST API  |
//! ```
//!
//! # Table of contents
//!
//! - [Introduction](#introduction)
//! - [Configuration](#configuration)
//! - [Announce handler](#announce-handler)
//! - [Scrape handler](#scrape-handler)
//! - [Authentication](#authentication)
//! - [Databases](#databases)
//! - [Torrent](#torrent)
//! - [Whitelist](#whitelist)
//!
//! # Introduction
//!
//! The main purpose of this crate is to provide a generic `BitTorrent` tracker.
//!
//! It has two main responsibilities:
//!
//! - To handle **announce** requests.
//! - To handle **scrape** requests.
//!
//! The crate has also other features:
//!
//! - **Authentication**: It handles authentication keys which are used by HTTP trackers.
//! - **Persistence**: It handles persistence of data into a database.
//! - **Torrent**: It handles the torrent data.
//! - **Whitelist**: When the tracker runs in [`listed`](https://docs.rs/torrust-tracker-configuration/latest/torrust_tracker_configuration/type.Core.html) mode
//!   all operations are restricted to whitelisted torrents.
//!
//! Refer to [torrust-tracker-configuration](https://docs.rs/torrust-tracker-configuration)
//! crate docs to get more information about the tracker settings.
//!
//! # Configuration
//!
//! You can control the behavior of this crate with the `Core` settings:
//!
//! ```toml
//! [logging]
//! threshold = "debug"
//!
//! [core]
//! inactive_peer_cleanup_interval = 600
//! listed = false
//! private = false
//! tracker_usage_statistics = true
//!
//! [core.announce_policy]
//! interval = 120
//! interval_min = 120
//!
//! [core.database]
//! driver = "sqlite3"
//! path = "./storage/tracker/lib/database/sqlite3.db"
//!
//! [core.net]
//! on_reverse_proxy = false
//! external_ip = "2.137.87.41"
//!
//! [core.tracker_policy]
//! max_peer_timeout = 900
//! persistent_torrent_completed_stat = false
//! remove_peerless_torrents = true
//! ```
//!
//! Refer to the [`configuration` module documentation](https://docs.rs/torrust-tracker-configuration) to get more information about all options.
//!
//! # Announce handler
//!
//! The `AnnounceHandler` is responsible for handling announce requests.
//!
//! Please refer to the [`announce_handler`] documentation.
//!
//! # Scrape handler
//!
//! The `ScrapeHandler` is responsible for handling scrape requests.
//!
//! Please refer to the [`scrape_handler`] documentation.
//!
//! # Authentication
//!
//! The `Authentication` module is responsible for handling authentication keys which are used by HTTP trackers.
//!
//! Please refer to the [`authentication`] documentation.
//!
//! # Databases
//!
//! The `Databases` module is responsible for handling persistence of data into a database.
//!
//! Please refer to the [`databases`] documentation.
//!
//! # Torrent
//!
//! The `Torrent` module is responsible for handling the torrent data.
//!
//! Please refer to the [`torrent`] documentation.
//!
//! # Whitelist
//!
//! The `Whitelist` module is responsible for handling the whitelist.
//!
//! Please refer to the [`whitelist`] documentation.
pub mod announce_handler;
pub mod authentication;
pub mod container;
pub mod databases;
pub mod error;
pub mod scrape_handler;
pub mod torrent;
pub mod whitelist;

pub mod peer_tests;
pub mod test_helpers;

use torrust_tracker_clock::clock;
/// This code needs to be copied into each crate.
/// Working version, for production.
#[cfg(not(test))]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Working;

/// Stopped version, for testing.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) type CurrentClock = clock::Stopped;

#[cfg(test)]
mod tests {
    mod the_tracker {
        use std::sync::Arc;

        use torrust_tracker_test_helpers::configuration;

        use crate::announce_handler::AnnounceHandler;
        use crate::scrape_handler::ScrapeHandler;
        use crate::test_helpers::tests::initialize_handlers;

        fn initialize_handlers_for_public_tracker() -> (Arc<AnnounceHandler>, Arc<ScrapeHandler>) {
            let config = configuration::ephemeral_public();
            initialize_handlers(&config)
        }

        fn initialize_handlers_for_listed_tracker() -> (Arc<AnnounceHandler>, Arc<ScrapeHandler>) {
            let config = configuration::ephemeral_listed();
            initialize_handlers(&config)
        }

        mod for_all_config_modes {

            mod handling_a_scrape_request {

                use std::net::{IpAddr, Ipv4Addr};

                use bittorrent_primitives::info_hash::InfoHash;
                use torrust_tracker_primitives::core::ScrapeData;
                use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

                use crate::announce_handler::PeersWanted;
                use crate::test_helpers::tests::{complete_peer, incomplete_peer};
                use crate::tests::the_tracker::initialize_handlers_for_public_tracker;

                #[tokio::test]
                async fn it_should_return_the_swarm_metadata_for_the_requested_file_if_the_tracker_has_that_torrent() {
                    let (announce_handler, scrape_handler) = initialize_handlers_for_public_tracker();

                    let info_hash = "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(); // DevSkim: ignore DS173237

                    // Announce a "complete" peer for the torrent
                    let mut complete_peer = complete_peer();
                    announce_handler
                        .announce(
                            &info_hash,
                            &mut complete_peer,
                            &IpAddr::V4(Ipv4Addr::new(126, 0, 0, 10)),
                            &PeersWanted::AsManyAsPossible,
                        )
                        .await
                        .unwrap();

                    // Announce an "incomplete" peer for the torrent
                    let mut incomplete_peer = incomplete_peer();
                    announce_handler
                        .announce(
                            &info_hash,
                            &mut incomplete_peer,
                            &IpAddr::V4(Ipv4Addr::new(126, 0, 0, 11)),
                            &PeersWanted::AsManyAsPossible,
                        )
                        .await
                        .unwrap();

                    // Scrape
                    let scrape_data = scrape_handler.scrape(&vec![info_hash]).await.unwrap();

                    // The expected swarm metadata for the file
                    let mut expected_scrape_data = ScrapeData::empty();
                    expected_scrape_data.add_file(
                        &info_hash,
                        SwarmMetadata {
                            complete: 0, // the "complete" peer does not count because it was not previously known
                            downloaded: 0,
                            incomplete: 1, // the "incomplete" peer we have just announced
                        },
                    );

                    assert_eq!(scrape_data, expected_scrape_data);
                }
            }
        }

        mod configured_as_whitelisted {

            mod handling_a_scrape_request {

                use bittorrent_primitives::info_hash::InfoHash;
                use torrust_tracker_primitives::core::ScrapeData;
                use torrust_tracker_primitives::swarm_metadata::SwarmMetadata;

                use crate::tests::the_tracker::initialize_handlers_for_listed_tracker;

                #[tokio::test]
                async fn it_should_return_the_zeroed_swarm_metadata_for_the_requested_file_if_it_is_not_whitelisted() {
                    let (_announce_handler, scrape_handler) = initialize_handlers_for_listed_tracker();

                    let non_whitelisted_info_hash = "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap(); // DevSkim: ignore DS173237

                    let scrape_data = scrape_handler.scrape(&vec![non_whitelisted_info_hash]).await.unwrap();

                    // The expected zeroed swarm metadata for the file
                    let mut expected_scrape_data = ScrapeData::empty();
                    expected_scrape_data.add_file(&non_whitelisted_info_hash, SwarmMetadata::zeroed());

                    assert_eq!(scrape_data, expected_scrape_data);
                }
            }
        }
    }
}
