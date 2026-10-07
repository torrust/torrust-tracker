//! This module contains the ephemeral instance keys used by the application.
//!
//! They are ephemeral because they are generated at runtime when the
//! application starts and are not persisted anywhere.

use std::sync::LazyLock;

use rand::Rng;
use rand::rngs::ThreadRng;

use crate::crypto::cookie_cipher::CookieCipher;

pub type Seed = [u8; 32];

/// The random static seed.
pub static RANDOM_SEED: LazyLock<Seed> = LazyLock::new(|| {
    let mut rng = ThreadRng::default();
    rng.random::<Seed>()
});

/// The process-wide connection-cookie cipher, with a random key.
///
/// Transitional: issue #2458 replaces this static with one cipher injected
/// from the composition root, and then removes it.
pub static RANDOM_CIPHER_BLOWFISH: LazyLock<CookieCipher> = LazyLock::new(CookieCipher::random);
