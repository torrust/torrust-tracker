/// A number of bytes: what a peer has uploaded, downloaded, or has left to
/// download.
///
/// A number of bytes cannot be negative, so the type holds a `u64` and cannot
/// represent a negative count. Each delivery protocol refuses a negative value
/// when it converts its own wire type into this one.
///
/// ```rust
/// use torrust_tracker_primitives::NumberOfBytes;
///
/// let left = NumberOfBytes::new(16_384);
///
/// assert_eq!(left.get(), 16_384);
/// ```
// adr: docs/adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NumberOfBytes(u64);

impl NumberOfBytes {
    #[must_use]
    pub const fn new(v: u64) -> Self {
        Self(v)
    }

    #[must_use]
    pub const fn get(&self) -> u64 {
        self.0
    }
}
