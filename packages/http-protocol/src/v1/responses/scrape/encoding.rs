//! Encoding layer for the `Scrape` response.
//!
//! Contains the `Bencoded` struct and its conversion from `ScrapeData`.
use std::borrow::Cow;

use torrust_bencode::{BMutAccess, ben_int, ben_map};

use crate::v1::responses::scrape::data::ScrapeData;

/// The `Scrape` response for the HTTP tracker.
///
/// ```rust
/// use torrust_tracker_http_protocol::v1::responses::scrape::Bencoded;
/// use torrust_info_hash::InfoHash;
/// use torrust_tracker_http_protocol::v1::responses::scrape::{ScrapeData, SwarmMetadata};
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
        use torrust_info_hash::InfoHash;

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
