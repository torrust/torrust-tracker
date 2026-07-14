//! Data types for the `Scrape` response.
//!
//! These protocol DTOs intentionally mirror some domain fields but must remain
//! protocol-owned. Keeping this type local avoids protocol->domain coupling and
//! confines translation to boundary adapters.
use std::collections::BTreeMap;

use torrust_info_hash::InfoHash;

// Intentional boundary duplication: this represents scrape response payload
// semantics for the HTTP protocol crate, not tracker-domain semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SwarmMetadata {
    pub complete: u32,
    pub downloaded: u32,
    pub incomplete: u32,
}

// Intentional boundary duplication: this represents scrape response payload
// semantics for the HTTP protocol crate, not tracker-domain semantics.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ScrapeData {
    pub files: BTreeMap<InfoHash, SwarmMetadata>,
}

impl ScrapeData {
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, info_hash: &InfoHash, swarm_metadata: SwarmMetadata) {
        self.files.insert(*info_hash, swarm_metadata);
    }
}
