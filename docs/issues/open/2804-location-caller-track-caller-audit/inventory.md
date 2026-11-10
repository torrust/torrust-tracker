---
doc-type: issue-inventory
issue-spec: docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md
last-updated-utc: "2026-11-10 17:49"
---

# Inventory of `Location::caller()` and `#[track_caller]` (tasks T1 to T3)

Snapshot of `torrust/develop` on 2026-11-10 (toolchain: stable `rustc 1.99.1`). It covers every `.rs`
file in `packages/`, `src/`, `console/` and `contrib/`. The standalone crate `torrust-located-error`
3.0.0 is covered by the experiments below. Line numbers are those of this snapshot.

## Method

A throwaway script listed every line matching `Location::caller`, `#[track_caller]`, `Located(` or
`(Arc::new(..) as DynError).into()`, skipped comments, and marked a line as test code when it is
under `tests/` or after a `#[cfg(test)]` module. The enclosing `fn` was found by looking upwards.
Each production row was then classified by hand. The script is not kept: the table is the result.

Counts: 86 matching lines. 58 are production: 38 `Location::caller()`, 3 `#[track_caller]` and
17 `.into()` conversions into a `LocatedError`. 28 are test code: 26 `Location::caller()` and 2
`.into()`. (The spec's "68 lines" counted one comment, the 6 lines of `asserts.rs` and no `.into()` lines.)

## Experiments (what `Location::caller()` returns)

Single-file experiment compiled with `rustc 1.99.1 --edition 2021`, and a throwaway test in
`packages/configuration` for `torrust-located-error` (both deleted). `L` is the line of the call
site named in the first column.

| #   | Shape                                                          | Reported location                    | Meaning                                            |
| --- | -------------------------------------------------------------- | ------------------------------------ | -------------------------------------------------- |
| E1  | `#[track_caller] fn`, called from `main`                       | the call line in `main`              | correct                                            |
| E2  | non-`#[track_caller]` helper that calls a `#[track_caller] fn` | the line inside the helper           | the chain is broken at the helper                  |
| E3  | closure calling a `#[track_caller] fn`                         | the line inside the closure          | the closure body, not the closure's caller         |
| E4  | `map(\|_\| loc())`                                             | the line inside the closure          | same as E3                                         |
| E5  | trait method with `#[track_caller]` on the impl, direct call   | the call line                        | correct                                            |
| E6  | the same method called through a generic bound `X: Trait`      | the line inside the generic function | the chain breaks at the generic function           |
| E7  | `#[track_caller] const fn`                                     | the call line                        | correct                                            |
| E8  | `#[track_caller] async fn`                                     | the line inside the `async fn`       | no-op: rustc warns `ungated_async_fn_track_caller` |
| E9  | `Located(e).into()` written in `fn main`-like code             | that line                            | correct                                            |
| E10 | `Located(e).into()` written inside a helper fn                 | the line inside the helper           | the location is the helper's                       |
| E11 | `(Arc::new(e) as DynError).into()` inside a helper fn          | the line inside the helper           | same, `DynError` conversion                        |
| E12 | `Located(e).into()` inside a closure                           | the line inside the closure          | same as E3                                         |

Consequences: `#[track_caller]` helps only for a synchronous chain in which every function is
`#[track_caller]` and the last link is called by code that is itself a meaningful call site.
It cannot help through `str::parse` (see `manual-verification-evidence.md`, D1), generic bounds
(E6), closures (E3) or `async fn` (E8). A `LocatedError` written inline in the function that
detects the error carries that function's line, which is a source location of the _detection_, not
of the caller (E10, E11).

## Classification

Classes:

- **C (correct)**: a `#[track_caller]` chain reaches a call site that means something to the reader.
- **D (detection site)**: the location is a fixed line in the library, correct as "where the library
  detected this" but not as "who called it".
- **W (wrong)**: the error is created in a trait impl or conversion whose callers are anonymous
  (`FromStr`, `TryFrom`, `From`), or in a helper, so the location says nothing about the caller and
  cannot be fixed by `#[track_caller]` alone.
- **T (test only)**: in test code.

### Production sites with `#[track_caller]` (3)

| File:line                                 | What                             | Class | Note                                                                    |
| ----------------------------------------- | -------------------------------- | ----- | ----------------------------------------------------------------------- |
| `configuration/src/lib.rs:348`            | `From<figment::Error> for Error` | C     | Used with `?` in the configuration loader; the `?` line is reported     |
| `tracker-core/src/databases/error.rs:112` | `From<(SqlxError, Driver)>`      | C     | Reported line is the `.map_err(\|e\| (e, DRIVER))?` site in each driver |
| `tracker-core/src/databases/error.rs:139` | `From<(MigrateError, Driver)>`   | C     | Same                                                                    |

Note on the databases sites: the `?`/`.into()` caller is inside an `async fn` of a driver, and the
line inside an `async fn` body is correct (E9); only the _attribute_ is a no-op there (E8).

### Production sites with `Location::caller()` (38)

| #   | Package, file:line                                                                                                                                              | Enclosing fn                                                                                                                   | Class | Reason                                                                   |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | ----- | ------------------------------------------------------------------------ |
| P1  | `primitives/src/peer.rs:73`                                                                                                                                     | `FromStr for PeerRole::from_str`                                                                                               | W     | Reached through `str::parse` (D1 evidence)                               |
| P2  | `primitives/src/peer.rs:391`, `:397`                                                                                                                            | `TryFrom<Vec<u8>> for Id::try_from`                                                                                            | W     | Reached through `try_into`/generic code                                  |
| P3  | `http-protocol/src/v1/query.rs:176`                                                                                                                             | `FromStr for NameValuePair::from_str`                                                                                          | W     | Called from `parse`                                                      |
| P4  | `http-protocol/src/v1/requests/announce.rs:258`, `:312`                                                                                                         | `FromStr for Event`, `FromStr for Compact`                                                                                     | W     | Called from `parse`                                                      |
| P5  | `http-protocol/src/v1/requests/announce.rs:548`, `:564`, `:575`, `:578`, `:604`, `:611`, `:647`                                                                 | `extract_info_hash`, `extract_peer_id`, `extract_port`, `extract_number_of_bytes_from_param`, `extract_numwant`                | D     | Private sync helpers; the location is the helper line, never the request |
| P6  | `http-protocol/src/v1/requests/scrape.rs:93`                                                                                                                    | `extract_info_hashes` (in `TryFrom<Query>`)                                                                                    | D     | Same, private helper                                                     |
| P7  | `http-protocol/src/v1/services/peer_ip_resolver.rs:86`, `:96`                                                                                                   | `try_client_ip_from_connection_info`, `try_client_ip_from_proxy_header`                                                        | D     | `const fn` helpers; fixed line                                           |
| P8  | `axum-http-server/src/v1/extractors/announce_request.rs:66`                                                                                                     | `extract_announce_from`                                                                                                        | D     | Fixed line in the extractor                                              |
| P9  | `axum-http-server/src/v1/extractors/scrape_request.rs:66`                                                                                                       | `extract_scrape_from`                                                                                                          | D     | Same                                                                     |
| P10 | `axum-http-server/src/v1/extractors/authentication_key.rs:109`, `:118`, `:122`                                                                                  | `parse_key`, `custom_error`                                                                                                    | D     | Same                                                                     |
| P11 | `http-core/src/services/announce.rs:308`                                                                                                                        | `authenticate` (async)                                                                                                         | D     | Fixed line inside an `async fn`; attribute would be a no-op              |
| P12 | `tracker-core/src/whitelist/authorization.rs:61`                                                                                                                | `authorize` (async)                                                                                                            | D     | Same                                                                     |
| P13 | `tracker-core/src/authentication/service.rs:98`                                                                                                                 | `verify_auth_key` (async)                                                                                                      | D     | Same                                                                     |
| P14 | `tracker-core/src/authentication/key/mod.rs:158`                                                                                                                | `verify_key_expiration` (sync, public)                                                                                         | D     | Could be `#[track_caller]`; its callers are `async fn`s                  |
| P15 | `tracker-core/src/databases/driver/{sqlite,mysql,postgres}/whitelist_store.rs:59`, `:82`/`:83`; `auth_key_store.rs:82`-`:84`, `:105`-`:108` (12 sites in total) | `add_info_hash_to_whitelist`, `remove_info_hash_from_whitelist`, `add_key_to_keys`, `remove_key_from_keys` (async trait impls) | D     | Fixed line inside async trait-impl methods                               |
| P16 | `axum-server/src/tls.rs:43`                                                                                                                                     | `make_rust_tls` (async)                                                                                                        | D     | Fixed line                                                               |

That is 38 lines: P1 (1), P2 (2), P3 (1), P4 (2), P5 (7), P6 (1), P7 (2), P8 (1), P9 (1), P10 (3),
P11 (1), P12 (1), P13 (1), P14 (1), P15 (12), P16 (1).

### Production `.into()` conversions into `LocatedError` (17)

| Package, file:line                                                                                | Enclosing fn                             | Class | Note                                                                                      |
| ------------------------------------------------------------------------------------------------- | ---------------------------------------- | ----- | ----------------------------------------------------------------------------------------- |
| `configuration/src/lib.rs:351`, `tracker-core/src/databases/error.rs:118`, `:127`, `:131`, `:144` | the `#[track_caller]` `From` impls above | C     | Covered by the three attributes                                                           |
| `tracker-core/src/authentication/key/mod.rs:198`                                                  | `From<sqlx::Error> for Error::from`      | W     | A `From` impl without `#[track_caller]`: the location is that line, whatever the `?` site |
| `configuration/src/lib.rs:258`                                                                    | `attach_explicit_config_path`            | D     | Plain method; fixed line                                                                  |
| `axum-server/src/tls.rs:53`                                                                       | `make_rust_tls` (async)                  | D     | Inline in a `map_err` closure                                                             |
| `http-protocol/src/v1/requests/announce.rs:543`, `:560`; `scrape.rs:84`                           | `extract_*` helpers                      | D     | Inline in `map_err` closures                                                              |
| `tracker-core/src/authentication/handler.rs:103`, `:108`, `:120`, `:125`, `:141`, `:150`          | `add_peer_key` (async)                   | D     | Inline in the handler                                                                     |

The table names all 17: 5 in the `#[track_caller]` impls (class C), 1 W and 11 D.

### Test code (28)

26 `Location::caller()` and 2 `.into()` lines in `#[cfg(test)]` modules and `tests/`: in
`http-core` (`error_mapping.rs`, `announce.rs`, `scrape.rs`), `http-protocol/.../responses/error.rs`,
`tracker-core` (`error.rs`, `statistics/event/handler.rs`, `authentication/handler.rs`),
`udp-server` (`event.rs`, `handlers/error.rs`) and `axum-http-server/tests/server/asserts.rs`.
Class T. They construct error values for assertions; they change with the production types.

## T2 - Which wrong or detection-site locations reach production output

"Reaches" means that the error's `Display` is formatted into something that leaves the process or is
written to a log. Checked by reading the formatting code, not by running the server (that is the job
of the fix slices, M1 and M2 of the spec).

| Error type (location in `Display`)                                                                   | Origin sites | Output path (the formatting line)                                                                                                                                                                                                                                         | Audience                                 |
| ---------------------------------------------------------------------------------------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| `ParseQueryError::InvalidParam` (`invalid param {raw_param} in {location}`)                          | P3           | `From<ParseQueryError>` into `responses::error::Error` (`failure_reason: "Bad request. Cannot parse query params: {err}"`), bencoded to the BitTorrent client                                                                                                             | HTTP client                              |
| `ParseAnnounceQueryError::*`, `ParseScrapeQueryError::*` (`missing param ... in {location}`, others) | P4-P6        | `failure_reason: "... for announce request: {err}"` and the scrape equivalent                                                                                                                                                                                             | HTTP client                              |
| `auth::Error::*` (`... Error in {location}`)                                                         | P10          | `From<auth::Error>`: `"Tracker authentication error: {err}"`                                                                                                                                                                                                              | HTTP client                              |
| `PeerIpResolutionError::*` (`... in {location}`)                                                     | P7           | `From<PeerIpResolutionError>`: `"Error resolving peer IP: {err}"`                                                                                                                                                                                                         | HTTP client                              |
| `ParsePeerRoleError`, `peer::IdConversionError`                                                      | P1, P2       | Not formatted into a response in the production code read for this inventory; they surface only where a caller formats `{e}`                                                                                                                                              | Library consumers                        |
| `WhitelistError::TorrentNotWhitelisted` (`..., {location}`)                                          | P12          | Announce: `ErrorResponse { message: error.to_string() }` (`udp-server/src/handlers/error.rs:57-59`); HTTP announce: tracker-core error wrapped by `authenticate`; REST: `format!("failed to whitelist torrent: {e}")` (`axum-rest-api-server/.../whitelist/responses.rs`) | UDP client, HTTP client, REST API client |
| `authentication::key::Error::*` (`Key has expired, {location}`, ...)                                 | P13, P14     | HTTP announce through the authentication path; REST: `format!("failed to add key: {e}")` and the other messages in `auth_key/responses.rs`                                                                                                                                | HTTP client, REST API client             |
| `databases::error::Error::{InsertFailed, DeleteFailed, ...}` (`..., {location}`)                     | P15          | REST: `format!("failed to add key: {e}")`, `failed to whitelist torrent: {e}`, `failed to reload ...`; UDP when `persistent_torrent_completed_stat` loads from the database                                                                                               | REST API client, UDP client              |
| `axum_server::tls::Error::BadTlsConfig` (`LocatedError` source)                                      | P16          | Startup failure, printed by the process: operator-facing                                                                                                                                                                                                                  | Operator                                 |
| `LocatedError` sources of `configuration::Error`                                                     | C rows       | Startup failure, operator-facing; the location is correct here                                                                                                                                                                                                            | Operator                                 |

Not checked, to be covered by the fix slices: metrics labels (none of the `Display` strings were
found in label code), event payloads (`udp-server/src/event.rs` stores `source.to_string()` as a
string in `ErrorKind::Whitelist`, so the location can reach an event consumer and a log), CLI output
of `console/` (no `location` use found with `grep -rn location console`).

Result: **at least five production surfaces show a library path to a user**: the HTTP tracker
failure reason, the UDP error message, the REST API error text, the event payload string, and
(by the same `Display`) logs. It is not only developer-facing.

## T3 - Guidance and enforceability

- **Skill** `.github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md` recommends the
  `located-error` package ("Consider `located-error` for diagnostics-rich errors") and says nothing
  about `#[track_caller]`, about keeping locations out of user-facing `Display`, or about the
  chain-breaking shapes E2 to E12. Its own advice (clarity, context, actionability) argues against
  a `Display` that ends in a library path.
- **ADRs**: none about located errors. `20261005145329_return_result_only_for_concretely_fallible_public_apis.md`
  covers `#[non_exhaustive]` on public error enums, which matters for the fix options.
- **Lint configuration**: the workspace `Cargo.toml` denies `clippy::all`, `pedantic`, `nursery`
  (warn) and the usual rustc groups. There is no `clippy.toml`.
- **Enforcement options**: rustc has the lints `ungated_async_fn_track_caller` (warns on the
  attribute on an `async fn`, already active through `warnings = "deny"`) and `tail_call_track_caller`.
  Neither clippy 1.99.1 nor rustc has a lint for "`Location::caller()` in a function without
  `#[track_caller]`", so a lint is not available. A repository test or a `grep`-based check can
  enforce a narrower rule: for example that `Location::caller()` appears only in an allow-list of
  files, or never inside an `impl FromStr|TryFrom|From`. That is T9's design question.

## What this changes in the spec

- Class counts for planning: W rows (P1 to P4 and `key/mod.rs:198`) are the certain bugs; D rows are
  the audit's real question, because their location is correct as a detection site and wrong only
  where it reaches a user (T2 table, first six rows).
- The first fix slice (primitives) is confirmed. The second (`http-protocol` and the extractors)
  has the largest client-visible exposure. The REST API exposure (not in the original spec text) is a
  third path that `axum-rest-api-server` slices must cover.
- T4 (options) now has evidence for the recommendation: option 3 (context instead of location) for
  everything that reaches a client, since options 1 and 2 cannot fix P1 to P4 (E6, `str::parse`)
  or P11 to P16 (E8).
