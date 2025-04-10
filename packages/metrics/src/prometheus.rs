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
