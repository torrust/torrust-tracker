---
pr-number: 2983
pr-url: https://github.com/torrust/torrust-tracker/pull/2983
last-updated-utc: "2026-11-20 07:28"
---

<!-- skill-link: process-pr-review -->

# PR #2983 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2983.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the findings were processed before the merge).

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

The review 5988749356 of `da2ce7` (changes requested) has three inline findings, `F1` to `F3`; its
body lists the same three and adds no other request. The reviewer's own finding IDs are kept.
Copilot's review 5988828122, made on the first head, has four inline findings. Copilot labelled
them `F1`, `R1`, `F1` and `F2`. Three of those labels (`F1`, `F1` and `F2`) collide with the IDs
above; `R1` does not. This record names the four `R1` to `R4` in the order of their comments and
keeps Copilot's label in each detail. That departs from step 3 of `process-pr-review`, which keeps
a label that does not collide: Copilot's `R1` is the row `R2` here. A finding reference never
changes once assigned, so the IDs stay (see `F4`). The second review of `da2ce7`, 5988957491
(approved), has two inline findings, `F4` and `F5`; its body adds no other request.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2983-f1 | Human | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2983-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2983-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2983-r1 | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2983-r2 | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| R3 | review-finding:pr-2983-r3 | Copilot | Minor | testing | RE_RAISE_OF:R1 | FIXED | RESOLVED |
| R4 | review-finding:pr-2983-r4 | Copilot | Minor | testing | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F4 | review-finding:pr-2983-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2983-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The parent EPIC keeps the old path and `IN_PROGRESS` for order 5

- PR number: 2983
- Source review ID: 5988749356
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639466688
- Concern: The archive move left `docs/issues/open/2392-test-isolation/EPIC.md` naming the old path in `related-artifacts` and in subissue row 5, with `IN_PROGRESS`. The path names no tracked file after the merge, the frontmatter validator reports `missing-artifact`, and Steps 4 and 5 of `cleanup-completed-issues` ask for the EPIC update in the same change.
- Solution: `EPIC.md` names the `docs/issues/closed/...` path in both places, row 5 reads `DONE` with the pull requests of the remaining work, `last-updated-utc` is current and the progress log has an entry. The EPIC 2392 coordinator session, whose brief had excluded the file, agreed to the edit.
- Current-tree verification: `cargo run -q -p frontmatter-validator --bin frontmatter-validator -- docs/issues/open/2392-test-isolation/EPIC.md` printed no diagnostic; before the fix it printed one `missing-artifact` error. `grep -c 'closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md' docs/issues/open/2392-test-isolation/EPIC.md` printed 2.
- Resolution reference: `docs(issues): [#1419] mark order 5 done in EPIC #2392 and point it at the archived spec`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639495296

### F2 - The AC8 and AC9 rows narrow their scope without saying so

- PR number: 2983
- Source review ID: 5988749356
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639466695
- Concern: The closure review marked AC8 and AC9 as met for the eleven fixture-based targets only. "No suite starts the application another way" does not hold for `tests/`, where four targets start an executable as a child process, and the AC9 row cited no run of those four.
- Solution: The AC8 row says it is read for the fixture-based suites and names the four child-process targets as outside AC8 by D3 and `tests/AGENTS.md`. The AC9 row cites a run of those four, which passed, and says that all 15 main-level targets pass.
- Current-tree verification: `grep -c 'all 15 main-level' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1. `cargo test --test lifecycle-signals --test lifecycle-shutdown-policy --test cli-configuration --test http_health_check` exited with status 0 (`rustc 1.100.0`, `dev` profile).
- Resolution reference: `docs(issues): [#1419] state the scope of AC8 and AC9 in the closure review and drop two stale status remnants`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639495536

### F3 - Two stale status remnants in the archived record

- PR number: 2983
- Source review ID: 5988749356
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639466700
- Concern: The closed specification says every task is complete, but the historical plan still read `T9 | TODO` and a ticked checkpoint still said that cooperative server shutdown "remains deferred".
- Solution: T9 reads `DONE` and says it was superseded by R5, R8 and R9 of the completion plan. The checkpoint no longer has the "remains deferred" clause.
- Current-tree verification: `grep -c '^| T9  | DONE' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1 and `grep -c 'remains deferred)' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 0.
- Resolution reference: `docs(issues): [#1419] state the scope of AC8 and AC9 in the closure review and drop two stale status remnants`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639495789

### R1 - No run shows concurrency between test executables for AC2

- PR number: 2983
- Source review ID: 5988828122
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639530237
- Concern: Plain `cargo test` runs the selected test binaries one after another, and each fixture-based target has one test, so the cited invocation shows no overlap between suites.
- Solution: The AC2 and AC9 rows cite a `cargo nextest run` of all 15 main-level targets, which runs every test in its own process in parallel across binaries, and say that the plain Cargo runs are sequential.
- Current-tree verification: `grep -c '122 tests across 15 binaries' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1. `cargo nextest run -p torrust-tracker` with one `--test` argument for each of the 15 main-level targets exited with status 0 (`cargo-nextest 0.9.88`, `rustc 1.100.0`, `dev` profile).
- Resolution reference: `docs(issues): [#1419] cite a concurrent nextest run of all 15 main-level targets for AC2 and AC9`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639549090

### R2 - The evidence for AC2 and AC9 is neither concurrent nor complete

- PR number: 2983
- Source review ID: 5988828122
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639530277
- Concern: The same sequential-run concern as R1, and the cited command selected only the fixture-based targets although AC9 requires all current main-level targets.
- Solution: The concurrent nextest run covers the full set of 15 targets, including the four child-process targets.
- Current-tree verification: `grep -c '122 tests across 15 binaries' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1. `cargo nextest run -p torrust-tracker` with one `--test` argument for each of the 15 main-level targets exited with status 0 (`cargo-nextest 0.9.88`, `rustc 1.100.0`, `dev` profile).
- Resolution reference: `docs(issues): [#1419] cite a concurrent nextest run of all 15 main-level targets for AC2 and AC9`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639549364

### R3 - Cite concurrent execution evidence for the closure review

- PR number: 2983
- Source review ID: 5988828122
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639530305
- Concern: The same concern as R1 and R2, with the suggestion to cite the Docker test check of pull request 2971, which uses nextest.
- Solution: The closure review cites a local concurrent nextest run instead, because a local run records its command, toolchain and result in the specification.
- Current-tree verification: `grep -c '122 tests across 15 binaries' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1. `cargo nextest run -p torrust-tracker` with one `--test` argument for each of the 15 main-level targets exited with status 0 (`cargo-nextest 0.9.88`, `rustc 1.100.0`, `dev` profile).
- Resolution reference: `docs(issues): [#1419] cite a concurrent nextest run of all 15 main-level targets for AC2 and AC9`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639549625

### R4 - AC9 evidence omits the remaining main-level targets

- PR number: 2983
- Source review ID: 5988828122
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639530330
- Concern: The cited command omitted `lifecycle-signals`, `lifecycle-shutdown-policy` and `cli-configuration`, which AC9 covers.
- Solution: The AC9 row cites a sequential run of the four child-process targets (added with the fix of F2 above) and the concurrent nextest run of all 15 targets. AC9 is not narrowed.
- Current-tree verification: `grep -c '122 tests across 15 binaries' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1. `cargo nextest run -p torrust-tracker` with one `--test` argument for each of the 15 main-level targets exited with status 0 (`cargo-nextest 0.9.88`, `rustc 1.100.0`, `dev` profile).
- Resolution reference: `docs(issues): [#1419] cite a concurrent nextest run of all 15 main-level targets for AC2 and AC9`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639549928

### F4 - The IDs and relationships of the four Copilot rows depart from the skill

- PR number: 2983
- Source review ID: 5988957491
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639630264
- Concern: Step 3 of `process-pr-review` keeps a reviewer's label that does not collide. Only three of Copilot's four labels collide, yet the record renamed all four and said all four collide. R3 and R4 ask for a change that an earlier finding already asked for, yet their relationship said `ORIGINAL`.
- Solution: The intro says which three labels collide, that the renaming of Copilot's `R1` departs from the skill, and that the references stay because they never change once assigned. R3 is `RE_RAISE_OF:R1` and R4 is `RE_RAISE_OF:F2`.
- Current-tree verification: `grep -c '^| R[34] | .* | RE_RAISE_OF:' docs/pr-reviews/pr-2983-review/PR-REVIEW.md` printed 2.
- Resolution reference: `docs(pr-reviews): [#1419] record which Copilot labels collide and the re-raised findings in the audit of PR 2983`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639655631

### F5 - The health check target does not use the native tracker helper

- PR number: 2983
- Source review ID: 5988957491
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639630274
- Concern: The AC8 row said the four child-process targets start an executable through `tests/common/native_tracker/`, but `tests/http_health_check.rs` builds its own command and runs the `http_health_check` tool, not the tracker.
- Solution: The AC8 row names the three targets that use the helper and says that `tests/http_health_check.rs` runs the tool with its own command.
- Current-tree verification: `grep -c 'runs the .http_health_check. tool with its own command' docs/issues/closed/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` printed 1.
- Resolution reference: `docs(issues): [#1419] say that the health check target runs its tool without the native tracker helper`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2983#discussion_r4639656195

## Processing Log

- 2026-11-20 07:02 UTC - The review 5988749356 of `da2ce7` (changes requested, submitted 07:02:23) arrived on the first head with F1 to F3.
- 2026-11-20 07:03 UTC - Fixed F2 and F3 in the working tree and asked the EPIC 2392 coordinator session whether to edit `EPIC.md`, which its brief had excluded. It agreed to the edit that F1 asks for.
- 2026-11-20 07:05 UTC - Committed the fix of F1 (commit time 07:05:12) and the fix of F2 and F3 (commit time 07:05:45) locally, not pushed yet.
- 2026-11-20 07:06 UTC - Replied on the three threads (all posted by 07:06:25). Each reply says its commit is local and goes up in one push with this record.
- 2026-11-20 07:06 UTC - Created this audit record with F1 to F3. One push of the three commits of round 1 and the resolution of the three threads follow this commit and are not logged.
- 2026-11-20 07:10 UTC - Pushed round 1 and resolved the three threads (done by 07:10:07); each mutation returned `isResolved: true` and a GraphQL query then listed no unresolved thread.
- 2026-11-20 07:11 UTC - Copilot's review 5988828122 (submitted 07:11:16) arrived with R1 to R4. It was made on the first head, before round 1.
- 2026-11-20 07:13 UTC - Ran all 15 main-level targets concurrently with nextest, rewrote the AC2 and AC9 rows and committed the fix locally (commit time 07:13:09), not pushed yet.
- 2026-11-20 07:14 UTC - Replied on the four threads (all posted by 07:14:02). Each reply says its commit is local and goes up in one push with this update.
- 2026-11-20 07:14 UTC - Added R1 to R4 and these entries. One push of the two commits of round 2 and the resolution of the four threads follow this commit and are not logged.
- 2026-11-20 07:17 UTC - Pushed round 2 and resolved the four threads (done by 07:17:42); each mutation returned `isResolved: true` and a GraphQL query then listed no unresolved thread.
- 2026-11-20 07:24 UTC - The second review of `da2ce7`, 5988957491 (approved, submitted 07:24:46), arrived on the round 2 head with F4 and F5.
- 2026-11-20 07:27 UTC - Committed the fixes of F5 and F4 locally (commit times 07:26:41 and 07:27:14) and replied on the two threads (both posted by 07:27:52). Each reply says its commit is local and goes up in one push with this update.
- 2026-11-20 07:28 UTC - Added F4 and F5 and these entries. One push of the three commits of round 3 and the resolution of the two threads follow this commit and are not logged.

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
