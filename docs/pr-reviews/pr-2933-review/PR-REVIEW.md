---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2933 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2933.

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
| F1 | review-finding:pr-2933-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2933-f2 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F3 | review-finding:pr-2933-f3 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - archive checkpoint says the spec moved from closed/ to closed/

- PR number: 2933
- Source review ID: 5954125695
- Source URL: https://github.com/torrust/torrust-tracker/pull/2933#discussion_r4611903204
- Concern: The archive checkpoint in `ISSUE.md` read "from `docs/issues/closed/` to `docs/issues/closed/`", because the blanket replacement in the archive commit also rewrote the source folder. Suggested fix: restore `docs/issues/open/` as the first folder.
- Solution: Restored `docs/issues/open/` as the source folder on line 145, then swept the spec and the evidence file for other lines that describe history with a rewritten path; found one more, F3.
- Current-tree verification: `sed -n 145p docs/issues/closed/2931-nightly-uncommon-codepoints-zerocopy/ISSUE.md` on the pushed head shows "moved from `docs/issues/open/` to `docs/issues/closed/`".
- Resolution reference: docs(issues): [#2931] correct the archive paths in the spec checkpoints
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2933#discussion_r4612027478

### F2 - same closed-to-closed checkpoint, raised again

- PR number: 2933
- Source review ID: 5954222633
- Source URL: https://github.com/torrust/torrust-tracker/pull/2933#discussion_r4611970887
- Concern: Copilot reported the same line 145 as F1: the checkpoint says the spec moved from `docs/issues/closed/` to `docs/issues/closed/`.
- Solution: The fix for F1 changes the same line, so this is a re-raise of F1 with its own row, and it is fixed by the same commit.
- Current-tree verification: Same `sed -n 145p` check as F1, on the same pushed head.
- Resolution reference: docs(issues): [#2931] correct the archive paths in the spec checkpoints
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2933#discussion_r4612027957

### F3 - drafting checkpoint names closed/ as the drafting location

- PR number: 2933
- Source review ID: 5954222633
- Source URL: https://github.com/torrust/torrust-tracker/pull/2933#discussion_r4611970930
- Concern: Copilot reported that the checkpoint "Folder-style spec drafted in `docs/issues/closed/...`" reads as if the spec was originally drafted under `docs/issues/closed/`, when per the PR description it was drafted under `docs/issues/open/` and later archived, and asked to keep the historical drafting location.
- Solution: The checkpoint now names `docs/issues/drafts/nightly-uncommon-codepoints-zerocopy/ISSUE.md` and says it moved to `docs/issues/open/2931-nightly-uncommon-codepoints-zerocopy/` once issue #2931 existed. This change named a `drafts/` stage that no commit records, and a later review round (F4) replaced it by restoring the recorded `open/` path.
- Current-tree verification: `sed -n 135p docs/issues/closed/2931-nightly-uncommon-codepoints-zerocopy/ISSUE.md` on the pushed head shows the corrected text.
- Resolution reference: docs(issues): [#2931] correct the archive paths in the spec checkpoints
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2933#discussion_r4612028397

## Processing Log

- 2026-11-18 09:47 UTC - Review 5954125695 (`da2ce7`, state DISMISSED) submitted with one inline finding (F1).
- 2026-11-18 09:53 UTC - Review 5954222633 (Copilot, COMMENTED) submitted with two inline findings (F2, F3). The review poller watched Copilot only, so review 5954125695 was seen only when the threads were listed.
- 2026-11-18 09:54 UTC - Committed the fix, `docs(issues): [#2931] correct the archive paths in the spec checkpoints`, for all three findings. It was pushed before the replies below, which cite it.
- 2026-11-18 09:57 UTC - Replied on the three threads, then resolved them. The audit record was created after the replies, so this is a late start; it was created from the template and the replies' URLs recorded in the rows.

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
