---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/announce_interval.rs
status: approved
last-updated-utc: "2026-11-12 20:30"
---

# `announce_interval.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 2, slice S3. Status: **approved** by the maintainer on 2026-11-12 (first gate; Q1 approved as written); next, the tests and the second
gate (review of the completed file).

## Current State

- **Unit-only coverage:** 79 / 88 lines, measured 2026-11-12 on `develop` after #2841 (package: 848 / 995 lines, 81
  unit tests). 10 unit tests, all in the file.
- **Review of the existing tests: `clean`.** Names follow `it_should_*`; Act is one visible call; expectations are
  literals written in the test (`2_147_483_647`, `"2147483648"`), not derived from `MAX`. The three error-message
  tests assert on substrings of the text, which is the contract the ADR names (value and limit). Nothing to refactor.
- **Hypothesis:** the type is already well protected at its boundary (accept `i32::MAX`, reject one above, reject above
  `u32::MAX`, conversions to the wire types, serde). The 9 uncovered lines are three trivial delegations. One behavior
  group has no test even though its line is covered: a negative number in the configuration.

## Coverage And Decision Analysis

| Behavior group | Lines | Classification | Decision |
| --- | --- | --- | --- |
| `TryFrom<u32>` delegates to `new` | 67-73 | Unit | One test: a valid value converts, a value above the limit fails with the same error as `new` (pins that the conversion enforces the bound; a `Self(value)` shortcut would pass every other test). |
| `From<AnnounceIntervalInSecs> for u32` | 82-86 | Unit | One test with a literal interval (`120`): the seconds come back unchanged. |
| `Display` | 100-104 | Unit | One test: `120` prints as `"120"` (the text a log line or a message shows; no unit, no padding). |
| Deserializing a negative number | 112-117 | Unit (line already covered) | One test: `-1` is rejected (a configuration with `interval = -1` must not load). The error text comes from serde and names the type it expected, so only the failure is asserted, not the text. A change of the `Deserialize` to read a signed value would be caught. |
| Everything else (accept `MAX`, reject `MAX + 1`, above `u32::MAX`, `u16` conversion, wire conversions, serialize) | 34-65, 75-98, 106-110 | Unit, **tested** | Covered by the 10 existing tests. |

Nothing in this file is owned by another issue and no finding applies. The ADR
`20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md` is the owner of the rule being
protected; the tests pin the rule, not any wire format.

## Property-Test Decision

This is the second approved `quickcheck` seam (the first was `peer::Id`). Candidate: for any `u64`, deserializing
`value` from JSON succeeds if and only if `value <= 2_147_483_647`, and then `get()` equals `value`. It would cover the
whole range, where the examples cover the two edges and one value above `u32::MAX`.

Pre-registered rule, because the examples are already strong: the property is **kept only if it kills a mutant that
all the example tests let survive**, tried with at least these three: a mid-range slip in the bound (`value <= MAX && value != 1_000_000`), a narrowing
cast instead of `u32::try_from` (the value `2^32 + 5` wraps to `5`), and a deserializer that returns `value + 1`
above a threshold. If it kills none, it is dropped and the record says so: a property that only repeats the edge tests is not
worth its runtime and its machinery. Dependencies are already in place (`quickcheck`, `quickcheck_macros`, from S2).

## Questions For The Maintainer

- **Q1: the property rule above.** Approve the "keep only if it kills a mutant the examples miss" rule? The alternative
  is to add it unconditionally (the plan of the ISSUE listed this seam as approved, so no further dependency decision
  is needed).
- No question about the four tests: all test package-owned behavior and none pins a finding.

## Prose-First AAA Review

Written when the tests exist, one row per added test, before the completed-file gate. Expected shape: one causal
Arrange (a literal interval or a literal number), one Act (the conversion, the formatting or the deserialization),
literal Assert.

## Validation

`cargo test -p torrust-tracker-primitives`; `cargo +nightly fmt --all -- --check`; `git diff --check`; the pre-commit
gate; unit-only coverage re-run with before and after recorded here; a hand-run mutation sample of this file
(each mutant applied alone and reverted with `cmp`).
