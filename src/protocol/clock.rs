use std::time::{SystemTime};

pub trait Clock {
    fn now_as_timestamp(&self) -> u64;
}

/// A [`Clock`] which uses the operating system to determine the time.
struct SystemClock;

impl Clock for SystemClock {
    fn now_as_timestamp(&self) -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }
}

/// It returns the current timestamp using the system clock.
pub fn current_timestamp() -> u64 {
    let system_clock = SystemClock;
    system_clock.now_as_timestamp()
}
