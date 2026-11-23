---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3010 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3010>.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 covers two reviews of the first head of the pull request.

Review 6025366087 (Greptile, COMMENTED, submitted 2026-11-23T16:32:02Z) has one inline thread on `decisions.md`. Greptile is classified `Unknown`. Its comment carries a `P2` badge and no `[Severity][FindingId]` first line, so its severity is `Minor (inferred)` and it gets the audit-local ID F9.

Review 6025633932 (da2ce7, COMMENTED, submitted 2026-11-23T16:49:15Z) has eight inline threads, each with a `[Severity][FindingId]` first line. Its body repeats no finding and adds none. The reviewer's IDs F1 to F8 do not collide and are kept. F1 asks for the same change as F9, which was submitted earlier, so F1 is a re-raise of F9.

CodeRabbit posted a rate-limit notice and no finding.

Round 2 covers review 6026001901 (Copilot, COMMENTED, submitted 2026-11-23T17:13:14Z). It reviewed the first head, after the round 1 fixes were pushed, and its body says the full review did not run before a timeout. It has three inline threads on `EPIC.md` with the same text. They have no `[Severity][FindingId]` first line; the review overview labels each `Medium severity`, recorded as `Minor (inferred)`. The first is F10, and F11 and F12 are re-raises of it.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F9 | review-finding:pr-3010-f9 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F1 | review-finding:pr-3010-f1 | Human | Minor | correctness | RE_RAISE_OF:F9 | FIXED | RESOLVED |
| F2 | review-finding:pr-3010-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3010-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3010-f4 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3010-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3010-f6 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3010-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3010-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3010-f10 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3010-f11 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F10 | NO_ACTION | SUPERSEDED |
| F12 | review-finding:pr-3010-f12 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F10 | NO_ACTION | SUPERSEDED |

## Finding Details

### F9 - Duration naming rules conflict for `cookie_lifetime`

- PR number: 3010
- Source review ID: 6025366087
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4668727836
- Concern: Decision D6 logs a configured duration under its configuration key, but prescribes `cookie_lifetime_in_secs` while the key is `cookie_lifetime`, and the sub-issue that renames keys covered two other keys only. A contributor could not satisfy both rules.
- Solution: `cookie_lifetime` is added to the sub-issue that gives configuration keys a unit (SI-4), with the note that its value changes from a `Duration` table to an integer of seconds and that the maintainer confirms it in that sub-issue's specification pull request. Decision D6 now states the exception: until SI-4 lands, the log uses the name the three keys will have.
- Current-tree verification: `grep -n 'cookie_lifetime' packages/configuration/src/v3_0_0/udp_tracker.rs` shows `pub cookie_lifetime: Duration` at line 26. `grep -n 'cookie_lifetime' docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md` shows the rename in the D6 follow-ups and the paragraph that starts "These three keys are the only exception to rule 3". `grep -c 'cookie_lifetime' docs/issues/drafts/2387-si-4-configuration-keys-with-units/ISSUE.md` is greater than zero.
- Resolution reference: docs(issues): [#2387] add cookie_lifetime to the configuration keys that get a unit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669101025

### F1 - D5 and D6 name two configured durations by fields that are not their configuration keys

- PR number: 3010
- Source review ID: 6025633932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4668946015
- Concern: The same defect as F9 for `cookie_lifetime`. In addition, decision D5 gave `request_drain_in_secs` where the configuration key is `udp_request_drain_in_secs`, and the drafts of the duration sub-issue and of the key-rename sub-issue repeated the count of two keys.
- Solution: As F9. Decision D5 now names `udp_request_drain_in_secs`. The two drafts say three keys.
- Current-tree verification: `grep -n 'request_drain_in_secs' docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md` shows only `udp_request_drain_in_secs`. `grep -rn 'two keys' docs/issues/drafts/2387-si-* docs/issues/open/2387-review-tracker-lifecycle-logs` prints nothing. `grep -n 'udp_request_drain_in_secs' packages/configuration/src/v3_0_0/shutdown.rs` shows the key.
- Resolution reference: docs(issues): [#2387] add cookie_lifetime to the configuration keys that get a unit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669101498

### F2 - The quoted shutdown message is not in the captures this table cites

- PR number: 3010
- Source review ID: 6025633932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4668946024
- Concern: The EPIC quoted `Received cancellation request, shutting down …`, a message of the replaced 2026-09-30 capture. The new captures say `Received cancellation request, draining … event listener.`.
- Solution: The EPIC quotes the message of the new captures. The other log texts quoted in the EPIC and in the decisions were checked against the three captures and the source in the same round; no other was wrong.
- Current-tree verification: `grep -c 'Received cancellation request, draining' docs/issues/open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md` prints 7. `grep -rn 'Received cancellation request, shutting down' docs/issues/open/2387-review-tracker-lifecycle-logs docs/issues/drafts/2387-si-*` prints nothing. The sweep counted each of these quoted texts in the `info` and `debug` captures and in the source: `timeout_seconds=`, `bind_addr=`, `local_addr=`, `{address=`, `bind_address=`, `Job will be asked to stop after`, `Waiting for job to finish`, `Job completed after cooperative cancellation`, `Starting UDP tracker instance`, `Started UDP tracker`, `Starting on:`, `Started on:`, `drain_timeout=15s`, `request_drain_deadline=5s`, `Torrust tracker successfully shutdown` and `latency_ms`. Each is in a capture; `bind_addr=` and `local_addr=` are only in the `debug` capture.
- Resolution reference: docs(issues): [#2387] quote the shutdown message that the new captures contain
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669101965

### F3 - The sub-issue process skips the approval the skill requires before the issue is created

- PR number: 3010
- Source review ID: 6025633932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4668946043
- Concern: Process step 5 of the EPIC went from "completes its draft" to "creates its GitHub issue" with no maintainer approval between the two, which the `create-issue` skill requires.
- Solution: Step 5 names the stop for the maintainer's review and approval, and says that the issue is created only after it.
- Current-tree verification: `grep -n 'stops for the maintainer to review and approve it' docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md` shows step 5.
- Resolution reference: docs(issues): [#2387] name the maintainer approval before a sub-issue creates its GitHub issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669102487

### F4 - `si-N` in the folder names is not the working order, but the open-spec convention says it is

- PR number: 3010
- Source review ID: 6025633932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4668946053
- Concern: `docs/issues/open/AGENTS.md` defines the `si-{N}` segment as the sub-issue's place in the implementation order. The drafts were numbered in the order they were proposed, so the folders would have stated a false order.
- Solution: The sub-issues are renumbered by working order, the first of the three options the reviewer gave. The old numbers 2, 1, 15, 12, 3, 4, 5, 6, 10, 7, 8, 9, 13, 14, 17, 18, 19, 20, 11, 21, 16, 22, 23 are now 1 to 23 in that sequence. The two blocked sub-issues are 21 and 22, and the close-out is 23. The EPIC table, its working order, phases and acceptance evidence, the decisions, the 23 draft folders and their cross-references, the statistics-disabled baseline, the specification of issue 2766 and the pull request description use the new numbers.
- Current-tree verification: `ls docs/issues/drafts | grep 2387-si` lists `2387-si-1-statistics-disabled-warning-per-event` to `2387-si-23-close-out`. In the EPIC table every "Depends on" cell names only lower numbers, except the two blocked rows. `grep -n 'SI-' docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md` shows SI-1 to SI-20, SI-20 for the automated check, SI-21 as blocked and SI-22 for `run_id`. The frontmatter validator and `linter all` ran in the pre-commit hook of the commit and passed.
- Resolution reference: docs(issues): [#2387] number the sub-issues of the lifecycle logs EPIC by working order
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669102992

### F5 - The SI-1 row lists fewer decisions than its draft

- PR number: 3010
- Source review ID: 6025633932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4668946062
- Concern: The EPIC row of the convention sub-issue gave D9, D11 and D12, while its draft implements D2 to D6, D9, D11 and D12.
- Solution: The row lists the same eight decisions as the draft. The convention sub-issue is SI-2 after the renumbering of F4.
- Current-tree verification: `grep -n 'Record the logging convention' docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md` shows `D2, D3, D4, D5, D6, D9, D11, D12` in the row, and `grep -n 'Implements' docs/issues/drafts/2387-si-2-logging-convention-adr-and-docs/ISSUE.md` shows the same list.
- Resolution reference: docs(issues): [#2387] list every decision of the convention sub-issue in the EPIC table
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669103528

### F6 - Two targets, not three, already use the underscore style

- PR number: 3010
- Source review ID: 6025633932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4668946068
- Concern: Decision D2 said three targets already use uppercase with underscores. Two do: `TRACKER_CORE` and `SWARM_COORDINATION_REGISTRY`.
- Solution: The decision says two and names them.
- Current-tree verification: `grep -rn 'pub const .*TARGET.*=' --include=*.rs src packages` lists nine target constants with eight distinct values (`HTTP TRACKER` is defined in two packages); the two with an underscore in the value are `TRACKER_CORE` and `SWARM_COORDINATION_REGISTRY`. `grep -n 'two targets already use it' docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md` shows the corrected sentence.
- Resolution reference: docs(issues): [#2387] say that two targets, not three, already use the underscore style
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669104067

### F7 - `project-liaison` is not defined anywhere in the tree

- PR number: 3010
- Source review ID: 6025633932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4668946077
- Concern: The EPIC named a `project-liaison` session as the one that starts the coordinator. The name is defined nowhere in the repository, against engineering policy 7 of `AGENTS.md`.
- Solution: The step says that the maintainer starts the coordinator session, and names no session.
- Current-tree verification: `grep -rln 'project-liaison' docs .claude AGENTS.md` lists only this audit record.
- Resolution reference: docs(issues): [#2387] say who starts the EPIC coordinator without naming an undefined session
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669104587

### F8 - T0 has no Commit Points row

- PR number: 3010
- Source review ID: 6025633932
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4668946085
- Concern: The task T0 added to the specification of issue 2766 changes documentation, and its Commit Points table started at T1.
- Solution: The table has a T0 row: one documentation commit, before T1.
- Current-tree verification: `grep -n 'T0' docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md` shows T0 in the Implementation Plan and in Commit Points.
- Resolution reference: docs(issues): [#2387] add the commit point of task T0 to the daemon JSON logging specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669105081

### F10 - The decision summary table has one `D3`, but the sub-issue table cites `D3a` to `D3d`

- PR number: 3010
- Source review ID: 6026001901
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669239849
- Concern: The EPIC's decision summary had a single D3 row while the sub-issue table cites D3a, D3b, D3c and D3d, so a reader could not tell where the parts are defined.
- Solution: The D3 row names its four parts and the D7 row its two parts (the same mismatch existed for D7a and D7b). The parts are the headings of `decisions.md`.
- Current-tree verification: `grep -n '^| D3 \|^| D7 ' docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md` shows `D3a`, `D3b`, `D3c`, `D3d`, `D7a` and `D7b` in the two rows. `grep -c '^### D3[a-d]\|^### D7[ab]' docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md` prints 6.
- Resolution reference: docs(issues): [#2387] name the parts of decisions D3 and D7 in the EPIC summary table
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669312716

### F11 - Duplicate of F10: the same comment on the decision summary table

- PR number: 3010
- Source review ID: 6026001901
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669239918
- Concern: The same text as F10, posted as a separate thread by the same review.
- Solution: No separate change. F10's fix covers it.
- Current-tree verification: As F10.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669313349
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669313349

### F12 - Duplicate of F10: the same comment on the decision summary table

- PR number: 3010
- Source review ID: 6026001901
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669239961
- Concern: The same text as F10, posted as a separate thread by the same review.
- Solution: No separate change. F10's fix covers it.
- Current-tree verification: As F10.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669314154
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669314154

## Processing Log

- 2026-11-23 16:32 UTC - Round 1: review 6025366087 submitted by Greptile at 16:32:02Z with one inline thread, recorded as F9.
- 2026-11-23 16:49 UTC - Round 1: review 6025633932 submitted by da2ce7 at 16:49:15Z (the poller reported it) with eight inline threads, F1 to F8. F1 is a re-raise of F9.
- 2026-11-23 16:58 UTC - Made the eight fix commits between 16:53 and 16:58 UTC (committer times), one for each of F4, F1 with F9, F2, F3, F5, F6, F7 and F8, in that order. The pre-commit hook ran for each.
- 2026-11-23 16:59 UTC - Created this audit record from the template, before the first reply.
- 2026-11-23 17:01 UTC - Pushed the eight commits to the pull request branch.
- 2026-11-23 17:02 UTC - Replied on the nine threads (replies created 17:01:59Z to 17:02:19Z) and updated the pull request description to the new sub-issue numbers and the third configuration key. Recorded the reply URLs here. No thread is resolved yet.
- 2026-11-23 17:03 UTC - The audit validator passed (9 rows, 0 failures). Resolved the nine threads; a new GraphQL fetch reports 0 unresolved threads.
- 2026-11-23 17:05 UTC - Committed this audit record with the round 1 entries, with `Greptile` added to `project-words.txt` for the spell check, and pushed it.
- 2026-11-23 17:13 UTC - Round 2: review 6026001901 submitted by Copilot at 17:13:14Z (the poller reported it) with three inline threads of the same text, F10 to F12.
- 2026-11-23 17:18 UTC - Pushed the F10 fix (committed at 17:15 UTC).
- 2026-11-23 17:18 UTC - Replied on the three threads (replies created 2026-11-23T17:18:23Z to 2026-11-23T17:18:28Z) and recorded the reply URLs here.
- 2026-11-23 17:19 UTC - The audit validator passed (12 rows, 0 failures). Resolved the three threads; a new GraphQL fetch reports 0 unresolved threads.

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
