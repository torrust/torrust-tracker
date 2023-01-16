#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct InfoHash(pub [u8; 20]);

impl std::fmt::Display for InfoHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut chars = [0u8; 40];
        binascii::bin2hex(&self.0, &mut chars).expect("failed to hexlify");
        write!(f, "{}", std::str::from_utf8(&chars).unwrap())
    }
}

impl std::str::FromStr for InfoHash {
    type Err = binascii::ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = Self([0u8; 20]);
        if s.len() != 40 {
            return Err(binascii::ConvertError::InvalidInputLength);
        }
        binascii::hex2bin(s.as_bytes(), &mut i.0)?;
        Ok(i)
    }
}

impl Ord for InfoHash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::cmp::PartialOrd<InfoHash> for InfoHash {
    fn partial_cmp(&self, other: &InfoHash) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::convert::From<&[u8]> for InfoHash {
    fn from(data: &[u8]) -> InfoHash {
        assert_eq!(data.len(), 20);
        let mut ret = InfoHash([0u8; 20]);
        ret.0.clone_from_slice(data);
        ret
    }
}

impl std::convert::From<[u8; 20]> for InfoHash {
    fn from(val: [u8; 20]) -> Self {
        InfoHash(val)
    }
}

impl serde::ser::Serialize for InfoHash {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut buffer = [0u8; 40];
        let bytes_out = binascii::bin2hex(&self.0, &mut buffer).ok().unwrap();
        let str_out = std::str::from_utf8(bytes_out).unwrap();
        serializer.serialize_str(str_out)
    }
}

impl<'de> serde::de::Deserialize<'de> for InfoHash {
    fn deserialize<D: serde::de::Deserializer<'de>>(des: D) -> Result<Self, D::Error> {
        des.deserialize_str(InfoHashVisitor)
    }
}

struct InfoHashVisitor;

impl<'v> serde::de::Visitor<'v> for InfoHashVisitor {
    type Value = InfoHash;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a 40 character long hash")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        if v.len() != 40 {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"a 40 character long string",
            ));
        }

        let mut res = InfoHash([0u8; 20]);

        if binascii::hex2bin(v.as_bytes(), &mut res.0).is_err() {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"a hexadecimal string",
            ));
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use super::InfoHash;

    #[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
    struct ContainingInfoHash {
        pub info_hash: InfoHash,
    }

    #[test]
    fn an_info_hash_can_be_created_from_a_valid_40_utf8_char_string_representing_an_hexadecimal_value() {
        let info_hash = InfoHash::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
        assert!(info_hash.is_ok());
    }

    #[test]
    fn an_info_hash_can_not_be_created_from_a_utf8_string_representing_a_not_valid_hexadecimal_value() {
        let info_hash = InfoHash::from_str("GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG");
        assert!(info_hash.is_err());
    }

    #[test]
    fn an_info_hash_can_only_be_created_from_a_40_utf8_char_string() {
        let info_hash = InfoHash::from_str(&"F".repeat(39));
        assert!(info_hash.is_err());

        let info_hash = InfoHash::from_str(&"F".repeat(41));
        assert!(info_hash.is_err());
    }

    #[test]
    fn an_info_hash_should_by_displayed_like_a_40_utf8_lowercased_char_hex_string() {
        let info_hash = InfoHash::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();

        let output = format!("{info_hash}");

        assert_eq!(output, "ffffffffffffffffffffffffffffffffffffffff");
    }

    #[test]
    fn an_info_hash_can_be_created_from_a_valid_20_byte_array_slice() {
        let info_hash: InfoHash = [255u8; 20].as_slice().into();

        assert_eq!(
            info_hash,
            InfoHash::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap()
        );
    }

    #[test]
    fn an_info_hash_can_be_created_from_a_valid_20_byte_array() {
        let info_hash: InfoHash = [255u8; 20].into();

        assert_eq!(
            info_hash,
            InfoHash::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap()
        );
    }

    #[test]
    fn an_info_hash_can_be_serialized() {
        let s = ContainingInfoHash {
            info_hash: InfoHash::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap(),
        };

        let json_serialized_value = serde_json::to_string(&s).unwrap();

        assert_eq!(
            json_serialized_value,
            r#"{"info_hash":"ffffffffffffffffffffffffffffffffffffffff"}"#
        );
    }

    #[test]
    fn an_info_hash_can_be_deserialized() {
        let json = json!({
            "info_hash": "ffffffffffffffffffffffffffffffffffffffff",
        });

        let s: ContainingInfoHash = serde_json::from_value(json).unwrap();

        assert_eq!(
            s,
            ContainingInfoHash {
                info_hash: InfoHash::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap()
            }
        );
    }
}
