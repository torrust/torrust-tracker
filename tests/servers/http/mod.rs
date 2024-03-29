pub mod asserts;
pub mod client;
pub mod environment;
pub mod v1;

use torrust_tracker::servers::http::server;

pub type Started = environment::Environment<server::Running>;

//pub(crate) const TIMEOUT: Duration = Duration::from_secs(5);
