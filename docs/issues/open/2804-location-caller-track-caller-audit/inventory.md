---
doc-type: issue-inventory
issue-spec: docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md
last-updated-utc: "2026-11-10 21:51"
---

# Inventory of `Location::caller()` and `#[track_caller]` (tasks T1 to T3)

Snapshot of `torrust/develop` on 2026-11-10 (toolchain: stable `rustc 1.99.1`). It covers every `.rs`
file in `packages/`, `src/`, `console/` and `contrib/`, and the eight standalone `torrust-*` registry
crates of `Cargo.lock` (see [Standalone crates](#standalone-crates-in-the-dependency-tree)). Line
numbers are those of this snapshot.

## Method

A throwaway script listed every line matching `Location::caller`, `#[track_caller]`, `Located(` or
`(Arc::new(..) as DynError).into()`, skipped comments, and marked a line as test code when it is
under `tests/` or after a `#[cfg(test)]` module. The enclosing `fn` was found by looking upwards.
Each production row was then classified by hand. The script is not kept: the table is the result.

Counts: 86 matching lines. 58 are production: 38 `Location::caller()`, 3 `#[track_caller]` and
17 `.into()` conversions into a `LocatedError`. 28 are test code: 26 `Location::caller()` and 2
`.into()`. (The spec's "68 lines" counted one comment, the 6 lines of `asserts.rs` and no `.into()` lines.)

## Experiments (what `Location::caller()` returns)

Programs, commands, toolchain and raw output are in [`experiments.md`](experiments.md). `L` below
means the line the call is written on.

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
  (`FromStr`, `TryFrom`, `From`), so the location says nothing about the caller and cannot be fixed
  by `#[track_caller]` alone. Helper functions are class D, not W.
- **T (test only)**: in test code.

### Production sites with `#[track_caller]` (3)

| File:line                                 | What                             | Class | Note                                                                    |
| ----------------------------------------- | -------------------------------- | ----- | ----------------------------------------------------------------------- |
| `configuration/src/lib.rs:348`            | `From<figment::Error> for Error` | C     | Used with `?` in the configuration loader; the `?` line is reported     |
| `tracker-core/src/databases/error.rs:112` | `From<(SqlxError, Driver)>`      | C     | Reported line is the `.map_err(\|e\| (e, DRIVER))?` site in each driver |
| `tracker-core/src/databases/error.rs:139` | `From<(MigrateError, Driver)>`   | C     | Same                                                                    |

Note on the databases sites: the `?`/`.into()` caller is inside an `async fn` of a driver, and the line written inside an `async fn` body is the real line of that code (E8 reports the line of
the body); only the _attribute_ is a no-op there.

### Production sites with `Location::caller()` (38)

"Shape" says what stands between the site and a meaningful caller: `trait impl` (called through
`parse`, `try_into` or a generic bound, E6), `closure` (the site is written inside a closure, E3),
`async` (the site is inside an `async fn`, E8), or `plain` (a synchronous function, E2).

| #   | Package, file:line                                                                                                                                              | Enclosing fn                                                                                               | Class | Shape                                                                         | Reason                                                                                      |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ----- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| P1  | `primitives/src/peer.rs:73`                                                                                                                                     | `FromStr for PeerRole::from_str`                                                                           | W     | trait impl                                                                    | Reached through `str::parse` (D1 evidence)                                                  |
| P2  | `primitives/src/peer.rs:391`, `:397`                                                                                                                            | `TryFrom<Vec<u8>> for Id::try_from`                                                                        | W     | trait impl                                                                    | Reached through `try_into` or generic code                                                  |
| P3  | `http-protocol/src/v1/query.rs:176`                                                                                                                             | `FromStr for NameValuePair::from_str`                                                                      | W     | trait impl                                                                    | Called from `parse`                                                                         |
| P4  | `http-protocol/src/v1/requests/announce.rs:258`, `:312`                                                                                                         | `FromStr for Event`, `FromStr for Compact`                                                                 | W     | trait impl                                                                    | Called from `parse`                                                                         |
| P5a | `http-protocol/src/v1/requests/announce.rs:548`, `:564`, `:578`                                                                                                 | `extract_info_hash`, `extract_peer_id`, `extract_port` (`None` arms)                                       | D     | plain; reached only through `TryFrom<Query> for Announce` (`announce.rs:334`) | Private helper; `#[track_caller]` would stop at the `TryFrom` impl (E6)                     |
| P5b | `http-protocol/src/v1/requests/announce.rs:575`, `:604`, `:611`, `:647`                                                                                         | `extract_port`, `extract_number_of_bytes_from_param`, `extract_numwant`                                    | D     | closure (`map_err`, `map_or`)                                                 | The site is inside a closure, so `#[track_caller]` on the helper would not change it (E3)   |
| P6  | `http-protocol/src/v1/requests/scrape.rs:93`                                                                                                                    | `extract_info_hashes`                                                                                      | D     | plain; reached only through `TryFrom<Query> for Scrape` (`scrape.rs:64`)      | Same as P5a                                                                                 |
| P7  | `http-protocol/src/v1/services/peer_ip_resolver.rs:86`, `:96`                                                                                                   | `try_client_ip_from_connection_info`, `try_client_ip_from_proxy_header`                                    | D     | plain (`const fn`)                                                            | Fixed line                                                                                  |
| P8  | `axum-http-server/src/v1/extractors/announce_request.rs:66`                                                                                                     | `extract_announce_from`                                                                                    | D     | plain                                                                         | Fixed line in the extractor                                                                 |
| P9  | `axum-http-server/src/v1/extractors/scrape_request.rs:66`                                                                                                       | `extract_scrape_from`                                                                                      | D     | plain                                                                         | Same                                                                                        |
| P10 | `axum-http-server/src/v1/extractors/authentication_key.rs:109`, `:118`, `:122`                                                                                  | `parse_key`, `custom_error`                                                                                | D     | plain                                                                         | Same                                                                                        |
| P11 | `http-core/src/services/announce.rs:308`                                                                                                                        | `authenticate`                                                                                             | D     | async                                                                         | Fixed line inside an `async fn`; the attribute would be a no-op                             |
| P12 | `tracker-core/src/whitelist/authorization.rs:61`                                                                                                                | `authorize`                                                                                                | D     | async                                                                         | Same                                                                                        |
| P13 | `tracker-core/src/authentication/service.rs:98`                                                                                                                 | `verify_auth_key`                                                                                          | D     | async                                                                         | Same                                                                                        |
| P14 | `tracker-core/src/authentication/key/mod.rs:158`                                                                                                                | `verify_key_expiration` (sync, public)                                                                     | D     | closure (`map_or_else`, line 155)                                             | The site is inside a closure, so `#[track_caller]` on the function would not change it (E3) |
| P15 | `tracker-core/src/databases/driver/{sqlite,mysql,postgres}/whitelist_store.rs:59`, `:82`/`:83`; `auth_key_store.rs:82`-`:84`, `:105`-`:108` (12 sites in total) | `add_info_hash_to_whitelist`, `remove_info_hash_from_whitelist`, `add_key_to_keys`, `remove_key_from_keys` | D     | async (trait impl methods)                                                    | Fixed line inside async trait-impl methods                                                  |
| P16 | `axum-server/src/tls.rs:43`                                                                                                                                     | `make_rust_tls`                                                                                            | D     | async                                                                         | Fixed line                                                                                  |

That is 38 lines: P1 (1), P2 (2), P3 (1), P4 (2), P5a (3), P5b (4), P6 (1), P7 (2), P8 (1), P9 (1),
P10 (3), P11 (1), P12 (1), P13 (1), P14 (1), P15 (12), P16 (1).

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

## T2 - Which locations reach production output

"Reaches" means that the error's `Display` is formatted into something that leaves the process or is
written to a log. Checked by reading the producers and the formatting code at `4a46f98b2`, not by
running the server (that is the job of the fix slices, M1 and M2 of the spec). Each row names the
producers (rows of the tables above, plus the class-C conversion `databases/error.rs:112` and the
standalone crates), then the formatting line, then the audience. Class C rows are listed because a
`#[track_caller]` chain that ends at a line in a library driver is correct for the chain and still
shows a library path to a user.

| Error (location in `Display`)                                                                                                        | Producers                                                                                                                                                                      | Output path (formatting line)                                                                                                                                                                                                                                                                                                                                                               | Audience                            |
| ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------- |
| `ParseQueryError::InvalidParam`                                                                                                      | P3                                                                                                                                                                             | `From<ParseQueryError>`: `"Bad request. Cannot parse query params: {err}"` (`requests/announce.rs:318-323`)                                                                                                                                                                                                                                                                                 | HTTP tracker client                 |
| `ParseAnnounceQueryError::*`                                                                                                         | P4, P5a, P5b, P8 (`MissingParams`)                                                                                                                                             | `"... for announce request: {err}"` (`announce.rs:329`)                                                                                                                                                                                                                                                                                                                                     | HTTP tracker client                 |
| `ParseScrapeQueryError::*`                                                                                                           | P6, P9 (`MissingParams`)                                                                                                                                                       | `"... for scrape request: {err}"` (`scrape.rs:59`)                                                                                                                                                                                                                                                                                                                                          | HTTP tracker client                 |
| `auth::Error::{InvalidKeyFormat, CannotExtractKeyParam}`                                                                             | P10                                                                                                                                                                            | `"Tracker authentication error: {err}"` (`responses/error.rs:54`)                                                                                                                                                                                                                                                                                                                           | HTTP tracker client                 |
| `PeerIpResolutionError::*`                                                                                                           | P7                                                                                                                                                                             | `"Error resolving peer IP: {err}"` (`responses/error.rs:62`)                                                                                                                                                                                                                                                                                                                                | HTTP tracker client                 |
| `authentication::key::Error::{MissingAuthKey, UnableToReadKey, KeyExpired}`                                                          | P11 (`MissingAuthKey`), P13 (`UnableToReadKey`), P14 (`KeyExpired`)                                                                                                            | `"Tracker authentication error: {source}"` (`http-core/src/services/error_mapping.rs`). Never REST: the REST key handlers receive `PeerKeyError` or database errors (`rest-api-runtime-adapter/.../adapters/auth_key.rs:74-85`). UDP has no key authentication                                                                                                                              | HTTP tracker client                 |
| `WhitelistError::TorrentNotWhitelisted`                                                                                              | P12                                                                                                                                                                            | HTTP: returned by `authorize` (`http-core/src/services/announce.rs:213`, `From<WhitelistError>` at `:403`), formatted `"Tracker whitelist error: {source}"` (`error_mapping.rs:13`). UDP announce: `handle_announce` calls `authorize` at `udp-core/src/services/announce.rs:109`, then `ErrorResponse { message: error.to_string() }` (`udp-server/src/handlers/error.rs:57-59`). Not REST | HTTP and UDP tracker clients        |
| `databases::error::Error::{InsertFailed, DeleteFailed}`                                                                              | P15 (12 sites). Built only in the driver add/remove methods, called only from `whitelist/repository/persisted.rs:36,52` and `authentication/key/repository/persisted.rs:43,57` | REST: `failed to whitelist torrent: {e}` and the other messages in `whitelist/responses.rs`; for keys, `PeerKeyError::DatabaseError` mapped at `adapters/auth_key.rs:85` into `failed to add key: {e}`. Never UDP                                                                                                                                                                           | REST API client                     |
| `databases::error::Error::{QueryReturnedNoRows, ConnectionError, InvalidQuery, ...}` through the class-C `From<(SqlxError, Driver)>` | Every `.map_err(\|e\| (e, DRIVER))?` in the drivers, for example `load_torrent_downloads` (`sqlite/torrent_metrics_store.rs:69`)                                               | The `LocatedError` source prints the driver line. UDP: reached when `core.tracker_policy.persistent_torrent_completed_stat` loads downloads from the database, through `UdpAnnounceError`/`UdpScrapeError` into `error.to_string()`. REST: `failed to reload ...` and the add/remove messages above. This is the path through which class C reaches clients                                 | UDP tracker client, REST API client |
| `PeerKeyError::{DatabaseError, InvalidKey}` sources                                                                                  | `authentication/handler.rs:103`, `:108`, `:120`, `:125`, `:141`, `:150` (`Located(..).into()`, class D)                                                                        | `map_peer_key_error`: `AuthKeyError::Database(source.to_string())` and `reason: source.to_string()` (`adapters/auth_key.rs:80-84`), then `failed to add key: {e}` and the generate messages in `auth_key/responses.rs`. These carry the handler line plus the inner database location                                                                                                       | REST API client                     |
| `torrust_info_hash::ConversionError::{NotEnoughBytes, TooManyBytes}` (standalone crate, class W)                                     | `torrust-info-hash` 0.2.0 `info_hash.rs:273`, `:279`, reached through `percent_decode_info_hash` (`http-protocol/src/percent_encoding.rs:55-58`)                               | Wrapped in `ParseAnnounceQueryError::InvalidInfoHashParam` / `ParseScrapeQueryError::InvalidInfoHashParam` with `Located(err).into()` (`announce.rs:543`, `scrape.rs:84`), then the HTTP failure reasons above: the response then holds two locations, one inside a registry crate                                                                                                          | HTTP tracker client                 |
| `ErrorKind::Whitelist(String)` event payload                                                                                         | P12 through `source.to_string()` (`udp-server/src/event.rs:135`, `:145`)                                                                                                       | Event consumers (statistics, logs)                                                                                                                                                                                                                                                                                                                                                          | Operators, event consumers          |
| `axum_server::tls::Error::{MissingTlsConfig, BadTlsConfig}`, `configuration::Error` sources                                          | P16, class-C configuration impl                                                                                                                                                | Startup failure printed by the process. The configuration location is correct there                                                                                                                                                                                                                                                                                                         | Operator                            |
| `ParsePeerRoleError`, `peer::IdConversionError`                                                                                      | P1, P2                                                                                                                                                                         | No production path found: `ParsePeerRoleError` and `IdConversionError` are not used outside `primitives/src/peer.rs`                                                                                                                                                                                                                                                                        | Library consumers only              |

Not checked, to be covered by the fix slices: metrics labels (none of the `Display` strings were
found in label code), and CLI output of `console/` (`grep -rn location console` finds no use).

Result: **a library path reaches a user on at least five surfaces**: the HTTP tracker failure
reason, the UDP error message, the REST API error text, event payload strings and logs. It is not
only developer-facing. Class C does not protect users: the database conversion reaches UDP and REST
clients with a driver line.

### Standalone crates in the dependency tree

`Cargo.lock` at `4a46f98b2` lists eight registry `torrust-*` crates. Scanned in `~/.cargo/registry/src`
with `grep -rn 'Location::caller\|track_caller' --include=*.rs`:

| Crate                                                                                                                                                        | Matches | Result                                                                                                                                               |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `torrust-info-hash` 0.2.0                                                                                                                                    | 2       | `src/info_hash.rs:273`, `:279`: `Location::caller()` in `TryFrom<Vec<u8>> for InfoHash` (class W, same shape as P2). Reaches HTTP clients (T2 table) |
| `torrust-located-error` 3.0.0                                                                                                                                | 8       | `src/lib.rs:89`, `:102`: `#[track_caller]` on both `Into` impls (class C). The other lines are the doc example and a test helper                     |
| `torrust-bencode` 3.0.0, `torrust-clock` 3.0.0, `torrust-metrics` 0.2.0, `torrust-net-primitives` 0.1.0, `torrust-peer-id` 0.1.0, `torrust-server-lib` 0.5.0 | 0       | No use                                                                                                                                               |

A fix for `torrust-info-hash` belongs in its own repository, so T9 must open an issue there; the
workspace slices cannot change it.

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

- Class counts for planning: W rows (P1 to P4, `key/mod.rs:198` and `torrust-info-hash`) are the certain bugs; D rows are
  the audit's real question, because their location is correct as a detection site and wrong only
  where it reaches a user (T2 table).
- The first fix slice (primitives) is confirmed. The second (`http-protocol` and the extractors)
  has the largest client-visible exposure. The REST API exposure (not in the original spec text) is a third path that the REST slices must cover, and the class-C database conversion reaches UDP and REST clients, so the `tracker-core` slice must change it too, not only the class-D driver sites.
- T4 (options) now has evidence for the recommendation: option 3 (context instead of location) for
  everything that reaches a client, since options 1 and 2 cannot fix P1 to P4 (E6, `str::parse`), P5a and P6 (reached only through a `TryFrom` impl, E6), P5b and P14 (the site is in a closure, E3)
  or P11 to P13, P15 and P16 (async, E8).
