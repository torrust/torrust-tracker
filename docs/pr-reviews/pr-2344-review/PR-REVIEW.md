---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/pr-reviews/pr-2339-review/PR-REVIEW.md
    - docs/issues/closed/2333-2278-fetch-all-review-threads/ISSUE.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore unreassigned -->

# PR #2344 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2344>.

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

Audit IDs `F1`-`F2` are the Copilot review 5325624421 threads, which carry no finding IDs. The
da2ce7 review 5325898273 numbered its findings `F1`-`F4`; those collide with the Copilot rows and
are recorded as audit `F3`-`F6` (reviewer `F<k>` is audit `F<k+2>`), with the reviewer's ID in each
detail entry. Copilot rated both of its findings "Medium", which is not in the severity vocabulary,
so they are recorded as `Minor (inferred)`.

Round 2 findings `F5`-`F7` from da2ce7 (reviews 5326065019 and 5326267066) collide with audit
`F5`-`F6` and are recorded as audit `F7`-`F9` (reviewer `F<k>` is audit `F<k+2>`).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2344-f1` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2344-f2` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2344-f3` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2344-f4` | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2344-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2344-f6` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2344-f7` | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2344-f8` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2344-f9` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The fallback-query helper is tied to the skill's `-f query='...'` layout

- PR number: 2344
- Source review ID: 5325624421
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111103143>
- Concern: `skill_fallback_query` finds the query by its `-f query='...'` shell layout, so a change
  of quoting or wrapping in the skill breaks it; a fenced or marked block, or a shared source, was
  suggested.
- Solution: kept the extraction, because a skill is plain Markdown and cannot include a shared
  file, and the single `-f query='` block is the documented fallback itself. The helper's doc
  comment now states that a layout change fails loudly and never passes falsely. Round 2 (F7)
  showed that wording overstated what the helper checked; it was narrowed and an exactly-one-block
  assertion added.
- Current-tree verification: `git grep -n "a missing or second block fails here" HEAD -- contrib/dev-tools/github/github-review-threads/src/lib.rs`
  matches one line, the doc comment above `fn skill_fallback_query`; `cargo test --package github-review-threads` passes 15 unit
  and 7 CLI tests (stable Rust 1.98.1).
- Resolution reference: `test(dev-tools): document fallback-query extraction and evidence-window intent`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111444531>

### F2 - The evidence-field assertion pins token order and adjacency

- PR number: 2344
- Source review ID: 5325624421
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111103150>
- Concern: the window assertion fails on a semantics-preserving reorder; either document why order
  and adjacency are required or check the fields independently.
- Solution: kept the adjacency and documented it in the Arrange comment: `path` occurs only on
  threads, so requiring `line` and `resolvedBy { login }` next to it pins them at thread level,
  whereas `login` also occurs under comment authors.
- Current-tree verification: `git grep -n "adjacency pins them" HEAD -- contrib/dev-tools/github/github-review-threads/src/lib.rs`
  matches one line, the Arrange comment of `it_should_request_the_line_and_resolver_of_each_thread`; the test suite passes as in F1.
- Resolution reference: `test(dev-tools): document fallback-query extraction and evidence-window intent`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111444602>

### F3 - The post-merge steps assume the merged PR already has an audit

- PR number: 2344
- Source review ID: 5325898273
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111358649>
- Concern: version 1.3 brings findings "not yet audited" at merge into the workflow, but step 2
  records approval "in the original audit", step 3 normalizes into "the merged PR's existing
  audit", and the checklist names the "original audit"; PR #2339 had no audit.
- Solution: step 2 now says the durable approval comment alone satisfies the gate until the
  follow-up creates the audit; step 3 says the follow-up branch creates
  `docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md` with the approval URL in its Ownership
  section; the checklist item names "the audit the follow-up created".
- Current-tree verification: `git grep -n "or the audit the follow-up created" HEAD -- .github/skills/dev/pr-reviews/process-pr-review/SKILL.md`
  matches one line, the post-merge follow-up checklist item; `git grep -n "durable comment alone satisfies this gate\|the follow-up branch creates" HEAD -- .github/skills/dev/pr-reviews/process-pr-review/SKILL.md`
  matches the step 2 and step 3 sentences; `linter markdown` exits `0`.
- Resolution reference: `docs(pr-reviews): cover post-merge follow-ups for PRs without an audit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111444681>

### F4 - The PR #2334 audit's frontmatter still points at the old `open/` path

- PR number: 2344
- Source review ID: 5325898273
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111358656>
- Concern: after the #2333 archive, `docs/pr-reviews/pr-2334-review/PR-REVIEW.md:7` lists a path
  that no longer exists; `develop` commit `2e36b46a` treats this field as navigational.
- Solution: pointed the frontmatter entry at `docs/issues/closed/2333-2278-fetch-all-review-threads/ISSUE.md`;
  the dated narrative at lines 56, 62, and 77 and the EPIC 16:07 log line keep the path as it was.
- Current-tree verification: `git grep -n "issues/open/2333" HEAD -- ':!docs/pr-reviews/pr-2344-review'`
  matches four historical lines: the EPIC's 2026-09-24 16:07 UTC log entry and three narrative
  lines of the PR #2334 audit. The exclusion stops this entry from matching its own command. The
  PR #2334 audit's frontmatter names the `closed/` path.
- Resolution reference: `docs(pr-reviews): point PR #2334 audit to the archived #2333 spec`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111444733>

### F5 - The evidence correction cites the finding by prose instead of its reference

- PR number: 2344
- Source review ID: 5325898273
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111358662>
- Concern: the #2333 evidence correction note cites "PR #2339 review finding F3", while repository
  artifacts should cite the immutable `review-finding:pr-<N>-<id>` reference.
- Solution: the note now cites (`review-finding:pr-2339-f3`).
- Current-tree verification: `git grep -n "review-finding:pr-2339-f3" HEAD -- docs/issues/closed/2333-2278-fetch-all-review-threads/manual-verification-evidence.md`
  matches one line, the correction note.
- Resolution reference: `docs(issues): cite the #2333 evidence correction by its finding reference`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111444792>

### F6 - `Reviewer finding ID` repeats IDs that were not reassigned

- PR number: 2344
- Source review ID: 5325898273
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111358666>
- Concern: the PR #2339 audit records `F1`-`F3` as reviewer IDs although its audit IDs equal them;
  the skill asks for `N/A` unless the ID was reassigned.
- Solution: the three detail entries now record `Reviewer finding ID: N/A`.
- Current-tree verification: `git grep -c "Reviewer finding ID: N/A" HEAD -- docs/pr-reviews/pr-2339-review/PR-REVIEW.md`
  counts 3; `validate-audit-record.py --pr-number 2339` exits `0` (3 rows, 0 failures).
- Resolution reference: `docs(pr-reviews): use N/A for unreassigned reviewer IDs in PR #2339 audit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111444836>

### F7 - The fallback-query doc comment promises more than the helper checks

- PR number: 2344
- Source review ID: 5326065019
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111523041>
- Concern: the F1 doc comment said a layout change "fails loudly here, never silently", but the
  helper read only the first `-f query='` block, so a second block could drift while both tests
  pass; the F1 Solution repeated the claim.
- Solution: `skill_fallback_query` now asserts that the skill holds exactly one `-f query='`
  block, and its doc comment says a missing or second block fails there and a changed one fails
  the parity test. The F1 Solution notes the narrowing.
- Current-tree verification: with the old helper restored in the working tree, appending a second
  fallback block to the skill passed all 15 unit tests; with the new helper, the same mutant fails
  `it_should_document_the_same_fallback_query_the_tool_sends`. Both files were restored before
  committing, and the suite passes 15 unit and 7 CLI tests (stable Rust 1.98.1).
- Resolution reference: `test(dev-tools): require exactly one skill fallback query block`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4112885461>

### F8 - The F4 verification's recorded result cannot be reproduced

- PR number: 2344
- Source review ID: 5326065019
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111523043>
- Concern: F4's `git grep -n "issues/open/2333" HEAD` had no path filter, so it also matched the
  audit line quoting it and returned five lines, not the recorded four.
- Solution: F4's command now excludes this audit, and its result names the four historical lines
  by content.
- Current-tree verification: `git grep -n "issues/open/2333" HEAD -- ':!docs/pr-reviews/pr-2344-review'`
  returns the four lines F4 names.
- Resolution reference: `docs(pr-reviews): make PR #2344 audit verifications reproducible at head`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4112885553>

### F9 - The F3 verification cited a line number the rebase moved

- PR number: 2344
- Source review ID: 5326267066
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4111710920>
- Concern: after the rebase onto `4fd1876c`, F3's recorded "line 293" is line 297, so the result
  no longer reproduces.
- Solution: F3 now records matched text instead of line numbers, and so do F1, F2, and F5, whose
  cited `lib.rs` and evidence lines this round's changes could also move.
- Current-tree verification: a search of this audit for line-number results (`at line N`,
  `matches line N`, `lines N-M carry`) finds none; each F1-F5 command returns the text its entry names.
- Resolution reference: `docs(pr-reviews): make PR #2344 audit verifications reproducible at head`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2344#discussion_r4112885623>

## Processing Log

- 2026-09-26 13:03 UTC - Started audit for round 1: Copilot review 5325624421 (two inline
  threads, no finding IDs) and da2ce7 review 5325898273 (approval with four inline findings; its
  body lists the same four and assesses the two Copilot threads, so it has no extra row). Rebased
  the branch onto `develop` `0f1dcd28`, committed one fix per finding (F1 and F2 share one commit),
  force-pushed with a lease on the reviewed head, re-derived each fix from the pushed tree, and
  replied on all six threads.
- 2026-09-26 13:09 UTC - After `docs(pr-reviews): add PR #2344 review audit` was pushed,
  `reply-status --login josecelano` reported 6 of 6 threads replied, and the six threads were
  resolved. A refreshed GraphQL fetch reports 6 threads, 0 unresolved, all resolved by `josecelano`.
- 2026-09-26 21:40 UTC - Round 2: three da2ce7 `[Nit]` threads (reviews 5326065019 and
  5326267066), recorded as F7-F9, arrived beside approvals at `5213cf15`. Fixed in
  `test(dev-tools): require exactly one skill fallback query block` and
  `docs(pr-reviews): make PR #2344 audit verifications reproducible at head`, pushed, and replied
  on all three threads. Correction: that second commit changed in place the `Current-tree
  verification` of F1-F5 (line numbers replaced by matched text, and F4's command given a
  path filter) and added a sentence to F1's Solution; earlier log entries are unchanged.
- 2026-09-26 21:44 UTC - After `docs(pr-reviews): record PR #2344 review round 2` was pushed,
  `reply-status --login josecelano` reported 3 of 3 unresolved threads replied, and the three
  round-2 threads were resolved. A refreshed GraphQL fetch reports 9 threads, 0 unresolved.

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
