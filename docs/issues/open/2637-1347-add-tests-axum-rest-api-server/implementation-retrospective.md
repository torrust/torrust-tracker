---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/ISSUE.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/lessons.md
    - docs/issues/closed/1349-1347-add-tests-http-core/implementation-retrospective.md
---

# Implementation Retrospective — Issue #2637 Add Tests to the Axum REST API Server Package

## Purpose

Record evidence-based process improvements discovered while implementing #2637. This is a blameless review of the
implementation approach; it does not replace the acceptance verification in [ISSUE.md](ISSUE.md).

## Outcome

All 31 source files of `packages/axum-rest-api-server/src/` were processed through the per-file workflow in seven
pull requests (the specification, then slices S1-S5 and this final one). Unit-only lines went from 792 / 1,240
(63.87%) to 2,324 / 2,404 (96.67%), regions from 63.98% to 96.38% and functions from 51.34% to 93.97%; aggregate
lines from 87.98% to 97.67% ([coverage-evidence.md](coverage-evidence.md)). The package has 170 unit tests (the totals
include the in-file test modules) and keeps its 58 integration tests. Every uncovered line has a recorded owner or
reason. A whole-package mutation run gave 70 caught, 4 missed and 44 unviable mutants
([mutation-evidence.md](mutation-evidence.md)); the one real survivor got a test in S6. The manual run of M1-M5
against a built tracker passed ([manual-verification-evidence.md](manual-verification-evidence.md)). Production code
changed only by documentation: the `tls_config` spelling in `lib.rs`. Bugs found on the way were handed off, not fixed
here (#2653, #2719, #2726, #2733).

## What Went Well

1. Per-file plans approved before any test was written caught wrong predictions early (for example the `server.rs`
   plan's "equivalent mutant" claim was found wrong by the independent review and fixed with one assertion).
2. Named mutations per plan, plus a whole-package run at the end, found the only real gap (a duplicated token
   value) that coverage could not show: the line was already covered.
3. Shared test support (`within_deadline`, `TestApiContainer`) removed hangs and leftover SQLite files from several
   test helpers at once.
4. Re-measuring at every slice start kept the baselines honest when `develop` moved.

## What Changed During Implementation

- The agent that started the work was replaced at S5; the hand-off file and a verification pass of its facts
  bridged the gap.
- Three post-merge review findings from earlier slices (an audit note, SQLite files left behind, a missing test
  deadline) were triaged with the maintainer and fixed inside S5.
- Tests of the legacy `Halted` start/stop path were left to #1488, and the test environment to #2482 and #2449.
- A plan prediction (`API_LOG_TARGET` would survive mutation) was wrong: 21 integration tests pin it.

## Root Cause

Plans predicted mutation outcomes from reading code, not from running the mutation, so two predictions were wrong.
Post-merge findings had no recorded triage step in the first slices, so three waited until S5.

## Improvements for Future Work

1. Run the named mutations before writing a "survives by design" claim into a plan, or label it a prediction.
2. Run the whole-package mutation assessment earlier than the last slice when the package is small (118 mutants ran
   in under four minutes), so a gap like the duplicated-token one is found while its file plan is still open.
3. Record the triage of post-merge review findings in the slice that receives them.

## Avoiding Overcorrection

No new rule is justified for hand-offs or for the S5 findings: each was a one-off. The `500` for rejected tokens is
not a testing-process problem; it is a REST API v2 matter decided by the maintainer.

## Evidence

- [ISSUE.md](ISSUE.md), [lessons.md](lessons.md), [coverage-evidence.md](coverage-evidence.md)
- [mutation-evidence.md](mutation-evidence.md), [manual-verification-evidence.md](manual-verification-evidence.md)
- [agent-review-reports.md](agent-review-reports.md) and the plans under [test-refactor-plans](test-refactor-plans/)
