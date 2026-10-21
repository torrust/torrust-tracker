---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2525-ci-gate-required-check/pr-ci-cost-baseline.md
    - docs/issues/open/2525-ci-gate-required-check/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2590 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2590>.

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

One review was submitted so far. Copilot's review 5596405557 read the head whose subject is "docs(issues):
[#2525] record the before-baseline of the pull-request CI runner time". It carries three inline threads, F1
to F3. Its overview repeats them, so the body adds no rows.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2590-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2590-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2590-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The per-pull-request cohort cannot be reproduced from the saved data

- PR number: 2590
- Source review ID: 5596405557
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2590#discussion_r4334212630>
- Concern: section 2.2 excludes 11 pull requests by creation date, but the saved JSON holds only the class, the needed groups, the file count, and the merge time, and the CSV's `created_at` describes runs, not pull requests, so the 87 included pull requests cannot be identified and their statistics cannot be reproduced offline as the document promises. Save each pull request's creation time or an explicit included list, and document how to apply the selection.
- Solution: each of the 98 entries of the saved JSON has `created` and `merged` times and a `complete_history` flag, true for the 87 pull requests created on or after 2026-10-07 00:00 UTC. Section 5 says how to apply the selection and defines the 90th percentile.
- Current-tree verification: table 2.2 was recomputed from the two committed files alone with exactly that selection, and every cell matched (47 and 40 pull requests, 4.87 and 4.30 heads, the means, medians, 90th percentiles, and maxima).
- Resolution reference: docs(issues): [#2525] save the creation time and cohort of each baseline pull request
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2590#discussion_r4334292662>

### F2 - The `Docs Lint` duration bucket counts are wrong

- PR number: 2590
- Source review ID: 5596405557
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2590#discussion_r4334212688>
- Concern: the CSV has four `Docs Lint` durations between 1.5 and 3.0 minutes (1.65, 1.78, 2.83, and 2.98), so with 444 runs and 48 at 3.0 or more, 392 take under 1.5 minutes, not 394. Correct both counts.
- Solution: the text says 392 under 1.5 minutes, 48 at 3.0 to 4.9, and 4 in between, and names the four.
- Current-tree verification: from the committed CSV: 392 runs under 1.5, 4 in the interval from 1.5 up to but not including 3.0 (1.65, 1.78, 2.83, 2.98), and 48 at 3.0 or more, which sum to 444.
- Resolution reference: docs(issues): [#2525] correct the Docs Lint duration bucket counts in the baseline
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2590#discussion_r4334292958>

### F3 - The slow-run rate pools documentation-only and code pull requests

- PR number: 2590
- Source review ID: 5596405557
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2590#discussion_r4334212732>
- Concern: the one-in-nine rate is 48 of 444 runs over both classes, but documentation-only pull requests have 17 slow runs out of 270 (6.3%, about one in sixteen, with a slow-run median of 4.32 minutes), so the pooled rate overstates the baseline the gate's cost is compared with. State it as a per-run statistic by class and correct the repeated frequency in `ISSUE.md`.
- Solution: section 2.4 has a per-class table (documentation-only 17 of 270, 6.3%, slow median 4.32 minutes; code 31 of 174, 17.8%, slow median 4.35) and says the share is per run. The log entry in `ISSUE.md` says one run in sixteen takes about 4.3 minutes.
- Current-tree verification: recomputed from the committed CSV: 17 of 270 and 31 of 174, with the medians above. The median of all code runs is 0.94 minutes; it was first written as 0.9 and corrected in the same commit.
- Resolution reference: docs(issues): [#2525] give the Docs Lint slow-run rate per class of pull request
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2590#discussion_r4334293194>

## Processing Log

- 2026-10-21 07:38 UTC - Copilot's review arrived with three threads, F1 to F3. Each was checked against
  the committed data, and all were confirmed; F2 was an error of mine.
- 2026-10-21 07:44 UTC - Fixed each finding in its own signed commit, recorded the round in the
  specification, ran the pre-push checks, and pushed.
- 2026-10-21 07:48 UTC - Replied on the three threads after the push, and started this record.

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
