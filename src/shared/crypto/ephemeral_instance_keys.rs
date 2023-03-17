use rand::rngs::ThreadRng;
use rand::Rng;

pub type Seed = [u8; 32];

lazy_static! {
    pub static ref RANDOM_SEED: Seed = Rng::gen(&mut ThreadRng::default());
}
