# PR #3082 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3082.

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

Reviews processed: CodeRabbit 6079274055 (F1). Review 6079228215 (claude[bot], fork notice) and CodeRabbit's summary comment contain no finding and have no row.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3082-f1 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Database guard drops before the UDP services that use the database

- PR number: 3082
- Source review ID: 6079274055
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3082#discussion_r4712466109
- Concern: The guard sat inside `CoreTrackerServices`, which a fixture drops before `CoreUdpTrackerServices`, and the latter still holds the database; on a platform that cannot delete open files the guard's removal of the directory would fail.
- Solution: Moved the guard out of `CoreTrackerServices` into the `Tracker` fixture of `packages/udp-server/src/handlers/scrape.rs` as its last field, declared after both service bundles, and corrected the spec and its progress log. The PR description was edited to match.
- Current-tree verification: Read the `Tracker` struct: `_sqlite_database_guard` is declared after `core_tracker_services` and `core_udp_tracker_services`; `CoreTrackerServices` in `handlers/mod.rs` has no guard field.
- Resolution reference: test(udp-server): [#2925] keep the database guard in the fixture, after both service bundles
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3082#discussion_r4712524802

## Processing Log

- 2026-11-26 19:09 UTC - Started audit for CodeRabbit review 6079274055 (one inline finding, F1; the review body has no other actionable assertion).
- 2026-11-26 19:09 UTC - The commits `test(udp-server): [#2925] keep the database guard in the fixture, after both service bundles` and `docs(issues): [#2925] correct where the udp-server guard lives` were pushed (`git ls-remote` equal to local HEAD) before the reply (19:09:30 UTC, GitHub `created_at`); the thread was resolved after the audit push. The audit record was created after the reply, so this start entry is late.

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
