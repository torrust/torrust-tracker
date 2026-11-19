---
pr-number: 2961
pr-url: https://github.com/torrust/torrust-tracker/pull/2961
last-updated-utc: "2026-11-19 09:18"
---

<!-- skill-link: process-pr-review -->

# PR #2961 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2961.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the findings were processed before the merge).

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

Copilot's review 5972177251 has one inline finding, `R1`. The review 5972178096 of `da2ce7` has four
inline findings, `F1` to `F4`; its body lists the same four and adds no other request. The reviewers'
own finding IDs are kept.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| R1 | review-finding:pr-2961-r1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F1 | review-finding:pr-2961-f1 | Human | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2961-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2961-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2961-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### R1 - D4 cites the wrong file for the deadline-abort coverage

- PR number: 2961
- Source review ID: 5972177251
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626545911
- Concern: Decision D4 of the R8 plan cited `tests/lifecycle/shutdown_policy.rs` as the coverage of the hung-component deadline abort, but that file injects no hung component; the checks are in `packages/hung-component-tracker/tests/hung_component_abort.rs`.
- Solution: D4 now cites `packages/hung-component-tracker/tests/hung_component_abort.rs`.
- Current-tree verification: `grep -c 'tests/lifecycle/shutdown_policy.rs' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 0 and `grep -c 'hung_component_abort.rs' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1 on the head that carries the fix.
- Resolution reference: `docs(issues): [#1419] cite the test that covers the deadline abort in decision D4`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626607830

### F1 - D4's recommendation rests on a wrong file and a false premise

- PR number: 2961
- Source review ID: 5972178096
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626546550
- Concern: D4 recommended no negative test because a hung component "needs a production-only test hook" and the deadline abort was said to be covered by `tests/lifecycle/shutdown_policy.rs`. Both are wrong: the abort test is `packages/hung-component-tracker/tests/hung_component_abort.rs`, and `packages/hung-component-tracker/src/main.rs` injects the hung component with the public `jobs.reserve` and `jobs.spawn`. The existing test covers the supervisor, not the fixture's new assertion, so the last sentence of E1 was unproven. The PR description repeated the path.
- Solution: The path is corrected by the commit of R1. D4 now recommends option (b), drops the hook claim and states the cost of (b). The planned change gained a step for the new target, the evidence table gained E5, the unproven sentence of E1 is removed, AC8a points at E5, and the spec records a local probe of option (b). The PR description was corrected with `gh pr edit`. The author read `packages/hung-component-tracker/src/main.rs` and `tests/lifecycle/shutdown_policy.rs` to confirm the reviewer's two points before changing the recommendation.
- Current-tree verification: `grep -c 'production-only test hook' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 0, `grep -c '^| E5' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1, and `grep -n '^| D4' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` shows the row recommending "(b)" with its cost.
- Resolution reference: `docs(issues): [#1419] recommend the negative fixture test in decision D4 and state its cost`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626608186

### F2 - The parent EPIC still lists the issue as blocked

- PR number: 2961
- Source review ID: 5972178096
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626546563
- Concern: `docs/issues/open/2392-test-isolation/EPIC.md` keeps order 5 at `BLOCKED` while this spec says `in-progress`. The reviewer asked to update the row or to say in the description which change will.
- Solution: The second option. The EPIC #2392 coordinator session owns the EPIC spec and this session was told not to edit it. The author sent the coordinator the requested row change on 2026-11-19, and the PR description now has a section "Parent EPIC row" that says the coordinator updates the row in its own change. No tree change.
- Current-tree verification: `gh pr view 2961 --repo torrust/torrust-tracker --json body --jq .body | grep -c '^## Parent EPIC row'` printed 1 (run with the bot token).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626608577
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626608577

### F3 - The progress-log entry names the agent but not its model

- PR number: 2961
- Source review ID: 5972178096
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626546577
- Concern: The `create-issue` skill asks an AI agent to name its model after the agent in each log entry; the new entry said only "Claude".
- Solution: The entry now says "Claude Code (Claude Fable 5.2)", in the form of the parent EPIC's entries.
- Current-tree verification: `grep -c '2026-11-19 08:23 UTC - Claude Code (Claude Fable 5.2) -' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1.
- Resolution reference: `docs(issues): [#1419] name the model in the progress-log entry of the R8 plan`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626608899

### F4 - The 2026-11-19 timing lacks its command, exit status and toolchain

- PR number: 2961
- Source review ID: 5972178096
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626546588
- Concern: The measurement that retires the 60 to 81 second figures was recorded without its command, exit status or toolchain, unlike the M rows and against the toolchain-qualified evidence rule of the `create-issue` skill.
- Solution: The review findings now name the E2 command, exit status 0, and the toolchain and host; the two probe paragraphs name their commands and exit status.
- Current-tree verification: `grep -c 'rustc 1.100.0 (0ef1bc386 2026-11-09)' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1 and `grep -c 'exited with status 0' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 3.
- Resolution reference: `docs(issues): [#1419] record the command, exit status and toolchain of the R8 measurements`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626609277

## Processing Log

- 2026-11-19 09:10 UTC - Copilot review 5972177251 (submitted 09:10:48) and the review 5972178096 of `da2ce7` (submitted 09:10:52, changes requested) arrived on the first head.
- 2026-11-19 09:15 UTC - Committed the four fixes locally (commit times 09:12:33, 09:13:54, 09:14:47 and 09:15:30), not pushed yet.
- 2026-11-19 09:16 UTC - Edited the PR description (D4 wording, the "Parent EPIC row" section, the second probe and the toolchain) and sent the EPIC #2392 coordinator session the row change that F2 asks for.
- 2026-11-19 09:17 UTC - Replied on the five threads (09:16:51 to 09:17:00). Each reply says its commit is local and goes up in one push with this record.
- 2026-11-19 09:17 UTC - Committed a sweep that points AC8a at E5 (commit time 09:17:12), part of the F1 change.
- 2026-11-19 09:18 UTC - Created this audit record. It was created after the replies, not before them as the skill asks, so the reply URLs could be recorded in the same commit.

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
