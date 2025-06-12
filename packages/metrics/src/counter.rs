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

    #[test]
    fn it_could_be_converted_from_u32() {
        let counter: Counter = 42u32.into();
        assert_eq!(counter.value(), 42);
    }

    #[test]
    fn it_could_be_converted_from_i32() {
        let counter: Counter = 42i32.into();
        assert_eq!(counter.value(), 42);
    }

    #[test]
    fn it_should_return_primitive_value() {
        let counter = Counter::new(123);
        assert_eq!(counter.primitive(), 123);
    }

    #[test]
    fn it_should_handle_zero_value() {
        let counter = Counter::new(0);
        assert_eq!(counter.value(), 0);
        assert_eq!(counter.primitive(), 0);
    }

    #[test]
    fn it_should_handle_large_values() {
        let counter = Counter::new(u64::MAX);
        assert_eq!(counter.value(), u64::MAX);
    }

    #[test]
    fn it_should_handle_u32_max_conversion() {
        let counter: Counter = u32::MAX.into();
        assert_eq!(counter.value(), u64::from(u32::MAX));
    }

    #[test]
    fn it_should_handle_i32_max_conversion() {
        let counter: Counter = i32::MAX.into();
        assert_eq!(counter.value(), i32::MAX as u64);
    }

    #[test]
    fn it_should_handle_negative_i32_conversion() {
        let counter: Counter = (-42i32).into();
        #[allow(clippy::cast_sign_loss)]
        let expected = (-42i32) as u64;
        assert_eq!(counter.value(), expected);
    }

    #[test]
    fn it_should_handle_i32_min_conversion() {
        let counter: Counter = i32::MIN.into();
        #[allow(clippy::cast_sign_loss)]
        let expected = i32::MIN as u64;
        assert_eq!(counter.value(), expected);
    }

    #[test]
    fn it_should_handle_large_increments() {
        let mut counter = Counter::new(100);
        counter.increment(1000);
        assert_eq!(counter.value(), 1100);

        counter.increment(u64::MAX - 1100);
        assert_eq!(counter.value(), u64::MAX);
    }

    #[test]
    fn it_should_support_multiple_absolute_operations() {
        let mut counter = Counter::new(0);

        counter.absolute(100);
        assert_eq!(counter.value(), 100);

        counter.absolute(50);
        assert_eq!(counter.value(), 50);

        counter.absolute(0);
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn it_should_be_displayable() {
        let counter = Counter::new(42);
        assert_eq!(counter.to_string(), "42");

        let counter = Counter::new(0);
        assert_eq!(counter.to_string(), "0");
    }

    #[test]
    fn it_should_be_debuggable() {
        let counter = Counter::new(42);
        let debug_string = format!("{counter:?}");
        assert_eq!(debug_string, "Counter(42)");
    }

    #[test]
    fn it_should_be_cloneable() {
        let counter = Counter::new(42);
        let cloned_counter = counter.clone();
        assert_eq!(counter, cloned_counter);
        assert_eq!(counter.value(), cloned_counter.value());
    }

    #[test]
    fn it_should_support_equality_comparison() {
        let counter1 = Counter::new(42);
        let counter2 = Counter::new(42);
        let counter3 = Counter::new(43);

        assert_eq!(counter1, counter2);
        assert_ne!(counter1, counter3);
    }

    #[test]
    fn it_should_have_default_value() {
        let counter = Counter::default();
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn it_should_handle_conversion_roundtrip() {
        let original_value = 12345u64;
        let counter = Counter::from(original_value);
        let converted_back: u64 = counter.into();
        assert_eq!(original_value, converted_back);
    }

    #[test]
    fn it_should_handle_u32_conversion_roundtrip() {
        let original_value = 12345u32;
        let counter = Counter::from(original_value);
        assert_eq!(counter.value(), u64::from(original_value));
    }

    #[test]
    fn it_should_handle_i32_conversion_roundtrip() {
        let original_value = 12345i32;
        let counter = Counter::from(original_value);
        #[allow(clippy::cast_sign_loss)]
        let expected = original_value as u64;
        assert_eq!(counter.value(), expected);
    }

    #[test]
    fn it_should_serialize_large_values_to_prometheus() {
        let counter = Counter::new(u64::MAX);
        assert_eq!(counter.to_prometheus(), u64::MAX.to_string());

        let counter = Counter::new(0);
        assert_eq!(counter.to_prometheus(), "0");
    }
}
