//! `Scrape` response for the HTTP tracker [`scrape`](crate::v1::requests::scrape::Scrape) request.
//!
//! Data structures and logic to build the `scrape` response.
use std::borrow::Cow;
use std::collections::BTreeMap;

use bittorrent_primitives::info_hash::InfoHash;
use torrust_tracker_contrib_bencode::{BMutAccess, ben_int, ben_map};

// These protocol DTOs intentionally mirror some domain fields but must remain
// protocol-owned. Keeping this type local avoids protocol->domain coupling and
// confines translation to boundary adapters.
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

/// The `Scrape` response for the HTTP tracker.
///
/// ```rust
/// use torrust_tracker_http_tracker_protocol::v1::responses::scrape::Bencoded;
/// use bittorrent_primitives::info_hash::InfoHash;
/// use torrust_tracker_http_tracker_protocol::v1::responses::scrape::{ScrapeData, SwarmMetadata};
///
/// let info_hash = InfoHash::from_bytes(&[0x69; 20]);
/// let mut scrape_data = ScrapeData::empty();
/// scrape_data.add_file(
///     &info_hash,
///     SwarmMetadata {
///         complete: 1,
///         downloaded: 2,
///         incomplete: 3,
///     },
/// );
///
/// let response = Bencoded::from(scrape_data);
///
/// let bytes = response.body();
///
/// // cspell:disable-next-line
/// let expected_bytes = b"d5:filesd20:iiiiiiiiiiiiiiiiiiiid8:completei1e10:downloadedi2e10:incompletei3eeee";
///
/// assert_eq!(
///     String::from_utf8(bytes).unwrap(),
///     String::from_utf8(expected_bytes.to_vec()).unwrap()
/// );
/// ```
#[derive(Debug, PartialEq, Default)]
pub struct Bencoded {
    /// The scrape data to be bencoded.
    scrape_data: ScrapeData,
}

impl Bencoded {
    /// Returns the bencoded representation of the `Scrape` struct.
    ///
    /// # Panics
    ///
    /// Will return an error if it can't access the bencode as a mutable `BDictAccess`.    
    #[must_use]
    pub fn body(&self) -> Vec<u8> {
        let mut scrape_list = ben_map!();

        let scrape_list_mut = scrape_list.dict_mut().unwrap();

        for (info_hash, value) in &self.scrape_data.files {
            scrape_list_mut.insert(
                Cow::from(info_hash.bytes().to_vec()),
                ben_map! {
                    "complete" => ben_int!(i64::from(value.complete)),
                    "downloaded" => ben_int!(i64::from(value.downloaded)),
                    "incomplete" => ben_int!(i64::from(value.incomplete))
                },
            );
        }

        (ben_map! {
            "files" => scrape_list
        })
        .encode()
    }
}

impl From<ScrapeData> for Bencoded {
    fn from(scrape_data: ScrapeData) -> Self {
        Self { scrape_data }
    }
}

#[cfg(test)]
mod tests {

    mod scrape_response {
        use bittorrent_primitives::info_hash::InfoHash;

        use crate::v1::responses::scrape::{Bencoded, ScrapeData, SwarmMetadata};

        fn sample_scrape_data() -> ScrapeData {
            let info_hash = InfoHash::from_bytes(&[0x69; 20]);
            let mut scrape_data = ScrapeData::empty();
            scrape_data.add_file(
                &info_hash,
                SwarmMetadata {
                    complete: 1,
                    downloaded: 2,
                    incomplete: 3,
                },
            );
            scrape_data
        }

        #[test]
        fn should_be_converted_from_scrape_data() {
            let response = Bencoded::from(sample_scrape_data());

            assert_eq!(
                response,
                Bencoded {
                    scrape_data: sample_scrape_data()
                }
            );
        }

        #[test]
        fn should_be_bencoded() {
            let response = Bencoded {
                scrape_data: sample_scrape_data(),
            };

            let bytes = response.body();

            // cspell:disable-next-line
            let expected_bytes = b"d5:filesd20:iiiiiiiiiiiiiiiiiiiid8:completei1e10:downloadedi2e10:incompletei3eeee";

            assert_eq!(
                String::from_utf8(bytes).unwrap(),
                String::from_utf8(expected_bytes.to_vec()).unwrap()
            );
        }

        #[test]
        fn should_encode_large_download_counts_as_i64() {
            let info_hash = InfoHash::from_bytes(&[0x69; 20]);
            let mut scrape_data = ScrapeData::empty();
            scrape_data.add_file(
                &info_hash,
                SwarmMetadata {
                    complete: 1,
                    downloaded: u32::MAX,
                    incomplete: 3,
                },
            );

            let response = Bencoded::from(scrape_data);
            let bytes = response.body();
            let body = String::from_utf8(bytes).unwrap();

            assert!(body.contains(&format!("downloadedi{}e", i64::from(u32::MAX))));
        }
    }
}
