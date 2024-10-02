/// Trait for types that can be used as a timestamp clock stopped
/// at a given time.
#[allow(clippy::module_name_repetitions)]
pub struct StoppedClock {}

#[allow(clippy::module_name_repetitions)]
pub trait Stopped: clock::Time {
    /// It sets the clock to a given time.
    fn local_set(unix_time: &DurationSinceUnixEpoch);

    /// It sets the clock to the Unix Epoch.
    fn local_set_to_unix_epoch() {
        Self::local_set(&DurationSinceUnixEpoch::ZERO);
    }

    /// It sets the clock to the time the application started.
    fn local_set_to_app_start_time();

    /// It sets the clock to the current system time.
    fn local_set_to_system_time_now();

    /// It adds a `Duration` to the clock.
    ///
    /// # Errors
    ///
    /// Will return `IntErrorKind` if `duration` would overflow the internal `Duration`.
    fn local_add(duration: &Duration) -> Result<(), IntErrorKind>;

    /// It subtracts a `Duration` from the clock.
    /// # Errors
    ///
    /// Will return `IntErrorKind` if `duration` would underflow the internal `Duration`.
    fn local_sub(duration: &Duration) -> Result<(), IntErrorKind>;

    /// It resets the clock to default fixed time that is application start time (or the unix epoch when testing).
    fn local_reset();
}

use std::num::IntErrorKind;
use std::time::Duration;

use super::{DurationSinceUnixEpoch, Time};
use crate::clock;

impl Time for clock::Stopped {
    fn now() -> DurationSinceUnixEpoch {
        detail::FIXED_TIME.with(|time| {
            return *time.borrow();
        })
    }

    fn dbg_clock_type() -> String {
        "Stopped".to_owned()
    }
}

impl Stopped for clock::Stopped {
    fn local_set(unix_time: &DurationSinceUnixEpoch) {
        detail::FIXED_TIME.with(|time| {
            *time.borrow_mut() = *unix_time;
        });
    }

    fn local_set_to_app_start_time() {
        Self::local_set(&detail::get_app_start_time());
    }

    fn local_set_to_system_time_now() {
        Self::local_set(&detail::get_app_start_time());
    }

    fn local_add(duration: &Duration) -> Result<(), IntErrorKind> {
        detail::FIXED_TIME.with(|time| {
            let time_borrowed = *time.borrow();
            *time.borrow_mut() = match time_borrowed.checked_add(*duration) {
                Some(time) => time,
                None => {
                    return Err(IntErrorKind::PosOverflow);
                }
            };
            Ok(())
        })
    }

    fn local_sub(duration: &Duration) -> Result<(), IntErrorKind> {
        detail::FIXED_TIME.with(|time| {
            let time_borrowed = *time.borrow();
            *time.borrow_mut() = match time_borrowed.checked_sub(*duration) {
                Some(time) => time,
                None => {
                    return Err(IntErrorKind::NegOverflow);
                }
            };
            Ok(())
        })
    }

    fn local_reset() {
        Self::local_set(&detail::get_default_fixed_time());
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    use crate::clock::stopped::Stopped as _;
    use crate::clock::{Stopped, Time, Working};

    #[test]
    fn it_should_default_to_zero_when_testing() {
        assert_eq!(Stopped::now(), DurationSinceUnixEpoch::ZERO);
    }

    #[test]
    fn it_should_possible_to_set_the_time() {
        // Check we start with ZERO.
        assert_eq!(Stopped::now(), Duration::ZERO);

        // Set to Current Time and Check
        let timestamp = Working::now();
        Stopped::local_set(&timestamp);
        assert_eq!(Stopped::now(), timestamp);

        // Elapse the Current Time and Check
        Stopped::local_add(&timestamp).unwrap();
        assert_eq!(Stopped::now(), timestamp + timestamp);

        // Reset to ZERO and Check
        Stopped::local_reset();
        assert_eq!(Stopped::now(), Duration::ZERO);
    }

    #[test]
    fn it_should_default_to_zero_on_thread_exit() {
        assert_eq!(Stopped::now(), Duration::ZERO);
        let after5 = Working::now_add(&Duration::from_secs(5)).unwrap();
        Stopped::local_set(&after5);
        assert_eq!(Stopped::now(), after5);

        let t = thread::spawn(move || {
            // each thread starts out with the initial value of ZERO
            assert_eq!(Stopped::now(), Duration::ZERO);

            // and gets set to the current time.
            let timestamp = Working::now();
            Stopped::local_set(&timestamp);
            assert_eq!(Stopped::now(), timestamp);
        });

        // wait for the thread to complete and bail out on panic
        t.join().unwrap();

        // we retain our original value of current time + 5sec despite the child thread
        assert_eq!(Stopped::now(), after5);

        // Reset to ZERO and Check
        Stopped::local_reset();
        assert_eq!(Stopped::now(), Duration::ZERO);
    }
}

mod detail {
    use std::cell::RefCell;
    use std::time::SystemTime;

    use torrust_tracker_primitives::DurationSinceUnixEpoch;

    use crate::static_time;

    thread_local!(pub static FIXED_TIME: RefCell<DurationSinceUnixEpoch>   = RefCell::new(get_default_fixed_time()));

    pub fn get_app_start_time() -> DurationSinceUnixEpoch {
        (*static_time::TIME_AT_APP_START)
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
    }

    #[cfg(not(test))]
    pub fn get_default_fixed_time() -> DurationSinceUnixEpoch {
        get_app_start_time()
    }

    #[cfg(test)]
    pub fn get_default_fixed_time() -> DurationSinceUnixEpoch {
        DurationSinceUnixEpoch::ZERO
    }

    #[cfg(test)]
    mod tests {
        use std::time::Duration;

        use crate::clock::stopped::detail::{get_app_start_time, get_default_fixed_time};

        #[test]
        fn it_should_get_the_zero_start_time_when_testing() {
            assert_eq!(get_default_fixed_time(), Duration::ZERO);
        }

        #[test]
        fn it_should_get_app_start_time() {
            const TIME_AT_WRITING_THIS_TEST: Duration = Duration::new(1_662_983_731, 22312);
            assert!(get_app_start_time() > TIME_AT_WRITING_THIS_TEST);
        }
    }
}
