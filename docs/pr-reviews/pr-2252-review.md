---
doc-type: pr-review-audit
pr-number: 2252
pr-url: https://github.com/torrust/torrust-tracker/pull/2252
last-updated-utc: 2026-09-17 13:42
---

<!-- cspell:disable -->

# PR #2252 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2252.

## Ownership

The PR author owns this tracked audit record. Reviewers, including Copilot, deliver findings through GitHub and have no repository-artifact obligation.

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2252-f1 | Copilot | Major (inferred) | maintainability | ORIGINAL | FOLLOW_UP | RESOLVED |
| F2 | review-finding:pr-2252-f2 | Copilot | Major (inferred) | correctness | ORIGINAL | SUPERSEDED | SUPERSEDED |
| F3 | review-finding:pr-2252-f3 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2252-f4 | Copilot | Blocker (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2252-f5 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Preserve the published job API

- PR number: 2252
- Source review ID: PRR_kwDOGp2yqc8AAAABOBOVEA
- Source URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4036943513
- Concern: The `pub fn run_job` signature changes from an absolute cutoff timestamp to `max_peer_timeout`, which may affect external consumers if this published package is consumed outside the workspace.
- Solution: FOLLOW_UP. The package has `publish.workspace = true`, so compatibility must be rechecked before a future release. This PR needs the timeout policy to reach the job so the cutoff can be recomputed on every tick; preserving the old signature would preserve the stale-capture design. The compatibility audit and any API migration belong in `follow-up-issue-draft.md`, not in this focused bug fix.
- Current-tree verification: `rg -n '^(name|publish|repository)' packages/swarm-coordination-registry/Cargo.toml` confirms the package is publishable; `follow-up-issue-draft.md` requires a public-API and downstream-consumer audit before a rename or compatibility change.
- Resolution reference: Follow-up documented in `docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/follow-up-issue-draft.md`; https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038268468
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038268468

### F2 - Avoid epoch fallback when `now_sub` fails

- PR number: 2252
- Source review ID: PRR_kwDOGp2yqc8AAAABOBOVEA
- Source URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4036943568
- Concern: `unwrap_or_default()` could turn a clock-underflow error into an epoch cutoff.
- Solution: SUPERSEDED. The thread is outdated after the cutoff implementation was moved and the current tree recomputes the cutoff at each update. The existing clock contract and pre-existing fallback remain unchanged; changing error semantics is outside this focused correction.
- Current-tree verification: The thread is marked outdated by GitHub; the current implementation is `CurrentClock::now_sub(...).unwrap_or_default()` inside `update_activity_metrics`, and the regression plus full pre-push validation pass.
- Resolution reference: `Superseded by F2: the reviewed line is outdated after the cutoff computation moved into the per-tick update path; the existing fallback is unchanged.`; https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038271080
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038271080

### F3 - Avoid hard-coded test interval

- PR number: 2252
- Source review ID: PRR_kwDOGp2yqc8AAAABOBOVEA
- Source URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4036943610
- Concern: The regression test hard-codes 15 seconds and reads the gauge once, coupling it to the job schedule.
- Solution: FIXED. Introduced `ACTIVITY_METRICS_UPDATE_INTERVAL_SECS` and used it in both production scheduling and the stopped-clock test.
- Current-tree verification: `cargo test -p torrust-tracker-swarm-coordination-registry it_should_recompute_inactivity_cutoff_before_each_update_tick -- --nocapture` passes; the test now advances by the shared constant.
- Resolution reference: `fix(swarm-coordination-registry): address PR review findings`; https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038272236
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038272236

### F4 - Restore the missing `Future` import

- PR number: 2252
- Source review ID: PRR_kwDOGp2yqc8AAAABOB07FQ
- Source URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4037448509
- Concern: The bootstrap file uses `impl Future` without importing `std::future::Future`.
- Solution: FIXED. Restored `use std::future::Future;`.
- Current-tree verification: `cargo check -p torrust-tracker --lib` passes.
- Resolution reference: `fix(swarm-coordination-registry): address PR review findings`; https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038273396
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038273396

### F5 - Shorten stopped-clock test syntax

- PR number: 2252
- Source review ID: PRR_kwDOGp2yqc8AAAABOB07FQ
- Source URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4037448582
- Concern: The stopped-clock UFCS path is unnecessarily verbose.
- Solution: FIXED. Added explicit `StoppedClock` and `StoppedClockTrait` aliases and used the shorter UFCS call.
- Current-tree verification: The focused regression test passes with the shortened call.
- Resolution reference: `fix(swarm-coordination-registry): address PR review findings`; https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038274907
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2252#discussion_r4038274907

## Processing Log

- 2026-09-17 13:20 UTC - Fetched all GraphQL review threads for PR #2252 and identified five unresolved threads, including one outdated thread.
- 2026-09-17 13:35 UTC - Classified F1 as a follow-up compatibility concern, F2 as superseded, and F3-F5 as actionable fixes.
- 2026-09-17 13:42 UTC - Applied the actionable code fixes; focused regression test, main crate compile check, and `git diff --check` passed.
- 2026-09-17 14:05 UTC - Pushed signed commit `fix(swarm-coordination-registry): address PR review findings`, replied to all five threads, resolved them through the GraphQL fallback, and refreshed the thread list with no unresolved output.

## Completion Checklist

- [x] GraphQL data collected for all review threads
- [x] Review bodies split into independent findings
- [x] Findings classified and mapped to current tree
- [x] Every action verified against current tree, validated, and committed
- [x] Every resolvable thread replied to before resolving it
- [x] Final GraphQL fetch reports no unresolved actionable thread
- [ ] Audit committed separately from product fixes
