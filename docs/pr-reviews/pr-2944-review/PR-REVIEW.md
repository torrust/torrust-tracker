---
pr-number: 2944
pr-url: https://github.com/torrust/torrust-tracker/pull/2944
last-updated-utc: "2026-11-18 13:54"
---

<!-- skill-link: process-pr-review -->

# PR #2944 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2944.

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

- Round 1: Copilot review 5957896114 (COMMENTED) with two findings without reviewer IDs, as audit findings F1 and F2.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2944-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2944-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The AC4 evidence counts G2 as isolated and then says it is not

- PR number: 2944
- Source review ID: 5957896114
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4614977060
- Concern: The AC4 evidence row of the EPIC summarizes the inventory as 29 isolated findings and then adds that G2, one of them, is not isolated, so the row contradicts itself.
- Solution: The row now counts G2 once, under "tracked by a subissue" (order 11), with 28 isolated findings, and says that the inventory itself still counts G2 as isolated until order 11 corrects its row.
- Current-tree verification: `grep -n 'isolated (28)' docs/issues/open/2392-test-isolation/EPIC.md` finds the AC4 row, and `grep -c 'counted here as isolated' docs/issues/open/2392-test-isolation/EPIC.md` prints 0.
- Resolution reference: docs(issues): [#2943] count G2 once in the AC4 evidence and cite the edition guide
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615054529

### F2 - The claim about `unsafe` environment functions has no reference or version

- PR number: 2944
- Source review ID: 5957896114
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4614977134
- Concern: The #2943 spec says that the 2024 edition marks `std::env::set_var` and `std::env::remove_var` as `unsafe`, without a reference or a Rust version that a reader can check.
- Solution: The sentence now names the version (2024 edition, stable since Rust 1.85) and links the "Newly unsafe functions" page of the Rust Edition Guide.
- Current-tree verification: `grep -n 'newly-unsafe-functions\|stable since Rust 1.85' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds both in the Background section.
- Resolution reference: docs(issues): [#2943] count G2 once in the AC4 evidence and cite the edition guide
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615055028

## Processing Log

- 2026-11-18 13:54 UTC - Round 1: Copilot review 5957896114 with F1 and F2. Committed and pushed the fix (`docs(issues): [#2943] count G2 once in the AC4 evidence and cite the edition guide`), then replied to the two threads (13:53:25Z and 13:53:27Z).
- 2026-11-18 13:54 UTC - The validator on the worktree record reported 2 rows, 1 log entry, 0 failures. Resolved the two round 1 threads (about 13:54:38Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Committed these entries.

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
