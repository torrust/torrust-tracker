---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2671 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2671>.

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

One review was submitted so far: the review of `copilot-pull-request-reviewer` (5664535906) on the head
whose subject is `docs(issues): [#2670] specify building shipped artifacts without test features`. It
carries no "did not start before the timeout" warning, so it was not re-requested. Its body lists its three
inline threads as F1 to F3 and adds no other row. The Copilot review was requested through the GraphQL
`requestReviews` mutation with the Copilot reviewer bot because the `request_copilot_review` MCP tool failed twice
with `header mismatch: missing Mcp-Param-repo header` and once with the same error for `owner`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2671-f1 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2671-f2 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2671-f3 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Add required test-design review gates

- PR number: 2671
- Source review ID: 5664535906
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2671#discussion_r4386794592>
- Concern: T2 and T3 add tests, but the Regression Test Strategy and the Commit Points omit the test-design
  review gates that `create-issue` requires: a recorded prose-first Arrange-Act-Assert review after each
  passing increment, and a maintainer-review stop after the final increment.
- Solution: the finding was right. The Regression Test Strategy now lists the progressive loop (smallest
  increment, recorded design review before the next increment or commit, review completed for T2 before T3,
  maintainer stop after the final increment of each task, Task Reviewer only for the final pre-PR review). The
  Commit Points rows for T2 and T3 require the recorded review and the maintainer's review before their commit.
- Current-tree verification: `.github/skills/dev/planning/create-issue/SKILL.md` lines 180 to 194 describe the
  loop; `grep -n "Arrange-Act-Assert" docs/issues/open/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md`
  shows the new text.
- Resolution reference: docs(issues): [#2670] require test design reviews and maintainer stops in the spec
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2671#discussion_r4386846364>

### F2 - Require baseline preservation with recorded test-count deltas

- PR number: 2671
- Source review ID: 5664535906
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2671#discussion_r4386794675>
- Concern: T2 adds binary-resolution tests to a module that the container's lifecycle and configuration test
  targets include, so preserving every existing test can still raise the count above T1's baseline; the spec
  required an equal count in T5, AC2 and M2.
- Solution: the finding was right. `tests/lifecycle/signals.rs` and `tests/configuration/cli_configuration.rs`
  include `tests/common/native_tracker/mod.rs` through `#[path]`, so T2's unit tests are compiled into those
  targets. T1 now records the baseline test list, and T5, AC2 and M2 require every baseline test to remain present
  and passing with the delta for new tests recorded.
- Current-tree verification: `grep -n "#\[path" tests/lifecycle/signals.rs tests/configuration/cli_configuration.rs`
  prints both includes; `grep -n "same test\|test count" docs/issues/open/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md`
  no longer finds an equality criterion.
- Resolution reference: docs(issues): [#2670] require baseline tests to remain with a recorded delta
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2671#discussion_r4386846679>

### F3 - Specify Containerfile in baseline build commands

- PR number: 2671
- Source review ID: 5664535906
- Reviewer finding ID: R3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2671#discussion_r4386794731>
- Concern: scenario M1's `docker build --target release .` omits `--file Containerfile`; Docker defaults to
  `Dockerfile`, which the repository root does not have, so the baseline command fails.
- Solution: the finding was right. M1 now builds both targets with `--file Containerfile` and a tag, as
  `docs/containers.md` does.
- Current-tree verification: `ls Dockerfile` reports no such file; `grep -n "docker build" docs/issues/open/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md`
  shows `--file Containerfile` in M1.
- Resolution reference: docs(issues): [#2670] select the Containerfile in the baseline build commands
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2671#discussion_r4386846969>

## Processing Log

- 2026-10-27 09:09 UTC - The review of `copilot-pull-request-reviewer` arrived with three inline threads, F1 to
  F3. Each was checked against the tree and confirmed. Fixed in one signed commit each and pushed, then replied
  on each thread. The threads are resolved after this record is committed and pushed.

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
