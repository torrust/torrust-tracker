---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2835 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2835.

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

All four findings are in Copilot's review 5862741540 and are inline threads; each carries its own finding ID.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2835-f1 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2835-f2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2835-r2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2835-r1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Stop the main flow when the identity lookup is interrupted

- PR number: 2835
- Source review ID: 5862741540
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541082521
- Concern: A signal during `api user` ended only `report_identity`, so `wait` could still start another command and a spawn failure could replace the `interrupted` diagnostic.
- Solution: `report_identity` returns the interruption to `main`, which reports `interrupted` and exits 1 before the wait starts.
- Current-tree verification: `main.rs` `report_identity` and its caller; the new scenario `it_should_stop_on_sigterm_during_the_lookup_without_running_another_command` passes.
- Resolution reference: fix(wait-for-pr-review): [#2822] stop on a signal during the identity lookup and test its bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541757954

### F2 - Test the stated lookup bound rather than only an early return

- PR number: 2835
- Source review ID: 5862741540
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541082594
- Concern: The CLI scenario also accepted a four- or six-second lookup, and nothing covered the ten-second cap.
- Solution: `lookup_bound` is extracted with unit tests for both branches of `min(10 s, timeout / 4)`, and the CLI scenario asserts a lower and an upper time.
- Current-tree verification: `main.rs` unit tests `it_should_bound_the_lookup_at_a_quarter_of_a_short_timeout` and `it_should_cap_the_lookup_bound_at_ten_seconds_for_a_long_timeout`; the scenario's two assertions.
- Resolution reference: fix(wait-for-pr-review): [#2822] stop on a signal during the identity lookup and test its bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541758586

### R2 - Complete the stalled-lookup verification before marking M4 done

- PR number: 2835
- Source review ID: 5862741540
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541082663
- Concern: M4 and M4b had ended in `gh_api_error` with an incomplete fake and only M1 and M3 were rerun.
- Solution: M4 and M4b were rerun with the corrected fake and their stdout, exit code and timing are recorded.
- Current-tree verification: The M4 and M4b rows of `manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#2822] keep the manual-verification fake and rerun M4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541759130

### R1 - Preserve the manual-verification fake and setup

- PR number: 2835
- Source review ID: 5862741540
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541082723
- Concern: The fake lived only in a scratchpad outside the repository.
- Solution: The fake is kept as `evidence/fake-gh-command.sh`, embedded in the evidence file with its setup commands.
- Current-tree verification: `evidence/fake-gh-command.sh` and the evidence file's environment section.
- Resolution reference: docs(issues): [#2822] keep the manual-verification fake and rerun M4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541759863

## Processing Log

- 2026-11-11 17:24 UTC - Review 5862741540 (Copilot) submitted on the first head of the PR: F1, F2, R2, R1.
- 2026-11-11 18:24 UTC - The 60-minute wait for da2ce7 ended without a review; the wait stopped and the hand-over says so.
- 2026-11-11 18:27 UTC - Pushed the two commits that fix the four findings (push log time).
- 2026-11-11 18:28 UTC - Replied to and resolved the four threads after that push; the replies cite the pushed commit subjects and their URLs are in the entries.

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
