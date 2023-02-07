use crate::protocol::info_hash::{ConversionError, InfoHash};
use crate::tracker::peer::{self, IdConversionError};

/// # Errors
///
/// Will return `Err` if if the decoded bytes do not represent a valid `InfoHash`.
pub fn percent_decode_info_hash(raw_info_hash: &str) -> Result<InfoHash, ConversionError> {
    let bytes = percent_encoding::percent_decode_str(raw_info_hash).collect::<Vec<u8>>();
    InfoHash::try_from(bytes)
}

/// # Errors
///
/// Will return `Err` if if the decoded bytes do not represent a valid `peer::Id`.
pub fn percent_decode_peer_id(raw_peer_id: &str) -> Result<peer::Id, IdConversionError> {
    let bytes = percent_encoding::percent_decode_str(raw_peer_id).collect::<Vec<u8>>();
    peer::Id::try_from(bytes)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::http::percent_encoding::{percent_decode_info_hash, percent_decode_peer_id};
    use crate::protocol::info_hash::InfoHash;
    use crate::tracker::peer;

    #[test]
    fn it_should_decode_a_percent_encoded_info_hash() {
        let encoded_infohash = "%3B%24U%04%CF%5F%11%BB%DB%E1%20%1C%EAjk%F4Z%EE%1B%C0";

        let info_hash = percent_decode_info_hash(encoded_infohash).unwrap();

        assert_eq!(
            info_hash,
            InfoHash::from_str("3b245504cf5f11bbdbe1201cea6a6bf45aee1bc0").unwrap()
        );
    }

    #[test]
    fn it_should_fail_decoding_an_invalid_percent_encoded_info_hash() {
        let invalid_encoded_infohash = "invalid percent-encoded infohash";

        let info_hash = percent_decode_info_hash(invalid_encoded_infohash);

        assert!(info_hash.is_err());
    }

    #[test]
    fn it_should_decode_a_percent_encoded_peer_id() {
        let encoded_peer_id = "%2DqB00000000000000000";

        let peer_id = percent_decode_peer_id(encoded_peer_id).unwrap();

        assert_eq!(peer_id, peer::Id(*b"-qB00000000000000000"));
    }

    #[test]
    fn it_should_fail_decoding_an_invalid_percent_encoded_peer_id() {
        let invalid_encoded_peer_id = "invalid percent-encoded peer id";

        let peer_id = percent_decode_peer_id(invalid_encoded_peer_id);

        assert!(peer_id.is_err());
    }
}
