// Copied from aquatic_udp_protocol 0.9.0 by Joakim Frostegard (greatest-ape).
// Source:     https://crates.io/crates/aquatic_udp_protocol/0.9.0
// Repository: https://github.com/greatest-ape/aquatic
// License:    Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//
// This in-house crate started from the aquatic 0.9.0 sources that were previously vendored
// under packages/aquatic-udp-protocol and packages/aquatic-peer-id.
// `FromBytes` derives expand to empty helper enums on current nightly Clippy.
// The protocol wire structs are inhabited and cannot use Clippy's suggested replacement.
#![allow(
    clippy::empty_enums,
    reason = "FromBytes derives generate empty helper enums for inhabited protocol wire structs"
)]
// `zerocopy-derive` 0.8.63 generates `ẕ<field>` identifiers with the span of the user's field names.
// Nightly rustc with Unicode 18 (rust-lang/rust#165289) reports them as `uncommon_codepoints`,
// and rustc only honours this lint's level at crate level (rust-lang/rust#86824).
// Remove this allow when a `zerocopy` release no longer generates non-ASCII identifiers.
// Issue: https://github.com/torrust/torrust-tracker/issues/2931
#![allow(
    uncommon_codepoints,
    reason = "zerocopy-derive generates identifiers with a non-ASCII prefix that nightly rustc flags"
)]

pub mod announce;
pub mod common;
pub mod connect;
pub mod request;
pub mod response;
pub mod scrape;

pub use self::announce::*;
pub use self::common::*;
pub use self::connect::*;
pub use self::request::*;
pub use self::response::*;
pub use self::scrape::*;
