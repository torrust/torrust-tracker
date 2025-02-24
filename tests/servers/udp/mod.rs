pub mod asserts;
pub mod contract;
pub mod environment;

use torrust_udp_tracker_server::server::states::Running;

pub type Started = environment::Environment<Running>;
