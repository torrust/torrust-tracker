#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NumberOfBytes(pub i64);

impl NumberOfBytes {
    #[must_use]
    pub const fn new(v: i64) -> Self {
        Self(v)
    }
}
