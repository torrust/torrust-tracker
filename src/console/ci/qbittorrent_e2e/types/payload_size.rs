/// The total byte size of a test payload used in the E2E torrent scenario.
///
/// Distinct from [`PieceLength`] to prevent an accidental swap of the two
/// `usize` torrent-construction arguments.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PayloadSize(usize);

impl PayloadSize {
    /// Creates a new [`PayloadSize`] from a byte count.
    pub(crate) const fn new(bytes: usize) -> Self {
        Self(bytes)
    }

    /// Returns the byte count as a `usize`.
    #[must_use]
    pub(crate) const fn as_usize(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::PayloadSize;

    #[test]
    fn it_should_round_trip_payload_size() {
        let size = PayloadSize::new(16_384);

        assert_eq!(size.as_usize(), 16_384);
    }
}
