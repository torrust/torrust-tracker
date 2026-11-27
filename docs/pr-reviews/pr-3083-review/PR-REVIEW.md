---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3083 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3083.

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

Review 6084545579 (`claude[bot]`) has no finding: it only says that automated review is disabled for a pull request from a fork.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3083-f1 | coderabbitai[bot] | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3083-f2 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3083-f3 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3083-f4 | coderabbitai[bot] | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3083-f5 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3083-f6 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3083-f7 | coderabbitai[bot] | Minor (inferred) | correctness | RE_RAISE_OF:F6 | FIXED | RESOLVED |
| F8 | review-finding:pr-3083-f8 | da2ce7 | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3083-f9 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3083-f10 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3083-f11 | da2ce7 | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3083-f12 | da2ce7 | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3083-f13 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3083-f14 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3083-f15 | da2ce7 | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-3083-f16 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The ADR links the folder of the issue, whose path changes when the issue is closed

- PR number: 3083
- Source review ID: 6084598281
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716532325
- Concern: The frontmatter of the ADR listed `docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md` and `decisions.md`, and the body linked `decisions.md`; closing the issue moves the folder to `docs/issues/closed/` and the paths break. The reviewer cites the guideline to prefer the issue number.
- Solution: The two paths are removed from the frontmatter, and the body says that `decisions.md` stays with the specification of issue #2766 and that its path changes when the issue is closed, so it is not linked. This follows the stability warning of `docs/skills/semantic-skill-link-convention.md`.
- Current-tree verification: `grep -c "docs/issues" docs/adrs/20261127063102_write_the_daemon_logs_as_json_on_stderr_by_default.md` prints 0.
- Resolution reference: docs(adrs): [#2766] refer to the issue by number in the ADR of the daemon logs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716571391

### F2 - The reproduction setup of R0b, R0c and R1 is not recorded

- PR number: 3083
- Source review ID: 6084598281
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716532334
- Concern: R0b and R0c need a `tracker.toml`, and R0c a listener on port 17071; the evidence has neither the configuration nor the listener command, so a reviewer cannot repeat the baseline or the recheck.
- Solution: A subsection "Setup of R0b, R0c and R1" holds the configuration (with the scratch directory as a placeholder), how the second configuration and `off.toml` differ, the commands that start the tracker and send the `SIGTERM`, the one-line listener and a note that nothing needs cleaning up.
- Current-tree verification: `grep -n "Setup of R0b, R0c and R1" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` finds the heading before "Verification Processes".
- Resolution reference: docs(issues): [#2766] record the setup of the reproductions of the daemon logs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716571634

### F3 - R1 says the exit codes are unchanged, but the baseline exit code of the normal run was not recorded

- PR number: 3083
- Source review ID: 6084598281
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716532339
- Concern: The R1 paragraph claimed the exit codes match R0b and R0c, including `0` after `SIGTERM`, while R0b says its exit code was not recorded.
- Solution: The claim is limited to the bind failure (`1` in R0c and in the recheck). The paragraph now says the normal run was not compared, that the recheck records `0` after `SIGTERM`, and that the lifecycle test `lifecycle-signals` asserts exit code `0` after `SIGTERM` and passes on the final tree.
- Current-tree verification: `grep -n "does not cover it" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` finds the sentence; `grep -c "same exit codes"` on the same file prints 0.
- Resolution reference: docs(issues): [#2766] limit the claim of unchanged exit codes to the one that was recorded
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716571879

### F4 - The commands that start and stop the tracker in the reproduction setup are wrong

- PR number: 3083
- Source review ID: 6084598281
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716532334
- Concern: Raised in a reply on the thread of F2 (comment 4716605638, review 6084688285), so the source is the thread and the review that opened it. In `cd ok && $B … & P=$!` the `&` applies to the whole list, so `$!` is the shell and `kill` stops it and leaves the tracker running; the reviewer asks for `exec` or a separate `cd`, an absolute path for the binary, and a readiness check for the listener of R0c. The comment has no severity; `Minor` is inferred from the reviewer's other comments on the setup.
- Solution: The setup changes the directory in its own command and starts the tracker with `$B -c tracker.toml > out 2> err & P=$!`, with `B` absolute; this is the form the recheck of R1 used, and the text explains why the other form is wrong and that the first run of R0b hit it. The pause that the runs used for the listener stays, marked as not a readiness check, and a loop that waits until the listener accepts a connection is added.
- Current-tree verification: `grep -n "cd ok && " docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` finds only the sentence that explains the wrong form; `grep -c "until python3"` on the same file finds the readiness loop.
- Resolution reference: docs(issues): [#2766] correct the commands that start and stop the tracker in the reproduction setup
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716647035

### F5 - The wait for the listener in the reproduction setup has no limit

- PR number: 3083
- Source review ID: 6084763933
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716667173
- Concern: The loop that waits for the listener of R0c to accept a connection waits forever if the listener, which lives for eight seconds, has already ended, for example when a reader types the commands one by one; the reviewer asks to keep the listener alive through the tracker attempt and to stop the wait at a deadline.
- Solution: The loop is bounded (50 tries of 0.2 seconds, about ten seconds) and the text says to stop and look at the listener if it is not accepting after that. The variant for a reader who types the commands one by one starts a listener that lives for 60 seconds. The recorded runs used the 8-second listener and a one-second pause, which the text keeps and says plainly.
- Current-tree verification: `grep -n "seq 50" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` finds the bounded loop; `grep -c "until python3"` on the same file prints 0.
- Resolution reference: docs(issues): [#2766] bound the wait for the listener in the reproduction setup
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716707735

### F6 - A failed wait for the listener looks like a successful one

- PR number: 3083
- Source review ID: 6084834974
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716726170
- Concern: In the bounded loop of the fix for F5 the last `sleep` succeeds, so a loop in which every attempt failed ends with the same status and output as one that succeeded; the reader cannot tell whether to start the tracker, and the reviewer asks to report the failure and stop before the tracker starts.
- Solution: The loop is removed. The text gives one script that binds and listens on the port first, runs the tracker while it holds the port, with a 20-second timeout, and prints the exit code; if the port cannot be bound it fails with an error and the tracker is not started. The author ran it against the real binary: `exit=1`, 17305 bytes on stdout, 290 on stderr, as in the recorded bind failure. `returncode` is added to `project-words.txt`.
- Current-tree verification: `grep -c "seq 50" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` prints 0; `grep -c "subprocess.run"` on the same file finds the script.
- Resolution reference: docs(issues): [#2766] replace the readiness wait of the reproduction setup with one script that holds the port
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716778913

### F7 - The bounded loop still gives no message, and its listener can expire

- PR number: 3083
- Source review ID: 6084763933
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716667173
- Concern: Raised in a reply on the thread of F5 (comment 4716742352, review 6084854408), about the head of the F5 fix, so the source is the thread and the review that opened it: the loop prints nothing on success or on timeout, and the 60-second listener can expire before a manually entered tracker command runs; the reviewer asks to report readiness or timeout and to keep the listener alive until the tracker attempt finishes. The comment has no severity; `Minor` is inferred. It requests what F6 requested (an explicit report of a failed wait) and adds the listener lifetime, which the fix for F6 also removes.
- Solution: Fixed by the change of F6, which was already pushed when the comment was read: the loop and the 60-second listener are replaced by one script that holds the port while it runs the tracker and prints the exit code. The reply opens with the `Superseded by F6` form because no change was made for this comment; the row is `FIXED` because the concern is fixed in the tree, by F6 (F12).
- Current-tree verification: Same as F6: `grep -c "seq 50" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` prints 0 and `grep -c "subprocess.run"` on the same file prints 1.
- Resolution reference: docs(issues): [#2766] replace the readiness wait of the reproduction setup with one script that holds the port
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716817327

### F8 - The generated summary block of the description claims behavior this head does not ship

- PR number: 3083
- Source review ID: 6087228439
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718633051
- Concern: The block "Summary by CodeRabbit" at the end of the description lists JSON on stderr and JSON failure records as new features; this head ships neither, and the merge tool copies the description into the merge commit.
- Solution: The block, which CodeRabbit appended, is removed from the description, and the description ends with a paragraph saying what this pull request does not change. The fix is outside the tree. CodeRabbit may append the block again, so the description is checked before every hand-over, and the maintainer is told, because the setting and the merge tool are his.
- Current-tree verification: `gh pr view 3083 --json body` after the edit: `grep -c "Summary by CodeRabbit"` on the body prints 0.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718795006
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718795006

### F9 - The log crate maximum is said to stay at info after the configuration is applied

- PR number: 3083
- Source review ID: 6087228439
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718633062
- Concern: `try_init` installs the bridge of the `log` crate with the level of that moment (`info`), and `Controls::apply` was said never to raise it, so `trace_filter = "debug"` would drop the `log` records of dependencies; no test could catch it.
- Solution: The behavior was already right: `reload::Handle::reload` sets the maximum of the `log` crate after every reload (`tracing-subscriber` 0.3.23, `reload.rs`, function `modify`). Checked by running: `log::max_level()` reads `Debug` after `debug` is applied over a bridge installed at `info`. No production code changed except a comment at `try_init`. The test the reviewer suggested is added and pins the dependency behavior; `log` and `tracing-log` are development dependencies.
- Current-tree verification: `cargo test --lib bootstrap::logging` passes 19 tests, among them `it_should_pass_the_debug_records_of_the_log_crate_when_the_configuration_asks_for_them`.
- Resolution reference: test(bootstrap): [#2766] pin that a level change also reaches the records of the log crate
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718795360

### F10 - The claim that the lines of every style are unchanged does not hold

- PR number: 3083
- Source review ID: 6087228439
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718633069
- Concern: The records written before the configuration is applied use the `full` formatter whatever `trace_style` says, so a `json` stream now opens with a text line and `pretty` and `compact` gain a line in another layout; `Logging initialized` has a new target; the recheck ran `full` only. The claim is in the evidence, the description, the progress log of `ISSUE.md` and a commit body.
- Solution: The evidence limits the comparison to `full`, names the new target, and has a section R1b with a run of the four styles. The progress-log entry and the description are corrected. The commit body stays, because history is kept; this record says it was wrong.
- Current-tree verification: `grep -n "R1b" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` finds the section; `grep -c "The lines are those of R0b and R0c"` on the same file prints 0.
- Resolution reference: docs(issues): [#2766] record the four styles after the logging is installed first and correct the claim of unchanged output
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718795708

### F11 - Every span is formatted by four formatters, also on the request path, without a measurement

- PR number: 3083
- Source review ID: 6087228439
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718633081
- Concern: All four layers were told about every span for the whole life of the process, so each span stored its fields four times; the UDP request path opens several `info` spans per request and the benchmark configuration would not show the cost.
- Solution: The reviewer's second option: until the style is applied every formatter is told; from then on only the applied one. The style is applied once, and a later, different style is refused with a warning. Tests cover both phases, the kept first style, and that the first application does not warn (a run of the real binary showed the first version warned on every start).
- Current-tree verification: `cargo test --lib bootstrap::logging` passes the tests `it_should_tell_every_style_about_a_span_created_before_the_style_is_known` and `it_should_tell_only_the_active_style_about_a_span_created_after_the_style_is_applied`; the four-style run of R1b has no warning record.
- Resolution reference: perf(bootstrap): [#2766] tell only the applied style about the spans created after it is applied
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718796075

### F12 - The row of F7 and the reply it cites disagree on the disposition

- PR number: 3083
- Source review ID: 6087228439
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718633087
- Concern: The row of F7 is `FIXED` / `RESOLVED` while its reply opens with the `Superseded by` form that the skill pairs with `NO_ACTION` / `SUPERSEDED`.
- Solution: The row keeps `FIXED` / `RESOLVED` as a re-raise of F6, and its detail entry now says why the reply uses the other form.
- Current-tree verification: The detail entry of F7 in this file ends its Solution with the sentence that explains the reply form.
- Resolution reference: docs(pr-reviews): [#2766] record the review of da2ce7 on pull request 3083
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718796424

### F13 - Two live specs still cite the logging setup that this change deletes

- PR number: 3083
- Source review ID: 6087228439
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718633096
- Concern: `docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` and `docs/issues/drafts/2387-si-22-run-id-on-every-record/ISSUE.md` cite `packages/configuration/src/logging.rs` and the crate's `setup`.
- Solution: Each has a sentence, next to the old text, saying the setup moved to `src/bootstrap/logging.rs` and the file is deleted.
- Current-tree verification: `grep -c "src/bootstrap/logging.rs"` prints 1 for each of the two files.
- Resolution reference: docs(issues): [#2766] note in two live specs that the logging setup moved to the tracker application
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718796760

### F14 - The bootstrap docs still say the setup initializes the logging

- PR number: 3083
- Source review ID: 6087228439
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718633100
- Concern: The module doc of `src/bootstrap/app.rs` ("3. Initialize logging.") and the Bootstrap Flow of `src/AGENTS.md` describe the old order.
- Solution: The module doc says step 3 applies the logging configuration; `src/AGENTS.md` shows `bootstrap::logging::install()` first under `main()`, the new role of `initialize_global_services()`, and a file-map row for `bootstrap/logging.rs`.
- Current-tree verification: `grep -n "logging::install" src/AGENTS.md` finds the line of the flow; `grep -c "3. Initialize logging" src/bootstrap/app.rs` prints 0.
- Resolution reference: docs(bootstrap): [#2766] show in the bootstrap docs that the entry point installs the logging
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718797103

### F15 - The test named for the full style only checks that the line is not JSON

- PR number: 3083
- Source review ID: 6087228439
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718633105
- Concern: `!line.starts_with('{')` also holds for `compact` and `pretty`.
- Solution: The test writes the record inside a span with a field and asserts `outer{answer=42}:` before the message, which only `full` writes.
- Current-tree verification: `grep -n "it_should_write_the_span_and_its_fields_before_the_message_in_the_full_style" src/bootstrap/logging.rs` finds the test.
- Resolution reference: test(bootstrap): [#2766] assert the layout that only the full style writes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718797410

### F16 - The quoted red output is not what the committed tests print

- PR number: 3083
- Source review ID: 6087228439
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718633110
- Concern: The evidence quoted the messages of an earlier wording of the two instrumentation tests.
- Solution: The block quotes the current output, taken by restoring the instrumentation, and a sentence gives the earlier wording.
- Current-tree verification: `grep -c "a span records the configuration" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` prints 1.
- Resolution reference: docs(issues): [#2766] record the four styles after the logging is installed first and correct the claim of unchanged output
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4718797878

## Processing Log

- 2026-11-27 06:53 UTC - Review 6084545579 (`claude[bot]`) arrived; it has no finding.
- 2026-11-27 07:00 UTC - CodeRabbit's review 6084598281 arrived with F1 to F3, on the first head.
- 2026-11-27 07:06 UTC - Pushed the three fix commits.
- 2026-11-27 07:06 UTC - Replied on the three threads of F1 to F3.
- 2026-11-27 07:07 UTC - A GraphQL listing showed the three threads resolved, each with my reply as its last comment.
- 2026-11-27 07:07 UTC - Created this record from the template. It was started after the replies were posted, not before the first reply as `process-pr-review` requires; the late start is logged here.
- 2026-11-27 07:09 UTC - CodeRabbit's review 6084666893 arrived: a reply on the thread of F1 that confirms the ADR fix and asks for nothing more (no row).
- 2026-11-27 07:12 UTC - CodeRabbit's review 6084688285 arrived: a reply on the thread of F2 with F4.
- 2026-11-27 07:15 UTC - Committed the fix for F4.
- 2026-11-27 07:18 UTC - Pushed it and replied on the thread of F2; a GraphQL listing then showed the thread resolved with my reply as its last comment, and the three threads of this pull request resolved.
- 2026-11-27 07:19 UTC - Added F4 to this record.
- 2026-11-27 07:19 UTC - CodeRabbit's reply on the thread of F2 (comment 4716655322, review 6084749816) confirmed the setup fix and asks for nothing more (no row). The poller reported it with the next event.
- 2026-11-27 07:21 UTC - CodeRabbit's review 6084763933 arrived with F5, on the head of the F4 fix.
- 2026-11-27 07:23 UTC - Committed the fix for F5.
- 2026-11-27 07:27 UTC - Pushed it and replied on the thread of F5.
- 2026-11-27 07:28 UTC - Added F5 to this record.
- 2026-11-27 07:28 UTC - Resolved the thread of F5, whose last comment was my reply. A GraphQL listing shows no unresolved thread.
- 2026-11-27 07:30 UTC - CodeRabbit's review 6084834974 arrived with F6, on the head of the F5 fix.
- 2026-11-27 07:34 UTC - Committed the fix for F6.
- 2026-11-27 07:38 UTC - Pushed it and replied on the thread of F6.
- 2026-11-27 07:38 UTC - Added F6 to this record and resolved its thread, whose last comment was my reply. A GraphQL listing shows no unresolved thread.
- 2026-11-27 07:43 UTC - Read CodeRabbit's reply on the thread of F5 (comment 4716742352, review 6084854408), posted at 07:32 about the head of the F5 fix, after the F6 fix was pushed. Replied on the thread of F5 that the loop and the listener it is about no longer exist (F7, a re-raise of F6). The thread stayed resolved.
- 2026-11-27 07:44 UTC - Added F7 to this record.
- 2026-11-27 07:44 UTC - The maintainer commented `ACK` on the head before the F7 row.
- 2026-11-27 08:42 UTC - Pushed the F7 row, after the maintainer chose that over merging without it.
- 2026-11-27 11:08 UTC - da2ce7's review 6087228439 arrived (changes requested) with F8 to F16.
- 2026-11-27 11:18 UTC - Committed the code fixes for F11, F15 and F9, after a first attempt that the pre-commit hook refused for the nightly import formatting.
- 2026-11-27 11:21 UTC - Edited the pull request description (F8, F10).
- 2026-11-27 11:22 UTC - Committed the documentation fixes for F14, F13, F10 and F16.
- 2026-11-27 11:25 UTC - Pushed the six fix commits.
- 2026-11-27 11:26 UTC - Replied on the nine threads of F8 to F16.
- 2026-11-27 11:27 UTC - Added F8 to F16 to this record.
- 2026-11-27 11:28 UTC - Resolved the nine threads of F8 to F16, each with my reply as its last comment. A GraphQL listing shows no unresolved thread.

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
