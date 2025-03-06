pub mod container;
pub mod services;
pub mod statistics;

#[cfg(test)]
pub(crate) mod tests {
    use bittorrent_primitives::info_hash::InfoHash;

    /// # Panics
    ///
    /// Will panic if the string representation of the info hash is not a valid info hash.
    #[must_use]
    pub fn sample_info_hash() -> InfoHash {
        "3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0" // DevSkim: ignore DS173237
            .parse::<InfoHash>()
            .expect("String should be a valid info hash")
    }
}
