---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
    - docs/issues/open/2824-1347-add-tests-primitives/ISSUE.md
    - docs/issues/open/2824-1347-add-tests-primitives/lessons.md
    - docs/issues/closed/2637-1347-add-tests-axum-rest-api-server/implementation-retrospective.md
---

# Implementation Retrospective — Issue #2824 Add Tests to the Primitives Package

## Purpose

Record evidence-based process improvements discovered while implementing #2824. This is a blameless review of the
implementation approach; it does not replace the acceptance verification in [ISSUE.md](ISSUE.md).

## Outcome

All 16 source files of `packages/primitives/src/` were processed through the per-file workflow in five pull requests
(the specification, a fix slice and three test slices). Unit-only lines went from 453 / 748 (60.56%) to 1,078 / 1,146
(94.07%), regions from 67.94% to 95.30% and functions from 57.23% to 94.17%; the package has no integration tests, so
unit-only equals aggregate ([coverage-evidence.md](coverage-evidence.md)). It has 109 unit tests (47 at the baseline).
The only uncovered lines are the `peer::fixture` test support and `peer::Id::new`, owned by the F5 follow-up. One
production defect was fixed here (F2, the `PeerRole` JSON casing, S1); two doc comments were corrected (S3); the others
were handed off (F1 #2798, F3 #2804, F4 #2823, F5 draft spec PR #2865, F6 #2838 and #2850). A whole-package mutation
run gave 94 caught, 3 missed (2 equivalent), 2 timeouts and 48 unviable ([mutation-evidence.md](mutation-evidence.md));
the real survivor got a one-value test fix. M1 and M2 passed against a built tracker
([manual-verification-evidence.md](manual-verification-evidence.md)).

## What Went Well

1. Two maintainer gates per file kept each slice small: every plan named its mutants in advance, and each per-file
   survivor was explained in its record (an equivalent mutant or a decision taken by design).
2. A `quickcheck` property was kept only where it killed a mutant no example caught (`announce_interval.rs`), so the
   new dependency carries a recorded reason.
3. The whole-package run at the end found a gap the per-file samples missed (a test value equal to a default mutant's
   constant), and the fix was one literal.

## What Changed During Implementation

1. **Review rounds of S3.** PR #2852 took five review rounds, most of them about the records, not the tests: a
   mislabeled round, Processing Log entries rewritten in place, stamps before the events, branch SHAs as references.
   The audit-record rules were known; they were not checked before each push.
2. **Stale coverage data.** The first S4 baseline read old profile data in the shared target directory (lines of a
   deleted impl in a 70-line file). Every later figure used a separate `CARGO_LLVM_COV_TARGET_DIR`.
3. **Spec archive timing.** The slice table said S4 moves the spec to `docs/issues/closed/`. The
   `cleanup-completed-issues` skill archives after the issue is closed, and #2637 did it in its own PR; S4 follows the
   skill.

## Root Cause

1. The audit record is long and append-only; correcting it under time pressure invited in-place edits. No local check
   compares a new stamp with the event it records.
2. The stale report came after `cargo llvm-cov clean --workspace` in the default coverage target; why that directory
   still held old data was not established. A separate directory gave consistent figures in every later run.

## Improvements for Future Work

1. Before each push of a review fix, read the Processing Log diff and confirm it only appends, and take every stamp
   with `date -u` after the event it records.
2. Measure coverage in a per-slice `CARGO_LLVM_COV_TARGET_DIR`, and reject a report whose line numbers exceed the file
   length.
3. In accessor tests, use values that no default mutant returns (not `0`, `1` or `Default::default()`).

## Avoiding Overcorrection

The two-gate workflow is the reason the slices stayed small; the improvements above are local checks, not more gates.

## Evidence

- [ISSUE.md](ISSUE.md): progress log, ledger, slice table and acceptance verification.
- [coverage-evidence.md](coverage-evidence.md), [mutation-evidence.md](mutation-evidence.md),
  [manual-verification-evidence.md](manual-verification-evidence.md), [lessons.md](lessons.md).
- `docs/pr-reviews/pr-2852-review/PR-REVIEW.md`: the five review rounds of S3.
