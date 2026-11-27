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

Reviews processed: CodeRabbit 6079274055 (F1) and da2ce7 6087226203 (F2 to F4; APPROVED, no finding in its body). Review 6079228215 (claude[bot], fork notice) and CodeRabbit's summary comment contain no finding and have no row.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3082-f1 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3082-f2 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3082-f3 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3082-f4 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

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

### F2 - Evidence predates the guard move and names no artifact or stamps

- PR number: 3082
- Source review ID: 6087226203
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3082#discussion_r4718631429
- Concern: The T2 evidence section was committed before the guard moved into the `Tracker` fixture, named only the base, and had no step times.
- Solution: Reran both commands at the head, named the artifact under test by commit subject and stamped each step (11:12:35 to 11:12:50 UTC).
- Current-tree verification: Read the "T2: udp-server" section of the evidence file: artifact named by subject, stamps present, counts 230 and 17 passing, entries 0 and 67.
- Resolution reference: docs(issues): [#2925] rerun the udp-server evidence at the final head and name the converted test
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3082#discussion_r4718716936

### F3 - public_with_persistent_completed_statistics is the fixture constructor, not a test

- PR number: 3082
- Source review ID: 6087226203
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3082#discussion_r4718631443
- Concern: The evidence and the spec progress log called the `Tracker` constructor a test.
- Solution: Named the test `it_should_return_the_persisted_downloads_of_torrents_absent_from_memory_in_request_order` in the evidence file (run alone) and appended a correction entry to the spec progress log; the earlier entry is kept.
- Current-tree verification: `grep -n` in `packages/udp-server/src/handlers/scrape.rs` finds the constructor and the test as separate functions; read the evidence step 4 and the spec's correction entry.
- Resolution reference: docs(issues): [#2925] rerun the udp-server evidence at the final head and name the converted test
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3082#discussion_r4718717497

### F4 - Audit log reports a resolution before the push and resolution happened

- PR number: 3082
- Source review ID: 6087226203
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3082#discussion_r4718631452
- Concern: The 19:09 UTC log entry, carried by the audit commit, said the thread was resolved after the audit push, which had not happened when the entry was written.
- Solution: Appended an entry that corrects it, orders the events from GitHub data, names CodeRabbit's later reply (review 6079350735) and states that GitHub exposes no resolution time. The earlier entry is kept.
- Current-tree verification: Read the last Processing Log entry; reply and review times were taken from GitHub `created_at` and `submitted_at`.
- Resolution reference: docs(pr-reviews): [#2925] correct the resolution entry of the PR 3082 audit log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3082#discussion_r4718742973

## Processing Log

- 2026-11-26 19:09 UTC - Started audit for CodeRabbit review 6079274055 (one inline finding, F1; the review body has no other actionable assertion).
- 2026-11-26 19:09 UTC - The commits `test(udp-server): [#2925] keep the database guard in the fixture, after both service bundles` and `docs(issues): [#2925] correct where the udp-server guard lives` were pushed (`git ls-remote` equal to local HEAD) before the reply (19:09:30 UTC, GitHub `created_at`); the thread was resolved after the audit push. The audit record was created after the reply, so this start entry is late.
- 2026-11-27 11:17 UTC - Correction of the 19:09 UTC entry above, which said the thread "was resolved after the audit push" before that push and the resolution had happened. Order from the data: the commit `docs(pr-reviews): [#2925] add the audit record of the PR 3082 review round` was committed at 2026-11-26T19:09:48Z and pushed after that; the reply on the thread was created at 2026-11-26T19:09:30Z; CodeRabbit then posted its own reply on the same thread at 2026-11-26T19:10:42Z (review 6079350735, an automated reply with no new finding); the thread was resolved by this account after the push. GitHub's thread data exposes no resolution time, so none is stated; `isResolved` was first read as `true` in the refresh that followed. The earlier entry is kept, not rewritten.
- 2026-11-27 11:20 UTC - Round 2: da2ce7 review 6087226203 (APPROVED) has no finding in its body and three inline Minors (F2 to F4). The commit `docs(issues): [#2925] rerun the udp-server evidence at the final head and name the converted test` was pushed before the replies to F2 and F3; the commit `docs(pr-reviews): [#2925] correct the resolution entry of the PR 3082 audit log` was pushed before the reply to F4. Thread resolution follows the final audit push.

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
