---
pr-number: 2983
pr-url: https://github.com/torrust/torrust-tracker/pull/2983
last-updated-utc: "2026-11-20 07:06"
---

<!-- skill-link: process-pr-review -->

# PR #2983 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2983.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the findings were processed before the merge).

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

The review 5988749356 of `da2ce7` (changes requested) has three inline findings, `F1` to `F3`; its
body lists the same three and adds no other request. The reviewer's own finding IDs are kept.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2983-f1 | Human | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2983-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2983-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The parent EPIC keeps the old path and `IN_PROGRESS` for order 5

- PR number: 2983
- Source review ID: 5988749356
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639466688
- Concern: The archive move left `docs/issues/open/2392-test-isolation/EPIC.md` naming the old path in `related-artifacts` and in subissue row 5, with `IN_PROGRESS`. The path names no tracked file after the merge, the frontmatter validator reports `missing-artifact`, and Steps 4 and 5 of `cleanup-completed-issues` ask for the EPIC update in the same change.
- Solution: `EPIC.md` names the `docs/issues/closed/...` path in both places, row 5 reads `DONE` with the pull requests of the remaining work, `last-updated-utc` is current and the progress log has an entry. The EPIC 2392 coordinator session, whose brief had excluded the file, agreed to the edit.
- Current-tree verification: `cargo run -q -p frontmatter-validator --bin frontmatter-validator -- docs/issues/open/2392-test-isolation/EPIC.md` printed no diagnostic; before the fix it printed one `missing-artifact` error. `grep -c 'closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md' docs/issues/open/2392-test-isolation/EPIC.md` printed 2.
- Resolution reference: `docs(issues): [#1419] mark order 5 done in EPIC #2392 and point it at the archived spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639495296

### F2 - The AC8 and AC9 rows narrow their scope without saying so

- PR number: 2983
- Source review ID: 5988749356
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639466695
- Concern: The closure review marked AC8 and AC9 as met for the eleven fixture-based targets only. "No suite starts the application another way" does not hold for `tests/`, where four targets start an executable as a child process, and the AC9 row cited no run of those four.
- Solution: The AC8 row says it is read for the fixture-based suites and names the four child-process targets as outside AC8 by D3 and `tests/AGENTS.md`. The AC9 row cites a run of those four, which passed, and says that all 15 main-level targets pass.
- Current-tree verification: `grep -c 'all 15 main-level' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1. `cargo test --test lifecycle-signals --test lifecycle-shutdown-policy --test cli-configuration --test http_health_check` exited with status 0 (`rustc 1.100.0`, `dev` profile).
- Resolution reference: `docs(issues): [#1419] state the scope of AC8 and AC9 in the closure review and drop two stale status remnants`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639495536

### F3 - Two stale status remnants in the archived record

- PR number: 2983
- Source review ID: 5988749356
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639466700
- Concern: The closed specification says every task is complete, but the historical plan still read `T9 | TODO` and a ticked checkpoint still said that cooperative server shutdown "remains deferred".
- Solution: T9 reads `DONE` and says it was superseded by R5, R8 and R9 of the completion plan. The checkpoint no longer has the "remains deferred" clause.
- Current-tree verification: `grep -c '^| T9  | DONE' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1 and `grep -c 'remains deferred)' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 0.
- Resolution reference: `docs(issues): [#1419] state the scope of AC8 and AC9 in the closure review and drop two stale status remnants`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639495789

## Processing Log

- 2026-11-20 07:02 UTC - The review 5988749356 of `da2ce7` (changes requested, submitted 07:02:23) arrived on the first head with F1 to F3.
- 2026-11-20 07:03 UTC - Fixed F2 and F3 in the working tree and asked the EPIC 2392 coordinator session whether to edit `EPIC.md`, which its brief had excluded. It agreed to the edit that F1 asks for.
- 2026-11-20 07:05 UTC - Committed the fix of F1 (commit time 07:05:12) and the fix of F2 and F3 (commit time 07:05:45) locally, not pushed yet.
- 2026-11-20 07:06 UTC - Replied on the three threads (all posted by 07:06:25). Each reply says its commit is local and goes up in one push with this record.
- 2026-11-20 07:06 UTC - Created this audit record with F1 to F3. One push of the three commits of round 1 and the resolution of the three threads follow this commit and are not logged.

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
