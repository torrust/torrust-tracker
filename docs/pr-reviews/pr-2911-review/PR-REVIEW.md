---
pr-number: 2911
pr-url: https://github.com/torrust/torrust-tracker/pull/2911
last-updated-utc: "2026-11-17 18:02"
---

<!-- skill-link: process-pr-review -->

# PR #2911 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2911.

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

Round 1: da2ce7 review 5943211502 (APPROVED) with findings F1 to F6, all non-blocking; the reviewer's IDs are this audit's IDs. Copilot review 5943096341 (COMMENTED) reported no finding. Round 2: Copilot review 5943750223 (COMMENTED, on the head that fixed round 1) with findings R1 and R2, which Copilot rates medium; this audit maps that to `Minor (inferred)`, since the comments carry no severity bracket. The `R` IDs are this audit's.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2911-f1 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2911-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2911-f3 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2911-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2911-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2911-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2911-r1 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2911-r2 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A second root-crate version requirement missing from the release bumps

- PR number: 2911
- Source review ID: 5943211502
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603174960
- Concern: Steps 2 and 9 of the `release-new-version` skill named only the `torrust-tracker` requirement of `packages/e2e-tools/Cargo.toml`; `packages/hung-component-tracker/Cargo.toml` adds a second one, and a requirement left behind at a bump stops the workspace from resolving.
- Solution: Both steps name the two manifests and give the command that lists every manifest requiring the root crate; the skill version is 1.1. The `version` key stays, following `e2e-tools`.
- Current-tree verification: `grep -rl --include=Cargo.toml '^torrust-tracker = ' packages` lists the two manifests; `grep -c 'hung-component-tracker/Cargo.toml' .claude/skills/release-new-version/SKILL.md` prints 2.
- Resolution reference: `docs(skills): [#2847] name every root-crate requirement in the release version bumps`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603316548

### F2 - The stale-build surprise is not in the evidence

- PR number: 2911
- Source review ID: 5943211502
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603174975
- Concern: The spec's progress log said the stale library build in the shared target directory is recorded in the evidence, but `manual-verification-evidence.md` did not mention it, and the M4 logs cannot show by themselves which binaries ran.
- Solution: The evidence's Failures and Follow-up section records what was stale, how it was found and fixed, and how each binary copy of V1 to V4 was confirmed: copy order, sizes, and `nm` finding `run_until_shutdown_signal` in the two branch binaries only.
- Current-tree verification: `grep -n 'Stale build in the shared target directory' manual-verification-evidence.md` finds the entry; `nm -C .tmp/m/torrust-tracker-before | grep -c 'torrust_tracker_lib::app::run_until_shutdown_signal'` printed 0, and 4 for `torrust-tracker-after` and `hung-component-tracker`.
- Resolution reference: `docs(issues): [#2847] record the stale build and correct the discarded M4 run's cause`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603316968

### F3 - The discarded M4 run's stated cause cannot produce its order

- PR number: 2911
- Source review ID: 5943211502
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603174988
- Concern: The run script takes the "sending SIGTERM" stamp before `kill`, so lagging `date` stamps cannot place the tracker's shutdown line before it; load explains neither order.
- Solution: The evidence records the cause as unknown and notes the readiness stamp was off the same way; the rerun stands on its own committed logs. The spec's 16:47 progress-log entry, which repeated the wrong cause, is corrected in place with a note, and a new entry logs this round.
- Current-tree verification: `grep -c 'lagged by about 0.4 s' manual-verification-evidence.md` prints 0; `grep -n 'The cause is' manual-verification-evidence.md` finds the unknown-cause sentence; `grep -n 'Corrected in review round 1 of PR #2911' ISSUE.md` finds the note.
- Resolution reference: `docs(issues): [#2847] record the stale build and correct the discarded M4 run's cause`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603317375

### F4 - Agent instructions and the shutdown inventory are stale

- PR number: 2911
- Source review ID: 5943211502
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603175004
- Concern: `src/AGENTS.md` still put the signal wait and job cancellation in `main.rs` and described `app.rs` as startup only, `packages/AGENTS.md` did not list the new crate among the developer tools, and `task-inventory.md` cited `src/main.rs` for holding the container past `wait_for_all`.
- Solution: `src/AGENTS.md` places the shutdown sequence in `app::run_until_shutdown_signal`, `packages/AGENTS.md` lists `hung-component-tracker`, and `task-inventory.md` names `run_until_shutdown_signal` in `src/app.rs`; its dated recheck row keeps its original result and adds where the hold lives since #2847.
- Current-tree verification: `grep -n 'run_until_shutdown_signal' src/AGENTS.md docs/features/shutdown-process/task-inventory.md` finds the new text; `grep -n 'hung-component-tracker' packages/AGENTS.md` finds the list entry.
- Resolution reference: `docs(agents): [#2847] place the shutdown sequence in app.rs and list the new crate`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603317780

### F5 - No `# Panics` section on the new public function

- PR number: 2911
- Source review ID: 5943211502
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603175014
- Concern: `run_until_shutdown_signal` reaches the `expect`s of `wait_for_shutdown_signal`, but as public library API it did not document its panics, and `clippy::missing_panics_doc` does not look through the call.
- Solution: Added a `# Panics` section naming a signal handler that cannot be installed and a `SIGTERM` stream that closes before a signal arrives.
- Current-tree verification: `grep -n '# Panics' -A4 src/app.rs` shows the section above `run_until_shutdown_signal`.
- Resolution reference: `docs(app): [#2847] document the panics of run_until_shutdown_signal and the test CLI's flag`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603318272

### F6 - The test CLI comment overstates the match with the shipped flag

- PR number: 2911
- Source review ID: 5943211502
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603175025
- Concern: The comment said the test binary has "the same configuration option as the shipped executable", but it lacks the shipped `parse_non_empty_path` check that rejects an empty path.
- Solution: The comment now says the flag lacks the shipped empty-path check, which the tests do not need. The shipped parser stays private to the shipped `main.rs`.
- Current-tree verification: `grep -n 'without its check that' packages/hung-component-tracker/src/main.rs` finds the comment.
- Resolution reference: `docs(app): [#2847] document the panics of run_until_shutdown_signal and the test CLI's flag`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603318782

### R1 - The readiness poll copies the whole output each time

- PR number: 2911
- Source review ID: 5943750223
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603593852
- Concern: `wait_until_signal_handlers_installed` called `self.output()` on every 10 ms poll, cloning the whole captured output each time, so the loop's cost grows with the log.
- Solution: The poll calls a new `output_contains`, which checks `contains` under the mutex guard without cloning; the output is cloned only for a panic message or the final `ChildExit`.
- Current-tree verification: `grep -n 'output_contains' packages/hung-component-tracker/tests/common/mod.rs` finds the helper and its one caller in the readiness loop; `cargo test -p torrust-tracker-hung-component-tracker` passed both cases after the change.
- Resolution reference: `test(hung-component-tracker): [#2847] poll the child output without copying it and fail on a reader panic`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603682434

### R2 - Reader-thread panics are discarded

- PR number: 2911
- Source review ID: 5943750223
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603593928
- Concern: `join_readers` dropped each `JoinHandle::join()` result, so a reader panic would surface only later, as missing log lines in an assertion message.
- Solution: `join_readers`, used on the normal and the timeout paths, asserts each join is `Ok` with a direct message. `Drop` keeps ignoring the results, because it also runs while a failed assertion unwinds and a second panic would abort the test process.
- Current-tree verification: `grep -n 'an output reader thread panicked' packages/hung-component-tracker/tests/common/mod.rs` finds the assertion; `grep -n 'must not panic again' packages/hung-component-tracker/tests/common/mod.rs` finds the `Drop` comment.
- Resolution reference: `test(hung-component-tracker): [#2847] poll the child output without copying it and fail on a reader panic`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2911#discussion_r4603682816

## Processing Log

- 2026-11-17 17:30 UTC - Round 1: da2ce7 review 5943211502 (APPROVED, ACK on the head before this round) with findings F1 to F4 (Minor) and F5, F6 (Nit), none blocking; Copilot review 5943096341 (COMMENTED) had no finding and no inline comment. The maintainer chose to fix all six in this PR. Committed the fixes in four commits (17:24:28Z to 17:28:11Z), then, before pushing so that one push carries the fixes and this record, replied to the six threads (17:29:25Z to 17:29:36Z), each naming the commit subject the push brings, and resolved them (17:29:58Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Created this record.
- 2026-11-17 18:02 UTC - Round 2: Copilot review 5943750223 (COMMENTED, 17:54:07Z) on the head that fixed round 1, after both ACKs of that head, with findings R1 and R2. The maintainer chose to fix both with a new push. Committed the fix (18:01:19Z), replied to both threads (18:01:55Z and 18:01:58Z) naming the commit subject the push brings, and resolved them (18:02:08Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Recorded R1 and R2.

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
