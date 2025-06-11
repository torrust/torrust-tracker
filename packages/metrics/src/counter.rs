use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::prometheus::PrometheusSerializable;

#[derive(Debug, Display, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Counter(u64);

impl Counter {
    #[must_use]
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> u64 {
        self.0
    }

    #[must_use]
    pub fn primitive(&self) -> u64 {
        self.value()
    }

    pub fn increment(&mut self, value: u64) {
        self.0 += value;
    }

    pub fn absolute(&mut self, value: u64) {
        self.0 = value;
    }
}

impl From<u32> for Counter {
    fn from(value: u32) -> Self {
        Self(u64::from(value))
    }
}

impl From<u64> for Counter {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<i32> for Counter {
    fn from(value: i32) -> Self {
        #[allow(clippy::cast_sign_loss)]
        Self(value as u64)
    }
}

impl From<Counter> for u64 {
    fn from(counter: Counter) -> Self {
        counter.value()
    }
}

impl PrometheusSerializable for Counter {
    fn to_prometheus(&self) -> String {
        format!("{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_created_from_integer_values() {
        let counter = Counter::new(0);
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn it_could_be_converted_from_u64() {
        let counter: Counter = 42.into();
        assert_eq!(counter.value(), 42);
    }

    #[test]
    fn it_could_be_converted_into_u64() {
        let counter = Counter::new(42);
        let value: u64 = counter.into();
        assert_eq!(value, 42);
    }

    #[test]
    fn it_could_be_incremented() {
        let mut counter = Counter::new(0);
        counter.increment(1);
        assert_eq!(counter.value(), 1);

        counter.increment(2);
        assert_eq!(counter.value(), 3);
    }

    #[test]
    fn it_could_set_to_an_absolute_value() {
        let mut counter = Counter::new(0);
        counter.absolute(1);
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn it_serializes_to_prometheus() {
        let counter = Counter::new(42);
        assert_eq!(counter.to_prometheus(), "42");
    }
}
