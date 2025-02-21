use std::sync::Arc;

use torrust_tracker_configuration::Core;

use crate::announce_handler::AnnounceHandler;
use crate::authentication::handler::KeysHandler;
use crate::authentication::key::repository::in_memory::InMemoryKeyRepository;
use crate::authentication::key::repository::persisted::DatabaseKeyRepository;
use crate::authentication::service::AuthenticationService;
use crate::databases::setup::initialize_database;
use crate::databases::Database;
use crate::scrape_handler::ScrapeHandler;
use crate::torrent::manager::TorrentsManager;
use crate::torrent::repository::in_memory::InMemoryTorrentRepository;
use crate::torrent::repository::persisted::DatabasePersistentTorrentRepository;
use crate::whitelist;
use crate::whitelist::authorization::WhitelistAuthorization;
use crate::whitelist::manager::WhitelistManager;
use crate::whitelist::repository::in_memory::InMemoryWhitelist;
use crate::whitelist::setup::initialize_whitelist_manager;

pub struct TrackerCoreContainer {
    pub core_config: Arc<Core>,
    pub database: Arc<Box<dyn Database>>,
    pub announce_handler: Arc<AnnounceHandler>,
    pub scrape_handler: Arc<ScrapeHandler>,
    pub keys_handler: Arc<KeysHandler>,
    pub authentication_service: Arc<AuthenticationService>,
    pub in_memory_whitelist: Arc<InMemoryWhitelist>,
    pub whitelist_authorization: Arc<whitelist::authorization::WhitelistAuthorization>,
    pub whitelist_manager: Arc<WhitelistManager>,
    pub in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    pub db_torrent_repository: Arc<DatabasePersistentTorrentRepository>,
    pub torrents_manager: Arc<TorrentsManager>,
}

impl TrackerCoreContainer {
    #[must_use]
    pub fn initialize(core_config: &Arc<Core>) -> Self {
        let database = initialize_database(core_config);
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(core_config, &in_memory_whitelist.clone()));
        let whitelist_manager = initialize_whitelist_manager(database.clone(), in_memory_whitelist.clone());
        let db_key_repository = Arc::new(DatabaseKeyRepository::new(&database));
        let in_memory_key_repository = Arc::new(InMemoryKeyRepository::default());
        let authentication_service = Arc::new(AuthenticationService::new(core_config, &in_memory_key_repository));
        let keys_handler = Arc::new(KeysHandler::new(
            &db_key_repository.clone(),
            &in_memory_key_repository.clone(),
        ));
        let in_memory_torrent_repository = Arc::new(InMemoryTorrentRepository::default());
        let db_torrent_repository = Arc::new(DatabasePersistentTorrentRepository::new(&database));

        let torrents_manager = Arc::new(TorrentsManager::new(
            core_config,
            &in_memory_torrent_repository,
            &db_torrent_repository,
        ));

        let announce_handler = Arc::new(AnnounceHandler::new(
            core_config,
            &whitelist_authorization,
            &in_memory_torrent_repository,
            &db_torrent_repository,
        ));

        let scrape_handler = Arc::new(ScrapeHandler::new(&whitelist_authorization, &in_memory_torrent_repository));

        Self {
            core_config: core_config.clone(),
            database,
            announce_handler,
            scrape_handler,
            keys_handler,
            authentication_service,
            in_memory_whitelist,
            whitelist_authorization,
            whitelist_manager,
            in_memory_torrent_repository,
            db_torrent_repository,
            torrents_manager,
        }
    }
}
