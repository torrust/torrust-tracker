---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/policy.rs
status: approved
last-updated-utc: "2026-11-13 07:24"
---

# `policy.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 4, slice S3. Status: **approved** by the maintainer on 2026-11-13 (first gate); next, the tests and the second
gate (review of the completed file).

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

## Prose-First AAA Review

Written when the tests exist, one row per added test, before the completed-file gate.

## Validation

`cargo test -p torrust-tracker-primitives`; `cargo +nightly fmt --all -- --check`; `cargo clippy -p
torrust-tracker-primitives --all-targets`; `git diff --check`; the pre-commit gate; unit-only coverage before and
after in this record; a hand-run mutation sample (each mutant applied alone and reverted with `cmp`).
