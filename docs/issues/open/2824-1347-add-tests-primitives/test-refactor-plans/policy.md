---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/policy.rs
status: done
last-updated-utc: "2026-11-13 07:28"
---

# `policy.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 4, slice S3. Status: **done**: approved by the maintainer on 2026-11-13 (first gate) and the completed file approved on
2026-11-13 (second gate).

## Current State

- **Unit-only coverage:** 0 / 16 lines, measured 2026-11-12 on the S3 branch after `announce.rs` (package: 935 /
  1,059 lines, 93 unit tests). The file has no tests.
- **Review of the existing tests:** none to review.
- **Hypothesis:** the 16 lines are `TrackerPolicy::default` and its three default functions (also used by the serde
  attributes). They are operator-visible: a configuration without these keys gets a 900 s peer timeout, no persistent
  completed-download count and removal of peerless torrents, and a changed default silently changes what a tracker
  keeps in memory and on disk. The constructor is derived.

## Coverage And Decision Analysis

| Behavior group | Lines | Classification | Decision |
| --- | --- | --- | --- |
| `TrackerPolicy::default` and its default functions | 48-70 | Unit | One test of the whole default value against literals: 900, `false`, `true`. |
| Serde defaults of the three fields | 22, 36, 44 | Unit | An empty JSON object gives the literal defaults; one object sets only `max_peer_timeout`; one object flips both booleans. Each default function is reached through its serde attribute, so these tests protect the attributes, and the flipped booleans show that each key lands in its own field (the two booleans default to opposite values, so a swap is visible). |
| JSON shape of `TrackerPolicy` | 16-46 | Unit | One test: a policy of three distinct non-default values serializes to one literal document. The names are the configuration keys. |
| `Constructor` derive (`TrackerPolicy::new`) | 16 | Unselected | Generated code with no decision. |

The meaning of `persistent_torrent_completed_stat` in announce and scrape responses (issue #2406) is behavior of
other packages; this record tests only the default and the key.

No finding applies and nothing is owned elsewhere.

## Property-Test Decision

None. Constants and one-to-one fields; a generated input would repeat the examples.

## Questions For The Maintainer

None. All five tests pin documented defaults or configuration keys, and none pins a finding.

## Results (2026-11-13)

Toolchains: stable Rust 1.99.1 for tests and coverage, `cargo-llvm-cov 0.6.16`, nightly `cargo fmt`.

| Measure | Before | After |
| --- | ---: | ---: |
| Unit tests of the package | 93 | 98 (5 added, all in `policy.rs`) |
| `policy.rs` lines | 0 / 16 | 40 / 40 (the line count includes the new test module) |
| Package unit-only lines | 935 / 1,059 (88.29%) | 977 / 1,083 (90.21%) |
| Package unit-only regions | 1,488 / 1,623 (91.68%) | 1,548 / 1,669 (92.75%) |
| Package unit-only functions | 181 / 207 (87.44%) | 190 / 212 (89.62%) |

Applied as planned, no deviation. Every expected value is a literal written in the test.

### Mutation sample (7 mutants, each applied alone and reverted exactly)

| Mutant | Result | Killed by |
| --- | --- | --- |
| default timeout 900 to 901 | killed | 3 tests (the default, the empty object, the flipped flags) |
| default `persistent_torrent_completed_stat` `true` | killed | 3 tests (the default, the empty object, the partial object) |
| default `remove_peerless_torrents` `false` | killed | the same 3 tests |
| serde default removed from `max_peer_timeout` | killed | the empty-object and the flipped-flags tests |
| serde default of `persistent_torrent_completed_stat` points to the `remove_peerless_torrents` function | killed | the empty-object and the partial-object tests (the two booleans default differently, so this swap is visible, unlike the `announce.rs` one) |
| `Default` swaps the two flags | killed | the default test |
| key `remove_peerless_torrents` renamed in JSON | killed | the flipped-flags and the serialization tests |

No survivor. `cmp` against a saved copy showed the file restored byte for byte after every mutant.

## Prose-First AAA Review (every added test)

Temporary prose per test was written and compared with the code; the code expresses it.

| Test | Arrange (the one causal state) | Act | Assert (literal, independent) | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_default_to_the_documented_values` | none | `TrackerPolicy::default()` | the whole value built from `900`, `false`, `true` | clean |
| `it_should_use_the_default_of_every_field_missing_from_the_json` | the JSON text `{}` | `from_str` | the same literal policy | clean |
| `it_should_keep_the_given_peer_timeout_and_default_the_rest` | `{"max_peer_timeout": 60}` | `from_str` | `60`, `false`, `true` | clean |
| `it_should_keep_each_flag_in_its_own_field` | both flags set to the opposite of their defaults | `from_str` | `900`, `true`, `false` | clean |
| `it_should_serialize_to_the_three_configuration_keys` | a policy of `60`, `true`, `false` (non-default values) | `to_string` | one literal JSON document | clean |

Test smells found and fixed while writing: none.

## Validation

`cargo test -p torrust-tracker-primitives` (98 + 2 + 1 passed); `cargo +nightly fmt --all -- --check`;
`cargo clippy -p torrust-tracker-primitives --all-targets`; `git diff --check`; the pre-commit gate (at commit);
unit-only coverage before and after in the table above.
