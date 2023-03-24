//! It includes functionality to handle time extents.
//!
//! Time extents are used to represent a duration of time which contains
//! N times intervals of the same duration.
//!
//! Given a duration of: 60 seconds.
//!
//! ```text
//! |------------------------------------------------------------|
//! ```
//!
//! If we define a **base** duration of `10` seconds, we would have `6` intervals.
//!
//! ```text
//! |----------|----------|----------|----------|----------|----------|
//!            ^--- 10 seconds
//! ```
//!
//! Then, You can represent half of the duration (`30` seconds) as:
//!
//! ```text
//! |----------|----------|----------|----------|----------|----------|
//!                                  ^--- 30 seconds
//! ```
//!
//! `3` times (**multiplier**) the **base** interval (3*10 = 30 seconds):
//!
//! ```text
//! |----------|----------|----------|----------|----------|----------|
//!                                  ^--- 30 seconds (3 units of 10 seconds)
//! ```
//!
//! Time extents are a way to measure time duration using only one unit of time
//! (**base** duration) repeated `N` times (**multiplier**).
//!
//! Time extents are not clocks in a sense that they do not have a start time.
//! They are not synchronized with the real time. In order to measure time,
//! you need to define a start time for the intervals.
//!
//! For example, we could measure time is "lustrums" (5 years) since the start
//! of the 21st century. The time extent would contains a base 5-year duration
//! and the multiplier. The current "lustrum" (2023) would be 5th one if we
//! start counting "lustrums" at 1.
//!
//! ```text
//! Lustrum 1: 2000-2004
//! Lustrum 2: 2005-2009
//! Lustrum 3: 2010-2014
//! Lustrum 4: 2015-2019
//! Lustrum 5: 2020-2024
//! ```
//!
//! More practically time extents are used to represent number of time intervals
//! since the Unix Epoch. Each interval is typically an amount of seconds.
//! It's specially useful to check expiring dates. For example, you can have an
//! authentication token that expires after 120 seconds. If you divide the
//! current timestamp by 120 you get the number of 2-minute intervals since the
//! Unix Epoch, you can hash that value with a secret key and send it to a
//! client. The client can authenticate by sending the hashed value back to the
//! server. The server can build the same hash and compare it with the one sent
//! by the client. The hash would be the same during the 2-minute interval, but
//! it would change after that. This method is one of the methods used by UDP
//! trackers to generate and verify a connection ID, which a a token sent to
//! the client to identify the connection.
use std::num::{IntErrorKind, TryFromIntError};
use std::time::Duration;

use super::{Stopped, TimeNow, Type, Working};

/// This trait defines the operations that can be performed on a `TimeExtent`.
pub trait Extent: Sized + Default {
    type Base;
    type Multiplier;
    type Product;

    /// It creates a new `TimeExtent`.
    fn new(unit: &Self::Base, count: &Self::Multiplier) -> Self;

    /// It increases the `TimeExtent` by a multiplier.
    ///
    /// # Errors
    ///
    /// Will return `IntErrorKind` if `add` would overflow the internal `Duration`.
    fn increase(&self, add: Self::Multiplier) -> Result<Self, IntErrorKind>;

    /// It decreases the `TimeExtent` by a multiplier.
    ///
    /// # Errors
    ///
    /// Will return `IntErrorKind` if `sub` would underflow the internal `Duration`.
    fn decrease(&self, sub: Self::Multiplier) -> Result<Self, IntErrorKind>;

    /// It returns the total `Duration` of the `TimeExtent`.
    fn total(&self) -> Option<Result<Self::Product, TryFromIntError>>;

    /// It returns the total `Duration` of the `TimeExtent` plus one increment.
    fn total_next(&self) -> Option<Result<Self::Product, TryFromIntError>>;
}

/// The `TimeExtent` base `Duration`, which is the duration of a single interval.
pub type Base = Duration;
/// The `TimeExtent` `Multiplier`, which is the number of `Base` duration intervals.
pub type Multiplier = u64;
/// The `TimeExtent` product, which is the total duration of the `TimeExtent`.
pub type Product = Base;

/// A `TimeExtent` is a duration of time which contains N times intervals
/// of the same duration.
#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct TimeExtent {
    pub increment: Base,
    pub amount: Multiplier,
}

/// A zero time extent. It's the additive identity for a `TimeExtent`.
pub const ZERO: TimeExtent = TimeExtent {
    increment: Base::ZERO,
    amount: Multiplier::MIN,
};

/// The maximum value for a `TimeExtent`.
pub const MAX: TimeExtent = TimeExtent {
    increment: Base::MAX,
    amount: Multiplier::MAX,
};

impl TimeExtent {
    #[must_use]
    pub const fn from_sec(seconds: u64, amount: &Multiplier) -> Self {
        Self {
            increment: Base::from_secs(seconds),
            amount: *amount,
        }
    }
}

fn checked_duration_from_nanos(time: u128) -> Result<Duration, TryFromIntError> {
    const NANOS_PER_SEC: u32 = 1_000_000_000;

    let secs = time.div_euclid(u128::from(NANOS_PER_SEC));
    let nanos = time.rem_euclid(u128::from(NANOS_PER_SEC));

    assert!(nanos < u128::from(NANOS_PER_SEC));

    match u64::try_from(secs) {
        Err(error) => Err(error),
        Ok(secs) => Ok(Duration::new(secs, nanos.try_into().unwrap())),
    }
}

impl Extent for TimeExtent {
    type Base = Base;
    type Multiplier = Multiplier;
    type Product = Product;

    fn new(increment: &Self::Base, amount: &Self::Multiplier) -> Self {
        Self {
            increment: *increment,
            amount: *amount,
        }
    }

    fn increase(&self, add: Self::Multiplier) -> Result<Self, IntErrorKind> {
        match self.amount.checked_add(add) {
            None => Err(IntErrorKind::PosOverflow),
            Some(amount) => Ok(Self {
                increment: self.increment,
                amount,
            }),
        }
    }

    fn decrease(&self, sub: Self::Multiplier) -> Result<Self, IntErrorKind> {
        match self.amount.checked_sub(sub) {
            None => Err(IntErrorKind::NegOverflow),
            Some(amount) => Ok(Self {
                increment: self.increment,
                amount,
            }),
        }
    }

    fn total(&self) -> Option<Result<Self::Product, TryFromIntError>> {
        self.increment
            .as_nanos()
            .checked_mul(u128::from(self.amount))
            .map(checked_duration_from_nanos)
    }

    fn total_next(&self) -> Option<Result<Self::Product, TryFromIntError>> {
        self.increment
            .as_nanos()
            .checked_mul(u128::from(self.amount) + 1)
            .map(checked_duration_from_nanos)
    }
}

/// A `TimeExtent` maker. It's a clock base on time extents.
/// It gives you the time in time extents.
pub trait Make<Clock>: Sized
where
    Clock: TimeNow,
{
    /// It gives you the current time extent (with a certain increment) for
    /// the current time. It gets the current timestamp front he `Clock`.
    ///
    /// For example:
    ///
    /// - If the base increment is `1` second, it will return a time extent
    ///   whose duration is `1 second` and whose multiplier is the the number
    ///   of seconds since the Unix Epoch (time extent).
    /// - If the base increment is `1` minute, it will return a time extent
    ///   whose duration is `60 seconds` and whose multiplier is the number of
    ///   minutes since the Unix Epoch (time extent).
    #[must_use]
    fn now(increment: &Base) -> Option<Result<TimeExtent, TryFromIntError>> {
        Clock::now()
            .as_nanos()
            .checked_div((*increment).as_nanos())
            .map(|amount| match Multiplier::try_from(amount) {
                Err(error) => Err(error),
                Ok(amount) => Ok(TimeExtent::new(increment, &amount)),
            })
    }

    /// Same as [`now`](crate::shared::clock::time_extent::Make::now), but it
    /// will add an extra duration to the current time before calculating the
    /// time extent. It gives you a time extent for a time in the future.
    #[must_use]
    fn now_after(increment: &Base, add_time: &Duration) -> Option<Result<TimeExtent, TryFromIntError>> {
        match Clock::add(add_time) {
            None => None,
            Some(time) => time
                .as_nanos()
                .checked_div(increment.as_nanos())
                .map(|amount| match Multiplier::try_from(amount) {
                    Err(error) => Err(error),
                    Ok(amount) => Ok(TimeExtent::new(increment, &amount)),
                }),
        }
    }

    /// Same as [`now`](crate::shared::clock::time_extent::Make::now), but it
    /// will subtract a duration to the current time before calculating the
    /// time extent. It gives you a time extent for a time in the past.
    #[must_use]
    fn now_before(increment: &Base, sub_time: &Duration) -> Option<Result<TimeExtent, TryFromIntError>> {
        match Clock::sub(sub_time) {
            None => None,
            Some(time) => time
                .as_nanos()
                .checked_div(increment.as_nanos())
                .map(|amount| match Multiplier::try_from(amount) {
                    Err(error) => Err(error),
                    Ok(amount) => Ok(TimeExtent::new(increment, &amount)),
                }),
        }
    }
}

/// A `TimeExtent` maker which makes `TimeExtents`.
///
/// It's a clock which measures time in `TimeExtents`.
#[derive(Debug)]
pub struct Maker<const CLOCK_TYPE: usize> {}

/// A `TimeExtent` maker which makes `TimeExtents` from the `Working` clock.
pub type WorkingTimeExtentMaker = Maker<{ Type::WorkingClock as usize }>;

/// A `TimeExtent` maker which makes `TimeExtents` from the `Stopped` clock.
pub type StoppedTimeExtentMaker = Maker<{ Type::StoppedClock as usize }>;

impl Make<Working> for WorkingTimeExtentMaker {}
impl Make<Stopped> for StoppedTimeExtentMaker {}

/// The default `TimeExtent` maker. It is `WorkingTimeExtentMaker` in production
/// and `StoppedTimeExtentMaker` in tests.
#[cfg(not(test))]
pub type DefaultTimeExtentMaker = WorkingTimeExtentMaker;

/// The default `TimeExtent` maker. It is `WorkingTimeExtentMaker` in production
/// and `StoppedTimeExtentMaker` in tests.
#[cfg(test)]
pub type DefaultTimeExtentMaker = StoppedTimeExtentMaker;

#[cfg(test)]
mod test {

    use crate::shared::clock::time_extent::{
        checked_duration_from_nanos, Base, DefaultTimeExtentMaker, Extent, Make, Multiplier, Product, TimeExtent, MAX, ZERO,
    };
    use crate::shared::clock::{Current, DurationSinceUnixEpoch, StoppedTime};

    const TIME_EXTENT_VAL: TimeExtent = TimeExtent::from_sec(2, &239_812_388_723);

    mod fn_checked_duration_from_nanos {
        use std::time::Duration;

        use super::*;

        const NANOS_PER_SEC: u32 = 1_000_000_000;

        #[test]
        fn it_should_give_zero_for_zero_input() {
            assert_eq!(checked_duration_from_nanos(0).unwrap(), Duration::ZERO);
        }

        #[test]
        fn it_should_be_the_same_as_duration_implementation_for_u64_numbers() {
            assert_eq!(
                checked_duration_from_nanos(1_232_143_214_343_432).unwrap(),
                Duration::from_nanos(1_232_143_214_343_432)
            );
            assert_eq!(
                checked_duration_from_nanos(u128::from(u64::MAX)).unwrap(),
                Duration::from_nanos(u64::MAX)
            );
        }

        #[test]
        fn it_should_work_for_some_numbers_larger_than_u64() {
            assert_eq!(
                checked_duration_from_nanos(u128::from(TIME_EXTENT_VAL.amount) * u128::from(NANOS_PER_SEC)).unwrap(),
                Duration::from_secs(TIME_EXTENT_VAL.amount)
            );
        }

        #[test]
        fn it_should_fail_for_numbers_that_are_too_large() {
            assert_eq!(
                checked_duration_from_nanos(u128::MAX).unwrap_err(),
                u64::try_from(u128::MAX).unwrap_err()
            );
        }
    }

    mod time_extent {
        use super::*;

        mod fn_default {

            use super::*;

            #[test]
            fn it_should_default_initialize_to_zero() {
                assert_eq!(TimeExtent::default(), ZERO);
            }
        }

        mod fn_from_sec {
            use super::*;

            #[test]
            fn it_should_make_empty_for_zero() {
                assert_eq!(TimeExtent::from_sec(u64::MIN, &Multiplier::MIN), ZERO);
            }
            #[test]
            fn it_should_make_from_seconds() {
                assert_eq!(
                    TimeExtent::from_sec(TIME_EXTENT_VAL.increment.as_secs(), &TIME_EXTENT_VAL.amount),
                    TIME_EXTENT_VAL
                );
            }
        }

        mod fn_new {
            use super::*;

            #[test]
            fn it_should_make_empty_for_zero() {
                assert_eq!(TimeExtent::new(&Base::ZERO, &Multiplier::MIN), ZERO);
            }

            #[test]
            fn it_should_make_new() {
                assert_eq!(
                    TimeExtent::new(&Base::from_millis(2), &TIME_EXTENT_VAL.amount),
                    TimeExtent {
                        increment: Base::from_millis(2),
                        amount: TIME_EXTENT_VAL.amount
                    }
                );
            }
        }

        mod fn_increase {
            use std::num::IntErrorKind;

            use super::*;

            #[test]
            fn it_should_not_increase_for_zero() {
                assert_eq!(ZERO.increase(0).unwrap(), ZERO);
            }

            #[test]
            fn it_should_increase() {
                assert_eq!(
                    TIME_EXTENT_VAL.increase(50).unwrap(),
                    TimeExtent {
                        increment: TIME_EXTENT_VAL.increment,
                        amount: TIME_EXTENT_VAL.amount + 50,
                    }
                );
            }

            #[test]
            fn it_should_fail_when_attempting_to_increase_beyond_bounds() {
                assert_eq!(TIME_EXTENT_VAL.increase(u64::MAX), Err(IntErrorKind::PosOverflow));
            }
        }

        mod fn_decrease {
            use std::num::IntErrorKind;

            use super::*;

            #[test]
            fn it_should_not_decrease_for_zero() {
                assert_eq!(ZERO.decrease(0).unwrap(), ZERO);
            }

            #[test]
            fn it_should_decrease() {
                assert_eq!(
                    TIME_EXTENT_VAL.decrease(50).unwrap(),
                    TimeExtent {
                        increment: TIME_EXTENT_VAL.increment,
                        amount: TIME_EXTENT_VAL.amount - 50,
                    }
                );
            }

            #[test]
            fn it_should_fail_when_attempting_to_decrease_beyond_bounds() {
                assert_eq!(TIME_EXTENT_VAL.decrease(u64::MAX), Err(IntErrorKind::NegOverflow));
            }
        }

        mod fn_total {
            use super::*;

            #[test]
            fn it_should_be_zero_for_zero() {
                assert_eq!(ZERO.total().unwrap().unwrap(), Product::ZERO);
            }

            #[test]
            fn it_should_give_a_total() {
                assert_eq!(
                    TIME_EXTENT_VAL.total().unwrap().unwrap(),
                    Product::from_secs(TIME_EXTENT_VAL.increment.as_secs() * TIME_EXTENT_VAL.amount)
                );

                assert_eq!(
                    TimeExtent::new(&Base::from_millis(2), &(TIME_EXTENT_VAL.amount * 1000))
                        .total()
                        .unwrap()
                        .unwrap(),
                    Product::from_secs(TIME_EXTENT_VAL.increment.as_secs() * TIME_EXTENT_VAL.amount)
                );

                assert_eq!(
                    TimeExtent::new(&Base::from_secs(1), &(u64::MAX)).total().unwrap().unwrap(),
                    Product::from_secs(u64::MAX)
                );
            }

            #[test]
            fn it_should_fail_when_too_large() {
                assert_eq!(MAX.total(), None);
            }

            #[test]
            fn it_should_fail_when_product_is_too_large() {
                let time_extent = TimeExtent {
                    increment: MAX.increment,
                    amount: 2,
                };
                assert_eq!(
                    time_extent.total().unwrap().unwrap_err(),
                    u64::try_from(u128::MAX).unwrap_err()
                );
            }
        }

        mod fn_total_next {
            use super::*;

            #[test]
            fn it_should_be_zero_for_zero() {
                assert_eq!(ZERO.total_next().unwrap().unwrap(), Product::ZERO);
            }

            #[test]
            fn it_should_give_a_total() {
                assert_eq!(
                    TIME_EXTENT_VAL.total_next().unwrap().unwrap(),
                    Product::from_secs(TIME_EXTENT_VAL.increment.as_secs() * (TIME_EXTENT_VAL.amount + 1))
                );

                assert_eq!(
                    TimeExtent::new(&Base::from_millis(2), &(TIME_EXTENT_VAL.amount * 1000))
                        .total_next()
                        .unwrap()
                        .unwrap(),
                    Product::new(
                        TIME_EXTENT_VAL.increment.as_secs() * (TIME_EXTENT_VAL.amount),
                        Base::from_millis(2).as_nanos().try_into().unwrap()
                    )
                );

                assert_eq!(
                    TimeExtent::new(&Base::from_secs(1), &(u64::MAX - 1))
                        .total_next()
                        .unwrap()
                        .unwrap(),
                    Product::from_secs(u64::MAX)
                );
            }

            #[test]
            fn it_should_fail_when_too_large() {
                assert_eq!(MAX.total_next(), None);
            }

            #[test]
            fn it_should_fail_when_product_is_too_large() {
                let time_extent = TimeExtent {
                    increment: MAX.increment,
                    amount: 2,
                };
                assert_eq!(
                    time_extent.total_next().unwrap().unwrap_err(),
                    u64::try_from(u128::MAX).unwrap_err()
                );
            }
        }
    }

    mod make_time_extent {
        use super::*;

        mod fn_now {
            use super::*;

            #[test]
            fn it_should_give_a_time_extent() {
                assert_eq!(
                    DefaultTimeExtentMaker::now(&TIME_EXTENT_VAL.increment).unwrap().unwrap(),
                    TimeExtent {
                        increment: TIME_EXTENT_VAL.increment,
                        amount: 0
                    }
                );

                Current::local_set(&DurationSinceUnixEpoch::from_secs(TIME_EXTENT_VAL.amount * 2));

                assert_eq!(
                    DefaultTimeExtentMaker::now(&TIME_EXTENT_VAL.increment).unwrap().unwrap(),
                    TIME_EXTENT_VAL
                );
            }

            #[test]
            fn it_should_fail_for_zero() {
                assert_eq!(DefaultTimeExtentMaker::now(&Base::ZERO), None);
            }

            #[test]
            fn it_should_fail_if_amount_exceeds_bounds() {
                Current::local_set(&DurationSinceUnixEpoch::MAX);
                assert_eq!(
                    DefaultTimeExtentMaker::now(&Base::from_millis(1)).unwrap().unwrap_err(),
                    u64::try_from(u128::MAX).unwrap_err()
                );
            }
        }

        mod fn_now_after {
            use std::time::Duration;

            use super::*;

            #[test]
            fn it_should_give_a_time_extent() {
                assert_eq!(
                    DefaultTimeExtentMaker::now_after(
                        &TIME_EXTENT_VAL.increment,
                        &Duration::from_secs(TIME_EXTENT_VAL.amount * 2)
                    )
                    .unwrap()
                    .unwrap(),
                    TIME_EXTENT_VAL
                );
            }

            #[test]
            fn it_should_fail_for_zero() {
                assert_eq!(DefaultTimeExtentMaker::now_after(&Base::ZERO, &Duration::ZERO), None);

                Current::local_set(&DurationSinceUnixEpoch::MAX);
                assert_eq!(DefaultTimeExtentMaker::now_after(&Base::ZERO, &Duration::MAX), None);
            }

            #[test]
            fn it_should_fail_if_amount_exceeds_bounds() {
                Current::local_set(&DurationSinceUnixEpoch::MAX);
                assert_eq!(
                    DefaultTimeExtentMaker::now_after(&Base::from_millis(1), &Duration::ZERO)
                        .unwrap()
                        .unwrap_err(),
                    u64::try_from(u128::MAX).unwrap_err()
                );
            }
        }
        mod fn_now_before {
            use std::time::Duration;

            use super::*;

            #[test]
            fn it_should_give_a_time_extent() {
                Current::local_set(&DurationSinceUnixEpoch::MAX);

                assert_eq!(
                    DefaultTimeExtentMaker::now_before(
                        &Base::from_secs(u64::from(u32::MAX)),
                        &Duration::from_secs(u64::from(u32::MAX))
                    )
                    .unwrap()
                    .unwrap(),
                    TimeExtent {
                        increment: Base::from_secs(u64::from(u32::MAX)),
                        amount: 4_294_967_296
                    }
                );
            }

            #[test]
            fn it_should_fail_for_zero() {
                assert_eq!(DefaultTimeExtentMaker::now_before(&Base::ZERO, &Duration::ZERO), None);

                assert_eq!(DefaultTimeExtentMaker::now_before(&Base::ZERO, &Duration::MAX), None);
            }

            #[test]
            fn it_should_fail_if_amount_exceeds_bounds() {
                Current::local_set(&DurationSinceUnixEpoch::MAX);
                assert_eq!(
                    DefaultTimeExtentMaker::now_before(&Base::from_millis(1), &Duration::ZERO)
                        .unwrap()
                        .unwrap_err(),
                    u64::try_from(u128::MAX).unwrap_err()
                );
            }
        }
    }
}
