---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261007-ai-model-provenance-in-commits/README.md
---

<!-- skill-link: process-pr-review -->

# PR #2486 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2486>.

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

Copilot review 5455025162 left one inline finding, headed `[Major][F1]`, on the `project-automation-is-a-dependency` paragraph under Q11 of the discussion README. The reviewer's ID F1 collides with no earlier finding, so it is kept. The review's overview lists the same finding with a Low severity icon and adds no other request, so it adds no row; the row records the thread's bracket, which the validator compares.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2486-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The project-automation entry states pinning as a fact about the repository

- PR number: 2486
- Source review ID: 5455025162
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4217662427>
- Concern: The `project-automation-is-a-dependency` paragraph opened with "The tools the project runs in continuous integration are locked and fully defined.", which does not hold for the repository: CI selects floating toolchains such as `dtolnay/rust-toolchain@stable` (`.github/workflows/testing.yaml:57`) and `@nightly` (`.github/workflows/generate_coverage_pr.yaml:41`), and the agent profiles enable model invocation without naming a model (`.github/agents/pr-reviewer.agent.md:7`). Because `docs/discussions/AGENTS.md` makes an inaccurate repository claim a round blocker, the reviewer asked for the pinning to be phrased as a requirement of the proposal or qualified. The thread's bracket is `[Major]`; the review overview labels the same finding with a Low severity icon.
- Solution: The opening sentence is replaced by two sentences that state the concept instead of describing the workflows: "A public process attracts provenance of its own accord: what the project's automation runs is visible in tracked bytes and changes only through a pull request, so a model that acts inside it can be named exactly where a personal tool cannot. The record's subject is not that a model was used but the choices it may make in that process, such as certifying a new tagged release; this rule proposes to pin such a model's identity so that a choice can be traced to it." The rest of the entry is unchanged.
- Current-tree verification: At the branch head, `git grep -F 'locked and fully defined'` finds nothing under `docs/discussions`; README line 337 (line 334 on the commit the thread was posted on), the line the thread anchors, opens with the two sentences quoted above; a word diff from the round commit to the fix changes only that opening sentence. Copilot's citations still read as cited (`@stable` at `testing.yaml:57`, `@nightly` at `generate_coverage_pr.yaml:41`, `disable-model-invocation: false` and no model field in `pr-reviewer.agent.md`), and the paragraph no longer states that CI tooling is locked: its remaining statement that such a model's identity is pinned follows the new sentence in which the rule proposes that pinning.
- Resolution reference: `docs(discussions): [#2003] state the pinning premise of my project-automation entry as a concept`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2486#discussion_r4217828891>

## Processing Log

- 2026-10-08 09:58 UTC - Opened the pull request with the round and requested reviews; Copilot's review started at 09:59 UTC.
- 2026-10-08 10:12 UTC - Squashed the branch to one commit, `docs(discussions): [#2003] add my round on AI model provenance in commits`, and force-pushed it at 10:12:29 UTC.
- 2026-10-08 10:13 UTC - Copilot review 5455025162, submitted at 10:13:04 UTC on the head from before the squash, left one inline finding (comment 4217662427), recorded as F1 (`Major`). Its overview labels the finding Low and adds no other request, so it adds no row.
- 2026-10-08 10:30 UTC - Fixed F1 in `docs(discussions): [#2003] state the pinning premise of my project-automation entry as a concept` (author time 10:30:04 UTC) and pushed it; the reply below was posted on that head.
- 2026-10-08 10:31 UTC - Replied `FIXED` on the F1 thread at 10:31:46 UTC with the fix commit's subject and the restated sentences, then resolved the thread.
- 2026-10-08 10:32 UTC - Amended the fix commit under the same subject and author time to restate the sentence as a concept (committer time 10:32:18 UTC) and force-pushed it at 10:32:22 UTC; the reply was edited at 10:32:22 UTC and now names the amended commit.
- 2026-10-08 10:33 UTC - Recorded F1 here from the GitHub capture taken at 10:33 UTC: the pull request's only review thread is resolved and outdated, and no unresolved actionable thread remains.

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
