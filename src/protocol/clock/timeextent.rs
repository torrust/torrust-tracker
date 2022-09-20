use std::num::{IntErrorKind, TryFromIntError};
use std::time::Duration;

use super::{ClockType, StoppedClock, TimeNow, WorkingClock};

pub trait Extent: Sized + Default {
    type Base;
    type Multiplier;
    type Product;

    fn new(unit: &Self::Base, count: &Self::Multiplier) -> Self;

    fn add(&self, add: Self::Multiplier) -> Result<Self, IntErrorKind>;
    fn sub(&self, sub: Self::Multiplier) -> Result<Self, IntErrorKind>;

    fn total(&self) -> Result<Option<Self::Product>, TryFromIntError>;
    fn total_next(&self) -> Result<Option<Self::Product>, TryFromIntError>;
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

    fn add(&self, add: Self::Multiplier) -> Result<Self, IntErrorKind> {
        match self.amount.checked_add(add) {
            None => Err(IntErrorKind::PosOverflow),
            Some(amount) => Ok(Self {
                increment: self.increment,
                amount,
            }),
        }
    }

    fn sub(&self, sub: Self::Multiplier) -> Result<Self, IntErrorKind> {
        match self.amount.checked_sub(sub) {
            None => Err(IntErrorKind::NegOverflow),
            Some(amount) => Ok(Self {
                increment: self.increment,
                amount,
            }),
        }
    }

    fn total(&self) -> Result<Option<Self::Product>, TryFromIntError> {
        match u32::try_from(self.amount) {
            Err(error) => Err(error),
            Ok(amount) => Ok(self.increment.checked_mul(amount)),
        }
    }

    fn total_next(&self) -> Result<Option<Self::Product>, TryFromIntError> {
        match u32::try_from(self.amount) {
            Err(e) => Err(e),
            Ok(amount) => match amount.checked_add(1) {
                None => Ok(None),
                Some(amount) => match self.increment.checked_mul(amount) {
                    None => Ok(None),
                    Some(extent) => Ok(Some(extent)),
                },
            },
        }
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

    fn now_add(increment: &TimeExtentBase, add_time: &Duration) -> Option<Result<TimeExtent, TryFromIntError>> {
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
    fn now_sub(increment: &TimeExtentBase, sub_time: &Duration) -> Option<Result<TimeExtent, TryFromIntError>> {
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

pub type WorkingClockTimeExtentMaker = TimeExtentMaker<{ ClockType::WorkingClock as usize }>;
pub type StoppedClockTimeExtentMaker = TimeExtentMaker<{ ClockType::StoppedClock as usize }>;

impl MakeTimeExtent<WorkingClock> for WorkingClockTimeExtentMaker {}
impl MakeTimeExtent<StoppedClock> for StoppedClockTimeExtentMaker {}

#[cfg(not(test))]
pub type DefaultClockTimeExtentMaker = WorkingClockTimeExtentMaker;

#[cfg(test)]
pub type DefaultClockTimeExtentMaker = StoppedClockTimeExtentMaker;

#[cfg(test)]
mod test {

    use std::time::Duration;

    use crate::protocol::clock::timeextent::{DefaultClockTimeExtentMaker, Extent, MakeTimeExtent, TimeExtent};
    use crate::protocol::clock::{DefaultClock, DurationSinceUnixEpoch, StoppedTime};

    #[test]
    fn it_should_get_the_total_duration() {
        assert_eq!(TimeExtent::default().total().unwrap().unwrap(), Duration::ZERO);

        assert_eq!(
            TimeExtent::from_sec(12, &12).total().unwrap().unwrap(),
            Duration::from_secs(144)
        );
        assert_eq!(
            TimeExtent::from_sec(12, &12).total_next().unwrap().unwrap(),
            Duration::from_secs(156)
        );
    }

    #[test]
    fn it_should_make_the_current_extent() {
        assert_eq!(
            DefaultClockTimeExtentMaker::now(&Duration::from_secs(2)).unwrap().unwrap(),
            TimeExtent::from_sec(2, &0)
        );

        DefaultClock::local_set(&DurationSinceUnixEpoch::from_secs(12387687123));

        assert_eq!(
            DefaultClockTimeExtentMaker::now(&Duration::from_secs(2)).unwrap().unwrap(),
            TimeExtent::from_sec(2, &6193843561)
        );
    }
}
