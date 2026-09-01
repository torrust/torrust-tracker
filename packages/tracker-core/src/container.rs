//! Tracker-core dependency composition.
//!
//! Persistence optionality is resolved at this initialization seam; see ADR
//! [`20260825193119_make_persistence_an_optional_application_composition_capability`](../../../docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md).
use std::sync::Arc;

use torrust_tracker_configuration::v3_0_0::core::Core;
use torrust_tracker_configuration::v3_0_0::database::Database;
use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

use crate::announce_handler::AnnounceHandler;
use crate::authentication::handler::KeysHandler;
use crate::authentication::key::repository::in_memory::InMemoryKeyRepository;
use crate::authentication::key::repository::persisted::DatabaseKeyRepository;
use crate::authentication::service::AuthenticationService;
use crate::databases::setup::{DatabaseStores, initialize_database_from_configuration};
use crate::scrape_handler::ScrapeHandler;
use crate::statistics::persisted::downloads::DatabaseDownloadsMetricRepository;
use crate::torrent::manager::TorrentsManager;
use crate::torrent::repository::in_memory::InMemoryTorrentRepository;
use crate::whitelist::authorization::WhitelistAuthorization;
use crate::whitelist::manager::WhitelistManager;
use crate::whitelist::repository::in_memory::InMemoryWhitelist;
use crate::whitelist::setup::initialize_whitelist_manager;
use crate::{statistics, whitelist};

/// Errors while composing the tracker core.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The configured persistence driver could not be initialized or migrated.
    #[error(
        "Could not initialize configured tracker persistence. Verify the database connection, credentials, and schema permissions: {source}"
    )]
    Persistence { source: crate::databases::error::Error },

    /// Persistent completed statistics were requested without persistence.
    #[error(
        "Persistent completed statistics require configured persistence. Add `[core.database]` or disable `core.tracker_policy.persistent_torrent_completed_stat`."
    )]
    PersistentStatisticsRequirePersistence,
}

pub struct TrackerCoreContainer {
    pub core_config: Arc<Core>,
    pub announce_handler: Arc<AnnounceHandler>,
    pub scrape_handler: Arc<ScrapeHandler>,
    pub authentication_service: Arc<AuthenticationService>,
    pub in_memory_whitelist: Arc<InMemoryWhitelist>,
    pub whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    pub in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    pub torrents_manager: Arc<TorrentsManager>,
    pub stats_repository: Arc<statistics::repository::Repository>,
    pub persistence: Option<PersistenceServices>,
}

pub struct PersistenceServices {
    pub database_stores: DatabaseStores,
    pub keys_handler: Arc<KeysHandler>,
    pub whitelist_manager: Arc<WhitelistManager>,
    pub db_downloads_metric_repository: Arc<DatabaseDownloadsMetricRepository>,
}

impl TrackerCoreContainer {
    /// Constructs tracker-core services and, when configured, their persistence services.
    ///
    /// # Errors
    ///
    /// Returns a typed persistence-composition error when the configured database driver
    /// or migrations fail, or when persistent statistics lack persistence services.
    pub async fn initialize_from(
        core_config: &Arc<Core>,
        swarm_coordination_registry_container: &Arc<SwarmCoordinationRegistryContainer>,
        database: Option<&Database>,
    ) -> Result<Self, Error> {
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(core_config, &in_memory_whitelist.clone()));
        let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
        let authentication_service = Arc::new(AuthenticationService::new(core_config, &in_memory_key_repository));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::new(
            swarm_coordination_registry_container.swarms.clone(),
        ));
        let persistence = if let Some(database) = database {
            let database_stores = initialize_database_from_configuration(database)
                .await
                .map_err(|source| Error::Persistence { source })?;
            let whitelist_manager =
                initialize_whitelist_manager(database_stores.whitelist_store.clone(), in_memory_whitelist.clone());
            let db_key_repository = Arc::new(DatabaseKeyRepository::new(&database_stores.auth_key_store));
            let keys_handler = Arc::new(KeysHandler::new(&db_key_repository, &in_memory_key_repository));
            let db_downloads_metric_repository =
                Arc::new(DatabaseDownloadsMetricRepository::new(&database_stores.torrent_metrics_store));

            Some(PersistenceServices {
                database_stores,
                keys_handler,
                whitelist_manager,
                db_downloads_metric_repository,
            })
        } else {
            None
        };

        let torrents_manager = Arc::new(TorrentsManager::new(core_config, &in_memory_torrent_repository));
        let stats_repository = Arc::new(statistics::repository::Repository::new(
            core_config.tracker_usage_statistics,
            core_config.tracker_policy.persistent_torrent_completed_stat,
        ));
        let announce_handler = if core_config.tracker_policy.persistent_torrent_completed_stat {
            let persistence = persistence.as_ref().ok_or(Error::PersistentStatisticsRequirePersistence)?;
            Arc::new(AnnounceHandler::new_with_persistent_completed_statistics(
                core_config,
                &whitelist_authorization,
                &in_memory_torrent_repository,
                &persistence.db_downloads_metric_repository,
            ))
        } else {
            Arc::new(AnnounceHandler::new_public(
                core_config,
                &whitelist_authorization,
                &in_memory_torrent_repository,
            ))
        };
        let scrape_handler = Arc::new(ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository));

        Ok(Self {
            core_config: core_config.clone(),
            announce_handler,
            scrape_handler,
            authentication_service,
            in_memory_whitelist,
            whitelist_authorization,
            in_memory_torrent_repository,
            torrents_manager,
            stats_repository,
            persistence,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};
    use std::sync::Arc;

    use torrust_tracker_configuration::v3_0_0::core::Core;
    use torrust_tracker_events::bus::SenderStatus;
    use torrust_tracker_swarm_coordination_registry::container::SwarmCoordinationRegistryContainer;

    use super::{Error, TrackerCoreContainer};
    use crate::announce_handler::PeersWanted;
    use crate::test_helpers::tests::{ephemeral_configuration, sample_info_hash, sample_peer};

    #[tokio::test]
    async fn it_should_construct_a_tracker_core_container_without_persistence() {
        // Arrange
        let core_config = Arc::new(Core::default());
        let swarm_coordination_registry_container =
            Arc::new(SwarmCoordinationRegistryContainer::initialize(SenderStatus::Disabled));

        // Act
        let container = TrackerCoreContainer::initialize_from(&core_config, &swarm_coordination_registry_container, None).await;

        // Assert
        assert!(container.expect("composition should succeed").persistence.is_none());
    }

    #[tokio::test]
    async fn it_should_return_a_typed_error_when_persistent_statistics_lack_persistence() {
        // Arrange
        let mut core_config = Core::default();
        core_config.tracker_policy.persistent_torrent_completed_stat = true;
        let core_config = Arc::new(core_config);
        let swarm_coordination_registry_container =
            Arc::new(SwarmCoordinationRegistryContainer::initialize(SenderStatus::Disabled));

        // Act
        let result = TrackerCoreContainer::initialize_from(&core_config, &swarm_coordination_registry_container, None).await;

        // Assert
        assert!(matches!(result, Err(Error::PersistentStatisticsRequirePersistence)));
    }

    #[tokio::test]
    async fn it_should_construct_a_tracker_core_container_with_supplied_persistence() {
        // Arrange
        let core_config = Arc::new(ephemeral_configuration());
        let swarm_coordination_registry_container =
            Arc::new(SwarmCoordinationRegistryContainer::initialize(SenderStatus::Disabled));

        // Act
        let container = TrackerCoreContainer::initialize_from(
            &core_config,
            &swarm_coordination_registry_container,
            core_config.database.as_ref(),
        )
        .await;

        // Assert
        assert!(container.expect("composition should succeed").persistence.is_some());
    }

    #[tokio::test]
    async fn it_should_load_persistent_completed_statistics_when_a_torrent_is_first_announced() {
        // Arrange
        let mut core_config = ephemeral_configuration();
        core_config.tracker_policy.persistent_torrent_completed_stat = true;
        let core_config = Arc::new(core_config);
        let swarm_coordination_registry_container =
            Arc::new(SwarmCoordinationRegistryContainer::initialize(SenderStatus::Disabled));
        let info_hash = sample_info_hash();

        let container = TrackerCoreContainer::initialize_from(
            &core_config,
            &swarm_coordination_registry_container,
            core_config.database.as_ref(),
        )
        .await
        .expect("composition should succeed");
        container
            .persistence
            .as_ref()
            .unwrap()
            .db_downloads_metric_repository
            .save_torrent_downloads(&info_hash, 42)
            .await
            .unwrap();

        // Act
        let announce_data = container
            .announce_handler
            .handle_announcement(
                &info_hash,
                &mut sample_peer(),
                &IpAddr::V4(Ipv4Addr::LOCALHOST),
                None,
                &PeersWanted::AsManyAsPossible,
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(announce_data.stats.downloads(), 42);
    }
}
