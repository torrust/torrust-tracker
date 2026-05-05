// Copied from aquatic_udp_protocol 0.9.0 by Joakim Frostegård (greatest-ape).
// Source:     https://crates.io/crates/aquatic_udp_protocol/0.9.0
// Repository: https://github.com/greatest-ape/aquatic
// License:    Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//
// This is a verbatim internal fork. Modifications will be applied in subsequent migration steps.
// Pedantic lints are suppressed to preserve the original code unchanged in Step 1.
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

pub mod common;
pub mod request;
pub mod response;

pub use self::common::*;
pub use self::request::*;
pub use self::response::*;
