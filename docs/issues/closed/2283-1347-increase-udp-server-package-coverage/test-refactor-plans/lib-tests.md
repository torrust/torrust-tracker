---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/lib.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/lib.rs
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/ISSUE.md
---

# UDP Server Crate Root File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/udp-server/src/lib.rs`.

## Current State

- **Baseline unit-only coverage:** 28 / 29 lines (96.55%), 27 / 28 regions (96.43%), and 2 / 2
  functions (100.00%) from the T1 report in `coverage-evidence.md`.
- **Module-owned surface:** public module declarations, `CurrentClock` compile-time aliases, type
  aliases, `RawRequest` data, and test-only helpers `sample_peer` and `announce_events_match`.
- **Existing test surface:** this file has no `#[test]` functions. Its `tests` module is reused
  support code for tests in child modules.
- **T1 hypothesis:** probably no new runtime test. The crate-root declarations do not select
  behavior, and the only uncovered line is the `false` branch for mismatched events in the
  test-only `announce_events_match` helper.

## Current Tests Review

`lib.rs` has no test body to refactor. `sample_peer` and `announce_events_match` have one caller,
the external-IP announce scenario in `handlers/announce.rs`. That caller is a collaboration test of
the announce handler: the helper's event-comparison policy is part of its mock predicate, not a
crate-root contract. Review its Arrange, expected event, and predicate readability in the future
`handlers/announce.rs` file test plan; do not refactor that different source file while `lib.rs` is
active. There is no `lib.rs` test-cleanliness refactor to make.

## Coverage Analysis

| Behavior group | Classification | Decision to review |
| --- | --- | --- |
| Module visibility, aliases, and `RawRequest` data | Collaborator/compiler-owned | No direct test; compilation and child-module use provide the maintained contract. |
| `sample_peer` test fixture | Test-support code | Review only through its callers; do not add a fixture-value test unless a caller exposes hidden coupling. |
| Matching and mismatched `announce_events_match` outcomes | Test-support code | Review the helper/caller relationship. A direct helper test is justified only if the helper remains a reusable observable contract after R1. |
| Real UDP behavior | Integration | Not selected: this crate root has no real UDP act; integration tests belong to the owning server/handler file plans. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Reviewed the helper use. No `lib.rs` test body exists to refactor; the sole caller belongs to the future `handlers/announce.rs` plan. | Caller ownership review recorded; no code change. |
| R2 | DONE | No unit test added. The aliases and `RawRequest` data carry no crate-root decision; a direct helper test would test test-support implementation instead of package behavior. | Usage review and recorded no-change conclusion. |
| R3 | SKIPPED | No integration increment selected: `lib.rs` has no real UDP act; its consumers own loopback contracts. | Explicit `none` rationale. |
| R4 | DONE | Re-ran unit-only coverage: 28 / 29 lines (96.55%), 27 / 28 regions (96.43%), and 2 / 2 functions (100.00%), unchanged from baseline. | Updated plan and `coverage-evidence.md` per-file results. |

## Approval Requested

Approve this plan before R1 begins. The proposed initial conclusion is that `lib.rs` may need no
production or integration test, but the helper/caller review must establish that conclusion rather
than relying on the T1 hypothesis.

## Progress Log

- 2026-09-22 11:15 UTC - GitHub Copilot - Created the plan from the T1 baseline and crate-root
  review. No tests or production code changed. R1 awaits maintainer approval.
- 2026-09-22 11:15 UTC - User/maintainer - Approved the file test plan and R1 review.
- 2026-09-22 11:15 UTC - GitHub Copilot - Completed R1 with no `lib.rs` change. The one helper
  caller is an announce-handler collaboration test; its mock-predicate readability belongs to the
  future `handlers/announce.rs` plan. The crate root has no test body of its own to refactor.
- 2026-09-22 11:36 UTC - GitHub Copilot - Completed R2-R4. The declaration and helper-use review
  found no crate-root decision that a unit or integration test could protect. Fresh unit-only
  `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` recorded unchanged
  28 / 29 lines, 27 / 28 regions, and 2 / 2 functions. The plan awaits completed-file review.
- 2026-09-22 12:45 UTC - User/maintainer - Approved the completed-file review for `lib.rs`.
- 2026-09-22 12:45 UTC - GitHub Copilot - Marked this plan complete. No code or test change was
  justified; the recorded coverage result and ownership boundary are the completed file outcome.
