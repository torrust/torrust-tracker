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

    pub fn increment(&mut self, value: u64) {
        self.0 += value;
    }
}

impl From<u64> for Counter {
    fn from(value: u64) -> Self {
        Self(value)
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
    fn it_serializes_to_prometheus() {
        let counter = Counter::new(42);
        assert_eq!(counter.to_prometheus(), "42");
    }
}
