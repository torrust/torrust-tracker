---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3061 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3061.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`
- Reviewer: the author's GitHub login
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`, `maintainability`, `security`, `other`

## Findings

F10 is the pre-merge check in the review body of CodeRabbit's review 6064399158. F1 to F9 are the inline comments of da2ce7's review 6062055409. The review of `claude[bot]` (6061783591) is the
notice that automated review is disabled for fork pull requests and has no row. The review body of
6062055409 carries a summary, the checks and its own findings only as these inline comments.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3061-f1 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3061-f2 | da2ce7 | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3061-f3 | da2ce7 | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3061-f4 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3061-f5 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3061-f6 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3061-f7 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3061-f8 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3061-f9 | da2ce7 | Suggestion | maintainability | ORIGINAL | NO_ACTION | RESOLVED |
| F10 | review-finding:pr-3061-f10 | coderabbitai[bot] | Minor | other | ORIGINAL | NO_ACTION | NON_RESOLVABLE |

## Finding Details

### F1 - The V2 row of ISSUE.md is DONE but described the V1 measuring test, which was not what

- PR number: 3061
- Source review ID: 6062055409
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698466414
- Concern: The V2 row of ISSUE.md is DONE but described the V1 measuring test, which was not what ran on the container databases.
- Solution: Reworded the V2 row to the two harness cases, their runners and the evidence file; recorded the container image tags (mysql 8.0, postgres 16 defaults) in the evidence file.
- Current-tree verification: The V2 row and the evidence section V2 name the harness cases and the image tags.
- Resolution reference: docs(issues): [#3052] address the first review of the fix PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698541198

### F2 - The rounds rationale describes a read-then-write window that the store-level cases cannot

- PR number: 3061
- Source review ID: 6062055409
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698466424
- Concern: The rounds rationale describes a read-then-write window that the store-level cases cannot have, and the evidence claims too much when it says that the race reproduces on all three drivers.
- Solution: Rewrote the CONCURRENT_ROUNDS comment (repeating the cases catches nondeterministic lock errors) and the evidence paragraph (the store-level red shows duplicate-write rejection; the DatabaseWhitelist race was observed on SQLite only).
- Current-tree verification: The comment in databases/driver/mod.rs and the paragraph after the V2 red output say so.
- Resolution reference: docs(issues): [#3052] address the first review of the fix PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698541506

### F3 - No prose-first Arrange-Act-Assert comparison is recorded for the new and replaced

- PR number: 3061
- Source review ID: 6062055409
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698466434
- Concern: No prose-first Arrange-Act-Assert comparison is recorded for the new and replaced tests.
- Solution: Added a Test intent section to the evidence file: prose per test (two persisted.rs tests, two concurrent harness cases, three idempotency tests) and the result of the comparison with the code.
- Current-tree verification: The evidence file has the section `Test intent (prose-first Arrange-Act-Assert)` listing all seven tests.
- Resolution reference: docs(issues): [#3052] address the first review of the fix PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698541880

### F4 - The five completion criteria stay unchecked while the workflow checkpoints and AC1 to AC4

- PR number: 3061
- Source review ID: 6062055409
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698466439
- Concern: The five completion criteria stay unchecked while the workflow checkpoints and AC1 to AC4 are checked.
- Solution: Checked the five criteria, each backed by the pre-commit gate, the test runs and the evidence file.
- Current-tree verification: The five lines under Acceptance Criteria are `- [x]` in ISSUE.md.
- Resolution reference: docs(issues): [#3052] address the first review of the fix PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698542249

### F5 - The frontmatter still reads `status: planned` and `related-pr: null` with the fix PR

- PR number: 3061
- Source review ID: 6062055409
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698466446
- Concern: The frontmatter still reads `status: planned` and `related-pr: null` with the fix PR open.
- Solution: Set `status: in-review` and `related-pr: 3061`.
- Current-tree verification: The frontmatter of ISSUE.md shows both values and the frontmatter validator passes.
- Resolution reference: docs(issues): [#3052] address the first review of the fix PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698542597

### F6 - The V1 Evidence cell says `(before-fix result recorded)` although the section also

- PR number: 3061
- Source review ID: 6062055409
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698466452
- Concern: The V1 Evidence cell says `(before-fix result recorded)` although the section also records the rerun.
- Solution: Changed the cell to `(before and after the fix)`.
- Current-tree verification: The V1 row of ISSUE.md carries the new text.
- Resolution reference: docs(issues): [#3052] address the first review of the fix PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698542966

### F7 - The Design Decision bullet still states the superseded add contract (`0` means nothing to

- PR number: 3061
- Source review ID: 6062055409
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698466459
- Concern: The Design Decision bullet still states the superseded add contract (`0` means nothing to do) without pointing at the MySQL finding.
- Solution: Added a pointer from the bullet to the Risks section and AC3, which supersede it for the add count.
- Current-tree verification: The bullet in the Design Decision section carries the pointer.
- Resolution reference: docs(issues): [#3052] address the first review of the fix PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698543406

### F8 - The trait doc says the concurrent removes' row counts add up to 1, which holds only when

- PR number: 3061
- Source review ID: 6062055409
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698466474
- Concern: The trait doc says the concurrent removes' row counts add up to 1, which holds only when the torrent was listed.
- Solution: Reworded to `when the torrent was listed, their row counts add up to 1`.
- Current-tree verification: `databases/traits/whitelist.rs` carries the new wording.
- Resolution reference: docs(issues): [#3052] address the first review of the fix PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698543863

### F9 - One row of the open #2902 spec still cites a test this change

- PR number: 3061
- Source review ID: 6062055409
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698466484
- Concern: One row of the open #2902 spec still cites a test this change removes.
- Solution: None here: that spec folder is out of scope for this PR. The owner of the #2902 spec is told in the report so that the row can point at #3052.
- Current-tree verification: `docs/issues/open/2902-1347-add-tests-tracker-core/` is not touched by this PR.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698544290
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#discussion_r4698544290

### F10 - The pre-merge check asks to remove the review audit record from this pull request

- PR number: 3061
- Source review ID: 6064399158
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3061#pullrequestreview-6064399158
- Concern: CodeRabbit's "Out of Scope Changes" pre-merge check asks to remove `docs/pr-reviews/pr-3061-review/PR-REVIEW.md` from the diff.
- Solution: None: `process-pr-review` makes the audit record a tracked file owned by the pull request author, validated before each audit commit, so it belongs in this pull request.
- Current-tree verification: `.claude/skills/process-pr-review/SKILL.md` names the record and its validator; the maintainer reviewers check it in every round.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3061#issuecomment-7173571506
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3061#issuecomment-7173571506

## Processing Log

- 2026-11-25 17:27 UTC - da2ce7's approval 6062055409 arrived with nine inline findings (F1 to F9).
- 2026-11-25 17:38 UTC - Fixed F1 to F8 in one commit and pushed it; created this record from the template before replying; F9 needs no change here.
- 2026-11-25 19:57 UTC - The branch was rebased onto develop after the maintainer merge failed its pre-push tests (see the spec progress log); commit ids cited in review texts before this entry belong to the pre-rebase history, and this record cites commits by subject.
- 2026-11-25 20:22 UTC - CodeRabbit's review 6064399158 arrived on the rebased head with one review-body finding (F10); replied on the conversation and recorded it as NO_ACTION.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
