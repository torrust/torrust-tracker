//! `Peer` and Peer `Id` API resources.
use serde::{Deserialize, Serialize};

// issue: #2130
/// `Peer` API resource.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Peer {
    /// The peer's ID. See [`Id`].
    pub peer_id: Id,
    /// The peer's socket address. For example: `192.168.1.88:17548`.
    pub peer_addr: String,
    /// The peer's last update time as an absolute Unix timestamp in milliseconds since epoch.
    ///
    /// Deprecated: use [`Self::updated_at_ms`] instead. This field will be removed in API v2.
    #[deprecated(since = "2.0.0", note = "please use `updated_at_ms` instead")]
    pub updated: u128,
    /// The peer's last update time as an absolute Unix timestamp in milliseconds since epoch.
    ///
    /// Deprecated: despite the `_ago` suffix, this is not a relative duration. Use
    /// [`Self::updated_at_ms`] instead. This field will be removed in API v2.
    #[deprecated(since = "3.0.0", note = "please use `updated_at_ms` instead")]
    #[allow(clippy::doc_markdown)]
    pub updated_milliseconds_ago: u128,
    /// The peer's last update time as an absolute Unix timestamp in milliseconds since epoch.
    ///
    /// This field replaces [`Self::updated`] and [`Self::updated_milliseconds_ago`], which will
    /// be removed in API v2.
    pub updated_at_ms: u128,
    /// The peer's uploaded bytes.
    pub uploaded: i64,
    /// The peer's downloaded bytes.
    pub downloaded: i64,
    /// The peer's left bytes (pending to download).
    pub left: i64,
    /// The peer's event: `Started`, `Stopped`, `Completed`, `None` (`PascalCase`).
    pub event: String,
}

/// Peer `Id` API resource.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Id {
    /// The peer's ID in hex format. For example: `0x2d7142343431302d2a64465a3844484944704579`.
    pub id: Option<String>,
    /// The peer's client name. For example: `qBittorrent`.
    pub client: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{Id, Peer};

    #[test]
    fn it_should_serialize_and_deserialize_the_required_updated_at_ms_timestamp() {
        // Arrange
        #[allow(deprecated)]
        let peer = Peer {
            peer_id: Id {
                id: Some("0x2d7142343431302d2a64465a3844484944704579".to_string()),
                client: Some("qBittorrent".to_string()),
            },
            peer_addr: "192.168.1.88:17548".to_string(),
            updated: 1_680_082_693_001,
            updated_milliseconds_ago: 1_680_082_693_001,
            updated_at_ms: 1_680_082_693_001,
            uploaded: 0,
            downloaded: 0,
            left: 0,
            event: "None".to_string(),
        };

        // Act
        let serialized = serde_json::to_string(&peer).unwrap();
        let deserialized: Peer = serde_json::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(deserialized, peer);
        assert!(serialized.contains("\"updated_at_ms\":1680082693001"));
    }
}
