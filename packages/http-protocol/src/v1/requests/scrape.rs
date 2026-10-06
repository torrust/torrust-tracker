//! `Scrape` request for the HTTP tracker.
//!
//! Data structures and logic for parsing the `scrape` request.
use std::panic::Location;

use thiserror::Error;
use torrust_info_hash::InfoHash;
use torrust_located_error::{Located, LocatedError};

use crate::percent_encoding::percent_decode_info_hash;
use crate::v1::query::Query;
use crate::v1::responses;

// Query param names
const INFO_HASH: &str = "info_hash";

/// The maximum number of `info_hash` params kept from an HTTP scrape request.
///
/// An abuse-mitigation policy value that bounds the work of one request. Unlike
/// UDP, HTTP has no transport reason for a limit. Params past the limit,
/// duplicates included, are ignored without being decoded. A per-request cap
/// does not replace rate limiting: parallel requests bypass it.
///
/// See `docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md`.
pub const MAX_SCRAPE_INFO_HASHES: usize = 100;

#[derive(Debug, PartialEq, Eq)]
pub struct Scrape {
    pub info_hashes: Vec<InfoHash>,
}

/// Errors that can occur while parsing a scrape request.
///
/// Some variants retain raw query values, so this type must not be reused as an
/// event payload. See the [general error-events
/// EPIC](../../../../../docs/issues/drafts/generalize-error-events/EPIC.md) before
/// exposing parser failures through an event stream.
#[derive(Error, Debug)]
pub enum ParseScrapeQueryError {
    #[error("missing query params for scrape request in {location}")]
    MissingParams { location: &'static Location<'static> },
    #[error("missing param {param_name} in {location}")]
    MissingParam {
        location: &'static Location<'static>,
        param_name: String,
    },
    #[error("invalid param value {param_value} for {param_name} in {source}")]
    InvalidInfoHashParam {
        param_name: String,
        param_value: String,
        source: LocatedError<'static, torrust_info_hash::ConversionError>,
    },
}

impl From<ParseScrapeQueryError> for responses::error::Error {
    fn from(err: ParseScrapeQueryError) -> Self {
        Self {
            failure_reason: format!("Bad request. Cannot parse query params for scrape request: {err}"),
        }
    }
}

impl TryFrom<Query> for Scrape {
    type Error = ParseScrapeQueryError;

    fn try_from(query: Query) -> Result<Self, Self::Error> {
        Ok(Self {
            info_hashes: extract_info_hashes(&query)?,
        })
    }
}

fn extract_info_hashes(query: &Query) -> Result<Vec<InfoHash>, ParseScrapeQueryError> {
    match query.get_param_vec(INFO_HASH) {
        Some(raw_params) => {
            let mut info_hashes = vec![];

            for raw_param in raw_params.into_iter().take(MAX_SCRAPE_INFO_HASHES) {
                let info_hash =
                    percent_decode_info_hash(&raw_param).map_err(|err| ParseScrapeQueryError::InvalidInfoHashParam {
                        param_name: INFO_HASH.to_owned(),
                        param_value: raw_param.clone(),
                        source: Located(err).into(),
                    })?;

                info_hashes.push(info_hash);
            }

            Ok(info_hashes)
        }
        None => Err(ParseScrapeQueryError::MissingParam {
            location: Location::caller(),
            param_name: INFO_HASH.to_owned(),
        }),
    }
}

#[cfg(test)]
mod tests {

    mod scrape_request {

        use torrust_info_hash::InfoHash;

        use crate::v1::query::Query;
        use crate::v1::requests::scrape::{INFO_HASH, Scrape};

        #[test]
        fn should_be_instantiated_from_the_url_query_with_only_one_infohash() {
            let raw_query = Query::from(vec![(INFO_HASH, "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0")]).to_string();

            let query = raw_query.parse::<Query>().unwrap();

            let scrape_request = Scrape::try_from(query).unwrap();

            assert_eq!(
                scrape_request,
                Scrape {
                    info_hashes: vec!["3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0".parse::<InfoHash>().unwrap()],
                }
            );
        }

        mod when_it_is_instantiated_from_the_url_query_params {

            use crate::v1::query::Query;
            use crate::v1::requests::scrape::{INFO_HASH, Scrape};

            #[test]
            fn it_should_fail_if_the_query_does_not_include_the_info_hash_param() {
                let raw_query_without_info_hash = "another_param=NOT_RELEVANT";

                assert!(Scrape::try_from(raw_query_without_info_hash.parse::<Query>().unwrap()).is_err());
            }

            #[test]
            fn it_should_fail_if_the_info_hash_param_is_invalid() {
                let raw_query = Query::from(vec![(INFO_HASH, "INVALID_INFO_HASH_VALUE")]).to_string();

                assert!(Scrape::try_from(raw_query.parse::<Query>().unwrap()).is_err());
            }
        }

        /// HTTP keeps the first 100 `info_hash` params, an abuse-mitigation
        /// policy value with no transport reason (unlike UDP's 74). Counts are
        /// literals so that changing the limit fails here; see
        /// `docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md`.
        mod limiting_the_number_of_info_hashes {

            use torrust_info_hash::InfoHash;

            use crate::v1::query::Query;
            use crate::v1::requests::scrape::{ParseScrapeQueryError, Scrape};
            use crate::v1::requests::scrape_builder;

            fn distinct_info_hashes(count: u16) -> Vec<InfoHash> {
                (0..count)
                    .map(|index| {
                        let mut bytes = [0u8; 20];
                        bytes[..2].copy_from_slice(&index.to_be_bytes());
                        InfoHash(bytes)
                    })
                    .collect()
            }

            fn raw_query_for(info_hashes: &[InfoHash]) -> String {
                scrape_builder::Query {
                    info_hash: info_hashes.to_vec(),
                }
                .to_string()
            }

            fn parse_scrape(raw_query: &str) -> Result<Scrape, ParseScrapeQueryError> {
                Scrape::try_from(raw_query.parse::<Query>().unwrap())
            }

            #[test]
            fn it_should_keep_all_100_info_hashes_when_the_request_has_exactly_100() {
                // Arrange
                let info_hashes = distinct_info_hashes(100);

                // Act
                let scrape_request = parse_scrape(&raw_query_for(&info_hashes)).unwrap();

                // Assert
                assert_eq!(scrape_request.info_hashes, info_hashes);
            }

            #[test]
            fn it_should_keep_only_the_first_100_info_hashes_in_request_order_when_the_request_has_101() {
                // Arrange
                let info_hashes = distinct_info_hashes(101);

                // Act
                let scrape_request = parse_scrape(&raw_query_for(&info_hashes)).unwrap();

                // Assert
                assert_eq!(scrape_request.info_hashes, info_hashes[..100]);
            }

            #[test]
            fn it_should_ignore_an_invalid_info_hash_after_the_first_100_without_validating_it() {
                // Arrange
                let info_hashes = distinct_info_hashes(100);
                let raw_query = format!("{}&info_hash=INVALID_INFO_HASH_VALUE", raw_query_for(&info_hashes));

                // Act
                let scrape_request = parse_scrape(&raw_query).unwrap();

                // Assert
                assert_eq!(scrape_request.info_hashes, info_hashes);
            }

            #[test]
            fn it_should_still_fail_for_an_invalid_info_hash_within_the_first_100() {
                // Arrange
                let raw_query = format!(
                    "info_hash=INVALID_INFO_HASH_VALUE&{}",
                    raw_query_for(&distinct_info_hashes(100))
                );

                // Act
                let result = parse_scrape(&raw_query);

                // Assert
                assert!(
                    matches!(result, Err(ParseScrapeQueryError::InvalidInfoHashParam { .. })),
                    "expected an invalid info_hash error, got {result:?}"
                );
            }

            #[test]
            fn it_should_count_repeated_info_hashes_toward_the_limit_of_100() {
                // Arrange
                let distinct = distinct_info_hashes(100);
                let info_hashes_with_first_repeated: Vec<InfoHash> =
                    std::iter::once(distinct[0]).chain(distinct.iter().copied()).collect();

                // Act
                let scrape_request = parse_scrape(&raw_query_for(&info_hashes_with_first_repeated)).unwrap();

                // Assert
                assert_eq!(scrape_request.info_hashes, info_hashes_with_first_repeated[..100]);
            }
        }
    }
}
