---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3074 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3074>.

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
- Reviewer: the author's GitHub login, for example `da2ce7` or `coderabbitai[bot]`; `unknown`
  when GitHub shows no author
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

Round 1 covers review 6077978896 (da2ce7, APPROVED) of the head `docs(issues): [#2392] record the 2026-11-26 decisions for #2925`, with three inline findings, F1 to F3: two Minor and one Nit. The review body says none blocks. The CodeRabbit comment and the Claude bot review on the PR are a summary and a fork notice, not findings.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3074-f1 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3074-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3074-f3 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The #2925 spec did not record the decisions yet

- PR number: 3074
- Source review ID: 6077978896
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3074#discussion_r4711428271
- Concern: The new Decisions entry said the #2925 spec records the decisions, but at this head that spec (pull request 3067 not merged) still says the order and naming are open, so the sentence was true only if 3067 merged first.
- Solution: The sentence now says pull request 3067 records them in the #2925 spec, which is true before or after either pull request merges.
- Current-tree verification: `grep -c 'Pull request 3067 records them in the #2925 spec' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: `docs(issues): [#2392] set order 10 in progress and reword the 2026-11-26 decision entry`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3074#discussion_r4711568056

### F2 - Order 10 still said TODO

- PR number: 3074
- Source review ID: 6077978896
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3074#discussion_r4711428290
- Concern: The Subissues table kept order 10 at TODO although the Decisions entry records decisions taken in its T1.
- Solution: Order 10 is IN_PROGRESS, because its T1 spec pull request (3067) is open. A new progress-log entry records the change and the 16:11 entry keeps its text.
- Current-tree verification: `grep -c '^| 10 | #2925 .*| IN_PROGRESS |' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: `docs(issues): [#2392] set order 10 in progress and reword the 2026-11-26 decision entry`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3074#discussion_r4711568616

### F3 - A sentence parsed two ways

- PR number: 3074
- Source review ID: 6077978896
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3074#discussion_r4711428301
- Concern: "approved the order and the pattern with \"Approve both\" and the naming" could be read as approving with "Approve both" and the naming together.
- Solution: The sentence now says the maintainer approved the order and the pattern ("Approve both") and, separately, the naming, in the #2925 session.
- Current-tree verification: `grep -c 'and, separately, the naming' docs/issues/open/2392-test-isolation/EPIC.md` prints 1.
- Resolution reference: `docs(issues): [#2392] set order 10 in progress and reword the 2026-11-26 decision entry`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3074#discussion_r4711569083

## Processing Log

- 2026-11-26 17:14 UTC - Started audit. Round 1: da2ce7 review 6077978896 (APPROVED) with three findings, F1 to F3, all valid. Pushed the fix (`docs(issues): [#2392] set order 10 in progress and reword the 2026-11-26 decision entry`) and replied to the three threads. Entries are appended, never rewritten.
- 2026-11-26 17:15 UTC - The validator on the worktree record printed `{"status": "ok", "rows": 3, "log_entries": 1, "failures": 0}`. Resolved the three round 1 threads, selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread, and the newest review by a reviewer in the review list was 6077978896. Committed these entries.

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
