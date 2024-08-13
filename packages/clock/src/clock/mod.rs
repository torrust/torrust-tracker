use std::time::Duration;

use torrust_tracker_primitives::DurationSinceUnixEpoch;

use self::stopped::StoppedClock;
use self::working::WorkingClock;

pub mod stopped;
pub mod working;

/// A generic structure that represents a clock.
///
/// It can be either the working clock (production) or the stopped clock
/// (testing). It implements the `Time` trait, which gives you the current time.
#[derive(Debug)]
pub struct Clock<T> {
    clock: std::marker::PhantomData<T>,
}

/// The working clock. It returns the current time.
pub type Working = Clock<WorkingClock>;
/// The stopped clock. It returns always the same fixed time.
pub type Stopped = Clock<StoppedClock>;

/// Trait for types that can be used as a timestamp clock.
pub trait Time: Sized {
    fn now() -> DurationSinceUnixEpoch;

    fn dbg_clock_type() -> String;

    #[must_use]
    fn now_add(add_time: &Duration) -> Option<DurationSinceUnixEpoch> {
        Self::now().checked_add(*add_time)
    }
    #[must_use]
    fn now_sub(sub_time: &Duration) -> Option<DurationSinceUnixEpoch> {
        Self::now().checked_sub(*sub_time)
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::time::Duration;

    use crate::clock::{self, Stopped, Time, Working};
    use crate::CurrentClock;

    #[test]
    fn it_should_be_the_stopped_clock_as_default_when_testing() {
        // We are testing, so we should default to the fixed time.
        assert_eq!(TypeId::of::<Stopped>(), TypeId::of::<CurrentClock>());
        assert_eq!(Stopped::now(), CurrentClock::now());
    }

    #[test]
    fn it_should_have_different_times() {
        assert_ne!(TypeId::of::<clock::Stopped>(), TypeId::of::<Working>());
        assert_ne!(Stopped::now(), Working::now());
    }

    #[test]
    fn it_should_use_stopped_time_for_testing() {
        assert_eq!(CurrentClock::dbg_clock_type(), "Stopped".to_owned());

        let time = CurrentClock::now();
        std::thread::sleep(Duration::from_millis(50));
        let time_2 = CurrentClock::now();

        assert_eq!(time, time_2);
    }
}
