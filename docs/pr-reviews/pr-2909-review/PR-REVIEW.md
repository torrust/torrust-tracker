---
pr-number: 2909
pr-url: https://github.com/torrust/torrust-tracker/pull/2909
last-updated-utc: "2026-11-17 13:59"
---

<!-- skill-link: process-pr-review -->

# PR #2909 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2909.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1: da2ce7 review 5939688807 (CHANGES_REQUESTED) with findings F1 to F6; the reviewer's IDs are this audit's IDs.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2909-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2909-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2909-f3 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2909-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2909-f5 | Human | Suggestion | maintainability | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2909-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Alternative D rejected on a false premise

- PR number: 2909
- Source review ID: 5939688807
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600500956
- Concern: Alternative D was rejected because "real components select on their token while waiting", and the Background said no production component ignores cancellation, but `tracker_core_persistent_completed_statistics_event_listener` awaits each database write inside the `recv()` arm, outside the token branch, and its drain finishes an event it has taken, so a stalled MySQL write hangs a real component until `P`.
- Solution: The Background now says a production component can hang on a stalled dependency, names the listener and the two files, and explains why a synthetic component is used instead. Row D is rejected on cost (a database container and a completed-download event timed into the shutdown window), not feasibility. Case 1 names the listener as a real instance.
- Current-tree verification: `grep -n 'Rejected on cost, not feasibility' ISSUE.md` finds row D; `grep -c 'which no production component does' ISSUE.md` prints 0; `sed -n 147,153p packages/tracker-core/src/statistics/event/listener.rs` shows the write awaited in the `recv()` arm.
- Resolution reference: `docs(issues): [#2847] address review round 1 of PR 2909`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600581154

### F2 - A plain move changes the log target

- PR number: 2909
- Source review ID: 5939688807
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600500969
- Concern: The default fmt subscriber prints each event target; moving the events from the binary crate into the library changes it from `torrust_tracker` to `torrust_tracker_lib::app`, so "the same log lines" in D2, the first acceptance criterion and M4 would not hold.
- Solution: D2 says the moved events pin `target: "torrust_tracker"`, so the lines stay identical, target included. The first acceptance criterion and M4 now name the target.
- Current-tree verification: `grep -n 'pin .target' ISSUE.md` finds D2; `grep -n 'target included' ISSUE.md` finds D2 and the first acceptance criterion; `grep -n 'lines and targets' ISSUE.md` finds M4.
- Resolution reference: `docs(issues): [#2847] address review round 1 of PR 2909`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600581617

### F3 - A 4 s deadline alone fails validation

- PR number: 2909
- Source review ID: 5939688807
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600500979
- Concern: `deadline_in_secs = 4` with the default drains (15 s and 5 s) fails `Shutdown::validate`, so a reader following T3 or M1 gets a startup failure.
- Solution: T3 states the validation rule and names `deadline_in_secs = 4` with the three drain budgets at `1`, as `tests/lifecycle/shutdown_policy.rs` lowers them; M1 refers to the same budgets.
- Current-tree verification: `grep -n 'all three drain budgets' ISSUE.md` finds M1; `grep -n 'event_listener_drain_in_secs. all .1.' ISSUE.md` finds T3; `sed -n 221,232p packages/configuration/src/v3_0_0/shutdown.rs` shows the rule.
- Resolution reference: `docs(issues): [#2847] address review round 1 of PR 2909`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600582121

### F4 - Case 4 mechanism does not fit spawn_blocking

- PR number: 2909
- Source review ID: 5939688807
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600500989
- Concern: A component that awaits a `spawn_blocking` handle is at an await point and is aborted and joined; what keeps the process alive past `P` is the runtime drop waiting for the blocking thread, not a missed abort.
- Solution: Case 4 is split into a self-blocking task (never reaches an await point, so `wait_for_all` does not return at `P`) and a `spawn_blocking` thread (the task is aborted, but the runtime drop waits for the thread).
- Current-tree verification: `grep -n 'dropping the runtime' ISSUE.md` finds the new sentence.
- Resolution reference: `docs(issues): [#2847] address review round 1 of PR 2909`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600582518

### F5 - Interim .dockerignore block does not fit the crate

- PR number: 2909
- Source review ID: 5939688807
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600501000
- Concern: The only `.dockerignore` block for members outside the admitted roots is the interim AI-harness one, for crates with no tracker dependency; this crate depends on the root crate.
- Solution: The proposed path moves to `packages/hung-component-tracker`, which `.dockerignore` already admits, like the non-default `packages/e2e-tools`; D3 says no admission line is needed and why the interim block does not fit. The T2 row matches.
- Current-tree verification: `grep -n 'packages/hung-component-tracker' ISSUE.md` finds D1; `grep -c 'contrib/dev-tools/testing' ISSUE.md` prints 0; `grep -n '^!/packages/' .dockerignore` finds the admission.
- Resolution reference: `docs(issues): [#2847] address review round 1 of PR 2909`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600582870

### F6 - EPIC stamp not bumped with row 25

- PR number: 2909
- Source review ID: 5939688807
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600501008
- Concern: Row 25 of the EPIC changed but its `last-updated-utc` kept the previous value.
- Solution: The EPIC `last-updated-utc` is bumped to the time of this round.
- Current-tree verification: `sed -n 9p docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md` prints the new stamp.
- Resolution reference: `docs(issues): [#2847] address review round 1 of PR 2909`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2909#discussion_r4600583256

## Processing Log

- 2026-11-17 13:57 UTC - Started audit for round 1: da2ce7 review 5939688807 (CHANGES_REQUESTED), findings F1 (Major), F2 (Minor), F3 (Minor), F4 (Nit), F5 (Suggestion) and F6 (Nit). Committed the six fixes in one commit (13:56:06Z) before creating this record and before any reply.
- 2026-11-17 13:59 UTC - Pushed the fix commit (head 630207df0 on the bot fork, checked with `git ls-remote`), then replied to the six threads (13:59:26Z to 13:59:37Z) and recorded the reply URLs.

<!-- Copied verbatim into each audit record. -->

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
