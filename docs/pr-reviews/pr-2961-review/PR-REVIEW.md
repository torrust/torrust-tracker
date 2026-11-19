---
pr-number: 2961
pr-url: https://github.com/torrust/torrust-tracker/pull/2961
last-updated-utc: "2026-11-19 18:43"
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
own finding IDs are kept. The second review of `da2ce7`, 5972450674 (approved), has four inline
findings, `F5` to `F8`; its body adds no other request.
Copilot's second review, 5972638002, has one inline finding, `R2`. The third review of `da2ce7`,
5972648538 (approved), has one inline finding, `F9`. The fourth review of `da2ce7`, 5981075592
(approved), has two inline findings, `F10` and `F11`; its body adds no other request.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| R1 | review-finding:pr-2961-r1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F1 | review-finding:pr-2961-f1 | Human | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2961-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2961-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2961-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2961-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2961-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2961-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2961-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2961-r2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2961-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2961-f10 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2961-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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

### F5 - The last sentence of D3 contradicts the D4 (b) recommendation

- PR number: 2961
- Source review ID: 5972450674
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626757329
- Concern: D3 said a suite that wants a non-graceful shutdown is an executable-boundary test, not a user of this fixture, while D4 now recommends a fixture suite whose shutdown must not be graceful, placed in `tests/lifecycle/`.
- Solution: D3's sentence now covers suites that test a non-graceful shutdown of the tracker, names the two executable-boundary files, and names the negative suite of D4 option (b) as the one exception.
- Current-tree verification: `grep -c 'The one exception is the negative suite of D4 option (b)' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1.
- Resolution reference: `docs(issues): [#1419] carve the D4 negative suite out of the D3 rule about non-graceful suites`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626810042

### F6 - The description still says the pull request touches one file

- PR number: 2961
- Source review ID: 5972450674
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626757337
- Concern: The "What changed" section of the PR description opened with "Only" the issue spec, but the head also adds this audit record.
- Solution: The section now opens with "Two files" and names this audit record. No tree change.
- Current-tree verification: `gh pr view 2961 --repo torrust/torrust-tracker --json body --jq .body | grep -c '^Two files\.'` printed 1 (run with the bot token).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626810504
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626810504

### F7 - The Processing Log stops at the record's creation

- PR number: 2961
- Source review ID: 5972450674
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626757344
- Concern: The rows said `RESOLVED` but the log recorded no validator run, push, thread resolution or GraphQL refresh, and the 09:19:49Z follow-up reply on the F2 thread came after the audit commit.
- Solution: The log now has those round 1 events and the round 2 events. The commit that carries this entry is the last of round 2: its own push, and the resolution of the F7 thread after that push, are the events the log cannot carry.
- Current-tree verification: `grep -c '^- 2026-11-19 09:18 UTC - Ran' docs/pr-reviews/pr-2961-review/PR-REVIEW.md` printed 1 (the validator run) and `grep -c '^- 2026-11-19 09:19 UTC - Replied again on the F2 thread' docs/pr-reviews/pr-2961-review/PR-REVIEW.md` printed 1 (the follow-up reply). Both patterns are anchored to the start of a Processing Log entry, so they cannot match this line.
- Resolution reference: `docs(pr-reviews): [#1419] record round 2 in the review audit of PR 2961`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626811052

### F8 - With D4 (b) the planned change also touches the root Cargo.toml

- PR number: 2961
- Source review ID: 5972450674
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626757350
- Concern: Nested test targets are registered with `[[test]]` entries in the root `Cargo.toml`, so "stays in the test tree" in the spec and "test tree only" in the description were not exact.
- Solution: The planned change says it stays in the test tree except for one `[[test]]` entry in the root `Cargo.toml` if D4 is (b), and step 3 names the entry. The PR description says the same.
- Current-tree verification: `grep -c '\[\[test\]\]' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 2.
- Resolution reference: `docs(issues): [#1419] name the root Cargo.toml entry that the D4 negative target needs`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626811628

### R2 - The plan lacks the required test-design checkpoint

- PR number: 2961
- Source review ID: 5972638002
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626904228
- Concern: Step 3 of the planned change adds a test suite, but the R8 plan had no progressive test-development loop, which the `create-issue` skill requires when a plan adds tests: a recorded prose-first Arrange-Act-Assert review after each passing increment and a stop for maintainer review after the last one.
- Solution: The R8 plan has a new subsection "Test development" with the increment, the prose-first review and what it confirms, where the result is recorded, and the stop for maintainer review before the final verification, the commit and the pull request.
- Current-tree verification: `grep -c '^#### Test development' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1.
- Resolution reference: `docs(issues): [#1419] add the test-development loop to the R8 plan`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626949335

### F9 - The verification of F7 counts its own line

- PR number: 2961
- Source review ID: 5972648538
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626912242
- Concern: The two greps in F7's `Current-tree verification` printed 2 only because that line contained each pattern, so the output read as two log references each.
- Solution: F7's verification now uses two patterns anchored to the start of a Processing Log entry; each prints 1 and neither can match the verification line.
- Current-tree verification: `sed -n '/^### F7/,/^### F8/p' docs/pr-reviews/pr-2961-review/PR-REVIEW.md | grep -c 'cannot match this line'` printed 1; the section it reads does not contain this entry.
- Resolution reference: `docs(pr-reviews): [#1419] verify finding F7 of PR 2961 with checks that cannot match their own line`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626949769

### F10 - The planned negative suite does not prove which deadline the fixture uses

- PR number: 2961
- Source review ID: 5981075592
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4633365681
- Concern: The decision record said that the negative suite of D4 proves that the configured deadline of D2 is the one used, but the planned `should_panic` test passes with any finite deadline; the 3-second return was an observation of the probe, not an assertion of the plan.
- Solution: Step 3 of the planned change and E5 now require that the panic arrives well before 10 seconds, so the suite fails if the fixture ignores the configured deadline. The D4 line of the decision record ties the deadline claim to that bound.
- Current-tree verification: `grep -c 'well before 10 seconds' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 2 (step 3 and E5).
- Resolution reference: `docs(issues): [#1419] bound the elapsed time of the negative suite so it proves the configured deadline`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4633409479

### F11 - `begin_shutdown` cancels one token per component, not one token

- PR number: 2961
- Source review ID: 5981075592
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4633365687
- Concern: The D5 line of the decision record said that `begin_shutdown` "sets a flag and cancels a token", but `JobManager::cancel()` cancels the token of every component that waits for no other, and each component has its own token.
- Solution: The D5 line now says that it sets a flag and cancels the tokens of the components that wait for no other.
- Current-tree verification: `grep -c 'cancels the tokens of the' docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1.
- Resolution reference: `docs(issues): [#1419] say that begin_shutdown cancels one token per component`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4633409926

## Processing Log

- 2026-11-19 09:10 UTC - Copilot review 5972177251 (submitted 09:10:48) and the review 5972178096 of `da2ce7` (submitted 09:10:52, changes requested) arrived on the first head.
- 2026-11-19 09:15 UTC - Committed the four fixes locally (commit times 09:12:33, 09:13:54, 09:14:47 and 09:15:30), not pushed yet.
- 2026-11-19 09:16 UTC - Edited the PR description (D4 wording, the "Parent EPIC row" section, the second probe and the toolchain) and sent the EPIC #2392 coordinator session the row change that F2 asks for.
- 2026-11-19 09:17 UTC - Replied on the five threads (09:16:51 to 09:17:00). Each reply says its commit is local and goes up in one push with this record.
- 2026-11-19 09:17 UTC - Committed a sweep that points AC8a at E5 (commit time 09:17:12), part of the F1 change.
- 2026-11-19 09:18 UTC - Created this audit record. It was created after the replies, not before them as the skill asks, so the reply URLs could be recorded in the same commit.
- 2026-11-19 09:18 UTC - Ran `validate-audit-record.py --pr-number 2961 --base torrust/develop` with the bot token: 5 rows, 6 log entries, 0 failures. Committed the record (commit time 09:18:42).
- 2026-11-19 09:19 UTC - Replied again on the F2 thread (https://github.com/torrust/torrust-tracker/pull/2961#discussion_r4626637779, 09:19:49) to cite the coordinator's open pull request 2962, and edited the PR description to cite it.
- 2026-11-19 09:21 UTC - Pushed round 1 (the "Docs Lint" run of that head was created at 09:21:42), then resolved the five threads; a GraphQL query before the resolutions listed five unresolved threads and each mutation returned `isResolved: true`.
- 2026-11-19 09:31 UTC - The second review of `da2ce7`, 5972450674 (approved, submitted 09:31:39), arrived with F5 to F8.
- 2026-11-19 09:33 UTC - Committed the fixes of F5 and F8 (commit times 09:33:18 and 09:33:59) and edited the PR description for F6 and F8.
- 2026-11-19 09:36 UTC - Pushed the two fixes (the push ended at 09:36:36) and replied on the four threads (09:36:47 to 09:36:54).
- 2026-11-19 09:37 UTC - Resolved the threads of F5, F6 and F8 (each mutation returned `isResolved: true`); the GraphQL query before them listed four unresolved threads, so one, F7, stays open until this record is pushed.
- 2026-11-19 09:37 UTC - Added F5 to F8 and these entries. The push of this commit and the resolution of the F7 thread follow it and are not logged.
- 2026-11-19 09:45 UTC - After the push of round 2 and the resolution of the F7 thread, Copilot's review 5972638002 (submitted 09:45:35) arrived with R2, and the third review of `da2ce7`, 5972648538 (approved, submitted 09:46:15), with F9.
- 2026-11-19 09:48 UTC - Committed the fixes of R2 and F9 locally (commit times 09:48:05 and 09:48:57), not pushed yet.
- 2026-11-19 09:49 UTC - Replied on the two threads (09:49:33 and 09:49:36). Each reply says its commit is local and goes up in one push with this update.
- 2026-11-19 09:49 UTC - Added R2 and F9 and these entries. One push of the three commits of round 3 and the resolution of the two threads follow this commit and are not logged.
- 2026-11-19 16:42 UTC - The maintainer decided D1 to D5 in the agent session. Committed the decision record in the specification (commit time 16:43:14), not pushed.
- 2026-11-19 18:34 UTC - Pushed that commit at the maintainer's request and edited the PR description (its "Decisions" section now lists what was decided); both were done by 18:34:14.
- 2026-11-19 18:38 UTC - The fourth review of `da2ce7`, 5981075592 (approved, submitted 18:38:58), arrived on that head with F10 and F11.
- 2026-11-19 18:42 UTC - Committed the fixes of F11 and F10 locally (commit times 18:41:33 and 18:42:07) and replied on the two threads (both replies were posted by 18:42:55). Each reply says its commit is local and goes up in one push with this update.
- 2026-11-19 18:43 UTC - Added F10 and F11 and these entries. One push of the three commits of round 4 and the resolution of the two threads follow this commit and are not logged.

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
