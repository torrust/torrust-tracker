---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3018 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3018.

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

F1 to F5 come from CodeRabbit's review 6030014294 (COMMENTED, submitted 21:44:49Z on the first head, "test(source-location-check): [#2918] ignore the GIT_* variables of a running Git hook in the CLI tests"), in source order of its inline threads. F6 and F7 come from Copilot's review 6030466922 (COMMENTED, submitted 22:22:53Z on the same head); Copilot noted that its full agentic review did not start before its timeout, and the two inline findings are all it reported. Copilot rates findings Medium and Low; they are recorded as Minor and Nit, as in the audits of PRs #2991 and #3001.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3018-f1 | Unknown | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3018-f2 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3018-f3 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3018-f4 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3018-f5 | Unknown | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3018-f6 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3018-f7 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A test-only import with braces ends the skipping too late

- PR number: 3018
- Source review ID: 6030014294
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672437160
- Concern: CodeRabbit (rated Major) noted that `source-location-check` ended the skipping of a `#[cfg(test)]` item only at a closing brace of the attribute's indentation or at a one-line `;` item without braces. For `#[cfg(test)]` followed by `use std::{fmt, io};` neither held, so the loop consumed the production code after it, including a later `#[track_caller]`, and could report success for the rest of the file.
- Solution: A `#[cfg(test)]` item now ends at the `;` or the closing delimiter that balances `{}`, `()` and `[]` (`skip_test_item`), so the import ends at its semicolon and a test function with its braces on one line ends at its closing brace. The new tests `it_should_stop_skipping_at_the_end_of_a_test_only_import` and `it_should_skip_a_test_only_item_whose_braces_are_not_on_their_own_lines` pin both. They were written together with the fix, not run red first.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes 16 library tests and 4 CLI tests; `cargo run -q -p source-location-check` exits 0 on the tree.
- Resolution reference: fix(source-location-check): [#2918] mask comments and strings, balance delimiters and track module ancestry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672899188

### F2 - A comment line clears the pending `#[cfg(test)]`

- PR number: 3018
- Source review ID: 6030014294
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672437170
- Concern: CodeRabbit (rated Minor) noted that for `#[cfg(test)]`, then a comment line, then `mod helpers;`, the comment cleared the pending attribute in `test_only_modules`, so the helper file got no exemption and a permitted location in it failed the check.
- Solution: Comments are blanked before any scan (`mask`), so a comment line is an empty line, and `module_declarations` skips empty lines without clearing the pending `#[cfg(test)]`. `it_should_keep_the_cfg_test_attribute_across_comment_lines` covers a line comment and a block comment between the attribute and the declaration.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes, including that test.
- Resolution reference: fix(source-location-check): [#2918] mask comments and strings, balance delimiters and track module ancestry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672899659

### F3 - Test module files ignore the inline-module ancestry

- PR number: 3018
- Source review ID: 6030014294
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672437176
- Concern: CodeRabbit (rated Minor) noted that the path of a `#[cfg(test)] mod helpers;` declared inside an inline module (`mod inner { ... }`) was built directly under the file's module directory, so the check exempted `src/helpers.rs` and rejected the real `src/inner/helpers.rs`.
- Solution: `module_declarations` tracks the inline modules that enclose each declaration by brace depth, returns them as the declaration's `path`, and `test_only_paths` joins that path under the module directory. A declaration inside a `#[cfg(test)]` inline module is test-only as well. `it_should_keep_the_inline_module_ancestry_of_a_declaration` and `it_should_treat_declared_test_modules_and_tests_directories_as_test_code` pin `inner/deep.rs` and show that `deep.rs` stays checked.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes, including those tests.
- Resolution reference: fix(source-location-check): [#2918] mask comments and strings, balance delimiters and track module ancestry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672900052

### F4 - A file with a production declaration is exempted as test code

- PR number: 3018
- Source review ID: 6030014294
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672437187
- Concern: CodeRabbit (rated Minor) noted that a file declared `#[cfg(test)] mod shared;` in `lib.rs` and plainly `mod shared;` in `main.rs` was exempted wholesale, so a production `#[track_caller]` in it passed.
- Solution: `test_only_paths` collects the files and directory prefixes of test-only and of production declarations and removes from the test-only set every entry that a production declaration also includes. `it_should_keep_checking_a_file_that_a_production_declaration_also_includes` covers `lib.rs` plus `main.rs`.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes, including that test.
- Resolution reference: fix(source-location-check): [#2918] mask comments and strings, balance delimiters and track module ancestry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672900486

### F5 - M1 is marked DONE without a commit rejected by the hook

- PR number: 3018
- Source review ID: 6030014294
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672437195
- Concern: CodeRabbit (rated Minor) noted that m1 of the spec was marked `DONE` on the evidence of running the check's command directly, which does not show that the pre-commit hook invokes it; M1 asks for a commit that the hook rejects.
- Solution: M1 was run for real: a `Location::caller()` probe in `packages/primitives/src/lib.rs` was staged and `git commit` was run; the hook rejected it at the step "Checking that errors record no source locations" at 22:25:45 UTC and created no commit. The probe was reverted. V1 of `manual-verification-evidence.md` step 4 records the hook output, and the M1 row says the hook was exercised.
- Current-tree verification: `grep -n "step 4\|22:25:45" docs/issues/open/2918-2912-si-6-drop-located-error-and-enforce-the-rule/manual-verification-evidence.md` shows the record; `git status --short` was empty after the revert.
- Resolution reference: docs(issues): [#2918] record the hook run of M1 and round 1 of the review audit of PR #3018
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672900878

### F6 - Trailing and block comments are still scanned

- PR number: 3018
- Source review ID: 6030466922
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672824912
- Concern: Copilot (rated Medium, recorded as Minor) noted that only lines starting with `//` were skipped, so a trailing `// ...` comment on a code line or a `/* ... */` block comment could still be flagged, against the documented "comment lines are ignored", and asked for tests of both.
- Solution: `mask` blanks line comments (also trailing), nested block comments, string and raw string contents and character literals before the scan, and the module docs and the spec's design note say so. `it_should_ignore_trailing_line_comments_block_comments_and_string_literals` covers a trailing comment, a block comment over two lines, a string, a raw string and a `'{'` character literal.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes, including that test; the old comment-line test `it_should_ignore_comment_lines` still passes.
- Resolution reference: fix(source-location-check): [#2918] mask comments and strings, balance delimiters and track module ancestry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672901227

### F7 - Allow-list paths are not normalized to `/`

- PR number: 3018
- Source review ID: 6030466922
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672824963
- Concern: Copilot (rated Medium, recorded as Minor) noted that allow-list entries were stored as written while the checker normalizes file paths to `/`, so a backslash path pasted on Windows would never match.
- Solution: `parse_allow_list` now replaces `\\` with `/` in each entry's path. `it_should_normalize_backslashes_of_allow_list_paths` covers it.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes, including that test.
- Resolution reference: fix(source-location-check): [#2918] mask comments and strings, balance delimiters and track module ancestry
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672901583

## Processing Log

- 2026-11-23 22:27 UTC - Copilot's review 6030466922 (COMMENTED, submitted 22:22:53Z) reached the poller; listing the reviews through the REST API also showed CodeRabbit's review 6030014294 (submitted 21:44:49Z), which the poller does not watch. Seven inline findings, F1 to F7. The audit was started before the first reply. F1 to F4, F6 and F7 are fixed in "fix(source-location-check): [#2918] mask comments and strings, balance delimiters and track module ancestry" and F5 in this audit's commit.
- 2026-11-23 22:31 UTC - Pushed the fixes (pre-push hook passed, push finished 22:30:26 UTC by `date -u`), replied on the seven threads (replies posted 22:30:57 UTC) and resolved them (`resolveReviewThread` returned `true` for each; `date -u` after the resolve commands: 22:31:10). GraphQL refresh: zero unresolved threads on PR #3018. This entry is committed and pushed after the replies.
