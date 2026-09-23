---
pr-number: 2322
pr-url: https://github.com/torrust/torrust-tracker/pull/2322
last-updated-utc: "2026-09-23 17:25"
---

# PR #2322 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2322.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Review 5293086967 (Copilot, 15:30 UTC) provided the IDs `F1`, `F2`, `F3`, and `PR2322-003`.
Review 5293174934 (da2ce7, 15:37 UTC) reused `F1` to `F7` for different findings, so its rows
carry audit-local IDs assigned in source order with the reviewer's ID in the detail entry.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2322-f1` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2322-f2` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2322-f3` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| PR2322-003 | `review-finding:pr-2322-pr2322-003` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2322-f4` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2322-f5` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2322-f6` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2322-f7` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2322-f8` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2322-f9` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2322-f10` | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Use a numeric `related-pr` value in the #2318 spec

- PR number: 2322
- Source review ID: 5293086967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084213340
- Concern: `related-pr` held a GitHub URL while the frontmatter schema defines it as `<number|null>`.
- Solution: Set `related-pr: 2322`.
- Current-tree verification: `rg -n '^related-pr:' docs/issues/open/2318-2278-port-review-thread-scripts-to-rust/ISSUE.md` prints `related-pr: 2322`.
- Resolution reference: docs(issues): record review-thread parity deviations and checkpoints
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085192858

### F2 - Pipe stdout in the `fetch-review-threads` examples

- PR number: 2322
- Source review ID: 5293086967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084213275
- Concern: The documented `fetch`, `show`, and `list` commands left stdout on the terminal, so copying them produced `tty_refusal` instead of the operation.
- Solution: Every example now pipes stdout through `jq`, and the skill states the TTY refusal and the `--help` behaviour.
- Current-tree verification: `rg -n 'cargo run --package github-review-threads' -A2 .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md` shows each command ending in a `| jq` pipe.
- Resolution reference: docs(pr-reviews): pipe review-thread tool output in skill examples
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085193214

### F3 - Redirect stdout in the `reply-status` gate example

- PR number: 2322
- Source review ID: 5293086967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084213310
- Concern: The batch-resolution gate command was documented with stdout on the terminal, so the TTY guard returned exit code `2` before the documented `0`/`1` result.
- Solution: The example redirects stdout to `/dev/null` and the text says why.
- Current-tree verification: `rg -n '/dev/null' .github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md` matches the `reply-status` example.
- Resolution reference: docs(pr-reviews): pipe review-thread tool output in skill examples
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085193701

### PR2322-003 - Record the required `--login` argument as a deliberate deviation

- PR number: 2322
- Source review ID: 5293086967
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084213223
- Concern: The retired script defaulted `--login` to the authenticated `gh` user; the binary requires it, and the parity record did not say so.
- Solution: Kept the explicit argument, which keeps the guard deterministic and testable without a second `gh` call, and recorded the deviation with its rationale in the #2318 spec's Architectural Decisions.
- Current-tree verification: `rg -n 'reply-status --login. is required' docs/issues/open/2318-2278-port-review-thread-scripts-to-rust/ISSUE.md` finds the recorded decision.
- Resolution reference: docs(issues): record review-thread parity deviations and checkpoints
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085194165

### F4 - Keep the per-thread rows when `reply-status` finds a missing reply

- PR number: 2322
- Source review ID: 5293174934
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084289243
- Concern: On the missing-reply path the binary emitted only a generic diagnostic, so the operator could not tell which thread to answer, contradicting AC3 and the recorded captures.
- Solution: The `missing_reply` stderr record now carries the `threads` rows and `summary` counts and names the count and login in its message; stdout stays empty and the exit code stays `1`.
- Current-tree verification: `cargo test --package github-review-threads --test cli` passes `it_should_name_the_threads_without_a_reply_and_keep_the_retired_exit_status`, which compares the record's rows with `reply-status.stdout` and its exit code with `reply-status.exit-status`; the test fails when the rows are removed from the record.
- Resolution reference: fix(dev-tools): report missing-reply threads in the diagnostic
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085194654

### F5 - Make the recorded shell outputs a read parity reference

- PR number: 2322
- Source review ID: 5293174934
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084289250
- Concern: No test read `tests/fixtures/expected/`, and four files there recorded output the binary no longer produces.
- Solution: Moved the captures to `tests/fixtures/retired-scripts/` with a README naming each source script and which files the tests read; `tests/cli.rs` compares `list`, `show`, and `reply-status` data with them. The `fetch` captures stay documented as unread because they need `gh`.
- Current-tree verification: `rg -n 'retired-scripts' contrib/dev-tools/github/github-review-threads/tests/cli.rs` shows the four `include_str!` reads; `ls contrib/dev-tools/github/github-review-threads/tests/fixtures/` shows no `expected/`.
- Resolution reference: test(dev-tools): pin the review-thread CLI to retired-script captures
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085195020

### F6 - Emit one JSON object from `list` and `show`

- PR number: 2322
- Source review ID: 5293174934
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084289256
- Concern: `list` and `show` wrote a top-level JSON array where the ADR requires exactly one JSON object.
- Solution: Both return `{"threads":[...]}`; the skill documents the shape and the `jq '.threads[]'` filter, and the spec records the deviation.
- Current-tree verification: `cargo run --quiet --package github-review-threads -- list --threads-file contrib/dev-tools/github/github-review-threads/tests/fixtures/review-threads.json | jq -e 'type == "object" and (.threads | length == 2)'` prints `true`.
- Resolution reference: fix(dev-tools): restore thread_id and wrap list and show results; docs(pr-reviews): pipe review-thread tool output in skill examples
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085195390

### F7 - Keep `thread_id` and record the `reply-status` summary shape

- PR number: 2322
- Source review ID: 5293174934
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084289266
- Concern: `reply-status` renamed the script's `thread_id` key to `id` and replaced the `summary` marker line with a nested object without recording either.
- Solution: Restored `thread_id`; the nested `summary` object is recorded as a deliberate deviation beside the `show` deviation in the spec.
- Current-tree verification: `rg -n 'thread_id' contrib/dev-tools/github/github-review-threads/src/lib.rs` shows the serialized field; the `reply-status` CLI test compares rows with the capture key for key.
- Resolution reference: fix(dev-tools): restore thread_id and wrap list and show results; docs(issues): record review-thread parity deviations and checkpoints
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085195733

### F8 - Reconcile the spec's Workflow Checkpoints with the recorded work

- PR number: 2322
- Source review ID: 5293174934
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084289270
- Concern: Five checkpoints stayed unchecked although the task table, acceptance criteria, and evidence file recorded the work, and `last-updated-utc` predated the last log line.
- Solution: Checked the five completed checkpoints and refreshed `last-updated-utc`; reviewer, committer, and closure checkpoints stay open.
- Current-tree verification: `rg -n '^- \[ \]' docs/issues/open/2318-2278-port-review-thread-scripts-to-rust/ISSUE.md` lists only the reviewer, agent-review-report, committer, and closure checkpoints.
- Resolution reference: docs(issues): record review-thread parity deviations and checkpoints
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085196196

### F9 - Test the command-line layer

- PR number: 2322
- Source review ID: 5293174934
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084289279
- Concern: No test exercised argument parsing, exit codes, the stdout/stderr split, or the missing-reply path, and the `--help` behaviour was unrecorded.
- Solution: Added `tests/cli.rs` with five process-level tests over the built binary (`list`, `show`, `reply-status` missing and present, `--help`), and recorded the `--help` and TTY-before-parse behaviour in the skill and spec. TTY refusal itself needs a pseudo-terminal and stays covered by manual evidence V4.
- Current-tree verification: `cargo test --package github-review-threads` reports 5 unit and 5 CLI tests passing.
- Resolution reference: test(dev-tools): pin the review-thread CLI to retired-script captures
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085196757

### F10 - Route `emit_json` failures through the diagnostic path

- PR number: 2322
- Source review ID: 5293174934
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4084289286
- Concern: A serialization failure exited `1` with no stderr record and possibly partial stdout, and a failed newline write still returned success.
- Solution: `emit_json` serializes into a buffer with the trailing newline first, writes it in one call, and maps either failure to an `output_error` stderr record with exit code `1`.
- Current-tree verification: `rg -n 'output_error|to_vec|write_all' contrib/dev-tools/github/github-review-threads/src/main.rs` shows the buffered write and both error branches.
- Resolution reference: fix(dev-tools): report missing-reply threads in the diagnostic
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2322#discussion_r4085197067

## Processing Log

- 2026-09-23 16:50 UTC - Started audit; fetched GraphQL review threads with `github-review-threads fetch` and the two submitted reviews. Eleven unresolved inline threads: four from Copilot review 5293086967 and seven from da2ce7 review 5293174934; the review bodies summarize the inline findings and add no independent request.
- 2026-09-23 17:10 UTC - Normalized eleven findings; da2ce7's `F1`-`F7` collide with Copilot's IDs and were assigned `F4`-`F10` in source order. Fixed all eleven across five signed commits and appended re-verification evidence V6 to the #2318 evidence file.
- 2026-09-23 17:20 UTC - Pushed the fixes, replied on all eleven threads, and ran `validate-audit-record.py --pr-number 2322 --base torrust/develop`: `rows: 10, failures: 0`. The validator matches only `F<n>` IDs and skipped the `PR2322-003` row; its review ID, `[Minor]` bracket, same-thread reply, and commit subject were checked by hand.
- 2026-09-23 17:25 UTC - `github-review-threads reply-status --login josecelano` on a fresh fetch reported `11/11/0`; resolved all eleven threads with `resolve-all-unresolved-threads.sh`; a final fetch and `list` report zero unresolved threads.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
