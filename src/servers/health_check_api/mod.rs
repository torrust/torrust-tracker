pub mod handlers;
pub mod resources;
pub mod responses;
pub mod server;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Version {
    /// The `v0` i.e un-versioned version of the Health Check.
    V0,
}
