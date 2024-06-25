pub mod check;
pub mod handle;
pub mod launcher;
pub mod v0;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Version {
    /// The `v0` i.e un-versioned version of the Health Check.
    V0,
}
