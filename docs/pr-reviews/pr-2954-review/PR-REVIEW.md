---
pr-number: 2954
pr-url: https://github.com/torrust/torrust-tracker/pull/2954
last-updated-utc: "2026-11-18 19:35"
---

<!-- skill-link: process-pr-review -->

# PR #2954 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2954.

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

Round 1 is Copilot's review `5963601478` on the first head, with two inline findings that carry reviewer-provided IDs (F1 and F2). The review body only summarizes them and has no row of its own.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2954-f1` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2954-f2` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The spec exempted the changed fixture from the design review

- PR number: 2954
- Source review ID: 5963601478
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4619623059
- Concern: Task T3 changes a reusable asynchronous test fixture, so writing no new test function does not exempt the work from the design review that the `create-issue` skill requires; the Commit Points section said the review did not apply.
- Solution: The exemption is replaced by task T4, which records the prose-first Arrange-Act-Assert comparison and the fixture review after focused green validation and stops for maintainer review before final verification, commit or pull request. The Design and Ownership Review section names the post-vertical-slice checkpoint, and scenario M5 with evidence section V5 tracks it.
- Current-tree verification: `ISSUE.md` has task T4 "Design review of the fixture and test, then maintainer stop", the Commit Points paragraph describes the review and the stop, and the exemption sentence is gone (`grep -n "does not apply" ISSUE.md` prints nothing).
- Resolution reference: `docs(issues): [#2953] keep the design-review checkpoint for the changed fixture`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4619699253

### F2 - The plan had no maintained regression test for the wait helper

- PR number: 2954
- Source review ID: 5963601478
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4619622958
- Concern: The retained integration test passed all 400 plain runs with the faulty helper, and the probe is discarded, so the plan left no deterministic regression protection as the `fix-bug` skill requires.
- Solution: The Regression Test Strategy now plans a maintained test of the wait helper with no running listener: the database holds `1` while the in-memory counter is `0`, the helper must still be pending after a short timeout, and after the counter is incremented it must return. It is seen red against the unchanged helper (task T2, scenario M3) and green after the fix. AC6 states it. The integration test and the widened-window probe stay as the end-to-end check.
- Current-tree verification: `ISSUE.md` has task T2 "Add the wait-helper regression test (red)", AC6 and the rewritten Regression Test Strategy; `TestEnv::new` (no listeners), `torrent_metrics_store.save_global_downloads` and the public `Repository::increment_counter` exist on `develop`, so the test needs no production hook.
- Resolution reference: `docs(issues): [#2953] plan a maintained regression test for the wait helper`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4619699810

## Processing Log

- 2026-11-18 19:33 UTC - Started audit, before the replies of round 1.
- 2026-11-18 19:35 UTC - Round 1: Copilot review `5963601478` (submitted 19:28:54Z, poller `cursor`) with two Minor findings. The fixes were committed at 19:31:13Z (F2) and 19:32:26Z (F1) (`git log --format=%cI`); `git ls-remote` showed the fork branch at the local head at 19:35:07Z (`date -u`). The two replies were posted at 19:35:15Z and 19:35:18Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2954 --base torrust/develop`, run with the bot token in `GH_TOKEN` so its `gh` calls ran as `josecelano-bot`, exited 0 (2 rows, 0 failures) at 19:35:29Z; the two threads were then resolved by their thread IDs by 19:35:38Z, and a GraphQL refresh showed 0 unresolved threads.

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
