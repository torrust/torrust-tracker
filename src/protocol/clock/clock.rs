use std::num::IntErrorKind;
pub use std::time::Duration;

pub type DurationSinceUnixEpoch = Duration;

#[derive(Debug)]
pub enum ClockType {
    WorkingClock,
    StoppedClock,
}

#[derive(Debug)]
pub struct Clock<const T: usize>;

pub type WorkingClock = Clock<{ ClockType::WorkingClock as usize }>;
pub type StoppedClock = Clock<{ ClockType::StoppedClock as usize }>;

#[cfg(not(test))]
pub type DefaultClock = WorkingClock;

#[cfg(test)]
pub type DefaultClock = StoppedClock;

pub trait Time: Sized {
    fn now() -> DurationSinceUnixEpoch;
}

pub trait TimeNow: Time {
    fn add(add_time: &Duration) -> Option<DurationSinceUnixEpoch> {
        Self::now().checked_add(*add_time)
    }
    fn sub(sub_time: &Duration) -> Option<DurationSinceUnixEpoch> {
        Self::now().checked_sub(*sub_time)
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use crate::protocol::clock::clock::{DefaultClock, StoppedClock, Time, WorkingClock};

    #[test]
    fn it_should_be_the_stopped_clock_as_default_when_testing() {
        // We are testing, so we should default to the fixed time.
        assert_eq!(TypeId::of::<StoppedClock>(), TypeId::of::<DefaultClock>());
        assert_eq!(StoppedClock::now(), DefaultClock::now())
    }

    #[test]
    fn it_should_have_different_times() {
        assert_ne!(TypeId::of::<StoppedClock>(), TypeId::of::<WorkingClock>());
        assert_ne!(StoppedClock::now(), WorkingClock::now())
    }
}

mod working_clock {
    use std::time::SystemTime;

    use super::{DurationSinceUnixEpoch, Time, TimeNow, WorkingClock};

    impl Time for WorkingClock {
        fn now() -> DurationSinceUnixEpoch {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
        }
    }

    impl TimeNow for WorkingClock {}
}

pub trait StoppedTime: TimeNow {
    fn local_set(unix_time: &DurationSinceUnixEpoch);
    fn local_set_to_unix_epoch() {
        Self::local_set(&DurationSinceUnixEpoch::ZERO)
    }
    fn local_set_to_app_start_time();
    fn local_set_to_system_time_now();
    fn local_add(duration: &Duration) -> Result<(), IntErrorKind>;
    fn local_sub(duration: &Duration) -> Result<(), IntErrorKind>;
    fn local_reset();
}

mod stopped_clock {
    use std::num::IntErrorKind;
    use std::time::Duration;

    use super::{DurationSinceUnixEpoch, StoppedClock, StoppedTime, Time, TimeNow};

    impl Time for StoppedClock {
        fn now() -> DurationSinceUnixEpoch {
            detail::FIXED_TIME.with(|time| {
                return *time.borrow();
            })
        }
    }

    impl TimeNow for StoppedClock {}

    impl StoppedTime for StoppedClock {
        fn local_set(unix_time: &DurationSinceUnixEpoch) {
            detail::FIXED_TIME.with(|time| {
                *time.borrow_mut() = *unix_time;
            })
        }

        fn local_set_to_app_start_time() {
            Self::local_set(&detail::get_app_start_time())
        }

        fn local_set_to_system_time_now() {
            Self::local_set(&detail::get_app_start_time())
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
            Self::local_set(&detail::get_default_fixed_time())
        }
    }

    #[cfg(test)]
    mod tests {
        use std::thread;
        use std::time::Duration;

        use crate::protocol::clock::clock::{DurationSinceUnixEpoch, StoppedClock, StoppedTime, Time, TimeNow, WorkingClock};

        #[test]
        fn it_should_default_to_zero_when_testing() {
            assert_eq!(StoppedClock::now(), DurationSinceUnixEpoch::ZERO)
        }

        #[test]
        fn it_should_possible_to_set_the_time() {
            // Check we start with ZERO.
            assert_eq!(StoppedClock::now(), Duration::ZERO);

            // Set to Current Time and Check
            let timestamp = WorkingClock::now();
            StoppedClock::local_set(&timestamp);
            assert_eq!(StoppedClock::now(), timestamp);

            // Elapse the Current Time and Check
            StoppedClock::local_add(&timestamp).unwrap();
            assert_eq!(StoppedClock::now(), timestamp + timestamp);

            // Reset to ZERO and Check
            StoppedClock::local_reset();
            assert_eq!(StoppedClock::now(), Duration::ZERO);
        }

        #[test]
        fn it_should_default_to_zero_on_thread_exit() {
            assert_eq!(StoppedClock::now(), Duration::ZERO);
            let after5 = WorkingClock::add(&Duration::from_secs(5)).unwrap();
            StoppedClock::local_set(&after5);
            assert_eq!(StoppedClock::now(), after5);

            let t = thread::spawn(move || {
                // each thread starts out with the initial value of ZERO
                assert_eq!(StoppedClock::now(), Duration::ZERO);

                // and gets set to the current time.
                let timestamp = WorkingClock::now();
                StoppedClock::local_set(&timestamp);
                assert_eq!(StoppedClock::now(), timestamp);
            });

            // wait for the thread to complete and bail out on panic
            t.join().unwrap();

            // we retain our original value of current time + 5sec despite the child thread
            assert_eq!(StoppedClock::now(), after5);

            // Reset to ZERO and Check
            StoppedClock::local_reset();
            assert_eq!(StoppedClock::now(), Duration::ZERO);
        }
    }

    mod detail {
        use std::cell::RefCell;
        use std::time::SystemTime;

        use crate::protocol::clock::clock::DurationSinceUnixEpoch;
        use crate::static_time;

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

        thread_local!(pub static FIXED_TIME: RefCell<DurationSinceUnixEpoch>   = RefCell::new(get_default_fixed_time()));

        #[cfg(test)]
        mod tests {
            use std::time::Duration;

            use crate::protocol::clock::clock::stopped_clock::detail::{get_app_start_time, get_default_fixed_time};

            #[test]
            fn it_should_get_the_zero_start_time_when_testing() {
                assert_eq!(get_default_fixed_time(), Duration::ZERO);
            }

            #[test]
            fn it_should_get_app_start_time() {
                const TIME_AT_WRITING_THIS_TEST: Duration = Duration::new(1662983731, 000022312);
                assert!(get_app_start_time() > TIME_AT_WRITING_THIS_TEST);
            }
        }
    }
}
