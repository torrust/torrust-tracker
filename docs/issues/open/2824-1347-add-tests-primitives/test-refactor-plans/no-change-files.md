---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/ (ledger rows 8-16)
status: done
last-updated-utc: "2026-11-15 07:48"
---

# No-Change Files Test Plan

Follow the shared [guidance](README.md). Ledger rows 8-16, slice S4, one shared record. Status: **done**: approved by
the maintainer on 2026-11-15 (first gate; Q1 yes, Q2 and Q3 as recommended) and the completed rows approved on
2026-11-15 (second gate).

## Current State

Unit-only coverage measured 2026-11-14 on `develop` after #2852 (package 1,049 / 1,126 lines, 105 unit tests). Every
file with executable lines is fully covered, so no group below is an uncovered line; the review is of the existing tests.

| Row | File | Lines | Tests | Review of the existing tests | Decision |
| ---: | --- | ---: | ---: | --- | --- |
| 8 | `scrape.rs` | 28 / 28 | 1 | `refactor`-light: the expectation is built with `add_file_with_zeroed_metadata`, a production path, so the test pins that `zeroed` and `add_file_with_zeroed_metadata` agree, not the value; the value is now pinned by the `SwarmMetadata::zeroed` test of row 7. `empty` and `add_file` are `HashMap::new` and `insert` delegations, reached through the same test. | No change (Q2). |
| 9 | `service_role.rs` | 40 / 40 | 3 | `clean`: one table per contract (`as_str`, `Display`, JSON), literals written in the test. The loop runs the same Act and Assert over a table of literals, with no other logic. `Deserialize` is unselected: a search found no code that names `ServiceRole` with a deserializer, a `from_str` or TOML. | No change. |
| 10 | `configuration_instance_id.rs` | 42 / 42 | 5 | `clean`, one smell: the JSON test compares a serialized **string** with `{"service_role":"http_tracker","instance_index":0}`, so it fails on a field reorder that changes nothing a reader sees. S3 replaced the same form with a comparison of `serde_json::Value`s after review. | Test changed (Q1). |
| 11 | `runtime_service_metadata.rs` | 32 / 32 | 2 | `clean`, minor: the first test asserts three accessors of one value built in one step; they describe one state, so one reason to fail. `with_public_url(None)` is not tested; it is an assignment. | No change. |
| 12 | `readiness.rs` | 52 / 52 | 5 | `clean`. Likely cause of the varying coverage seen in S3 (50 / 52 once, not reproduced since): the concurrent test's reader threads loop on `while reader.is_ready() { yield_now() }` (lines 105-106 of the **test** module). When the main thread marks the value before a reader starts, that loop body never runs and its two lines count as uncovered. Every production line is covered in every run, and the test result does not depend on the timing. | No change (Q3). |
| 13 | `driver.rs` | 59 / 59 | 13 | `clean`, style: one test per variant and contract (no AAA blocks; each test is one line of Act and one assert, so the blocks would add nothing). `Display` prints the variant names (`Sqlite3`, `MySQL`), unlike `as_str` and the JSON form (lowercase); the tests pin both as they are. The invalid-driver test asserts only the failure. `Deserialize` is collaborator-owned: the `configuration` tests load `driver = "mysql"` and `"postgresql"` from TOML. | No change. |
| 14 | `number_of_bytes.rs` | 3 / 3 | 0 | A `const fn new` around an `i64`; reached through other modules' tests. | No change (unselected: one assignment). |
| 15 | `peer_id.rs` | none | 0 | A deprecated re-export, no executable line. Removal is owned by EPIC #1669. | No change (wiring). |
| 16 | `lib.rs` | none | 0 | Module declarations, re-exports (some deprecated, owned by #1669) and two type aliases. | No change (wiring). |

No finding: nothing here is wrong behavior, and nothing is owned by an open issue other than the #1669 removals.

## Property-Test Decision

None. No file has an invariant over a range of inputs; the enums are closed sets, already tested per variant.

## Questions For The Maintainer

- **Q1: change the JSON test of `configuration_instance_id.rs`** to compare `serde_json::to_value` with a `json!`
  literal, as S3 did after review? It makes row 10 "test changed", with a Prose-First AAA row and one mutant (a field
  rename). Recommendation: yes, so the package uses one form for JSON objects. Alternative: no change, and the smell is
  recorded here only.
- **Q2: `scrape.rs` breadth.** Leave it, as recommended: with the `zeroed` value now pinned, a wider test would only
  repeat `HashMap` behavior. Alternative: one test that `zeroed` of two info hashes holds two zeroed entries.
- **Q3: `readiness.rs` coverage variance.** Leave the test, as recommended: the variance is in the test's own loop,
  not in a production line, and a fix (a barrier so every reader starts before the mark) would add machinery for a
  coverage figure only. The `lessons.md` row is answered by this record. Alternative: add the barrier.

## Results (2026-11-15)

Toolchains: stable Rust 1.99.1 for tests and coverage, `cargo-llvm-cov 0.6.16`, nightly `cargo fmt`.

| Measure | Before | After |
| --- | ---: | ---: |
| Unit tests of the package | 109 | 109 (one test changed, none added) |
| `configuration_instance_id.rs` lines | 42 / 42 | 43 / 43 (the assertion now spans more lines) |
| Package unit-only lines | 1,077 / 1,145 (94.06%) | 1,078 / 1,146 (94.07%) |
| Package unit-only regions | 1,698 / 1,782 (95.29%) | 1,702 / 1,786 (95.30%) |
| Package unit-only functions | 210 / 223 (94.17%) | 210 / 223 (94.17%) |

Applied as approved: Q1 changed `it_should_serialize_the_role_and_instance_index` to compare `serde_json::to_value` with
a `json!` literal (and `expect` in place of `unwrap`). Q2 and Q3 changed nothing; Q3 answers the `readiness.rs` row of
`lessons.md`. `readiness.rs` read 52 / 52 in both S4 runs.

### Mutation sample (3 mutants, each applied alone and reverted exactly)

| Mutant | Result | Killed by |
| --- | --- | --- |
| `instance_index` renamed in JSON (`#[serde(rename = "index")]`) | killed | the JSON test |
| `service_role` left out of the JSON (`#[serde(skip)]`) | killed | the JSON test |
| The two fields swapped in the struct | **survived, by design** | the JSON object is the same; the old string comparison failed on it. The derived `Ord` changes order (index before role). The identifiers are keys of the `metrics_policy` `BTreeMap`s in `http-core` and `udp-core`, which look them up by key; any total order consistent with `Eq` gives the same lookups, so the order stays unselected. |

`cmp` against a saved copy showed the file restored byte for byte after every mutant.

## Prose-First AAA Review (every changed test)

Temporary prose was written and compared with the code; the code expresses it.

| Test | Arrange (the one causal state) | Act | Assert (literal, independent) | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_serialize_the_role_and_instance_index` | an id for `HttpTracker`, index `0` | `to_value` | the literal document, as a `serde_json::Value` | clean |

Test smells found and fixed while writing: none beyond Q1. The other rows keep their tests unchanged.

## Validation

`cargo test -p torrust-tracker-primitives` (109 + 2 + 1 passed); `cargo +nightly fmt --all -- --check`;
`cargo clippy -p torrust-tracker-primitives --all-targets`; `git diff --check`; the pre-commit gate (at commit);
unit-only coverage before and after in the table above.
