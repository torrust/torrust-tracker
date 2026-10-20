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

One review was submitted so far. Copilot's review 5588853094 read the head whose subject is "docs(issues):
[#2525] record the verdict decisions and evidence, and mark T3 done". It carries one inline thread, F1.
Its overview repeats that thread, so the body adds no rows.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2580-f1 | Copilot | Medium (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

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

## Processing Log

- 2026-10-20 17:19 UTC - Fetched Copilot's review and its one thread, confirmed the finding by running the
  real binary with an extra field at both levels, and wrote the failing tests first.
- 2026-10-20 17:21 UTC - Fixed it in one signed commit, with mutation checks per nested type, and recorded
  the round in the specification and the evidence.
- 2026-10-20 17:23 UTC - Ran the pre-push checks, pushed, and, after the push, replied on the thread.
- 2026-10-20 17:25 UTC - Started this record, with the reply URL in the row.

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
