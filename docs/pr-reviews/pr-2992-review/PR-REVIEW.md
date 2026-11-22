---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2992 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2992.

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

da2ce7's review `6011641355` (`APPROVED` when submitted at 2026-11-22 10:12:09 UTC, on head `chore(issues): archive closed issue #2949 spec to docs/issues/closed and set EPIC row 24 to DONE`; `DISMISSED` since the round-1 push) has no inline comment and states "Findings: none", so it has no row.

Round 1 is Copilot review `6011656162` (`COMMENTED`, on the same head), with one inline comment, F1 (an audit-local ID). The comment carries no severity label; `Suggestion` is inferred from its wording ("consider formatting"). The review body says the full agentic review did not run because it did not start before the timeout, and raises nothing else.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | `review-finding:pr-2992-f1` | Copilot | Suggestion (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The section title in EPIC row 24 reads as part of the sentence

- PR number: 2992
- Source review ID: 6011656162
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2992#discussion_r4657672369
- Concern: EPIC row 24 named the section An ACK Names One Commit as plain text, so the title could read as part of the surrounding sentence.
- Solution: The row puts the title in quotes.
- Current-tree verification: `grep -c 'the section "An ACK Names One Commit"' docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md` prints 1.
- Resolution reference: `docs(issues): [#2949] quote the section title in EPIC row 24`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2992#discussion_r4657692217

## Processing Log

- 2026-11-22 10:23 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Started audit. The poller reported da2ce7's review 6011641355 (an approval with no finding) and their `ACK` on the first head (2026-11-22 10:12:12 UTC), then Copilot review 6011656162 (submitted 10:16:48 UTC). Round 1: normalized Copilot's inline comment as F1, fixed it in its own commit and pushed it, which made da2ce7's `ACK` stale; replied on the thread (10:23:18 UTC), then resolved it. Before writing this entry, listed every review and conversation comment of the pull request: none is left out.

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
