---
pr-number: 2954
pr-url: https://github.com/torrust/torrust-tracker/pull/2954
last-updated-utc: "2026-11-19 08:19"
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

Round 2 is Copilot's review `5964198269` on the head that carried the round-1 fixes. It ran with the "didn't start before the timeout" warning and left two inline findings without IDs or severity brackets (F3 and F4, audit-local, severity inferred). Copilot labelled both "Medium severity".

Round 3 is Copilot's review `5964694968` on the head with the round-2 audit, again with the timeout warning, and one inline finding without an ID or severity bracket (F5, audit-local, severity inferred). Copilot labelled it "Medium severity".

Basis of the inferred severities: Copilot's Low/Medium labels are not the audit's five-level scale, and the round-1 findings labelled "Low severity" already carried Copilot's own `[Minor]` brackets, so the labels do not map one-to-one. Each inferred severity therefore rests on the finding's effect if it were true: F3 and F4 claim a table-rendering defect with no effect on meaning (`Nit`), and F5 is a wording inconsistency that could change how the helper's comparison is implemented (`Minor`).

Round 4 is da2ce7's review `5970974561` (APPROVED) on the head with the round-3 fix. It left four inline findings with reviewer IDs F1 to F4, which collide with this audit's IDs, so they are recorded as F6 to F9 here; the reviewer IDs stay in each detail. The review body summarizes them and has no row of its own.

Round 5 is da2ce7's review `5971136740` (APPROVED) on the head with the round-4 audit. It left one inline finding with reviewer ID F7, recorded as F10 here.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2954-f1` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2954-f2` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2954-f3` | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | `review-finding:pr-2954-f4` | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F5 | `review-finding:pr-2954-f5` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2954-f6` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2954-f7` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2954-f8` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2954-f9` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2954-f10` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

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

### F3 - Tables in the spec were said to start with `||`

- PR number: 2954
- Source review ID: 5964198269
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4620088928
- Concern: Rows of the Implementation Plan, Commit Points, Manual Verification Scenarios and Acceptance Verification tables in `ISSUE.md` were said to start with `||`, which would render an empty first column.
- Solution: No change. Every table row starts with a single `|`; the leading `+` of the diff view was likely read as a second pipe.
- Current-tree verification: `git grep -c -E '^\|\|'` on the reviewed head over the issue folder and this audit found no match (exit 1); line 182 of `ISSUE.md`, where the comment is anchored, starts with `| T6  |`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4620120360
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4620120360

### F4 - Tables in this audit were said to start with `||`

- PR number: 2954
- Source review ID: 5964198269
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4620088993
- Concern: Rows of the Findings table in this audit were said to start with `||`, which would render an empty first column.
- Solution: No change. Every table row starts with a single `|`; the leading `+` of the diff view was likely read as a second pipe.
- Current-tree verification: the same `git grep` found no match; line 51 of this audit at the reviewed head, where the comment is anchored, starts with `| F2 |`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4620121079
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4620121079

### F5 - The spec mixed "at least" and "reach" for the helper's comparison

- PR number: 2954
- Source review ID: 5964694968
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4620476829
- Concern: AC1 says the helper returns when both values are at least `expected`, while the In Scope item said they "reach the expected value", which can read as equality; the comparison should be stated the same way everywhere.
- Solution: The In Scope item now says "at least the expected value (`>= expected`)" and gives the reason: the helper already compares the database value with `>=`, and a caller waiting for `1` after more downloads were persisted must not hang until the timeout. The Goal and the Regression Test Strategy state no comparison.
- Current-tree verification: `grep -n -i "reach" ISSUE.md` prints nothing; the In Scope item and AC1 both say "at least"; `test_env.rs` on `develop` has `u64::from(downloads) >= expected`.
- Resolution reference: `docs(issues): [#2953] state the at-least comparison of the wait helper in scope`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4620545071

### F6 - The References section named a test that does not exist

- PR number: 2954
- Source review ID: 5970974561
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625618483
- Concern: The References entry pointed to an `it_should_restore_*` test, but no test in `packages/tracker-core/tests/integration.rs` has that name, so the reference did not resolve.
- Solution: The entry now names the `restored` timeout loop of `it_should_persist_the_number_of_completed_peers_for_each_torrent_into_the_database`, the test above the affected one.
- Current-tree verification: `grep -c it_should_restore ISSUE.md` prints `0`; `grep -n "async fn it_should_persist_the_number_of_completed_peers_for_each_torrent_into_the_database" packages/tracker-core/tests/integration.rs` matches line 177, and its body holds `let restored = tokio::time::timeout(`.
- Resolution reference: `docs(issues): [#2953] name the test with the similar bounded-retry loop`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625668223

### F7 - The spec dates and Progress Log stopped before the review-round changes

- PR number: 2954
- Source review ID: 5970974561
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625618490
- Concern: The `last-updated-utc` of `ISSUE.md` (18:50) and of the evidence file (17:41) predated later commits that changed them, and the Progress Log ended at the approval, so it read as if the approved text were the one that merges.
- Solution: Both `last-updated-utc` values were set from `date -u` when edited, and a Progress Log entry lists the review-round changes (T2, T4, AC6, M5, the `>= expected` In Scope sentence and the renamed test reference).
- Current-tree verification: `grep -n last-updated-utc` over the issue folder shows `2026-11-19 07:31` in both files; the last Progress Log entry is dated `2026-11-19 07:31 UTC`.
- Resolution reference: `docs(issues): [#2953] record the review-round spec changes in the dates and progress log`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625668516

### F8 - The audit did not say what the inferred severities rest on

- PR number: 2954
- Source review ID: 5970974561
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625618495
- Concern: Copilot labelled F3, F4 and F5 "Medium severity", yet the audit recorded `Nit (inferred)` for F3 and F4 and `Minor (inferred)` for F5 without saying why, so aggregated severities were not comparable.
- Solution: The Findings section now records Copilot's labels for rounds 2 and 3 and states the basis: the Low/Medium labels do not map one-to-one onto the five-level scale (round 1's "Low severity" findings carried `[Minor]` brackets), so each inferred severity rests on the finding's effect if it were true.
- Current-tree verification: the Findings section has the paragraph starting "Basis of the inferred severities", and the round 2 and round 3 paragraphs quote "Medium severity".
- Resolution reference: `docs(pr-reviews): [#2953] state the basis of the inferred severities`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625668847

### F9 - The PR description covered audit round 1 only

- PR number: 2954
- Source review ID: 5970974561
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625618501
- Concern: The description's Files entry for this audit described round 1 only, while the merge tool copies the description into the merge commit, which cannot be edited afterwards.
- Solution: The description's Files entry now covers all four rounds (F1 to F9). The fix is outside the repository tree, so the reply is the resolution reference.
- Current-tree verification: `gh-bot pr view 2954 --json body` shows the entry with "Round 4 (da2ce7 review 5970974561, approved)" (checked at 07:37Z).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625669117
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625669117

### F10 - The round-4 log entry was stamped before the events it reports

- PR number: 2954
- Source review ID: 5971136740
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625748861
- Concern: The round-4 Processing Log entry and `last-updated-utc` said `07:37`, but the entry reports the validator run at 07:38:07Z and the resolutions by 07:38:19Z, while rounds 1 to 3 carry the minute of their last reported event.
- Solution: The Processing Log has an appended correction entry, stamped `2026-11-19 08:19 UTC`, which states that the round-4 entry's last reported event is at 07:38:19Z, after its `07:37` stamp. The round-4 entry keeps its original stamp, because log entries are append-only (F11).
- Current-tree verification: the Processing Log has the line starting `- 2026-11-19 07:37 UTC - Round 4:` and the correction entry starting `- 2026-11-19 08:19 UTC - Correction:`, which names the round-4 entry.
- Resolution reference: `docs(pr-reviews): [#2953] stamp the round-4 log entry at its last event`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2954#discussion_r4625786754

## Processing Log

- 2026-11-18 19:33 UTC - Started audit, before the replies of round 1.
- 2026-11-18 19:35 UTC - Round 1: Copilot review `5963601478` (submitted 19:28:54Z, poller `cursor`) with two Minor findings. The fixes were committed at 19:31:13Z (F2) and 19:32:26Z (F1) (`git log --format=%cI`); `git ls-remote` showed the fork branch at the local head at 19:35:07Z (`date -u`). The two replies were posted at 19:35:15Z and 19:35:18Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2954 --base torrust/develop`, run with the bot token in `GH_TOKEN` so its `gh` calls ran as `josecelano-bot`, exited 0 (2 rows, 0 failures) at 19:35:29Z; the two threads were then resolved by their thread IDs by 19:35:38Z, and a GraphQL refresh showed 0 unresolved threads.
- 2026-11-18 20:11 UTC - Round 2: Copilot review `5964198269` (submitted 20:08:41Z, poller `cursor`) on the head with the round-1 fixes, with the "didn't start before the timeout" warning and two inline findings without IDs. Both say table rows start with `||`; `git grep -c -E '^\|\|'` on the reviewed head found no match, so F3 and F4 are `NO_ACTION`. The prescribed `Superseded by F3: ...` and `Superseded by F4: ...` replies were posted at 20:10:52Z and 20:10:54Z and edited at 20:11:04Z and 20:11:06Z (forge `created_at` and `updated_at`) to drop a PR-branch commit id they cited. `validate-audit-record.py --pr-number 2954 --base torrust/develop`, with the bot token in `GH_TOKEN`, exited 0 (4 rows, 0 failures) at 20:11:25Z; the two threads were then resolved by 20:11:33Z, and a GraphQL refresh showed 0 unresolved threads. No spec change was needed in this round.
- 2026-11-18 20:46 UTC - Round 3: Copilot review `5964694968` (submitted 20:40:20Z, poller `cursor`) on the head with the round-2 audit, with the timeout warning and one inline finding (F5). The fix was committed at 20:42:45Z (`git log --format=%cI`); `git ls-remote` showed the fork branch at the local head at 20:45:36Z (`date -u`). The reply was posted at 20:45:43Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2954 --base torrust/develop`, with the bot token in `GH_TOKEN`, exited 0 (5 rows, 0 failures) at 20:46:01Z; the thread was then resolved by 20:46:08Z, and a GraphQL refresh showed 0 unresolved threads.
- 2026-11-19 07:37 UTC - Round 4: da2ce7 review `5970974561` (APPROVED, submitted 07:30:54Z, poller `cursor`) on the head with the round-3 fix, with four inline findings recorded as F6 to F9. The pre-commit gate exited 0 before the three fix commits; the commits were pushed and `git ls-remote` showed the fork branch at the local head at 07:37:07Z (`date -u`). The PR description was edited for F9 at 07:37Z, and the four replies were posted at 07:37:19Z, 07:37:22Z, 07:37:24Z and 07:37:26Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2954 --base torrust/develop`, with the bot token in `GH_TOKEN`, exited 0 (9 rows, 0 failures) at 07:38:07Z; the four threads were then resolved by 07:38:19Z, and a GraphQL refresh showed 0 unresolved threads.
- 2026-11-19 07:52 UTC - Round 5: da2ce7 review `5971136740` (APPROVED, submitted 07:47:13Z, poller `cursor`) on the head with the round-4 audit, with one inline finding recorded as F10. The fix was committed at 07:48:41Z (`git log --format=%cI`); `git ls-remote` showed the fork branch at the local head at 07:51:51Z (`date -u`). The reply was posted at 07:51:58Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2954 --base torrust/develop`, with the bot token in `GH_TOKEN`, exited 0 (10 rows, 0 failures) at 07:52:24Z; the thread was then resolved by 07:52:33Z, and a GraphQL refresh showed 0 unresolved threads.
- 2026-11-19 08:19 UTC - Correction: the round-4 entry above is stamped `07:37`, but the last event it reports, the resolution of the four threads, happened by 07:38:19Z. The commit `docs(pr-reviews): [#2953] stamp the round-4 log entry at its last event` changed that stamp to `07:38` in place, which breaks the append-only rule. The original `07:37` stamp is now restored, and this entry records the correction instead (F10, F11).

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
