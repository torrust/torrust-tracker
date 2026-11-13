---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/mode.rs
status: approved
last-updated-utc: "2026-11-13 07:43"
---

# `mode.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 5, slice S3. Status: **approved** by the maintainer on 2026-11-13 (first gate; Q1 yes); next, the tests and the second
gate (review of the completed file).

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

## Prose-First AAA Review

Written when the tests exist, one row per added test, before the completed-file gate.

## Validation

`cargo test -p torrust-tracker-primitives`; `cargo +nightly fmt --all -- --check`; `cargo clippy -p
torrust-tracker-primitives --all-targets`; `git diff --check`; the pre-commit gate; unit-only coverage before and
after in this record; a hand-run mutation sample (each mutant applied alone and reverted with `cmp`).
