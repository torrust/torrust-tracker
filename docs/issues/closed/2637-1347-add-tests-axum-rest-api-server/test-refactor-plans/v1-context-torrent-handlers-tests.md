---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs
    - packages/axum-rest-api-server/src/v1/context/torrent/responses.rs
    - packages/axum-rest-api-server/src/test_support.rs
    - packages/rest-api-application/src/v1/ports/torrent.rs
    - packages/rest-api-application/src/v1/use_cases/torrent.rs
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/test-refactor-plans/v1-context-whitelist-handlers-tests.md
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
| `QueryParams` deserialization (`empty_string_as_none`, `default`, the `info_hash` rename) | 3A unit | R2: read through `axum_extra::extract::Query::try_from_uri`, the extractor the router uses. Cases: absent and empty `offset` and `limit` are `None`; `0`, a normal value and `u32::MAX` are `Some`; `-1`, `1.1`, a word, a space and `u32::MAX + 1` are rejected; two `info_hash` parameters are collected in order; an empty `info_hash=` is collected as an empty string. `empty_string_as_none` itself is also tested directly with JSON (empty string, number in a string, `null`, absent, a word, an overflow, and a number that is not a string). |
| The text of a rejected query | Not here | `axum_extra` and `serde` own the wording; the integration test `should_fail_getting_torrents_when_the_offset_query_parameter_cannot_be_parsed` pins it end to end. The unit cases assert only that the value is rejected. |
| `QueryParamError`'s `Display` | Not tested | Nothing prints it: the handler matches the only variant and builds the response from the string. A test of the `thiserror` message would add no behavior. The enum is `pub`, so the message is public API of the crate, a candidate for the maintainer's call. |
| Tracing of the pagination (`tracing::debug!`) | Not tested | A log line without an observer; the line runs in every list test. |
| Torrent data and ordering of results | Collaborator-owned | tracker-core and the runtime adapter, covered by the integration tests. |

### Observation for the record, not a finding

Written after the mutation run: the empty-string arm of `empty_string_as_none` is not reachable through the real query
extractor. `axum_extra`'s `Query` already reads an empty `offset=` as absent, so the mutation T11 survived the query
cases and the plan's expectation that they would kill it was wrong. The arm is a second line of defense for other
deserializers, so it is tested directly with a JSON probe struct that uses the function. The same probe covers a value
that is not a string at all (the `?` on the underlying deserializer error).

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
| R2 | DONE | Add the in-file `#[cfg(test)]` module with the fake and the cases above. | Focused tests, `cargo +nightly fmt --all -- --check`, `cargo clippy`, `git diff --check`, prose-first AAA review, and the named mutations. |
| R3 | DONE | Integration increment: `none` expected. | The 14 torrent integration tests keep protecting the wire contract. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results below and `coverage-evidence.md`. |

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
| T10 | `get_torrents_handler` | Report the rejected string upper-cased in the `400` text. | the invalid-hash-after-valid and two-invalid cases |
| T11 | `empty_string_as_none` | Drop the empty-string arm. | the direct JSON case with an empty string (the query cases cannot kill it, see below) |
| T12 | `QueryParams` | Remove `#[serde(default)]` from `offset`. | the absent-`offset` cases |
| T13 | `QueryParams` | Remove `rename = "info_hash"`. | the `info_hash` cases |

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_reject_an_invalid_info_hash_without_calling_the_service_when_getting_a_torrent` | A recording fake that would answer a torrent; the path parameter `abc`. | `get_torrent_handler`. | The whole response is the `400` with `abc`, and the call log is empty. | Kept. |
| `it_should_answer_the_torrent_and_forward_the_parsed_info_hash_to_the_service_when_getting_a_torrent` | A fake that answers a torrent; a valid hash. | `get_torrent_handler`. | The response is the torrent JSON and the log is the one call with the parsed hash. | Response and forwarding are one behavior of one call, asserted as one value. Kept. |
| `it_should_answer_the_documented_not_known_text_with_ok_when_the_torrent_is_not_known` | A fake that has no torrent. | `get_torrent_handler`. | `200`, JSON, `"torrent not known"` and the one call. | Its doc comment names the module documentation, so the odd `200` reads as the contract. Kept. |
| `it_should_serve_a_page_of_torrents_when_no_info_hash_is_given` (5) | A fake that answers two items; a query string with none, empty, one or both pagination values. | `get_torrents_handler` with the query read by the real extractor. | The list JSON and the single page call with the literal pagination. | The expected `Pagination` is in each case, so the default is visible. Kept. |
| `it_should_serve_the_given_torrents_in_order_and_ignore_the_pagination_when_info_hashes_are_given` | A fake; three hashes (the first two differ, the last repeats the second) and an offset and a limit. | `get_torrents_handler`. | The list JSON and one `Torrents` call with the hashes in the given order. | The first version used a palindrome order, so the reverse-order mutation T9 survived; the hashes are now not a palindrome. Kept. |
| `it_should_reject_the_first_invalid_info_hash_without_calling_the_service_when_listing_torrents` (3) | A recording fake; an invalid hash after a valid one, two invalid ones, an empty one. | `get_torrents_handler`. | The `400` with the first invalid string, and an empty log. | Kept. |
| `it_should_read_an_empty_or_absent_pagination_value_as_not_set` (5) | A query string only; no fake. | Read the query with the extractor. | The pair `(offset, limit)` equals the literal. | Kept. |
| `it_should_reject_a_pagination_value_that_is_not_an_unsigned_32_bit_number` (7) | A query string with one bad value. | Read the query with the extractor. | The extractor rejects it; the message is not asserted (owned by `axum_extra`). | Kept. |
| `it_should_collect_every_info_hash_parameter_in_order` (3) | A query string with two, an empty and no hashes. | Read the query with the extractor. | The collected strings equal the literal list. | Kept. |
| `it_should_read_an_empty_string_as_not_set_and_reject_what_is_not_a_number` (7) | A JSON object with one `value` field read by a probe struct that uses the function. | `serde_json::from_str`. | `Ok` with the expected `Option<u32>`, or an error. | Kept. |

Helpers: `invalid_info_hash` builds the literal expected `400`; `a_torrent`, `no_torrent` and `two_items` are the scripted
outcomes; `FakeTorrentQueryPort`, `state_with`, `param` and `query` are the Arrange mechanics; `recorded` reads the typed
call log; the shared `observed` and `Observed` come from `test_support`.

## Results

- **Unit-only coverage** (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`, clean workspace): `torrent/handlers.rs` 21 / 50
  lines at the S4 baseline; 194 / 194 lines (100.00%), 248 / 248 regions and 30 / 30 functions after (the counts include
  the test module). The row 13 file `torrent/responses.rs` is now 9 / 9 lines through the "not known" test. Package
  unit-only lines after this file: 1,844 / 1,999 (92.25%), regions 2,766 / 3,070 (90.10%), functions 313 / 345 (90.72%),
  140 unit tests.
- **Tests:** 10 test functions, 34 test cases counting the tables; the package went from 106 to 140 unit tests. No
  integration test was added or changed.
- **Mutations** (each applied alone, stable toolchain, then reverted; the file was identical to the tested one
  afterwards): T1 killed by the invalid-hash test, T2 by the unknown-torrent test, T3 by the two forwarding tests, T4 by
  the known-torrent test, T5 by the page and invalid-hash cases, T6 by the three pagination cases that set a value, T7
  and T9 by the order test, T8 and T10 by the invalid-hash cases, T11 by the direct JSON case, T12 and T13 by the
  deserialization and list cases. Corrections made while writing: T9 survived at first because the three hashes of the
  order test formed a palindrome, so they were changed; T11 survived because the real extractor already reads an empty
  value as absent, so the direct JSON test was added (see the observation); T2 and T7 first failed to compile (an unused
  import or variable under `unused = "deny"`) and were redone as mutants that drop the original value; one more case (a
  JSON number instead of a string) covers the `?` of the underlying deserializer error, the only uncovered region.
- **Dependency:** none new. **Clippy allows:** one `#[allow(clippy::unnecessary_wraps, reason = ...)]` on `a_torrent`.
  The first clippy run also asked to rename the `Call` variants (all started with `Get`) and to avoid an
  `Option<Option<u32>>` expectation; both were changed.
- **Validation:** `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy -p
  torrust-tracker-axum-rest-api-server --all-targets` (stable), `cargo machete --with-metadata`, `git diff --check` and
  `cargo test -p torrust-tracker-axum-rest-api-server` (140 unit and 58 integration tests) pass.
- **Review outcome:** the maintainer approved the plan (asked on 2026-11-03, answered on 2026-11-04) and the completed file on 2026-11-04.
