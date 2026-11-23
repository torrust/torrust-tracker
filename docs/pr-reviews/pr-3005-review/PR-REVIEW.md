---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore Greptile -->

# PR #3005 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3005>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F11 | review-finding:pr-3005-f11 | Unknown | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3005-f12 | Unknown | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F1 | review-finding:pr-3005-f1 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3005-f2 | Human | Minor | testing | RE_RAISE_OF:F11 | FIXED | RESOLVED |
| F3 | review-finding:pr-3005-f3 | Human | Minor | correctness | RE_RAISE_OF:F12 | FIXED | RESOLVED |
| F4 | review-finding:pr-3005-f4 | Human | Minor | metadata | ORIGINAL | FOLLOW_UP | RESOLVED |
| F5 | review-finding:pr-3005-f5 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3005-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3005-f7 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3005-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3005-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3005-f10 | Human | Suggestion | security | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3005-f13 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3005-f14 | Copilot | Minor | testing | RE_RAISE_OF:F11 | FIXED | RESOLVED |
| F15 | review-finding:pr-3005-f15 | Copilot | Minor | testing | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F16 | review-finding:pr-3005-f16 | Copilot | Minor | correctness | RE_RAISE_OF:F12 | FIXED | RESOLVED |
| F17 | review-finding:pr-3005-f17 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-3005-f18 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-3005-f19 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-3005-f20 | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F11 - The planned tests do not cover the key in `check` results

- PR number: 3005
- Source review ID: 6022884092
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4666756177
- Concern: Greptile (P2): the planned tests cover the masking helper and a failed announce but not the `check` result, which copies the configured URL into `Checks.url`; they could pass while `check` still writes the key. Add a keyed `check` regression test over stdout and stderr, including the `url` field.
- Solution: The Regression Test Strategy has a black-box `tracker_client check` test with a keyed HTTP URL on a closed local port that asserts neither stdout nor stderr holds the key and the `url` field shows `****`.
- Current-tree verification: `grep -n "black-box test of .tracker_client check." docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches the new bullet.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667098019

### F12 - M1 pipes stdout into `cat` and loses the exit status

- PR number: 3005
- Source review ID: 6022884092
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4666756186
- Concern: Greptile (P2): M1 expects exit `1`, but `... | cat` without `pipefail` returns `cat`'s status; M2 inherits it. Redirect stdout to a file as R0 does, or capture the exit code.
- Solution: M1 redirects stdout and stderr to files as R0 does and echoes `exit=$?` right after the client; M2 says "the same", so it inherits the form.
- Current-tree verification: `grep -c "| cat" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` prints 0; `grep -cF '> out.txt 2> err.txt; echo "exit=$?"'` on the spec prints 1 (the M1 row).
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667098716

### F1 - The spec names the unused `CheckerOutput` as the `check` URL source

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015650
- Concern: `CheckerOutput` in `checks/structs.rs` is never used; the `url` that leaks in R0c is `Checks.url` in `checks/http.rs` (`:17`, filled at `:36-37`), and the per-request texts come from `clients/http/mod.rs:11,15`. Point the list there and at `checks/health.rs` for the health-check URL.
- Solution: The Known sources bullet names `Checks.url` (`checks/http.rs:17`, filled at `:37`), the `Err` texts from `clients/http/mod.rs:11,15`, the health-check `Checks.url` (`checks/health.rs:31`), and records `CheckerOutput` as unused.
- Current-tree verification: `grep -rn CheckerOutput console/tracker-client/src` matches only its declaration at `checks/structs.rs:9`; `sed -n 31p console/tracker-client/src/console/clients/checker/checks/health.rs` shows `url: Url,`.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667099522

### F2 - The planned regression tests cover neither `check` nor `http scrape`

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015664
- Concern: AC1 covers every subcommand, but the planned tests cover neither `check` (whose URL never goes through the library error) nor `http scrape`. Add a black-box `check` test with a keyed URL, and name scrape in the black-box case or say why announce covers it.
- Solution: Black-box tests for `http announce` and `http scrape` (separate URL and error paths, so one test each) and a black-box `check` test; Bug-Fix Process step 3 names them.
- Current-tree verification: `grep -n "tracker_client http scrape" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches the black-box bullet; `grep -n "black-box" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` shows step 3 and the strategy bullets.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667100212

### F3 - M1 cannot observe the expected exit `1` through `| cat`

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015672
- Concern: With `| cat`, the exit status is `cat`'s, so M1 cannot observe exit `1`; M2 inherits it. Use R0's redirect form or `${PIPESTATUS[0]}`.
- Solution: Same change as F12: M1 uses R0's redirect form and echoes the exit status.
- Current-tree verification: `grep -c "| cat" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` prints 0.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667100846

### F4 - The EPIC #2758 table does not list this issue

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015682
- Concern: The spec sets `epic: 2758`, but the EPIC Subissues table and progress log do not list it; the last sub-issue spec added its row in the same commit.
- Solution: Not changed here: the EPIC coordinator session owns the EPIC table and adds this row (with #3006) in PR #3007, so this PR does not edit `EPIC.md` and does not conflict with it.
- Current-tree verification: `gh-bot pr view 3007 --repo torrust/torrust-tracker` shows the open PR `docs(issues): [#2758] add #3004 and #3006 to the EPIC and the #2770 scope` from branch `2758-epic-rows-3004-3006`; `git diff --name-only torrust/develop... -- docs/issues/open/2758-cli-output-contract-tools-migration` is empty on this branch.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667101490
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/3007
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667101490

### F5 - The progressive test loop and the maintainer stop are missing

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015708
- Concern: `create-issue` requires the plan to state the prose-first Arrange-Act-Assert design review per test-producing subtask and a maintainer stop after the final increment; T6 named only the R0 recheck, while `fix-bug` pairs it with the green run.
- Solution: The Regression Test Strategy states the progressive loop and the recorded comparison; the plan has T5 "Maintainer stop" and T7 "Green run and recheck"; the Commit Points say T2 to T4 follow the loop.
- Current-tree verification: `grep -n "Maintainer stop\|Green run and recheck\|prose-first" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches the plan rows and the strategy text.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667102292

### F6 - Out of Scope refers to other issues without numbers

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015754
- Concern: The UDP-port panic is #2984; no schema-version issue was in the tree. Name them, or say the second is proposed.
- Solution: Out of Scope names #2984 and #3006 (the schema-version issue filed for this EPIC).
- Current-tree verification: `grep -n "#2984\|#3006" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches both Out of Scope bullets; `gh-bot issue view 3006 --repo torrust/torrust-tracker` exists.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667103055

### F7 - The evidence outcome class and its timestamp

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015764
- Concern: The outcome is not one of `fix-bug`'s classes (it should say `Reproduced`), and `last-updated-utc` predates the `issue-spec` path written after the issue existed.
- Solution: The Outcome starts with `Reproduced.`; `last-updated-utc` is the time of the edit from `date -u`.
- Current-tree verification: `grep -n "^Reproduced\.\|last-updated-utc" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/manual-verification-evidence.md` shows both.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667103741

### F8 - The public-issue risk is left open while the log records it as approved

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015778
- Concern: State the public-issue decision as decided, with the reason, so the risk and the progress log agree.
- Solution: The risk states the maintainer's decision and the reason (the key is the caller's own, in the caller's own output).
- Current-tree verification: `grep -n "Public issue: decided" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667104431

### F9 - The PR description has no validation line

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015791
- Concern: `open-pull-request` requires the description to state the validation performed.
- Solution: The PR description (outside the tree) has a Validation section: `linter all`, the pre-commit gate on each commit, the re-read against `develop` `ff3600f10`.
- Current-tree verification: `gh-bot pr view 3005 --repo torrust/torrust-tracker --json body` shows the `## Validation` section.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667105219
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667105219

### F10 - The e2e runner logs the checker configuration verbatim

- PR number: 3005
- Source review ID: 6023213938
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667015800
- Concern: `src/console/ci/e2e/tracker_checker.rs:88` logs the whole checker configuration; a keyed URL there would reach the same logs. Today it carries no key (`runner.rs:117-118`). A line in Out of Scope keeps the T1 audit from missing it.
- Solution: Out of Scope records the log line, why it has no key today, and that T1 records it as checked.
- Current-tree verification: `grep -n "tracker_checker.rs:88" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches the Out of Scope bullet; `sed -n 88p src/console/ci/e2e/tracker_checker.rs` shows the `Tracker Checker config:` log.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667105945

### F13 - Ownership of the regression-test resources is not defined

- PR number: 3005
- Source review ID: 6023264037
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667053870
- Concern: The planned request and black-box tests involve asynchronous I/O and a child process, so "Not applicable" leaves waits and cleanup unspecified; define ownership, bounded waits, normal and failure/drop cleanup and a checkpoint after the first vertical slice.
- Solution: Design and Ownership Review defines the closed port, the test-owned local HTTP responder (aborted at the end, no readiness wait needed), the child process run with `Command::output()`, the deadlines (client timeout plus `tokio::time::timeout`) and the post-slice checkpoint.
- Current-tree verification: `grep -n "Closed port\|Local HTTP responder\|Child process\|Deadlines\|Checkpoint" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches the five rules.
- Resolution reference: `docs(issues): [#3004] add test resource ownership and more leak-path tests to the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667106647

### F14 - The tests reach only the connection-failure path

- PR number: 3005
- Source review ID: 6023264037
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667053929
- Concern: The request tests exercise only a refused connection: they never reach `UnsuccessfulResponse` or `response.bytes()`, and neither checks the URL serialized by `check`. Add keyed cases for the checker result, a non-2xx reply and a body-read failure.
- Solution: The keyed `check` black-box test came with the F11 fix (commit `docs(issues): [#3004] fix the review round 1 findings in the spec`); this commit adds library-seam tests for a non-2xx reply and a failed body read with the local responder.
- Current-tree verification: `grep -n "non-2xx reply" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches the strategy bullet; the `check` bullet is as in F11.
- Resolution reference: `docs(issues): [#3004] add test resource ownership and more leak-path tests to the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667107368

### F15 - The test-design review is not auditable

- PR number: 3005
- Source review ID: 6023264037
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667054003
- Concern: The plan omits the recorded prose-first Arrange-Act-Assert comparison and the checkpoint before the next test-producing subtask.
- Solution: Same change as F5: the comparison is recorded in the evidence file after each increment, before the next starts.
- Current-tree verification: `grep -n "prose-first" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches the strategy text.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667108057

### F16 - M1 and M2 lose the client exit status

- PR number: 3005
- Source review ID: 6023264037
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667054056
- Concern: Without `pipefail`, M1 reports `cat`'s status; redirect stdout and stderr as in R0 and capture the client status.
- Solution: Same change as F12.
- Current-tree verification: `grep -c "| cat" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` prints 0.
- Resolution reference: `docs(issues): [#3004] fix the review round 1 findings in the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667108765

### F17 - The completion review has no recording procedure

- PR number: 3005
- Source review ID: 6023264037
- Reviewer finding ID: R4
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667054101
- Concern: `Not yet assessed` supplies no procedure; the template requires a retrospective or a progress-log explanation.
- Solution: Implementation Completion Review says the result is compared with the spec and recorded either in `implementation-retrospective.md` or in a progress-log entry, and the checkbox needs one of the two.
- Current-tree verification: `grep -n "completion-review checkbox" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches.
- Resolution reference: `docs(issues): [#3004] add test resource ownership and more leak-path tests to the spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667109458

### F18 - The body-read test is placed at the library seam and in the console client

- PR number: 3005
- Source review ID: 6023559554
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667287157
- Concern: The ownership map put the body-read test at the library seam while the strategy put it in the console client, where `bytes()` is called (`unified/http.rs:187,215`, `checks/http.rs:76,94`). Say which responder the console-side test uses; `tests/common/fake_trackers/http.rs` already owns a fake HTTP server.
- Solution: The local responder covers only the non-2xx case at the library seam. The body-read case is a black-box test of `tracker_client http announce` against the existing fake HTTP tracker extended with a fixed-reply mode; the ownership map has a "Fake HTTP tracker" rule for it.
- Current-tree verification: `grep -n "Fake HTTP tracker\|the non-2xx case at the library seam" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches both; `ls console/tracker-client/tests/common/fake_trackers/http.rs` exists.
- Resolution reference: `docs(issues): [#3004] place the body-read test in the console client`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667323105

### F19 - The Processing Log's reply window ends too early

- PR number: 3005
- Source review ID: 6023559554
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667287167
- Concern: The round 1 log entry says the replies were created 14:23:44Z to 14:24:29Z, but the F14 to F17 replies were created 14:24:32Z to 14:24:43Z.
- Solution: The log is append-only, so a correction entry below states the window as 14:23:44Z to 14:24:43Z; the round 1 entry is unchanged.
- Current-tree verification: `gh-bot api repos/torrust/torrust-tracker/pulls/3005/reviews` lists the 17 `josecelano-bot` reply reviews of round 1 submitted from 14:23:44Z to 14:24:43Z.
- Resolution reference: `docs(pr-reviews): [#3005] record review round 2`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667324172

### F20 - Confirm that a body-read error carries the URL

- PR number: 3005
- Source review ID: 6023559554
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667287179
- Concern: If reqwest builds body errors without the request URL, the "does not contain the key" assertion of the body-read case cannot fail on the unchanged code; check `response.rs` and `error.rs` and assert what the source shows.
- Solution: Checked: `Response::bytes()` calls `do_bytes`, which maps a collect failure with `crate::error::decode(err).with_url(*self.url)` (`reqwest-0.13.5/src/async_impl/response.rs:435`), so the error carries the URL and the key assertion can be red. The spec cites the line in the Background and in the test bullet, and keeps the AC2 red case for the checker's `Debug` text.
- Current-tree verification: `sed -n 430,436p ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/reqwest-0.13.5/src/async_impl/response.rs` shows the `with_url` call; `grep -n "response.rs:435" docs/issues/open/3004-2758-tracker-client-mask-tracker-key/ISSUE.md` matches the Background and the test bullet.
- Resolution reference: `docs(issues): [#3004] place the body-read test in the console client`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3005#discussion_r4667325011

## Processing Log

- 2026-11-23 14:20 UTC - Round 1 had three reviews: Greptile 6022884092 (COMMENTED, 13:55:02Z, two inline comments, F11 and F12), da2ce7 6023213938 (COMMENTED, 14:16:41Z, ten inline comments, F1 to F10) and Copilot 6023264037 (COMMENTED, 14:19:58Z, five inline comments, F13 to F17; its IDs R1, R2, R2, F2 and R4 collide, so audit IDs were assigned). Fix commits made (committer times 14:20:35Z and 14:22:27Z); the PR description was edited for F9.
- 2026-11-23 14:23 UTC - Replied on the 17 threads as `josecelano-bot` (reply `created_at` 14:23:44Z to 14:24:29Z, before the fix commits were pushed) and resolved each with GraphQL; every `resolveReviewThread` returned `isResolved` true.
- 2026-11-23 14:25 UTC - Started this audit late: the record was written after the replies, not before them as the workflow asks. The validator (`validate-audit-record.py`, run with the bot token) first failed on F14's two-subject resolution reference; F14 now cites one subject.
- 2026-11-23 14:40 UTC - Round 2: review 6023559554 (da2ce7, APPROVED at `0cb4c9cca`, 14:39:23Z) with three inline comments; its IDs F11 to F13 collide with audit IDs, so they are F18 to F20. Fix commit made (committer time 14:41:20Z).
- 2026-11-23 14:42 UTC - Correction to the round 1 entry of 14:23 UTC: the round 1 replies were created from 14:23:44Z to 14:24:43Z (the F17 reply), not to 14:24:29Z (finding F19). Replied on the three round 2 threads as `josecelano-bot` (reply `created_at` 14:42:09Z to 14:42:18Z) and resolved each; every `resolveReviewThread` returned `isResolved` true.

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
