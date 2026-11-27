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
- Solution: Fixed by the change of F6, which was already pushed when the comment was read: the loop and the 60-second listener are replaced by one script that holds the port while it runs the tracker and prints the exit code.
- Current-tree verification: Same as F6: `grep -c "seq 50" docs/issues/open/2766-2758-daemon-json-logging/manual-verification-evidence.md` prints 0 and `grep -c "subprocess.run"` on the same file prints 1.
- Resolution reference: docs(issues): [#2766] replace the readiness wait of the reproduction setup with one script that holds the port
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3083#discussion_r4716817327

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
