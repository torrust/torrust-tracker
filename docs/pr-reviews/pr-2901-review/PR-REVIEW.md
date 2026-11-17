---
pr-number: 2901
pr-url: https://github.com/torrust/torrust-tracker/pull/2901
last-updated-utc: "2026-11-17 10:55"
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

## Processing Log

- 2026-11-17 10:39 UTC - Reviews 5936922508 (da2ce7, submitted 10:38:51) and 5936930487 (Copilot, submitted 10:39:28) arrived on the first head.
- 2026-11-17 10:49 UTC - Pushed the fixes of round 1 (the head commit time is 10:49:18).
- 2026-11-17 10:54 UTC - Replied on all twelve threads (the reply times are 10:53:50 to 10:54:15) and resolved them; `reply-status` reported `all_replied` before the resolution.
- 2026-11-17 10:55 UTC - Created this audit record after the replies and the resolution (a late start: the fixes had to be pushed first so the replies could cite their commit subjects).

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
