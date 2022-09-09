use std::time::SystemTime;

pub type UnixTime = u64;

/// A Clock which uses the UNIX time.
pub trait UnixClock {
    fn now(&self) -> UnixTime;
}

/// A Clock which uses the operating system to determine the time.
pub struct SystemUnixClock;

impl UnixClock for SystemUnixClock {
    fn now(&self) -> UnixTime {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }
}

/// It returns the current timestamp using the system clock.
pub fn current_timestamp() -> UnixTime {
    SystemUnixClock.now()
}
