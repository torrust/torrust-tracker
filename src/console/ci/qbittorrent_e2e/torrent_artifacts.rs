use std::fmt::Write as _;

use anyhow::Context;
use sha1::{Digest as Sha1Digest, Sha1};

use super::bencode::BencodeValue;
use super::types::InfoHash;

/// Artifacts produced by [`build_torrent_bytes`].
pub(super) struct TorrentArtifacts {
    /// Raw bytes of the `.torrent` metainfo file.
    pub(super) torrent_bytes: Vec<u8>,
    /// v1 `InfoHash`: SHA-1 of the bencoded `info` dict, lowercase hex (40 chars).
    /// Matches the hash format returned by the qBittorrent Web API.
    pub(super) info_hash: InfoHash,
}

pub(super) fn build_payload_bytes(length: usize) -> Vec<u8> {
    let pattern = (0_u8..=250_u8).collect::<Vec<_>>();

    (0..length).map(|index| pattern[index % pattern.len()]).collect()
}

pub(super) fn build_torrent_bytes(
    payload_bytes: &[u8],
    payload_name: &str,
    announce_url: &str,
    piece_length: usize,
) -> anyhow::Result<TorrentArtifacts> {
    let pieces = payload_bytes
        .chunks(piece_length)
        .map(|piece| Sha1::digest(piece).to_vec())
        .collect::<Vec<_>>()
        .concat();

    let payload_length = i64::try_from(payload_bytes.len()).context("payload length does not fit in i64")?;
    let piece_length = i64::try_from(piece_length).context("piece length does not fit in i64")?;

    let info = BencodeValue::Dictionary(vec![
        (b"length".to_vec(), BencodeValue::Integer(payload_length)),
        (b"name".to_vec(), BencodeValue::Bytes(payload_name.as_bytes().to_vec())),
        (b"piece length".to_vec(), BencodeValue::Integer(piece_length)),
        (b"pieces".to_vec(), BencodeValue::Bytes(pieces)),
    ]);

    let info_bytes = info.encode();
    let info_hash_bytes: [u8; 20] = Sha1::digest(&info_bytes).into();
    let mut info_hash_hex = String::with_capacity(40);
    for b in info_hash_bytes {
        write!(info_hash_hex, "{b:02x}").expect("writing to String is infallible");
    }

    let torrent = BencodeValue::Dictionary(vec![
        (b"announce".to_vec(), BencodeValue::Bytes(announce_url.as_bytes().to_vec())),
        (b"created by".to_vec(), BencodeValue::Bytes(b"torrust-qb-e2e".to_vec())),
        (b"creation date".to_vec(), BencodeValue::Integer(0)),
        (b"info".to_vec(), BencodeValue::Raw(info_bytes)),
    ]);

    Ok(TorrentArtifacts {
        torrent_bytes: torrent.encode(),
        info_hash: InfoHash::new(info_hash_hex),
    })
}

#[cfg(test)]
mod tests {
    use super::{build_payload_bytes, build_torrent_bytes};

    #[test]
    fn it_should_build_payload_bytes_with_the_right_length() {
        assert_eq!(build_payload_bytes(5).len(), 5);
    }

    #[test]
    fn it_should_build_payload_bytes_with_a_repeating_pattern() {
        // Pattern starts at 0.
        assert_eq!(build_payload_bytes(3), vec![0, 1, 2]);
    }

    #[test]
    fn it_should_build_payload_bytes_wrapping_around_the_pattern() {
        // Pattern is 0..=250 (251 bytes). Index 251 wraps back to 0.
        let bytes = build_payload_bytes(252);
        assert_eq!(bytes[250], 250);
        assert_eq!(bytes[251], 0);
    }

    #[test]
    fn it_should_build_torrent_bytes_as_a_valid_bencode_dictionary() {
        // A valid bencode dict starts with b'd' and ends with b'e'.
        let payload = build_payload_bytes(1);
        let artifacts = build_torrent_bytes(&payload, "test", "http://tracker:7070/announce", 1).unwrap();
        assert_eq!(artifacts.torrent_bytes.first(), Some(&b'd'));
        assert_eq!(artifacts.torrent_bytes.last(), Some(&b'e'));
    }

    #[test]
    fn it_should_embed_the_announce_url_verbatim_in_the_torrent_bytes() {
        let payload = build_payload_bytes(1);
        let url = "http://tracker:7070/announce";
        let artifacts = build_torrent_bytes(&payload, "test", url, 1).unwrap();
        let url_bytes = url.as_bytes();
        assert!(
            artifacts.torrent_bytes.windows(url_bytes.len()).any(|w| w == url_bytes),
            "announce URL not found in torrent bytes"
        );
    }

    #[test]
    fn it_should_embed_the_info_dict_raw_so_it_appears_as_a_nested_bencode_dict() {
        // The outer dict must contain the inner info dict as a raw bencode dict
        // (starting with b'd'), not as a length-prefixed byte string.
        // This verifies the two-pass InfoHash pattern: encode info, embed via Raw.
        let payload = build_payload_bytes(1);
        let artifacts = build_torrent_bytes(&payload, "test", "http://tracker:7070/announce", 1).unwrap();
        // b"4:info" is the bencode key; the very next byte must be b'd' (dict), not a digit (byte string).
        let key = b"4:info";
        let pos = artifacts
            .torrent_bytes
            .windows(key.len())
            .position(|w| w == key)
            .expect("key '4:info' not found in torrent bytes");
        assert_eq!(
            artifacts.torrent_bytes[pos + key.len()],
            b'd',
            "info value should be a nested bencode dict (b'd'), not a byte string"
        );
    }

    #[test]
    fn it_should_produce_deterministic_torrent_bytes_for_identical_inputs() {
        let payload = build_payload_bytes(100);
        let first = build_torrent_bytes(&payload, "test.bin", "http://tracker:7070/announce", 16).unwrap();
        let second = build_torrent_bytes(&payload, "test.bin", "http://tracker:7070/announce", 16).unwrap();
        assert_eq!(first.torrent_bytes, second.torrent_bytes);
        assert_eq!(first.info_hash, second.info_hash);
    }

    #[test]
    fn it_should_produce_different_torrent_bytes_for_different_payloads() {
        let payload_a = build_payload_bytes(10);
        let payload_b = build_payload_bytes(20);
        let torrent_a = build_torrent_bytes(&payload_a, "test", "http://tracker:7070/announce", 8).unwrap();
        let torrent_b = build_torrent_bytes(&payload_b, "test", "http://tracker:7070/announce", 8).unwrap();
        assert_ne!(torrent_a.torrent_bytes, torrent_b.torrent_bytes);
        assert_ne!(torrent_a.info_hash, torrent_b.info_hash);
    }

    #[test]
    fn it_should_produce_a_40_character_lowercase_hex_info_hash() {
        let payload = build_payload_bytes(100);
        let artifacts = build_torrent_bytes(&payload, "test.bin", "http://tracker:7070/announce", 16).unwrap();
        assert_eq!(
            artifacts.info_hash.as_str().len(),
            40,
            "InfoHash hex must be 40 characters (20 bytes × 2)"
        );
        assert!(
            artifacts
                .info_hash
                .as_str()
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()),
            "InfoHash hex must contain only lowercase hex digits"
        );
    }

    #[test]
    fn it_should_produce_a_different_info_hash_when_only_the_payload_changes() {
        // The InfoHash covers the info dict (payload content, name, piece length).
        // Two torrents with different payloads must have different hashes.
        let payload_a = build_payload_bytes(10);
        let payload_b = build_payload_bytes(20);
        let hash_a = build_torrent_bytes(&payload_a, "test", "http://tracker:7070/announce", 8)
            .unwrap()
            .info_hash;
        let hash_b = build_torrent_bytes(&payload_b, "test", "http://tracker:7070/announce", 8)
            .unwrap()
            .info_hash;
        assert_ne!(hash_a, hash_b);
    }

    #[test]
    fn it_should_produce_the_same_info_hash_regardless_of_the_announce_url() {
        // The announce URL is outside the info dict and must not affect the InfoHash.
        let payload = build_payload_bytes(10);
        let hash_a = build_torrent_bytes(&payload, "test", "http://tracker-a:7070/announce", 8)
            .unwrap()
            .info_hash;
        let hash_b = build_torrent_bytes(&payload, "test", "http://tracker-b:7070/announce", 8)
            .unwrap()
            .info_hash;
        assert_eq!(hash_a, hash_b, "announce URL must not affect the InfoHash");
    }
}
