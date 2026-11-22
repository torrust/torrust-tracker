---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2994 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2994>.

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

Round 1 covers review 6012624473 (Copilot, COMMENTED, submitted 2026-11-22T14:05:07Z, a lite review) with seven inline comments on `docs/issues/open/2993-secrets-and-client-error-text/EPIC.md`. Its body is an overview with no finding beyond the inline comments; its warning that the full agentic review did not start is context and makes no row. The comments carry no `[Severity]` bracket. The audit severity scale has no medium or low level, so this audit maps the overview's `Medium severity` badge (F1 to F3) to `Minor (inferred)` and its `Low severity` badge (F4 to F7) to `Nit (inferred)`, as the PR #2990 audit does. The seven comments are three texts: F2 and F3 repeat F1 on other lines, and F6 and F7 repeat F5, so they are re-raises with their own rows.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2994-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2994-f2 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2994-f3 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2994-f4 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2994-f5 | Copilot | Nit (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-2994-f6 | Copilot | Nit (inferred) | documentation | RE_RAISE_OF:F5 | NO_ACTION | SUPERSEDED |
| F7 | review-finding:pr-2994-f7 | Copilot | Nit (inferred) | documentation | RE_RAISE_OF:F5 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - A10 and A11 are missing from the A table and the "A1 to A9 were reproduced" sentence is inaccurate

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421443
- Concern: The inventory numbering is inconsistent: A10 and A11 are introduced later but are not in the A table, and "A1 to A9 above were reproduced on a running tracker" is inaccurate because A9 is documentation and A10 and A11 are also part of the verified set. Copilot suggests adding A10 and A11 to the table and making the verification bullet match what was reproduced.
- Solution: The A table has rows A10 and A11. The verification bullet says which items were reproduced on a running tracker (A1, A3, A4; A2 and A5 in the next item; A10 and A11 below), which were confirmed by reading the code (A6, A7, A8), and that A9 is documentation. Copilot's example wording "A1–A8, A10–A11" was not used because A6 to A8 were not reproduced on a running tracker.
- Current-tree verification: `grep -c '^| A1[01] |' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 2; `grep -c 'A1 to A9 above were reproduced' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 0; the verification bullet reads "A1, A3 and A4 were reproduced on a running tracker (A2 and A5 in the next item, A10 and A11 below). A6, A7 and A8 were confirmed by reading the code, and A9 is documentation."
- Resolution reference: `docs(issues): [#2993] list A10 and A11 in the inventory table and state what was reproduced`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463288

### F2 - Same inventory-numbering comment on the first verification bullet

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421459
- Concern: Same text as F1, anchored on original line 76 of the reviewed commit (the verification bullet).
- Solution: No separate change: F1's fix covers it.
- Current-tree verification: Same as F1: `grep -c '^| A1[01] |' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 2; `grep -c 'A1 to A9 above were reproduced' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 0; the verification bullet reads "A1, A3 and A4 were reproduced on a running tracker (A2 and A5 in the next item, A10 and A11 below). A6, A7 and A8 were confirmed by reading the code, and A9 is documentation."
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463401
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463401

### F3 - Same inventory-numbering comment on the A10 and A11 bullets

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421468
- Concern: Same text as F1, anchored on the A11 bullet (line 84 of the current file).
- Solution: No separate change: F1's fix covers it.
- Current-tree verification: Same as F1: `grep -c '^| A1[01] |' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 2; `grep -c 'A1 to A9 above were reproduced' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 0; the verification bullet reads "A1, A3 and A4 were reproduced on a running tracker (A2 and A5 in the next item, A10 and A11 below). A6, A7 and A8 were confirmed by reading the code, and A9 is documentation."
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463527
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463527

### F4 - "Key must be have 32 chars" reads like an error in the spec

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421476
- Concern: The excerpt "Key must be have 32 chars" reads like a grammatical error. Copilot asks to mark it as verbatim (a note or "[sic]") if it is the system's output, or to correct it if paraphrased, and suggests naming this message quality issue in the motivation of SI-B.
- Solution: The A2 reproduction says the text is verbatim, including the grammar slip "must be have", cites the source line, and says SI-A replaces the message. The suggestion to add it to SI-B's motivation was not taken: the message is about a peer key, and In Scope item 2 of the SI-A draft (`docs/issues/drafts/2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md`) owns the new key messages.
- Current-tree verification: `grep -c 'The text is verbatim, including the grammar slip "must be have"' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 1; `packages/tracker-core/src/authentication/key/peer_key.rs:195` reads `#[error("Invalid key length. Key must be have 32 chars")]`; line 54 of the SI-A draft proposes `Invalid authentication key: expected 32 characters, got <n>`.
- Resolution reference: `docs(issues): [#2993] mark the quoted key-length message as verbatim`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463632

### F5 - 'Subissues', 'sub-issue' and 'subissue' are used inconsistently

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421494
- Concern: The document uses 'Subissues', 'sub-issue' and 'subissue'. Copilot recommends one form (for example 'sub-issue' / 'Sub-issues') throughout.
- Solution: No change. Every line with "Subissues" or "subissue" (the `## Subissues` heading, the Delivery Strategy sentence, the Workflow Checkpoints and the Acceptance Criteria items) is unchanged text of the repository template `docs/templates/EPIC.md`, which other EPIC specifications share. The prose written for this specification uses "sub-issue". Changing the template wording is outside this specification-only pull request.
- Current-tree verification: `grep -ci 'subissue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 14 (lines 178, 193, 220 to 225, 261 and 264 to 268), and `grep -qFx` finds each of those 14 lines unchanged in `docs/templates/EPIC.md`; `grep -c 'sub-issue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 11, none of them a template line. `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` has the same `## Subissues` heading.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463773
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463773

### F6 - Same terminology comment on the sub-issue drafts sentence

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421514
- Concern: Same text as F5, anchored on original line 185 of the reviewed commit.
- Solution: No separate change: same disposition as F5.
- Current-tree verification: Same as F5: `grep -ci 'subissue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 14 (lines 178, 193, 220 to 225, 261 and 264 to 268), and `grep -qFx` finds each of those 14 lines unchanged in `docs/templates/EPIC.md`; `grep -c 'sub-issue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 11, none of them a template line. `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` has the same `## Subissues` heading.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463980
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463980

### F7 - Same terminology comment on the Delivery Strategy sentence

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421527
- Concern: Same text as F5, anchored on line 193 ("For each subissue implementation in this EPIC, ...").
- Solution: No separate change: same disposition as F5. Line 193 is a template line.
- Current-tree verification: Same as F5: `grep -ci 'subissue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 14 (lines 178, 193, 220 to 225, 261 and 264 to 268), and `grep -qFx` finds each of those 14 lines unchanged in `docs/templates/EPIC.md`; `grep -c 'sub-issue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 11, none of them a template line. `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` has the same `## Subissues` heading.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658464189
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658464189

## Processing Log

- 2026-11-22 14:16 UTC - Started audit. Round 1: review 6012624473 (Copilot, COMMENTED, submitted 14:05:07Z) with seven inline comments, F1 to F7. Fetched the seven review threads with GraphQL as `josecelano-bot`: all unresolved, `viewerCanResolve` true for each. The fixes of F1 and F4 were already committed (committer times 14:09:00Z and 14:10:03Z) and on the pull request head. Checked each disposition against the current tree with the commands in the detail entries.
- 2026-11-22 14:17 UTC - Replied on the seven threads as `josecelano-bot` (reply `created_at` 14:17:20Z to 14:17:34Z) and recorded the reply URLs.
- 2026-11-22 14:18 UTC - A copy of the audit validator that calls `gh-bot` exits 0 (7 rows, 0 failures). After it, `github-review-threads reply-status` reported `all_replied` (7 of 7) and the seven threads were resolved as `josecelano-bot`. A new GraphQL fetch shows 7 threads, 0 unresolved. This record is not committed yet.

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
