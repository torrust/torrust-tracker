---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/mode.rs
status: done
last-updated-utc: "2026-11-13 08:24"
---

# `mode.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 5, slice S3. Status: **done**: approved by the maintainer on 2026-11-13 (first gate; Q1 yes) and the completed file approved on
2026-11-13 (second gate).

## Current State

- **Unit-only coverage:** 0 / 8 lines, measured 2026-11-13 on the S3 branch after `policy.rs` (package: 977 / 1,083
  lines, 98 unit tests). The file has no tests.
- **Review of the existing tests:** none to review.
- **Hypothesis:** the 8 lines are `PrivateMode::default` and its one default function (also used by the serde
  attribute). The default is operator-visible: the tracker in private mode checks the expiration date of keys unless the
  configuration turns it off, and a flipped default would silently accept expired keys. `tracker-core` reads the field
  (`if private_mode.check_keys_expiration` rejects an expired key), which confirms that `true` means the date **is
  checked**.
- **Documentation defect found while reading (not a behavior defect):** the doc comment of `check_keys_expiration`
  says "A flag to disable expiration date for peer keys. When true, if the keys is not permanent the expiration date
  will be ignored", which is the opposite of what the code and the field name do. See question Q1.

## Coverage And Decision Analysis

| Behavior group | Lines | Classification | Decision |
| --- | --- | --- | --- |
| `PrivateMode::default` and its default function | 26-38 | Unit | One test: the default value has `check_keys_expiration` `true`. |
| Serde default of `check_keys_expiration` | 22 | Unit | An empty JSON object gives `true`; an object with `false` keeps `false` (so the key lands in its field and the default is not applied over an explicit value). |
| JSON shape of `PrivateMode` | 16-24 | Unit | One test: the value with the flag `false` serializes to the literal `{"check_keys_expiration":false}`. The name is the configuration key. |
| `Constructor` and `Display` derives | 16 | Unselected | Generated code. `Display` prints the one field; I found no formatting of a `PrivateMode` in the eight files that name it, so it carries no decision to protect. |

No finding blocks tests and nothing is owned elsewhere.

## Property-Test Decision

None. One boolean field; a generated input would repeat the examples.

## Questions For The Maintainer

- **Q1: correct the doc comment of `check_keys_expiration`** (documentation only, in scope as a "documentation-comment
  correction in the files under test"): say that when `true` (the default) the tracker rejects an expired key, and
  when `false` it accepts one. The same pattern as Q2 of `peer.rs` (`to_hex_string`). Alternative: leave it and
  record only a `lessons.md` row. Recommendation: correct it: the comment states the opposite of the behavior and
  would mislead the next reader.

## Results (2026-11-13)

Toolchains: stable Rust 1.99.1 for tests and coverage, `cargo-llvm-cov 0.6.16`, nightly `cargo fmt`.

| Measure | Before | After |
| --- | ---: | ---: |
| Unit tests of the package | 98 | 102 (4 added, all in `mode.rs`) |
| `mode.rs` lines | 0 / 8 | 25 / 25 (the line count includes the new test module) |
| Package unit-only lines | 977 / 1,083 (90.21%) | 1,002 / 1,100 (91.09%) |
| Package unit-only regions | 1,548 / 1,669 (92.75%) | 1,590 / 1,705 (93.26%) |
| Package unit-only functions | 190 / 212 (89.62%) | 196 / 216 (90.74%) |

Applied as planned. The plan listed three groups; the serde group became two tests (the empty object, and the explicit
`false`), so four tests were added. Q1 applied: the doc comment of `check_keys_expiration` now says that `true` (the
default) rejects an expired key and `false` accepts it; documentation only, `cargo doc` builds.

### Mutation sample (5 mutants, each applied alone and reverted exactly)

| Mutant | Result | Killed by |
| --- | --- | --- |
| default function returns `false` | killed | the default test and the empty-object test |
| serde default attribute removed | killed | the empty-object test |
| serde default replaced by `bool::default` (`false`) | killed | the empty-object test |
| `Default` hardcodes `false` | killed | the default test |
| key renamed in JSON (`check_expiration`) | killed | the explicit-`false` test and the serialization test |

No survivor. `cmp` against a saved copy showed the file restored byte for byte after every mutant.

## Prose-First AAA Review (every added test)

Temporary prose per test was written and compared with the code; the code expresses it.

| Test | Arrange (the one causal state) | Act | Assert (literal, independent) | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_check_the_keys_expiration_by_default` | none | `PrivateMode::default()` | the value built with `true` | clean |
| `it_should_check_the_keys_expiration_when_the_json_does_not_say_otherwise` | the JSON text `{}` | `from_str` | the value built with `true` | clean |
| `it_should_keep_an_explicit_choice_not_to_check_the_keys_expiration` | `{"check_keys_expiration": false}` | `from_str` | the value built with `false` | clean |
| `it_should_serialize_to_the_configuration_key` | a mode with the flag `false` (non-default) | `to_string` | the literal document | clean |

Test smells found and fixed while writing: none.

## Validation

`cargo test -p torrust-tracker-primitives` (102 + 2 + 1 passed); `cargo +nightly fmt --all -- --check`;
`cargo clippy -p torrust-tracker-primitives --all-targets`; `cargo doc -p torrust-tracker-primitives --no-deps`;
`git diff --check`; the pre-commit gate (at commit); unit-only coverage before and after in the table above.
