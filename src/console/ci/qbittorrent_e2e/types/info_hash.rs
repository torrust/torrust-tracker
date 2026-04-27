use std::fmt;
use std::ops::Deref;

/// A v1 `BitTorrent` `InfoHash` — a 40-character lowercase hex-encoded SHA-1 digest.
///
/// Wraps a [`String`] to give the value a precise type at every call site,
/// eliminating confusion with other hex strings (e.g. peer IDs, piece hashes).
///
/// The format matches what the qBittorrent Web API returns in the `hash` field
/// of `/api/v2/torrents/info`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct InfoHash(String);

impl InfoHash {
    /// Creates a new [`InfoHash`] from any value that converts into a [`String`].
    pub(crate) fn new(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    /// Returns the hash as a `&str`.
    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for InfoHash {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for InfoHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for InfoHash {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::InfoHash;

    #[test]
    fn it_should_construct_info_hash_and_expose_accessors() {
        let hash = InfoHash::new("0123456789abcdef0123456789abcdef01234567"); // DevSkim: ignore DS173237

        assert_eq!(hash.as_str(), "0123456789abcdef0123456789abcdef01234567"); // DevSkim: ignore DS173237
        assert_eq!(&*hash, "0123456789abcdef0123456789abcdef01234567"); // DevSkim: ignore DS173237
        assert_eq!(hash.to_string(), "0123456789abcdef0123456789abcdef01234567");
        // DevSkim: ignore DS173237
    }

    #[test]
    fn it_should_deserialize_info_hash_from_json_string() {
        let parsed = serde_json::from_str::<InfoHash>("\"abcdef0123456789abcdef0123456789abcdef01\""); // DevSkim: ignore DS173237

        assert!(parsed.is_ok());
        let hash = parsed.unwrap_or_else(|error| panic!("failed to parse hash: {error}"));
        assert_eq!(hash.as_str(), "abcdef0123456789abcdef0123456789abcdef01"); // DevSkim: ignore DS173237
    }
}
