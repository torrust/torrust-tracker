---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2874 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2874.

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

Review 5917243839 by da2ce7 (`APPROVED`, 2026-11-16 08:50:01 UTC, at the head ending in `docs(issues):
[#2393] record a full-suite run after the rebase onto develop`) raised six inline findings. Its
body's other paragraphs are a summary and checks, with no request.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2874-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2874-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2874-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2874-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2874-f5 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2874-f6 | Human | Suggestion | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Post-rebase run stamped later than its commit

- PR number: 2874
- Source review ID: 5917243839
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583631168
- Concern: The run after the rebase was stamped 2026-11-16 08:35 UTC, later than the 08:23:00Z commit that records it, and the evidence `last-updated-utc` (08:25) was earlier than that section.
- Solution: The 08:35 stamp was a guess. The section now gives the real interval, between the rebased commit (08:22:32 UTC) and the commit that records the run (08:23:00 UTC), names the tree it ran on, and `last-updated-utc` is set to the time of this fix. The same sweep found the same mistake in the previous progress-log entry of `ISSUE.md` (08:25, while its commit was authored at 08:18:46), now stamped 08:18.
- Current-tree verification: `TZ=UTC git log --format='%h %cd %ad %s' --date=format-local:%FT%T` shows the rebased commit at 08:22:32 and the recording commit at 08:23:00; the evidence section now states that interval.
- Resolution reference: docs(issues): [#2393] correct the run times and record the maintainer review and repeated green runs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583686715

### F2 - Description claims 20 of 20 green runs for every V5 group

- PR number: 2874
- Source review ID: 5917243839
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583631185
- Concern: The PR description says each V5 group then passed 20 of 20 runs, but the T8 table and the helper group recorded only one pass.
- Solution: Ran the T8 contract tests and the helper tests 20 times each on the current code and recorded both repeated greens in V5, so the description's claim now holds for every group.
- Current-tree verification: 20 of 20 runs passed for each filter, 2026-11-16 08:53 to 08:54 UTC; V5 records them under T8 and the helper.
- Resolution reference: docs(issues): [#2393] correct the run times and record the maintainer review and repeated green runs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583687124

### F3 - AC7 cites a maintainer review that V7 does not record

- PR number: 2874
- Source review ID: 5917243839
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583631194
- Concern: AC7 and M7 cite V7 for the maintainer's review of the final increment, but V7 recorded no such review, and no decision on the REST API Complex Arrange it left for that review.
- Solution: Added a V7 entry, "Maintainer review of the final increment", with when and where the review happened, what it covered, and its outcome: T8 and T9 approved with no change, and the REST API Complex Arrange kept as it is. AC7 now names that entry.
- Current-tree verification: The new entry is the last V7 subsection of `manual-verification-evidence.md`, and AC7 in `ISSUE.md` names it.
- Resolution reference: docs(issues): [#2393] correct the run times and record the maintainer review and repeated green runs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583687521

### F4 - Last acceptance criterion left unchecked without a reason

- PR number: 2874
- Source review ID: 5917243839
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583631202
- Concern: The checkpoint for reviewed acceptance criteria is ticked, but "Relevant focused tests and pre-push checks pass" stayed unchecked with no reason.
- Solution: Ticked it with its evidence: the focused V5 runs, the pre-commit hook on every commit, and the pre-push hook that `git-bot-push` runs on each push and that blocks a failing push.
- Current-tree verification: `ISSUE.md` now has no unchecked acceptance criterion.
- Resolution reference: docs(issues): [#2393] correct the run times and record the maintainer review and repeated green runs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583687926

### F5 - Rewritten comment drops the no-retries guard

- PR number: 2874
- Source review ID: 5917243839
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583631209
- Concern: The rewritten comment of `available_http_bind_address` dropped the warning against retries or waits, which still applies because a socket bound to `0.0.0.0` can still take the port.
- Solution: Restored the warning, naming the handoff risk that remains (a socket bound to `0.0.0.0` on the same port).
- Current-tree verification: `git grep -n 'Do not add retries' -- '*.rs'` finds the sentence in `packages/axum-http-server/src/server.rs`; the package tests pass.
- Resolution reference: test(http-server): [#2393] keep the no-retries guard on the released test address
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583688338

### F6 - Two copies of the one-service registration

- PR number: 2874
- Source review ID: 5917243839
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583631212
- Concern: `registar_with_one_service` repeats the body of `readiness::registar_with_a_probed_service`.
- Solution: `registar_with_a_probed_service` now calls `registar_with_one_service` with its fixed binding and role, so the file has one registration path; the imports it no longer needs are removed.
- Current-tree verification: `cargo nextest run -p torrust-tracker-axum-health-check-api-server -p torrust-tracker-axum-http-server --all-features --no-fail-fast`: 135 tests run, 135 passed.
- Resolution reference: test(health-check): [#2393] register the readiness tests' probed service through the shared helper
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2874#discussion_r4583688624

## Processing Log

- 2026-11-16 08:55 UTC - Started audit for review 5917243839; F1 to F6 fixed in three commits.
- 2026-11-16 08:56 UTC - Replied on the six threads of review 5917243839 before pushing the fixes.

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
