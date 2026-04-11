//! Generic test helper functions for the whitelist module.
//!
//! This module provides utility functions to initialize the whitelist services required for testing.
//! In particular, it sets up the `WhitelistAuthorization` and `WhitelistManager` services using a
//! configured database and an in-memory whitelist repository.
#[cfg(test)]
pub(crate) mod tests {

    use std::sync::Arc;

    use torrust_tracker_configuration::Configuration;

    use crate::databases::setup::initialize_database;
    use crate::whitelist::authorization::WhitelistAuthorization;
    use crate::whitelist::manager::WhitelistManager;
    use crate::whitelist::repository::in_memory::InMemoryWhitelist;
    use crate::whitelist::setup::initialize_whitelist_manager;

    #[must_use]
    pub fn initialize_whitelist_services(config: &Configuration) -> (Arc<WhitelistAuthorization>, Arc<WhitelistManager>) {
        let database = initialize_database(&config.core);
        let in_memory_whitelist = Arc::new(InMemoryWhitelist::default());
        let whitelist_authorization = Arc::new(WhitelistAuthorization::new(&config.core, &in_memory_whitelist.clone()));
        let whitelist_manager = initialize_whitelist_manager(database.whitelist_store(), in_memory_whitelist.clone());

        (whitelist_authorization, whitelist_manager)
    }

    #[must_use]
    pub fn initialize_whitelist_services_for_listed_tracker() -> (Arc<WhitelistAuthorization>, Arc<WhitelistManager>) {
        use torrust_tracker_test_helpers::configuration;

        initialize_whitelist_services(&configuration::ephemeral_listed())
    }
}
