use std::time::Duration;

use torrust_tracker_clock::clock::Time;
use tracing::level_filters::LevelFilter;

use crate::common::logging::{tracing_stderr_init, INIT};
use crate::CurrentClock;

#[test]
fn it_should_use_stopped_time_for_testing() {
    INIT.call_once(|| {
        tracing_stderr_init(LevelFilter::ERROR);
    });

    assert_eq!(CurrentClock::dbg_clock_type(), "Stopped".to_owned());

    let time = CurrentClock::now();
    std::thread::sleep(Duration::from_millis(50));
    let time_2 = CurrentClock::now();

    assert_eq!(time, time_2);
}
