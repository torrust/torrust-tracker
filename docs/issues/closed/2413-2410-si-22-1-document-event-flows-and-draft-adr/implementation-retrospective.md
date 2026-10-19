---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr/ISSUE.md
    - docs/issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr/agent-review-reports.md
    - docs/issues/closed/2413-2410-si-22-1-document-event-flows-and-draft-adr/manual-verification-evidence.md
    - docs/pr-reviews/pr-2553-review/review-retrospective.md
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
---

# Implementation Retrospective — Document Event Flows and Draft the Shutdown-Order ADR

## Purpose

Record evidence-based process improvements discovered while implementing issue #2413 (SI-22.1).
This is a blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

Written after merge, on 2026-10-19, to close the sub-EPIC's retrospective gate, which was skipped
before push. The PR review retrospective,
[`review-retrospective.md`](../../../pr-reviews/pr-2553-review/review-retrospective.md), already
covers how PR #2553 was reviewed: its first six reviews, why rounds 3 to 5 were about the author's
own records, and the inventory search miss (`review-finding:pr-2553-f2`). This document does not
repeat that. It covers what the PR review retrospective does not: the tooling defect hit while
pushing, how the differences found by T1 were routed, the skipped delivery gates, and the findings
of the post-merge Task Reviewer report in [`agent-review-reports.md`](agent-review-reports.md).

## Outcome

PR #2553 delivered the four tasks as separate commits: the producer-consumer inventory in
`docs/features/shutdown-process/task-inventory.md` (T1), the move of the jobs doc to
`docs/architecture/application-jobs.md` (T2a), `docs/architecture/glossary.md` (T2b), and
`adr-draft.md` in the sub-EPIC folder (T2c), approved by the maintainer before it was committed. It
merged on 2026-10-19 at 15:18 UTC after 32 review findings over ten review rounds, all fixed, and
da2ce7's approval 5571665038 with no finding (`PR-REVIEW.md`).

The post-merge Task Reviewer report found that AC1 to AC3 and `linter all` pass on the merged tree,
with four should-fix findings (verdict `POST_MERGE_FINDINGS`). Its F1 is corrected in the prep pull
request that also carries this document.

## What Went Well

1. **Small commits per task.** T1, T2a (move and link updates only), T2b, and T2c are separate
   commits, as the Task Reviewer confirmed, so each criterion could be checked against its own
   change.
2. **Differences were reported, not silently applied.** T1 reported to the coordinator, under D14,
   that `docs/architecture/events.md` omits the swarm bus and that tokio in `Cargo.lock` is 1.53.3,
   instead of rewriting the design record on its own.
3. **The tooling defect followed the bug process.** When the pre-push hook damaged the real
   repository, nothing was pushed. The damage was reverted by hand, the defect was reproduced
   against a throwaway repository with the real test binary, and a regression test at the seam was
   red before the fix and green after it (`manual-verification-evidence.md`).

## What Changed During Implementation

1. **The pre-push hook ran fixture Git commands against the real repository.** Git exports
   `GIT_DIR` and other repository-selecting variables to hooks, and the fixture helpers of
   `clippy-allow-reasons` did not clear them. The first push left a stray test commit, an extra
   branch, and `user.name` and `user.email` in the shared `.git/config`. This was fixed in its own
   commit, `fix(dev-tools): [#2413] isolate clippy-allow-reasons fixture Git commands from hook environment`,
   inside a documentation-only issue. No tracking issue was opened. The maintainer accepted these
   extra changes on 2026-10-19 (Task Reviewer F4).
2. **The entry 27 recheck credited the entry with facts it does not state** (Task Reviewer F1).
   The table had two rows, "`udp_ban_cleanup` has no events" and "Torrent cleanup and the REST API
   publish no event of their own", and marked the second one "Differs". Neither appears in entry 27.
   The nearest source, the sub-EPIC's "D Versus F" section, already named torrent cleanup as a swarm
   producer, so the reported difference did not exist. `git log -S` shows that the two rows'
   wording first appears in the T1 commit itself. Corrected in
   `docs(shutdown): [#2413] attribute the publish-nothing facts to D Versus F, not entry 27`.
3. **A T1 finding stayed in logs until 2026-10-19.** The missing swarm bus in `events.md` was
   recorded in two progress logs and in the inventory ("left to the sub-EPIC's later documentation
   task"), but not in the task that will fix it. It reached #2416's T10 and the sub-EPIC's T10 step
   only in the prep pull request, after the coordinator picked it up from the logs.
4. **Gates were skipped.** The Task Reviewer report and this retrospective were not done before
   push. The two new tests in `contrib/dev-tools/checks/clippy-allow-reasons/tests/cli.rs` got no
   prose-first Arrange-Act-Assert review with the maintainer (Task Reviewer F3). In
   `it_should_not_let_fixture_commands_inherit_the_repository_environment` the expectation is the
   same constant the helper loops over. `it_should_list_every_variable_git_reports_as_repository_local`
   pins that constant to `git rev-parse --local-env-vars`, which compensates. The maintainer chose to
   record this gap here and not to review the tests after the fact.

## Root Cause

1. **The "documentation only" framing outlived the plan.** The spec marked the bug process and
   regression tests "not applicable" because the issue changed only Markdown. When an unplanned code
   fix with new tests entered the pull request, nobody re-applied the test-design gates to it. The
   PR was opened after `linter all` and the hooks only, as the PR review retrospective records.
2. **A recheck table summarized its source instead of citing it.** The column "Fact in entry 27"
   was filled from memory of the whole design record, not from the entry's text, so one row was a
   fact from another section and another was a claim that no section made.
3. **Coordinator findings had no owner in the specs.** D14 asks for design changes to be agreed and
   recorded. These findings were facts for a later task, not design changes, and the progress log
   was the only place they were written. No later task reads the log, so the finding waited for
   someone to notice it.

## Improvements for Future Work

1. **Run the Task Reviewer and record the retrospective before push** — sub-EPIC Shared Delivery
   Gates — no change: the guidance existed and was not followed. The coordinator's #2414 brief
   repeats it.
2. **When unplanned code enters a pull request, apply the gates for code to it** (bug process,
   prose-first Arrange-Act-Assert review, Task Reviewer), whatever the issue type says. A lesson for
   #2414 to #2416; no rule added (see below).
3. **In a recheck table, cite the source line for every row.** If a row's fact is not in the cited
   source, it does not belong in that table. A lesson for the #2414 and #2416 evidence; no template
   change.
4. **Write a finding that needs later work into the spec of the task that will do it, in the same
   pull request.** Applied for the swarm-bus finding in the prep pull request (#2416 T10 and the
   sub-EPIC T10 step).

## Avoiding Overcorrection

- No rule against fixing tooling defects inside another issue's pull request. The repository lets
  minor process improvements ride along in their own commits, and the maintainer accepted this fix.
- No new checker for recheck tables or log-only findings: one occurrence each.
- No workspace-wide Git-isolation rule. A search for `Command::new("git")` in Rust test files finds
  fixture Git commands only in `frontmatter-validator` and `clippy-allow-reasons`, and both now clear
  the repository-local variables (searched on 2026-10-19, read from source, not run). Shell and
  Python tests were not checked.

## Evidence

- [Issue specification](ISSUE.md): Scope, Verification Plan, and progress log
- [Manual verification evidence](manual-verification-evidence.md): M1 and its correction, and the
  tooling defect's reproduction, red, green, and recheck
- [Agent review reports](agent-review-reports.md): the post-merge Task Reviewer report and its
  maintainer decisions
- [PR review retrospective](../../../pr-reviews/pr-2553-review/review-retrospective.md) and
  [PR review audit](../../../pr-reviews/pr-2553-review/PR-REVIEW.md)
- Sub-EPIC [`EPIC.md`](../../open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md):
  record entry 27, "D Versus F", Shared Delivery Gates
- PR #2553 and its merge commit
  `Merge torrust/torrust-tracker#2553: docs(shutdown): [#2413] document event flows and draft the shutdown-order ADR`
