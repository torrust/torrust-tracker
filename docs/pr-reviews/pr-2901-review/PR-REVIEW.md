---
pr-number: 2901
pr-url: https://github.com/torrust/torrust-tracker/pull/2901
last-updated-utc: "2026-11-17 15:26"
---

<!-- skill-link: process-pr-review -->

# PR #2901 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2901.

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

Round 1, on the head that carries the commit `docs(issues): [#2765] record the implementation and manual verification`: da2ce7 review 5936922508 (APPROVED) with findings F1 to F9 (five Minor, three Nit, one Suggestion), and Copilot review 5936930487 (COMMENTED) with three threads that carry the reviewer IDs F1, F6 and F2. Those IDs collide with da2ce7's, so they are recorded here as F10 (Copilot F1), F11 (Copilot F6) and F12 (Copilot F2), in source order; none is a re-raise of a da2ce7 finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2901-f1 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2901-f2 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2901-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2901-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2901-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2901-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2901-f7 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2901-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2901-f9 | Human | Suggestion | maintainability | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2901-f10 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2901-f11 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2901-f12 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2901-f13 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2901-f14 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2901-f15 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2901-f16 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2901-f17 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2901-f18 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2901-f19 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2901-f20 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2901-f21 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2901-f22 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2901-f23 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2901-f24 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - No test guards the capture itself

- PR number: 2901
- Source review ID: 5936922508
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598406927
- Concern: Nothing asserted the captured output: with `Stdio::inherit()` instead of `Stdio::piped()` all six tests stayed green, although the capture is what the spec asks for.
- Solution: `execute` returns the captured stdout and stderr and the tests assert them (including 300000 `x` and 300000 `y`).
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 10 tests; with both `Stdio::piped()` replaced by `Stdio::inherit()` 4 of the 10 failed (mutation run, reverted).
- Resolution reference: `fix(e2e): [#2765] bound the checker run, build before it and test the capture`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598568001

### F2 - The kill test asserts only the error kind

- PR number: 2901
- Source review ID: 5936922508
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598406941
- Concern: With the `child.kill()` of the deadline path removed the test still passed, about 60 s later.
- Solution: The test asserts the elapsed time is under 20 s besides the error kind.
- Current-tree verification: With the `child.kill()` of `wait_with_deadline` removed, `it_should_kill_the_child_and_return_promptly_when_the_deadline_expires` failed after 60.0 s (mutation run, reverted); with the code as committed it passes in the 10-test run.
- Resolution reference: `fix(e2e): [#2765] bound the checker run, build before it and test the capture`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598568453

### F3 - The AC2 evidence does not hold at this head

- PR number: 2901
- Source review ID: 5936922508
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598406951
- Concern: The AC2 row cited a grep that the progress log did not record, and the open spec of #2594 still told the reader to run `tracker_checker`.
- Solution: The #2594 spec names `tracker_client check` (commit `docs(issues): [#2594] name tracker_client check in the extraction spec`); the evidence file records the search command and its output (section V3) and the AC2 row cites it.
- Current-tree verification: The search recorded in V3 (`git grep -nE ...` with the exclusions listed there) printed nothing and exited 1 on the committed tree.
- Resolution reference: `docs(issues): [#2765] record the AC2 search, the V2 harness and the completion review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598568880

### F4 - V2 cannot be reproduced from this record

- PR number: 2901
- Source review ID: 5936922508
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598406968
- Concern: The throwaway harness and its command were not recorded, AC4 was checked on a runner-level claim the run did not support, and the PR body did not say the temporary test is not included.
- Solution: V2 records the harness verbatim and the command, was re-run on the new helper, and says it is a helper-level smoke test; AC4 is unchecked until the container job passes on the final head; the PR body says the temporary test is not included.
- Current-tree verification: Read the evidence file (section V2) and the AC4 row; the PR body was edited with `gh pr edit` and now has the "Not included" line.
- Resolution reference: `docs(issues): [#2765] record the AC2 search, the V2 harness and the completion review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598569245

### F5 - Required review records are missing

- PR number: 2901
- Source review ID: 5936922508
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598406977
- Concern: The design-review checkpoint, the prose-first Arrange-Act-Assert comparison and the Implementation Completion Review were missing, and the new tests had no AAA sections.
- Solution: The spec records the checkpoint (stating it was written after review), the comparison table and the completion review, with `implementation-retrospective.md`; the tests have AAA sections and assert error kinds (commit `fix(e2e): [#2765] bound the checker run, build before it and test the capture`).
- Current-tree verification: Read the spec sections and the test module; `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 10 tests.
- Resolution reference: `docs(issues): [#2765] record the AC2 search, the V2 harness and the completion review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598569648

### F6 - The comment holds only while no descendant keeps the pipes

- PR number: 2901
- Source review ID: 5936922508
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598406991
- Concern: The readers' join after a kill ran outside the deadline and the comment relied on an unstated assumption.
- Solution: The assumption is stated in the doc comment of `execute`, and the drain is bounded by a grace period so it does not depend on it.
- Current-tree verification: Read the doc comment; `it_should_not_wait_for_a_descendant_that_keeps_the_pipes_open` passes in the 10-test run.
- Resolution reference: `fix(e2e): [#2765] bound the checker run, build before it and test the capture`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598570040

### F7 - The frontmatter disagrees with the branch

- PR number: 2901
- Source review ID: 5936922508
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598407006
- Concern: `branch` and `related-pr` in the spec frontmatter did not match the PR.
- Solution: Frontmatter says `branch: "2765-remove-deprecated-client-bins"` and `related-pr: 2901`.
- Current-tree verification: Read lines 10 and 11 of the spec.
- Resolution reference: `docs(issues): [#2765] record the AC2 search, the V2 harness and the completion review`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598570468

### F8 - The README Output block does not match the command

- PR number: 2901
- Source review ID: 5936922508
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598407016
- Concern: `tracker_client check` prints one-line JSON by default while the unchanged Output block shows the pretty form.
- Solution: The README command and the checker doc examples pass `--format text`.
- Current-tree verification: Ran `target/debug/tracker_client check --format text` with the default config file: it printed the pretty form.
- Resolution reference: `docs(tracker-client): [#2765] use the unified flags in the migrated examples`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598570780

### F9 - The deadline also bounds the cargo build

- PR number: 2901
- Source review ID: 5936922508
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598407021
- Concern: A cold build on a loaded runner could fail the e2e step.
- Solution: `run` builds `tracker_client` first under a 15-minute deadline, then runs the check under a 5-minute deadline.
- Current-tree verification: Read `run` and the `BUILD_ARGS` test; V2 re-run logged both commands in order.
- Resolution reference: `fix(e2e): [#2765] bound the checker run, build before it and test the capture`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598571089

### F10 - Bound output draining as well as the child wait

- PR number: 2901
- Source review ID: 5936930487
- Reviewer finding ID: Copilot F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598413367
- Concern: If a descendant keeps stdout or stderr, the reader joins could block past the deadline; the process tree should be terminated and the drain bounded, with a regression test.
- Solution: The readers send chunks over a channel and the collector stops after a 5 s grace, keeping the output read so far; two regression tests cover a descendant holding the pipes. Terminating the process tree was not added: it needs a process group and a `kill` call (a new dependency), and on Unix `cargo run` replaces itself with the binary; the assumption is stated in the code.
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 10 tests, including both descendant tests.
- Resolution reference: `fix(e2e): [#2765] bound the checker run, build before it and test the capture`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598571413

### F11 - Migrate the HTTP example peer-address flag too

- PR number: 2901
- Source review ID: 5936930487
- Reviewer finding ID: Copilot F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598413423
- Concern: The HTTP all-options example passed `--peer-addr`, which the unified client names `--ip`.
- Solution: The example uses `--ip 10.0.0.1`.
- Current-tree verification: `grep -n "peer-addr" console/tracker-client/src/console/clients/http/app.rs` prints nothing.
- Resolution reference: `docs(tracker-client): [#2765] use the unified flags in the migrated examples`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598571817

### F12 - Replace the legacy format value in the migrated examples

- PR number: 2901
- Source review ID: 5936930487
- Reviewer finding ID: Copilot F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598413476
- Concern: The migrated examples kept `--format pretty`, which the unified client rejects (`json` or `text`).
- Solution: `--format pretty` became `--format text` in the UDP and HTTP examples.
- Current-tree verification: `grep -rn "format pretty"` over the two `app.rs` files prints nothing; `tracker_client udp announce --help` lists `json, text`.
- Resolution reference: `docs(tracker-client): [#2765] use the unified flags in the migrated examples`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598572312

### F13 - On the deadline path the output read so far is dropped

- PR number: 2901
- Source review ID: 5937298122
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598704325
- Concern: `execute` returned the error before the captured streams reached `Captured` and `run_command` logged only on success, so a hung checker ended with the timeout error and none of its output; the statement in the F10 solution of this audit that the output read so far is kept held only when the child exits.
- Solution: `execute` returns the captured output with the status or the error that ended the wait, and `run_command` logs both streams before it propagates the error; a test asserts the output written before a hang.
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 12 tests, including `it_should_return_the_output_written_before_the_deadline_expired`.
- Resolution reference: `fix(e2e): [#2765] keep the output on the deadline path and share one drain window`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598757185

### F14 - The drain bound is twice the grace period

- PR number: 2901
- Source review ID: 5937298122
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598704345
- Concern: the two `collect` calls each opened their own grace window, so the drain could take twice the documented bound.
- Solution: both calls share one end instant.
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 12 tests, including `it_should_share_one_drain_window_between_both_pipes`; no mutation run was done for that test.
- Resolution reference: `fix(e2e): [#2765] keep the output on the deadline path and share one drain window`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598757602

### F15 - Enforce the drain deadline before receiving queued chunks

- PR number: 2901
- Source review ID: 5937491189
- Reviewer finding ID: Copilot F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598857730
- Concern: Copilot labelled it a re-raise of its first finding (F10 here). The earlier change, bounding the drain by a grace period, is in the tree; this one asks for a different change in the new collector: `recv_timeout` with a zero timeout still returns queued chunks after the window ended, so a backlog or a descendant that keeps writing could extend the drain. Recorded as `ORIGINAL`, because the requested change differs from F10's.
- Solution: `collect` checks the end of the window before every receive; two tests cover a queued chunk after the window and a descendant that keeps writing.
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 14 tests; no mutation run was done for the new tests.
- Resolution reference: `fix(e2e): [#2765] enforce the drain window before receiving queued chunks`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598905282

### F16 - The prose-first record and the PR body still count ten tests

- PR number: 2901
- Source review ID: 5937498680
- Reviewer finding ID: da2ce7 F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598862732
- Concern: The spec table had no rows for two of the helper tests and the PR body said "ten unit tests", which was stale.
- Solution: The table has a row for every helper test and the PR body no longer states a count. The thread was resolved before it was replied to (see the log); the reply is late.
- Current-tree verification: Read the spec table (15 rows for the 15 tests of `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker`, which passes 15 tests); `gh pr edit` updated the body.
- Resolution reference: `docs(issues): [#2765] add the prose-first rows for the new helper tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599092929

### F17 - The bound leaves about 0.7 s of slack for a loaded runner

- PR number: 2901
- Source review ID: 5937498680
- Reviewer finding ID: da2ce7 F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4598862738
- Concern: `it_should_share_one_drain_window_between_both_pipes` expected about 1.1 s against a 1.8 s bound.
- Solution: The test uses a 3 s window and a 5 s bound (about 3.1 s expected, about 6.1 s with two windows). Resolved before replying (see the log).
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 15 tests, including the test.
- Resolution reference: `fix(e2e): [#2765] keep the queued output after the drain window and cap it`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599093325

### F18 - The check also drops output that was already read and queued for the second pipe

- PR number: 2901
- Source review ID: 5937712954
- Reviewer finding ID: da2ce7 F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599028233
- Concern: Both pipes share the window, so the second `collect` started after its end and discarded chunks that were already queued.
- Solution: After the window `collect` takes the chunks already queued, without waiting, up to an 8 MiB cap; a test with the reviewer's script asserts both streams, and the queued-chunk test now asserts the chunk is kept.
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 15 tests, including `it_should_keep_the_output_of_the_second_pipe_when_the_first_one_uses_the_whole_window`.
- Resolution reference: `fix(e2e): [#2765] keep the queued output after the drain window and cap it`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599093720

### F19 - An unbounded yes stream test keeps gigabytes in memory

- PR number: 2901
- Source review ID: 5937712954
- Reviewer finding ID: da2ce7 F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599028250
- Concern: The descendant test with `yes` pushed about 1.15 GB through the pipe and the helper kept it all.
- Solution: The test is removed and the helper keeps at most 8 MiB per stream; a test asserts the cap. This supersedes the part of the F15 solution that named the `yes` test.
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 15 tests, including `it_should_cut_the_output_at_the_byte_cap_even_when_more_is_queued`; no test of the helper starts `yes` any more.
- Resolution reference: `fix(e2e): [#2765] keep the queued output after the drain window and cap it`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599094074

### F20 - Round 3 also had review 5937498680, whose two threads have no reply and no row

- PR number: 2901
- Source review ID: 5937712954
- Reviewer finding ID: da2ce7 F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599028260
- Concern: Two threads (F16 and F17 here) were resolved without a reply and without rows, and the audit F15 reused a reviewer ID without recording it.
- Solution: Replies were posted on both threads, rows F16 and F17 were added, and the colliding IDs were mapped to the next free ones with the reviewer ID recorded; the audit F15 is the Copilot finding and the reviewer-provided IDs of this round are recorded in each entry.
- Current-tree verification: Read this audit and `validate-audit-record.py` output (ok).
- Resolution reference: `docs(issues): [#2765] add the prose-first rows for the new helper tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599094506

### F21 - The reader channel is unbounded while the cap applies only after the wait

- PR number: 2901
- Source review ID: 5937956316
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599222310
- Concern: `mpsc::channel()` for the reader threads is unbounded and `MAX_CAPTURED_BYTES` was applied only in `collect`, after the wait, so a child that writes without end could grow the queue during the wait. An inline thread (first recorded in this audit as a review-body finding, which was wrong; see the log).
- Solution: the reader thread queues at most `MAX_CAPTURED_BYTES` and goes on reading and discarding the rest, so memory is bounded and the child never blocks on a full pipe; a bounded channel was not used because the collector runs only after the wait, so a full queue would block the child.
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 16 tests, including `it_should_queue_at_most_the_byte_cap_from_a_pipe_that_writes_more`.
- Resolution reference: `fix(e2e): [#2765] cap the bytes the reader threads queue`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599404323
- Conversation comment (earlier answer): https://github.com/torrust/torrust-tracker/pull/2901#issuecomment-6947491371

### F22 - F21 is an inline thread, not a review-body finding

- PR number: 2901
- Source review ID: 5938165981
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599388323
- Concern: the audit recorded F21 as a review-body finding (review URL, `NON_RESOLVABLE`, a conversation comment as its reply) although it is an inline thread that stayed unresolved without a reply.
- Solution: replied on the thread and resolved it; the F21 row is now anchored on the discussion with its reply and `RESOLVED`, and the log has an appended correction.
- Current-tree verification: `validate-audit-record.py --pr-number 2901` with the F21 entry anchored on `r4599222310` returns ok.
- Resolution reference: `docs(pr-reviews): [#2765] audit review round 6 of PR 2901`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599680349

### F23 - Past the cap the reader never notices that the receiver is gone

- PR number: 2901
- Source review ID: 5938165981
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599388336
- Concern: past the cap the reader read and discarded without sending, so it could not notice the dropped receiver and a descendant that writes without end kept it busy.
- Solution: the reader sends an empty chunk once per MiB discarded and ends when the send fails.
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passes 17 tests, including `it_should_stop_reading_a_pipe_past_the_cap_when_the_receiver_is_gone`; with the probe removed that test failed (47 GB read after the drop; mutation run, reverted).
- Resolution reference: `fix(e2e): [#2765] end the reader thread when the receiver is gone past the cap`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599679621

### F24 - The byte-delta bound of the receiver-gone test races the reader

- PR number: 2901
- Source review ID: 5938701192
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4599775490
- Concern: the test asserted that the reader read at most about 1 MiB between two lines, but the reader reads that in tens of microseconds, so a pause of the test thread could fail the test with the code correct.
- Solution: first left as `NO_ACTION` on the maintainer's decision (a reply on the thread); then the Docker test job failed on this exact assertion (`read 34963456 bytes after the drop`), and the maintainer allowed a seventh round. The delta assertion is removed and the test keeps the check that the read count stops growing after the drop.
- Current-tree verification: `cargo test -p torrust-tracker --lib console::ci::e2e::tracker_checker` passed 17 tests in three runs; with the probe removed the test still failed (mutation run, reverted).
- Resolution reference: `test(e2e): [#2765] drop the racy byte-delta assertion of the receiver-gone test`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2901#discussion_r4601712393

## Processing Log

- 2026-11-17 10:39 UTC - Reviews 5936922508 (da2ce7, submitted 10:38:51) and 5936930487 (Copilot, submitted 10:39:28) arrived on the first head.
- 2026-11-17 10:49 UTC - Pushed the fixes of round 1 (the head commit time is 10:49:18).
- 2026-11-17 10:54 UTC - Replied on all twelve threads (the reply times are 10:53:50 to 10:54:15) and resolved them; `reply-status` reported `all_replied` before the resolution.
- 2026-11-17 10:55 UTC - Created this audit record after the replies and the resolution (a late start: the fixes had to be pushed first so the replies could cite their commit subjects).
- 2026-11-17 11:05 UTC - Round 2: da2ce7 review 5937298122 (APPROVED, submitted 11:05:49) with F13 (Minor) and F14 (Nit); the IDs are free in this audit.
- 2026-11-17 11:10 UTC - Fixed both in one commit, replied and resolved the two threads. Correction to the F10 entry: its Solution and the reply on that thread said the output read before a deadline is kept; that held only when the child exits, and F13 fixed the deadline path.
- 2026-11-17 11:20 UTC - Round 3: Copilot review 5937491189 (COMMENTED, submitted 11:20:08) with one thread; its reviewer ID F1 collides, so it is F15.
- 2026-11-17 11:20 UTC - Round 3 also had da2ce7 review 5937498680 (submitted 11:20:36, later shown as dismissed) with two threads (reviewer IDs F15 and F16). They were not processed in round 3: my batch resolution resolved them without a reply and without rows (a process error), and they were not in this audit.
- 2026-11-17 11:25 UTC - Fixed in one commit, replied and resolved the thread.
- 2026-11-17 11:37 UTC - Round 4: da2ce7 review 5937712954 (COMMENTED, submitted 11:37:44) with F17 to F19 of its own numbering, which with the two threads above give reviewer IDs F15 to F19; they are F16 to F20 here.
- 2026-11-17 11:45 UTC - Fixed in two commits, replied on the five threads and on the Copilot thread of F15 (a follow-up reply, because the fix changed what that reply said), resolved the three that were open. Correction to F15: its Solution and Verification name two tests, one of which (the `yes` descendant test) was removed in this round and the behaviour that test guarded was changed (queued chunks are kept up to a cap).
- 2026-11-17 11:57 UTC - Round 5: da2ce7 review 5937956316 (APPROVED, submitted 11:57:15) with one review-body finding, F21 (Minor); it has no thread, so the reply is a PR conversation comment.
- 2026-11-17 12:03 UTC - Fixed in one commit and replied on the conversation. This is fix round 5 of 5.
- 2026-11-17 12:14 UTC - Round 6: da2ce7 review 5938165981 (COMMENTED, submitted 12:13:59) with F22 (Minor) and F23 (Nit), after the five fix rounds; the maintainer allowed a sixth round.
- 2026-11-17 12:40 UTC - Fixed F23 in code, replied on both threads and on r4599222310 and resolved them. Correction to the 11:57 entry: F21 was an inline thread (r4599222310), not a review-body finding; its row is now discussion-anchored. Two batch resolutions earlier (rounds 3 and 6) resolved threads before they were replied to; both were reopened or answered afterwards.
- 2026-11-17 12:49 UTC - Round 7: da2ce7 review 5938701192 (APPROVED, submitted 12:49:30) with F24 (Suggestion); the maintainer first chose to leave it, and a reply with `NO_ACTION` was posted on the thread.
- 2026-11-17 13:56 UTC - The `Test (Docker) (release)` job of the head failed on the test of F24 in the image test stage (the failure time is the log time of the run).
- 2026-11-17 15:26 UTC - The maintainer asked for the fix; fixed in one commit, replied on the thread (which replaces the earlier `NO_ACTION` reply) and resolved it.

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
