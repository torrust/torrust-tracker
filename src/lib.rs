pub mod databases;
pub mod jobs;
pub mod logging;
pub mod protocol;
pub mod servers;
pub mod setup;
pub mod signals;
pub mod stats;
pub mod tracker;

#[macro_use]
extern crate lazy_static;

pub mod static_time {
    use std::time::SystemTime;

    lazy_static! {
        pub static ref TIME_AT_APP_START: SystemTime = SystemTime::now();
    }
}

pub mod ephemeral_instance_keys {
    use rand::rngs::ThreadRng;
    use rand::Rng;

    pub type Seed = [u8; 32];

    lazy_static! {
        pub static ref RANDOM_SEED: Seed = Rng::gen(&mut ThreadRng::default());
    }
}
