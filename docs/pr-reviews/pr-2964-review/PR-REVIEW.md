---
pr-number: 2964
pr-url: https://github.com/torrust/torrust-tracker/pull/2964
last-updated-utc: "2026-11-19 11:24"
---

<!-- skill-link: process-pr-review -->

# PR #2964 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2964.

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

Round 1 is da2ce7's review `5973114030` (APPROVED) on the first head. It left six inline findings with reviewer IDs F1 to F6, kept as the audit IDs. The review body summarizes them and has no row of its own.

Round 2 is Copilot's review `5973163140` on the first head, submitted after da2ce7's review and before the round-1 fixes were pushed. It left three inline findings without IDs or severity brackets, recorded as F7 to F9 (audit-local, severity inferred from each finding's effect if it were true).

Round 3 is Copilot's review `5974020256` on the head with the round-2 audit. It left one inline finding with reviewer ID F10, kept as the audit ID; Copilot labelled it "Low severity" and bracketed it `[Nit]`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2964-f1` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2964-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2964-f3` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2964-f4` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2964-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2964-f6` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2964-f7` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2964-f8` | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2964-f9` | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F10 | `review-finding:pr-2964-f10` | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - No test failed when the database clause of the wait was dropped

- PR number: 2964
- Source review ID: 5973114030
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627264943
- Concern: AC1 requires the helper to return only when both values reach `expected`, but with the database half of the condition removed, both helper tests and the integration test still passed, so that clause had no failing test (`write-unit-test`, "Prove a Test Guards Each Clause of a Rule").
- Solution: A third test, `it_should_keep_waiting_for_the_persisted_global_downloads_when_only_the_in_memory_counter_has_reached_the_expected_value`, sets the counter to 1 with no database value and expects the wait to still be pending. Evidence section V6 records one compiling mutation per clause; each fails exactly one test. The reviewer's `if persisted_in_memory {` mutation does not compile under `-D warnings`, which V6 also records.
- Current-tree verification: `grep -c "fn it_should_keep_waiting_for_the_persisted_global_downloads_when_only_the_in_memory_counter_has_reached_the_expected_value" packages/tracker-core/tests/common/test_env.rs` prints `1`; `manual-verification-evidence.md` has section V6 with the M-a and M-b results.
- Resolution reference: `test(tracker-core): [#2953] guard the database clause of the persisted downloads wait`; `docs(issues): [#2953] record the clause mutations and correct the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627354807

### F2 - V3's initial state named a commit the red runs predate

- PR number: 2964
- Source review ID: 5973114030
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627264952
- Concern: V3 said the red runs were made with the T2 commit, but they ran on the uncommitted test change with `HEAD` at `1f85267e8`; the commit came after.
- Solution: V3's initial state now says the runs used the uncommitted test change with `HEAD` at `1f85267e8`, committed unchanged right after under its subject.
- Current-tree verification: the V3 initial state in `manual-verification-evidence.md` says "with `HEAD` still at `1f85267e8` and the new tests as an uncommitted change".
- Resolution reference: `docs(issues): [#2953] record the clause mutations and correct the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627355266

### F3 - V4 credited the whole time difference to the probe

- PR number: 2964
- Source review ID: 5973114030
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627264964
- Concern: V4 said the 0.37 s difference between the passing and failing runs was the probe's delay, but the passing run also pays the restarted environment's 100 ms listener-start sleep and up to one 50 ms poll.
- Solution: V4's conclusion now names all three parts: the probe's 300 ms, the 100 ms listener-start sleep of the restarted `TestEnv::started`, and up to one 50 ms poll.
- Current-tree verification: V4's conclusion contains "100 ms listener-start sleep"; `test_env.rs` has the 100 ms sleep in `run_jobs`.
- Resolution reference: `docs(issues): [#2953] record the clause mutations and correct the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627355686

### F4 - T6 and the workflow checkpoints disagreed

- PR number: 2964
- Source review ID: 5973114030
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627264973
- Concern: T6 was DONE and the automatic-verification checkpoint ticked while the progress log said the pre-push hook would run on the push, and the acceptance-review checkpoint stayed unticked although the Acceptance Verification table was all DONE.
- Solution: The pre-push hook passed on the 09:29:53 to 09:32:32 UTC push (all 7 steps pass); a new progress-log entry records that and the acceptance review, and the "Acceptance criteria reviewed after implementation" checkpoint is ticked. The earlier entry is unchanged.
- Current-tree verification: `ISSUE.md` has `- [x] Acceptance criteria reviewed after implementation and updated with evidence` and a progress-log entry starting with the round-1 stamp that names the pre-push run.
- Resolution reference: `docs(issues): [#2953] record the clause mutations and correct the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627356089

### F5 - The fix commit's test comment was not described

- PR number: 2964
- Source review ID: 5973114030
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627264981
- Concern: The fix commit also adds the one-line Arrange comment to the first new test, which neither its message nor the PR description mentioned.
- Solution: The PR description's Commits entry for the fix commit now names the comment and where it came from (V5). History is not rewritten, so the commit message stays. The fix is outside the tree, so the reply is the resolution reference.
- Current-tree verification: `gh-bot pr view 2964 --json body` shows "a one-line Arrange comment in the first new test" (edited at 10:26:04Z).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627356521
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627356521

### F6 - The finishing test shared the short bound

- PR number: 2964
- Source review ID: 5973114030
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627264988
- Concern: Both tests shared one 500 ms bound, which the pending test always spends but which could fail the finishing test on a loaded host if the first SQLite read stalls.
- Solution: The pending tests use `STILL_PENDING_AFTER` (500 ms), and the finishing test uses `FINISHES_WITHIN` (4 s), below the helper's own 5 s so this test's message is the one reported.
- Current-tree verification: `test_env.rs` defines `STILL_PENDING_AFTER` and `FINISHES_WITHIN`; `grep -c WAIT_LIMIT` prints `0`.
- Resolution reference: `test(tracker-core): [#2953] guard the database clause of the persisted downloads wait`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627356926

### F7 - A database error was treated as "not persisted yet"

- PR number: 2964
- Source review ID: 5973163140
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627303502
- Concern: The helper's `matches!` treated an `Err` from `load_global_downloads()` like a missing value, so a database failure surfaced only as the 5-second timeout, without its cause.
- Solution: The helper calls `expect` on the load result, so a database error fails the test at once with the error. The tests, the clause mutations, the probe and the plain runs were repeated on this helper (evidence V7).
- Current-tree verification: `test_env.rs` has `.expect("loading the global downloads from the database should not fail")` in `wait_for_global_downloads_persisted`; `manual-verification-evidence.md` has section V7.
- Resolution reference: `test(tracker-core): [#2953] fail the persisted downloads wait at once on a database error`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627461295

### F8 - The access chain was repeated on every poll

- PR number: 2964
- Source review ID: 5973163140
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627303553
- Concern: Each poll iteration repeated the container and persistence access chain, with its `expect`, which made the loop harder to read.
- Solution: The helper looks up `torrent_metrics_store` and `stats_repository` once, before the loop; each iteration only loads the two values.
- Current-tree verification: in `wait_for_global_downloads_persisted`, `let torrent_metrics_store = &self` and `let stats_repository = &self.tracker_core_container.stats_repository;` come before `tokio::time::timeout`.
- Resolution reference: `test(tracker-core): [#2953] fail the persisted downloads wait at once on a database error`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627461804

### F9 - The brace of a long test header was said to be misplaced

- PR number: 2964
- Source review ID: 5973163140
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627303605
- Concern: The opening brace of `it_should_finish_waiting_..._have_reached_the_expected_value` sits on its own line, which was said to be inconsistent with standard Rust formatting.
- Solution: No change. That layout is what nightly rustfmt produces for a function header longer than the line limit, and the repository's format check passes.
- Current-tree verification: `cargo +nightly fmt --all -- --check` exits 0 on this head; the pre-push hook's formatting step passed on the push at 10:36:06Z.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627462289
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627462289

### F10 - V7's probe-removal check did not name the file it covered

- PR number: 2964
- Source review ID: 5974020256
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4627949098
- Concern: V7 recorded `git diff --quiet HEAD` as exiting 0 while the helper change was uncommitted, which an unscoped diff would not do.
- Solution: The script that ran V7 limited the check to `handler.rs` (`git diff --quiet HEAD -- packages/tracker-core/src/statistics/event/handler.rs`, after `git checkout --` of that file), as V4 did; V7 now records that command and why it is limited to that file.
- Current-tree verification: V7 step 3 in `manual-verification-evidence.md` names `git diff --quiet HEAD -- packages/tracker-core/src/statistics/event/handler.rs`; `grep -c 'git diff --quiet HEAD`' manual-verification-evidence.md` prints `0`.
- Resolution reference: `docs(issues): [#2953] name the file the V7 probe-removal check covers`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2964#discussion_r4628002681

## Processing Log

- 2026-11-19 10:27 UTC - Started audit. Round 1: da2ce7 review `5973114030` (APPROVED, submitted 10:18:13Z, poller `cursor`) on the first head, with six inline findings. All CI checks on that head passed. The two fix commits were made at 10:19:58Z and 10:22:24Z (`git log --format=%cI`); `git ls-remote` showed the fork branch at the local head at 10:25:49Z (`date -u`). The PR description was edited for F5 at 10:26:04Z, and the six replies were posted from 10:26:17Z to 10:26:29Z (forge `created_at`), after the push.
- 2026-11-19 10:36 UTC - Round 1, continued: `validate-audit-record.py --pr-number 2964 --base torrust/develop`, with the bot token in `GH_TOKEN`, exited 0 (6 rows, 0 failures) at 10:27:26Z. The resolve step then resolved every unresolved thread by 10:27:45Z, nine instead of six: it also resolved the three threads of Copilot review `5973163140`, which had arrived at 10:21:34Z, before they had a reply or an audit row. Those three were unresolved again at 10:28:02Z, then read and processed as round 2.
- 2026-11-19 10:36 UTC - Round 2: Copilot review `5973163140` (submitted 10:21:35Z) on the first head, before the round-1 push, with three inline findings (F7 to F9). The fix was committed at 10:31:23Z and the V7 evidence at 10:32:23Z (`git log --format=%cI`); `git ls-remote` showed the fork branch at the local head at 10:36:06Z (`date -u`). The three replies were posted at 10:36:12Z, 10:36:15Z and 10:36:18Z (forge `created_at`), after the push.
- 2026-11-19 10:37 UTC - Round 2, continued: `validate-audit-record.py --pr-number 2964 --base torrust/develop`, with the bot token in `GH_TOKEN`, exited 0 (9 rows, 0 failures) at 10:37:12Z; the three Copilot threads were then resolved by thread ID, matched by their root comment IDs, by 10:37:17Z, and a GraphQL refresh showed 0 unresolved threads.
- 2026-11-19 11:24 UTC - Round 3: Copilot review `5974020256` (submitted 11:18:32Z, poller `cursor`) on the head with the round-2 audit, with one inline finding (F10). The fix was committed at 11:20:11Z (`git log --format=%cI`); `git ls-remote` showed the fork branch at the local head at 11:23:42Z (`date -u`). The reply was posted at 11:23:43Z (forge `created_at`), after the push. `validate-audit-record.py --pr-number 2964 --base torrust/develop`, with the bot token in `GH_TOKEN`, exited 0 (10 rows, 0 failures) at 11:24:05Z. A GraphQL query then listed only that thread as unresolved; it was resolved by 11:24:14Z, and a refresh showed 0 unresolved threads.

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
