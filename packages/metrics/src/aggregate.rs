use derive_more::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq)]
pub struct AggregateValue(f64);

impl AggregateValue {
    #[must_use]
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl From<f64> for AggregateValue {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<AggregateValue> for f64 {
    fn from(value: AggregateValue) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn it_should_be_created_with_new() {
        let value = AggregateValue::new(42.5);
        assert_relative_eq!(value.value(), 42.5);
    }

    #[test]
    fn it_should_return_the_inner_value() {
        let value = AggregateValue::new(123.456);
        assert_relative_eq!(value.value(), 123.456);
    }

    #[test]
    fn it_should_handle_zero_value() {
        let value = AggregateValue::new(0.0);
        assert_relative_eq!(value.value(), 0.0);
    }

    #[test]
    fn it_should_handle_negative_values() {
        let value = AggregateValue::new(-42.5);
        assert_relative_eq!(value.value(), -42.5);
    }

    #[test]
    fn it_should_handle_infinity() {
        let value = AggregateValue::new(f64::INFINITY);
        assert_relative_eq!(value.value(), f64::INFINITY);
    }

    #[test]
    fn it_should_handle_nan() {
        let value = AggregateValue::new(f64::NAN);
        assert!(value.value().is_nan());
    }

    #[test]
    fn it_should_be_created_from_f64() {
        let value: AggregateValue = 42.5.into();
        assert_relative_eq!(value.value(), 42.5);
    }

    #[test]
    fn it_should_convert_to_f64() {
        let value = AggregateValue::new(42.5);
        let f64_value: f64 = value.into();
        assert_relative_eq!(f64_value, 42.5);
    }

    #[test]
    fn it_should_be_displayable() {
        let value = AggregateValue::new(42.5);
        assert_eq!(value.to_string(), "42.5");
    }

    #[test]
    fn it_should_be_debuggable() {
        let value = AggregateValue::new(42.5);
        let debug_string = format!("{value:?}");
        assert_eq!(debug_string, "AggregateValue(42.5)");
    }

    #[test]
    fn it_should_be_cloneable() {
        let value = AggregateValue::new(42.5);
        let cloned_value = value;
        assert_eq!(value, cloned_value);
    }

    #[test]
    fn it_should_be_copyable() {
        let value = AggregateValue::new(42.5);
        let copied_value = value;
        assert_eq!(value, copied_value);
    }

    #[test]
    fn it_should_support_equality_comparison() {
        let value1 = AggregateValue::new(42.5);
        let value2 = AggregateValue::new(42.5);
        let value3 = AggregateValue::new(43.0);

        assert_eq!(value1, value2);
        assert_ne!(value1, value3);
    }

    #[test]
    fn it_should_handle_special_float_values_in_equality() {
        let nan1 = AggregateValue::new(f64::NAN);
        let nan2 = AggregateValue::new(f64::NAN);
        let infinity = AggregateValue::new(f64::INFINITY);
        let neg_infinity = AggregateValue::new(f64::NEG_INFINITY);

        // NaN is not equal to itself in IEEE 754
        assert_ne!(nan1, nan2);
        assert_eq!(infinity, AggregateValue::new(f64::INFINITY));
        assert_eq!(neg_infinity, AggregateValue::new(f64::NEG_INFINITY));
        assert_ne!(infinity, neg_infinity);
    }

    #[test]
    fn it_should_handle_conversion_roundtrip() {
        let original_value = 42.5;
        let aggregate_value = AggregateValue::from(original_value);
        let converted_back: f64 = aggregate_value.into();
        assert_relative_eq!(original_value, converted_back);
    }
}
