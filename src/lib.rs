pub use api::server::*;
pub use http::server::*;
pub use protocol::common::*;
pub use udp::server::*;

pub use self::config::*;
pub use self::tracker::*;

pub mod api;
pub mod config;
pub mod databases;
pub mod http;
pub mod jobs;
pub mod logging;
pub mod protocol;
pub mod setup;
pub mod tracker;
pub mod udp;

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

pub mod block_ciphers {

    use blowfish::BlowfishLE;
    use cipher::KeyInit;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    pub type Cipher = BlowfishLE;

    pub mod ephemeral_instance {
        use super::*;
        use crate::ephemeral_instance_keys::RANDOM_SEED;

        lazy_static! {
            pub static ref BLOCK_CIPHER_BLOWFISH: Cipher = <BlowfishLE as KeyInit>::new(&<BlowfishLE as KeyInit>::generate_key(
                <StdRng as SeedableRng>::from_seed(*RANDOM_SEED)
            ));
        }
    }

    pub mod testing {
        use super::*;

        lazy_static! {
            pub static ref TEST_BLOCK_CIPHER_BLOWFISH: Cipher = <BlowfishLE as KeyInit>::new(
                &<BlowfishLE as KeyInit>::generate_key(<StdRng as SeedableRng>::from_seed([0u8; 32]))
            );
        }
    }
}
