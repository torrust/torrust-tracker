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
}
