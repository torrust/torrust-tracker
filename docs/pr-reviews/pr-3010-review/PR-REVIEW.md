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

Round 3 covers review 6026059751 (da2ce7, submitted 2026-11-23T17:16:25Z on the head that carried the round 1 fixes and the first audit commit). Its body gives the verdict APPROVE; GitHub shows the review as DISMISSED after later pushes. It has two inline threads with a `[Severity][FindingId]` first line. The reviewer's IDs F10 and F11 collide with the Copilot findings recorded in round 2, which was submitted three minutes earlier, so they are recorded as F13 and F14 with the reviewer's IDs in the detail entries.

Round 4 covers review 6026228066 (CodeRabbit, COMMENTED, submitted 2026-11-23T17:25:02Z) with eight inline threads, F15 to F22. CodeRabbit is classified `Unknown`. Each comment carries a `Minor` badge in its first line and no `[Severity][FindingId]` bracket, so each severity is `Minor (inferred)`.

Round 5 covers review 6026371049 (da2ce7, COMMENTED, submitted 2026-11-23T17:33:29Z) with one inline thread on this record. Its ID F15 collides with the first CodeRabbit finding, which was submitted earlier, so it is recorded as F23.

Round 6 covers three items. CodeRabbit's review 6026538673 (submitted 2026-11-23T17:43:28Z) is one follow-up comment on the F20 thread that says the F20 fix left a conflict; it is F24, a re-raise of F20, with severity inferred from F20. CodeRabbit's review 6026541291 (17:43:38Z) only confirms F19 and has no finding. Greptile's review 6026552595 (17:44:20Z) has one inline thread with a `P2` badge, F25. Review 6026737720 (da2ce7, APPROVED, submitted 17:55:54Z) has one inline thread, F26, which asks for the same change as F24.

Round 7 covers review 6026909751 (CodeRabbit, submitted 2026-11-23T18:07:08Z) with one inline thread carrying a `Minor` badge, F27, and review 6027161752 (Copilot, COMMENTED, submitted 2026-11-23T18:24:08Z) with two inline threads, F28 and F29. Copilot's comments carry no severity; `Minor (inferred)` is this record's reading of them. Five rounds of fixes had been pushed before this round, which is the limit after which the author asks the maintainer; the maintainer said to fix all three.

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
| F13 | review-finding:pr-3010-f13 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3010-f14 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3010-f15 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3010-f16 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-3010-f17 | Unknown | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-3010-f18 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-3010-f19 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-3010-f20 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-3010-f21 | Unknown | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-3010-f22 | Unknown | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-3010-f23 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-3010-f24 | Unknown | Minor (inferred) | correctness | RE_RAISE_OF:F20 | FIXED | RESOLVED |
| F25 | review-finding:pr-3010-f25 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-3010-f26 | Human | Minor | correctness | RE_RAISE_OF:F24 | FIXED | RESOLVED |
| F27 | review-finding:pr-3010-f27 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-3010-f28 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F29 | review-finding:pr-3010-f29 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

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

### F13 - The renumbering commit dropped the upstream issue links added in round 1

- PR number: 3010
- Source review ID: 6026059751
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669281516
- Concern: The commit that renumbered the sub-issues put two draft lines back to their text before the commit that linked the two upstream issues: the `torrust-net-primitives` issue link in the `service_binding` draft (now SI-7) and the `torrust-metrics` issue link in the metrics draft (now SI-21). Its body said only that it renumbers.
- Solution: Both links are restored. The cause: the drafts are generated by a script kept outside the repository, the two links had been added to the draft files only, and regenerating the drafts for the renumbering lost them. The script now has both links.
- Current-tree verification: `grep -n 'torrust-net-primitives/issues/1' docs/issues/drafts/2387-si-7-service-binding-without-trailing-slash/ISSUE.md` and `grep -n 'torrust-metrics/issues/4' docs/issues/drafts/2387-si-21-metrics-registration-summary-line/ISSUE.md` each print one line. The 23 drafts were regenerated into a scratch directory and compared with the committed drafts with `diff`: only these two drafts differ, by their `last-updated-utc` and the progress-log line this fix added. `git diff --stat` of the commit that linked the upstream issues shows that it changed two drafts only, so no other hand edit of a draft existed to lose.
- Resolution reference: docs(issues): [#2387] restore the upstream issue links that the renumbering commit dropped
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669418879

### F14 - The F6 row's verification command prints nine target constants, not eight

- PR number: 3010
- Source review ID: 6026059751
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669281529
- Concern: The F6 entry of this record said its command lists eight target constants. It prints nine lines, because `HTTP TRACKER` is defined in two packages; there are eight distinct values.
- Solution: The F6 entry says nine target constants with eight distinct values and names the duplicate.
- Current-tree verification: `grep -rn 'pub const .*TARGET.*=' --include=*.rs src packages | wc -l` prints 9.
- Resolution reference: docs(pr-reviews): [#2387] say that the F6 command of the PR 3010 audit lists nine target constants
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669419468

### F15 - The job names sub-issue does not list the start lines sub-issue as a dependency

- PR number: 3010
- Source review ID: 6026228066
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669400173
- Concern: SI-13's acceptance check selects a service's lines by `service_role` and `instance_index`, which the start lines carry as event fields only after SI-10. Its dependency list gave SI-5 only, so a reordering could start it before SI-10.
- Solution: The SI-13 draft and its EPIC row list SI-5 and SI-10.
- Current-tree verification: `grep -n 'Depends on' docs/issues/drafts/2387-si-13-job-names-from-role-and-index/ISSUE.md` shows `SI-5, SI-10`, and `grep -n '^| 13 ' docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md` shows the same in the row.
- Resolution reference: docs(issues): [#2387] make the job names sub-issue depend on the start lines sub-issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669626570

### F16 - The close-out removes the catalogue marks of work that may be detached

- PR number: 3010
- Source review ID: 6026228066
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669400215
- Concern: The close-out draft removed every "not yet implemented" mark from the catalogue, which would present the lines of a detached SI-21 or SI-22 as implemented.
- Solution: The close-out removes the mark only from entries whose line is implemented; an entry of an open or detached sub-issue keeps its mark and names its issue.
- Current-tree verification: `grep -n 'not yet implemented' docs/issues/drafts/2387-si-23-close-out/ISSUE.md` shows the new scope item.
- Resolution reference: docs(issues): [#2387] keep the catalogue marks of sub-issues that are open or detached at close-out
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669627197

### F17 - The note at the top of the drafts creates the issue without the maintainer's approval

- PR number: 3010
- Source review ID: 6026228066
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669400229
- Concern: The note at the top of each draft went from completing the draft to creating the GitHub issue, while the checkpoint list of the same draft requires the approval first. The comment says eight drafts; the note was the same in all 23.
- Solution: The note in all 23 drafts names the stop for the maintainer's review and approval before the issue is created.
- Current-tree verification: `grep -L 'stops for the maintainer to review and approve it' docs/issues/drafts/2387-si-*/ISSUE.md` prints nothing, so all 23 drafts have the sentence.
- Resolution reference: docs(issues): [#2387] name the maintainer approval in the note of every sub-issue draft
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669627852

### F18 - The pre-bind `bind_addr` field is not in the rename

- PR number: 3010
- Source review ID: 6026228066
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669400238
- Concern: The UDP bind code logs `bind_addr` before the bind. The SI-8 scope replaced that name only after the bind, so a `debug` capture would still contain it and fail the first acceptance criterion.
- Solution: The SI-8 scope renames the pre-bind `bind_addr` to `bind_address`, names the one line that has it today, and the second acceptance criterion requires `bind_address` on every pre-bind line.
- Current-tree verification: `grep -n 'bind_addr' packages/udp-server/src/server/bound_socket.rs` shows the pre-bind `tracing::debug!` with `bind_addr`. `grep -n 'bound_socket.rs' docs/issues/drafts/2387-si-8-single-listener-identity-field/ISSUE.md` shows the new scope item.
- Resolution reference: docs(issues): [#2387] rename the pre-bind bind_addr field in the listener identity sub-issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669628376

### F19 - The duration sub-issue's acceptance criterion ignores the key-name exception

- PR number: 3010
- Source review ID: 6026228066
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669400251
- Concern: AC2 of the SI-9 draft required every configured duration to be logged under its configuration key, which the three keys of the D6 exception cannot meet when SI-4 is not merged.
- Solution: AC2 states the exception for the three keys. In the working order SI-4 runs before SI-9; the criterion now holds in either order.
- Current-tree verification: `grep -n 'AC2:' docs/issues/drafts/2387-si-9-duration-fields-in-lifecycle-lines/ISSUE.md` shows the exception.
- Resolution reference: docs(issues): [#2387] apply the key-name exception to the duration sub-issue's acceptance criterion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669628879

### F20 - The specification does not say how a sub-second `cookie_lifetime` migrates

- PR number: 3010
- Source review ID: 6026228066
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669400261
- Concern: `cookie_lifetime` can hold nanoseconds today and the UDP server uses it as a fraction of seconds, so an integer of seconds cannot hold every value. D6 and the SI-4 draft fixed the integer form without a rule for such values, and the SI-4 draft's out-of-scope list excluded changing values while its scope changed one.
- Solution: D6 no longer fixes the integer form: the form and the handling of a value with a fraction are decided in SI-4's specification, and the tracker must not round a configured value silently. The SI-4 draft lists the three alternatives to compare and its out-of-scope item no longer contradicts the scope. The choice is the maintainer's, so it is not made in this pull request.
- Current-tree verification: `grep -n 'cookie_lifetime: f64' packages/udp-server/src/handlers/mod.rs` shows the fractional use. `grep -n 'sub-second' docs/issues/drafts/2387-si-4-configuration-keys-with-units/ISSUE.md` shows the decision item, and `grep -n 'round a configured value silently' docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md` shows the D6 sentence.
- Resolution reference: docs(issues): [#2387] leave the form of cookie_lifetime and its sub-second values to the key sub-issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669629335

### F21 - The EPIC says the JSON logging issue starts after the EPIC, which reads as after its close

- PR number: 3010
- Source review ID: 6026228066
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669400278
- Concern: "After this EPIC" can mean after the EPIC closes, but SI-22 stays in the EPIC and waits for issue 2766, so that reading is circular. The specification of issue 2766 names SI-1 to SI-20.
- Solution: The EPIC's out-of-scope item and its reference line, and decision D1, name SI-1 to SI-20 and say that the issue does not wait for the EPIC to close.
- Current-tree verification: `grep -n 'after this EPIC' docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md` shows two lines only: the new out-of-scope sentence, which says "not after this EPIC closes", and a progress-log entry of 16:20 UTC.
- Resolution reference: docs(issues): [#2387] say that the JSON logging issue waits for SI-1 to SI-20, not for the EPIC to close
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669629822

### F22 - The output contract EPIC says issue 7 waits for the lifecycle logs EPIC, not for its sub-issues

- PR number: 3010
- Source review ID: 6026228066
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669400294
- Concern: The same circular reading as F21, in the Delivery Strategy of the output contract EPIC.
- Solution: The Delivery Strategy sentence and row 7 of the sub-issue table name SI-1 to SI-20.
- Current-tree verification: `grep -c 'SI-1 to SI-20 of EPIC #2387' docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` prints 2.
- Resolution reference: docs(issues): [#2387] name the sub-issues that issue 7 of the output contract EPIC waits for
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669630269

### F23 - Review 6026059751 predates this record's last commit but has no rows

- PR number: 3010
- Source review ID: 6026371049
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669513830
- Concern: At the head the reviewer read, this record had no rows for review 6026059751, while the replies on its two threads and the progress lines of two drafts cited F13 and F14, and neither thread was resolved.
- Solution: The rows F13 and F14 and the paragraph on round 3 were committed at 17:27 UTC and pushed at 17:40 UTC, after the head the reviewer read. The two threads are resolved in this round. The replies were posted before their rows existed in a pushed commit, which the Processing Log records.
- Current-tree verification: `grep -n '^| F13 \|^| F14 ' docs/pr-reviews/pr-3010-review/PR-REVIEW.md` shows both rows, and `grep -c '6026059751' docs/pr-reviews/pr-3010-review/PR-REVIEW.md` is greater than zero.
- Resolution reference: docs(pr-reviews): [#2387] record review round 3 of PR 3010
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669630702

### F24 - The key sub-issue still fixes an integer of seconds for `cookie_lifetime`

- PR number: 3010
- Source review ID: 6026228066
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669400261
- Concern: Raised by the follow-up comment https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669642665 (review 6026538673) on the F20 thread, so the source fields name that thread and its review. After the F20 fix, one scope line of the SI-4 draft still said that `cookie_lifetime` becomes an integer of seconds while the next line left the form to the maintainer, and one alternative uses `cookie_lifetime_in_ms` while D6 and the draft's goal name `cookie_lifetime_in_secs`.
- Solution: The scope line says the value becomes one number whose form is decided in the next item. That item says which alternatives keep the name `cookie_lifetime_in_secs` and that the `_in_ms` alternative means correcting D5, D6 and the SI-9 draft in the same specification pull request.
- Current-tree verification: `grep -n 'integer of seconds' docs/issues/drafts/2387-si-4-configuration-keys-with-units/ISSUE.md` shows the phrase only inside the item that lists the alternatives. `grep -n 'means correcting D5, D6' docs/issues/drafts/2387-si-4-configuration-keys-with-units/ISSUE.md` shows the new sentence.
- Resolution reference: docs(issues): [#2387] defer the form of cookie_lifetime in the scope line of the key sub-issue too
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669879444

### F25 - Pre-bind field rules disagree between D3a and the listener identity sub-issue

- PR number: 3010
- Source review ID: 6026552595
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669653954
- Concern: The SI-8 draft, after the F18 fix, requires `bind_address` on the `UdpSocket::bind (binding)` debug line, while decision D3a allowed that field only on the `Starting` line.
- Solution: D3a allows `bind_address` on every line logged before the bind and names the `Starting` line and a `DEBUG` line of the bind call as the two kinds. The SI-8 draft is unchanged.
- Current-tree verification: `grep -n 'appears only on lines logged before the bind' docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md` shows the sentence, and `grep -rn 'only on the .Starting. line' docs/issues/open/2387-review-tracker-lifecycle-logs docs/issues/drafts/2387-si-*` prints nothing.
- Resolution reference: docs(issues): [#2387] let decision D3a allow bind_address on every line logged before the bind
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669879919

### F26 - The key sub-issue's scope line still fixes an integer of seconds, and the next offers a name that D6 rules out

- PR number: 3010
- Source review ID: 6026737720
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669798489
- Concern: The same two points as F24, raised by the reviewer on the SI-4 draft, with the note that the automated follow-up on the F20 thread had no answer.
- Solution: As F24. The three alternatives are kept, and the `_in_ms` one says what else it changes, which is the second option the reviewer gave.
- Current-tree verification: As F24.
- Resolution reference: docs(issues): [#2387] defer the form of cookie_lifetime in the scope line of the key sub-issue too
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669880370

### F27 - The pre-bind criterion covers lines that are not about a listener

- PR number: 3010
- Source review ID: 6026909751
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4669930343
- Concern: After the F18 fix, AC2 of the SI-8 draft required `bind_address` on every pre-bind line. Lines such as the build and run identity line are logged before any bind and have no listener address.
- Solution: AC2 is limited to pre-bind lines about a listener and says that other lines have no such field.
- Current-tree verification: `grep -n 'AC2:' docs/issues/drafts/2387-si-8-single-listener-identity-field/ISSUE.md` shows `every pre-bind line about a listener`.
- Resolution reference: docs(issues): [#2387] limit the pre-bind criterion of the listener identity sub-issue to listener lines
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4670268175

### F28 - Waiting for `Tracker started` alone does not make it safe to send `SIGTERM`

- PR number: 3010
- Source review ID: 6027161752
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4670128855
- Concern: The SI-20 draft's test waited only for `Tracker started` before sending `SIGTERM`, while the SI-17 draft left open whether that line comes before or after the signal handlers are installed. The native tracker fixture waits for health readiness and for the signal-handler line because the handlers are installed after the jobs start.
- Solution: The SI-17 draft logs `Tracker started` after the shutdown signal handlers are installed and its AC1 requires it. The SI-20 draft waits until the handlers are installed, through that line, and keeps the fixture's existing wait if SI-17 did not land with that order.
- Current-tree verification: `grep -n 'signal_handlers_are_installed' tests/common/native_tracker/mod.rs` shows the fixture's wait. `grep -n 'after the shutdown signal handlers are installed' docs/issues/drafts/2387-si-17-tracker-started-line/ISSUE.md` shows the scope item and AC1, and `grep -n 'Safe means' docs/issues/drafts/2387-si-20-automated-lifecycle-log-check/ISSUE.md` shows the new sentence.
- Resolution reference: docs(issues): [#2387] log Tracker started after the signal handlers so the automated check can stop the tracker
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4670268750

### F29 - AC15 forbids what SI-11 and SI-20 plan to do

- PR number: 3010
- Source review ID: 6027161752
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4670128982
- Concern: AC15 of the EPIC said that no program reads a fact from the message text of a log line. SI-11 selects lines by the message `Started <service>` and SI-20 waits for `Tracker started`, so the criterion could not be met.
- Solution: AC15 forbids reading a value from message text and allows matching a message that the catalogue documents, to select a line or to wait for a state. Rule 1 of the decisions and the close-out search of SI-23 say the same.
- Current-tree verification: `grep -n 'AC15:' docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md` shows the new criterion; `grep -n 'Prefer structured fields' docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md` shows rule 1 with the sentence on catalogued messages; `grep -n 'AC15' docs/issues/drafts/2387-si-23-close-out/ISSUE.md` shows the search item.
- Resolution reference: docs(issues): [#2387] let programs match catalogued messages while values come from fields
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3010#discussion_r4670269202

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
- 2026-11-23 17:19 UTC - Correction to the entry above: the resolving step resolved five threads, not three. It resolved every unresolved thread without listing them first, and two of them were the threads of review 6026059751 (da2ce7, submitted 17:16:25Z), which had not been read or replied to. Committed the round 2 record at 17:19 UTC before noticing.
- 2026-11-23 17:20 UTC - Reopened those two threads (17:20:21 UTC) and started round 3: F13 and F14. Committed the F13 fix at 17:20 UTC.
- 2026-11-23 17:23 UTC - Committed the F14 correction of this record at 17:23 UTC and pushed both commits.
- 2026-11-23 17:26 UTC - Replied on the two threads (replies created 17:26:25Z and 17:26:27Z). The F13 reply says that the regenerated drafts were compared with the committed ones; that comparison was run at 17:27 UTC, after the reply, and it confirmed the statement.
- 2026-11-23 17:27 UTC - The audit validator passed (14 rows, 0 failures). Listed the unresolved threads first (exactly the two of round 3), resolved them, and a new GraphQL fetch reports 10 unresolved threads.
- 2026-11-23 17:28 UTC - Correction to the entry above: it is wrong. The listing showed ten unresolved threads, not two: eight threads of review 6026228066 (CodeRabbit, submitted 17:25:02Z) had arrived. The guard on the list therefore resolved nothing, and the two round 3 threads stayed open. The entry was written before the result was read, and the round 3 record was committed with it at 17:27 UTC.
- 2026-11-23 17:34 UTC - Round 4: made the eight fix commits for F15 to F22 between 17:29 and 17:34 UTC (committer times), one for each finding.
- 2026-11-23 17:40 UTC - Pushed the round 3 record and the eight round 4 commits (the push ended at 17:40:41 UTC). Review 6026371049 (da2ce7, submitted 17:33:29Z, one thread, F23) had read a head without the round 3 record.
- 2026-11-23 17:42 UTC - Replied on the nine threads of rounds 4 and 5 (replies created 17:42:12Z to 17:42:32Z) and recorded the reply URLs here.
- 2026-11-23 17:43 UTC - The audit validator passed (23 rows, 0 failures). Listed the unresolved threads: six, all with a reply and a fix (the two of round 3, three of round 4 and the one of round 5). The other five threads of round 4 were already resolved; this session did not resolve them. Resolved the six by their IDs, and a new GraphQL fetch reports 0 unresolved threads.
- 2026-11-23 17:44 UTC - Committed the record of rounds 4 and 5 at 17:43 UTC and pushed it.
- 2026-11-23 17:55 UTC - Round 6: review 6026737720 submitted by da2ce7 at 17:55:54Z, state APPROVED, with one thread (F26). Reading the pull request showed two automated items submitted before it and not yet seen: CodeRabbit's follow-up on the F20 thread (17:43:28Z, F24) and Greptile's thread (17:44:20Z, F25). CodeRabbit's comment of 17:43:38Z on the F19 thread confirms that fix.
- 2026-11-23 18:02 UTC - Committed the two fixes at 17:58 UTC and pushed them (the push ended at 18:02:06 UTC).
- 2026-11-23 18:02 UTC - Replied on the three threads (replies created 18:02:36Z to 18:02:40Z) and recorded the reply URLs here.
- 2026-11-23 18:03 UTC - The audit validator passed (26 rows, 0 failures). Listed the unresolved threads: two, those of F25 and F26; the F20 thread, which carries F24, was still resolved. Resolved the two by their IDs, and a new GraphQL fetch reports 0 unresolved threads.
- 2026-11-23 18:04 UTC - Committed the record of round 6 at 18:04 UTC and pushed it. The maintainer's `ACK` comment of 18:05:43Z names the commit before this record commit, and da2ce7's `ACK` of 17:55:58Z names the head of round 5, so both were stale from that push.
- 2026-11-23 18:24 UTC - Round 7: review 6026909751 (CodeRabbit, 18:07:08Z, F27) and review 6027161752 (Copilot, 18:24:08Z, F28 and F29). Did not fix or reply: five rounds of fixes were pushed. Reported the three findings to the maintainer and asked.
- 2026-11-23 18:35 UTC - The maintainer said to fix all three. Committed the three fixes between 18:31 and 18:32 UTC and pushed them (the push ended at 18:35:56 UTC).
- 2026-11-23 18:36 UTC - Replied on the three threads (replies created 18:36:31Z to 18:36:37Z) and recorded the reply URLs here.
- 2026-11-23 18:37 UTC - The audit validator passed (29 rows, 0 failures). Listed the unresolved threads: two, those of F28 and F29; the F27 thread was already resolved, not by this session. Resolved the two by their IDs, and a new GraphQL fetch reports 0 unresolved threads.

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
