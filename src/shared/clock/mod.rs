//! Time related functions and types.
//!
//! It's usually a good idea to control where the time comes from
//! in an application so that it can be mocked for testing and it can be
//! controlled in production so we get the intended behavior without
//! relying on the specific time zone for the underlying system.
//!
//! Clocks use the type `DurationSinceUnixEpoch` which is a
//! `std::time::Duration` since the Unix Epoch (timestamp).
//!
//! ```text
//! Local time:     lun 2023-03-27 16:12:00 WEST
//! Universal time: lun 2023-03-27 15:12:00 UTC
//! Time zone:      Atlantic/Canary (WEST, +0100)
//! Timestamp:      1679929914
//! Duration:       1679929914.10167426
//! ```
//!
//! > **NOTICE**: internally the `Duration` is stores it's main unit as seconds in a `u64` and it will
//! overflow in 584.9 billion years.
//!
//! > **NOTICE**: the timestamp does not depend on the time zone. That gives you
//! the ability to use the clock regardless of the underlying system time zone
//! configuration. See [Unix time Wikipedia entry](https://en.wikipedia.org/wiki/Unix_time).
pub mod static_time;
pub mod time_extent;
pub mod utils;

use std::num::IntErrorKind;
use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, NaiveDateTime, Utc};

/// Duration since the Unix Epoch.
pub type DurationSinceUnixEpoch = Duration;

/// Clock types.
#[derive(Debug)]
pub enum Type {
    /// Clock that returns the current time.
    WorkingClock,
    /// Clock that returns always the same fixed time.
    StoppedClock,
}

/// A generic structure that represents a clock.
///
/// It can be either the working clock (production) or the stopped clock
/// (testing). It implements the `Time` trait, which gives you the current time.
#[derive(Debug)]
pub struct Clock<const T: usize>;

/// The working clock. It returns the current time.
pub type Working = Clock<{ Type::WorkingClock as usize }>;
/// The stopped clock. It returns always the same fixed time.
pub type Stopped = Clock<{ Type::StoppedClock as usize }>;

/// The current clock. Defined at compilation time.
/// It can be either the working clock (production) or the stopped clock (testing).
#[cfg(not(test))]
pub type Current = Working;

/// The current clock. Defined at compilation time.
/// It can be either the working clock (production) or the stopped clock (testing).
#[cfg(test)]
pub type Current = Stopped;

/// Trait for types that can be used as a timestamp clock.
pub trait Time: Sized {
    fn now() -> DurationSinceUnixEpoch;
}

/// Trait for types that can be manipulate the current time in order to
/// get time in the future or in the past after or before a duration of time.
pub trait TimeNow: Time {
    #[must_use]
    fn add(add_time: &Duration) -> Option<DurationSinceUnixEpoch> {
        Self::now().checked_add(*add_time)
    }
    #[must_use]
    fn sub(sub_time: &Duration) -> Option<DurationSinceUnixEpoch> {
        Self::now().checked_sub(*sub_time)
    }
}

/// It converts a string in ISO 8601 format to a timestamp.
/// For example, the string `1970-01-01T00:00:00.000Z` which is the Unix Epoch
/// will be converted to a timestamp of 0: `DurationSinceUnixEpoch::ZERO`.
///
/// # Panics
///
/// Will panic if the input time cannot be converted to `DateTime::<Utc>`, internally using the `i64` type.
/// (this will naturally happen in 292.5 billion years)
#[must_use]
pub fn convert_from_iso_8601_to_timestamp(iso_8601: &str) -> DurationSinceUnixEpoch {
    convert_from_datetime_utc_to_timestamp(&DateTime::<Utc>::from_str(iso_8601).unwrap())
}

/// It converts a `DateTime::<Utc>` to a timestamp.
/// For example, the `DateTime::<Utc>` of the Unix Epoch will be converted to a
/// timestamp of 0: `DurationSinceUnixEpoch::ZERO`.
///
/// # Panics
///
/// Will panic if the input time overflows the `u64` type.
/// (this will naturally happen in 584.9 billion years)
#[must_use]
pub fn convert_from_datetime_utc_to_timestamp(datetime_utc: &DateTime<Utc>) -> DurationSinceUnixEpoch {
    DurationSinceUnixEpoch::from_secs(u64::try_from(datetime_utc.timestamp()).expect("Overflow of u64 seconds, very future!"))
}

/// It converts a timestamp to a `DateTime::<Utc>`.
/// For example, the timestamp of 0: `DurationSinceUnixEpoch::ZERO` will be
/// converted to the `DateTime::<Utc>` of the Unix Epoch.
///
/// # Panics
///
/// Will panic if the input time overflows the `u64` seconds overflows the `i64` type.
/// (this will naturally happen in 292.5 billion years)
#[must_use]
pub fn convert_from_timestamp_to_datetime_utc(duration: DurationSinceUnixEpoch) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(
            i64::try_from(duration.as_secs()).expect("Overflow of i64 seconds, very future!"),
            duration.subsec_nanos(),
        )
        .unwrap(),
        Utc,
    )
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use crate::shared::clock::{Current, Stopped, Time, Working};

    #[test]
    fn it_should_be_the_stopped_clock_as_default_when_testing() {
        // We are testing, so we should default to the fixed time.
        assert_eq!(TypeId::of::<Stopped>(), TypeId::of::<Current>());
        assert_eq!(Stopped::now(), Current::now());
    }

    #[test]
    fn it_should_have_different_times() {
        assert_ne!(TypeId::of::<Stopped>(), TypeId::of::<Working>());
        assert_ne!(Stopped::now(), Working::now());
    }

    mod timestamp {
        use chrono::{DateTime, NaiveDateTime, Utc};

        use crate::shared::clock::{
            convert_from_datetime_utc_to_timestamp, convert_from_iso_8601_to_timestamp, convert_from_timestamp_to_datetime_utc,
            DurationSinceUnixEpoch,
        };

        #[test]
        fn should_be_converted_to_datetime_utc() {
            let timestamp = DurationSinceUnixEpoch::ZERO;
            assert_eq!(
                convert_from_timestamp_to_datetime_utc(timestamp),
                DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), Utc)
            );
        }

        #[test]
        fn should_be_converted_from_datetime_utc() {
            let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(0, 0).unwrap(), Utc);
            assert_eq!(
                convert_from_datetime_utc_to_timestamp(&datetime),
                DurationSinceUnixEpoch::ZERO
            );
        }

        #[test]
        fn should_be_converted_from_datetime_utc_in_iso_8601() {
            let iso_8601 = "1970-01-01T00:00:00.000Z".to_string();
            assert_eq!(convert_from_iso_8601_to_timestamp(&iso_8601), DurationSinceUnixEpoch::ZERO);
        }
    }
}

mod working_clock {
    use std::time::SystemTime;

    use super::{DurationSinceUnixEpoch, Time, TimeNow, Working};

    impl Time for Working {
        fn now() -> DurationSinceUnixEpoch {
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
        }
    }

    impl TimeNow for Working {}
}

/// Trait for types that can be used as a timestamp clock stopped
/// at a given time.
pub trait StoppedTime: TimeNow {
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

mod stopped_clock {
    use std::num::IntErrorKind;
    use std::time::Duration;

    use super::{DurationSinceUnixEpoch, Stopped, StoppedTime, Time, TimeNow};

    impl Time for Stopped {
        fn now() -> DurationSinceUnixEpoch {
            detail::FIXED_TIME.with(|time| {
                return *time.borrow();
            })
        }
    }

    impl TimeNow for Stopped {}

    impl StoppedTime for Stopped {
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

        use crate::shared::clock::{DurationSinceUnixEpoch, Stopped, StoppedTime, Time, TimeNow, Working};

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
            let after5 = Working::add(&Duration::from_secs(5)).unwrap();
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

        use crate::shared::clock::{static_time, DurationSinceUnixEpoch};

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

            use crate::shared::clock::stopped_clock::detail::{get_app_start_time, get_default_fixed_time};

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
}
