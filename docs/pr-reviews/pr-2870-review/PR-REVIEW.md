---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2870 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2870.

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

Copilot used the reviewer finding ID `R1` for two different findings. The second keeps `R1` as its
reviewer finding ID and gets the audit-local ID `F3`. The da2ce7 review numbers its findings `F1` to
`F9`, which collide with `F3`, so they get the audit-local IDs `C1` to `C9` and keep their own IDs as
reviewer finding IDs. Its second review continues with `F10` to `F12`, recorded as `C10` to `C12`, and its approving review
with `F13`, recorded as `C13`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| R1 | review-finding:pr-2870-r1 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2870-r2 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2870-f3 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| C1 | review-finding:pr-2870-c1 | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| C2 | review-finding:pr-2870-c2 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| C3 | review-finding:pr-2870-c3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| C4 | review-finding:pr-2870-c4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| C5 | review-finding:pr-2870-c5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| C6 | review-finding:pr-2870-c6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| C7 | review-finding:pr-2870-c7 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| C8 | review-finding:pr-2870-c8 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| C9 | review-finding:pr-2870-c9 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| C10 | review-finding:pr-2870-c10 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| C11 | review-finding:pr-2870-c11 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| C12 | review-finding:pr-2870-c12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| C13 | review-finding:pr-2870-c13 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### R1 - Gate the drain controller in the server-join regression test

- PR number: 2870
- Source review ID: 5912933402
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4580239341
- Concern: The table test's `joined` signal only proved the controller reached its send, so it passed even if the supervisor returned on a server-join error without joining the controller.
- Solution: Removed the `joined` assertion from the table test and added `it_should_wait_for_the_drain_controller_when_the_server_join_fails`, which gates the controller after it observes cancellation and asserts the supervisor stays pending until the gate is released, on both branches.
- Current-tree verification: `cargo test --lib token_aware_server` passes 4 tests. With the old early return restored on both branches, the new test fails at its timeout assertion (`src/bootstrap/jobs/token_aware_server.rs:262`); the file was then restored.
- Resolution reference: test(jobs): gate the drain controller in the server-join regression test (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4580268950

### R2 - Keep the unexecuted M4 scenario pending

- PR number: 2870
- Source review ID: 5912933402
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4580239379
- Concern: M4 was marked DONE although only a unit test was cited and the real-binary run was not performed.
- Solution: M4 is TODO in `ISSUE.md` and the evidence file, with the reason (the configuration rejects a reset interval below 3600 s). Sweep for the same mistake: M3 had been marked BLOCKED without running and is TODO too, and the manual-verification checkpoint is unticked.
- Current-tree verification: `ISSUE.md` lists M1 DONE, M2 DONE, M3 TODO and M4 TODO, and the checkpoint `Manual verification scenarios executed` is unchecked.
- Resolution reference: docs(issues): record reproducible M2 evidence and keep M3 and M4 pending (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4580269051

### F3 - Record reproducible evidence for each M2 signal run

- PR number: 2870
- Source review ID: 5912933402
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4580239397
- Concern: The M2 evidence recorded neither the launch, signal and wait commands nor the binding-release checks, and summarized the SIGINT run as "Same sequence".
- Solution: Re-ran M2 for SIGTERM and SIGINT with a script on dedicated ports, and recorded the script and its full output: config diff, bindings before start, after start and after exit, exit status and time after the signal, and the relevant log lines per run.
- Current-tree verification: `manual-verification-evidence.md` contains the script and the output of the 21:13 UTC run: both runs exit 0 and every binding is gone after exit.
- Resolution reference: docs(issues): record reproducible M2 evidence and keep M3 and M4 pending (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4580269155

### C1 - Remove the closing keyword from the PR description

- PR number: 2870
- Source review ID: 5915608609
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4582424447
- Concern: Line 17 of the PR description used `Closes #2845` in a negated sentence; the keyword closes the issue wherever it appears, and line 5 credited the table test with the controller-join case.
- Solution: Edited the PR description: the line reads `Related to #2845` and says the issue stays open; line 5 names `it_should_wait_for_the_drain_controller_when_the_server_join_fails`; the M3/M4 line says both stay TODO.
- Current-tree verification: `gh-bot pr view 2870 --json body` shows no closing keyword; the only `#2845` mention is `Related to #2845`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583109548
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583109548

### C2 - Show the other components' stop order in the M2 evidence

- PR number: 2870
- Source review ID: 5915608609
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4582424450
- Concern: M2 expects the other components' stop order unchanged, but the evidence kept only the `udp_ban_cleanup` and `health_check_api` lines.
- Solution: Re-ran M2 keeping every job-manager stop line for all components, and stated in the Result that "unchanged" means matching the edges this PR did not touch.
- Current-tree verification: `manual-verification-evidence.md` records the 2026-11-16 07:39 UTC run: every listener is asked to stop after its producers completed and `health_check_api` last, for both signals.
- Resolution reference: docs: fix the stale shutdown docs and record M2 with every component's stop lines (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583109772

### C3 - Repair the `reserve_stop_last` docs

- PR number: 2870
- Source review ID: 5915608609
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4582424454
- Concern: Removing the legacy clause left a sentence fragment and two empty doc lines; the `reserve` and `cancel` paragraphs were unevenly wrapped.
- Solution: Dropped the fragment and one empty line, and rewrapped both paragraphs.
- Current-tree verification: `src/bootstrap/jobs/manager.rs` has no fragment; `cargo test --lib` and clippy pass.
- Resolution reference: refactor(jobs): repair manager doc comments and neutral stop-order assertion wording (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583110017

### C4 - Place the ban cleanup among the direct components in the inventory

- PR number: 2870
- Source review ID: 5915608609
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4582424457
- Concern: The tree hung the ban cleanup off `JobManager` beside the direct components and the detailed inventory had no entry for it.
- Solution: Moved the node under `Direct JoinSet components`, added a detail bullet, and updated the stale Findings item 1 that called it a compatibility-registry entry.
- Current-tree verification: `docs/features/shutdown-process/task-inventory.md` tree, detailed inventory and Findings agree with the table row.
- Resolution reference: docs: fix the stale shutdown docs and record M2 with every component's stop lines (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583110251

### C5 - Count three usage-gated components

- PR number: 2870
- Source review ID: 5915608609
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4582424464
- Concern: `tracker_usage_statistics` gates three direct components, but the formula used `2I_{usage}`.
- Solution: Changed the term to `3I_{usage}` and named the three components below the formula.
- Current-tree verification: With the stop-order test's configuration the formula gives 14, the number of outcomes the test expects.
- Resolution reference: docs: fix the stale shutdown docs and record M2 with every component's stop lines (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583110513

### C6 - Remove the live root-token mention

- PR number: 2870
- Source review ID: 5915608609
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4582424468
- Concern: `docs/features/shutdown-process/README.md` still said the application cancels the root token.
- Solution: Reworded it to `JobManager::cancel()`.
- Current-tree verification: `git grep -i "root token"` over `docs/` outside issues and reviews finds only historical contrasts.
- Resolution reference: docs: fix the stale shutdown docs and record M2 with every component's stop lines (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583110754

### C7 - Align the evidence frontmatter with its template

- PR number: 2870
- Source review ID: 5915608609
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4582424473
- Concern: The evidence used `doc-type: evidence` and fields the template does not define, and had no `issue-spec`.
- Solution: Set `doc-type: manual-verification-evidence`, `issue-spec` and `last-updated-utc`, as in the template.
- Current-tree verification: The frontmatter of `manual-verification-evidence.md` matches `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- Resolution reference: docs: fix the stale shutdown docs and record M2 with every component's stop lines (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583111053

### C8 - Word the ordering assertion for any edge

- PR number: 2870
- Source review ID: 5915608609
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4582424484
- Concern: `assert_completes_before_every` reported a bus that `udp_ban_cleanup` does not publish on.
- Solution: Renamed the parameters to `first` and `later` and worded the message neutrally.
- Current-tree verification: `src/app.rs` helper message reads `'{first}' must complete before '{component}' is cancelled`; the stop-order test passes.
- Resolution reference: refactor(jobs): repair manager doc comments and neutral stop-order assertion wording (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583111244

### C9 - Remove trailing spaces from the recorded output

- PR number: 2870
- Source review ID: 5915608609
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4582424487
- Concern: `git diff --check` flagged seven trailing-space lines in the recorded M2 output.
- Solution: Fixed the script (`paste -sd' '`, `none` when nothing is bound, trimmed diff and log output) and re-ran it rather than editing the output.
- Current-tree verification: `git diff --check` from the merge base `abd7a10ff` to the current head prints nothing.
- Resolution reference: docs: fix the stale shutdown docs and record M2 with every component's stop lines (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583111464

### C10 - Cite C1's fix by a durable URL

- PR number: 2870
- Source review ID: 5916635531
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583200322
- Concern: C1's resolution reference `PR description edit (no commit)` is neither a commit subject nor a URL, so the audit validator rejects it.
- Solution: Replaced it with the durable thread reply URL that states the description edit.
- Current-tree verification: C1's `Resolution reference` is `https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583109548`.
- Resolution reference: docs(pr-reviews): audit the second da2ce7 review of PR 2870 (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583218165

### C11 - Do not cite a PR-branch commit by id

- PR number: 2870
- Source review ID: 5916635531
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583200328
- Concern: C9's verification cited a PR-branch commit by its id (the commit `docs(pr-reviews): audit the first Copilot review of PR 2870 (#2845)`), which a rebase would change.
- Solution: Cited the merge base `abd7a10ff` on `develop` instead.
- Current-tree verification: `git diff --check abd7a10ff HEAD` prints nothing, and no PR-branch commit id in `abd7a10ff..HEAD` appears in `docs/` or `src/`.
- Resolution reference: docs(pr-reviews): audit the second da2ce7 review of PR 2870 (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583218455

### C12 - Drop the stale summary line of the assertion helper

- PR number: 2870
- Source review ID: 5916635531
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583200335
- Concern: `assert_completes_before_every` kept its old summary line naming `producer` and `consumers`.
- Solution: Dropped that line.
- Current-tree verification: `src/app.rs` has one summary line for the helper, naming `first` and `later`.
- Resolution reference: test(app): drop the stale summary line of the stop-order assertion helper (#2845)
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583218733

### C13 - Remove the PR-branch commit id from C11

- PR number: 2870
- Source review ID: 5916759580
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583289425
- Concern: C11's Concern named the PR-branch commit id it set out to remove, so the commit-id check still printed it, and C11's verification and the F11 reply were wrong at that head.
- Solution: The maintainer merged PR #2870 as is; the close-out PR describes that commit by its subject in C11.
- Current-tree verification: the `open-pull-request` skill's commit-id check prints no PR-branch commit id for this record.
- Resolution reference: docs(issues): close out #2845 with the M3 and M4 evidence
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2880
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2870#discussion_r4583363366

## Processing Log

- 2026-11-15 21:06 UTC - Copilot review 5912933402 submitted with three inline findings.
- 2026-11-15 21:09 UTC - Committed the R1 fix (`test(jobs): gate the drain controller ...`).
- 2026-11-15 21:14 UTC - Committed the R2 and F3 fix (`docs(issues): record reproducible M2 evidence ...`).
- 2026-11-15 21:15 UTC - Replied on the three threads.
- 2026-11-16 05:55 UTC - da2ce7 review 5915608609 (changes requested) submitted with nine inline findings.
- 2026-11-16 07:41 UTC - Committed the C3 and C8 fix (`refactor(jobs): repair manager doc comments ...`).
- 2026-11-16 07:41 UTC - Committed the C2, C4 to C7 and C9 fix (`docs: fix the stale shutdown docs ...`).
- 2026-11-16 07:42 UTC - Edited the PR description for C1.
- 2026-11-16 07:42 UTC - Replied on the nine threads.
- 2026-11-16 07:55 UTC - da2ce7 review 5916635531 (commented) submitted with three inline findings.
- 2026-11-16 07:57 UTC - Committed the C12 fix (`test(app): drop the stale summary line ...`).
- 2026-11-16 07:58 UTC - Replied on the three threads.
- 2026-11-16 08:08 UTC - da2ce7 review 5916759580 (approved) submitted with one inline finding (F13, non-blocking).
- 2026-11-16 08:17 UTC - Replied on the F13 thread: fix deferred to the close-out PR at the maintainer's choice.
- 2026-11-16 08:27 UTC - PR #2870 merged by the maintainer.

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
