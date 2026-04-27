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
