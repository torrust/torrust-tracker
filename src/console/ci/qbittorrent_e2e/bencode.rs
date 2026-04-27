pub(crate) enum BencodeValue {
    Integer(i64),
    Bytes(Vec<u8>),
    Dictionary(Vec<(Vec<u8>, BencodeValue)>),
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
