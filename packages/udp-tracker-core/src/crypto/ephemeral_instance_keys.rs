//! This module contains the ephemeral instance keys used by the application.
//!
//! They are ephemeral because they are generated at runtime when the
//! application starts and are not persisted anywhere.

use blowfish::BlowfishLE;
use cipher::{Block, KeyInit};
use rand::rngs::ThreadRng;
use rand::RngExt;

pub type Seed = [u8; 32];
pub type CipherBlowfish = BlowfishLE;
pub type CipherArrayBlowfish = Block<CipherBlowfish>;

lazy_static! {
    /// The random static seed.
    pub static ref RANDOM_SEED: Seed = {
        let mut rng = ThreadRng::default();
        rng.random::<Seed>()
    };

    /// The random cipher from the seed.
    pub static ref RANDOM_CIPHER_BLOWFISH: CipherBlowfish = {
        let mut rng = ThreadRng::default();
        let seed: Seed = rng.random();
        CipherBlowfish::new_from_slice(&seed).expect("it could not generate key")
    };

    /// The constant cipher for testing.
    pub static ref ZEROED_TEST_CIPHER_BLOWFISH: CipherBlowfish = CipherBlowfish::new_from_slice(&[0u8; 32]).expect("it could not generate key");
}
