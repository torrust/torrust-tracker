//! This module contains the ephemeral instance keys used by the application.
//!
//! They are ephemeral because they are generated at runtime when the
//! application starts and are not persisted anywhere.
use rand::rngs::ThreadRng;
use rand::Rng;

pub type Seed = [u8; 32];

lazy_static! {
    /// The random static seed.
    pub static ref RANDOM_SEED: Seed = Rng::gen(&mut ThreadRng::default());
}
