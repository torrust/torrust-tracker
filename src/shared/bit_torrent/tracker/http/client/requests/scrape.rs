use std::error::Error;
use std::fmt::{self};
use std::str::FromStr;

use torrust_tracker_primitives::info_hash::InfoHash;

use super::Scrape;
use crate::shared::bit_torrent::tracker::http::{percent_encode_byte_array, ByteArray20};

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Query {
    pub infohashes: Vec<ByteArray20>,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ConversionError(String);

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid infohash: {}", self.0)
    }
}

impl Error for ConversionError {}

impl FromIterator<InfoHash> for Query {
    fn from_iter<T: IntoIterator<Item = InfoHash>>(iter: T) -> Self {
        let mut infohashes = Vec::default();

        for infohash in iter {
            infohashes.push(infohash.0);
        }

        Query { infohashes }
    }
}

impl TryFrom<&[String]> for Query {
    type Error = ConversionError;

    fn try_from(values: &[String]) -> Result<Self, Self::Error> {
        let mut infohashes = Vec::default();

        for i in values {
            infohashes.push(InfoHash::from_str(i).map_err(|_| ConversionError(i.clone()))?);
        }

        Ok(Query::from_iter(infohashes))
    }
}

impl TryFrom<Vec<String>> for Query {
    type Error = ConversionError;

    fn try_from(info_hashes: Vec<String>) -> Result<Self, Self::Error> {
        let mut validated_info_hashes: Vec<ByteArray20> = Vec::new();

        for info_hash in info_hashes {
            let validated_info_hash = InfoHash::from_str(&info_hash).map_err(|_| ConversionError(info_hash.clone()))?;
            validated_info_hashes.push(validated_info_hash.0);
        }

        Ok(Self {
            infohashes: validated_info_hashes,
        })
    }
}

/// HTTP Tracker Scrape Request:
///
/// <https://www.bittorrent.org/beps/bep_0048.html>
impl Query {
    #[must_use]
    pub fn build(&self) -> String {
        self.params().to_string()
    }

    #[must_use]
    pub fn params(&self) -> QueryParams {
        QueryParams::from(self)
    }
}

#[derive(Default)]
pub struct QueryBuilder {
    query: Query,
}

impl FromIterator<InfoHash> for QueryBuilder {
    fn from_iter<T: IntoIterator<Item = InfoHash>>(iter: T) -> Self {
        Self {
            query: Query::from_iter(iter),
        }
    }
}

impl From<&InfoHash> for QueryBuilder {
    fn from(value: &InfoHash) -> Self {
        Self {
            query: Query {
                infohashes: [value.0].to_vec(),
            },
        }
    }
}

impl QueryBuilder {
    #[must_use]
    pub fn add_info_hash(mut self, info_hash: &InfoHash) -> Self {
        self.query.infohashes.push(info_hash.0);
        self
    }

    #[must_use]
    pub fn build(self) -> Scrape {
        self.query.into()
    }
}

/// It contains all the GET parameters that can be used in a HTTP Scrape request.
///
/// The `info_hash` param is the percent encoded of the the 20-byte array info hash.
///
/// Sample Scrape URL with all the GET parameters:
///
/// For `IpV4`:
///
/// ```text
/// http://127.0.0.1:7070/scrape?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22
/// ```
///
/// For `IpV6`:
///
/// ```text
/// http://[::1]:7070/scrape?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22
/// ```
///
/// You can add as many info hashes as you want, just adding the same param again.
pub struct QueryParams {
    pub info_hash: Vec<String>,
}

impl QueryParams {
    pub fn set_one_info_hash_param(&mut self, info_hash: &str) {
        self.info_hash = vec![info_hash.to_string()];
    }
}

/// It builds the URL query component for the scrape request.
///
/// This custom URL query params encoding is needed because `reqwest` does not allow
/// bytes arrays in query parameters. More info on this issue:
///
/// <https://github.com/seanmonstar/reqwest/issues/1613>
impl std::fmt::Display for QueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = self
            .info_hash
            .iter()
            .map(|info_hash| format!("info_hash={}", &info_hash))
            .collect::<Vec<String>>()
            .join("&");

        write!(f, "{query}")
    }
}

impl From<Scrape> for QueryParams {
    fn from(value: Scrape) -> Self {
        let query: &Query = &Scrape::into(value);
        query.into()
    }
}

impl From<&Query> for QueryParams {
    fn from(value: &Query) -> Self {
        let info_hashes = value
            .infohashes
            .iter()
            .map(percent_encode_byte_array)
            .collect::<Vec<String>>();

        Self { info_hash: info_hashes }
    }
}
