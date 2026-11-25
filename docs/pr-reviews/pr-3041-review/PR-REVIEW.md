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

Review 6057005909 (CodeRabbit, 2026-11-25 11:45 UTC, on the head `docs(pr-reviews): [#2996] record review round 2 of PR 3041 in its audit`) posted one inline comment, F6. Its body repeats F6 and adds a failed "Out of Scope Changes" pre-merge check, F7, which has no thread.

Review 6057270149 (da2ce7, APPROVED, 2026-11-25 12:04 UTC, on the head `docs(pr-reviews): [#2996] record the replies of round 3 in the PR 3041 audit`) posted four inline Nits, F8 to F11. Its body lists the same four and says none blocks, so it adds no row. Its body also notes reply 4694522201 (CodeRabbit, 11:57 UTC) on the F6 thread, which confirms the fix and raises nothing new, so it adds no row either.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3041-f1 | Unknown | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3041-f2 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3041-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3041-f4 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3041-f5 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-3041-f6 | Unknown | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3041-f7 | Unknown | Minor (inferred) | other | ORIGINAL | NO_ACTION | NON_RESOLVABLE |
| F8 | review-finding:pr-3041-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3041-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3041-f10 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3041-f11 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |

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
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694352682>

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
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694353090>

### F4 - Refresh `last-updated-utc` of the spec

- PR number: 3041
- Source review ID: 6054002314
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4692111210>
- Concern: the spec frontmatter still read `last-updated-utc: "2026-11-23 15:53"` although this PR changes the file.
- Solution: the frontmatter now reads `2026-11-25 11:37`, the time of this round's spec update, and the evidence file's stamp moved with it.
- Current-tree verification: `grep -n last-updated-utc docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/*.md` prints the time of each file's last change: `2026-11-25 11:37` for `ISSUE.md`, and a later stamp for `manual-verification-evidence.md`, which F6 and F11 changed again.
- Resolution reference: docs(issues): [#2996] cite each acceptance criterion's own evidence and refresh the SI-8 spec metadata
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694353579>

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
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694354098>

### F6 - Cite the round-2 findings by their audit references

- PR number: 3041
- Source review ID: 6057005909
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694393004>
- Concern: `manual-verification-evidence.md` cited "review finding F4", "F2" and "F1" of PR 3041, which are da2ce7's own numbers; in this audit those findings are F5, F3 and F2, and F4, F2 and F1 name other findings.
- Solution: the three citations now use the audit references `review-finding:pr-3041-f5`, `review-finding:pr-3041-f3` and `review-finding:pr-3041-f2`, and the file's `last-updated-utc` is refreshed. The round-2 commit bodies keep da2ce7's numbers, prefixed with the reviewer's name; they are history and stay unchanged.
- Current-tree verification: `grep -n "review-finding\|finding F" docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/manual-verification-evidence.md` prints only the three `review-finding:pr-3041-f*` references.
- Resolution reference: docs(issues): [#2996] cite the PR 3041 review findings by their audit references in the SI-8 evidence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694474088>

### F7 - Remove the review audit record from this PR

- PR number: 3041
- Source review ID: 6057005909
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#pullrequestreview-6057005909>
- Concern: the review body's failed "Out of Scope Changes" pre-merge check asks to remove `docs/pr-reviews/pr-3041-review/PR-REVIEW.md` from this PR or move it to a separate review-process change.
- Solution: no change. The repository's `process-pr-review` skill makes the PR author own this tracked audit record at `docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md` on the PR's own branch, created before the first reply; moving it out would break that workflow. CodeRabbit read no configuration file from this repository, so its scope check does not know the rule.
- Current-tree verification: `.claude/skills/process-pr-review/SKILL.md` opens with "The PR author owns one tracked audit record at `docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md`".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/3041#issuecomment-7161636511>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#issuecomment-7161636511>

### F8 - Name the head that review 6057005909 saw

- PR number: 3041
- Source review ID: 6057270149
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694605342>
- Concern: the Findings intro named the head of CodeRabbit review 6057005909 by the subject of its parent commit; the review was posted on the commit `docs(pr-reviews): [#2996] record review round 2 of PR 3041 in its audit`.
- Solution: the intro now names that commit's subject.
- Current-tree verification: `git log -1 --format=%s` of the review's `commit_id` (from `gh api repos/torrust/torrust-tracker/pulls/3041/reviews/6057005909`) prints `docs(pr-reviews): [#2996] record review round 2 of PR 3041 in its audit`, the subject the intro now quotes.
- Resolution reference: docs(pr-reviews): [#2996] record review round 4 of PR 3041 and correct three audit entries
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694763427>

### F9 - Keep the F4 stamp check true for the current tree

- PR number: 3041
- Source review ID: 6057270149
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694605359>
- Concern: F4's verification said the `last-updated-utc` grep prints `2026-11-25 11:37` for both spec files, but F6 later refreshed the evidence file's stamp, so the sentence no longer described the current tree.
- Solution: the sentence now says each file carries the time of its last change, `2026-11-25 11:37` for `ISSUE.md` and a later stamp for the evidence file, which F6 and F11 changed again.
- Current-tree verification: `grep -n last-updated-utc docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/*.md` prints `2026-11-25 11:37` for `ISSUE.md` and `2026-11-25 12:11` for `manual-verification-evidence.md`.
- Resolution reference: docs(pr-reviews): [#2996] record review round 4 of PR 3041 and correct three audit entries
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694763965>

### F10 - Class da2ce7's findings as `Human`

- PR number: 3041
- Source review ID: 6057270149
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694605367>
- Concern: rows F2 to F5, from review 6054002314, gave the author class `Unknown`, but the `process-pr-review` skill classes a human account as `Human` and its author is a `User` account.
- Solution: rows F2 to F5, and the new rows F8 to F11 of the same account, read `Human`. F1, F6 and F7 (Greptile and CodeRabbit, bot accounts that are not Copilot) stay `Unknown`.
- Current-tree verification: `gh api users/da2ce7 --jq .type` prints `User`; `grep -n "^| F" docs/pr-reviews/pr-3041-review/PR-REVIEW.md` shows `Human` on F2 to F5 and F8 to F11.
- Resolution reference: docs(pr-reviews): [#2996] record review round 4 of PR 3041 and correct three audit entries
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694764349>

### F11 - Name the message a fragment case fails to reject

- PR number: 3041
- Source review ID: 6057270149
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694605383>
- Concern: when a case was not rejected, the test reported only `it should reject the message: ()`, without naming which of the seven messages slipped through.
- Solution: the test binds the panic payload with `let Err(panic_payload) = catch_unwind(...) else { panic!("it should reject {message:?}") };`, so an accepted message is named.
- Current-tree verification: `cargo test -q -p torrust-tracker-test-helpers` passes (10 tests); with `"rustlib/src"` removed on a scratch change, the test failed with `it should reject "failed in /toolchain/lib/rustlib/src"` (V4 re-run in `manual-verification-evidence.md`); the file was restored.
- Resolution reference: test(test-helpers): [#2996] name the message a fragment case fails to reject
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3041#discussion_r4694764807>

## Processing Log

- 2026-11-24 18:01 UTC - Started audit for review 6044596970 (Greptile); F1 fixed in a signed commit, reply and thread resolution follow after the push.
- 2026-11-24 18:04 UTC - Replied on the F1 thread (reply created 18:04:39 UTC) after pushing the fix and the audit, and resolved the thread; a GraphQL refresh shows it resolved and no other thread.
- 2026-11-25 11:38 UTC - Added F2 to F5 for review 6054002314 (da2ce7, approved); all four fixed in three signed commits; replies and thread resolution follow after the push.
- 2026-11-25 11:42 UTC - Replied on the F2 to F5 threads after pushing the fixes and the audit, and resolved the four threads; a GraphQL refresh shows 0 unresolved threads.
- 2026-11-25 11:49 UTC - Added F6 and F7 for review 6057005909 (CodeRabbit); F6 fixed in a signed commit, F7 declined (the audit record belongs to the PR by the `process-pr-review` skill); replies follow after the push.
- 2026-11-25 11:53 UTC - Replied on the F6 thread after the push (CodeRabbit then resolved it) and answered F6 and F7 in a PR conversation comment; a GraphQL refresh shows all six threads resolved.
- 2026-11-25 12:13 UTC - Added F8 to F11 for review 6057270149 (da2ce7, approved); F11 fixed in a signed test commit; F8, F9 and F10 corrected in this record (F10 also changed the author class of F2 to F5 to `Human`); replies follow after the push.
- 2026-11-25 12:18 UTC - Replied on the F8 to F11 threads after pushing the fixes and the audit, and resolved the four threads; a GraphQL refresh shows 0 unresolved threads.

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
