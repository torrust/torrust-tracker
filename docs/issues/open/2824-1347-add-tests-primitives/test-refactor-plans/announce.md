---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/announce.rs
status: approved
last-updated-utc: "2026-11-12 20:41"
---

# `announce.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 3, slice S3. Status: **approved** by the maintainer on 2026-11-12 (first gate); next, the tests and the second
gate (review of the completed file).

## Current State

- **Unit-only coverage:** 0 / 16 lines, measured 2026-11-12 on the S3 branch after the `announce_interval.rs` file
  (package: 883 / 1,021 lines, 87 unit tests). The file has no tests.
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

None. All five tests pin documented defaults or the configuration keys, and none pins a finding.

## Prose-First AAA Review

Written when the tests exist, one row per added test, before the completed-file gate.

## Validation

`cargo test -p torrust-tracker-primitives`; `cargo +nightly fmt --all -- --check`; `cargo clippy -p
torrust-tracker-primitives --all-targets`; `git diff --check`; the pre-commit gate; unit-only coverage before and
after in this record; a hand-run mutation sample (each mutant applied alone and reverted with `cmp`).
