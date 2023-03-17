use std::num::{IntErrorKind, TryFromIntError};
use std::time::Duration;

use super::{Stopped, TimeNow, Type, Working};

pub trait Extent: Sized + Default {
    type Base;
    type Multiplier;
    type Product;

    fn new(unit: &Self::Base, count: &Self::Multiplier) -> Self;

    /// # Errors
    ///
    /// Will return `IntErrorKind` if `add` would overflow the internal `Duration`.
    fn increase(&self, add: Self::Multiplier) -> Result<Self, IntErrorKind>;

    /// # Errors
    ///
    /// Will return `IntErrorKind` if `sub` would underflow the internal `Duration`.
    fn decrease(&self, sub: Self::Multiplier) -> Result<Self, IntErrorKind>;

    fn total(&self) -> Option<Result<Self::Product, TryFromIntError>>;
    fn total_next(&self) -> Option<Result<Self::Product, TryFromIntError>>;
}

pub type Base = Duration;
pub type Multiplier = u64;
pub type Product = Base;

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct TimeExtent {
    pub increment: Base,
    pub amount: Multiplier,
}

pub const ZERO: TimeExtent = TimeExtent {
    increment: Base::ZERO,
    amount: Multiplier::MIN,
};
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

pub trait Make<Clock>: Sized
where
    Clock: TimeNow,
{
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

#[derive(Debug)]
pub struct Maker<const CLOCK_TYPE: usize> {}

pub type WorkingTimeExtentMaker = Maker<{ Type::WorkingClock as usize }>;
pub type StoppedTimeExtentMaker = Maker<{ Type::StoppedClock as usize }>;

impl Make<Working> for WorkingTimeExtentMaker {}
impl Make<Stopped> for StoppedTimeExtentMaker {}

#[cfg(not(test))]
pub type DefaultTimeExtentMaker = WorkingTimeExtentMaker;

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
