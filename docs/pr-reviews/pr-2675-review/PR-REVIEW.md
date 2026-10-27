---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2675 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2675>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the review was processed before the merge)

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

Round 1 is the Copilot review `5666060526` (https://github.com/torrust/torrust-tracker/pull/2675#pullrequestreview-5666060526), submitted 2026-10-27 11:07:54 UTC. Its body holds a verdict and a summary of one open issue that links to the single inline thread below, so it adds no separate row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2675-f1 | Copilot | Major | correctness | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Prevent new jobs from starting during Docker recovery

- PR number: 2675
- Source review ID: 5666060526
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2675#discussion_r4387924622
- Concern: `contrib/dev-tools/self-hosted-runner/docker-ci-prune.sh` checks for a running job with `pgrep -x Runner.Worker` and then restarts `docker.service`, but the runner stays online in between. A job that starts after the check or during the restart can lose its containers or fail to reach Docker, because Docker's live restore is off. Copilot asks to coordinate recovery with runner job admission so new work is blocked until Docker is ready without interrupting existing jobs, and to add a regression case where a job arrives after the idle check.
- Solution: no change, declined by the maintainer (josecelano) on 2026-10-27 in the chat session that processed this review; the durable record is the reply below. The finding is accurate. The risk is accepted for now because the restart only runs at 04:00 UTC and only when the disk is still at or above 70% after the prune, which is the stuck state of issue 2673 (the disk measured 42% with a 100 GB cache and 51% with a 127 GB cache on 2026-10-27), and the cost of a hit is one failed check that is re-run. The limitation is documented in `docs/self-hosted-runner.md`, and the design that would fix it, with the trigger to implement it, is recorded as R5 in `docs/issues/open/2673-runner-disk-full-build-cache-not-pruned/deferred-proposals.md`, in the commit `docs(self-hosted-runner): [#2673] document the job-arrival race of the Docker restart`. The option that was proposed to the maintainer, a lock held by the script plus a runner job-started hook that waits on it, was not implemented; its hook behavior was not verified on the runner.
- Current-tree verification: `grep -n "pgrep -x Runner.Worker\|systemctl restart docker.service" contrib/dev-tools/self-hosted-runner/docker-ci-prune.sh` printed line 41 and line 60 on the pushed branch, so the finding holds. `grep -n "^### R5\|^6\. Implement R5"` on `deferred-proposals.md` printed lines 68 and 83, and `grep -n "Known limitation" docs/self-hosted-runner.md` printed line 416, so the documentation the reply cites exists on the branch.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2675#discussion_r4388533997
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2675#discussion_r4388533997

## Processing Log

- 2026-10-27 11:07 UTC - Copilot review `5666060526` submitted (GitHub `submitted_at` 11:07:54 UTC) with one inline Major finding (F1) and no other actionable assertion.
- 2026-10-27 12:05 UTC - Fetched the threads again with `github-review-threads fetch` (command start 12:05:21 UTC) and found one unresolved thread, F1. Before that, on the maintainer's request to address Copilot's review, I had fetched the same thread once and asked the maintainer how to handle F1; that question was cancelled and nothing was changed.
- 2026-10-27 12:13 UTC - The maintainer declined F1 in chat (message time 12:13 UTC) and asked for the thread to be replied to with the reason and the decision recorded in this audit.
- 2026-10-27 12:14 UTC - Committed `docs(self-hosted-runner): [#2673] document the job-arrival race of the Docker restart` (commit time 12:14:50 UTC, from `git log --format=%cI`), and pushed it before replying (the push output showed `6eb27e8db..d22d42af1`).
- 2026-10-27 12:15 UTC - Replied on the thread (reply `created_at` 12:15:20 UTC, GitHub); `reply-status --login josecelano` exited 0; the resolve mutation returned `isResolved: true` (the command's own date line printed 12:15:32 UTC). A refetch then listed the thread as resolved by josecelano and zero unresolved threads.
- 2026-10-27 12:16 UTC - Wrote this audit and validated it with `validate-audit-record.py --pr-number 2675 --base torrust/develop` (run at 12:16:15 UTC), which exited 0 with 1 row, 6 log entries, and 0 failures.

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
