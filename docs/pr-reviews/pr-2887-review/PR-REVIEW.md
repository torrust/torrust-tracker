---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2887 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2887.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

F1, F2 and F3 are the three inline comments of Copilot's review 5923805482 (`COMMENTED`), submitted at 16:46:34 UTC on 2026-11-16 on the PR's first head commit, `docs(issues): [#2003] replace the issue body mirror rule with a short pointer body`, in that source order. They carry no finding IDs, so they take audit-local IDs. Their severity is inferred from the review overview, which marks F1 and F2 "Medium severity" and F3 "Low severity"; earlier audits record both marks as `Minor (inferred)`. The review body says Copilot's full agentic review did not start before its timeout; the body holds no finding beyond the three inline comments.

DOC-001 is the one inline comment of Copilot's review 5924220422 (`COMMENTED`), submitted at 17:14:15 UTC on 2026-11-16 on the head commit `docs(pr-reviews): [#2003] record the PR #2887 review round 1`. The ID and the `[Minor]` severity are the reviewer's own.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2887-f1 | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2887-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2887-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| DOC-001 | review-finding:pr-2887-doc-001 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The new decision item is not indented like the lines above it

- PR number: 2887
- Source review ID: 5923805482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2887#discussion_r4588659136
- Concern: Line 300 of the EPIC specification starts at column 0 while lines 297 to 299 above it are indented, so the reviewer expected it to break the list.
- Solution: No change. Line 300 records a separate decision, so it is a top-level item like the other decisions of "Decisions Recorded on This EPIC"; lines 297 to 299 are sub-items of the previous decision.
- Current-tree verification: `awk '/^## Decisions Recorded/,/^## Friction Register/' docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md | grep -c '^- \*\*'` counts the top-level items of the section: it printed 14 on the branch and 13 on the merge base `c8b1477ad`, so the new item is counted as a top-level one; `linter markdown` passed in the pre-commit checks of the two fix commits.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2887#discussion_r4588716956
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2887#discussion_r4588716956

### F2 - "da2ce7" is ambiguous in the decision entry

- PR number: 2887
- Source review ID: 5923805482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2887#discussion_r4588659216
- Concern: A reader of the decision entry cannot tell whether "da2ce7" is a commit hash, a user name or another identifier.
- Solution: The entry now says "the GitHub user da2ce7, whose ownership ended on 2026-11-06". It uses no `@` mention; the ownership dates and their links are in the EPIC owner line at the top of the file.
- Current-tree verification: `grep -c "GitHub user da2ce7, whose ownership ended on 2026-11-06"` on the EPIC specification printed 1.
- Resolution reference: docs(issues): [#2003] say who da2ce7 is in the issue body decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2887#discussion_r4588717976

### F3 - The last sentence of Issue Body Pointer is hard to parse

- PR number: 2887
- Source review ID: 5923805482
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2887#discussion_r4588659284
- Concern: The sentence ended with "only a change to ... does", with the verb implied.
- Solution: It is now two sentences, and the second names its verb: "The body changes only when the goal, this file's path or the place where the register is kept changes."
- Current-tree verification: `grep -c "The body changes only when the goal"` on the EPIC specification printed 1.
- Resolution reference: docs(issues): [#2003] reword when the issue body needs a change
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2887#discussion_r4588719103

### DOC-001 - The specification records the body conversion as done

- PR number: 2887
- Source review ID: 5924220422
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2887#discussion_r4588988577
- Concern: "The body now holds only stable content" read as if the issue body were already replaced, but issue #2003 still had its long body and the PR description leaves that edit to the maintainer.
- Solution: The decision entry, the Issue Body Pointer section, the risk line and this change's progress-log entry now state the rule ("must hold only") and say that replacing the stale body is a separate edit by the maintainer, which the change does not make. The four statements were found by searching the specification for "stable content". The PR description already says the body is not in the PR.
- Current-tree verification: `grep -n "now holds\|body holds only\|It holds only"` on the EPIC specification prints one line, 444, an unrelated progress-log entry of 2026-10-13; none of the four statements matches.
- Resolution reference: docs(issues): [#2003] state the issue body rule as a rule, not as done
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2887#discussion_r4589049418

## Processing Log

- 2026-11-16 16:51 UTC - Started audit, after the replies: review 5923805482 arrived at 16:46:34 UTC; the two fix commits were made at 16:47:43 and 16:48:18 UTC and pushed; the three replies were posted at 16:51:09, 16:51:14 and 16:51:20 UTC and the three threads were then resolved (each `resolveReviewThread` call returned `isResolved: true`). The record was created after the replies, not before the first one as the workflow asks.
- 2026-11-16 17:20 UTC - Round 2: review 5924220422 arrived at 17:14:15 UTC with DOC-001. The fix commit was made at 17:17:03 UTC and pushed. The reply was posted at 17:19:43 UTC and the thread was then resolved (`isResolved: true`). The reply first said the verification command prints 0; it prints one unrelated line, so the reply was edited at 17:19:57 UTC to say so and to name the edit.

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
