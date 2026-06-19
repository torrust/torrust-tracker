//! This module contains the ephemeral instance keys used by the application.
//!
//! They are ephemeral because they are generated at runtime when the
//! application starts and are not persisted anywhere.

use std::sync::LazyLock;

use blowfish::BlowfishLE;
use cipher::{Block, KeyInit};
use rand::Rng;
use rand::rngs::ThreadRng;

pub type Seed = [u8; 32];
pub type CipherBlowfish = BlowfishLE;
pub type CipherArrayBlowfish = Block<CipherBlowfish>;

/// The random static seed.
pub static RANDOM_SEED: LazyLock<Seed> = LazyLock::new(|| {
    let mut rng = ThreadRng::default();
    rng.random::<Seed>()
});

/// The random cipher from the seed.
pub static RANDOM_CIPHER_BLOWFISH: LazyLock<CipherBlowfish> = LazyLock::new(|| {
    let mut rng = ThreadRng::default();
    let seed: Seed = rng.random();
    CipherBlowfish::new_from_slice(&seed).expect("it could not generate key")
});

/// The constant cipher for testing.
pub static ZEROED_TEST_CIPHER_BLOWFISH: LazyLock<CipherBlowfish> =
    LazyLock::new(|| CipherBlowfish::new_from_slice(&[0u8; 32]).expect("it could not generate key"));
