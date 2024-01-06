use torrust_tracker::servers::udp::server;

pub mod asserts;
pub mod contract;
pub mod environment;

pub type Started = environment::Environment<server::Running>;
