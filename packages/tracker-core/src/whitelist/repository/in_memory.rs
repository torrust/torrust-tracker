//! The in-memory list of allowed torrents.
use torrust_info_hash::InfoHash;

/// In-memory whitelist to manage allowed torrents.
///
/// Stores `InfoHash` values for quick lookup and modification.
#[derive(Debug, Default)]
pub struct InMemoryWhitelist {
    /// A thread-safe set of whitelisted `InfoHash` values.
    whitelist: tokio::sync::RwLock<std::collections::HashSet<InfoHash>>,
}

impl InMemoryWhitelist {
    /// Adds a torrent to the in-memory whitelist.
    ///
    /// # Returns
    ///
    /// - `true` if the torrent was newly added.
    /// - `false` if the torrent was already in the whitelist.
    pub async fn add(&self, info_hash: &InfoHash) -> bool {
        self.whitelist.write().await.insert(*info_hash)
    }

    /// Removes a torrent from the in-memory whitelist.
    ///
    /// # Returns
    ///
    /// - `true` if the torrent was present and removed.
    /// - `false` if the torrent was not found.
    pub(crate) async fn remove(&self, info_hash: &InfoHash) -> bool {
        self.whitelist.write().await.remove(info_hash)
    }

    /// Checks if a torrent is in the whitelist.
    pub async fn contains(&self, info_hash: &InfoHash) -> bool {
        self.whitelist.read().await.contains(info_hash)
    }

    /// Clears all torrents from the whitelist.
    pub(crate) async fn clear(&self) {
        let mut whitelist = self.whitelist.write().await;
        whitelist.clear();
    }
}

#[cfg(test)]
mod tests {

    use crate::test_helpers::tests::sample_info_hash;
    use crate::whitelist::repository::in_memory::InMemoryWhitelist;

    #[tokio::test]
    async fn should_allow_adding_a_new_torrent_to_the_whitelist() {
        let info_hash = sample_info_hash();

        let whitelist = InMemoryWhitelist::default();

        whitelist.add(&info_hash).await;

        assert!(whitelist.contains(&info_hash).await);
    }

    #[tokio::test]
    async fn should_allow_removing_a_new_torrent_to_the_whitelist() {
        let info_hash = sample_info_hash();

        let whitelist = InMemoryWhitelist::default();

        whitelist.add(&info_hash).await;
        whitelist.remove(&sample_info_hash()).await;

        assert!(!whitelist.contains(&info_hash).await);
    }

    #[tokio::test]
    async fn should_allow_clearing_the_whitelist() {
        let info_hash = sample_info_hash();

        let whitelist = InMemoryWhitelist::default();

        whitelist.add(&info_hash).await;
        whitelist.clear().await;

        assert!(!whitelist.contains(&info_hash).await);
    }

    #[tokio::test]
    async fn should_allow_checking_if_an_infohash_is_whitelisted() {
        let info_hash = sample_info_hash();

        let whitelist = InMemoryWhitelist::default();

        whitelist.add(&info_hash).await;

        assert!(whitelist.contains(&info_hash).await);
    }
}
