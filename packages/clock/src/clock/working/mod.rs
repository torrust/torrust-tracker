use std::time::SystemTime;

use crate::{DurationSinceUnixEpoch, clock};

#[allow(clippy::module_name_repetitions)]
pub struct WorkingClock;

impl clock::Time for clock::Working {
    fn now() -> DurationSinceUnixEpoch {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
    }

    fn dbg_clock_type() -> String {
        "Working".to_owned()
    }
}
