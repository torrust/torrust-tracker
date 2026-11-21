---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2989 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2989.

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

Round 1 is Copilot review `6006218967` (`COMMENTED`, on head `docs(issues): [#2949] record the implementation progress and the manual verification evidence`), with one inline comment, F1 (an audit-local ID). The comment carries no severity label, so `Nit` is inferred from its prose: a formatting defect in one list item. The review body says the full agentic review did not run because it did not start before the timeout, and raises nothing else.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | `review-finding:pr-2989-f1` | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A continuation line of the evidence list is not indented

- PR number: 2989
- Source review ID: 6006218967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653326417
- Concern: In `manual-verification-evidence.md`, the "Artifact under test" item wrapped a commit subject across lines, and the last line was not indented as part of the list item.
- Solution: The commit subject is on one line of its own, indented under the item, so the whole entry is one list item.
- Current-tree verification: `sed -n 17,19p docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/manual-verification-evidence.md` prints the item with both continuation lines indented by two spaces.
- Resolution reference: `docs(issues): [#2949] keep the artifact line of the evidence inside its list item`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2989#discussion_r4653358233

## Processing Log

- 2026-11-21 15:20 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - Started audit. Round 1: the poller reported Copilot review 6006218967 (submitted 2026-11-21 15:12:33 UTC). Normalized its inline comment as F1, fixed it in its own commit, pushed it with the commit that records this pull request in the specification, replied on the thread (15:20:30 UTC), then resolved the thread.

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
