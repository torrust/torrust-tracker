use anyhow::Context;
use sha1::{Digest as Sha1Digest, Sha1};

use super::bencode::BencodeValue;

pub(super) fn build_payload_bytes(length: usize) -> Vec<u8> {
    let pattern = (0_u8..=250_u8).collect::<Vec<_>>();

    (0..length).map(|index| pattern[index % pattern.len()]).collect()
}

pub(super) fn build_torrent_bytes(
    payload_bytes: &[u8],
    payload_name: &str,
    announce_url: &str,
    piece_length: usize,
) -> anyhow::Result<Vec<u8>> {
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
    let torrent = BencodeValue::Dictionary(vec![
        (b"announce".to_vec(), BencodeValue::Bytes(announce_url.as_bytes().to_vec())),
        (b"created by".to_vec(), BencodeValue::Bytes(b"torrust-qb-e2e".to_vec())),
        (b"creation date".to_vec(), BencodeValue::Integer(0)),
        (b"info".to_vec(), BencodeValue::Raw(info_bytes)),
    ]);

    Ok(torrent.encode())
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
        let torrent = build_torrent_bytes(&payload, "test", "http://tracker:7070/announce", 1).unwrap();
        assert_eq!(torrent.first(), Some(&b'd'));
        assert_eq!(torrent.last(), Some(&b'e'));
    }

    #[test]
    fn it_should_embed_the_announce_url_verbatim_in_the_torrent_bytes() {
        let payload = build_payload_bytes(1);
        let url = "http://tracker:7070/announce";
        let torrent = build_torrent_bytes(&payload, "test", url, 1).unwrap();
        let url_bytes = url.as_bytes();
        assert!(
            torrent.windows(url_bytes.len()).any(|w| w == url_bytes),
            "announce URL not found in torrent bytes"
        );
    }

    #[test]
    fn it_should_embed_the_info_dict_raw_so_it_appears_as_a_nested_bencode_dict() {
        // The outer dict must contain the inner info dict as a raw bencode dict
        // (starting with b'd'), not as a length-prefixed byte string.
        // This verifies the two-pass InfoHash pattern: encode info, embed via Raw.
        let payload = build_payload_bytes(1);
        let torrent = build_torrent_bytes(&payload, "test", "http://tracker:7070/announce", 1).unwrap();
        // b"4:info" is the bencode key; the very next byte must be b'd' (dict), not a digit (byte string).
        let key = b"4:info";
        let pos = torrent
            .windows(key.len())
            .position(|w| w == key)
            .expect("key '4:info' not found in torrent bytes");
        assert_eq!(
            torrent[pos + key.len()],
            b'd',
            "info value should be a nested bencode dict (b'd'), not a byte string"
        );
    }

    #[test]
    fn it_should_produce_deterministic_torrent_bytes_for_identical_inputs() {
        let payload = build_payload_bytes(100);
        let first = build_torrent_bytes(&payload, "test.bin", "http://tracker:7070/announce", 16).unwrap();
        let second = build_torrent_bytes(&payload, "test.bin", "http://tracker:7070/announce", 16).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn it_should_produce_different_torrent_bytes_for_different_payloads() {
        let payload_a = build_payload_bytes(10);
        let payload_b = build_payload_bytes(20);
        let torrent_a = build_torrent_bytes(&payload_a, "test", "http://tracker:7070/announce", 8).unwrap();
        let torrent_b = build_torrent_bytes(&payload_b, "test", "http://tracker:7070/announce", 8).unwrap();
        assert_ne!(torrent_a, torrent_b);
    }
}
