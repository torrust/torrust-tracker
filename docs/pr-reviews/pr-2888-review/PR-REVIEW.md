---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2888 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2888.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2888-f1 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2888-f2 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The fixture tests and the two properties carry no AAA markers

- PR number: 2888
- Source review ID: 5923877279
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4588718259
- Concern: The `write-unit-test` checklist asks for `// Arrange` / `// Act` / `// Assert` markers,
  and the format test follows it; the 14 fixture tests and the two `#[quickcheck]` properties do
  not. The reviewer offered two fixes: add the markers, or record in the test plan that they were
  dropped as redundant prose.
- Solution: Recorded the decision in the test plan, for both the properties and the fixture tests:
  the markers are redundant prose for one-expression properties and one- to three-statement bodies
  (the skill's prose-first step 4), matching the other short tests in `peer.rs`.
- Current-tree verification: `grep -n "markers were left out" docs/issues/open/2876-1347-peer-id-test-helper/test-plans/qbittorrent-peer-id.md`
  finds the property note (T2 section) and the fixture-test note (T7 section).
- Resolution reference: docs(issues): [#2876] record why the short peer.rs tests carry no AAA markers
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4588757231

### F2 - The amendment leaves the O1c claim that machete does not flag the self dev-dependency

- PR number: 2888
- Source review ID: 5923877279
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4588718262
- Concern: The branch's own retrospective shows that `cargo machete --with-metadata` flagged the
  self dev-dependency of `primitives`, a package without `tests/` or examples, but the ADR's O1c row
  still says machete does not flag it, and the retrospective deferred the note.
- Solution: The 2026-11-16 amendment says the O1c note holds only when `tests/` or examples name
  the crate, and that a package without them lists the self dev-dependency in its machete `ignored`
  list with the reason. The retrospective's improvement 1 now points at the amendment.
- Current-tree verification: `grep -n "O1c note" docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md`
  finds the clause in the amendment of 2026-11-16.
- Resolution reference: docs(adrs): [#2876] say when cargo machete flags a testing self dev-dependency
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2888#discussion_r4588757687

## Processing Log

- 2026-11-16 16:52 UTC - Started audit for review round 1: review 5923877279 (da2ce7, APPROVED, with
  ACK, one Nit F1 and one Suggestion F2, neither blocking). The maintainer had posted his ACK on the
  same head before the review. Both fixed by commit; the two thread replies posted at 16:53 UTC,
  before the push of the fix commits.

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
