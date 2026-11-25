---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #2996"
---

<!-- skill-link: process-pr-review -->

# PR #3041 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/3041>.

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

Review 6044596970 (Greptile, 2026-11-24 17:57 UTC) posted one inline comment, F1. Its summary comment 7139941117 only repeats it, so it adds no row.

Review 6054002314 (da2ce7, APPROVED, 2026-11-25 08:17 UTC) posted four inline comments, F2 to F5 here (its own F1 to F4). Its body lists the same four findings and says none blocks, so it adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3041-f1 | Unknown | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3041-f2 | Unknown | Minor | testing | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-3041-f3 | Unknown | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F4 | review-finding:pr-3041-f4 | Unknown | Nit | metadata | ORIGINAL | FIXED | OPEN |
| F5 | review-finding:pr-3041-f5 | Unknown | Nit | formatting | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - Import `catch_unwind` instead of calling it by its full path

- PR number: 3041
- Source review ID: 6044596970
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4684295605>
- Concern: the new unit test in `packages/test-helpers/src/assertions.rs` called `std::panic::catch_unwind` by its full path, while `AGENTS.md` asks for short imported names unless a name conflict needs the full path.
- Solution: the test module imports `std::panic::catch_unwind` at its top and calls `catch_unwind` directly; no name conflict needs the full path.
- Current-tree verification: `grep -n "catch_unwind" packages/test-helpers/src/assertions.rs` prints the `use std::panic::catch_unwind;` line and one call `catch_unwind(...)`; `cargo test -p torrust-tracker-test-helpers` passes (10 tests) and `linter all` exits 0.
- Resolution reference: refactor(test-helpers): [#2996] import catch_unwind in the assertions test module
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4684386574>

### F2 - Pin the `rustlib/src` fragment in the shared test

- PR number: 3041
- Source review ID: 6054002314
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4692111193>
- Concern: the test message for `rustlib/src` also held `src/`, which the loop checks first, so the test still passed with `"rustlib/src"` removed from the fragment list, while AC1 and the description claim a test for each of the seven fragments.
- Solution: the test now pairs each fragment with a message that holds only that fragment (the `rustlib/src` message ends at the fragment) and checks that the panic text names the expected fragment, so a fragment removed from the list fails its own case.
- Current-tree verification: `cargo test -q -p torrust-tracker-test-helpers` passes (10 tests); on a scratch change that removed `"rustlib/src"` from the list, the test failed with "it should reject the message" (`manual-verification-evidence.md` V4); the file was restored.
- Resolution reference: test(test-helpers): [#2996] check that each rejected message names its own fragment
- Follow-up PR URL: N/A
- Reply URL: N/A

### F3 - Give each acceptance row its own evidence

- PR number: 3041
- Source review ID: 6054002314
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4692111203>
- Concern: the five acceptance-verification cells all read "V1, V2; tests green", and neither V1 nor V2 shows AC4's claim that the panic wording is the same in all five definitions.
- Solution: `manual-verification-evidence.md` gains V3 (`grep -rn 'must not hold a source path' packages`: one identical line per definition) and V4 (the F2 red check), and each acceptance row now cites the evidence that shows it.
- Current-tree verification: `grep -n "AC[1-5] | DONE" docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md` prints five different cells; the V3 command prints five identical lines.
- Resolution reference: docs(issues): [#2996] cite each acceptance criterion's own evidence and refresh the SI-8 spec metadata
- Follow-up PR URL: N/A
- Reply URL: N/A

### F4 - Refresh `last-updated-utc` of the spec

- PR number: 3041
- Source review ID: 6054002314
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4692111210>
- Concern: the spec frontmatter still read `last-updated-utc: "2026-11-23 15:53"` although this PR changes the file.
- Solution: the frontmatter now reads `2026-11-25 11:37`, the time of this round's spec update, and the evidence file's stamp moved with it.
- Current-tree verification: `grep -n last-updated-utc docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/*.md` prints `2026-11-25 11:37` for both files.
- Resolution reference: docs(issues): [#2996] cite each acceptance criterion's own evidence and refresh the SI-8 spec metadata
- Follow-up PR URL: N/A
- Reply URL: N/A

### F5 - Wrap the long cross-link doc lines

- PR number: 3041
- Source review ID: 6054002314
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4692111217>
- Concern: the "This definition is separate because ..." doc line ran to 171, 166, 140 and 132 columns in `primitives`, `http-protocol`, `configuration` and `axum-server`, past `max_width = 130`, while the shared function's copy is wrapped.
- Solution: each of the four lines is wrapped near 100 columns, like the rest of its paragraph. V1 was re-run because the wrap moved four definitions by one line.
- Current-tree verification: `awk 'length>130'` over the five files prints nothing; `cargo +nightly fmt --all -- --check` passes.
- Resolution reference: docs(test-helpers): [#2996] wrap the long cross-link lines of the kept assert_names_no_source_path copies
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-11-24 18:01 UTC - Started audit for review 6044596970 (Greptile); F1 fixed in a signed commit, reply and thread resolution follow after the push.
- 2026-11-24 18:04 UTC - Replied on the F1 thread (reply created 18:04:39 UTC) after pushing the fix and the audit, and resolved the thread; a GraphQL refresh shows it resolved and no other thread.
- 2026-11-25 11:38 UTC - Added F2 to F5 for review 6054002314 (da2ce7, approved); all four fixed in three signed commits; replies and thread resolution follow after the push.

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
