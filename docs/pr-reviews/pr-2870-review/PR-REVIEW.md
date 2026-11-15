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
reviewer finding ID and gets the audit-local ID `F3`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| R1 | review-finding:pr-2870-r1 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2870-r2 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2870-f3 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

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

## Processing Log

- 2026-11-15 21:06 UTC - Copilot review 5912933402 submitted with three inline findings.
- 2026-11-15 21:09 UTC - Committed the R1 fix (`test(jobs): gate the drain controller ...`).
- 2026-11-15 21:14 UTC - Committed the R2 and F3 fix (`docs(issues): record reproducible M2 evidence ...`).
- 2026-11-15 21:15 UTC - Replied on the three threads.

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
