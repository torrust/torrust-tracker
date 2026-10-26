---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2654 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2654>.

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

Round 1 is the Copilot review `5649374266` (submitted 2026-10-26 08:42 UTC). The reviewer
did not assign a finding ID, so the audit assigns F1. The maintainer da2ce7 was a requested reviewer
and had submitted no review on this pull request when this round was processed.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2654-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - Add required manual reproduction evidence before bug-spec review

- PR number: 2654
- Source review ID: 5649374266
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2654#discussion_r4375360491
- Concern: The fix-bug skill requires an attempted reproduction and a recorded outcome before a bug specification is reviewed, but the issue folder held only `ISSUE.md` and the reverted-test summary did not preserve reproducible evidence. The reviewer asked for `manual-verification-evidence.md` with the commands, output and an outcome of `Reproduced`, `Trigger only` or `Infeasible`, leaving red/green testing and the final recheck to implementation.
- Solution: Reproduced the bug on an isolated real tracker (REST API only) with seven sequential `curl` probes and recorded the configuration, probe order with timestamps, output and conclusion in the new `manual-verification-evidence.md` (V1, outcome `Reproduced`). `ISSUE.md` marks T2 and M1 as done, links the evidence and sets `related-pr` to 2654; V2 stays pending for implementation.
- Current-tree verification: Read `manual-verification-evidence.md` and the T2, M1 and Bug-Fix Process lines of `ISSUE.md` after the change; the probe output in the evidence is the output of the commands that ran (P1 `200`, P2 and P3 `500 unknown token provided`, P4 and P7 `500 unauthorized`, P5 and P6 `200`).
- Resolution reference: docs(issues): [#2653] record the real-tracker reproduction before spec review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2654#discussion_r4377341444

## Processing Log

- 2026-10-26 08:42 UTC - Copilot review `5649374266` submitted with one inline finding. The maintainer da2ce7 had submitted no review on this pull request.
- 2026-10-26 12:43 UTC - Reproduced the bug on the real tracker and committed the evidence as F1's fix; pushed it to the branch.
- 2026-10-26 12:45 UTC - Replied to the F1 thread (reply created 12:45:58 UTC).

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
