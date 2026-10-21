---
semantic-links:
  skill-links:
    - write-markdown-docs
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2415-2410-si-22-3-per-component-cancellation-tokens/ISSUE.md
    - docs/issues/open/2415-2410-si-22-3-per-component-cancellation-tokens/manual-verification-evidence.md
    - docs/issues/open/2415-2410-si-22-3-per-component-cancellation-tokens/agent-review-reports.md
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
---

# Implementation Retrospective — Give Each Application Component Its Own Cancellation Token

## Purpose

Record evidence-based process improvements discovered while implementing #2415 (SI-22.3, T6 of
sub-EPIC #2410). This is a blameless review of the implementation approach; it does not replace
acceptance-criteria verification.

## Outcome

`JobManager::reserve` issues each top-level component its own cancellation token, not derived from
the root token, and `JobManager::spawn` consumes the reservation. A reservation from another manager
is rejected with `SpawnError`, and a dropped unspawned reservation is released. `cancel()` still
cancels every component and the legacy root token at once, so shutdown is unchanged. All 13
components in `src/app.rs` use their reservation token, and only the legacy `udp_ban_cleanup` uses the
root token. Evidence: V1 to V4 in `manual-verification-evidence.md`, including the like-for-like M1
comparison with `develop` for SIGTERM and SIGINT, and the Task Reviewer report.

## What Went Well

1. **The design review came before the broad conversion.** The T6a review settled the spawn error,
   its propagation, the accessor name, and the commit split. The 13 call sites were then converted
   once, with no rework. A temporary adapter kept the intermediate commit compiling, so the review
   had a green tree to look at.
2. **Mutations found a gap the existing suite could not see.** Giving one component an unrelated
   token passed every test and `lifecycle-signals`. The component was only aborted at the deadline,
   and `main` discards outcomes. One application-level test now fails for each of 14 such mutations
   (V2).
3. **The `compile_fail` doctest's companion earned its place.** A mechanical `sed` rename also
   rewrote the doc examples. The `compile_fail` doctest still passed, now for the wrong reason, and
   only the `no_run` companion failed (V1).
4. **M1 was compared against `develop` several times.** Three runs of the `develop` binary gave three
   different completion orders. That showed the order difference between the two binaries is
   scheduling, not a regression (V3).

## What Changed During Implementation

- **The spawn error and its handling were settled at the T6a review** (record entry 30):
  `spawn` returns `SpawnError::ForeignReservation`, which bootstrap propagates as
  `app::Error::ComponentRegistration`, and the root-token accessor became
  `legacy_cancellation_token()`. The Chosen Design had left the precise errors to T6.
- **AC1 had no guard**, so an application-level test was added (V2).
- **The test-side `JobManager::new()` count in `src/app.rs` was six, not four.** The sub-EPIC's T6
  step was corrected.
- **The Task Reviewer found a weak assertion (S2).** The release test passed under a mutation that
  cancels the *other* components on drop. My T6a mutations never modelled that clause of the rule (V4).
- **An existing test was removed without being recorded (S1).** It was a duplicate after the
  conversion, but neither the evidence nor the commit message said so.

## Root Cause

- **S2:** I picked the mutations by asking how each mechanism could break (no release, no cancel,
  foreign accepted). I did not derive them from each clause of the spec's lifecycle rules. "A
  dropped unspawned reservation must release its state without cancelling unrelated components" has
  two clauses, and only the first had a mutation.
- **The AC1 gap:** it comes from the shutdown design, not from this change. A component that ignores
  its stop request is aborted at the shared deadline and the process still exits 0, so only a test
  that reads the outcomes can notice. SI-20 owns turning those outcomes into the exit code.
- **S1:** the conversion treated removing a duplicate test as a mechanical detail. In a
  behavior-preserving refactor whose acceptance criterion is "all existing tests pass", a removed
  test changes the evidence.

## Improvements for Future Work

1. When a spec states lifecycle rules, derive at least one mutation from each clause, including the
   negative ones ("without cancelling ...", "is not lost"), and record the clause next to the mutation.
   Proposal for the `write-unit-test` skill; not applied here, it needs the maintainer's approval.
2. In a refactor whose acceptance criterion says existing tests still pass, record every removed or
   merged test, and why, in the evidence and the commit message. Proposal for the same skill; not
   applied.
3. For #2416 (stop order): extend the application-level outcome test instead of relying on
   `lifecycle-signals`. A component that ignores its token is not visible at process level until SI-20
   maps outcomes to the exit code.

## Avoiding Overcorrection

- No mutation-testing tool (for example `cargo-mutants`) is justified. Targeted, recorded mutations
  found both gaps; the problem was choosing them, not running them.
- Do not add tests for the unreachable `app::Error::ComponentRegistration` path. A foreign reservation
  cannot occur in bootstrap today, and the manager's own test covers the error.
- Do not reorder or rename further `JobManager` APIs ahead of #2416. `StopBefore` belongs to T7 and
  T8.

## Evidence

- Issue specification: `ISSUE.md` in this folder; sub-EPIC record entry 30.
- `manual-verification-evidence.md` V1 to V4; `agent-review-reports.md` (Task Reviewer, 2026-10-21).
- Commits: `refactor(bootstrap): [#2415] add JobManager reservations that issue each component its own token`
  and `refactor(bootstrap): [#2415] give every application component its own reservation token`.
