//! This module contains the logic to manage the torrent whitelist.
//!
//! In tracker configurations where the tracker operates in "listed" mode, only
//! torrents that have been explicitly added to the whitelist are allowed to
//! perform announce and scrape actions. This module provides all the
//! functionality required to manage such a whitelist.
//!
//! The module is organized into the following submodules:
//!
//! - **`authorization`**: Contains the logic to authorize torrents based on their
//!   whitelist status.
//! - **`manager`**: Provides high-level management functions for the whitelist,
//!   such as adding or removing torrents.
//! - **`repository`**: Implements persistence for whitelist data.
//! - **`setup`**: Provides initialization routines for setting up the whitelist
//!   system.
//! - **`test_helpers`**: Contains helper functions and fixtures for testing
//!   whitelist functionality.
pub mod authorization;
pub mod manager;
pub mod repository;
pub mod setup;
pub mod test_helpers;

#[cfg(test)]
mod tests {

    mod configured_as_whitelisted {

        mod handling_authorization {
            use crate::test_helpers::tests::sample_info_hash;
            use crate::whitelist::test_helpers::tests::initialize_whitelist_services_for_listed_tracker;

            #[tokio::test]
            async fn it_should_authorize_the_announce_and_scrape_actions_on_whitelisted_torrents() {
                let (whitelist_authorization, whitelist_manager) = initialize_whitelist_services_for_listed_tracker().await;

                let info_hash = sample_info_hash();

                let result = whitelist_manager.add_torrent_to_whitelist(&info_hash).await;
                assert!(result.is_ok());

                let result = whitelist_authorization.authorize(&info_hash).await;
                assert!(result.is_ok());
            }

            #[tokio::test]
            async fn it_should_not_authorize_the_announce_and_scrape_actions_on_not_whitelisted_torrents() {
                let (whitelist_authorization, _whitelist_manager) = initialize_whitelist_services_for_listed_tracker().await;

                let info_hash = sample_info_hash();

                let result = whitelist_authorization.authorize(&info_hash).await;
                assert!(result.is_err());
            }
        }
    }
}
