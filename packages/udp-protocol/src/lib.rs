// Copied from aquatic_udp_protocol 0.9.0 by Joakim Frostegard (greatest-ape).
// Source:     https://crates.io/crates/aquatic_udp_protocol/0.9.0
// Repository: https://github.com/greatest-ape/aquatic
// License:    Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//
// This in-house crate started from the aquatic 0.9.0 sources that were previously vendored
// under packages/aquatic-udp-protocol and packages/aquatic-peer-id.
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::legacy_numeric_constants)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::wildcard_imports)]

pub mod announce;
pub mod common;
pub mod connect;
mod peer_id;
pub mod request;
pub mod response;

pub use self::announce::*;
pub use self::common::*;
pub use self::connect::*;
pub use self::peer_id::{PeerClient, PeerId};
pub use self::request::*;
pub use self::response::*;
