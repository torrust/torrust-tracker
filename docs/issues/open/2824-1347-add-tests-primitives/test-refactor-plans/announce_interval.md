---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/announce_interval.rs
status: done
last-updated-utc: "2026-11-13 09:44"
---

# `announce_interval.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 2, slice S3. Status: **done**: approved by the maintainer on
2026-11-12 (first gate, Q1 as written) and the completed file approved on 2026-11-12 (second gate).

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
| Deserializing a negative number | 112-117 | Unit (line already covered) | One test: `-1` is rejected (a configuration with `interval = -1` must not load). The error text comes from serde and names the type it expected, so only the failure is asserted, not the text. A `Deserialize` that clamps a negative number to zero would be caught; one that reads a signed value and casts it still rejects `-1`, with another message, and survives by design (see the mutation table). |
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

## Results (2026-11-12)

Toolchains: stable Rust 1.99.1 for tests and coverage, `cargo-llvm-cov 0.6.16`, nightly `cargo fmt`.

| Measure | Before | After |
| --- | ---: | ---: |
| Unit tests of the package | 81 | 87 (6 added, all in `announce_interval.rs`) |
| `announce_interval.rs` lines | 79 / 88 | 113 / 113 |
| Package unit-only lines | 848 / 995 (85.23%) | 882 / 1,020 (86.47%) |
| Package unit-only regions | 1,345 / 1,506 (89.31%) | 1,406 / 1,553 (90.53%) |
| Package unit-only functions | 161 / 194 (82.99%) | 170 / 200 (85.00%) |

Applied as planned: the four group tests (`TryFrom<u32>` in two tests, the `u32` conversion, `Display`, the negative
number) and one `quickcheck` property (`interval_bound`). No deviation. `cargo clippy --all-targets` is clean (a first
`match` in the property was rewritten as one comparison after `option_if_let_else`). The figures were first measured
before that rewrite (114 / 114 for the file, 883 / 1,021 for the package); the rewrite removed one covered line, and the
table shows the re-measure at the commit `test(primitives): [#2824] test the announce interval conversions and bound`
(`review-finding:pr-2852-f3`).

### Property decision (the Q1 rule)

The property deserializes any `u32` and expects success with the same value if and only if it is within the limit.
It draws a `u32` and not a `u64`: `quickcheck` draws a full-range `u64` almost always above the limit, which would
test one side only.

| Mutant | Result | Killed by |
| --- | --- | --- |
| Bound slips for one value (`value != 1_000_000`) | **survived** | nothing: a random search does not find a single value; the edges do not either. Accepted as the limit of the technique. |
| Bound slips for a mid-range band (`1_000_000_000..2_000_000_000` rejected) | killed | **the property only** (`it_should_deserialize_a_number_if_and_only_if_it_is_within_the_limit`, shrunk to `1000000000`); no example test fails |

One mutant that only the property kills meets the rule, so the property is **kept**. None of the three mutants named in
advance (above) met the rule: the single-value slip survived everything, and the narrowing cast and the `value + 1`
mutant are killed by example tests. The deciding mutant, the band slip, was added after them, when the property had been
run (`review-finding:pr-2852-f1`).

### Mutation sample (8 mutants, each applied alone and reverted exactly)

| Mutant | Result | Killed by |
| --- | --- | --- |
| Narrowing cast `as u32` instead of `u32::try_from` | killed | the above-`u32` message test (existing) |
| Deserializer returns `value + 1` above a threshold | killed | 4 tests incl. the property (existing edge tests too) |
| `TryFrom<u32>` skips the bound | killed | the `TryFrom` above-the-limit test |
| `From<_> for u32` adds one | killed | the `u32` conversion test |
| `Display` adds a unit suffix | killed | the display test |
| Deserializer reads `i64` and casts to `u64` | **survived, by design** | `-1` becomes `2^64 - 1`, which the bound rejects anyway, so the failure is the same; the error message is not: it names `18446744073709551615` instead of serde's "invalid value: integer `-1`, expected u64". The test asserts only the failure, so it cannot tell them apart; a test of the message would. Accepted: the message is serde's, not a rule of this type |
| Deserializer clamps a negative number to `0` | killed | the negative-number test (this is the case the test protects) |
| Mid-range band slip | killed | the property only (above) |

`cmp` against a saved copy showed the file restored byte for byte after every mutant.

## Prose-First AAA Review (every added test)

Temporary prose per test was written and compared with the code; the code expresses it.

| Test | Arrange (the one causal state) | Act | Assert (literal, independent) | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_be_built_from_a_u32_within_the_limit` | the literal `120` | `try_from` | `get()` is `120` | clean |
| `it_should_fail_to_be_built_from_a_u32_above_the_limit` | the literal one above the limit | `try_from` | the whole error with the rejected value | clean |
| `it_should_convert_to_a_u32_without_changing_the_seconds` | an interval of `120` | `u32::from` | `120` | clean |
| `it_should_display_as_the_plain_number_of_seconds` | an interval of `120` | `to_string` | `"120"` | clean |
| `it_should_reject_deserializing_a_negative_interval` | the JSON text `-1` | `from_str` | the call fails (the serde text is not asserted) | clean |
| `interval_bound::it_should_deserialize_a_number_if_and_only_if_it_is_within_the_limit` | any `u32` (property) | `from_str` of its text | success with the same value iff within the limit, written as one comparison against the literal bound | clean; the bound is `MAX`, the test's own constant, not the production one |

Test smells found and fixed while writing: none left. Existing tests judged `clean` in the plan are unchanged.

## Validation

`cargo test -p torrust-tracker-primitives` (87 + 2 + 1 passed); `cargo +nightly fmt --all -- --check`;
`cargo clippy -p torrust-tracker-primitives --all-targets`; `git diff --check`; the pre-commit gate (at commit);
unit-only coverage before and after in the table above.
