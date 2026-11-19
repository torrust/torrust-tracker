---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2968 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2968.

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

F1 and F2 come from da2ce7's review 5974841608 (APPROVED on the first head, with an ACK comment);
the reviewer's IDs are kept. Copilot's review 5974816587 on the same head had no findings and gets
no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2968-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2968-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The compatibility claim does not hold for the two peer-IP variants

- PR number: 2968
- Source review ID: 5974841608
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2968#discussion_r4628578089
- Concern: The T6 notes of the evidence, and the PR description, said every reworded message keeps the old leading words, so a client or test matching the start of the old text still matches. That holds for the two `MissingParams` and the two `auth::Error` texts, but the new peer-IP texts share only "missing or invalid" and "cannot get the client IP" with the old ones, which is why five handler tests and a contract-test helper changed.
- Solution: The T6 note now says which four texts keep the old leading words and names the two `PeerIpResolutionError` texts as the exceptions, with the tests and the helper that changed with them. The PR description's "keeping their leading words" sentence is corrected the same way.
- Current-tree verification: `grep -rn "leading words" docs/issues/open/2915-2912-si-3-http-tracker-request-errors/` finds only the corrected note, which names the `PeerIpResolutionError` exception; the old peer-IP starts ("the right most X-Forwarded-For IP", "from the connection info") appear in no source file (`grep -rn` over `packages` finds only doc comments unrelated to the messages).
- Resolution reference: docs(issues): [#2915] name the peer-IP messages as exceptions to the kept leading words
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2968#discussion_r4628679002

### F2 - The stated commit time disagrees with the commit

- PR number: 2968
- Source review ID: 5974841608
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2968#discussion_r4628578101
- Concern: The evidence said the red-test commit was made at 11:28:51, but the commit records 11:27:44 as its author and committer date.
- Solution: 11:28:51 was the `date -u` run after the `git commit` command returned, its pre-commit hook having run in between. The evidence now gives 11:27:44 as the commit's recorded date and 11:28:51 as when the command returned. The sweep for other stamps found one loose wording on the same red run ("`git diff --quiet HEAD` exited 0 at 11:28:57", where the check ran just before the run's first stamp), now corrected. The other stamps were checked against `git log --format=%cI`: the "after" build at 11:34 and the green run from 11:34:24 follow the final code commit at 11:33:34.
- Current-tree verification: `git log -1 --format=%cI` of `test(http-protocol): [#2915] add red tests that HTTP tracker failure reasons name no source path` prints `2026-11-19T11:27:44Z`, the time the evidence now gives.
- Resolution reference: docs(issues): [#2915] give the red-test commit time that git records
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2968#discussion_r4628679457

## Processing Log

- 2026-11-19 12:22 UTC - Started audit for da2ce7's review 5974841608 (APPROVED, submitted 12:18:24Z on the first head, "docs(issues): [#2915] record the reproduction, red and green runs and the wording table", with an ACK comment at 12:18:28Z). Two findings, F1 (Minor) and F2 (Nit), with the reviewer's IDs. Fixes committed in "docs(issues): [#2915] name the peer-IP messages as exceptions to the kept leading words" (F1) and "docs(issues): [#2915] give the red-test commit time that git records" (F2). Not pushed yet; the PR description edit, the replies and thread resolution follow the push, and the Reply URL and Thread state fields are updated then.
- 2026-11-19 12:26 UTC - Pushed "docs(pr-reviews): [#2915] add the review audit of PR #2968" as the new head after the commit-id check printed nothing (pre-push hook passed), and edited the PR description for F1. Replied on F1 and F2 (replies created 12:26:10Z and 12:26:12Z), recorded the reply URLs, ran `validate-audit-record.py --pr-number 2968 --base torrust/develop` through a scratch copy that calls `gh-bot` (status ok, 2 rows, 0 failures), then resolved both threads by 12:26:41 UTC (a `date -u` run after the resolve commands). GraphQL refresh: zero unresolved threads on PR #2968.

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
