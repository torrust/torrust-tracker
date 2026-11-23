---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3014 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3014.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F1 | review-finding:pr-3014-f1 | Unknown | Major (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3014-f2 | Unknown | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| R3 | review-finding:pr-3014-r3 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-3014-r2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-3014-r1 | Copilot | Minor | correctness | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Health-check scope has no verification

- PR number: 3014
- Source review ID: 6027255837
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670206351
- Concern: Greptile (P2): the health-check API joined the SI-B scope, but the tasks, regression tests and manual scenarios still covered only the three original servers, although the health-check handler returns probe and join-error text to clients.
- Solution: Added task T7b with its commit point, a health-check regression test (failed probe, panicking probe job, shutting-down reply) and manual scenario M6 to the SI-B specification.
- Current-tree verification: Read `packages/axum-health-check-api-server/src/handlers.rs` on the branch: `probe_services` puts `the health check job failed: <join error>` and the probe error into the client report.
- Resolution reference: docs(issues): [#3000] split the health-check reply statuses and add its verification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670523578

### F2 - Health-check failure status recorded wrongly

- PR number: 3014
- Source review ID: 6027369330
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670298238
- Concern: Greptile (P2): the inventory said the health-check API returns 503 for failed probes; failed probes return 200 with `health check failed`, and only the not-ready path returns 503 with `shutting down`.
- Solution: The SI-B background now records the shutting-down reply (503) and the failed-probe reply (200) separately, and says the statuses stay.
- Current-tree verification: `handlers.rs:28-33` returns `StatusCode::SERVICE_UNAVAILABLE` only when `readiness.is_ready()` is false; `probe_services` returns `Json<Report>` (200) from `responses::error("health check failed", ...)`. Tests `tests/server/contract.rs:147-188` and `:711-741` exist on the branch.
- Resolution reference: docs(issues): [#3000] split the health-check reply statuses and add its verification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670524038

### R3 - CI-tools draft lacks the test-development gate

- PR number: 3014
- Source review ID: 6027563286
- Reviewer finding ID: R3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670461881
- Concern: Copilot: T2 and T3 produce tests, but the draft did not require the red/green loop, the recorded prose-first Arrange-Act-Assert review after each increment, or the maintainer checkpoint that `docs/templates/ISSUE.md:118-130` requires.
- Solution: Added the test-development loop paragraph used by the sibling specifications, task T4b (maintainer checkpoint), AC4 and its verification row, and changed the T2 and T3 commit policy to name the red and green runs and the review.
- Current-tree verification: Compared with `docs/templates/ISSUE.md:118-130` and the "Implementation Plan" paragraph of `docs/issues/open/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md`.
- Resolution reference: docs(issues): add the test loop and per-site log scenarios to the CI-tools draft
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670524934

### R2 - CI-tools manual check misses the DEBUG and compose sites

- PR number: 3014
- Source review ID: 6027563286
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670461936
- Concern: Copilot: the E2E runner fixes tracing at INFO, so the DEBUG `docker run` event never appears, and the compose command paths are not exercised; a clean M1 log cannot show those sites are redacted.
- Solution: Split the manual check into M1 (INFO configuration lines), M2 (`docker run` event at DEBUG) and M3 (the qBittorrent E2E runner, which uses `compose.rs`, for both compose command lines), each run as a baseline and after the fix with evidence that the site ran.
- Current-tree verification: `src/console/ci/e2e/runner.rs:74` calls `tracing_stdout_init(LevelFilter::INFO)`; `grep` shows `compose.rs` used by `src/console/ci/qbittorrent_e2e/services_setup.rs`.
- Resolution reference: docs(issues): add the test loop and per-site log scenarios to the CI-tools draft
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670525417

### R1 - Health-check lifecycle reply vs failed-probe report

- PR number: 3014
- Source review ID: 6027563286
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670461981
- Concern: Copilot: only the not-ready branch returns 503; failed or panicking probes return 200 with an error report. The inventory combined them.
- Solution: Same current-tree change as F2; no separate change.
- Current-tree verification: Same inspection as F2.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670524515
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3014#discussion_r4670524515

## Processing Log

- 2026-11-23 18:58 UTC - Started audit after the Copilot review 6027563286 (18:53 UTC); the two Greptile reviews 6027255837 (18:30 UTC) and 6027369330 (18:39 UTC) had arrived before it. Five findings, all checked against the branch.
- 2026-11-23 18:58 UTC - Replied on all five threads (replies 4670523578, 4670524038, 4670524515, 4670524934, 4670525417), before the two fix commits were pushed; they were pushed together with this record.

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
