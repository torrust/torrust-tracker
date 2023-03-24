//! A `BitTorrent` `InfoHash`. It's a unique identifier for a `BitTorrent` torrent.
//!
//! "The 20-byte sha1 hash of the bencoded form of the info value
//! from the metainfo file."
//!
//! See [BEP 3. The `BitTorrent` Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
//! for the official specification.
//!
//! This modules provides a type that can be used to represent infohashes.
//!
//! > **NOTICE**: It only supports Info Hash v1.
//!
//! Typically infohashes are represented as hex strings, but internally they are
//! a 20-byte array.
//!
//! # Calculating the info-hash of a torrent file
//!
//! A sample torrent:
//!
//! - Torrent file: `mandelbrot_2048x2048_infohash_v1.png.torrent`
//! - File: `mandelbrot_2048x2048.png`
//! - Info Hash v1: `5452869be36f9f3350ccee6b4544e7e76caaadab`
//! - Sha1 hash of the info dictionary: `5452869BE36F9F3350CCEE6B4544E7E76CAAADAB`
//!
//! A torrent file is a binary file encoded with [Bencode encoding](https://en.wikipedia.org/wiki/Bencode):
//!
//! ```text
//! 0000000: 6431 303a 6372 6561 7465 6420 6279 3138  d10:created by18
//! 0000010: 3a71 4269 7474 6f72 7265 6e74 2076 342e  :qBittorrent v4.
//! 0000020: 342e 3131 333a 6372 6561 7469 6f6e 2064  4.113:creation d
//! 0000030: 6174 6569 3136 3739 3637 3436 3238 6534  atei1679674628e4
//! 0000040: 3a69 6e66 6f64 363a 6c65 6e67 7468 6931  :infod6:lengthi1
//! 0000050: 3732 3230 3465 343a 6e61 6d65 3234 3a6d  72204e4:name24:m
//! 0000060: 616e 6465 6c62 726f 745f 3230 3438 7832  andelbrot_2048x2
//! 0000070: 3034 382e 706e 6731 323a 7069 6563 6520  048.png12:piece
//! 0000080: 6c65 6e67 7468 6931 3633 3834 6536 3a70  lengthi16384e6:p
//! 0000090: 6965 6365 7332 3230 3a7d 9171 0d9d 4dba  ieces220:}.q..M.
//! 00000a0: 889b 5420 54d5 2672 8d5a 863f e121 df77  ..T T.&r.Z.?.!.w
//! 00000b0: c7f7 bb6c 7796 2166 2538 c5d9 cdab 8b08  ...lw.!f%8......
//! 00000c0: ef8c 249b b2f5 c4cd 2adf 0bc0 0cf0 addf  ..$.....*.......
//! 00000d0: 7290 e5b6 414c 236c 479b 8e9f 46aa 0c0d  r...AL#lG...F...
//! 00000e0: 8ed1 97ff ee68 8b5f 34a3 87d7 71c5 a6f9  .....h._4...q...
//! 00000f0: 8e2e a631 7cbd f0f9 e223 f9cc 80af 5400  ...1|....#....T.
//! 0000100: 04f9 8569 1c77 89c1 764e d6aa bf61 a6c2  ...i.w..vN...a..
//! 0000110: 8099 abb6 5f60 2f40 a825 be32 a33d 9d07  ...._`/@.%.2.=..
//! 0000120: 0c79 6898 d49d 6349 af20 5866 266f 986b  .yh...cI. Xf&o.k
//! 0000130: 6d32 34cd 7d08 155e 1ad0 0009 57ab 303b  m24.}..^....W.0;
//! 0000140: 2060 c1dc 1287 d6f3 e745 4f70 6709 3631   `.......EOpg.61
//! 0000150: 55f2 20f6 6ca5 156f 2c89 9569 1653 817d  U. .l..o,..i.S.}
//! 0000160: 31f1 b6bd 3742 cc11 0bb2 fc2b 49a5 85b6  1...7B.....+I...
//! 0000170: fc76 7444 9365 65                        .vtD.ee
//! ```
//!
//! You can generate that output with the command:
//!
//! ```text
//! xxd mandelbrot_2048x2048_infohash_v1.png.torrent
//! ```
//!
//! And you can show only the bytes (hexadecimal):
//!
//! ```text
//! 6431303a6372656174656420627931383a71426974746f7272656e742076
//! 342e342e3131333a6372656174696f6e2064617465693136373936373436
//! 323865343a696e666f64363a6c656e6774686931373232303465343a6e61
//! 6d6532343a6d616e64656c62726f745f3230343878323034382e706e6731
//! 323a7069656365206c656e67746869313633383465363a70696563657332
//! 32303a7d91710d9d4dba889b542054d526728d5a863fe121df77c7f7bb6c
//! 779621662538c5d9cdab8b08ef8c249bb2f5c4cd2adf0bc00cf0addf7290
//! e5b6414c236c479b8e9f46aa0c0d8ed197ffee688b5f34a387d771c5a6f9
//! 8e2ea6317cbdf0f9e223f9cc80af540004f985691c7789c1764ed6aabf61
//! a6c28099abb65f602f40a825be32a33d9d070c796898d49d6349af205866
//! 266f986b6d3234cd7d08155e1ad0000957ab303b2060c1dc1287d6f3e745
//! 4f706709363155f220f66ca5156f2c8995691653817d31f1b6bd3742cc11
//! 0bb2fc2b49a585b6fc767444936565
//! ```
//!
//! You can generate that output with the command:
//!
//! ```text
//! `xxd -ps mandelbrot_2048x2048_infohash_v1.png.torrent`.
//! ```
//!
//! The same data can be represented in a JSON format:
//!
//! ```json
//! {
//!     "created by": "qBittorrent v4.4.1",
//!     "creation date": 1679674628,
//!     "info": {
//!         "length": 172204,
//!         "name": "mandelbrot_2048x2048.png",
//!         "piece length": 16384,
//!         "pieces": "<hex>7D 91 71 0D 9D 4D BA 88 9B 54 20 54 D5 26 72 8D 5A 86 3F E1 21 DF 77 C7 F7 BB 6C 77 96 21 66 25 38 C5 D9 CD AB 8B 08 EF 8C 24 9B B2 F5 C4 CD 2A DF 0B C0 0C F0 AD DF 72 90 E5 B6 41 4C 23 6C 47 9B 8E 9F 46 AA 0C 0D 8E D1 97 FF EE 68 8B 5F 34 A3 87 D7 71 C5 A6 F9 8E 2E A6 31 7C BD F0 F9 E2 23 F9 CC 80 AF 54 00 04 F9 85 69 1C 77 89 C1 76 4E D6 AA BF 61 A6 C2 80 99 AB B6 5F 60 2F 40 A8 25 BE 32 A3 3D 9D 07 0C 79 68 98 D4 9D 63 49 AF 20 58 66 26 6F 98 6B 6D 32 34 CD 7D 08 15 5E 1A D0 00 09 57 AB 30 3B 20 60 C1 DC 12 87 D6 F3 E7 45 4F 70 67 09 36 31 55 F2 20 F6 6C A5 15 6F 2C 89 95 69 16 53 81 7D 31 F1 B6 BD 37 42 CC 11 0B B2 FC 2B 49 A5 85 B6 FC 76 74 44 93</hex>"
//!     }
//! }
//! ```
//!
//! The JSON object was generated with: <https://github.com/Chocobo1/bencode_online>
//!
//! As you can see, there is a `info` attribute:
//!
//! ```json
//! {
//!     "length": 172204,
//!     "name": "mandelbrot_2048x2048.png",
//!     "piece length": 16384,
//!     "pieces": "<hex>7D 91 71 0D 9D 4D BA 88 9B 54 20 54 D5 26 72 8D 5A 86 3F E1 21 DF 77 C7 F7 BB 6C 77 96 21 66 25 38 C5 D9 CD AB 8B 08 EF 8C 24 9B B2 F5 C4 CD 2A DF 0B C0 0C F0 AD DF 72 90 E5 B6 41 4C 23 6C 47 9B 8E 9F 46 AA 0C 0D 8E D1 97 FF EE 68 8B 5F 34 A3 87 D7 71 C5 A6 F9 8E 2E A6 31 7C BD F0 F9 E2 23 F9 CC 80 AF 54 00 04 F9 85 69 1C 77 89 C1 76 4E D6 AA BF 61 A6 C2 80 99 AB B6 5F 60 2F 40 A8 25 BE 32 A3 3D 9D 07 0C 79 68 98 D4 9D 63 49 AF 20 58 66 26 6F 98 6B 6D 32 34 CD 7D 08 15 5E 1A D0 00 09 57 AB 30 3B 20 60 C1 DC 12 87 D6 F3 E7 45 4F 70 67 09 36 31 55 F2 20 F6 6C A5 15 6F 2C 89 95 69 16 53 81 7D 31 F1 B6 BD 37 42 CC 11 0B B2 FC 2B 49 A5 85 B6 FC 76 74 44 93</hex>"
//!  }
//! ```
//!
//! The infohash is the [SHA1](https://en.wikipedia.org/wiki/SHA-1) hash
//! of the `info` attribute. That is, the SHA1 hash of:
//!
//! ```text
//! 64363a6c656e6774686931373232303465343a6e61
//! d6532343a6d616e64656c62726f745f3230343878323034382e706e6731
//! 23a7069656365206c656e67746869313633383465363a70696563657332
//! 2303a7d91710d9d4dba889b542054d526728d5a863fe121df77c7f7bb6c
//! 79621662538c5d9cdab8b08ef8c249bb2f5c4cd2adf0bc00cf0addf7290
//! 5b6414c236c479b8e9f46aa0c0d8ed197ffee688b5f34a387d771c5a6f9
//! e2ea6317cbdf0f9e223f9cc80af540004f985691c7789c1764ed6aabf61
//! 6c28099abb65f602f40a825be32a33d9d070c796898d49d6349af205866
//! 66f986b6d3234cd7d08155e1ad0000957ab303b2060c1dc1287d6f3e745
//! f706709363155f220f66ca5156f2c8995691653817d31f1b6bd3742cc11
//! bb2fc2b49a585b6fc7674449365
//! ```
//!
//! You can hash that byte string with <https://www.pelock.com/products/hash-calculator>
//!
//! The result is a 20-char string: `5452869BE36F9F3350CCEE6B4544E7E76CAAADAB`
use std::panic::Location;

use thiserror::Error;

/// `BitTorrent` Info Hash v1
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct InfoHash(pub [u8; 20]);

const INFO_HASH_BYTES_LEN: usize = 20;

impl InfoHash {
    /// Create a new `InfoHash` from a byte slice.
    ///
    /// # Panics
    ///
    /// Will panic if byte slice does not contains the exact amount of bytes need for the `InfoHash`.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len(), INFO_HASH_BYTES_LEN);
        let mut ret = Self([0u8; INFO_HASH_BYTES_LEN]);
        ret.0.clone_from_slice(bytes);
        ret
    }

    /// Returns the `InfoHash` internal byte array.
    #[must_use]
    pub fn bytes(&self) -> [u8; 20] {
        self.0
    }

    /// Returns the `InfoHash` as a hex string.
    #[must_use]
    pub fn to_hex_string(&self) -> String {
        self.to_string()
    }
}

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
        Ok(Self::from_bytes(&bytes))
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
    fn an_info_hash_should_return_its_a_40_utf8_lowercased_char_hex_representations_as_string() {
        let info_hash = InfoHash::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();

        assert_eq!(info_hash.to_hex_string(), "ffffffffffffffffffffffffffffffffffffffff");
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
    fn an_info_hash_can_be_created_from_a_byte_vector() {
        let info_hash: InfoHash = [255u8; 20].to_vec().try_into().unwrap();

        assert_eq!(
            info_hash,
            InfoHash::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap()
        );
    }

    #[test]
    fn it_should_fail_trying_to_create_an_info_hash_from_a_byte_vector_with_less_than_20_bytes() {
        assert!(InfoHash::try_from([255u8; 19].to_vec()).is_err());
    }

    #[test]
    fn it_should_fail_trying_to_create_an_info_hash_from_a_byte_vector_with_more_than_20_bytes() {
        assert!(InfoHash::try_from([255u8; 21].to_vec()).is_err());
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
