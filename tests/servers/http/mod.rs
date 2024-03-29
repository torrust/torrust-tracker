pub mod asserts;
pub mod environment;
pub mod v1;

use std::time::Duration;

use torrust_tracker::servers::http::server;

pub type Started = environment::Environment<server::Running>;

pub(crate) const TIMEOUT: Duration = Duration::from_secs(5);
