//! It contains a static variable that is set to the time at which
//! the application started.
use std::time::SystemTime;

lazy_static! {
    /// The time at which the application started.
    pub static ref TIME_AT_APP_START: SystemTime = SystemTime::now();
}
