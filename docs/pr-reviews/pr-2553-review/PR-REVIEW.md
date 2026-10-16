---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2413-2410-si-22-1-document-event-flows-and-draft-adr/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2553 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2553>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

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

## Findings

Copilot's review 5546625913 carries two inline threads, F1 and F2. Its overview is a summary of
those two findings and adds no row. The inline comments have no `[Severity]` bracket, so their
severity is inferred as `Minor` from the overview's "Low severity" badge on each finding.

da2ce7's approving review 5547031766 carries eleven inline threads, F3 to F13, each with a
`[Severity][Fn]` header. His IDs F1 to F11 collide with this audit's F1 and F2, so they are
reassigned in source order and the original ID is recorded as the reviewer finding ID. His review
body repeats no further request and adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2553-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2553-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2553-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2553-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2553-f5 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2553-f6 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2553-f7 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2553-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2553-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2553-f10 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2553-f11 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2553-f12 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2553-f13 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The glossary promises that server drains also preserve running handlers

- PR number: 2553
- Source review ID: 5546625913
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4293735143>
- Concern: the glossary's Drain entry said a drain does not interrupt a running handler, but the UDP
  server aborts unfinished request processors at its drain deadline and the HTTP server forces
  shutdown at its drain timeout. The guarantee holds only for the planned listener drain.
- Solution: the reviewer was right. The Drain and Drain timeout entries now separate a server drain
  deadline, which is a hard limit, from the planned listener admission timeout, which is checked
  only between events. They keep the shared-deadline abort exception.
- Current-tree verification: read from source, not run. `packages/udp-server/src/server/launcher.rs`
  aborts remaining processors with `tasks.abort_all()` after the drain deadline, and
  `packages/axum-server/src/signals.rs` calls `handle.shutdown()` when its drain timer fires.
  `docs/architecture/glossary.md` now has a "server drain deadline" and a "listener admission
  timeout" and no longer says every drain avoids interrupting a handler.
- Resolution reference: docs(architecture): [#2413] distinguish server drain deadlines from listener admission timeouts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4293792829>

### F2 - The producer inventory omits the UDP receive loop

- PR number: 2553
- Source review ID: 5546625913
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4293735240>
- Concern: `packages/udp-server/src/server/launcher.rs` also publishes `UdpRequestReceived`,
  `UdpRequestAborted`, `UdpRequestDiscarded` and `UdpRequestBanned` through an awaited helper, in
  the receive loop and not in the processor tasks, so the inventory omitted a producer execution
  path.
- Solution: the reviewer was right, and the miss was in the T1 method: it searched for `.send(`
  call sites, and the launcher reaches `send` through `publish_event_if_sender_available`. The
  UDP-server row now lists the receive loop and its four events, the producer send path section
  explains why a `.send(` search misses them and names both UDP execution paths, and a dated
  correction in the sub-EPIC `manual-verification-evidence.md` replaces the earlier "no unlisted
  producer" result. A re-sweep of every file that holds a bus sender found the launcher to be the
  only missing one.
- Current-tree verification: read from source, not run.
  `rg -n 'UdpRequestReceived|UdpRequestAborted|UdpRequestDiscarded|UdpRequestBanned' packages/udp-server/src/server/launcher.rs`
  shows the four publish sites at lines 385, 441, 472 and 499, all through the awaited helper at
  line 582. The inventory row and send-path section name them.
- Resolution reference: docs(shutdown): [#2413] add the UDP receive loop to the event producer inventory
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4293793074>

### F3 - Section 9 of the ADR draft drops the old ADR's Consequences

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064360>
- Concern: Section 9 says it restates every point that remains valid, but it omits the old ADR's Consequences: the add, migrate, deprecate, remove rule for lifecycle APIs, the deterministic-tests and executable-evidence rule, the inventory revisit rule, and the progressive-implementation statement.
- Solution: Restated all four in items 8 to 11 of section 9, and moved the rejected alternatives to item 12. The #1588 condition of the old inventory rule is recorded as lapsed, because that issue is closed.
- Current-tree verification: Read from source, not run: `adr-draft.md` section 9 has items 8 to 12; item 10 names the revisit-on-change rule and the lapsed #1588 condition.
- Resolution reference: docs(adrs): [#2413] restate the cancellation-tree ADR consequences in the draft
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294185904>

### F4 - The #2413 evidence is in the sub-EPIC file and the allowance is misattributed

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064376>
- Concern: The sub-EPIC's Shared Delivery Gates ask each child to keep its evidence in its own `manual-verification-evidence.md` and to obtain a Task Reviewer report and a retrospective. The #2413 evidence lived in the sub-EPIC's file, and the documentation-only allowance belongs to EPIC #1488, not to the sub-EPIC.
- Solution: Moved the #2413 sections to an issue-local `manual-verification-evidence.md` that links the sub-EPIC file as the EPIC baseline, left a pointer in the sub-EPIC file, attributed the allowance to EPIC #1488, and listed the Task Reviewer report and retrospective gates in the spec as unchecked checkpoints. `agent-review-reports.md` is the reviewer's to create, and M1 and the acceptance criteria stay unchecked.
- Current-tree verification: The #2413 folder holds `ISSUE.md` and `manual-verification-evidence.md`; the sub-EPIC file has a pointer section; the spec's Workflow Checkpoints list the two gates unchecked.
- Resolution reference: docs(issues): [#2413] move the #2413 evidence into an issue-local file
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294186164>

### F5 - The audit records Major (inferred) where the source badge is Low severity

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064385>
- Concern: Both round 1 rows recorded `Major (inferred)`, but the only source severity is the overview's "Low severity" badge, which earlier audits record as `Minor (inferred)`, with a sentence explaining the inference.
- Solution: Changed both rows to `Minor (inferred)` and added the badge sentence to the Findings introduction. The Processing Log was not rewritten; the entry below records the correction.
- Current-tree verification: The tracking rows for F1 and F2 say `Minor (inferred)`, and the Findings introduction explains the inference.
- Resolution reference: docs(reviews): [#2413] record Minor (inferred) severity for the PR 2553 round 1 findings
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294186365>

### F6 - Only the persistent listener is marked conditional in the inventory

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064396>
- Concern: The swarm and the tracker-core in-memory listeners also start only when `core.tracker_usage_statistics` is on, so "the five core listeners" are at most five.
- Solution: Marked both listeners as conditional in the swarm-bus row and replaced "the five core listeners" with the exact condition of each.
- Current-tree verification: Read from source, not run: `src/bootstrap/jobs/torrent_repository.rs` and `src/bootstrap/jobs/tracker_core.rs` gate the two listeners on `config.core.tracker_usage_statistics`; the inventory row and startup sentence say so.
- Resolution reference: docs(shutdown): [#2413] mark the swarm and in-memory listeners as conditional in the inventory
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294186555>

### F7 - Statuses lag the work this PR completes

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064409>
- Concern: The spec stayed `status: planned` and the sub-EPIC rows stayed `TODO` although T1 to T2c are done and the PR is open.
- Solution: Set the spec to `in-review`, the sub-EPIC's #2413 row to `IN_PROGRESS`, and plan rows T1 and T2 to `DONE`. Acceptance criteria stay unchecked.
- Current-tree verification: `ISSUE.md` has `status: in-review`; the sub-EPIC row and plan rows carry the new statuses; `in-review` is in the validator's accepted list.
- Resolution reference: docs(issues): [#2413] update the statuses to match the work this PR completes
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294186822>

### F8 - D13 still names tokio 1.53.1 as the Cargo.lock version

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064418>
- Concern: D13 says tokio 1.53.1 is "the version in `Cargo.lock`", but `Cargo.lock` has 1.53.3, and the draft cites 1.53.3.
- Solution: Corrected the decision text in place to say the claims were verified in 1.53.1 on 2026-10-01 and re-read in 1.53.3 on 2026-10-16 (read from source, not run). Discussion-record entry 19 keeps its dated 1.53.1 statement.
- Current-tree verification: `EPIC.md` D13 names 1.53.3 as the `Cargo.lock` version; `Cargo.lock` has `tokio` 1.53.3.
- Resolution reference: docs(issues): [#2413] correct the tokio version in the D13 decision text
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294187045>

### F9 - The tooling-defect evidence has an unrecorded time and no toolchain

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064430>
- Concern: "09:3x UTC" is not a recorded time, and the red and green commands omit the toolchain.
- Solution: The symptom time is 08:30 UTC (the stray commit's committer date was `09:30:35 +01:00`; the earlier line mislabelled local time). The evidence now records the toolchain and a timestamped rerun of red, green and the like-for-like recheck on the final test file. The first runs' exact minutes were not captured, and the file says so.
- Current-tree verification: The issue-local `manual-verification-evidence.md` has the Environment bullet, the 08:30 UTC symptom, and the 11:24 UTC rerun.
- Resolution reference: docs(issues): [#2413] record the minute and toolchain of the isolation evidence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294187219>

### F10 - The two UdpRequestDiscarded sites guard the same case

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064440>
- Concern: The processor's `UdpRequestDiscarded` publish guards the same source-port-zero case as the receive loop's and has no production path, so the qualifier on the receive loop alone misleads.
- Solution: Qualified both entries in the UDP-server row and the send-path paragraph: the receive loop's is the production discard site, and the processor's is defence in depth, unreachable from the tracker because the receive loop discards port 0 first.
- Current-tree verification: Read from source, not run: `processor.rs` (guard and comment) and `launcher.rs` (the only production `process_request` caller, after the port-0 discard); the inventory says both.
- Resolution reference: docs(shutdown): [#2413] qualify the two UdpRequestDiscarded publish sites in the inventory
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294187420>

### F11 - The inventory stamp did not move with its change

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064453>
- Concern: `last-updated-utc` stayed at 07:56 after the UDP-server row and send-path section changed.
- Solution: Set it to 11:21, the time of the last content change, in its own commit.
- Current-tree verification: The inventory's `last-updated-utc` is `2026-10-16 11:21`.
- Resolution reference: docs(shutdown): [#2413] bump the task inventory stamp to its last change
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294187663>

### F12 - Record that the exit-result policy is not applied yet

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064463>
- Concern: `main` discards the `Vec<JobOutcome>` from `JobManager::wait_for_all` and logs a successful shutdown, so a shutdown timeout or abort exits with code 0 today.
- Solution: Added the statement to item 6 of section 9, next to the budgets difference.
- Current-tree verification: Read from source, not run: `src/main.rs` calls `jobs.wait_for_all(...)` and ignores its result; item 6 of the draft says so.
- Resolution reference: docs(adrs): [#2413] note that the exit-result policy is not applied yet
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294187969>

### F13 - The regression test checks the helper against its own constant

- PR number: 2553
- Source review ID: 5547031766
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294064480>
- Concern: The test compares `fixture_command` with the constant it iterates, so it does not notice a Git release that adds a repository-local variable, nor a later `Command::new` that bypasses the helper.
- Solution: Added a test that compares `REPOSITORY_ENVIRONMENT` with `git rev-parse --local-env-vars` at test time, which closes the first gap. The second gap is declined with a reason in the reply: a test cannot see future code, a source-scanning test would be brittle, and the helper is the single seam all three existing call sites use.
- Current-tree verification: `it_should_list_every_variable_git_reports_as_repository_local` exists; the mutation red and the restored green are in the issue-local evidence file; `cargo test -p clippy-allow-reasons --test cli` gives 10 passed.
- Resolution reference: test(dev-tools): [#2413] check the fixture environment list against git rev-parse --local-env-vars
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294188226>

## Processing Log

- 2026-10-16 10:42 UTC - Fetched the review and both inline threads, classified them as Copilot
  findings F1 and F2, and re-derived each against the current tree. Both were valid.
- 2026-10-16 10:45 UTC - Fixed F1 and F2 in separate signed commits, and corrected the M1 evidence
  record in a third.
- 2026-10-16 10:47 UTC - Pushed, replied on both threads (and corrected a garbled phrase in the F1
  reply in place).
- 2026-10-16 10:48 UTC - Started this record, with a reply URL in every row.
- 2026-10-16 11:16 UTC - da2ce7's approving review 5547031766 arrived with eleven inline threads. Fetched
  them, reassigned his IDs F1 to F11 to F3 to F13 because they collide with F1 and F2, and re-derived each
  against the current tree. All were valid.
- 2026-10-16 11:28 UTC - Fixed F3 to F13 in separate signed commits. F5 corrected the severity of the F1 and
  F2 rows above to `Minor (inferred)` in place; they previously said `Major (inferred)`.
- 2026-10-16 11:32 UTC - Pushed, replied on all eleven threads, and resolved them. Added F3 to F13 to this
  record, with a reply URL in every row.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
