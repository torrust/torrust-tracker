---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/open/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/agent-review-reports.md
    - docs/issues/open/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/performance-evidence.md
    - docs/issues/closed/2324-1488-si-13-migrate-health-check-api-token-lifecycle/implementation-retrospective.md
---

# Implementation Retrospective — Migrate UDP Tracker to Token Lifecycle

## Purpose

Record evidence-based process improvements discovered while implementing
issue #2342 (SI-14 of EPIC #1488).

## Outcome

Each UDP instance now receives a `JobManager` child token. The receive loop
stops cooperatively between datagrams and returns `Ok(())` only on
cancellation; receive errors and stream end fail the component. The component
owns the loop through `OwnedTask`, built before its future is returned. The
legacy launcher is an adapter over the same loop, so it can no longer panic or
detach it. Manual SIGTERM, rebind, and before/after throughput evidence is
recorded; throughput showed no regression.

## What Went Well

1. Applying the SI-13 lesson up front (build the owner before returning the
   component future) avoided repeating that defect; the drop test was
   mutation-checked the same way.
2. Writing the two legacy regression tests before the adapter made the gaps
   concrete: one failed with the `Failed to install stop signal` panic, the
   other left the socket bound after the launcher was aborted.
3. Extracting the receive-error decision into the pure `admit_received`
   function gave deterministic unit tests for D3 without needing to provoke
   real socket errors.
4. Recording the throughput baseline before any code change made the
   performance question answerable with data rather than reasoning.

## What Changed During Implementation

- **AC12 pass rule is two-sided by accident.** The rule "the implementation
  mean is within the baseline's min-max spread" was meant to detect a
  regression, but the after-implementation runs were all above the baseline
  maximum, so the literal rule fails on an improvement. The spread itself was
  wide (about 8.9% of the mean on a desktop with background load), so this
  benchmark only rules out large regressions.
- **A pre-existing legacy example panic surfaced in M3.** Stopping the
  standalone `udp_only_public_tracker` example with Ctrl-C panics with
  `FailedToStartOrStopServer("Normal")`. A baseline build showed the identical
  panic, so it predates SI-14: the example and the legacy launcher both listen
  for the OS signal. It is recorded in the SI-17 draft.
- **The independent review found remaining ownership and hygiene gaps.** The
  token-aware start path held the raw loop handle across the registration
  await (a dropped start future would leave the loop running until its token
  was cancelled), token-aware traces still named the legacy launcher, some test
  awaits were unbounded, and the T5 test-design review and this retrospective
  were missing. All were fixed in "fix(udp-server): [#2342] address SI-14
  completion-review findings" and the following documentation commit.
- **Progress-log times drifted.** Several entries were written in local time
  (UTC+1) or estimated instead of taken from commits and log files, which put
  them out of order. They were corrected from commit and file timestamps.

## Root Cause

- The performance criterion was written as a band around the baseline instead
  of a one-sided bound, because the question asked was "does it affect
  performance?" rather than "can it make performance worse?".
- The ownership rule "move handles into a drop-safe owner before any await or
  return" was applied at the component boundary but not re-applied inside the
  package start function, which has its own await (registration).
- Timestamps were typed from memory rather than copied from `git log` or log
  files.

## Improvements for Future Work

1. Write performance acceptance criteria as a one-sided bound against the
   baseline (for example, "the after-implementation mean is not below the
   baseline minimum") and state the observed noise level so readers know which
   regressions the benchmark can detect.
2. When reviewing task ownership, check every `.await` between spawning a task
   and handing its handle to an owner, not only the final return.
3. Take progress-log and evidence times from `git log --date=format-local` with
   `TZ=UTC` or from log timestamps.
4. When a manual scenario exercises a legacy path, run the same scenario on
   the baseline before attributing any failure to the change.

## Avoiding Overcorrection

- No new owner abstraction is justified yet: `OwnedTask`,
  `TokenAwareServerTask`, and the package-private `OwnedReceiveLoop` differ in
  crate and handle count. Consolidation is listed for SI-19.
- The load test does not need to become a CI gate; a manual before/after
  measurement per hot-path change is proportionate.

## Evidence

- `ISSUE.md` progress log and acceptance verification
- `agent-review-reports.md` (independent Task Reviewer, 2026-09-26)
- `manual-verification-evidence.md` M1-M3
- `performance-evidence.md` M4
- Commits "fix(udp-server): [#2342] make the legacy launcher an adapter over
  the token-aware loop" and "fix(udp-server): [#2342] address SI-14
  completion-review findings"
