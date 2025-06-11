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
