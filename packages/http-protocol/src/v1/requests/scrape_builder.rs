//! `Scrape` request builder for the HTTP tracker.
//!
//! Types for building scrape request URLs to send to an HTTP tracker.
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use torrust_info_hash::InfoHash;

use crate::percent_encoding::percent_encode_byte_array;

/// The scrape request query string builder.
pub struct Query {
    pub info_hash: Vec<InfoHash>,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Query {
    /// It builds the URL query component for the scrape request.
    #[must_use]
    pub fn build(&self) -> String {
        self.params().to_string()
    }

    #[must_use]
    pub fn params(&self) -> QueryParams {
        QueryParams::from(self)
    }
}

/// Builder for constructing a scrape `Query`.
pub struct QueryBuilder {
    scrape_query: Query,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        let default_scrape_query = Query {
            info_hash: vec![InfoHash::from_str("9c38422213e30bff212b30c360d26f9a02136422").unwrap()], // DevSkim: ignore DS173237
        };
        Self {
            scrape_query: default_scrape_query,
        }
    }
}

impl QueryBuilder {
    #[must_use]
    pub fn with_one_info_hash(mut self, info_hash: &InfoHash) -> Self {
        self.scrape_query.info_hash = vec![*info_hash];
        self
    }

    #[must_use]
    pub fn add_info_hash(mut self, info_hash: &InfoHash) -> Self {
        self.scrape_query.info_hash.push(*info_hash);
        self
    }

    #[must_use]
    pub fn query(self) -> Query {
        self.scrape_query
    }
}

/// Query parameters for a HTTP Scrape request.
pub struct QueryParams {
    pub info_hash: Vec<String>,
}

impl QueryParams {
    pub fn set_one_info_hash_param(&mut self, info_hash: &str) {
        self.info_hash = vec![info_hash.to_string()];
    }
}

impl std::fmt::Display for QueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = self
            .info_hash
            .iter()
            .map(|info_hash| format!("info_hash={info_hash}"))
            .collect::<Vec<String>>()
            .join("&");

        write!(f, "{query}")
    }
}

impl QueryParams {
    #[must_use]
    pub fn from(scrape_query: &Query) -> Self {
        let info_hashes = scrape_query
            .info_hash
            .iter()
            .map(|info_hash| percent_encode_byte_array(&info_hash.bytes()))
            .collect::<Vec<String>>();

        Self { info_hash: info_hashes }
    }
}

#[derive(Debug)]
pub struct ConversionError(String);

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid infohash: {}", self.0)
    }
}

impl Error for ConversionError {}

impl TryFrom<&[String]> for Query {
    type Error = ConversionError;

    fn try_from(info_hashes: &[String]) -> Result<Self, Self::Error> {
        let mut validated_info_hashes: Vec<InfoHash> = Vec::new();

        for info_hash in info_hashes {
            let validated_info_hash = InfoHash::from_str(info_hash).map_err(|_| ConversionError(info_hash.clone()))?;
            validated_info_hashes.push(validated_info_hash);
        }

        Ok(Self {
            info_hash: validated_info_hashes,
        })
    }
}

impl TryFrom<Vec<String>> for Query {
    type Error = ConversionError;

    fn try_from(info_hashes: Vec<String>) -> Result<Self, Self::Error> {
        let mut validated_info_hashes: Vec<InfoHash> = Vec::new();

        for info_hash in info_hashes {
            let validated_info_hash = InfoHash::from_str(&info_hash).map_err(|_| ConversionError(info_hash.clone()))?;
            validated_info_hashes.push(validated_info_hash);
        }

        Ok(Self {
            info_hash: validated_info_hashes,
        })
    }
}
