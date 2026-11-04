---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs
status: in-progress
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs
    - packages/axum-rest-api-server/src/v1/context/torrent/responses.rs
    - packages/axum-rest-api-server/src/test_support.rs
    - packages/rest-api-application/src/v1/ports/torrent.rs
    - packages/rest-api-application/src/v1/use_cases/torrent.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-whitelist-handlers-tests.md
---

# Torrent Handlers File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs`, ledger row 12 (rank 5), the first file of S4. It
reuses the pattern approved for the auth-key and whitelist handlers ([whitelist plan](v1-context-whitelist-handlers-tests.md)):
the handlers are called directly, over a hand-written fake of the application port, and the response is read with the
shared `observed` helper of `test_support`.

## Current State

- **Unit-only coverage:** 21 / 50 lines (42.00%), 20 / 67 regions, 4 / 7 functions (stable Rust 1.99.1,
  `cargo-llvm-cov 0.6.16`, clean workspace, `--lib`, S4 baseline). The `v1/routes.rs` tests only reach the router
  registration. The uncovered lines are the whole bodies of the two handlers, `parse_info_hashes` and
  `empty_string_as_none`.
- **Existing tests:** none in the file. `tests/server/v1/contract/context/torrent.rs` has 14 integration tests that go
  through a real server and the real tracker: list, limit, pagination, list by info hashes, an unparsable `offset`
  and `limit`, an invalid info hash, a torrent and its peers, an unknown torrent, an invalid info hash for one torrent,
  and the three unauthenticated cases.
- **Module-owned decisions:**
  - `get_torrent_handler` parses the info hash in the path (invalid: `400`), asks the service for that hash and answers
    the torrent as JSON, or the legacy `200` with the JSON string `"torrent not known"` (documented in the module
    docs) when the service has none.
  - `get_torrents_handler` serves a page (offset and limit forwarded, absent values left to the default pagination)
    when no `info_hash` parameter is given, and otherwise serves exactly those torrents, in the order given and
    ignoring `offset` and `limit`; the first invalid hash gives the `400`.
  - `QueryParams` reads `offset` and `limit` with `empty_string_as_none` (an empty value is the same as an absent one)
    and collects every `info_hash` parameter.
  - Owned elsewhere: whether an info hash is valid (`torrust-info-hash`), the default and the maximum page size
    (`torrust-tracker-primitives` `Pagination`), the text of a rejected query (`axum_extra`), the shape of `Torrent` and
    `ListItem` (`rest-api-protocol`) and the torrent data (tracker-core through the runtime adapter).
- **T1 hypothesis:** test candidate, weak property.

## Maintainer Decisions

The maintainer approved the plan (asked on 2026-11-03, answered on 2026-11-04) with three decisions: pin the documented `200` "torrent not known" answer
(not a finding), no property test (boundary examples only, no `quickcheck`), and no test of `QueryParamError`'s
`Display`.

## Current Tests Review

No tests in the file, so R1 is skipped.

## Coverage And Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| `get_torrent_handler`: invalid info hash | 3A unit | R2: `400`, plain text, the literal message with the given string; the fake records no call. |
| `get_torrent_handler`: known torrent | 3A unit | R2: `200`, `application/json`, the literal JSON of the torrent (no `peers` key when it has none); the fake receives the parsed hash. |
| `get_torrent_handler`: unknown torrent | 3A unit | R2: the documented legacy answer, `200`, `application/json`, body `"torrent not known"`; this also covers row 13's `torrent_not_known_response`. |
| `get_torrents_handler`: no `info_hash` parameter | 3A unit | R2: a table over the pagination the fake receives: no parameters give `Pagination::default()`, `offset` only and `limit` only keep the other at its default, both given are forwarded as given (offset 5, limit 2 are literals), and the page items are the JSON list. The page call is the only one made. |
| `get_torrents_handler`: `info_hash` parameters | 3A unit | R2: the fake receives `get_torrents` with the parsed hashes in the order given (including a repeated hash), `offset` and `limit` given together are ignored, and the list items are the JSON list. |
| `get_torrents_handler`: invalid hash among valid ones | 3A unit | R2: `400` with the first invalid string (an invalid hash after a valid one, and two invalid ones), and the fake records no call. |
| `QueryParams` deserialization (`empty_string_as_none`, `default`, the `info_hash` rename) | 3A unit | R2: read through `axum_extra::extract::Query::try_from_uri`, the extractor the router uses. Cases: absent and empty `offset` and `limit` are `None`; `0`, a normal value and `u32::MAX` are `Some`; `-1`, `1.1`, a word, a space and `u32::MAX + 1` are rejected; two `info_hash` parameters are collected in order; an empty `info_hash=` is collected as an empty string. |
| The text of a rejected query | Not here | `axum_extra` and `serde` own the wording; the integration test `should_fail_getting_torrents_when_the_offset_query_parameter_cannot_be_parsed` pins it end to end. The unit cases assert only that the value is rejected. |
| `QueryParamError`'s `Display` | Not tested | Nothing prints it: the handler matches the only variant and builds the response from the string. A test of the `thiserror` message would add no behavior. The enum is `pub`, so the message is public API of the crate, a candidate for the maintainer's call. |
| Tracing of the pagination (`tracing::debug!`) | Not tested | A log line without an observer; the line runs in every list test. |
| Torrent data and ordering of results | Collaborator-owned | tracker-core and the runtime adapter, covered by the integration tests. |

### Observation for the record, not a finding

An `info_hash=` parameter with an empty value is collected as an empty string and rejected as an invalid hash (`400`),
whereas an empty `offset=` or `limit=` means "not set". The two are different parameters and the behavior is
defensible, so it is pinned as it is, with one case in the deserialization table and one in the invalid-hash table.

### Cases for R2 (literal expectations)

| Handler | Situation | Expected status | Content type | Body |
| --- | --- | --- | --- | --- |
| get one | invalid hash `abc` | `400` | `text/plain; charset=utf-8` | `Invalid URL: invalid infohash param: string "abc", expected a 40 character long string` |
| get one | known torrent (hash `443c7602b4fde83d1154d6d9da48808418b181b6`, 1 seeder, 2 completed, 3 leechers, no peers) | `200` | `application/json` | `{"info_hash":"443c7602b4fde83d1154d6d9da48808418b181b6","seeders":1,"completed":2,"leechers":3}` |
| get one | unknown torrent | `200` | `application/json` | `"torrent not known"` |
| list | page of two items | `200` | `application/json` | `[{"info_hash":"...","seeders":..,"completed":..,"leechers":..},{...}]` |
| list | invalid hash after a valid one | `400` | `text/plain; charset=utf-8` | `Invalid URL: invalid infohash param: string "<the invalid one>", expected a 40 character long string` |

## Test Design

- **Fake:** a `FakeTorrentQueryPort` implementing `TorrentQueryPort`, with two scripted outcomes (function pointers
  returning `Option<Torrent>` and `Vec<ListItem>`, because `Torrent` and `ListItem` are not `Clone`) and an
  `Arc<Mutex<Vec<Call>>>` log of a test-local `Call` enum (`GetTorrentInfo(InfoHash)`, `GetTorrentsPage(Pagination)`,
  `GetTorrents(Vec<InfoHash>)`), typed as agreed in the review of #2702. `Pagination` already implements `PartialEq` and
  `Debug`.
- **Helpers:** the shared `Observed` and `observed` of `test_support`; the query values are read with
  `Query::<QueryParams>::try_from_uri(&uri)` (a literal URI per case), then passed to the handler as the router would.
- **Lint allows:** the scripted success outcomes may need `#[allow(clippy::unnecessary_wraps, reason = ...)]`, as in S3.

## Property-Test Decision

The ledger flags a weak property for `empty_string_as_none`: any `u32` rendered in `offset` is read back unchanged.
Not selected: the function is generic glue over `FromStr`, so the property tests the standard library. The examples
above include the boundaries (`0`, `u32::MAX`, `u32::MAX + 1`), the empty value and the rejected forms, which is the
package's own decision. No new dependency (no `quickcheck`).

## Dependencies

None new: `async-trait`, `rstest` and `axum-extra` (already a dependency, with the `query` feature) are available.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests in the file. | None. |
| R2 | TODO | Add the in-file `#[cfg(test)]` module with the fake and the cases above. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | TODO | Integration increment: `none` expected. | The 14 torrent integration tests keep protecting the wire contract. |
| R4 | TODO | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change a test must catch; revert it exactly after the run. A mutant that leaves an import
or variable unused does not compile under `unused = "deny"`, so such a mutant is redone so that it compiles.

| ID | Site | Temporary change | Expected killer |
| --- | --- | --- | --- |
| T1 | `get_torrent_handler` | Invalid-hash arm answers `torrent_not_known_response`. | the invalid-hash test |
| T2 | `get_torrent_handler` | Unknown torrent answers an empty list `[]`. | the unknown-torrent test |
| T3 | `get_torrent_handler` | Forward a different hash to the service. | the known-torrent forwarding assertion |
| T4 | `get_torrent_handler` | Known torrent answers `torrent_not_known_response`. | the known-torrent test |
| T5 | `get_torrents_handler` | Invert the `is_empty()` test. | the page and the hashes tests |
| T6 | `get_torrents_handler` | Swap offset and limit in `new_with_options`. | the pagination table |
| T7 | `get_torrents_handler` | Hashes branch calls `get_torrents_page`. | the hashes test |
| T8 | `parse_info_hashes` | Skip an invalid hash instead of returning the error. | the invalid-hash-among-valid tests |
| T9 | `parse_info_hashes` | Reverse the parsed hashes. | the order assertion |
| T10 | `get_torrents_handler` | Answer the first hash in the `400` text instead of the invalid one. | the invalid-hash-after-valid test |
| T11 | `empty_string_as_none` | Drop the empty-string arm. | the empty `offset` and `limit` cases |
| T12 | `QueryParams` | Remove `#[serde(default)]` from `offset`. | the absent-`offset` cases |
| T13 | `QueryParams` | Remove `rename = "info_hash"`. | the `info_hash` cases |

## Prose-First AAA Review

Written after the tests, before comparing them with the code.

## Results

Written after the tests.
