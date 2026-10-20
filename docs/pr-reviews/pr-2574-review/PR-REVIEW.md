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

One review was submitted so far. Copilot's review 5585221487 carries two inline threads, F1 and F2.
Its overview repeats them, so the body adds no rows. It read the head whose subject is
"fix(ci-gate): [#2525] avoid an intra-doc link to a private type". Both findings have one root cause,
the format of the tool's input, and are fixed by one change.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2574-f1 | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2574-f2 | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

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

## Processing Log

- 2026-10-20 13:00 UTC - Fetched the review and its two threads, classified and numbered the findings,
  reproduced both with real git output, and fixed F1 and F2 together in one signed commit, with
  regression tests that were seen to fail on the original behavior.
- 2026-10-20 13:07 UTC - Re-ran the evidence with the new input format and committed it separately.
- 2026-10-20 13:10 UTC - Ran the pre-push checks, pushed, and, after the push, replied on both
  threads.
- 2026-10-20 13:12 UTC - Started this record, with a reply URL in every row.

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
