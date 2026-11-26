---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3079 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3079.

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

Three inline findings from source review 6078083300, one from source review 6078251304 and one
from source review 6078413481 are tracked below. All five replies were posted and all five threads
are resolved. The validator run at 17:46 covered the four-row record; the five-row record awaits
validation.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3079-f1 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3079-f2 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3079-f3 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3079-f4 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3079-f5 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Assign the formatter-path commit to a task

- PR number: 3079
- Source review ID: 6078083300
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711500081
- Concern: Commit Points assigned one commit to each of T1 through T15 but did not assign the sixteenth prepared commit for the formatter path.
- Solution: T6 now names the separate final formatter-path correction; Commit Points gives it its own commit without a register row.
- Current-tree verification: The implementation history has sixteen commits above base commit `66c12ffe1`; the final commit changes the run-linters skill and reference and says it closes no register row. The specification's Commit Points now assigns that commit to T6.
- Resolution reference: docs(issues): [#3073] assign the formatter correction to T6
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711725292

### F2 - Allow a conditional retrospective in AC3

- PR number: 3079
- Source review ID: 6078083300
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711500089
- Concern: AC3's 32-file limit excluded the issue-local retrospective that the completion review can require.
- Solution: Scope, T16 and AC3 now allow that file when required: 32 files in the stated plan or 33 with a retrospective.
- Current-tree verification: The completion review requires `implementation-retrospective.md` when its listed conditions occur. AC3 now permits it conditionally, in T16's evidence commit.
- Resolution reference: docs(issues): [#3073] allow a required retrospective in AC3
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711725664

### F3 - Check the out-of-scope passages in AC5 evidence

- PR number: 3079
- Source review ID: 6078083300
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711500105
- Concern: The AC5 evidence row required whole lines to stay unchanged, although T14 adds an in-scope index instruction on lines an out-of-scope question also cites.
- Solution: The evidence row now checks the passages the out-of-scope rows ask about and allows an in-scope edit on the same lines.
- Current-tree verification: The diff from base commit `66c12ffe1` to the implementation head for `docs/discussions/AGENTS.md` adds the index instruction to the opening-round paragraph; the current AC5 evidence row checks the out-of-scope text rather than unchanged lines.
- Resolution reference: docs(issues): [#3073] make AC5 evidence check the reserved text
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711726066

### F4 - Replace the broken formatter test in the verification plan

- PR number: 3079
- Source review ID: 6078251304
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711628915
- Concern: The required dictionary-formatter test resolves fixture paths below `contrib/contrib/dev-tools/` and fails before it exercises T5's hook change.
- Solution: T5 and the acceptance criterion now use the hook-step comparison in M2; the failed test is removed from the verification plan, and its path defect is named outside this issue's scope.
- Current-tree verification: Line 13 of `contrib/dev-tools/checks/tests/test-format-project-words.sh` climbs three parents from its `tests/` directory. Running the script exited 1, printed `cp` and `chmod` errors for missing files under `contrib/contrib/dev-tools/`, then printed a dictionary diff. The specification now relies on `linter all` and M2 for T5.
- Resolution reference: docs(issues): [#3073] remove broken formatter test from verification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711726454

### F5 - Allow conditional reviewer reports in AC3

- PR number: 3079
- Source review ID: 6078413481
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711748598
- Concern: AC3 excluded the issue-local `agent-review-reports.md` required when an independent reviewer receives the folder-style specification.
- Solution: T16 and AC3 now allow the conditional report alongside the conditional retrospective; the scope and EPIC row state 32 files without either, 33 with one and 34 with both.
- Current-tree verification: The specification and `docs/templates/ISSUE.md` require the report under that condition. The specification's Scope, T16 and AC3 now include it, and the EPIC row states the same file counts.
- Resolution reference: docs(issues): [#3073] allow conditional reviewer reports in AC3
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3079#discussion_r4711881982

## Processing Log

- 2026-11-26 17:13 UTC - Started the audit from captured review data after committing three specification corrections locally. Reply URLs and final thread states await publication and replies.
- 2026-11-26 17:23 UTC - Added F4 from review 6078251304 after the dictionary-formatter test exited 1; the specification correction is committed locally. The reply URL awaits publication and reply.
- 2026-11-26 17:29 UTC - Six specification commits were pushed to PR #3079; the push returned at 17:29:12Z.
- 2026-11-26 17:29 UTC - Replies to comments 4711500081, 4711500089, 4711500105 and 4711628915 were posted at 17:29:14Z, 17:29:16Z, 17:29:19Z and 17:29:22Z, respectively. The threads remain open.
- 2026-11-26 17:35 UTC - A capture taken at 17:35Z showed the F1, F2 and F3 threads resolved by `coderabbitai[bot]`; their exact resolution times are unavailable.
- 2026-11-26 17:46 UTC - The validator run reported `ok` at 17:46:34Z against the committed four-row record: 4 rows, 4 log entries and 0 failures. It did not cover F5.
- 2026-11-26 17:46 UTC - The review-record commit and the reviewer-reports fix were pushed; the push completed at 17:46:56Z.
- 2026-11-26 17:46 UTC - The reply to comment 4711748598 was posted at 17:46:57Z.
- 2026-11-26 17:47 UTC - `da2ce7` resolved F4 at 17:47:00Z and F5 at 17:47:01Z. The thread capture at 17:47:22Z shows all five resolved.

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
