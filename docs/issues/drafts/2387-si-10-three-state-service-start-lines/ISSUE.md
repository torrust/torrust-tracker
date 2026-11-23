---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-10-three-state-service-start-lines/ISSUE.md
branch: "{issue-number}-2387-si-10-three-state-service-start-lines-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:53"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - packages/udp-server/src/server/states.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/axum-http-server/src/server.rs
    - src/bootstrap/jobs/udp_tracker.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Log three state lines when a service starts

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-10 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D4, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-6, SI-7.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

Each service logs `Starting`, `Bound … socket` and `Started`, each at the moment its state is reached and with its facts as fields. A crash during the start leaves the last reached state in the log.

## Background

Today a UDP or HTTP tracker logs `Starting … instance` before the bind, then `Starting on: <url>` and `Started on: <url>` one after the other after the bind, then `Started UDP tracker` before the service is registered in the `Registar`. `Starting on:` is false when printed, the URL is in message text, and `Started` comes before the health check can see the service. This issue adds the three new lines and moves `Started` after the registration. It does not delete `Starting on:` and `Started on:`, because the E2E runner and the test fixture read them; SI-11 moves those readers and deletes the lines.

## Scope

### In Scope

- For the UDP tracker, the HTTP tracker, the REST API and the health check API: `Starting <service>` before the bind, with `service_role`, `instance_index`, `bind_address` and the service's settings; `Bound <service> socket` after the bind, with `service_role`, `instance_index` and `service_binding`; `Started <service>` after the registration succeeds, with `service_role`, `instance_index`, `service_binding`, and `public_url` when it is configured.
- `service_role` and `instance_index` as event fields on these three lines.
- Remove the old `Started <service>` line that is logged before the registration (its replacement is the new one).
- A test for each service that the three lines are logged in order and that `Started` comes after the registration.
- The failure paths: a bind error leaves `Starting` as the last line of that service, followed by the error.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- Deleting `Starting on:` and `Started on:` and moving their readers (SI-11).
- Span fields (SI-12).
- Duration field names on the `Starting` line beyond what exists today (SI-9).

## Architectural Decisions

- Related decisions: D4 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: none. If the REST API or the health check API cannot log `Started` after registration without a structural change, stop and ask.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. If a decision of the EPIC does not hold in the code, stop, record the finding and ask the maintainer.

## Design and Ownership Review

Network readiness is involved. Before implementation the specification states, for each service: which function owns the bind, which owns the registration, what is logged when the registration fails after a successful bind, and that no new await without a deadline is added between `Bound` and `Started`.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Confirm the sequence of each service | From the inventory of SI-2: bind, spawn, registration, in code order |
| T2 | TODO | UDP tracker | Three lines and a test |
| T3 | TODO | HTTP tracker | Three lines and a test |
| T4 | TODO | REST API | Three lines and a test |
| T5 | TODO | Health check API | Three lines and a test |
| T6 | TODO | Failure path | A test or a manual scenario with a port already in use |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Confirm the sequence of each service | Commit after focused validation and review. |
| T2 | UDP tracker | Commit after focused validation and review. |
| T3 | HTTP tracker | Commit after focused validation and review. |
| T4 | REST API | Commit after focused validation and review. |
| T5 | Health check API | Commit after focused validation and review. |
| T6 | Failure path | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-10-three-state-service-start-lines/ISSUE.md`
- [ ] Draft completed by the sub-issue session and approved by the maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the agent.

- 2026-11-23 16:16 UTC - Claude Code (Fable 5.2) - Drafted with EPIC #2387 from the maintainer's decisions of 2026-11-21 and 2026-11-23. Not approved as a final specification; no GitHub issue created.
- 2026-11-23 16:53 UTC - Claude Code (Fable 5.2) - Renumbered the sub-issues by working order after review round 1 of PR 3010 (finding F4); this draft's number and its references to other sub-issues follow the new numbers.

## Acceptance Criteria

- [ ] AC1: Each of the four services logs the three lines in order, with the fields above.
- [ ] AC2: `Started` is logged only after the service is in the `Registar`.
- [ ] AC3: With a bind address already in use, the log shows `Starting` for that service and then the error, with no `Bound` and no `Started`.
- [ ] AC4: `Starting on:` and `Started on:` are still logged and every existing reader still passes.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Relevant tests for changed components, including tests that assert log output
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Read a full start | Start the release tracker with the development configuration | Three lines for each of the six services, in order | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Port `0` | Set one UDP tracker's `bind_address` to port `0` and start | `Starting` shows port `0`; `Bound` and `Started` show the assigned port | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Bind failure | Occupy an HTTP tracker's port with another process and start | `Starting`, then the error; exit code `1` | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass. Record the actual commands, output and relevant tracker logs in `manual-verification-evidence.md`, created from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- Every recorded command result names the toolchain or runtime that produced it when one can affect behavior.
- The capture steps are the ones of the baseline: start the release binary, poll the health check, wait one second, send `SIGTERM`, strip ANSI codes.

### Disposable Verification Scripts

None planned. If one is needed, record its path, purpose and removal owner here before creating it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- `develop` has both old and new start lines until SI-11. Mitigation: SI-11 follows directly.
- Moving `Started` changes when a test sees readiness. Mitigation: readers still use `Started on:` until SI-11.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D4)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
