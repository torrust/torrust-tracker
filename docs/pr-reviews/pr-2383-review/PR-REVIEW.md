---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #1840"
---

<!-- skill-link: process-pr-review -->

# PR #2383 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2383>.

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

Copilot review 5364586939 (round 1, at the head
`docs(self-hosted-runner): add capacity as a second server`) left three inline comments without
severity brackets. Its review overview rates F1 and F2 Medium and F3 Low, recorded as
`Minor (inferred)` and `Nit (inferred)` as earlier audits do. The review also warns that Copilot's
full agentic review did not start before its timeout; that warning names no finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2383-f1` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2383-f2` | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2383-f3` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The draft cited raw data in the untracked `.tmp/` directory

- PR number: 2383
- Source review ID: 5364586939
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2383#discussion_r4143280315>
- Concern: The queue evidence pointed readers to `.tmp/runner-queue-jobs.tsv`, which is git-ignored
  and unavailable to anyone else.
- Solution: The spec was given a new scope and rewritten. Its queue evidence now states the source and the
  method (jobs API, `started_at - created_at`) and the time windows, and task T2 records the
  reproduced data in the issue-local `manual-verification-evidence.md`.
- Current-tree verification: at the PR head after the 2026-09-30 10:15 UTC changes,
  `grep -n 'runner-queue-jobs' docs/issues/drafts/1840-self-hosted-runner-minimum-capacity/ISSUE.md`
  prints nothing. The spec still mentions `.tmp` twice, but neither cites data: a progress-log
  entry records that the draft moved out of `.tmp/`, and the pre-commit command sets
  `TORRUST_GIT_HOOKS_LOG_DIR=.tmp`.
- Resolution reference: `docs(issues): [#1840] rescope runner capacity to a minimum-capacity issue`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2383#discussion_r4143469633>

### F2 - Numeric step references in the operations guide are brittle

- PR number: 2383
- Source review ID: 5364586939
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2383#discussion_r4143280367>
- Concern: "Repeat steps 2 to 13" breaks silently when setup steps are inserted or removed.
- Solution: Both "repeat steps 2 to 13" instructions now link to "Set Up a New Runner" and name its
  "Create the Server" through "Verify" sections. The three other numeric step references (steps 3,
  10, and 13) were replaced by section names too, because they break the same way.
- Current-tree verification: at the PR head after the 2026-09-30 10:15 UTC changes,
  `grep -cE 'step [0-9]+|steps [0-9]' docs/self-hosted-runner.md` prints `0`, and `linter lychee`
  passes for the new `#set-up-a-new-runner` fragment links.
- Resolution reference: `docs(self-hosted-runner): reference setup sections by name`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2383#discussion_r4143469817>

### F3 - The goal count did not match the numbered list

- PR number: 2383
- Source review ID: 5364586939
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2383#discussion_r4143280406>
- Concern: The Goal section said "Two goals must hold together" but listed three.
- Solution: The new spec has a single goal, determining and documenting the minimum runner
  capacity, so the numbered list is gone.
- Current-tree verification: at the PR head after the 2026-09-30 10:15 UTC changes,
  `grep -n 'goals must hold' docs/issues/drafts/1840-self-hosted-runner-minimum-capacity/ISSUE.md`
  prints nothing.
- Resolution reference: `docs(issues): [#1840] rescope runner capacity to a minimum-capacity issue`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2383#discussion_r4143469985>

## Processing Log

- 2026-09-30 10:12 UTC - Committed the rewritten spec, which removes the F1 and F3 texts.
- 2026-09-30 10:14 UTC - Committed the F2 fix.
- 2026-09-30 10:15 UTC - Replied on the F1, F2, and F3 threads and resolved all three; started
  this audit.
- 2026-09-30 10:19 UTC - Re-deriving the F1 claim showed the spec still mentions `.tmp` twice
  without citing data; edited the F1 reply, which had said the spec no longer referenced `.tmp/`.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the original thread outdated after the push. For a
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
