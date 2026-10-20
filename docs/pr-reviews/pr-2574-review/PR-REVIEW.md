---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - contrib/dev-tools/checks/ci-gate/src/change_detection.rs
    - docs/issues/open/2525-ci-gate-required-check/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2574 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2574>.

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

## Findings

Three reviews were submitted so far, all by Copilot.

- Review 5585221487 carries two inline threads, F1 and F2. Its overview repeats them, so the body adds
  no rows. It read the head whose subject is "fix(ci-gate): [#2525] avoid an intra-doc link to a
  private type". Both findings have one root cause, the format of the tool's input, and are fixed by
  one change.
- Review 5585732662 carries three inline threads, F3 to F5, on the head whose subject is "docs(pr-reviews):
  add the PR #2574 review audit". Its body says that the full agentic review did not start before the
  timeout and repeats the three threads, so it adds no rows. Each finding is fixed in its own commit.
- Review 5587424828 read the head whose subject is "docs(pr-reviews): record the second review round in
  the PR #2574 audit". It marks F3 to F5 as resolved and carries two findings in its body and no inline
  thread, so they are recorded as F6 and F7 with `NON_RESOLVABLE` and the review URL as their source.
  The reviewer's own IDs, F6 and F7, do not collide with the audit's.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2574-f1 | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2574-f2 | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2574-f3 | Copilot | Medium (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2574-f4 | Copilot | Medium (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2574-f5 | Copilot | Low (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2574-f6 | Copilot | Minor | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |
| F7 | review-finding:pr-2574-f7 | Copilot | Minor | testing | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - Whitespace that belongs to a file name is trimmed

- PR number: 2574
- Source review ID: 5585221487
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4325119193>
- Concern: `trim()` turns `README.md` followed by a space, or a leading space and `project-words.txt`,
  into the documentation files the workflows ignore, so testing, OS compatibility, coverage, and
  container are skipped although the workflow filters do not ignore the original paths. Preserve
  whitespace, ignore only empty records, update the parser documentation, and replace the
  whitespace-normalization test with regression cases.
- Solution: the tool reads the output of `git diff --name-only -z`: NUL-separated records kept exactly
  as they are, with only empty records ignored. The parser documentation says so, the
  whitespace-normalization test is replaced by tests that keep leading and trailing spaces, empty
  records, and a newline inside a name, and an end-to-end test classifies the two spaced names through
  the binary.
- Current-tree verification: `parse_changed_paths` takes bytes and splits on NUL. Reintroducing
  `trim()` makes the parser test and the end-to-end test fail (`cargo test -p ci-gate --no-fail-fast`),
  and `cargo test -p ci-gate` passes with 99 unit and 5 command-line tests.
- Resolution reference: fix(ci-gate): [#2525] read the changed paths as NUL-separated records
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4325308509>

### F2 - Git-quoted file names are not decoded

- PR number: 2574
- Source review ID: 5585221487
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4325119291>
- Concern: `git diff --name-only` C-quotes non-ASCII names, so `packages/tracker-core/src/café.rs` arrives
  as a quoted, escaped string. The leading quote keeps both database groups from matching their
  directory prefix, and quoted Markdown paths trigger the heavy groups. Decode the quoted records or
  adopt a lossless input format such as `git diff --name-only -z`, and add CLI regression cases.
- Solution: the same change as F1, which takes the lossless option: with `-z` git prints names as they
  are, and the tool reads them as they are. A name that is not valid UTF-8 has its invalid bytes
  replaced, which cannot change a match against the ASCII patterns. The usage text, the module
  documentation, and the PR description name `git diff --name-only -z`. New tests: a parser test for
  the unquoted non-ASCII name, a table case for the database rule, an end-to-end test that a `café.rs`
  file below `packages/tracker-core/` needs both database groups, and a test for a name that is not
  valid UTF-8.
- Current-tree verification: in a throwaway repository, `git diff --cached --name-only` printed the
  quoted name and `git diff --cached --name-only -z` printed it as it is; piping the `-z` output to
  the binary needs `testing`, `db-compatibility`, and `db-benchmarking`. Reintroducing the quoted form
  makes the parser test fail. The comparison with the workflows GitHub ran for the 60 latest merged
  pull requests was re-run with the new input: 60 of 60
  (`docs/issues/open/2525-ci-gate-required-check/manual-verification-evidence.md`, V2, V3, V6).
- Resolution reference: fix(ci-gate): [#2525] read the changed paths as NUL-separated records
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4325308851>

### F3 - The claim that lossy decoding cannot change a match is unsupported

- PR number: 2574
- Source review ID: 5585732662
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4325531763>
- Concern: the parser documentation says that replacing invalid bytes "does not change whether it matches
  the ASCII patterns", which is stronger than the implementation guarantees; an invalid byte inside the
  `packages/tracker-core/` prefix could change a prefix match. Soften the claim, or match on raw bytes.
- Solution: the claim is true, so the behaviour stays, and the documentation now gives the reason. Every
  pattern of the rules is ASCII, an ASCII byte decodes to itself, and the replacement character U+FFFD
  is not ASCII, so a pattern matches the decoded name exactly when it matches the raw bytes. Matching on
  raw bytes would need a second matching implementation without changing any outcome, so it was not
  adopted. Five table cases pin the equivalence for a directory prefix (an invalid byte after it and
  inside it), an extension (before it and after it), and an exact path.
- Current-tree verification: `cargo test -p ci-gate` passes with 104 unit and 5 command-line tests. A
  mutant that drops a path that is not valid UTF-8 fails three of them.
- Resolution reference: fix(ci-gate): [#2525] state why invalid UTF-8 cannot change a match
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4326573846>

### F4 - Success-path CLI tests do not assert the success contract first

- PR number: 2574
- Source review ID: 5585732662
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4325531868>
- Concern: several success-path CLI tests parse stdout without asserting a successful exit status and an
  empty stderr, so a failure shows up as a parse error. Add a helper for the success contract and use it
  in every success-path test.
- Solution: `assert_succeeded_without_diagnostics` asserts exit status 0 and an empty stderr, with the
  causal facts in the messages. All four success-path tests call it before reading stdout; the first
  one keeps its single-line check.
- Current-tree verification: `cargo test -p ci-gate --test cli` passes with 5 tests. A binary that
  writes a stray line to stderr on success fails all four success-path tests (`--no-fail-fast`).
- Resolution reference: test(ci-gate): [#2525] assert the success contract in every success-path CLI test
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4326574414>

### F5 - The module documentation counts six workflows, but there are eight

- PR number: 2574
- Source review ID: 5585732662
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4325531922>
- Concern: the module documentation says the rules replace "six workflow files", but the PR models
  eight groups, one per workflow.
- Solution: the module documentation no longer states a count, and the T2 row of the specification, which
  had the same wrong count, says eight. The merged ADR also says "six workflow files"; it is a decision
  record of the situation when it was written and is left unchanged.
- Current-tree verification: the rule table has eight entries and `JobGroup::ALL` has length eight; no
  other "six" remains in the crate or in the specification.
- Resolution reference: docs(ci-gate): [#2525] correct the workflow count in the rules documentation
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2574#discussion_r4326574949>

### F6 - A non-UTF-8 argument panics instead of failing with a usage error

- PR number: 2574
- Source review ID: 5587424828
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2574#pullrequestreview-5587424828>
- Concern: on Unix a non-UTF-8 argument makes `env::args()` panic before argument validation or the
  failure handler runs, which prints a plain-text panic and exits with 101 instead of the JSON usage
  diagnostic and exit code 2 the CLI output contract requires. Use `args_os()` with a fallible
  conversion routed through the usage failure, and add a Unix-only regression test.
- Solution: the arguments are read with `args_os` and converted with a fallible step that fails with a
  usage error naming the lossy argument and carrying the usage text. A Unix-only command-line test
  passes a base branch that ends in an invalid byte and expects exit code 2, an empty stdout, and one
  `usage_error` record that mentions UTF-8.
- Current-tree verification: before the fix the real binary exited with 101 and a plain-text panic for a
  non-UTF-8 base branch and for a non-UTF-8 subcommand, and the new test failed. After it, the binary
  exits with 2 and one `usage_error` record, and `cargo test -p ci-gate` passes with 104 unit and 6
  command-line tests.
- Resolution reference: fix(ci-gate): [#2525] report a non-UTF-8 argument as a usage error
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2574#issuecomment-6294777122>

### F7 - The usage-error test writes stdin to a command that exits before reading it

- PR number: 2574
- Source review ID: 5587424828
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2574#pullrequestreview-5587424828>
- Concern: the binary rejects a missing base branch before it reads stdin, so if it exits before the
  test writes the supplied bytes, `write_all(...).expect(...)` panics with a broken pipe and fails the
  test although the binary behaved correctly. Pass empty stdin, since the test only exercises argument
  validation.
- Solution: the test passes empty stdin and says why in a comment. The new non-UTF-8 argument test does
  the same.
- Current-tree verification: the binary reads stdin only after it parses its arguments
  (`contrib/dev-tools/checks/ci-gate/src/main.rs`), so a write to it can lose the race; an empty
  write cannot fail. `cargo test -p ci-gate --test cli` passes with 6 tests.
- Resolution reference: test(ci-gate): [#2525] send no stdin to the command that rejects its arguments
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2574#issuecomment-6294777558>

## Processing Log

- 2026-10-20 13:00 UTC - Fetched the review and its two threads, classified and numbered the findings,
  reproduced both with real git output, and fixed F1 and F2 together in one signed commit, with
  regression tests that were seen to fail on the original behavior.
- 2026-10-20 13:07 UTC - Re-ran the evidence with the new input format and committed it separately.
- 2026-10-20 13:10 UTC - Ran the pre-push checks, pushed, and, after the push, replied on both
  threads.
- 2026-10-20 13:12 UTC - Started this record, with a reply URL in every row.
- 2026-10-20 13:31 UTC - Copilot's second review arrived with three threads, F3 to F5. Each was checked
  against the tree: F3's claim is true but nothing argued it, F4 and F5 were correct.
- 2026-10-20 15:05 UTC - Fixed each finding in its own signed commit and recorded the round in the
  specification. A health-check API test failed once in the full pre-push run, in a package this change
  does not touch; it passed three times alone and the rerun was clean. Pushed, and replied on the three
  threads.
- 2026-10-20 15:07 UTC - Added F3 to F5 and their reply URLs to this record.
- 2026-10-20 15:30 UTC - Copilot's third review arrived with two findings in its body and no inline
  thread, F6 and F7. F6 was reproduced with the real binary (exit code 101 and a plain-text panic), and
  its regression test was seen to fail before the fix. F7 is a race in the test and was confirmed by
  reading the binary's order of work.
- 2026-10-20 15:40 UTC - Fixed each finding in its own signed commit and recorded the round in the
  specification. Pushed and replied with a conversation comment per finding, since neither has a
  thread. No thread is unresolved.

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
