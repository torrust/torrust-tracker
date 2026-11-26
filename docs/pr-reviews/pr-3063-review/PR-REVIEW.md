---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3063 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3063.

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

Findings 1 to 7 come from review 6070914967 and findings 8 to 10 from review 6075692867, both by `da2ce7`. The
`claude[bot]` review (6062721178, automated review is disabled for fork pull requests) and the
`coderabbitai[bot]` comment (a rate-limit notice) hold no actionable assertion and have no row.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3063-f1 | da2ce7 | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3063-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3063-f3 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3063-f4 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3063-f5 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3063-f6 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3063-f7 | da2ce7 | Suggestion | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3063-f8 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3063-f9 | da2ce7 | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3063-f10 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Rule 10 still holds the open duplicate-key status

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774545
- Concern: The ADR still contained the open decision on the status of a duplicate REST key and a sentence promising an update before merge; merged at that head it would record no status and the sentence would be false.
- Solution: Recorded that a duplicate key keeps its 500 in v1 with the new text, removed the open block and the sentence, updated the REST table lead-in, the duplicate-key row and the pull request description.
- Current-tree verification: Read rule 10 and the REST table of the ADR at the new head: no `Open for the maintainer` text remains (`grep -n 'Open for\|open decision'` prints nothing).
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709492415

### F2 - The duplicate-key text in the Description predates #2999

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774555
- Concern: The Description said a duplicate key answers `UNIQUE constraint failed: keys.key`, which was true only before #2999.
- Solution: The Description now gives that text as history and states the current reply and that the status is still 500.
- Current-tree verification: Read the Description's database bullet in the ADR: it names #2999 and the current text.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709492920

### F3 - Only the no-persistence container configuration binds the health-check API to 0.0.0.0:1313

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774564
- Concern: The ADR said the container configurations bind the health-check API to `0.0.0.0:1313`; only the no-persistence and the e2e configurations do.
- Solution: Rule 9 now names the default `127.0.0.1:1313`, the two configurations that bind `0.0.0.0:1313` and that an operator can bind it publicly.
- Current-tree verification: Read rule 9 of the ADR.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709493436

### F4 - `char::escape_default` escapes more than control characters

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774572
- Concern: `escape_default` also escapes quotes, backslash and all non-ASCII characters, and the ADR did not say whether the 64-character cut counts before or after escaping.
- Solution: Rule 5 uses `char::escape_debug`, counts the 64 characters on the value as sent and escapes after the cut.
- Current-tree verification: Read rule 5 of the ADR.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709493938

### F5 - The tracker release keeps no changelog

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774580
- Concern: Rule 4 promised a changelog entry for a changed text, but the tracker has no changelog.
- Solution: Rule 4 now says the pull request that changes a text lists old and new in its description and the notes of the GitHub release summarize them.
- Current-tree verification: Read rule 4 of the ADR.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709494490

### F6 - The v2 mapping lists 409 only under refused

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774589
- Concern: Rule 10 listed `409` only under refused while a duplicate key, a bad request, maps to it.
- Solution: Rule 10 lists `409` under bad request too and says a duplicate key is a `409` in v2.
- Current-tree verification: Read the v2 paragraph of rule 10.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709495285

### F7 - Mark the amendment in the errors ADR's index row

- PR number: 3063
- Source review ID: 6070914967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4705774600
- Concern: The index row of the errors ADR still summarized rule 4 without marking the amendment.
- Solution: The row now ends with `(amended 2026-11-25)`, like the other amended ADRs.
- Current-tree verification: `grep -c 'amended 2026-11-25' docs/adrs/index.md` prints 1.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709495910

### F8 - No audit record exists at this head

- PR number: 3063
- Source review ID: 6075692867
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709637476
- Concern: The seven threads of the first review were answered and resolved, but `docs/pr-reviews/pr-3063-review/PR-REVIEW.md` did not exist, as `process-pr-review` requires before the first reply.
- Solution: Created the audit record with one row per finding of both rounds; it is a late start, logged in the Processing Log.
- Current-tree verification: Ran `validate-audit-record.py --pr-number 3063` at the new head.
- Resolution reference: docs(pr-reviews): [#3000] record the review rounds of pull request 3063
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709719639

### F9 - The branch conflicts with develop in docs/adrs/index.md

- PR number: 3063
- Source review ID: 6075692867
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709637487
- Concern: `develop` inserted the `20261124120307` row next to the edited row, so the branch conflicted and the pull request was not mergeable.
- Solution: Rebased the branch onto `develop` and resolved the conflict by keeping this ADR's rows and the new row, in order.
- Current-tree verification: `git merge-tree --write-tree torrust/develop HEAD` after the rebase exits 0, and the index lists the `20261124120307`, `20261124182709` and `20261125175726` rows in order.
- Resolution reference: docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709720083

### F10 - No record shows the decision the ADR attributes to the maintainer

- PR number: 3063
- Source review ID: 6075692867
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709637495
- Concern: The ADR sentence in rule 10 attributed the duplicate-key decision to the maintainer, but no record at that head showed it.
- Solution: The ADR sentence states the rule without naming who decided it, and the specification's progress log records the decision with its date and how it was given.
- Current-tree verification: Read the end of rule 10 and the progress-log entry of 2026-11-26 in the specification.
- Resolution reference: docs(adrs): [#3000] drop the decision attribution from rule 10 and record it in the specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3063#discussion_r4709720583

## Processing Log

- 2026-11-26 14:07 UTC - Late start: the seven replies to review 6070914967 were posted (14:07 UTC) before this record existed; the replies carry the dispositions, and this record was created after review 6075692867 noted its absence.
- 2026-11-26 14:21 UTC - Fixes for F1 to F7 committed in `docs(adrs): [#3000] keep the v1 status of a duplicate key and fix the review findings`; F9 resolved by the rebase onto `develop` in the same commit.
- 2026-11-26 14:22 UTC - Fix for F10 committed in `docs(adrs): [#3000] drop the decision attribution from rule 10 and record it in the specification`; the branch was pushed with `--force-with-lease` after the rebase.
- 2026-11-26 14:26 UTC - Replies to F8, F9 and F10 posted.

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
