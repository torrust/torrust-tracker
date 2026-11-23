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

F1 to F5 come from CodeRabbit's review 6030014294 (COMMENTED, submitted 21:44:49Z on the first head, "test(source-location-check): [#2918] ignore the GIT_* variables of a running Git hook in the CLI tests"), in source order of its inline threads. F6 and F7 come from Copilot's review 6030466922 (COMMENTED, submitted 22:22:53Z on the same head); Copilot noted that its full agentic review did not start before its timeout, and the two inline findings are all it reported. F8 to F11 come from CodeRabbit's review 6030628364 (COMMENTED, submitted 22:37:45Z on the round-1 head `0cade8030`'s tree, before the audit push) and F12 from a reply of CodeRabbit on the F4 thread (review 6030752681, 22:49:00Z), a re-raise of F4. F13 and F14 come from CodeRabbit's review 6031363636 (COMMENTED, 23:47:54Z, on the round-2 head) and F15 from its reply on the F9 thread (review 6031381794, 23:49:54Z). CodeRabbit also replied on the F1, F2 and F3 threads (22:39Z) that the current code addresses them; those replies hold no new finding. Copilot rates findings Medium and Low; they are recorded as Minor and Nit, as in the audits of PRs #2991 and #3001.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3018-f1 | Unknown | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3018-f2 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3018-f3 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3018-f4 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3018-f5 | Unknown | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3018-f6 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3018-f7 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-3018-f8 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3018-f9 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3018-f10 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3018-f11 | Unknown | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3018-f12 | Unknown | Minor | correctness | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F13 | review-finding:pr-3018-f13 | Unknown | Minor | correctness | ORIGINAL | FIXED | OPEN |
| F14 | review-finding:pr-3018-f14 | Unknown | Minor | correctness | ORIGINAL | FIXED | OPEN |
| F15 | review-finding:pr-3018-f15 | Unknown | Minor | testing | ORIGINAL | FIXED | OPEN |

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

### F8 - A raw byte string is not masked

- PR number: 3018
- Source review ID: 6030628364
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672965067
- Concern: CodeRabbit (rated Minor) noted that a raw byte string with hashes (`br#"""#`) was not recognized: the identifier guard rejected the `r` after the `b`, the three quotes left `mask` in the string state, and a real `Location::caller()` on the next line was masked and passed.
- Solution: `raw_prefix_allowed` accepts an `r` that follows a `b` which does not end an identifier, so `br"..."` and `br#"..."#` are masked as raw strings. `it_should_mask_a_raw_byte_string_and_an_escaped_apostrophe` checks that a location on the next line is reported.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes 20 library tests and 4 CLI tests, including the named test; `cargo run -q -p source-location-check` exits 0 on the tree. The tests were written with the fix; the first three were also run against the previous code and failed there.
- Resolution reference: fix(source-location-check): [#2918] mask raw byte strings and escaped apostrophes and parse attributes on the item's line
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4673532108

### F9 - An escaped apostrophe ends a character literal too early

- PR number: 3018
- Source review ID: 6030628364
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672965078
- Concern: CodeRabbit (rated Minor) noted that `char_literal_length` treated the apostrophe after an escaped apostrophe as the closing quote (`'\''` returned length 3 instead of 4), so in `['\'', '{']` the `{` stayed unmasked and could leave `skip_test_item` at a nonzero depth.
- Solution: The closing quote is searched after the backslash and the escaped character, so `'\''` has length 4. The same test covers `['\'', '{']` inside a function followed by a real violation.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes 20 library tests and 4 CLI tests, including the named test; `cargo run -q -p source-location-check` exits 0 on the tree. The tests were written with the fix; the first three were also run against the previous code and failed there.
- Resolution reference: fix(source-location-check): [#2918] mask raw byte strings and escaped apostrophes and parse attributes on the item's line
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4673532413

### F10 - An attribute on the item's line is discarded with the item

- PR number: 3018
- Source review ID: 6030628364
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672965088
- Concern: CodeRabbit (rated Minor) noted that after `#[cfg(test)]`, a line that starts with another attribute (`#[allow(dead_code)] fn helper() {}`) was discarded whole, so the test function was dropped from the skipped item and the next production item was consumed as the test item, hiding its violation.
- Solution: `skip_attributes` consumes only the attribute tokens, over balanced brackets, and returns the rest of the line, which `skip_test_item` and `module_declarations` process as the item. `it_should_skip_an_item_that_shares_its_line_with_another_attribute` pins it.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes 20 library tests and 4 CLI tests, including the named test; `cargo run -q -p source-location-check` exits 0 on the tree. The tests were written with the fix; the first three were also run against the previous code and failed there.
- Resolution reference: fix(source-location-check): [#2918] mask raw byte strings and escaped apostrophes and parse attributes on the item's line
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4673532677

### F11 - A multi-line attribute clears the pending `#[cfg(test)]`

- PR number: 3018
- Source review ID: 6030628364
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4672965106
- Concern: CodeRabbit (rated Minor) noted that a multi-line attribute between `#[cfg(test)]` and `mod helpers;` (`#[allow(\n dead_code\n)]`) cleared the pending test condition at its second line, so `helpers.rs` was classified as production.
- Solution: `module_declarations` tracks the open bracket depth of an attribute across lines (`skip_attributes`) and keeps the pending `#[cfg(test)]` until the item. `it_should_keep_the_cfg_test_attribute_across_a_multiline_attribute` pins it.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes 20 library tests and 4 CLI tests, including the named test; `cargo run -q -p source-location-check` exits 0 on the tree. The tests were written with the fix; the first three were also run against the previous code and failed there.
- Resolution reference: fix(source-location-check): [#2918] mask raw byte strings and escaped apostrophes and parse attributes on the item's line
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4673533009

### F12 - A production module below a test-only prefix is still skipped

- PR number: 3018
- Source review ID: 6030752681
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#pullrequestreview-6030752681
- Concern: CodeRabbit (rated Minor) noted that in a reply on the F4 thread, CodeRabbit found that a production inline module (`mod shared { mod child; }` in `main.rs`) next to `#[cfg(test)] mod shared;` in `lib.rs` still left `src/shared/child.rs` skipped, because the test-only prefix `src/shared/` stayed in force.
- Solution: `is_test_file` compares the longest matching test-only entry with the longest matching production entry and the more specific one decides; `test_only_paths` returns both sets (`ModulePaths`). `it_should_let_a_production_declaration_override_a_test_only_directory_prefix` pins `child.rs` and a file below it.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes 20 library tests and 4 CLI tests, including the named test; `cargo run -q -p source-location-check` exits 0 on the tree. The tests were written with the fix; the first three were also run against the previous code and failed there.
- Resolution reference: fix(source-location-check): [#2918] let a production module declaration override a test-only directory prefix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4673533475

### F13 - The rest of a line after an excluded item is dropped

- PR number: 3018
- Source review ID: 6031363636
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4673579305
- Concern: CodeRabbit (rated Minor) noted that `skip_test_item` returned at the closing brace of an excluded helper and the caller then dropped the rest of that line, so a `#[track_caller]` item that follows the helper on the same line passed the check.
- Solution: `skip_test_item` returns the line on which the item ended and the text after it; `violations_in` scans that remainder with the original line number and keeps looking for further `#[cfg(test)]` items in it. Attributes inside the excluded helper stay exempt. `it_should_resume_scanning_after_an_excluded_item_that_ends_mid_line` pins both; it fails on the previous code.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes 23 library tests and 4 CLI tests, including the named test; `cargo run -q -p source-location-check` exits 0 on the tree.
- Resolution reference: fix(source-location-check): [#2918] scan the rest of a line after an excluded item and inherit test-only status through module files
- Follow-up PR URL: N/A
- Reply URL: pending

### F14 - Modules of a test-only file are classified as production

- PR number: 3018
- Source review ID: 6031363636
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4673579311
- Concern: CodeRabbit (rated Minor) noted that a module declared with a plain `mod nested;` inside a test-only file (`helpers.rs`, declared `#[cfg(test)] mod helpers;`) got a production entry, and that entry overrode the test-only `helpers/` prefix, so the check rejected permitted test code.
- Solution: `test_only_paths` classifies a declaration as test-only when it is `#[cfg(test)]` or when the file that declares it is test code, repeating until the classification is stable. `it_should_treat_modules_declared_in_a_test_only_file_as_test_only` pins `helpers/nested.rs`; it fails on the previous code.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes 23 library tests and 4 CLI tests, including the named test; `cargo run -q -p source-location-check` exits 0 on the tree.
- Resolution reference: fix(source-location-check): [#2918] scan the rest of a line after an excluded item and inherit test-only status through module files
- Follow-up PR URL: N/A
- Reply URL: pending

### F15 - The F9 regression test does not run inside a test-only function

- PR number: 3018
- Source review ID: 6031381794
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3018#pullrequestreview-6031381794
- Concern: CodeRabbit (rated Minor), in its reply https://github.com/torrust/torrust-tracker/pull/3018#discussion_r4673594879 on the F9 thread (the review is the source here, as for F12), noted that the test added for F9 did not put the escaped apostrophe inside a `#[cfg(test)]` function, so it did not exercise the reported failure of `skip_test_item`, and asked for adjacent character literals in a test-only function followed by a production violation.
- Solution: `it_should_keep_the_delimiter_depth_across_adjacent_character_literals_in_a_test_only_function` covers `['\'','{']` in a `#[cfg(test)]` function followed by a `#[track_caller]` function; with the character literal length of the previous revision restored by hand it fails, with the fix it passes.
- Current-tree verification: `cargo test -p source-location-check` (stable Rust toolchain) passes 23 library tests and 4 CLI tests, including the named test; `cargo run -q -p source-location-check` exits 0 on the tree.
- Resolution reference: test(source-location-check): [#2918] cover adjacent character literals in a test-only function
- Follow-up PR URL: N/A
- Reply URL: pending

## Processing Log

- 2026-11-23 22:27 UTC - Copilot's review 6030466922 (COMMENTED, submitted 22:22:53Z) reached the poller; listing the reviews through the REST API also showed CodeRabbit's review 6030014294 (submitted 21:44:49Z), which the poller does not watch. Seven inline findings, F1 to F7. The audit was started before the first reply. F1 to F4, F6 and F7 are fixed in "fix(source-location-check): [#2918] mask comments and strings, balance delimiters and track module ancestry" and F5 in this audit's commit.
- 2026-11-23 22:31 UTC - Pushed the fixes (pre-push hook passed, push finished 22:30:26 UTC by `date -u`), replied on the seven threads (replies posted 22:30:57 UTC) and resolved them (`resolveReviewThread` returned `true` for each; `date -u` after the resolve commands: 22:31:10). GraphQL refresh: zero unresolved threads on PR #3018. This entry is committed and pushed after the replies.
- 2026-11-23 23:38 UTC - The poller (watching da2ce7 and Copilot only) timed out after an hour with no event, but listing the reviews showed CodeRabbit's round 2: review 6030628364 with four new inline findings (F8 to F11) and a reply on the F4 thread (F12). F8 to F11 fixed in "fix(source-location-check): [#2918] mask raw byte strings and escaped apostrophes and parse attributes on the item's line" and F12 in "fix(source-location-check): [#2918] let a production module declaration override a test-only directory prefix". The poller is restarted with the CodeRabbit login added as a watched reviewer.
- 2026-11-23 23:43 UTC - Pushed the round-2 fixes and audit (pre-push hook passed, push finished 23:41:42 UTC by `date -u`), replied on the five threads (replies posted 23:42:06 UTC) and resolved them (`resolveReviewThread` returned `true` for each, including the F4 thread that CodeRabbit's reply had left resolved; `date -u` after the resolve commands: 23:42:22). GraphQL refresh: zero unresolved threads on PR #3018. This entry is committed and pushed after the replies.
- 2026-11-23 23:52 UTC - The poller, now watching CodeRabbit too, returned CodeRabbit's round 3 (review 6031363636, 23:47:54Z, two inline findings F13 and F14) and a reply on the F9 thread asking for a stronger regression test (F15, 23:49:54Z); another CodeRabbit reply on the F4 thread (23:49:12Z) says the code and tests cover F12 and holds no finding. This is round 3 of at most 5. F13 and F14 fixed in "fix(source-location-check): [#2918] scan the rest of a line after an excluded item and inherit test-only status through module files" and F15 in "test(source-location-check): [#2918] cover adjacent character literals in a test-only function", which was run red against the previous character literal length by hand before the fix state was restored.
