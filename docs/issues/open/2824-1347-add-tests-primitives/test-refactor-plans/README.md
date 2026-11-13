---
doc-type: test-refactor-plan-guidance
issue: 2824
package: torrust-tracker-primitives
status: in-progress
last-updated-utc: "2026-11-12 16:45"
---

# `primitives` File Test Plan Guidance

This folder holds one decision record per Rust source file in `packages/primitives/src/` that needs
one (`<module>.md`), plus `s1-peer-role-serialization.md`, the design review of slice S1. The
workflow, the two maintainer gates, the commit points, the slices and the ledger are defined once in
[ISSUE.md](../ISSUE.md#per-file-workflow); do not duplicate them here. The
[coverage evidence](../coverage-evidence.md) holds the numbers.

Create a record right before starting its file, never in advance. A record authorizes neither test
changes nor work on another file until the maintainer approves it. Files with no change share one record.

## Records

| Sequence | Record | State |
| ---: | --- | --- |
| 0 | [s1-peer-role-serialization.md](s1-peer-role-serialization.md) | DONE (reviewed in PR #2834) |
| 1 | [peer.md](peer.md) | DONE (S2, PR #2841) |
| 2 | [announce_interval.md](announce_interval.md) | DONE (S3) |
| 3 | [announce.md](announce.md) | DONE (S3) |

## Shared Guardrails

- **Unit-only coverage is the primary metric** (`cargo llvm-cov -p torrust-tracker-primitives
  --all-features --lib`); the package has no integration tests. Do not add a test merely for coverage:
  a line is classified **unit**, **collaborator-owned**, **finding** or **unselected** with the reason.
- Judge existing tests against the `write-unit-test` skill (name `it_should_*`, visible
  Arrange-Act-Assert, state-centred Arrange, one reason to fail, expected values written in the test,
  no production-derived expectations, no logic in the test). List concrete smells; "clean" is also a
  recorded conclusion.
- **Prose-First AAA Review** row for every added or changed test, before the completed-file review.
- **Ownership:** `peer::fixture` is owned by the F5 follow-up (`peer-id-test-helper`; #2482 is closed and did not move it); the `AddAssign` of the swarm aggregate is gone
  (#2823); `Peer::Ord` is gone (#2798); `Location::caller()` is owned by #2804; `peer::Id::new` waits for its
  follow-up (F5). Do not test behavior that an open finding reports as wrong without a recorded maintainer
  decision (see [Findings During Implementation](../ISSUE.md#findings-during-implementation)).
- **Properties** only where the record names a natural invariant, and kept only if they protect state that
  examples miss. `quickcheck` is the approved dev-dependency (two seams), added with the
  `add-rust-dependency` skill and checked with `cargo machete --with-metadata`.
- **Toolchains:** stable Rust for tests and coverage, `cargo +nightly fmt --all -- --check` for
  formatting, `git diff --check` after each increment, `linter markdown` and `linter cspell` after
  changing these records. Record the toolchain with every validation result.
- Cite commits by their unique Conventional Commit subject, never by id, until merged.
