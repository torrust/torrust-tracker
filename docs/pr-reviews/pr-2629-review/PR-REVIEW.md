---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2629 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2629>.

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

Round 1: da2ce7's review 5631559010 (APPROVED, on the opening head, the commit whose subject is "docs(issues): [#2628] specify moving the test-helpers protocol helpers") carries four inline threads, F1 to F4, and an ACK of that head, which is stale after the fix commits. Copilot's review 5631574526 (COMMENTED, same head) says "No open issues" and opens no thread, so it raises no actionable assertion and adds no row. Severities are the labels of the source comments.

Round 2: da2ce7's review 5631788587 (COMMENTED, submitted 2026-10-23T17:04:09Z, after the round 1 fixes and audit) carries three nit threads, F5 to F7, next to an ACK of the head that carried the round 1 audit. Severities are the labels of the source comments.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2629-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2629-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2629-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2629-f4 | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2629-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2629-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2629-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Progress Log line filed under References

- PR number: 2629
- Source review ID: 5631559010
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4361996049>
- Concern: the EPIC row 2 Progress Log line sat under `## References`, not under `### Progress Log`.
- Solution: the entry is under `### Progress Log`.
- Current-tree verification: read the EPIC headings and the entry's position.
- Resolution reference: docs(issues): [#2628] file the EPIC row 2 progress-log line under Progress Log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362035742>

### F2 - Check scope wider than AC3

- PR number: 2629
- Source review ID: 5631559010
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4361996055>
- Concern: the spec said ADR rule B.1 becomes a check, which is wider than the entries AC3 adds.
- Solution: the spec states the check covers `client-lib` and the three domain crates and that B.1 stays a convention for the others.
- Current-tree verification: re-read the spec's deny.toml scope and AC3.
- Resolution reference: docs(issues): [#2628] state which crates the deny.toml check covers
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362036104>

### F3 - Spec frontmatter branch and status

- PR number: 2629
- Source review ID: 5631559010
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4361996061>
- Concern: the spec frontmatter `branch:` should name the spec branch and `status:` should be `planned` while the spec PR is open.
- Solution: `branch:` is the `-spec` branch and `status:` is `planned`.
- Current-tree verification: read the frontmatter.
- Resolution reference: docs(issues): [#2628] name the spec branch and keep status planned until the spec PR merges
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362036447>

### F4 - Drafted checkpoint lost the drafts path

- PR number: 2629
- Source review ID: 5631559010
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4361996069>
- Concern: the ticked checkpoint about drafting the spec named the `open/` path though it was drafted under `drafts/`.
- Solution: the checkpoint names the `drafts/` path again.
- Current-tree verification: read the checkpoint line.
- Resolution reference: docs(issues): [#2628] restore the draft path in the drafted checkpoint
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362036692>

### F5 - Rule B.3 misread

- PR number: 2629
- Source review ID: 5631788587
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362177235>
- Concern: the spec said "as rule B.3 already is", which misreads B.3.
- Solution: the sentence now says B.1 stays a convention, as rule B.3 already says.
- Current-tree verification: read the sentence and ADR rule B.3.
- Resolution reference: docs(issues): [#2628] say that rule B.3 already keeps B.1 a convention
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362373141>

### F6 - EPIC and inventory not restamped

- PR number: 2629
- Source review ID: 5631788587
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362177245>
- Concern: the EPIC and the inventory were edited in this PR but their `last-updated-utc` was not restamped.
- Solution: both files carry a fresh stamp.
- Current-tree verification: read the frontmatter of both files.
- Resolution reference: docs(issues): [#2628] restamp the EPIC and inventory edited by the spec PR
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362373424>

### F7 - Processing Log misstates the order of events

- PR number: 2629
- Source review ID: 5631788587
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362177253>
- Concern: the single round 1 entry misstates the order of fixes, pushes and replies and omits the bot review.
- Solution: the log is append-only, so a new entry records the actual order, see the 17:26 UTC entry below.
- Current-tree verification: compared the entry with the review and comment timestamps from the GitHub API.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362373718
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2629#discussion_r4362373718>

## Processing Log

- 2026-10-23 16:49 UTC - Started audit. da2ce7 reviewed at 16:44 UTC (4 threads, F1 to F4). Verified each claim, committed one fix each, rebased onto `develop`, pushed and replied on every thread; resolved all 4 threads at 16:48 UTC (GraphQL showed 0 unresolved). The ACK names the head before the fixes, so a new ACK is needed on the head that carries this audit record.
- 2026-10-23 17:27 UTC - Correction of the entry above, from the GitHub API timestamps. da2ce7's review 5631559010 was submitted at 16:43:57Z and Copilot's review 5631574526 at 16:45:24Z. The reply on F1 was posted at 16:48:23Z, one second before the force-push at 16:48:24Z, and the replies on F2 to F4 at 16:48:25Z to 16:48:29Z, after it. Round 2: da2ce7's review 5631788587 (17:04:09Z) raised F5 to F7; each got its own fix commit, a plain push, a reply (17:25:56Z to 17:26:00Z) and was then resolved; this audit commit records it. A new ACK is needed on the head that carries it.

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
