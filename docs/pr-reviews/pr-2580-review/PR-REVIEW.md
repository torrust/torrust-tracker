---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - contrib/dev-tools/checks/ci-gate/src/verdict.rs
    - docs/issues/open/2525-ci-gate-required-check/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2580 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2580>.

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

Two reviews were submitted so far, both by Copilot. Each body repeats its inline threads, so the bodies add
no rows.

- Review 5588853094 read the head whose subject is "docs(issues): [#2525] record the verdict decisions and
  evidence, and mark T3 done". It carries one inline thread, F1.
- Review 5589209156 read the head whose subject is "docs(pr-reviews): add the PR #2580 review audit". It
  carries five inline threads, F2 to F6. The threads of F4 and F5 ask for the same change to the same
  row, so F5 is recorded as a re-raise of F4.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2580-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2580-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2580-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2580-f4 | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2580-f5 | Copilot | Nit (inferred) | metadata | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F6 | review-finding:pr-2580-f6 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Unknown fields are accepted inside the nested classification objects

- PR number: 2580
- Source review ID: 5588853094
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4327983139>
- Concern: `deny_unknown_fields` on `VerdictInput` is not recursive. `Classification` and `GroupDecision`
  ignore extra fields, so an extra field inside either object of an otherwise passing input still gives
  `pass`, instead of the rejection the documentation promises. Deny unknown fields on both nested types
  and test both nesting levels.
- Solution: `deny_unknown_fields` is now on `Classification` and `GroupDecision`. One test case per
  nesting level inserts an unknown field into an otherwise valid input and expects the input to be
  rejected.
- Current-tree verification: before the fix, the real binary printed `pass` with exit code 0 for an
  extra field inside the classification and again inside a group decision, and both test cases failed.
  After it, the same inputs exit with 1, name the unknown field, and print nothing on stdout. Removing
  the attribute from either type fails that type's own case. `cargo test -p ci-gate` passes with 134 unit
  and 10 command-line tests.
- Resolution reference: fix(ci-gate): [#2525] reject unknown fields at every level of the verdict input
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328057398>

### F2 - The problems follow the classification order, not the documented group order

- PR number: 2580
- Source review ID: 5589209156
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328266639>
- Concern: the documentation says the problems follow the order of `JobGroup::ALL`, but the code keeps the order in which the classification lists the groups, so a classification in another order changes the problems and the explanation. Build the problems in group order.
- Solution: the needed groups are derived once, in `JobGroup::ALL` order, and both the problems and the explanation of a passing verdict use them. A test gives a classification whose groups are reversed and expects both explanations in group order.
- Current-tree verification: before the fix the new test failed, with `coverage (failure), testing (missing), docs-lint (failure)` where group order was expected; after it, `cargo test -p ci-gate` passes with 135 unit and 10 command-line tests.
- Resolution reference: fix(ci-gate): [#2525] list the verdict's groups in group order
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328367462>

### F3 - The evidence shows the override explanation without its backticks

- PR number: 2580
- Source review ID: 5589209156
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328266722>
- Concern: the explanation recorded in the V7 evidence omits the backticks that the tool prints around the actor's login, so a table meant as a verbatim observation does not match the output.
- Solution: the cell now quotes the explanation verbatim in a double-backtick code span, which holds the inner backticks without the lint complaining that had led to dropping them.
- Current-tree verification: the real command, run with `--override-actor josecelano` on a change to `src/main.rs`, printed the same text as the cell; markdown, spell and link checks pass.
- Resolution reference: docs(issues): [#2525] quote the override explanation verbatim in the V7 evidence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328367869>

### F4 - The audit records a severity that it does not define

- PR number: 2580
- Source review ID: 5589209156
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328266797>
- Concern: the audit defines the severities `Blocker`, `Major`, `Minor`, `Nit`, and `Suggestion`, but the row of F1 records `Medium (inferred)`. Map it to a defined value.
- Solution: the row of F1 records `Minor (inferred)`: the finding is a gap between the documentation and the behavior, and no verdict was wrong.
- Current-tree verification: the audit has no `Medium` left, and the audit validator passes.
- Resolution reference: docs(pr-reviews): use a defined severity value in the PR #2580 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328368150>

### F5 - The audit records a severity that it does not define (same row)

- PR number: 2580
- Source review ID: 5589209156
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328266866>
- Concern: the same concern as F4, raised on the table row instead of the vocabulary line: `Medium (inferred)` is not a defined severity value.
- Solution: the same change as F4.
- Current-tree verification: the audit has no `Medium` left, and the audit validator passes.
- Resolution reference: docs(pr-reviews): use a defined severity value in the PR #2580 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328368447>

### F6 - The contract table row no longer says the tool writes JSON diagnostics

- PR number: 2580
- Source review ID: 5589209156
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328266934>
- Concern: the row of `ci-gate` in the CLI output contract table used to mention JSON diagnostics, which the tool still writes to stderr under the contract; keep the note so the table stays uniformly informative.
- Solution: the row says `Needed job groups or the merge verdict as JSON; JSON diagnostics`, in the width of the column. I had dropped the note only to fit the column when I added the verdict.
- Current-tree verification: the row is within the width of the table, and the markdown check passes.
- Resolution reference: docs(adrs): [#2525] keep the JSON diagnostics note in the ci-gate contract row
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2580#discussion_r4328368752>

## Processing Log

- 2026-10-20 17:19 UTC - Fetched Copilot's review and its one thread, confirmed the finding by running the
  real binary with an extra field at both levels, and wrote the failing tests first.
- 2026-10-20 17:21 UTC - Fixed it in one signed commit, with mutation checks per nested type, and recorded
  the round in the specification and the evidence.
- 2026-10-20 17:23 UTC - Ran the pre-push checks, pushed, and, after the push, replied on the thread.
- 2026-10-20 17:25 UTC - Started this record, with the reply URL in the row.
- 2026-10-20 17:46 UTC - Copilot's second review arrived with five threads, F2 to F6. Each was checked
  against the tree and all were confirmed; F5 asks for the same change as F4.
- 2026-10-20 17:54 UTC - Fixed each finding in its own signed commit (F4 and F5 together), recorded the
  round in the specification and the evidence, ran the pre-push checks, pushed, and, after the push,
  replied on the five threads.
- 2026-10-20 17:56 UTC - Added F2 to F6 and their reply URLs to this record.

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
