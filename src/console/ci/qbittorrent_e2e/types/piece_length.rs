/// The piece length for a torrent, in bytes.
///
/// Distinct from [`PayloadSize`] to prevent an accidental swap of the two
/// `usize` torrent-construction arguments.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PieceLength(usize);

impl PieceLength {
    /// Creates a new [`PieceLength`] from a byte count.
    pub(crate) const fn new(bytes: usize) -> Self {
        Self(bytes)
    }

    /// Returns the piece length as a `usize`.
    #[must_use]
    pub(crate) fn as_usize(self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::PieceLength;

    #[test]
    fn it_should_round_trip_piece_length() {
        let piece_length = PieceLength::new(262_144);

        assert_eq!(piece_length.as_usize(), 262_144);
    }
}
