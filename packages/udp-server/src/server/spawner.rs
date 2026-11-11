//! The UDP server spawner: the address a UDP server binds to.
use std::net::SocketAddr;

use derive_more::Constructor;
use derive_more::derive::Display;
// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(
    clippy::redundant_field_names,
    reason = "derive_more::Constructor emits field initializers on this MSRV-compatible version"
)]
#[derive(Constructor, Copy, Clone, Debug, Display)]
#[display("(with socket): {bind_to}")]
pub struct Spawner {
    pub bind_to: SocketAddr,
}
