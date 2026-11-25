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

    mod the_return_values {
        use rstest::rstest;

        use crate::test_helpers::tests::sample_info_hash;
        use crate::whitelist::repository::in_memory::InMemoryWhitelist;

        #[rstest]
        #[case::a_new_hash(false, true)]
        #[case::a_hash_already_there(true, false)]
        #[tokio::test]
        async fn adding_should_report_whether_the_hash_was_new(#[case] already_there: bool, #[case] expected_new: bool) {
            // Arrange
            let info_hash = sample_info_hash();
            let whitelist = InMemoryWhitelist::default();
            if already_there {
                whitelist.add(&info_hash).await;
            }

            // Act
            let was_new = whitelist.add(&info_hash).await;

            // Assert
            assert_eq!(was_new, expected_new, "adding a hash that was there already: {already_there}");
            assert!(
                whitelist.contains(&info_hash).await,
                "the hash should be whitelisted after adding it"
            );
        }

        #[rstest]
        #[case::a_present_hash(true, true)]
        #[case::an_absent_hash(false, false)]
        #[tokio::test]
        async fn removing_should_report_whether_the_hash_was_present(
            #[case] already_there: bool,
            #[case] expected_present: bool,
        ) {
            // Arrange
            let info_hash = sample_info_hash();
            let whitelist = InMemoryWhitelist::default();
            if already_there {
                whitelist.add(&info_hash).await;
            }

            // Act
            let was_present = whitelist.remove(&info_hash).await;

            // Assert
            assert_eq!(
                was_present, expected_present,
                "removing a hash that was there already: {already_there}"
            );
            assert!(
                !whitelist.contains(&info_hash).await,
                "the hash should not be whitelisted after removing it"
            );
        }
    }
}
