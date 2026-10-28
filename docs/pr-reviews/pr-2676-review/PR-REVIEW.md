---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2676 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2676>.

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

One review was submitted so far: the review of `copilot-pull-request-reviewer` (5674519258) on the head
whose subject is `docs(issues): [#2670] record the pre-PR review, the final image and the completion review`.
It carries no "did not start before the timeout" warning, so it was not re-requested. Its body repeats its
single inline thread, F1, and adds no other row. The Copilot review was requested through the GraphQL
`requestReviews` mutation with the Copilot reviewer bot because the `request_copilot_review` MCP tool failed
with `header mismatch: missing Mcp-Param-owner header` (the same error as on PR #2671 for `repo` and `owner`).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2676-f1 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Resolve relative tracker binary paths before changing working directory

- PR number: 2676
- Source review ID: 5674519258
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2676#discussion_r4394295787>
- Concern: a relative `TORRUST_TRACKER_BINARY`, such as `./target/release/torrust-tracker`, makes the
  `cli-configuration` parent-directory test fail to spawn, because that test changes the child's working directory
  and the executable is then looked up inside the temporary fixture. The reviewer asks to resolve relative
  executable paths against the test process's working directory, keeping the `PATH` lookup for bare names, and to
  add a regression case for the changed-directory fixture.
- Solution: the finding was right, and the documented example in `docs/containers.md` used a relative path, so it
  would have documented a failing invocation. `tracker_binary()` now joins a relative path that has a directory part
  to the test process's working directory (`anchored_at`), and leaves an absolute path and a bare name as they are.
  Three unit tests cover the three shapes. The regression for the changed-directory fixture is the existing
  `it_should_not_search_parent_directories_for_a_relative_cli_configuration_file` test run with a relative override
  (evidence V7); a second test would need to set a process-wide environment variable, which the unit tests avoid.
- Current-tree verification: `TORRUST_TRACKER_BINARY=./target/release/torrust-tracker cargo test -p torrust-tracker
  --test cli-configuration` failed with `NotFound` (21 passed, 1 failed) before the fix and passes 25 tests after it;
  an absolute path passed before and after.
- Resolution reference: fix(tracker): [#2670] anchor a relative TORRUST_TRACKER_BINARY at the test working directory
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2676#discussion_r4397794200>

## Processing Log

- 2026-10-28 07:07 UTC - The review of `copilot-pull-request-reviewer` arrived with one inline thread, F1. It was
  reproduced against the tree and confirmed. Fixed in its own signed commit and pushed, then replied on the thread.
  The thread is resolved after this record is committed and pushed.

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
