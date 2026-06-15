//! Minimal bencode encoder for generating `.torrent` files in E2E tests.
//!
//! This module intentionally avoids pulling in `serde_bencode` or
//! `torrust-bencode`. The key reason is the `BencodeValue::Raw`
//! variant: it embeds pre-encoded bytes verbatim inside an outer dictionary,
//! which is required for the two-pass `InfoHash` pattern (encode the `info` dict,
//! SHA-1 hash it, then embed the raw bytes into the outer torrent dict). Neither
//! `serde_bencode` nor `torrust-bencode` can express that semantics without an
//! equivalent workaround.
//!
//! If encoding needs grow in complexity, consider migrating to one of those
//! crates rather than expanding this module.

pub(crate) enum BencodeValue {
    Integer(i64),
    Bytes(Vec<u8>),
    Dictionary(Vec<(Vec<u8>, Self)>),
    Raw(Vec<u8>),
}

impl BencodeValue {
    #[must_use]
    pub(crate) fn encode(&self) -> Vec<u8> {
        match self {
            Self::Integer(value) => format!("i{value}e").into_bytes(),
            Self::Bytes(value) => encode_bytes(value),
            Self::Dictionary(entries) => encode_dictionary(entries),
            Self::Raw(value) => value.clone(),
        }
    }
}

fn encode_dictionary(entries: &[(Vec<u8>, BencodeValue)]) -> Vec<u8> {
    let mut sorted_entries = entries.iter().collect::<Vec<_>>();
    sorted_entries.sort_by(|left, right| left.0.cmp(&right.0));

    let mut encoded = Vec::from(*b"d");
    for (key, value) in sorted_entries {
        encoded.extend(encode_bytes(key));
        encoded.extend(value.encode());
    }
    encoded.push(b'e');
    encoded
}

fn encode_bytes(value: &[u8]) -> Vec<u8> {
    let mut encoded = value.len().to_string().into_bytes();
    encoded.push(b':');
    encoded.extend(value);
    encoded
}

#[cfg(test)]
mod tests {
    use super::BencodeValue;

    #[test]
    fn it_should_encode_a_positive_integer() {
        assert_eq!(BencodeValue::Integer(42).encode(), b"i42e");
    }

    #[test]
    fn it_should_encode_a_negative_integer() {
        assert_eq!(BencodeValue::Integer(-3).encode(), b"i-3e");
    }

    #[test]
    fn it_should_encode_zero() {
        assert_eq!(BencodeValue::Integer(0).encode(), b"i0e");
    }

    #[test]
    fn it_should_encode_a_byte_string() {
        assert_eq!(BencodeValue::Bytes(b"spam".to_vec()).encode(), b"4:spam");
    }

    #[test]
    fn it_should_encode_an_empty_byte_string() {
        assert_eq!(BencodeValue::Bytes(vec![]).encode(), b"0:");
    }

    #[test]
    fn it_should_encode_a_dictionary_with_keys_sorted_lexicographically() {
        // Keys "bar" < "foo" — even though "foo" is listed first.
        let dict = BencodeValue::Dictionary(vec![
            (b"foo".to_vec(), BencodeValue::Integer(1)),
            (b"bar".to_vec(), BencodeValue::Integer(2)),
        ]);
        assert_eq!(dict.encode(), b"d3:bari2e3:fooi1ee"); // cspell:disable-line
    }

    #[test]
    fn it_should_encode_an_empty_dictionary() {
        assert_eq!(BencodeValue::Dictionary(vec![]).encode(), b"de");
    }

    #[test]
    fn it_should_embed_raw_bytes_verbatim() {
        // Raw is used to embed a pre-encoded inner dict (e.g. the info dict)
        // without re-encoding it. The bytes must appear unchanged in the output.
        let inner = BencodeValue::Integer(7).encode(); // b"i7e"
        assert_eq!(BencodeValue::Raw(inner).encode(), b"i7e");
    }

    #[test]
    fn it_should_embed_raw_inner_dict_inside_outer_dict() {
        // Simulates the two-pass InfoHash pattern: encode the info dict first,
        // then wrap it in the outer torrent dict via Raw.
        let info = BencodeValue::Dictionary(vec![(b"length".to_vec(), BencodeValue::Integer(100))]);
        let info_bytes = info.encode(); // b"d6:lengthi100ee" // cspell:disable-line

        let torrent = BencodeValue::Dictionary(vec![(b"info".to_vec(), BencodeValue::Raw(info_bytes))]);

        assert_eq!(torrent.encode(), b"d4:infod6:lengthi100eee"); // cspell:disable-line
    }
}
