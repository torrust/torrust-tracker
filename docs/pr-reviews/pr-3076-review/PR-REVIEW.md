---
pr-number: 3076
pr-url: https://github.com/torrust/torrust-tracker/pull/3076
last-updated-utc: "2026-11-26 17:03"
---

<!-- skill-link: process-pr-review -->

# PR #3076 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3076.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and do not own this record. A reply file that an issue-linked
review task names is a separate artifact, which the reviewer writes (see the `review-pr` skill).

- Post-merge workflow approval: N/A (pre-merge review).

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

Round 1: `da2ce7`'s review 6077983778 (approved, with four inline findings) on the head of commit `docs(issues): [#2904] complete the inventory rows of the researcher profile and record the task review of T8` (F1 to F4). The review of `claude[bot]` (6077611100) says that automated review is disabled for fork pull requests and carries no finding.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3076-f1 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F2 | review-finding:pr-3076-f2 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-3076-f3 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | OPEN |
| F4 | review-finding:pr-3076-f4 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - The final message drops the evidence the skill's report requires

- PR number: 3076
- Source review ID: 6077983778
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3076#discussion_r4711431841
- Concern: The final message listed the conclusion, the reply path, the sources and the open questions only, so a run without a reply file said which sources were read but not what each said, and with a reply file the restated question and the comparison had no field.
- Solution: The final message is the report of the skill followed by the verdict and the reply path, and the reply file names `summary` (the question, the conclusion with its confidence, the comparison) and `evidence` (each source and fact, and the sources consulted).
- Current-tree verification: Read "Final message" and "Reply file" of `.claude/agents/researcher.md` after the edit.
- Resolution reference: docs(agents): [#2904] make the final message of the researcher carry the report and limit the read-only boundary to the reply file and the clones
- Follow-up PR URL: N/A
- Reply URL: pending

### F2 - The read-only boundary's exceptions are not cases of its rule

- PR number: 3076
- Source review ID: 6077983778
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3076#discussion_r4711431849
- Concern: The boundary said "do not change any tracked file" but its two exceptions are an untracked file and clones in a git-ignored folder, so it let a session create any untracked file; the agent file listed only the reply file as an exception.
- Solution: The boundary says "do not create or change any file of the workspace" with the same two exceptions, in the skill and in the agent file, and the inventory row says so.
- Current-tree verification: Read the Boundaries of the skill, line 14 of the agent file and the inventory row after the edit.
- Resolution reference: docs(agents): [#2904] make the final message of the researcher carry the report and limit the read-only boundary to the reply file and the clones
- Follow-up PR URL: N/A
- Reply URL: pending

### F3 - The finding count of the progress log does not match the reply

- PR number: 3076
- Source review ID: 6077983778
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3076#discussion_r4711431856
- Concern: The entry and the pull-request description said "six minor findings and one suggestion", but the reply holds four minor findings, two nits and one suggestion.
- Solution: The entry and the pull-request description say "four minor findings, two nits and one suggestion".
- Current-tree verification: Compared the entry with `agent-replies/task-reviewer-t8-r1.json` and edited the description.
- Resolution reference: docs(agents): [#2904] make the final message of the researcher carry the report and limit the read-only boundary to the reply file and the clones
- Follow-up PR URL: N/A
- Reply URL: pending

### F4 - A quotation does not match the specification's text

- PR number: 3076
- Source review ID: 6077983778
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3076#discussion_r4711431865
- Concern: The introduction quoted "facts-not-decisions" as the specification's cell, which reads "Ad hoc, without the \"facts, not decisions\" rule".
- Solution: The quotation is the cell as it is.
- Current-tree verification: Read the table "What the ten profiles did" of `ISSUE.md` and the introduction after the edit.
- Resolution reference: docs(agents): [#2904] make the final message of the researcher carry the report and limit the read-only boundary to the reply file and the clones
- Follow-up PR URL: N/A
- Reply URL: pending

## Processing Log

- 2026-11-26 17:03 UTC - Round 1: started on `da2ce7`'s review `6077983778` (four inline comments); the record was created before the first reply, and the fixes are committed locally. Replies follow the push.

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
