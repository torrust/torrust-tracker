---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3062 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3062.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`
- Reviewer: the author's GitHub login
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`, `maintainability`, `security`, `other`

## Findings

F1 is from claude[bot]'s inline comment in review 6061977982. F2 to F9 are da2ce7's inline comments in review 6062149260 and F10 is the finding in that review's body; F2 re-raises F1. F11 to F13 are from CodeRabbit's review 6062534811: F11 and F12 are the two findings of its inline comment 4698868795 (they share one thread and one reply) and F13 is its inline comment 4698868806. F14 is da2ce7's inline comment in review 6062583818 (dismissed; he posted it with an `ACK` of the previous head) and F15 is CodeRabbit's reply 4698953937 (review 6062636679) in the thread of F11 and F12, recorded against that thread's root comment; CodeRabbit's reply 4698953208 in the thread of F13 withdraws that finding and has no row. F16 is CodeRabbit's reply 4699066263 (review 6062770209) in the same thread, recorded against its root comment. The claude[bot] reviews 6061840285 and 6061977982 summaries, the CodeRabbit review-limit comment and the maintainer's `@claude review` and `ACK` comments contain no actionable assertion and have no rows.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3062-f1 | claude[bot] | Suggestion (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3062-f2 | da2ce7 | Major | testing | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F3 | review-finding:pr-3062-f3 | da2ce7 | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3062-f4 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3062-f5 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3062-f6 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3062-f7 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3062-f8 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3062-f9 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3062-f10 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F11 | review-finding:pr-3062-f11 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3062-f12 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3062-f13 | coderabbitai[bot] | Minor | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F14 | review-finding:pr-3062-f14 | da2ce7 | Minor | correctness | RE_RAISE_OF:F12 | FIXED | RESOLVED |
| F15 | review-finding:pr-3062-f15 | coderabbitai[bot] | Minor (inferred) | correctness | RE_RAISE_OF:F11 | FIXED | RESOLVED |
| F16 | review-finding:pr-3062-f16 | coderabbitai[bot] | Minor (inferred) | correctness | RE_RAISE_OF:F15 | FIXED | RESOLVED |

## Finding Details

### F1 - The planned tests use a capture helper without the flake guard

- PR number: 3062
- Source review ID: 6061977982
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698401092
- Concern: The spec steered the T2 implementer to `capture_logs_while`, said `capture_logs_in_scope` was not on `develop`, and the tests could capture nothing under parallel `cargo test`.
- Solution: The Regression Test Strategy names `capture_logs_in_scope()` under `#[tokio::test]`, explains the second-dispatcher guard and the parallel tests that reach the same callsites, and drops the rename claim.
- Current-tree verification: `grep -n capture_logs_in_scope` on `ISSUE.md` shows lines 93 to 95; `grep -c capture_logs_while` is 1 (the explanation of why it is not used); both helpers exist in `packages/test-helpers/src/logging.rs`.
- Resolution reference: docs(issues): [#3060] plan the regression tests on capture_logs_in_scope and assert the stray field is absent
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698742096

### F2 - `capture_logs_while` is not the helper #3031 added and `capture_logs_in_scope` is on `develop`

- PR number: 3062
- Source review ID: 6062149260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698547956
- Concern: The same concern as F1, with the history: `capture_logs_in_scope` came with #3031 and is used by the listener tests, and only it calls `keep_a_second_dispatcher_alive`; a test on `capture_logs_while` can capture nothing, and the "no module-path prefix" check passes on an empty capture.
- Solution: Same change as F1, plus a non-empty-capture assertion with exactly one matching line, and the handoff aside is gone. V1 was rerun with `capture_logs_in_scope` (`evidence/v1-before-fix.txt`).
- Current-tree verification: Lines 93 to 95 of `ISSUE.md` state the helper, the reason and the assertions; the V1 evidence names `capture_logs_in_scope`.
- Resolution reference: docs(issues): [#3060] plan the regression tests on capture_logs_in_scope and assert the stray field is absent
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698742518

### F3 - AC1 and AC2 cite a "no `target` field" check the planned tests do not make

- PR number: 3062
- Source review ID: 6062149260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698547966
- Concern: The planned assertions covered the `UDP TRACKER:` prefix and the absence of the module path, not the absence of `target="UDP TRACKER"`.
- Solution: The planned assertions now include that the line does not contain `target="UDP TRACKER"`.
- Current-tree verification: Line 95 of `ISSUE.md` lists the four assertions, the last being the stray field.
- Resolution reference: docs(issues): [#3060] plan the regression tests on capture_logs_in_scope and assert the stray field is absent
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698742950

### F4 - The scope promises a guard the plan does not deliver

- PR number: 3062
- Source review ID: 6062149260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698547976
- Concern: The in-scope item promised a guard, while the Design Decision default adds none and no task delivers one.
- Solution: The in-scope item now reads as an open question with no guard planned by default, as suggested.
- Current-tree verification: Line 60 of `ISSUE.md` reads "Decide whether to add a repository-wide guard ... (open question, see Design Decision); none is planned by default."
- Resolution reference: docs(issues): [#3060] make AC3 a multi-line scan and state the guard as an open question
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698743416

### F5 - The AC3 command misses calls written across lines and the multi-line scan is not recorded

- PR number: 3062
- Source review ID: 6062149260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698547985
- Concern: `\(\s*target\s*=` matches only on one line; the multi-line variant was never written down and neither scan output was recorded.
- Solution: AC3 now uses the multi-line `git grep -n -P -z -o` scan, written out in full; both scans and their output are recorded in the evidence file (that recording is in the commit named in F6). Correction (round 2, finding F12): the `git grep -z` scan named here did not match across lines; it was replaced by a GNU `grep -rPzo` scan.
- Current-tree verification: Ran both scans at 2026-11-25 17:37 UTC: both print only `announce.rs:178` and `scrape.rs:140`; the evidence file section "V1 scope scans" holds the output.
- Resolution reference: docs(issues): [#3060] make AC3 a multi-line scan and state the guard as an open question
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698743895

### F6 - The scrape half of V1 is neither verbatim nor complete

- PR number: 3062
- Source review ID: 6062149260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698547996
- Concern: The scrape temporary test was only "the same shape" and its output was elided, so the stray field for scrape was not shown.
- Solution: The evidence file embeds both temporary tests verbatim and both captured lines in full; the diff and the trimmed cargo output are under `evidence/`. A later commit, `docs(issues): [#3060] strip the trailing space from the recorded scan output`, removes a trailing space that the recorded scan output carried (`git diff --check` flagged it).
- Current-tree verification: `evidence/v1-temporary-tests.diff` and `evidence/v1-before-fix.txt` exist; `git diff --check torrust/develop...HEAD` printed nothing after the last evidence commit.
- Resolution reference: docs(issues): [#3060] record the V1 reproduction verbatim for both services
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698744381

### F7 - `112 filtered out` does not recompute at the merge base

- PR number: 3062
- Source review ID: 6062149260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698548009
- Concern: The reviewer counted 109 library tests and 110 with the temporary test, so 109 filtered out, against the recorded 112.
- Solution: The evidence now records the line as the rerun printed it (`112 filtered out`) and the count command with its result (114 tests with the two temporary tests, 112 without). I did not reconcile this with the reviewer's count of 109; the record states only what the commands printed.
- Current-tree verification: `cargo test -p torrust-tracker-udp-core --lib -- --list | grep -c ": test$"` printed 114 with the temporary tests and 112 after the revert.
- Resolution reference: docs(issues): [#3060] record the V1 reproduction verbatim for both services
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698744830

### F8 - The V1 status is not a template value

- PR number: 3062
- Source review ID: 6062149260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698548016
- Concern: "DONE (before-fix result); after-fix rerun TODO" is not one of the template status values and disagreed with the evidence file.
- Solution: The table status is `IN_PROGRESS`, with the state in the Evidence column.
- Current-tree verification: Line 167 of `ISSUE.md` shows `IN_PROGRESS`; the evidence file V1 status is `IN_PROGRESS`.
- Resolution reference: docs(issues): [#3060] record the V1 reproduction verbatim for both services
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698745344

### F9 - The order of logging and sending is reversed in the Background

- PR number: 3062
- Source review ID: 6062149260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698548022
- Concern: The Background said `send_event` publishes the event and logs the DEBUG line; the code logs first and then sends.
- Solution: Step 1 now says it logs the DEBUG line and then publishes the event.
- Current-tree verification: Line 44 of `ISSUE.md`; the code order is at `announce.rs:178-180` and `scrape.rs:140-142`.
- Resolution reference: docs(issues): [#3060] state that send_event logs before it publishes the event
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698745752

### F10 - The PR description has no validation performed and does not say the temporary test is left out

- PR number: 3062
- Source review ID: 6062149260
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#pullrequestreview-6062149260
- Concern: The description lacked a validation section and did not say the temporary reproduction test is not part of the PR (`open-pull-request`, "PR body must include").
- Solution: The description now has a "Validation performed" section listing the commands that ran and stating that the temporary tests were reverted and are not part of the PR.
- Current-tree verification: the PR description was edited at 2026-11-25 17:51 UTC; `gh-bot pr view 3062 --repo torrust/torrust-tracker --json body` shows the section.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3062#issuecomment-7170096506
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#issuecomment-7170096506

### F11 - The AC3 scan misses `target` fields that are not the first macro argument

- PR number: 3062
- Source review ID: 6062534811
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698868795
- Concern: The pattern required `target` immediately after `!(`, so `tracing::event!(Level::DEBUG, target = "UDP TRACKER", "message")` is not matched.
- Solution: The pattern now allows other arguments before `target =`; AC3 and the Verified scope paragraph state it.
- Current-tree verification: ran the scan on `evidence/scan-sample.rs`, which holds an `event!(Level::DEBUG, target = ...)` call: it is matched; on the repository it prints only `announce.rs` and `scrape.rs`.
- Resolution reference: docs(issues): [#3060] scan for the field form with GNU grep so calls across lines and later arguments are found
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698934742

### F12 - The AC3 `git grep -z` scan does not match across newline boundaries

- PR number: 3062
- Source review ID: 6062534811
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698868795
- Concern: `git grep -z` alone does not match across lines; the scan was never shown to detect a call with the macro name and `target =` on separate lines.
- Solution: AC3 uses GNU `grep -rPzo`, which reads each file as one record, and the evidence file shows its detection on a sample file with a multi-line call. This also corrects my own earlier statement (replies to F5, the F5 entry below, and the first scan recorded in the evidence file) that the `git grep -z` scan was a multi-line scan; it was not, and the earlier evidence section was replaced.
- Current-tree verification: `git grep --no-index -c -P -z '(?s)debug!\(\s*target\s*=' -- s.rs` printed nothing on the sample; `grep -Pzo` with the new pattern prints the multi-line `debug!(` call and the `event!` call and not the colon form or the string `"target = nope"`; the repository scan prints only the two known sites.
- Resolution reference: docs(issues): [#3060] scan for the field form with GNU grep so calls across lines and later arguments are found
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698934742

### F13 - The evidence frontmatter keeps the `issue-spec` marker

- PR number: 3062
- Source review ID: 6062534811
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698868806
- Concern: Issue #3060 exists, so the evidence file should use `issue: "#3060"` instead of `issue-spec`, per a coding guideline the reviewer quotes.
- Solution: No change. `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` prescribes `issue-spec: docs/issues/open/{number}-{short-description}/ISSUE.md`, the field already holds the `open/3060-...` path, and all 16 open evidence files use `issue-spec`. I found no rule in the tree that asks for an `issue:` key here, and asked the reviewer to point to one.
- Current-tree verification: `sed -n 3p docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` prints the `issue-spec:` line; `grep -l '^issue-spec:' docs/issues/open/*/manual-verification-evidence.md | wc -l` printed 16, the number of open evidence files.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698935340
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698935340

### F14 - The AC3 scan still matches one line at a time

- PR number: 3062
- Source review ID: 6062583818
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698910699
- Concern: The reviewed head still carried the `git grep -n -P -z -o` scan, which has no multi-line mode (`-z` only separates output fields); on a sample it missed calls with `target = ...` or `= ...` on the next line. The same claim was in the Verified scope paragraph and in the evidence file. He suggested a scan that reads each file whole, with its output recorded.
- Solution: The scan was already replaced by GNU `grep -rPzo` in the commit named below (pushed before this review landed on its head) together with the sample-file detection check and the repository output; the later commit for F15 only widens the pattern. It is the same idea as his `perl -0777` example.
- Current-tree verification: `grep -Pzo` with the current pattern on `evidence/scan-sample.rs` prints the multi-line `debug!(` call and two `event!` calls; the repository scan prints only `announce.rs` and `scrape.rs`.
- Resolution reference: docs(issues): [#3060] scan for the field form with GNU grep so calls across lines and later arguments are found
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4699050783

### F15 - The AC3 scan misses a later `target` field after an argument with parentheses

- PR number: 3062
- Source review ID: 6062534811
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698868795
- Concern: Restated by CodeRabbit in its reply 4698953937 (review 6062636679) in the thread of F11 and F12: after F12's fix, F11 remained: the pattern `[^;{}()]*?` before `target =` stops at parentheses, so `event!(tracing::Level::DEBUG, count = (1 + 1), target = "UDP TRACKER", "message")` is not matched. The reviewer asked to extend the scan and the sample, or narrow AC3.
- Solution: The pattern now skips an earlier argument with one level of parentheses (`(?:[^;{}()]|\([^()]*\))*?`); the case is in `evidence/scan-sample.rs` and its recorded output; AC3 and the Verified scope paragraph describe the three cases.
- Current-tree verification: `grep -Pzo` with the current pattern on the sample prints three field-form calls (across lines, after a leading argument, after an argument with parentheses); the repository scan prints only `announce.rs` and `scrape.rs`.
- Resolution reference: docs(issues): [#3060] let the AC3 scan skip earlier arguments that contain parentheses
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4699051172

### F16 - AC3 still claims broader coverage than the scan provides

- PR number: 3062
- Source review ID: 6062534811
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4698868795
- Concern: Restated by CodeRabbit in its reply 4699066263 (review 6062770209) in the thread of F11 and F12: the scan misses a field after nested parentheses, for example `count = ((1 + 1)), target = ...`. The reviewer asked to narrow the claim to the supported cases or to handle nested expressions.
- Solution: A regular expression cannot follow arbitrary nesting, so the claim was narrowed: AC3, the Verified scope paragraph and the evidence file state the supported cases (across lines, a leading argument, one level of parentheses) and that deeper nesting is a known limit.
- Current-tree verification: `grep -Pzc` with the current pattern on a line `tracing::event!(tracing::Level::DEBUG, count = ((1 + 1)), target = "UDP TRACKER", "message");` printed 0 matches; `grep -n 'nested parentheses' ISSUE.md` shows the stated limit.
- Resolution reference: docs(issues): [#3060] limit the AC3 claim to one level of parentheses in earlier arguments
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3062#discussion_r4699150146

## Processing Log

- 2026-11-25 17:21 UTC - claude[bot]'s review 6061977982 with its inline comment (F1) arrived.
- 2026-11-25 17:34 UTC - da2ce7's review 6062149260 arrived with F2 to F10 (changes requested). The maintainer's `ACK` of the previous head (comment at 17:29) predates it.
- 2026-11-25 17:50 UTC - Pushed the fix commits for F1 to F9.
- 2026-11-25 17:51 UTC - Replied to the nine inline threads and edited the PR description and posted the conversation response for F10 (comment 7170096506).
- 2026-11-25 17:52 UTC - Committed a wording fix found while re-checking the spec: `docs(issues): [#3060] capitalize the Verified scope sentence`.
- 2026-11-25 17:53 UTC - Created this record from the template. It was started after the replies were posted, not before the first reply as `process-pr-review` requires; the late start is logged here. The `ACK` of the maintainer names the previous head, so the head needs a new `ACK` after this push.
- 2026-11-25 18:01 UTC - CodeRabbit's review 6062534811 arrived with F11 to F13.
- 2026-11-25 18:04 UTC - Committed the F11 and F12 fix (`docs(issues): [#3060] scan for the field form with GNU grep so calls across lines and later arguments are found`).
- 2026-11-25 18:07 UTC - Pushed it, then replied to both threads (F13 is a no-change reply, thread to be resolved).
- 2026-11-25 18:07 UTC - Added F11 to F13 and the F5 correction to this record.
- 2026-11-25 18:14 UTC - Found, in this order, da2ce7's review 6062583818 (posted 18:05 UTC, F14, with an `ACK` of the previous head after he dismissed review 6062149260) and CodeRabbit's replies (posted 18:09 UTC) in the thread of F13 (it withdraws that finding) and in the thread of F11 and F12 (F11 remains, F15). Then committed the F15 fix (`docs(issues): [#3060] let the AC3 scan skip earlier arguments that contain parentheses`).
- 2026-11-25 18:17 UTC - Pushed it and replied to the threads of F14 and F15.
- 2026-11-25 18:18 UTC - Added F14 and F15 to this record.
- 2026-11-25 18:23 UTC - Found CodeRabbit's reply 4699066263 (posted 18:19 UTC, F16) and committed the fix (`docs(issues): [#3060] limit the AC3 claim to one level of parentheses in earlier arguments`).
- 2026-11-25 18:26 UTC - Pushed it and replied in the thread.
- 2026-11-25 18:27 UTC - Added F16 to this record.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
