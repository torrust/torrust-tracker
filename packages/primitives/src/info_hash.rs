use std::hash::{DefaultHasher, Hash, Hasher};
use std::panic::Location;

use thiserror::Error;

pub type ByteArray20 = [u8; 20];

/// `BitTorrent` Info Hash v1
#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
pub struct InfoHash(pub ByteArray20);

pub const INFO_HASH_BYTES_LEN: usize = 20;

impl InfoHash {
    /// Returns the `InfoHash` internal byte array.
    #[must_use]
    pub fn bytes(&self) -> ByteArray20 {
        self.0
    }

    /// Returns the `InfoHash` as a hex string.
    #[must_use]
    pub fn to_hex_string(&self) -> String {
        self.to_string()
    }
}

impl Ord for InfoHash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::cmp::PartialOrd<InfoHash> for InfoHash {
    fn partial_cmp(&self, other: &InfoHash) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for InfoHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = [0u8; 40];
        binascii::bin2hex(&self.0, &mut chars).expect("failed to hexlify");
        write!(f, "{}", std::str::from_utf8(&chars).unwrap())
    }
}

impl std::str::FromStr for InfoHash {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const INFO_HASH_CHAR_LEN: usize = INFO_HASH_BYTES_LEN * 2;
        let mut i = Self([0u8; 20]);

        if s.len() < INFO_HASH_CHAR_LEN {
            return Err(ConversionError::NotEnoughBytes {
                location: Location::caller(),
                message: format! {"got {} string, expected {}", s.len(), INFO_HASH_CHAR_LEN},
            });
        }
        if s.len() > INFO_HASH_CHAR_LEN {
            return Err(ConversionError::TooManyBytes {
                location: Location::caller(),
                message: format! {"got {} string, expected {}", s.len(), INFO_HASH_CHAR_LEN},
            });
        }

        binascii::hex2bin(s.as_bytes(), &mut i.0).map_err(|e| ConversionError::HexToBinError {
            location: Location::caller(),
            message: format! {"got {e:?} error"},
        })?;

        Ok(i)
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

/// for testing
impl std::convert::From<&DefaultHasher> for InfoHash {
    fn from(data: &DefaultHasher) -> InfoHash {
        let n = data.finish().to_le_bytes();
        InfoHash([
            n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[0], n[1], n[2],
            n[3],
        ])
    }
}

impl std::convert::From<i32> for InfoHash {
    fn from(n: i32) -> InfoHash {
        let n = n.to_le_bytes();
        InfoHash([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, n[0], n[1], n[2], n[3]])
    }
}

impl std::convert::From<ByteArray20> for InfoHash {
    fn from(val: ByteArray20) -> Self {
        InfoHash(val)
    }
}

/// Errors that can occur when converting from a `Vec<u8>` to an `InfoHash`.
#[derive(Error, Debug)]
pub enum ConversionError {
    /// Not enough bytes for infohash. An infohash is 20 bytes.
    #[error("not enough bytes for infohash: {message} {location}")]
    NotEnoughBytes {
        location: &'static Location<'static>,
        message: String,
    },
    /// Too many bytes for infohash. An infohash is 20 bytes.
    #[error("too many bytes for infohash: {message} {location}")]
    TooManyBytes {
        location: &'static Location<'static>,
        message: String,
    },

    #[error("hex to bin didn't parse: {message} {location}")]
    HexToBinError {
        location: &'static Location<'static>,
        message: String,
    },
}

impl TryFrom<Vec<u8>> for InfoHash {
    type Error = ConversionError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.len() < INFO_HASH_BYTES_LEN {
            return Err(ConversionError::NotEnoughBytes {
                location: Location::caller(),
                message: format! {"got {} bytes, expected {}", bytes.len(), INFO_HASH_BYTES_LEN},
            });
        }
        if bytes.len() > INFO_HASH_BYTES_LEN {
            return Err(ConversionError::TooManyBytes {
                location: Location::caller(),
                message: format! {"got {} bytes, expected {}", bytes.len(), INFO_HASH_BYTES_LEN},
            });
        }
        Ok((*bytes).into())
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

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
