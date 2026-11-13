---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/announce.rs
status: done
last-updated-utc: "2026-11-13 09:44"
---

# `announce.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 3, slice S3. Status: **done**: approved by the maintainer on 2026-11-12 (first gate) and the completed file approved on
2026-11-12 (second gate).

## Current State

- **Unit-only coverage:** 0 / 16 lines, measured 2026-11-12 on the S3 branch after the `announce_interval.rs` file
  (package: 882 / 1,020 lines, 87 unit tests). The file has no tests.
- **Review of the existing tests:** none to review.
- **Hypothesis:** the 16 lines are the `Default` of `AnnouncePolicy`, its three default functions (also used by the
  serde attributes). The `Default` of `AnnounceData` is derived and has no lines of its own, but it composes the others. They carry operator-visible behavior: the defaults (interval 120 s, minimum
  interval 120 s, at most 74 peers) apply to every tracker whose configuration has no `[announce_policy]` values, and
  a changed default silently changes every announce response. The constructors and the `AnnounceEvent` enum are
  derived or declarative.

## Coverage And Decision Analysis

| Behavior group | Lines | Classification | Decision |
| --- | --- | --- | --- |
| `AnnouncePolicy::default` and its default functions | 63-85 | Unit | One test of the whole default value against literals: interval 120, minimum 120, 74 peers. |
| Serde defaults: an empty JSON object, and a partial object | 33, 47, 59 | Unit | An empty object gives the same literals; an object with one field keeps that value and defaults the other two. Each default function is reached through the serde attribute, not through `Default`, so these tests protect the attributes. |
| Serde bound: `interval` and `interval_min` above the limit | 33-48 | Unit | One test: an interval above `i32::MAX` is rejected when the policy is deserialized (the ADR rule applies through the field type; the bound itself is tested in `announce_interval.rs`). |
| JSON shape of `AnnouncePolicy` | 20-61 | Unit | One test: the whole value serializes to one literal document with the three field names. Field names are the configuration keys. |
| `AnnounceData::default` | 95-102 | Unit | One test: no peers, default statistics and default policy. |
| `Constructor` derives (`AnnouncePolicy::new`, `AnnounceData::new`) | 20, 95 | Unselected | Generated code with no decision; the tests above build values with `new` when convenient. |
| `AnnounceEvent` | 105-118 | Unselected | Four-variant enum with no behavior; its duplication is an ADR decision, not a package behavior. |

No finding applies and nothing is owned elsewhere. The two defaults of 120 are equal, so a swap between
`interval` and `interval_min` cannot be detected by a default test: the partial-object test uses
`interval_min` alone to give it one distinguishing value.

## Property-Test Decision

None. The values are constants and one-to-one fields; a generated input would repeat the examples.

## Questions For The Maintainer

None. All six tests pin documented defaults or the configuration keys, and none pins a finding.

## Results (2026-11-12)

Toolchains: stable Rust 1.99.1 for tests and coverage, `cargo-llvm-cov 0.6.16`, nightly `cargo fmt`.

| Measure | Before | After |
| --- | ---: | ---: |
| Unit tests of the package | 87 | 93 (6 added, all in `announce.rs`) |
| `announce.rs` lines | 0 / 16 | 55 / 55 (the line count includes the new test module) |
| Package unit-only lines | 882 / 1,020 (86.47%) | 937 / 1,059 (88.48%) |
| Package unit-only regions | 1,406 / 1,553 (90.53%) | 1,489 / 1,623 (91.74%) |
| Package unit-only functions | 170 / 200 (85.00%) | 181 / 207 (87.44%) |

Applied as planned, with one correction to the plan text: it said "five tests" while its table lists six (the empty
object and the partial object are two tests). No deviation. The `AnnounceData` test builds its expected value from
literals (`0`, `0`, `0`, `120`, `120`, `74`), not from `Default`, so a change of any default fails it.

### Mutation sample (7 mutants, each applied alone and reverted exactly)

| Mutant | Result | Killed by |
| --- | --- | --- |
| default peers 74 to 75 | killed | 4 tests (the default, the empty object, the partial object, `AnnounceData`) |
| default `interval` 120 to 121 | killed | the same 4 tests |
| default `interval_min` 120 to 121 | killed | 3 tests (the default, the empty object, `AnnounceData`) |
| serde default removed from `max_peers_per_announce` | killed | the empty-object and the partial-object tests |
| `Default` uses 60 peers directly | killed | the default test and the `AnnounceData` test |
| serde default of `interval_min` points to the `interval` function | **survived, equivalent** | both defaults are 120, so the result is identical (the limit named in the plan) |
| `interval` field typed as a plain `u32` | not applicable | does not compile: the conversion and the default are typed, so the compiler guards it; the bound test protects the behavior the type gives |

`cmp` against a saved copy showed the file restored byte for byte after every mutant.

## Prose-First AAA Review (every added test)

Temporary prose per test was written and compared with the code; the code expresses it.

| Test | Arrange (the one causal state) | Act | Assert (literal, independent) | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_default_to_the_documented_values` | none | `AnnouncePolicy::default()` | the whole value built from `120`, `120`, `74` | clean |
| `it_should_use_the_default_of_every_field_missing_from_the_json` | the JSON text `{}` | `from_str` | the same literal policy | clean |
| `it_should_keep_the_given_field_and_default_the_others` | `{"interval_min": 30}` | `from_str` | `120`, `30`, `74` | clean |
| `it_should_reject_an_interval_above_the_limit_of_every_delivery_protocol` | `{"interval": 2147483648}` | `from_str` | the call fails (the text belongs to `announce_interval.rs`) | clean |
| `it_should_serialize_to_the_three_configuration_keys` | a policy of `60`, `30`, `10` (three distinct values) | `to_string` | one literal JSON document | clean |
| `it_should_default_to_no_peers_empty_statistics_and_the_default_policy` | none | `AnnounceData::default()` | the whole value built from literals | clean |

Test smells found and fixed while writing: a first draft built the expected `AnnounceData` from the production
`Default` values, which would pass whatever the defaults were; it was rewritten with literals.

## Validation

`cargo test -p torrust-tracker-primitives` (93 + 2 + 1 passed); `cargo +nightly fmt --all -- --check`;
`cargo clippy -p torrust-tracker-primitives --all-targets`; `git diff --check`; the pre-commit gate (at commit);
unit-only coverage before and after in the table above.
