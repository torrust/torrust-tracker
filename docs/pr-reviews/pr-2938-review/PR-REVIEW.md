---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2938 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2938.

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
| F1 | review-finding:pr-2938-f1 | Copilot | Minor (inferred) | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2938-f2 | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2938-f3 | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2938-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2938-f5 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2938-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2938-f7 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2938-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Flush the PEM file before `make_rust_tls` reads it

- PR number: 2938
- Source review ID: 5955960198
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613402857
- Concern: `make_rust_tls` reads the PEM files right after `make_temp_file` writes them; Copilot suggests calling `flush()` after `write_all` to avoid buffering edge cases. Copilot's overview marks it medium severity, recorded here as `Minor (inferred)`.
- Solution: No change. `make_temp_file` returns a `NamedTempFile<File>`, whose `Write` implementation forwards each call to the inner `std::fs::File`. `File` has no user-space buffer: `write_all` hands the bytes to the operating system before it returns, and `File::flush` does nothing. Another open of the path, as `make_rust_tls` does, reads them. A `flush()` call would not change what the test observes.
- Current-tree verification: `packages/axum-server/src/tls.rs` imports `tempfile::{Builder, NamedTempFile}` and builds the file with `Builder::new().prefix(prefix).suffix(".pem").tempfile()` (line 73), which returns `NamedTempFile<File>`, and calls `file.write_all(...)`. `tempfile-3.27.0/src/file/mod.rs:1015` implements `Write for NamedTempFile<F>` by calling `self.as_file_mut().write(buf)` and `self.as_file_mut().flush()`. `cargo +stable test -p torrust-tracker-axum-server --all-features --lib tls::tests` passed 4 tests on 2026-11-18, including `it_should_error_on_bad_tls_config`, which reads the written files.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613449436
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613449436

### F2 - `tempfile` version in `packages/axum-server/Cargo.toml`

- PR number: 2938
- Source review ID: 5955960198
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613402975
- Concern: `tempfile = "3.27.0"` pins a patch version, which can add maintenance work; Copilot suggests a broader constraint or a shared workspace dependency.
- Solution: No change. In Cargo, `"3.27.0"` is a caret requirement (`^3.27.0`: 3.27.0 or any later 3.x), not a pin, so `cargo update` can move it within major version 3. The workspace has no `[workspace.dependencies]` table, and every other `packages/*` crate that uses `tempfile` declares it with the same `"3.27.0"`, so this crate follows that convention. Introducing a shared workspace dependency table is a repository-wide change outside this issue.
- Current-tree verification: `grep -n '^\[workspace.dependencies\]' Cargo.toml` prints nothing. `tempfile = "3.27.0"` is the declaration in the root `Cargo.toml` and in `packages/axum-rest-api-server`, `packages/hung-component-tracker` and `packages/tracker-core`, as well as the two crates this PR changes.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613449948
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613449948

### F3 - `tempfile` version in `packages/configuration/Cargo.toml`

- PR number: 2938
- Source review ID: 5955960198
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613403053
- Concern: the same as F2 for the configuration crate: `"3.27.0"` pins a patch release; Copilot suggests `"3"` or a workspace-managed dependency.
- Solution: No change, for the reasons in F2: `"3.27.0"` is a caret requirement within major version 3, there is no workspace dependency table, and the other `packages/*` crates use the same declaration. F3 asks for the same kind of change in a different file, so it is its own row rather than a re-raise of F2.
- Current-tree verification: `packages/configuration/Cargo.toml` declares `tempfile = "3.27.0"` under `[dev-dependencies]`, matching the other `packages/*` crates; `grep -n '^\[workspace.dependencies\]' Cargo.toml` prints nothing.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613450489
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613450489

### F4 - Eighteen tests in `lib.rs` call `clear_env()`, not five

- PR number: 2938
- Source review ID: 5956000792
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613436527
- Concern: the evidence file, the spec's progress log and the PR description say five tests in `packages/configuration/src/lib.rs` call `jail.clear_env()` and name lines 438 to 528; there are eighteen, one per test, at lines 438 to 892. A follow-up scoped from the record would miss thirteen tests.
- Solution: The evidence file names all eighteen lines in T3 and in "Failures and Follow-up" and says that the first version said five. The spec's progress log keeps the 10:06 entry as written and appends a correction entry naming it. The PR description says eighteen. The EPIC coordinator, who received the five-test count, was sent the correction.
- Current-tree verification: `grep -n 'clear_env()' packages/configuration/src/lib.rs | wc -l` prints 18 (lines 438, 462, 483, 506, 528, 555, 579, 604, 649, 671, 697, 734, 759, 786, 814, 838, 866, 892); `grep -rn 'clear_env()' packages/configuration/src` finds no other file.
- Resolution reference: `docs(issues): [#2907] correct the clear_env count and the rebase claim, and add a red run of both TLS tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613558971

### F5 - The red evidence of the two new tests cannot be audited

- PR number: 2938
- Source review ID: 5956000792
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613436541
- Concern: `fix-bug` asks for a red run of every maintained regression test and for the evidence's artifacts to stay in the issue folder. The AC3 red run used a variant whose code is recorded nowhere, its `tls.rs:105` resolves in no commit, and the drop test had no red run, although the T4 row says "Red then green recorded".
- Solution: The evidence file records the code of the temporary AC3 variant and says its line 105 refers to the file as it was during that run. It adds a second red run (mutate then restore): `make_temp_file`'s builder got `.disable_cleanup(true)`, so its guard keeps the file. Both maintained tests failed at their final assertion, with the four PEM files left in `TMPDIR`. After `git checkout -- packages/axum-server/src/tls.rs` the 4 tests passed with `TMPDIR` empty. The mutation diff and the output are inline. The T4 row says the red run covers both maintained tests.
- Current-tree verification: `cargo +stable test -p torrust-tracker-axum-server --all-features --lib tls::tests` with the mutation at 2026-11-18 11:54:15 UTC: 2 passed, 2 failed (`it_should_remove_the_temporary_file_when_its_guard_is_dropped`, `it_should_remove_the_temporary_file_when_the_test_using_it_fails`); without it at 11:54:16 UTC: 4 passed, `TMPDIR` empty. `git status --short packages/axum-server` was clean after the revert.
- Resolution reference: `docs(issues): [#2907] correct the clear_env count and the rebase claim, and add a red run of both TLS tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613559435

### F6 - The rebase brought in more than Markdown files

- PR number: 2938
- Source review ID: 5956000792
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613436554
- Concern: the progress log and the PR description say the `develop` commits the branch was rebased on change only Markdown files; they also changed `.dockerignore`, `project-words.txt` and six `.txt` evidence files.
- Solution: The spec's progress log appends a correction entry that names the 11:19 entry and lists what changed; the PR description says no Rust or Cargo file changed. The wrong claim came from comparing the rebased base with `torrust/develop`, which after the rebase were the same commit, so the comparison printed nothing.
- Current-tree verification: `git diff --name-status 90a7c7d3f a0d97f956 | grep -v '\.md$'` lists `.dockerignore`, `project-words.txt` and six renamed `.txt` files under `docs/issues/`, and no `.rs`, `Cargo.toml` or `Cargo.lock`.
- Resolution reference: `docs(issues): [#2907] correct the clear_env count and the rebase claim, and add a red run of both TLS tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613559842

### F7 - Import `fs` instead of the qualified path

- PR number: 2938
- Source review ID: 5956000792
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613436560
- Concern: `AGENTS.md` (Code Style, Imports) prefers short imported names; the two save-to-file tests call `std::fs::read_to_string` without a naming conflict.
- Solution: Both test modules import `std::fs`, and the tests call `fs::read_to_string`. Sweeping for the same mistake, `make_temp_file` called `tempfile::Builder::new()`; `tls.rs` now imports `tempfile::{Builder, NamedTempFile}`.
- Current-tree verification: `grep -n 'std::fs::read_to_string\|tempfile::Builder' packages/configuration/src packages/axum-server/src -r` finds nothing. `cargo +stable test -p torrust-tracker-configuration -p torrust-tracker-axum-server --all-features --lib` passed 159 and 9 tests with a fresh `TMPDIR` left empty, and `cargo +stable clippy -p torrust-tracker-axum-server -p torrust-tracker-configuration --all-targets --all-features -- -D warnings` exited `0`.
- Resolution reference: `test(configuration, axum-server): [#2907] import fs and Builder instead of using their full paths`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613560291

### F8 - The F1 verification describes the builder call as it was before the import fix

- PR number: 2938
- Source review ID: 5956302969
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2938#discussion_r4613696760
- Concern: F1's current-tree verification says `tls.rs` builds the file with `tempfile::Builder::new()`; after the F7 fix the file imports `tempfile::{Builder, NamedTempFile}` and calls `Builder::new()`. The disposition stands; the line should match the current tree, as `process-pr-review` step 5 asks.
- Solution: F1's verification names the import and `Builder::new()` at line 73. Sweeping for the same mistake, the T4 "Change" line of the evidence file also quoted `tempfile::Builder::new()`; it now quotes `Builder::new()` and names the import. F7's entries keep the old path on purpose, because they describe the code before that fix.
- Current-tree verification: `grep -n 'use tempfile' packages/axum-server/src/tls.rs` prints `use tempfile::{Builder, NamedTempFile};` (line 64) and line 73 is `let mut file = Builder::new()`. `grep -rn 'tempfile::Builder' docs/issues/open/2907-2392-remove-leaked-test-temporary-files` prints only the evidence line that names the import.
- Resolution reference: `docs(pr-reviews): [#2907] describe the current builder call in the F1 verification`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_F8>

## Processing Log

- 2026-11-18 11:51 UTC - Started audit for Copilot review 5955960198 (submitted 2026-11-18 11:48:19 UTC on the head that adds `docs(issues): [#2907] record the passing pre-push checks`). Its body is an overview with no request of its own; its three inline comments are F1 to F3.
- 2026-11-18 11:51 UTC - Replied to F1, F2 and F3 (no change); the replies were posted at 11:51:48, 11:51:50 and 11:51:52 UTC.
- 2026-11-18 11:52 UTC - Found da2ce7's review 5956000792 (`APPROVED`, submitted 2026-11-18 11:50:49 UTC on the same head) with four inline findings while listing the threads to resolve. Its finding IDs F1 to F4 collide with this audit's, so they are F4 to F7 here, with the reviewer's IDs in each detail entry. All four are valid on the current tree.
- 2026-11-18 11:56 UTC - Fixed F7 in `test(configuration, axum-server): [#2907] import fs and Builder instead of using their full paths` and F4, F5 and F6 in `docs(issues): [#2907] correct the clear_env count and the rebase claim, and add a red run of both TLS tests`; not pushed yet.
- 2026-11-18 12:00 UTC - Pushed both fix commits to the PR branch at 12:00:08 UTC (the pre-push hook passed). Replied to F4 to F7 at 12:00:23, 12:00:26, 12:00:28 and 12:00:31 UTC, then resolved all seven threads; a GraphQL refresh at 12:00:47 UTC showed 0 unresolved threads.
- 2026-11-18 12:13 UTC - Round 2: da2ce7's review 5956302969 (`APPROVED`, submitted 2026-11-18 12:11:57 UTC on the head that adds this record) has one inline finding, its F6, recorded here as F8 because F6 is taken. Valid on the current tree; fixed in `docs(pr-reviews): [#2907] describe the current builder call in the F1 verification`, not pushed yet.

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
