use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::prometheus::PrometheusSerializable;

#[derive(Debug, Display, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Gauge(f64);

impl Gauge {
    #[must_use]
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> f64 {
        self.0
    }

    #[must_use]
    pub fn primitive(&self) -> f64 {
        self.value()
    }

    pub fn set(&mut self, value: f64) {
        self.0 = value;
    }

    pub fn increment(&mut self, value: f64) {
        self.0 += value;
    }

    pub fn decrement(&mut self, value: f64) {
        self.0 -= value;
    }
}

impl From<f32> for Gauge {
    fn from(value: f32) -> Self {
        Self(f64::from(value))
    }
}

impl From<f64> for Gauge {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Gauge> for f64 {
    fn from(counter: Gauge) -> Self {
        counter.value()
    }
}

impl PrometheusSerializable for Gauge {
    fn to_prometheus(&self) -> String {
        format!("{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn it_should_be_created_from_integer_values() {
        let gauge = Gauge::new(0.0);
        assert_relative_eq!(gauge.value(), 0.0);
    }

    #[test]
    fn it_could_be_converted_from_u64() {
        let gauge: Gauge = 42.0.into();
        assert_relative_eq!(gauge.value(), 42.0);
    }

    #[test]
    fn it_could_be_converted_into_i64() {
        let gauge = Gauge::new(42.0);
        let value: f64 = gauge.into();
        assert_relative_eq!(value, 42.0);
    }

    #[test]
    fn it_could_be_set() {
        let mut gauge = Gauge::new(0.0);
        gauge.set(1.0);
        assert_relative_eq!(gauge.value(), 1.0);
    }

    #[test]
    fn it_could_be_incremented() {
        let mut gauge = Gauge::new(0.0);
        gauge.increment(1.0);
        assert_relative_eq!(gauge.value(), 1.0);
    }

    #[test]
    fn it_could_be_decremented() {
        let mut gauge = Gauge::new(1.0);
        gauge.decrement(1.0);
        assert_relative_eq!(gauge.value(), 0.0);
    }

    #[test]
    fn it_serializes_to_prometheus() {
        let counter = Gauge::new(42.0);
        assert_eq!(counter.to_prometheus(), "42");

        let counter = Gauge::new(42.1);
        assert_eq!(counter.to_prometheus(), "42.1");
    }

    #[test]
    fn it_could_be_converted_from_f32() {
        let gauge: Gauge = 42.5f32.into();
        assert_relative_eq!(gauge.value(), 42.5);
    }

    #[test]
    fn it_should_return_primitive_value() {
        let gauge = Gauge::new(123.456);
        assert_relative_eq!(gauge.primitive(), 123.456);
    }

    #[test]
    fn it_should_handle_zero_value() {
        let gauge = Gauge::new(0.0);
        assert_relative_eq!(gauge.value(), 0.0);
        assert_relative_eq!(gauge.primitive(), 0.0);
    }

    #[test]
    fn it_should_handle_negative_values() {
        let gauge = Gauge::new(-42.5);
        assert_relative_eq!(gauge.value(), -42.5);
    }

    #[test]
    fn it_should_handle_large_values() {
        let gauge = Gauge::new(f64::MAX);
        assert_relative_eq!(gauge.value(), f64::MAX);
    }

    #[test]
    fn it_should_handle_infinity() {
        let gauge = Gauge::new(f64::INFINITY);
        assert_relative_eq!(gauge.value(), f64::INFINITY);
    }

    #[test]
    fn it_should_handle_nan() {
        let gauge = Gauge::new(f64::NAN);
        assert!(gauge.value().is_nan());
    }

    #[test]
    fn it_should_be_displayable() {
        let gauge = Gauge::new(42.5);
        assert_eq!(gauge.to_string(), "42.5");

        let gauge = Gauge::new(0.0);
        assert_eq!(gauge.to_string(), "0");
    }

    #[test]
    fn it_should_be_debuggable() {
        let gauge = Gauge::new(42.5);
        let debug_string = format!("{gauge:?}");
        assert_eq!(debug_string, "Gauge(42.5)");
    }

    #[test]
    fn it_should_be_cloneable() {
        let gauge = Gauge::new(42.5);
        let cloned_gauge = gauge.clone();
        assert_eq!(gauge, cloned_gauge);
        assert_relative_eq!(gauge.value(), cloned_gauge.value());
    }

    #[test]
    fn it_should_support_equality_comparison() {
        let gauge1 = Gauge::new(42.5);
        let gauge2 = Gauge::new(42.5);
        let gauge3 = Gauge::new(43.0);

        assert_eq!(gauge1, gauge2);
        assert_ne!(gauge1, gauge3);
    }

    #[test]
    fn it_should_have_default_value() {
        let gauge = Gauge::default();
        assert_relative_eq!(gauge.value(), 0.0);
    }

    #[test]
    fn it_should_handle_conversion_roundtrip() {
        let original_value = 12345.678;
        let gauge = Gauge::from(original_value);
        let converted_back: f64 = gauge.into();
        assert_relative_eq!(original_value, converted_back);
    }

    #[test]
    fn it_should_handle_f32_conversion_roundtrip() {
        let original_value = 12345.5f32;
        let gauge = Gauge::from(original_value);
        assert_relative_eq!(gauge.value(), f64::from(original_value));
    }

    #[test]
    fn it_should_handle_multiple_operations() {
        let mut gauge = Gauge::new(100.0);

        gauge.increment(50.0);
        assert_relative_eq!(gauge.value(), 150.0);

        gauge.decrement(25.0);
        assert_relative_eq!(gauge.value(), 125.0);

        gauge.set(200.0);
        assert_relative_eq!(gauge.value(), 200.0);
    }

    #[test]
    fn it_should_serialize_special_values_to_prometheus() {
        let gauge = Gauge::new(f64::INFINITY);
        assert_eq!(gauge.to_prometheus(), "inf");

        let gauge = Gauge::new(f64::NEG_INFINITY);
        assert_eq!(gauge.to_prometheus(), "-inf");

        let gauge = Gauge::new(f64::NAN);
        assert_eq!(gauge.to_prometheus(), "NaN");
    }
}
