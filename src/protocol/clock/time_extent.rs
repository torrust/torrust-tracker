use std::num::{IntErrorKind, TryFromIntError};
use std::time::Duration;

use super::{ClockType, StoppedClock, TimeNow, WorkingClock};

pub trait Extent: Sized + Default {
    type Base;
    type Multiplier;
    type Product;

    fn new(unit: &Self::Base, count: &Self::Multiplier) -> Self;

    fn increase(&self, add: Self::Multiplier) -> Result<Self, IntErrorKind>;
    fn decrease(&self, sub: Self::Multiplier) -> Result<Self, IntErrorKind>;

    fn total(&self) -> Option<Result<Self::Product, TryFromIntError>>;
    fn total_next(&self) -> Option<Result<Self::Product, TryFromIntError>>;
}

pub type TimeExtentBase = Duration;
pub type TimeExtentMultiplier = u64;
pub type TimeExtentProduct = TimeExtentBase;

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct TimeExtent {
    pub increment: TimeExtentBase,
    pub amount: TimeExtentMultiplier,
}

impl TimeExtent {
    pub const fn from_sec(seconds: u64, amount: &TimeExtentMultiplier) -> Self {
        Self {
            increment: TimeExtentBase::from_secs(seconds),
            amount: *amount,
        }
    }
}

fn checked_duration_from_nanos(time: u128) -> Result<Duration, TryFromIntError> {
    const NANOS_PER_SEC: u32 = 1_000_000_000;

    let secs = time.div_euclid(NANOS_PER_SEC as u128);
    let nanos = time.rem_euclid(NANOS_PER_SEC as u128);

    assert!(nanos < NANOS_PER_SEC as u128);

    match u64::try_from(secs) {
        Err(error) => Err(error),
        Ok(secs) => Ok(Duration::new(secs, nanos.try_into().unwrap())),
    }
}

impl Extent for TimeExtent {
    type Base = TimeExtentBase;
    type Multiplier = TimeExtentMultiplier;
    type Product = TimeExtentProduct;

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
            .checked_mul(self.amount as u128)
            .map(checked_duration_from_nanos)
    }

    fn total_next(&self) -> Option<Result<Self::Product, TryFromIntError>> {
        self.increment
            .as_nanos()
            .checked_mul((self.amount as u128) + 1)
            .map(checked_duration_from_nanos)
    }
}

pub trait MakeTimeExtent<Clock>: Sized
where
    Clock: TimeNow,
{
    fn now(increment: &TimeExtentBase) -> Option<Result<TimeExtent, TryFromIntError>> {
        Clock::now()
            .as_nanos()
            .checked_div((*increment).as_nanos())
            .map(|amount| match TimeExtentMultiplier::try_from(amount) {
                Err(error) => Err(error),
                Ok(amount) => Ok(TimeExtent::new(increment, &amount)),
            })
    }

    fn now_after(increment: &TimeExtentBase, add_time: &Duration) -> Option<Result<TimeExtent, TryFromIntError>> {
        match Clock::add(add_time) {
            None => None,
            Some(time) => {
                time.as_nanos()
                    .checked_div(increment.as_nanos())
                    .map(|amount| match TimeExtentMultiplier::try_from(amount) {
                        Err(error) => Err(error),
                        Ok(amount) => Ok(TimeExtent::new(increment, &amount)),
                    })
            }
        }
    }
    fn now_before(increment: &TimeExtentBase, sub_time: &Duration) -> Option<Result<TimeExtent, TryFromIntError>> {
        match Clock::sub(sub_time) {
            None => None,
            Some(time) => {
                time.as_nanos()
                    .checked_div(increment.as_nanos())
                    .map(|amount| match TimeExtentMultiplier::try_from(amount) {
                        Err(error) => Err(error),
                        Ok(amount) => Ok(TimeExtent::new(increment, &amount)),
                    })
            }
        }
    }
}

#[derive(Debug)]
pub struct TimeExtentMaker<const CLOCK_TYPE: usize> {}

pub type WorkingTimeExtentMaker = TimeExtentMaker<{ ClockType::WorkingClock as usize }>;
pub type StoppedTimeExtentMaker = TimeExtentMaker<{ ClockType::StoppedClock as usize }>;

impl MakeTimeExtent<WorkingClock> for WorkingTimeExtentMaker {}
impl MakeTimeExtent<StoppedClock> for StoppedTimeExtentMaker {}

#[cfg(not(test))]
pub type DefaultTimeExtentMaker = WorkingTimeExtentMaker;

#[cfg(test)]
pub type DefaultTimeExtentMaker = StoppedTimeExtentMaker;

#[cfg(test)]
mod test {

    use crate::protocol::clock::time_extent::{
        checked_duration_from_nanos, DefaultTimeExtentMaker, Extent, MakeTimeExtent, TimeExtent, TimeExtentBase,
        TimeExtentProduct,
    };
    use crate::protocol::clock::{DefaultClock, DurationSinceUnixEpoch, StoppedTime};

    const TIME_EXTENT_VAL: TimeExtent = TimeExtent::from_sec(2, &239812388723);

    mod fn_checked_duration_from_nanos {
        use std::time::Duration;

        use super::*;

        const NANOS_PER_SEC: u32 = 1_000_000_000;

        #[test]
        fn it_should_return_a_duration() {
            assert_eq!(checked_duration_from_nanos(0).unwrap(), Duration::from_micros(0));
            assert_eq!(
                checked_duration_from_nanos(1232143214343432).unwrap(),
                Duration::from_nanos(1232143214343432)
            );
            assert_eq!(
                checked_duration_from_nanos(u64::MAX as u128).unwrap(),
                Duration::from_nanos(u64::MAX)
            );
            assert_eq!(
                checked_duration_from_nanos(TIME_EXTENT_VAL.amount as u128 * NANOS_PER_SEC as u128).unwrap(),
                Duration::from_secs(TIME_EXTENT_VAL.amount)
            );
        }

        #[test]
        fn it_should_return_tryfrom_int_error() {
            assert_eq!(
                checked_duration_from_nanos(u128::MAX).unwrap_err(),
                u64::try_from(u128::MAX).unwrap_err()
            );
        }
    }

    mod time_extent_from_sec {
        use super::*;

        #[test]
        fn it_should_make_time_extent() {
            assert_eq!(TIME_EXTENT_VAL.increment, TimeExtentBase::from_secs(2));
            assert_eq!(TIME_EXTENT_VAL.amount, 239812388723);
        }
    }

    mod time_extent_default {
        use super::*;

        #[test]
        fn it_should_make_time_extent() {
            let time_extent_default = TimeExtent::default();
            assert_eq!(time_extent_default.increment, TimeExtentBase::ZERO);
            assert_eq!(time_extent_default.amount, 0);
        }
    }

    mod time_extent_new {
        use super::*;

        #[test]
        fn it_should_make_time_extent() {
            let time_extent = TimeExtent::new(&TimeExtentBase::from_millis(2), &TIME_EXTENT_VAL.amount);
            assert_eq!(time_extent.increment, TimeExtentBase::from_millis(2));
            assert_eq!(time_extent.amount, TIME_EXTENT_VAL.amount);
        }
    }

    mod time_extent_increase {
        use std::num::IntErrorKind;

        use super::*;

        #[test]
        fn it_should_return_increased() {
            let time_extent_default = TimeExtent::default();
            let time_extent = TimeExtent::new(&TimeExtentBase::from_millis(2), &TIME_EXTENT_VAL.amount);

            let time_extent_default_increase = TimeExtent {
                increment: TimeExtentBase::ZERO,
                amount: 50,
            };
            let time_extent_increase = TimeExtent {
                increment: TimeExtentBase::from_millis(2),
                amount: TIME_EXTENT_VAL.amount + 50,
            };
            let time_extent_from_sec_increase = TimeExtent {
                increment: TIME_EXTENT_VAL.increment,
                amount: TIME_EXTENT_VAL.amount + 50,
            };

            assert_eq!(time_extent_default.increase(50).unwrap(), time_extent_default_increase);
            assert_eq!(time_extent.increase(50).unwrap(), time_extent_increase);
            assert_eq!(TIME_EXTENT_VAL.increase(50).unwrap(), time_extent_from_sec_increase);
        }

        #[test]
        fn it_should_postive_overflow() {
            assert_eq!(TIME_EXTENT_VAL.increase(u64::MAX), Err(IntErrorKind::PosOverflow));
        }
    }

    mod time_extent_decrease {
        use std::num::IntErrorKind;

        use super::*;

        #[test]
        fn it_should_return_decreased() {
            let time_extent_default = TimeExtent::default();
            let time_extent = TimeExtent::new(&TimeExtentBase::from_millis(2), &TIME_EXTENT_VAL.amount);

            let time_extent_default_decrease = TimeExtent {
                increment: TimeExtentBase::ZERO,
                amount: 0,
            };
            let time_extent_decrease = TimeExtent {
                increment: TimeExtentBase::from_millis(2),
                amount: TIME_EXTENT_VAL.amount - 50,
            };
            let time_extent_from_sec_decrease = TimeExtent {
                increment: TIME_EXTENT_VAL.increment,
                amount: TIME_EXTENT_VAL.amount - 50,
            };

            assert_eq!(time_extent_default.decrease(0).unwrap(), time_extent_default_decrease);
            assert_eq!(time_extent.decrease(50).unwrap(), time_extent_decrease);
            assert_eq!(TIME_EXTENT_VAL.decrease(50).unwrap(), time_extent_from_sec_decrease);
        }

        #[test]
        fn it_should_return_an_negitive_overflow() {
            assert_eq!(TIME_EXTENT_VAL.decrease(u64::MAX), Err(IntErrorKind::NegOverflow));
        }
    }

    mod time_extent_total {
        use super::*;

        #[test]
        fn it_should_return_total() {
            let time_extent_default = TimeExtent::default();
            let time_extent = TimeExtent::new(&TimeExtentBase::from_millis(2), &(TIME_EXTENT_VAL.amount / 1000));

            assert_eq!(time_extent_default.total().unwrap().unwrap(), TimeExtentProduct::ZERO);
            assert_eq!(
                time_extent.total().unwrap().unwrap(),
                TimeExtentProduct::new(479624, 776000000)
            );
            assert_eq!(
                TIME_EXTENT_VAL.total().unwrap().unwrap(),
                TimeExtentProduct::from_secs(TIME_EXTENT_VAL.increment.as_secs() * TIME_EXTENT_VAL.amount)
            );
        }

        #[test]
        fn it_should_return_none() {
            let time_extent_max = TimeExtent {
                increment: TimeExtentBase::MAX,
                amount: u64::MAX as u64,
            };
            assert_eq!(time_extent_max.total(), None);
        }

        #[test]
        fn it_should_return_tryfrom_int_error() {
            let time_extent_max = TimeExtent {
                increment: TimeExtentBase::MAX,
                amount: 2,
            };
            assert_eq!(
                time_extent_max.total().unwrap().unwrap_err(),
                u64::try_from(u128::MAX).unwrap_err()
            );
        }
    }

    mod time_extent_total_next {
        use super::*;

        #[test]
        fn it_should_get_the_time_extent_total_next() {
            let time_extent_default = TimeExtent::default();
            let time_extent = TimeExtent::new(&TimeExtentBase::from_millis(2), &TIME_EXTENT_VAL.amount);

            assert_eq!(
                time_extent_default.total_next().unwrap().unwrap(),
                TimeExtentProduct::from_secs(0)
            );
            assert_eq!(
                time_extent.total_next().unwrap().unwrap(),
                TimeExtentProduct::new(479624777, 448000000)
            );
            assert_eq!(
                TIME_EXTENT_VAL.total_next().unwrap().unwrap(),
                TimeExtentProduct::from_secs(TIME_EXTENT_VAL.increment.as_secs() * (TIME_EXTENT_VAL.amount + 1))
            );
        }

        #[test]
        fn it_should_return_none() {
            let time_extent_max = TimeExtent {
                increment: TimeExtentBase::MAX,
                amount: u64::MAX as u64,
            };
            assert_eq!(time_extent_max.total_next(), None);
        }

        #[test]
        fn it_should_return_tryfrom_int_error() {
            let time_extent_max = TimeExtent {
                increment: TimeExtentBase::MAX,
                amount: 2,
            };
            assert_eq!(
                time_extent_max.total_next().unwrap().unwrap_err(),
                u64::try_from(u128::MAX).unwrap_err()
            );
        }
    }

    mod make_time_extent_now {
        use super::*;

        #[test]
        fn it_should_return_a_time_extent() {
            assert_eq!(
                DefaultTimeExtentMaker::now(&TIME_EXTENT_VAL.increment).unwrap().unwrap(),
                TimeExtent {
                    increment: TIME_EXTENT_VAL.increment,
                    amount: 0
                }
            );

            DefaultClock::local_set(&DurationSinceUnixEpoch::from_secs(TIME_EXTENT_VAL.amount * 2));

            assert_eq!(
                DefaultTimeExtentMaker::now(&TIME_EXTENT_VAL.increment).unwrap().unwrap(),
                TIME_EXTENT_VAL
            );
        }

        #[test]
        fn it_should_return_none() {
            assert_eq!(DefaultTimeExtentMaker::now(&TimeExtentBase::ZERO), None);
        }

        #[test]
        fn it_should_return_tryfrom_int_error() {
            DefaultClock::local_set(&DurationSinceUnixEpoch::MAX);
            assert_eq!(
                DefaultTimeExtentMaker::now(&TimeExtentBase::from_millis(1))
                    .unwrap()
                    .unwrap_err(),
                u64::try_from(u128::MAX).unwrap_err()
            );
        }
    }

    mod make_time_extent_now_after {
        use std::time::Duration;

        use super::*;

        #[test]
        fn it_should_return_a_time_extent() {
            assert_eq!(
                DefaultTimeExtentMaker::now_after(&TIME_EXTENT_VAL.increment, &Duration::from_secs(TIME_EXTENT_VAL.amount * 2))
                    .unwrap()
                    .unwrap(),
                TIME_EXTENT_VAL
            );
        }

        #[test]
        fn it_should_return_none() {
            assert_eq!(
                DefaultTimeExtentMaker::now_after(&TimeExtentBase::ZERO, &Duration::ZERO),
                None
            );

            DefaultClock::local_set(&DurationSinceUnixEpoch::MAX);
            assert_eq!(DefaultTimeExtentMaker::now_after(&TimeExtentBase::ZERO, &Duration::MAX), None);
        }

        #[test]
        fn it_should_return_tryfrom_int_error() {
            DefaultClock::local_set(&DurationSinceUnixEpoch::MAX);
            assert_eq!(
                DefaultTimeExtentMaker::now_after(&TimeExtentBase::from_millis(1), &Duration::ZERO)
                    .unwrap()
                    .unwrap_err(),
                u64::try_from(u128::MAX).unwrap_err()
            );
        }
    }
    mod make_time_extent_now_before {
        use std::time::Duration;

        use super::*;

        #[test]
        fn it_should_return_a_time_extent() {
            DefaultClock::local_set(&DurationSinceUnixEpoch::MAX);

            assert_eq!(
                DefaultTimeExtentMaker::now_before(
                    &TimeExtentBase::from_secs(u32::MAX as u64),
                    &Duration::from_secs(u32::MAX as u64)
                )
                .unwrap()
                .unwrap(),
                TimeExtent {
                    increment: TimeExtentBase::from_secs(u32::MAX as u64),
                    amount: 4294967296
                }
            );
        }

        #[test]
        fn it_should_return_none() {
            assert_eq!(
                DefaultTimeExtentMaker::now_before(&TimeExtentBase::ZERO, &Duration::ZERO),
                None
            );

            assert_eq!(
                DefaultTimeExtentMaker::now_before(&TimeExtentBase::ZERO, &Duration::MAX),
                None
            );
        }

        #[test]
        fn it_should_return_tryfrom_int_error() {
            DefaultClock::local_set(&DurationSinceUnixEpoch::MAX);
            assert_eq!(
                DefaultTimeExtentMaker::now_before(&TimeExtentBase::from_millis(1), &Duration::ZERO)
                    .unwrap()
                    .unwrap_err(),
                u64::try_from(u128::MAX).unwrap_err()
            );
        }
    }
}
