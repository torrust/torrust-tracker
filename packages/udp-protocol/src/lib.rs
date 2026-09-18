// Copied from aquatic_udp_protocol 0.9.0 by Joakim Frostegard (greatest-ape).
// Source:     https://crates.io/crates/aquatic_udp_protocol/0.9.0
// Repository: https://github.com/greatest-ape/aquatic
// License:    Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//
// This in-house crate started from the aquatic 0.9.0 sources that were previously vendored
// under packages/aquatic-udp-protocol and packages/aquatic-peer-id.
#![allow(
    clippy::cast_possible_truncation,
    reason = "temporary: #2245 reviews numeric protocol wire conversion bounds"
)]
#![allow(
    clippy::default_trait_access,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
#![allow(
    clippy::doc_markdown,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
// `FromBytes` derives for transparent wire types expand to empty helper enums.
// Nightly Clippy reports those generated enums as `empty_enums`; the wire types
// themselves are inhabited and cannot use the suggested replacement.
#![allow(
    clippy::empty_enums,
    reason = "FromBytes derives expand to empty helper enums for inhabited transparent wire types"
)]
#![allow(
    clippy::explicit_iter_loop,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
#![allow(
    clippy::legacy_numeric_constants,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
#![allow(
    clippy::match_same_arms,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
#![allow(
    clippy::missing_errors_doc,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
#![allow(
    clippy::missing_panics_doc,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
#![allow(
    clippy::must_use_candidate,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
#![allow(
    clippy::needless_pass_by_value,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
#![allow(
    clippy::semicolon_if_nothing_returned,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
)]
#![allow(
    clippy::wildcard_imports,
    reason = "temporary: #2261 reviews the nonnumeric UDP protocol Clippy baseline"
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
