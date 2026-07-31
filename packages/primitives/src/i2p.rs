//! I2P addressing primitives.

use std::fmt;
use std::str::FromStr;

use base64::Engine;
use base64::alphabet::Alphabet;
use base64::engine::{GeneralPurpose, GeneralPurposeConfig};
use sha2::{Digest, Sha256};
use thiserror::Error;

const I2P_BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-~";
const I2P_SUFFIX: &str = "i2p";
const MIN_I2P_DESTINATION_BYTES: usize = 387;
const I2P_CERTIFICATE_LENGTH_OFFSET: usize = 385;

/// A validated I2P Base64 Destination.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I2pDestination {
    value: Box<str>,
    hash: [u8; 32],
}

impl I2pDestination {
    /// Returns the SHA-256 hash of the decoded binary Destination.
    #[must_use]
    pub const fn hash(&self) -> &[u8; 32] {
        &self.hash
    }
}

impl fmt::Display for I2pDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.value)
    }
}

impl FromStr for I2pDestination {
    type Err = ParseI2pDestinationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let encoded = value
            .rsplit_once('.')
            .filter(|(_, suffix)| suffix.eq_ignore_ascii_case(I2P_SUFFIX))
            .map_or(value, |(encoded, _)| encoded);
        let alphabet =
            Alphabet::new(I2P_BASE64_ALPHABET).expect("the I2P Base64 alphabet must contain 64 unique ASCII characters");
        let engine = GeneralPurpose::new(&alphabet, GeneralPurposeConfig::new());
        let decoded = engine.decode(encoded).map_err(|_| ParseI2pDestinationError::InvalidBase64)?;

        if decoded.len() < MIN_I2P_DESTINATION_BYTES {
            return Err(ParseI2pDestinationError::TooShort { actual: decoded.len() });
        }

        let certificate_payload_length = usize::from(u16::from_be_bytes([
            decoded[I2P_CERTIFICATE_LENGTH_OFFSET],
            decoded[I2P_CERTIFICATE_LENGTH_OFFSET + 1],
        ]));
        let expected_length = MIN_I2P_DESTINATION_BYTES + certificate_payload_length;

        if decoded.len() != expected_length {
            return Err(ParseI2pDestinationError::InvalidCertificateLength {
                declared: certificate_payload_length,
                actual: decoded.len() - MIN_I2P_DESTINATION_BYTES,
            });
        }

        Ok(Self {
            value: format!("{encoded}.{I2P_SUFFIX}").into_boxed_str(),
            hash: Sha256::digest(decoded).into(),
        })
    }
}

/// Error returned when parsing an I2P Destination.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseI2pDestinationError {
    #[error("the I2P Destination is not valid I2P Base64")]
    InvalidBase64,
    #[error("the decoded I2P Destination must contain at least {MIN_I2P_DESTINATION_BYTES} bytes, got {actual}")]
    TooShort { actual: usize },
    #[error("the I2P certificate declares a {declared}-byte payload, but the Destination contains {actual} payload bytes")]
    InvalidCertificateLength { declared: usize, actual: usize },
}

/// An I2P peer address. I2P routes by Destination and has no peer port.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct I2pPeerAddress {
    /// The peer's full I2P Destination.
    pub destination: I2pDestination,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_and_normalize_a_valid_i2p_base64_destination() {
        let destination = "A".repeat(516);
        let destination_with_uppercase_suffix = format!("{destination}.I2P");

        let parsed = I2pDestination::from_str(&destination_with_uppercase_suffix).unwrap();

        assert_eq!(parsed.to_string(), format!("{destination}.i2p"));
        assert_eq!(
            parsed.hash(),
            &[
                0x31, 0x19, 0xfc, 0xeb, 0x0e, 0xad, 0x1d, 0x08, 0x04, 0xdb, 0x90, 0xfb, 0x0c, 0x87, 0xa3, 0x38, 0x10, 0x89, 0xf9,
                0xd2, 0x26, 0x4a, 0x37, 0x6c, 0x41, 0xa3, 0x9a, 0x06, 0xe5, 0x32, 0xa6, 0x41,
            ]
        );
    }

    #[test]
    fn it_should_reject_an_i2p_destination_with_invalid_base64() {
        let destination = format!("{}.i2p", "!".repeat(516));

        let error = I2pDestination::from_str(&destination).unwrap_err();

        assert_eq!(error, ParseI2pDestinationError::InvalidBase64);
    }

    #[test]
    fn it_should_reject_an_i2p_destination_shorter_than_the_minimum_length() {
        let destination = "A".repeat(512);

        let error = I2pDestination::from_str(&destination).unwrap_err();

        assert_eq!(error, ParseI2pDestinationError::TooShort { actual: 384 });
    }

    #[test]
    fn it_should_parse_a_long_padded_destination_when_the_certificate_length_matches() {
        let certificate_payload_length = 91_u16;
        let mut decoded = vec![0; 387 + usize::from(certificate_payload_length)];
        decoded[384] = 5;
        decoded[385..387].copy_from_slice(&certificate_payload_length.to_be_bytes());
        let alphabet = Alphabet::new(I2P_BASE64_ALPHABET).unwrap();
        let encoded = GeneralPurpose::new(&alphabet, GeneralPurposeConfig::new()).encode(decoded);

        let parsed = I2pDestination::from_str(&encoded).unwrap();

        assert!(encoded.ends_with("=="));
        assert_eq!(parsed.to_string(), format!("{encoded}.i2p"));
    }

    #[test]
    fn it_should_reject_a_destination_when_the_certificate_length_does_not_match() {
        let destination = "A".repeat(520);

        let error = I2pDestination::from_str(&destination).unwrap_err();

        assert_eq!(
            error,
            ParseI2pDestinationError::InvalidCertificateLength { declared: 0, actual: 3 }
        );
    }
}
