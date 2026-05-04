use torrust_tracker_primitives::DurationSinceUnixEpoch;

pub trait PrometheusSerializable {
    /// Convert the implementing type into a Prometheus exposition format string.
    ///
    /// # Returns
    ///
    /// A `String` containing the serialized representation.
    fn to_prometheus(&self) -> String;
}

// Blanket implementation for references
impl<T: PrometheusSerializable> PrometheusSerializable for &T {
    fn to_prometheus(&self) -> String {
        (*self).to_prometheus()
    }
}

pub trait PrometheusDeserializable: Sized {
    /// Parse a Prometheus exposition text format string into `Self`.
    ///
    /// `now` is used as the sample timestamp when the exposition text does not
    /// include a timestamp for a given sample.
    ///
    /// # Errors
    ///
    /// Returns an error if the input cannot be parsed or contains unsupported
    /// or unknown metric types/values.
    fn from_prometheus(input: &str, now: DurationSinceUnixEpoch) -> Result<Self, PrometheusDeserializationError>;
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum PrometheusDeserializationError {
    /// The Prometheus text could not be parsed at all (syntax error).
    #[error("Failed to parse Prometheus exposition text: {message}")]
    ParseError { message: String },

    /// The parser emitted a metric type that is syntactically valid but that
    /// this implementation does not yet support (e.g. Histogram, Summary).
    #[error("Unsupported Prometheus metric type '{metric_type}' for metric '{metric_name}'")]
    UnsupportedType { metric_name: String, metric_type: String },

    /// The parser emitted a metric type that is not recognised at all.
    #[error("Unknown Prometheus metric type for metric '{metric_name}'")]
    UnknownType { metric_name: String },

    /// The value in the exposition does not match the declared metric type.
    #[error("Value mismatch for metric '{metric_name}': expected {expected_type}, got {actual}")]
    ValueMismatch {
        metric_name: String,
        expected_type: String,
        actual: String,
    },

    /// The value is of an unknown/unrecognised kind.
    #[error("Unknown value for metric '{metric_name}'")]
    UnknownValue { metric_name: String },

    /// The label set could not be converted (e.g. invalid label name or value).
    #[error("Failed to convert label set for metric '{metric_name}': {message}")]
    LabelConversion { metric_name: String, message: String },

    /// A structural error when assembling collections from parsed data.
    #[error("Failed to build collection data: {message}")]
    CollectionError { message: String },
}
