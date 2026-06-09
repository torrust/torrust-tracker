//! Whitelist authorization.
use std::panic::Location;
use std::sync::Arc;

use torrust_info_hash::InfoHash;
use torrust_tracker_configuration::Core;
use tracing::instrument;

use super::repository::in_memory::InMemoryWhitelist;
use crate::error::WhitelistError;

/// Manages the authorization of torrents based on the whitelist.
///
/// Used to determine whether a given torrent (`infohash`) is allowed
/// to be announced or scraped from the tracker.
pub struct WhitelistAuthorization {
    /// Core tracker configuration.
    config: Core,

    /// The in-memory list of allowed torrents.
    in_memory_whitelist: Arc<InMemoryWhitelist>,
}

impl WhitelistAuthorization {
    /// Creates a new `WhitelistAuthorization` instance.
    ///
    /// # Arguments
    /// - `config`: Tracker configuration.
    /// - `in_memory_whitelist`: The in-memory whitelist instance.
    ///
    /// # Returns
    /// A new `WhitelistAuthorization` instance.
    pub fn new(config: &Core, in_memory_whitelist: &Arc<InMemoryWhitelist>) -> Self {
        Self {
            config: config.clone(),
            in_memory_whitelist: in_memory_whitelist.clone(),
        }
    }

    /// Checks whether a torrent is authorized.
    ///
    /// - If the tracker is **public**, all torrents are authorized.
    /// - If the tracker is **private** (listed mode), only whitelisted torrents
    ///   are authorized.
    ///
    /// # Errors
    /// Returns `WhitelistError::TorrentNotWhitelisted` if the tracker is in `listed` mode
    /// and the `info_hash` is not in the whitelist.
    #[instrument(skip(self, info_hash), err)]
    pub async fn authorize(&self, info_hash: &InfoHash) -> Result<(), WhitelistError> {
        if !self.is_listed() {
            return Ok(());
        }

        if self.is_info_hash_whitelisted(info_hash).await {
            return Ok(());
        }

        Err(WhitelistError::TorrentNotWhitelisted {
            info_hash: *info_hash,
            location: Location::caller(),
        })
    }

    /// Checks if the tracker is running in "listed" mode.
    fn is_listed(&self) -> bool {
        self.config.listed
    }

    /// Checks if a torrent is present in the whitelist.
    async fn is_info_hash_whitelisted(&self, info_hash: &InfoHash) -> bool {
        self.in_memory_whitelist.contains(info_hash).await
    }
}

#[cfg(test)]
mod tests {

    mod the_whitelist_authorization_for_announce_and_scrape_actions {
        use std::sync::Arc;

        use torrust_tracker_configuration::Core;

        use crate::whitelist::authorization::WhitelistAuthorization;
        use crate::whitelist::repository::in_memory::InMemoryWhitelist;

        fn initialize_whitelist_authorization_with(config: &Core) -> Arc<WhitelistAuthorization> {
            let (whitelist_authorization, _in_memory_whitelist) =
                initialize_whitelist_authorization_and_dependencies_with(config);
            whitelist_authorization
        }

        fn initialize_whitelist_authorization_and_dependencies_with(
            config: &Core,
        ) -> (Arc<WhitelistAuthorization>, Arc<InMemoryWhitelist>) {
            let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
            let whitelist_authorization = Arc::new(WhitelistAuthorization::new(config, &in_memory_whitelist.clone()));

            (whitelist_authorization, in_memory_whitelist)
        }

        mod when_the_tacker_is_configured_as_listed {

            use torrust_tracker_configuration::Core;

            use crate::error::WhitelistError;
            use crate::test_helpers::tests::sample_info_hash;
            use crate::whitelist::authorization::tests::the_whitelist_authorization_for_announce_and_scrape_actions::{
                initialize_whitelist_authorization_and_dependencies_with, initialize_whitelist_authorization_with,
            };

            fn configuration_for_listed_tracker() -> Core {
                Core {
                    listed: true,
                    ..Default::default()
                }
            }

            #[tokio::test]
            async fn should_authorize_a_whitelisted_infohash() {
                let (whitelist_authorization, in_memory_whitelist) =
                    initialize_whitelist_authorization_and_dependencies_with(&configuration_for_listed_tracker());

                let info_hash = sample_info_hash();

                let _unused = in_memory_whitelist.add(&info_hash).await;

                let result = whitelist_authorization.authorize(&info_hash).await;

                assert!(result.is_ok());
            }

            #[tokio::test]
            async fn should_not_authorize_a_non_whitelisted_infohash() {
                let whitelist_authorization = initialize_whitelist_authorization_with(&configuration_for_listed_tracker());

                let result = whitelist_authorization.authorize(&sample_info_hash()).await;

                assert!(matches!(result.unwrap_err(), WhitelistError::TorrentNotWhitelisted { .. }));
            }
        }

        mod when_the_tacker_is_not_configured_as_listed {

            use torrust_tracker_configuration::Core;

            use crate::test_helpers::tests::sample_info_hash;
            use crate::whitelist::authorization::tests::the_whitelist_authorization_for_announce_and_scrape_actions::{
                initialize_whitelist_authorization_and_dependencies_with, initialize_whitelist_authorization_with,
            };

            fn configuration_for_non_listed_tracker() -> Core {
                Core {
                    listed: false,
                    ..Default::default()
                }
            }

            #[tokio::test]
            async fn should_authorize_a_whitelisted_infohash() {
                let (whitelist_authorization, in_memory_whitelist) =
                    initialize_whitelist_authorization_and_dependencies_with(&configuration_for_non_listed_tracker());

                let info_hash = sample_info_hash();

                let _unused = in_memory_whitelist.add(&info_hash).await;

                let result = whitelist_authorization.authorize(&info_hash).await;

                assert!(result.is_ok());
            }

            #[tokio::test]
            async fn should_also_authorize_a_non_whitelisted_infohash() {
                let whitelist_authorization = initialize_whitelist_authorization_with(&configuration_for_non_listed_tracker());

                let result = whitelist_authorization.authorize(&sample_info_hash()).await;

                assert!(result.is_ok());
            }
        }
    }
}
