pub mod apis;
pub mod config;
pub mod databases;
pub mod errors;
pub mod helpers;
pub mod http;
pub mod jobs;
pub mod located_error;
pub mod logging;
pub mod protocol;
pub mod settings;
pub mod setup;
pub mod stats;
pub mod tracker;
pub mod udp;

pub mod config_const {
    pub const CONFIG_FOLDER: &str = "config";
    pub const CONFIG_BACKUP_FOLDER: &str = "config.backup";
    pub const CONFIG_ERROR_FOLDER: &str = "config.error";
    pub const CONFIG_DEFAULT: &str = "default";
    pub const CONFIG_LOCAL: &str = "local";
    pub const CONFIG_OVERRIDE: &str = "override";
    pub const CONFIG_OLD: &str = "../config";
}

#[macro_use]
extern crate lazy_static;
extern crate derive_builder;

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

pub trait Empty: Sized {
    fn empty() -> Self;
}
