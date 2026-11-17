---
pr-number: 2929
pr-url: https://github.com/torrust/torrust-tracker/pull/2929
last-updated-utc: "2026-11-17 21:36"
---

<!-- skill-link: process-pr-review -->

# PR #2929 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2929.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

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

Round 1: Copilot review 5946976502 (COMMENTED, a lite review, since its full agentic review did not start before the timeout) with one inline finding, rated medium by Copilot; this audit maps that to `Minor (inferred)`, since the comment carries no severity bracket. The `F` ID is this audit's.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2929-f1 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Unquoted `last-updated-utc` in the evidence frontmatter

- PR number: 2929
- Source review ID: 5946976502
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606077014
- Concern: The archived evidence file's `last-updated-utc` was unquoted, while the spec and the EPIC quote it; YAML can parse an unquoted timestamp as a datetime instead of a string.
- Solution: Quoted the value, as the other frontmatter in this PR does. The value was already unquoted before the archive move; this PR only changed it.
- Current-tree verification: `head -5` of `docs/issues/closed/2847-1488-si-28-verify-hung-component-abort/manual-verification-evidence.md` shows `last-updated-utc: "2026-11-17 21:35"`; the pre-commit staged-frontmatter validator passed.
- Resolution reference: `docs(issues): [#2847] quote the evidence file's last-updated-utc timestamp`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2929#discussion_r4606101763

## Processing Log

- 2026-11-17 21:36 UTC - Round 1: Copilot review 5946976502 (COMMENTED, 21:33:38Z) with finding F1. Fixed it, following the maintainer's choice for the non-blocking findings of PR #2911: committed the fix, then, before pushing so that one push carries the fix and this record, replied to the thread (21:35:59Z) naming the commit subject the push brings, and resolved it (21:36:07Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Created this record.

<!-- Copied verbatim into each audit record. -->

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
