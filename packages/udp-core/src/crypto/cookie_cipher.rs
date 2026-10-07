//! The secret key that encrypts UDP connection IDs (cookies).
//!
//! Anyone who knows this key can forge a valid connection ID for any client
//! address, so the key is created once per composition root from a
//! cryptographically secure random number generator and injected into every
//! component that issues or validates connection IDs. See the ADR
//! `docs/adrs/20261007085634_inject_the_udp_connection_cookie_cipher.md`.
use std::fmt;

use blowfish::BlowfishLE;
use cipher::{Block, BlockCipherDecrypt, BlockCipherEncrypt, KeyInit};
use rand::Rng;
use zeroize::Zeroizing;

/// One cipher block: the 8 bytes of a connection ID.
pub type CookieBlock = Block<BlowfishLE>;

/// Length in bytes of the key material used to build a cipher.
const KEY_LEN: usize = 32;

/// The cipher that encrypts and decrypts UDP connection IDs.
///
/// It holds the expanded Blowfish key schedule, so it is built once and
/// shared (for example in an `Arc`), never rebuilt per request. It exposes no
/// key material, its `Debug` output is redacted, and the key schedule is
/// wiped when it is dropped. Wiping is best effort: it cannot reach copies the
/// compiler or the operating system may have made elsewhere.
///
/// Production code can only create a random key:
///
/// ```rust
/// use torrust_tracker_udp_core::crypto::cookie_cipher::CookieCipher;
///
/// let _production_key = CookieCipher::random();
/// ```
///
/// The fixed key used by this crate's unit tests does not exist in any other
/// build, so production code cannot select it:
///
/// ```rust,compile_fail,E0599
/// use torrust_tracker_udp_core::crypto::cookie_cipher::CookieCipher;
///
/// let _test_key = CookieCipher::fixed_for_testing();
/// ```
pub struct CookieCipher(BlowfishLE);

impl CookieCipher {
    /// Creates a cipher from a key drawn from `rand`'s thread-local
    /// cryptographically secure random number generator.
    ///
    /// The raw key bytes are filled in place and wiped when this function
    /// returns, so only the expanded key schedule, which is wiped on drop,
    /// keeps the key.
    #[must_use]
    pub fn random() -> Self {
        let mut key = Zeroizing::new([0; KEY_LEN]);
        rand::rng().fill(&mut *key);
        Self::from_key(&key)
    }

    /// Creates the fixed cipher used by this crate's unit tests, keyed with
    /// zero bytes, so their connection IDs are reproducible.
    #[cfg(test)]
    pub(crate) fn fixed_for_testing() -> Self {
        Self::from_key(&[0; KEY_LEN])
    }

    fn from_key(key: &[u8; KEY_LEN]) -> Self {
        Self(BlowfishLE::new_from_slice(key).expect("a 32-byte key should be a valid Blowfish key length"))
    }

    pub(crate) fn encrypt(&self, block: &mut CookieBlock) {
        self.0.encrypt_block(block);
    }

    pub(crate) fn decrypt(&self, block: &mut CookieBlock) {
        self.0.decrypt_block(block);
    }
}

impl fmt::Debug for CookieCipher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CookieCipher([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::{CookieBlock, CookieCipher, KEY_LEN};

    fn encrypted(cipher: &CookieCipher, plain_text: [u8; 8]) -> CookieBlock {
        let mut block = CookieBlock::from(plain_text);
        cipher.encrypt(&mut block);
        block
    }

    #[test]
    fn it_should_not_encrypt_like_the_fixed_test_key_when_the_key_is_random() {
        // Arrange
        let plain_text = [0; 8];
        let fixed_test_key = CookieCipher::fixed_for_testing();

        // Act
        let random_key = CookieCipher::random();

        // Assert
        assert_ne!(
            encrypted(&random_key, plain_text),
            encrypted(&fixed_test_key, plain_text),
            "a random key encrypted the zero block exactly like the fixed all-zero test key"
        );
    }

    #[test]
    fn it_should_create_a_different_key_each_time_when_the_key_is_random() {
        // Arrange
        let plain_text = [0; 8];

        // Act
        let first_key = CookieCipher::random();
        let second_key = CookieCipher::random();

        // Assert
        assert_ne!(
            encrypted(&first_key, plain_text),
            encrypted(&second_key, plain_text),
            "two random keys encrypted the zero block identically"
        );
    }

    #[test]
    fn it_should_redact_the_key_when_formatted_for_debugging() {
        // Arrange
        let unique_key_bytes = [0x5a; KEY_LEN];
        let cipher = CookieCipher::from_key(&unique_key_bytes);

        // Act
        let debug_output = format!("{cipher:?}");

        // Assert
        assert_eq!(debug_output, "CookieCipher([REDACTED])");
        assert!(
            !debug_output.contains(&format!("{unique_key_bytes:?}")) && !debug_output.contains("5a"),
            "the debug output `{debug_output}` contains the key bytes"
        );
    }
}
