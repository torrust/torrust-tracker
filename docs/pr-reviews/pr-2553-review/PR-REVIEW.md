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
body repeats no further request and adds no row. GitHub's timeline shows the next fix push dismissing
this review at 11:30:11Z.

His approving review 5547366758, at the head after the round 2 fixes, carries four more inline threads,
F14 to F17, with the same ID reassignment. It repeats no further request in its body and adds no row. The next fix push dismissed it at 11:56:24Z.

His approving review 5547599741, at the head after the round 3 fixes, carries four more inline threads,
F18 to F21, all `Nit`, with the same ID reassignment. It repeats no further request in its body and adds
no row. The next fix push dismissed it at 12:20:03Z.

His approving review 5547834689, at the head after the round 4 fixes, carries three more inline threads,
F22 to F24 (two `Nit` and one `Suggestion`), with the same ID reassignment. It repeats no further request
in its body and adds no row. The next fix push dismissed it at 12:41:19Z.

His review 5562034258, a re-review at the head after the retrospective, carries three more inline threads,
F25 to F27, all `Nit`, with the same ID reassignment. It repeats no further request in its body and adds no
row. The next fix push dismissed it at 2026-10-19 11:12:26Z. A conversation comment of 2026-10-19 10:40 UTC
answers the question in the re-review request about issue #2503 and asks for no change here.

His approving review 5570845377, at the head after the round 7 fixes, is round 8 (his ACK calls its fix the
round-8 fix). The numbering used here is the spec's, the sub-EPIC's and the retrospective's: round 6 is review
5548045897, round 7 is review 5562034258, and round 8 is review 5570845377. Two commit subjects use an earlier
numbering and are not reworded, because the repository keeps a branch's history: `docs(reviews): [#2413]
record the PR 2553 review round 6 audit` records round 7, and `docs(reviews): [#2413] record the PR 2553
review round 7 audit` records round 8. Round 8 carries one more inline thread, F28 (`Nit`), with the same ID
reassignment. It repeats no further request in its body and adds no row. The next fix push dismissed it at
2026-10-19 11:31:25Z.

His approving review 5571054785, at the head after the round 8 fix, is round 9. It carries three more inline
threads, F29 to F31 (all `Nit`), with the same ID reassignment. It repeats no further request in its body and
adds no row. The next fix push dismissed it at 2026-10-19 11:53:05Z.

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
| F14 | review-finding:pr-2553-f14 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2553-f15 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2553-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2553-f17 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2553-f18 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2553-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2553-f20 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2553-f21 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2553-f22 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2553-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2553-f24 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2553-f25 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2553-f26 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2553-f27 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-2553-f28 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F29 | review-finding:pr-2553-f29 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F30 | review-finding:pr-2553-f30 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F31 | review-finding:pr-2553-f31 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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

### F14 - The rerun's stamps show dependent steps issued together

- PR number: 2553
- Source review ID: 5547366758
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294333111>
- Concern: Items 1 to 3 of the rerun carry the same second, and item 6 carries a stamp before items 4 and 5, so the evidence is not shown to have run one step at a time, as the template requires. The recheck is not shown to have run on the restored file.
- Solution: Discarded the 11:24 rerun and recorded a new one on the final test file: six steps, one tool call each, with millisecond stamps. The evidence names the `git diff --quiet HEAD -- <test file>` check and where it ran: before the run at steps 1 and 6, and right after the restore at steps 3 and 5. It also records that the last step's end stamp precedes its start stamp.
- Current-tree verification: The issue-local `manual-verification-evidence.md` has the section `Sequential rerun on the final test file` with the six steps; the discarded rerun is described and no longer listed as evidence.
- Resolution reference: docs(issues): [#2413] rerun the isolation evidence one step at a time
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294393176>

### F15 - The M1 row still says TODO

- PR number: 2553
- Source review ID: 5547366758
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294333119>
- Concern: M1 ran and is recorded, with its correction, in the issue folder, but its row in the spec still says `TODO`. A scenario's status belongs to whoever runs it.
- Solution: Set the M1 row to `DONE`. The acceptance criteria and the two delivery-gate checkpoints stay unchecked for the reviewer.
- Current-tree verification: The M1 row of the spec's Manual Verification Scenarios table ends with `DONE`.
- Resolution reference: docs(issues): [#2413] mark manual scenario M1 as done in the spec
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294393423>

### F16 - The EPIC allowance is cited without a section

- PR number: 2553
- Source review ID: 5547366758
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294333125>
- Concern: The evidence cites "the Manual Verification task rule", which is not a heading of EPIC #1488's `ISSUE.md`.
- Solution: Cited the last sentence of the manual-verification bullet under "Release and Review Requirements", lines 279-280.
- Current-tree verification: `docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md` has the heading at line 260 and the sentence at lines 279-280, and the evidence file cites both.
- Resolution reference: docs(issues): [#2413] cite the EPIC documentation-only allowance by section and line
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294393617>

### F17 - Exact equality makes the test depend on the installed Git in both directions

- PR number: 2553
- Source review ID: 5547366758
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294333128>
- Concern: A newer Git that adds a name fails the test, which is the intended alarm, but an older Git that reports fewer than the 15 names fails it too, although the fixtures stay isolated.
- Solution: Changed the assertion to a subset check: every name Git reports must be in `REPOSITORY_ENVIRONMENT`. The alarm for a newer Git stays and the false failure for an older Git goes.
- Current-tree verification: `it_should_list_every_variable_git_reports_as_repository_local` filters Git's names against the constant; the sequential rerun shows red when the constant lacks `GIT_COMMON_DIR` and green on the committed file.
- Resolution reference: test(dev-tools): [#2413] assert that the fixture environment list covers Git's, not equals it
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294393827>

### F18 - The step 6 note reads more into the stamps than they show

- PR number: 2553
- Source review ID: 5547599741
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294519684>
- Concern: The note says the host clock was not monotonic, but a wall clock is never guaranteed monotonic, and a one-digit slip in the end stamp explains the stamps equally. The bytes cannot tell a step from a slip.
- Solution: The note now says only that the end stamp precedes the start stamp, that the cause is unknown, and that the stamps still show step 6 starting and ending after step 5 ended.
- Current-tree verification: The issue-local `manual-verification-evidence.md` states the two stamps, says the cause is unknown, and no longer mentions a clock that is not monotonic.
- Resolution reference: docs(issues): [#2413] state only what the step 6 stamps show
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294577877>

### F19 - F14's Solution names a check the evidence does not record

- PR number: 2553
- Source review ID: 5547599741
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294519692>
- Concern: F14's Solution says each restore was followed by a `git diff --quiet HEAD` check, but the evidence records no such command, only that the file "equals `HEAD`". The sentence also repeats the clock reading of F18.
- Solution: The evidence now names the check, `git diff --quiet HEAD -- <test file>` exiting 0 and printing `file == HEAD`, which ran before the run at steps 1 and 6 and right after the restore at steps 3 and 5. F14's Solution was reworded to state only what the evidence shows, without the clock reading.
- Current-tree verification: The evidence's intro to the sequential rerun names the command, and F14's Solution above matches it.
- Resolution reference: docs(reviews): [#2413] align the F14 solution with the recorded evidence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294578109>

### F20 - The 11:55 log entry says the fixes were pushed before they were

- PR number: 2553
- Source review ID: 5547599741
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294519700>
- Concern: The round 3 log entry at 11:55 says F14 to F17 were fixed and pushed, but the push came later: GitHub's timeline dismissed review 5547366758 at 11:56:24Z.
- Solution: The Processing Log is append-only, so a correction entry was appended that gives the push time as about 11:56, and the 11:55 entry is left as written.
- Current-tree verification: The Processing Log has the 12:15 correction entry after the 11:55 entry; the validator passes.
- Resolution reference: docs(reviews): [#2413] correct the push time in the round 3 processing log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294578334>

### F21 - A commit body claims a failure that no record shows

- PR number: 2553
- Source review ID: 5547599741
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294519712>
- Concern: The body of the commit that made the Git-variable test a subset check says the equality comparison "failed on an older Git that reports fewer names", but no such run is recorded. The evidence's Git (2.53.0) reports the same 15 names, and the case was only a possibility raised in review.
- Solution: The commit body is not rewritten, because the repository keeps a branch's history and a rewrite would change the commit ids the review rounds cite. The issue-local evidence has a section that records the correction and what the body should have said.
- Current-tree verification: The evidence has the section `Correction: the older-Git claim in a commit body`; the test file's own comment says "may report".
- Resolution reference: docs(issues): [#2413] correct the older-Git claim in a commit body
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294578624>

### F22 - Three statements of the HEAD-equality check disagree

- PR number: 2553
- Source review ID: 5547834689
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294705110>
- Concern: The evidence uses "equals `HEAD`" at steps 1, 3 and 5 only, F14's Solution said the check showed equality at steps 1, 3, 5 and 6, and F19's Solution said it ran before steps 1, 3, 5 and 6, as did the reply on its thread. Before steps 3 and 5 the file still carried a mutation, so a check there could not have exited 0.
- Solution: One placement, taken from the commands that ran: `git diff --quiet HEAD -- <test file>` ran before the run at steps 1 and 6 and right after the restore at steps 3 and 5. The evidence, F14's Solution and F19's Solution now say so, the evidence's step 6 records the check, and the reply on this thread says that the earlier reply on F19's thread was wrong.
- Current-tree verification: The evidence's intro to the sequential rerun and its step 6 state the placement, and F14's and F19's Solutions above repeat it.
- Resolution reference: docs(reviews): [#2413] state one placement for the HEAD-equality check
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294755266>

### F23 - The 12:19 log entry says the fixes were pushed before they were

- PR number: 2553
- Source review ID: 5547834689
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294705122>
- Concern: The round 4 log entry at 12:19 says F18 to F21 were fixed and pushed, but the push came later: GitHub's timeline dismissed review 5547599741 at 12:20:03Z.
- Solution: The Processing Log is append-only, so a correction entry was appended, as for round 3. It gives the push time as about 12:20, and the 12:19 entry is left as written.
- Current-tree verification: The Processing Log has the 12:37 correction entry after the 12:19 entry; the validator passes.
- Resolution reference: docs(reviews): [#2413] correct the push time in the round 4 processing log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294755504>

### F24 - The audit calls the reviews approving without noting their dismissal

- PR number: 2553
- Source review ID: 5547834689
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294705128>
- Concern: The audit calls da2ce7's reviews approving, but each was dismissed by the next fix push, as sibling audits note.
- Solution: Added one clause per round to the Findings introduction with the dismissal time from GitHub's timeline (11:30:11Z, 11:56:24Z and 12:20:03Z), and the same clause for this round's review (12:41:19Z).
- Current-tree verification: The Findings introduction has a dismissal clause after each of da2ce7's four reviews that were dismissed by a fix push, and none after Copilot's review, which was not dismissed.
- Resolution reference: docs(reviews): [#2413] note the dismissal of each approving review in the audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4294755748>

### F25 - The retrospective's audit-only count conflates two counts

- PR number: 2553
- Source review ID: 5562034258
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4306265333>
- Concern: The Review Summary row says all 11 `docs(reviews)` commits are audit-only, but the template's audit-only count covers commits that touch nothing but the audit, and only four do.
- Solution: Wrote "11 `docs(reviews)`, 4 of them audit-only: they touch nothing but `PR-REVIEW.md`".
- Current-tree verification: `git show --stat` for each of the 11 `docs(reviews)` commits up to the approved head lists only `PR-REVIEW.md` for four of them, and a file list for the other seven that includes the spec, the sub-EPIC or the evidence; the retrospective row says so.
- Resolution reference: docs(reviews): [#2413] count the audit-only commits as the template defines them
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313138453>

### F26 - The skill's round count disagrees with the retrospective's

- PR number: 2553
- Source review ID: 5562034258
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4306265340>
- Concern: The new `process-pr-review` section and the commit body say three of five rounds found nothing but statements in the author's records, but the retrospective counts round 3 as also having a deliverable finding (F17), so the measured count is two of the five rounds with findings.
- Solution: Changed the skill sentence to the measured count (two of the five rounds with findings, rounds 4 and 5, and most findings of rounds 3 to 5). The commit body is not reworded, because the repository keeps a branch's history and a rewrite would change the commit ids the review rounds cite.
- Current-tree verification: `rg 'three of five' .github docs` finds no match in the skill; its section states two of the five rounds with findings; the retrospective's round table agrees.
- Resolution reference: docs(skills): [#2413] state the measured round count in the author-records rules
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313138734>

### F27 - The log entry's commit time range ends a minute late

- PR number: 2553
- Source review ID: 5562034258
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4306265344>
- Concern: The 15:41 entry gives the template and skill commits as "15:35 to 15:39", but they are at 15:35:37Z, 15:36:58Z and 15:38:14Z.
- Solution: The Processing Log is append-only, so a correction entry was appended giving 15:35 to 15:38, and the 15:41 entry is left as written.
- Current-tree verification: `git log --format=%cI` gives those three times for the commits; the Processing Log has the 2026-10-19 11:10 correction after the 15:41 entry; the validator passes.
- Resolution reference: docs(reviews): [#2413] correct the commit time range in the retrospective log entry
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313138972>

### F28 - Two reviews are both called round 6

- PR number: 2553
- Source review ID: 5570845377
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313261505>
- Concern: the spec and sub-EPIC log entries called review 5562034258 "round 6", but the retrospective's round table uses round 6 for the approval review 5548045897, so "round 6" named two different reviews in the tree.
- Solution: renamed the two log entries to round 7, as the reviewer's own ACK calls it, and appended a log entry in each file that records the rename. The audit already gave the review no number. The subject of the commit that says round 6 is not reworded, because the repository keeps a branch's history and a rewrite would change the commit ids that the review rounds cite.
- Current-tree verification: `rg -n 'round 6' docs/issues/open/2413-* docs/issues/open/2410-*` finds two matches, the rename entries in the spec and in the sub-EPIC, which quote "round 6" to record the rename. No entry still calls review 5562034258 round 6, and the retrospective's table uses round 6 only for review 5548045897.
- Resolution reference: docs(issues): [#2413] number the retrospective re-review as round 7 in the spec and sub-EPIC logs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313294108>

### F29 - The 11:29 log entry reports a push that happened later

- PR number: 2553
- Source review ID: 5571054785
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313428822>
- Concern: The 2026-10-19 11:29 entry reports the push of the F28 fix, but GitHub's timeline dismissed review 5570845377 at 11:31:25Z.
- Solution: The Processing Log is append-only, so a correction entry was appended with the push time. Every other entry that reports a push was checked against the timeline dismissals, and none other was stamped before its push.
- Current-tree verification: `gh api repos/torrust/torrust-tracker/issues/2553/timeline` lists the dismissal at 2026-10-19T11:31:25Z; the log has the 11:49 correction entry after the 11:32 entry; the validator passes.
- Resolution reference: docs(reviews): [#2413] correct the push time in the round 8 processing log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313484245>

### F30 - The F28 verification says the search finds no match

- PR number: 2553
- Source review ID: 5571054785
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313428835>
- Concern: The F28 verification says the search for `round 6` finds no match in the spec or the sub-EPIC, but it finds the two rename entries that quote it.
- Solution: Stated the two matches and the narrower fact that holds: no entry still calls review 5562034258 round 6.
- Current-tree verification: `rg -n 'round 6' docs/issues/open/2413-* docs/issues/open/2410-*` prints one line in `ISSUE.md` and one in `EPIC.md`, both rename entries; the F28 verification says that.
- Resolution reference: docs(reviews): [#2413] state what the round 6 search finds in the F28 verification
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313484596>

### F31 - Two commit subjects use the round numbering the tree dropped

- PR number: 2553
- Source review ID: 5571054785
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313428842>
- Concern: The audit places review 5570845377 after the round 7 fixes, but the subject of the commit that records it says round 7, while the spec, the sub-EPIC and the retrospective give round 7 to review 5562034258. The subject of the commit that records review 5562034258 says round 6, which is the retrospective's number for review 5548045897.
- Solution: The audit's Findings introduction now states the numbering the tree uses (round 6 is review 5548045897, round 7 is review 5562034258, round 8 is review 5570845377) and which of the two commit subjects records which round. The subjects are not reworded, because the repository keeps a branch's history and a rewrite would change the commit ids that the review rounds cite. The spec, the sub-EPIC and the PR body were searched for other uses of these round numbers, and none contradicts this numbering.
- Current-tree verification: `git log --format=%s torrust/develop..HEAD` lists the two subjects; the introduction names both; `rg -n 'round [678]' docs/issues/open/2413-* docs/issues/open/2410-*` prints four log entries, the 11:13 and 11:29 entries in each file, which call review 5562034258 round 7 or quote the old "round 6".
- Resolution reference: docs(reviews): [#2413] state the round numbering and the two commit subjects that predate it
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4313484836>

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
- 2026-10-16 11:49 UTC - da2ce7's approving review 5547366758 arrived with four inline threads. Fetched them,
  reassigned his IDs F1 to F4 to F14 to F17, and re-derived each against the current tree. All were valid.
- 2026-10-16 11:55 UTC - Fixed F14 to F17 in separate signed commits and pushed.
- 2026-10-16 11:57 UTC - Replied on all four threads, resolved them, and added F14 to F17 to this record, with
  a reply URL in every row.
- 2026-10-16 12:14 UTC - da2ce7's approving review 5547599741 arrived with four inline threads. Fetched them,
  reassigned his IDs F1 to F4 to F18 to F21, and re-derived each against the current tree. All were valid.
- 2026-10-16 12:15 UTC - Correction to the 11:55 entry, which said F14 to F17 were fixed and pushed at 11:55:
  the fixes were committed by 11:54, and the push came later, at about 11:56 (GitHub's timeline dismissed
  review 5547366758 at 11:56:24Z). The 11:55 entry is left as written.
- 2026-10-16 12:19 UTC - Fixed F18 to F21 in separate signed commits and pushed (the earlier F20 correction
  and the evidence and audit corrections for F18, F19 and F21 landed between 12:15 and 12:18).
- 2026-10-16 12:21 UTC - Replied on all four threads, resolved them, and added F18 to F21 to this record, with
  a reply URL in every row.
- 2026-10-16 12:36 UTC - da2ce7's approving review 5547834689 arrived with three inline threads. Fetched them,
  reassigned his IDs F1 to F3 to F22 to F24, and re-derived each against the current tree. All were valid.
- 2026-10-16 12:37 UTC - Correction to the 12:19 entry, which said F18 to F21 were fixed and pushed at 12:19:
  the fixes were committed by 12:18, and the push came later, at about 12:20 (GitHub's timeline dismissed
  review 5547599741 at 12:20:03Z). The 12:19 entry is left as written.
- 2026-10-16 12:41 UTC - Fixed F22 to F24 in separate signed commits (committed 12:37 to 12:39) and pushed
  (GitHub's timeline dismissed review 5547834689 at 12:41:19Z).
- 2026-10-16 12:42 UTC - Replied on all three threads, resolved them, and added F22 to F24 to this record, with
  a reply URL in every row.
- 2026-10-16 15:41 UTC - At the maintainer's request, after da2ce7's approval 5548045897 with no finding,
  added `review-retrospective.md` beside this record (commit at 15:41:10Z), with the template and skill
  changes it lists in their own commits (15:35 to 15:39). Written with Claude Opus 5.5; the earlier
  entries were written with Claude Sonnet 5.5, per the maintainer.
- 2026-10-19 11:10 UTC - Correction to the 2026-10-16 15:41 entry, whose range "15:35 to 15:39" ends a minute
  late: the three template and skill commits are at 15:35:37Z, 15:36:58Z and 15:38:14Z, so the range is
  15:35 to 15:38. The 15:41 entry is left as written.
- 2026-10-19 11:12 UTC - da2ce7's re-review 5562034258 had arrived on 2026-10-18 at 08:01 UTC with three inline
  threads, which the author found only on 2026-10-19 (polling had stopped). Reassigned his IDs F1 to F3 to
  F25 to F27 and re-derived each against git; all were valid. The fixes were committed at 11:07:48Z,
  11:08:43Z and 11:10:37Z and pushed at about 11:12Z (the timeline dismissed review 5562034258 at 11:12:26Z).
- 2026-10-19 11:13 UTC - Replied on all three threads, resolved them, and added F25 to F27 to this record, with
  a reply URL in every row.
- 2026-10-19 11:27 UTC - da2ce7's approving review 5570845377 arrived with one inline thread. Fetched it,
  reassigned his ID F1 to F28, and re-derived it against the tree. It was valid.
- 2026-10-19 11:29 UTC - Fixed F28 in one signed commit (committed 11:29:25Z) and pushed (the timeline
  dismissed review 5570845377 at 11:31:25Z).
- 2026-10-19 11:32 UTC - Replied on the thread, resolved it, and added F28 to this record, with a reply URL in
  its row.
- 2026-10-19 11:49 UTC - da2ce7's approving review 5571054785 arrived with three inline threads at 11:47Z.
  Fetched them, reassigned his IDs F1 to F3 to F29 to F31, and re-derived each by running the command it names.
  All were valid. Correction to the 11:29 entry, which reported the push of the F28 fix: the push came at about
  11:31 (GitHub's timeline dismissed review 5570845377 at 11:31:25Z). The 11:29 entry is left as written. The
  other entries that report a push were checked against the timeline dismissals, and this was the only one
  stamped before its push.
- 2026-10-19 11:53 UTC - Fixed F29 to F31 in three signed commits (committed 11:49:20Z, 11:50:12Z and
  11:51:09Z) and pushed (the timeline dismissed review 5571054785 at 11:53:05Z).
- 2026-10-19 11:54 UTC - Replied on all three threads, resolved them, and added F29 to F31 to this record, with
  a reply URL in every row.

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
