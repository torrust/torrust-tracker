# Error Enum Inventory (#2454, task T1)

Spec: [ISSUE.md](ISSUE.md). Policy: [ADR 20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md)
and the Pre-publish API checklist in [EPIC #1669](../1669-overhaul-packages/EPIC.md).

Inventory taken at `torrust/develop` `55e08830f` (2026-10-19), before any code change.

## Method

A public error enum is an enum that is reachable from a crate's public API and either implements
`std::error::Error`, is the `E` of a public `Result`, or has `Error` in its name. The name is only
a way to find candidates, never the criterion.

1. Generated rustdoc JSON for every library target with the nightly Rust toolchain
   (`rustc 1.101.0-nightly (7df81381d 2026-10-14)`: `cargo +nightly doc --workspace --no-deps --lib --all-features`, `RUSTDOCFLAGS="-Z unstable-options
   --output-format json"`, a scratch target directory), then read it with a script that:
   - lists every enum of the crate that implements `Error`, is the `E` of a `Result` in a function
     signature, or has `Error` in its name;
   - follows `pub` items and `pub use` re-exports from the crate root, to decide public
     reachability (every listed enum below is reachable);
   - reads each enum's variant count, its trait implementations (derives), and whether it already
     has `#[non_exhaustive]`.
2. Cross-checked against `rg '^\s*pub(\([a-z]+\))? enum \w*Error'` over `packages/`, `console/`,
   `src/`, and `contrib/`: every name-based hit is in the rustdoc list, except the binary-only
   `DiscoveryError` recorded in "Not in scope".
3. Found downstream exhaustive matches with a throw-away experiment on the stable Rust toolchain
   (`rustc 1.99.0 (b940084d7 2026-09-28)`): marked every listed enum `#[non_exhaustive]`, ran `cargo check --workspace --all-targets
   --all-features` repeatedly, added a temporary wildcard arm at each `E0004` error, and reverted
   the experiment. The sites below are the complete set; the real arms come with each crate's commit.

Result: 60 enums in 18 publishable packages: 58 to mark `#[non_exhaustive]` and 2 to keep exhaustive
(maintainer decision 9, see "Exhaustive enums"). The 2026-10-06 hypothesis of 61 matches the 61 name-based
hits in publishable packages today; the difference is `ErrorKind`, listed under "Not in scope". No enum has `#[non_exhaustive]` today. No
enum is in a private module or binary-only code: every enum below is reachable from its library
crate's root. All 60 have at least one real variant.

Packages with no public error enum, so nothing to mark: `torrust-tracker-swarm-coordination-registry`,
`torrust-tracker-rest-api-application`, `torrust-tracker-rest-api-runtime-adapter`,
`torrust-tracker-test-helpers`.

## Not in scope

| Item | Reason |
| ---- | ------ |
| `github_review_threads::Error` (`contrib/dev-tools/github/github-review-threads`) | `publish = []`: a developer tool, not published (maintainer decision 1). |
| `DiscoveryError` (`contrib/dev-tools/checks/frontmatter-validator`, binary module) | `publish = []` and binary-only code (maintainer decisions 1 and 2). |
| `torrust_tracker_udp_server::event::ErrorKind` | Matches the name filter but is not an error type: it does not implement `Error`, and no public `Result` returns it. It classifies an error for UDP events and metric labels. See review question R1. |

## Per-package inventory, in dependency order

Dependency order is the T3 commit order: a crate comes before its dependents. Columns: the path is
relative to the crate root; "Derives" lists derives other than `Debug` and `Display`;
"impl Error" says whether the enum implements `std::error::Error`; "Downstream matches" counts the
exhaustive `match` sites in other crates that fail with `E0004` once the enum is `#[non_exhaustive]`
(the S-ids are in "Downstream exhaustive matches").
The decision is `mark` for every row except the two rows that say `keep exhaustive`.

### `torrust-tracker-primitives` (published, 3.0.0)

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `peer::IdConversionError` | 2 | - | yes | no | none | mark |
| `peer::ParsePeerRoleError` | 1 | - | yes | no | none | mark |

### `torrust-tracker-configuration` (published, 3.0.0)

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `validator::SemanticValidationError` | 1 | - | yes | yes | none | mark |
| `Error` | 8 | - | yes | yes | none | mark; remove the `Infallible` variant in T4 |

### `torrust-tracker-axum-server`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `tls::Error` | 2 | - | yes | yes | none | mark |

### `torrust-tracker-axum-health-check-api-server`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `server::Error` | 3 | - | yes | yes | none | mark |
| `environment::Error` | 1 | - | no | yes | none | mark |

### `torrust-tracker-events`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `receiver::RecvError` | 2 | `Clone`, `PartialEq`, `Eq` | yes | yes | none (E1: 6 sites avoided) | keep exhaustive (E1) |

### `torrust-tracker-core`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `authentication::key::Error` | 4 | `Clone` | yes | yes | none | mark |
| `authentication::key::peer_key::ParseKeyError` | 2 | - | yes | yes | none | mark |
| `container::Error` | 2 | - | yes | yes | none | mark |
| `error::PeerKeyError` | 3 | `Clone` | yes | yes | 1 (S2) | mark |
| `error::AnnounceError` | 2 | `Clone` | yes | yes | 1 (S3) | mark |
| `databases::error::Error` | 9 | `Clone` | yes | yes | none | mark |
| `error::ScrapeError` | 2 | `Clone` | yes | yes | 1 (S3) | mark |
| `statistics::persisted::Error` | 2 | `Clone` | yes | yes | none | mark |
| `error::TrackerCoreError` | 4 | `Clone` | yes | no | 1 (S1) | mark |
| `error::WhitelistError` | 1 | `Clone`, `PartialEq`, `Eq` | yes | yes | none | mark |

### `torrust-tracker-http-protocol`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `v1::query::ParseQueryError` | 1 | - | yes | no | none | mark |
| `v1::auth::Error` | 2 | - | yes | no | none | mark |
| `v1::responses::scrape::deserialization::BencodeParseError` | 5 | - | yes | yes | none | mark |
| `v1::requests::scrape::ParseScrapeQueryError` | 3 | - | yes | no | none | mark |
| `v1::services::peer_ip_resolver::PeerIpResolutionError` | 2 | `Clone` | yes | yes | none | mark |
| `v1::requests::announce::ParseAnnounceQueryError` | 7 | - | yes | yes | none | mark |
| `percent_encoding::PeerIdConversionError` | 2 | `Clone`, `PartialEq`, `Eq` | yes | yes | none | mark |

### `torrust-tracker-http-core`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `services::announce::HttpAnnounceError` | 5 | `Clone` | yes | yes | none | mark |
| `services::scrape::HttpScrapeError` | 2 | `Clone` | yes | yes | none | mark |

### `torrust-tracker-axum-http-server`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `server::Error` | 6 | - | yes | yes | none | mark |

### `torrust-tracker-rest-api-protocol`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `v1::context::whitelist::resources::whitelist::WhitelistError` | 2 | - | yes | no | none | mark |
| `v1::context::auth_key::resources::auth_key::AuthKeyError` | 4 | - | yes | no | 1 (S4) | mark |

### `torrust-tracker-rest-api-client`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `connection_info::OriginError` | 3 | - | yes | yes | none | mark |
| `v1::client::ClientError` | 4 | - | yes | yes | none | mark |

### `torrust-tracker-udp-protocol`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `request::RequestParseError` | 2 | - | no | yes | none (E2: 1 site avoided) | keep exhaustive (E2) |

### `torrust-tracker-udp-core`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `services::scrape::UdpScrapeError` | 3 | `Clone` | yes | yes | 1 (S3) | mark |
| `connection_cookie::ConnectionCookieError` | 3 | `Clone`, `PartialEq` | yes | yes | none | mark |
| `services::announce::UdpAnnounceError` | 3 | `Clone` | yes | yes | 1 (S3) | mark |

### `torrust-tracker-client-lib`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `udp::Error` | 14 | `Clone` | yes | yes | none | mark |
| `http::client::Error` | 3 | `Clone` | yes | yes | none | mark |

### `torrust-tracker-udp-server`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `server::UdpError` | 5 | - | yes | yes | none | mark |
| `error::Error` | 5 | `Clone` | yes | no | none | mark |
| `server::processor::ProcessorError` | 2 | - | yes | yes | none | mark |

### `torrust-tracker-axum-rest-api-server`

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `v1::context::torrent::handlers::QueryParamError` | 1 | - | yes | no | none | mark |
| `server::Error` | 6 | - | yes | yes | none | mark |

### `torrust-tracker` (root crate, published, 3.0.0; library target `torrust_tracker_lib`)

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `console::profiling::Error` | 1 | - | yes | yes | none | mark |
| `bootstrap::jobs::http_tracker::Error` | 2 | - | yes | yes | none | mark |
| `bootstrap::jobs::udp_tracker::Error` | 1 | - | yes | yes | none | mark |
| `bootstrap::persistence::PersistenceRequirementError` | 4 | `PartialEq`, `Eq` | yes | yes | none | mark |
| `bootstrap::jobs::tracker_apis::Error` | 2 | - | yes | yes | none | mark |
| `container::Error` | 3 | - | yes | yes | none | mark |
| `bootstrap::jobs::health_check_api::Error` | 1 | - | yes | yes | none | mark |
| `app::Error` | 5 | - | yes | yes | none | mark |
| `bootstrap::app::Error` | 4 | - | yes | yes | none | mark |
| `bootstrap::config::Error` | 2 | - | yes | yes | none | mark |
| `bootstrap::jobs::tracker_core::Error` | 1 | - | yes | yes | none | mark |

### `torrust-tracker-client` (`console/tracker-client`; library target `torrust_tracker_console_client`)

| Enum | Variants | Derives | impl Error | `E` of public `Result` | Downstream matches | Decision |
| ---- | -------- | ------- | ---------- | ---------------------- | ------------------ | -------- |
| `console::clients::checker::checks::health::Error` | 3 | `Clone`, `Serialize` | yes | no | none | mark |
| `console::clients::unified::app::Error` | 2 | - | no | yes | 1 (S5) | mark |
| `console::clients::http::Error` | 4 | `Clone`, `Serialize` | yes | no | none | mark |
| `console::clients::checker::error::AppError` | 2 | - | no | yes | none | mark |
| `console::clients::udp::Error` | 10 | `Clone`, `Serialize` | yes | yes | none | mark |
| `console::clients::checker::config::ConfigurationError` | 3 | - | yes | yes | none | mark |

## Exhaustive enums (maintainer decision 9)

Two enums stay exhaustive because their variants select different recovery actions. A wildcard arm
there would silently choose an action for a variant nobody designed for, so a new variant must be a
deliberate breaking change that every consumer sees at compile time. Each gets a doc comment saying
why, so that nobody adds the attribute later. The rule is in the amendment of
[ADR 20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md).

| ID | Enum | Why its variants select an action | Sites that would have needed a wildcard |
| -- | ---- | --------------------------------- | --------------------------------------- |
| E1 | `torrust_tracker_events::receiver::RecvError` | `Closed` ends a listener loop, `Lagged` keeps it running. A wrong guess for a new variant either stops a listener that should run or spins on a terminal error. | Six receive loops: `handle_receive_error` in `swarm-coordination-registry` and `tracker-core` (statistics listeners); `should_stop_after_receiving_event` in `http-core` and `udp-core`; `handle_received_event` in `udp-server` (statistics) and `udp-server` (banning). |
| E2 | `torrust_tracker_udp_protocol::request::RequestParseError` | `Sendable` carries the ids needed to reply to the client, `Unsendable` means the server must drop the request. A new variant must say which. | `From<RequestParseError> for SendableRequestParseError` in `udp-server`. |

The experiment found no other enum whose downstream `match` chooses an action; the remaining eight
sites below only report or convert.

## Downstream exhaustive matches and proposed wildcard handling

Each site fails with `E0004` once its enum is `#[non_exhaustive]`. The arm goes in the same commit
as the enum, so every commit builds. Each arm maps to the most specific handling that already exists
at that site, and neither adds a variant nor changes a message. Function names, not line numbers,
identify the sites, so this table stays valid as the files change. The maintainer confirmed S1 to
S5 at the T1 review (see "Review outcome").

| ID | Enum | Site | Wildcard handling |
| -- | ---- | ---- | ----------------- |
| S1 | `core::error::TrackerCoreError` | `protocol_error_from_tracker_core_error` in `http-core` | `failure_reason: format!("Tracker error: {error}")`, the same shape as the existing arms. |
| S2 | `core::error::PeerKeyError` | `map_peer_key_error` in `rest-api-runtime-adapter` | `AuthKeyError::Database(error.to_string())`, the only generic failure variant `AuthKeyError` has. |
| S3 | `core::error::AnnounceError`, `core::error::ScrapeError`, `udp-core::UdpAnnounceError`, `udp-core::UdpScrapeError` | Four nested matches in `From<Error> for ErrorKind` in `udp-server` | `Self::InternalServer(error.to_string())`, the existing generic `ErrorKind` variant. |
| S4 | `rest-api-protocol::AuthKeyError` | `add_auth_key_handler` in `axum-rest-api-server` | `failed_to_add_key_response(AuthKeyErrorDisplay(&err))`, the handling of the `Database` variant. |
| S5 | `console unified::app::Error` | `main` of the `tracker_client` binary | Report it as a `runtime_failure`, like `Error::Other`, with `format!("{error:?}")` as the message (the enum has no `Display`). |

## Derive findings (checklist item 2)

Checklist item 2 asks for derives every future variant can keep. Every enum below already derives
only traits its current payloads satisfy; what is at risk is a future variant whose payload cannot.
No derive change is proposed; changing one needs the maintainer's approval.

| Derive | Enums | Risk for a future variant |
| ------ | ----- | ------------------------- |
| `Clone` (22 enums) | `events::RecvError`; 8 in `tracker-core`; 2 in `http-protocol`; 2 in `http-core`; 3 in `udp-core`; 2 in `client-lib`; `udp-server::error::Error`; 3 in `console/tracker-client` | A variant wrapping a non-`Clone` source (for example `std::io::Error`) forces `Arc` or a string. |
| `PartialEq`, `Eq` | `events::RecvError`, `core::error::WhitelistError`, `http-protocol::PeerIdConversionError`, root `PersistenceRequirementError`; `udp-core::ConnectionCookieError` derives `PartialEq` only | A variant wrapping a float or a non-`PartialEq` source breaks the derive. |
| `Serialize` | three enums in `console/tracker-client` | The enum is the JSON error contract of the console client; a new variant also extends that contract. |

## Placeholder findings (checklist item 3)

- `torrust_tracker_configuration::Error::Infallible` ("The error for errors that can never happen.")
  is never constructed and never matched: its declaration in `packages/configuration/src/lib.rs` is
  the only `Infallible` in `packages/`, `console/`, `src/`, and `contrib/`. The maintainer decided to
  remove it in its own T4 commit after the configuration crate gains `#[non_exhaustive]`.
- No public API returns `Result<_, Infallible>`, and no public enum is empty (every enum above has at
  least one variant; the rustdoc scan found no variant-less enum in any crate).

## Publish bump per crate (AC6)

Four workspace crates are already on crates.io, all at 3.0.0 (the latest version published on
2024-10-03, read from the crates.io API on 2026-10-19). Every other package is unpublished (the same
API returns no crate for them), so `#[non_exhaustive]` is not a breaking change for them and their
first publish carries it.

| Crate | On crates.io | Gains `#[non_exhaustive]` | Next publish | Path that applies it |
| ----- | ------------ | ------------------------- | ------------ | -------------------- |
| `torrust-tracker` | 3.0.0 | yes (11 enums) | semver-major bump | the tracker application release |
| `torrust-tracker-configuration` | 3.0.0 | yes (2 enums) | semver-major bump | the per-package publish |
| `torrust-tracker-primitives` | 3.0.0 | yes (2 enums) | semver-major bump | the per-package publish |
| `torrust-tracker-test-helpers` | 3.0.0 | no (no public error enum) | none required by this issue | not applicable |

## Per-crate completion

Updated in each T3 commit. `TODO` until the crate's commit lands. `Marked` counts enums that gain
`#[non_exhaustive]`; `Kept exhaustive` counts enums that get a doc comment instead (E1, E2).

| Order | Package | Marked | Kept exhaustive | Status |
| ----- | ------- | ------ | --------------- | ------ |
| 1 | `torrust-tracker-primitives` | 2 | 0 | DONE: `feat(primitives)!: [#2454] mark public error enums non_exhaustive` |
| 2 | `torrust-tracker-configuration` | 2 | 0 | DONE: `feat(configuration)!: [#2454] mark public error enums non_exhaustive` |
| 3 | `torrust-tracker-axum-server` | 1 | 0 | DONE: `feat(axum-server)!: [#2454] mark public error enums non_exhaustive` |
| 4 | `torrust-tracker-axum-health-check-api-server` | 2 | 0 | DONE: `feat(axum-health-check-api-server)!: [#2454] mark public error enums non_exhaustive` |
| 5 | `torrust-tracker-events` | 0 | 1 (E1) | DONE: `docs(events): [#2454] document why RecvError stays exhaustive` |
| 6 | `torrust-tracker-core` | 10 | 0 | DONE: `feat(tracker-core)!: [#2454] mark public error enums non_exhaustive` |
| 7 | `torrust-tracker-http-protocol` | 7 | 0 | TODO |
| 8 | `torrust-tracker-http-core` | 2 | 0 | TODO |
| 9 | `torrust-tracker-axum-http-server` | 1 | 0 | TODO |
| 10 | `torrust-tracker-rest-api-protocol` | 2 | 0 | TODO |
| 11 | `torrust-tracker-rest-api-client` | 2 | 0 | TODO |
| 12 | `torrust-tracker-udp-protocol` | 0 | 1 (E2) | TODO |
| 13 | `torrust-tracker-udp-core` | 3 | 0 | TODO |
| 14 | `torrust-tracker-client-lib` | 2 | 0 | TODO |
| 15 | `torrust-tracker-udp-server` | 3 | 0 | TODO |
| 16 | `torrust-tracker-axum-rest-api-server` | 2 | 0 | TODO |
| 17 | `torrust-tracker` | 11 | 0 | TODO |
| 18 | `torrust-tracker-client` | 6 | 0 | TODO |

T4 (remove `configuration::Error::Infallible`): TODO.

## Review outcome

The maintainer reviewed this inventory on 2026-10-19 and approved it. His answers:

- **R1**: `udp_server::event::ErrorKind` is out of scope (it is not an error type) and stays unchanged.
- **R2**: for `RecvError` the maintainer did not want a wildcard that picks stop or continue, and
  decided that an enum whose variants select a recovery action stays exhaustive. See
  "Exhaustive enums" (E1) and the ADR amendment.
- **R3**: an unknown `PeerKeyError` maps to `AuthKeyError::Database(error.to_string())` (S2).
- **R4**: S1 and S3 to S5 are confirmed as proposed. `RequestParseError` also stays exhaustive (E2),
  which removed the former `udp-server` wildcard site for it.
