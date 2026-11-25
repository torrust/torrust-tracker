---
pr-number: 3043
pr-url: https://github.com/torrust/torrust-tracker/pull/3043
last-updated-utc: "2026-11-25 08:55"
---

<!-- skill-link: process-pr-review -->

# PR #3043 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3043.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the findings were fixed before the merge).

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

Round 1, on the head `docs(issues): [#3031] record the manual verification evidence and complete the specification`: Greptile review 6044956173 (two threads, P2 badges) and CodeRabbit review 6044982742 (one thread). All three were fixed in one commit.

Round 2, on the head `docs(pr-reviews): [#3031] record the audit of PR 3043 review round 1`: da2ce7 review 6054010185 (approved, six inline threads, F1 to F3 Minor and F4 to F6 Nit in the reviewer's own numbering). The reviewer's IDs collide with the audit IDs F1 to F3 above, so the rows below are F4 to F9 and the detail entries name the reviewer's ID.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3043-f1 | Unknown | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3043-f2 | Unknown | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3043-f3 | Unknown | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3043-f4 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3043-f5 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3043-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3043-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3043-f8 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3043-f9 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The log capture helper writes `std::sync::Arc` in full

- PR number: 3043
- Source review ID: 6044956173
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684595423
- Concern: `packages/test-helpers/src/logging.rs` used `std::sync::Arc` three times without a name conflict, against the import rule of `AGENTS.md`.
- Solution: `Arc` is imported next to `Mutex` and used by its short name.
- Current-tree verification: `grep -n "std::sync::Arc" packages/test-helpers/src/logging.rs` prints nothing; `cargo clippy -p torrust-tracker-test-helpers --all-targets` printed nothing.
- Resolution reference: fix(statistics): [#3031] address review round 1 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684667777

### F2 - The tracker-level test ignores the exit status

- PR number: 3043
- Source review ID: 6044956173
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684595444
- Concern: `wait_for_exit()` returns `Ok` for a non-zero exit or a signal death, so a tracker that crashed before its listeners finished could pass the test with incomplete logs.
- Solution: The test asserts `exit.status.success()` before it reads the logs and prints the tracker output when it fails.
- Current-tree verification: `cargo test --test metrics-statistics-disabled-logs` passed (27 tests, the new one included) after the change.
- Resolution reference: fix(statistics): [#3031] address review round 1 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684668271

### F3 - The evidence file names a spec path that goes stale

- PR number: 3043
- Source review ID: 6044982742
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684617214
- Concern: `manual-verification-evidence.md` had `issue-spec: <path under docs/issues/open/>`, which is wrong once the specification moves to `closed/`; `write-markdown-docs` asks for `issue: #<number>` after the GitHub issue exists.
- Solution: The frontmatter has `issue: 3031`. Superseded by round 2 row F5: the template key `issue-spec` is back.
- Current-tree verification: `sed -n 1,5p docs/issues/open/3031-2387-si-1-statistics-disabled-warning-per-event/manual-verification-evidence.md` shows `issue: 3031`; the frontmatter check of `./contrib/dev-tools/git/hooks/pre-commit.sh` passed.
- Resolution reference: fix(statistics): [#3031] address review round 1 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684668708

### F4 - The disabled-instance tests do not catch a regression to `info!` or `debug!`

- PR number: 3043
- Source review ID: 6054010185
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692117434
- Concern: The disabled-instance tests only excluded `WARN` and `ERROR`, so changing `tracing::trace!` to `tracing::info!` in the disabled arm kept every new test green.
- Solution: Each of the three tests now requires at least one record with `instance_index=1` and that every such record contains ` TRACE `.
- Current-tree verification: `cargo test -p torrust-tracker-udp-server -p torrust-tracker-udp-core -p torrust-tracker-http-core --lib statistics::event::listener` passed; the http-core run was repeated 60 times, udp-core and udp-server 40 times each, with no failure. I did not mutate the arm to `info!` to watch the test fail.
- Resolution reference: fix(statistics): [#3031] address review round 2 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692213105

### F5 - The evidence frontmatter departs from its template

- PR number: 3043
- Source review ID: 6054010185
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692117441
- Concern: The evidence file carried `issue:` while `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` and 134 other evidence files carry `issue-spec`; the round-1 change (F3) rested on a reading of `write-markdown-docs` that is about source artifacts of a draft specification.
- Solution: The frontmatter has `issue-spec: docs/issues/open/3031-2387-si-1-statistics-disabled-warning-per-event/ISSUE.md` again. F3 above followed CodeRabbit's request without checking it against the template; its Solution now says it is superseded by this row.
- Current-tree verification: `grep -n "^issue-spec:" docs/issues/open/3031-2387-si-1-statistics-disabled-warning-per-event/manual-verification-evidence.md` prints the line; the frontmatter check of the pre-commit script passed.
- Resolution reference: fix(statistics): [#3031] address review round 2 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692213439

### F6 - V3 rests on runs whose output is not kept

- PR number: 3043
- Source review ID: 6054010185
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692117445
- Concern: The two statistics-enabled runs and the scenario script were not in the issue folder, so AC4 could not be recomputed from the tree, and `ISSUE.md` said no script was planned.
- Solution: The logs and metric responses of both runs and the script are in `evidence/`, linked from the evidence file; `ISSUE.md` names the script. The release recheck was run again on the rebuilt binary because production code changed (F7).
- Current-tree verification: `ls docs/issues/open/3031-2387-si-1-statistics-disabled-warning-per-event/evidence` lists `v3-before-statistics-enabled.log`, `v3-after-statistics-enabled.log`, both `.metrics.txt` files and `run-scenario.sh`; `grep -cE ' (WARN|ERROR) '` on the three v2 and v3 logs prints 0 for each.
- Resolution reference: fix(statistics): [#3031] address review round 2 of the statistics-disabled fix; docs(issues): [#3031] keep the statistics-enabled run evidence and the scenario script
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692213771

### F7 - "Received a HTTP tracker event" should read "an HTTP"

- PR number: 3043
- Source review ID: 6054010185
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692117449
- Concern: The unknown-instance message of the HTTP listener had the article `a` before `HTTP`; the message is operator-visible.
- Solution: The message reads `Received an HTTP tracker event from an instance that has no statistics policy entry`.
- Current-tree verification: `grep -n "Received an\? HTTP" packages/http-core/src/statistics/event/listener.rs` shows only the `an` form.
- Resolution reference: fix(statistics): [#3031] address review round 2 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692214162

### F8 - Three `dead_code` reasons in the native fixture are wrong

- PR number: 3043
- Source review ID: 6054010185
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692117457
- Concern: `plain_output`, `http_tracker_address` and `wait_for_exit` said only the shutdown-policy binary uses them, but the statistics-disabled binary calls them too.
- Solution: The three reasons name both binaries.
- Current-tree verification: `grep -n "Only the shutdown-policy" tests/common/native_tracker/mod.rs` shows the three lines naming both binaries.
- Resolution reference: fix(statistics): [#3031] address review round 2 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692214564

### F9 - Trailing spaces in `t2-red-unit-tests.txt`

- PR number: 3043
- Source review ID: 6054010185
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692117462
- Concern: Three lines of the red unit test excerpt ended in a space.
- Solution: The trailing spaces are removed.
- Current-tree verification: `git diff --check torrust/develop...` printed nothing.
- Resolution reference: fix(statistics): [#3031] address review round 2 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4692215009

## Processing Log

- 2026-11-24 18:21 UTC - Round 1: Greptile review 6044956173 (18:21:43) with two threads.
- 2026-11-24 18:23 UTC - Round 1: CodeRabbit review 6044982742 (18:23:35) with one thread.
- 2026-11-24 18:24 UTC - Committed the three fixes in one commit (committer time 18:24:53), then pushed it.
- 2026-11-24 18:28 UTC - Replied on the three threads after the push (18:27:58 to 18:28:03) and resolved them.
- 2026-11-24 18:29 UTC - CodeRabbit comment 4684687075 (review 6045065462, 18:29:41) confirms F3 is fixed and says it does not resolve the thread before this audit validation; it holds no new request, so it has no row.
- 2026-11-24 18:31 UTC - Created this audit record after the replies and the resolutions (the first reply was posted before the record existed); the validation script ran after this entry.
- 2026-11-25 08:18 UTC - Round 2: da2ce7 review 6054010185 (08:18:03) with six threads; rows F4 to F9.
- 2026-11-25 08:24 UTC - Committed the code fixes (`fix(statistics): [#3031] address review round 2 of the statistics-disabled fix`) and the evidence (`docs(issues): [#3031] keep the statistics-enabled run evidence and the scenario script`) as two commits; the first attempt failed the shellcheck gate on the new scenario script, which was corrected before both commits were made. Pushed after that.
- 2026-11-25 08:28 UTC - Replied on the six threads after the push (08:28:19 to 08:28:31) and resolved them.
- 2026-11-25 08:29 UTC - F3 of round 1 rewrote its own Solution line to name its supersession by F5 (an appended sentence on the line; no earlier text was removed). During the round the helper `capture_logs_in_scope` was found flaky (one failure in about four runs of the http-core tests, `tracing` caching a callsite as never interested) and fixed in `fix(statistics): [#3031] address review round 2 of the statistics-disabled fix`; it is not a review finding.
- 2026-11-25 08:29 UTC - Updated this audit record after the replies and the resolutions; the validation script ran after this entry.
- 2026-11-25 08:55 UTC - Correction (review 6054395106, finding F7 of the reviewer): the Resolution references of F5, F6 and F9 named `docs(issues): [#3031] keep the statistics-enabled run evidence and the scenario script`, which carries only the quoting of the scenario script and one sentence of the evidence. The restored `issue-spec` key (F5), the five new evidence files and the `ISSUE.md` change (F6) and the trailing-space fix (F9) are in `fix(statistics): [#3031] address review round 2 of the statistics-disabled fix`; the three references now say so (F6 cites both subjects, because the second carries the quoting correction of the script). The 08:24 UTC entry above said the code fixes and the evidence went into two commits with the script corrected before both; what happened is that the first commit swept in the evidence with the unquoted script as it had run, and the second corrected the quoting. The 08:24 entry is left as written.

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
