//! It contains a static variable that is set to the time at which
//! the application started.
use std::sync::LazyLock;
use std::time::SystemTime;

/// The time at which the application started.
pub static TIME_AT_APP_START: LazyLock<SystemTime> = LazyLock::new(SystemTime::now);
