---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-5-service-binding-without-trailing-slash/ISSUE.md
branch: "{issue-number}-2387-si-5-service-binding-without-trailing-slash-spec"
related-pr: null
last-updated-utc: "2026-11-23 16:16"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - packages/axum-health-check-api-server/src/resources.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Print `service_binding` without a trailing slash in every log line

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-5 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries), creates the GitHub issue and opens the specification pull request before any code.

- Implements: D3b, D3c, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-1.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

A service binding has one text form in every log line: `<protocol>://<socket address>`, with no trailing slash for any protocol.

## Background

`ServiceBinding` (crate `torrust-net-primitives` 0.1.0) prints itself through `Url`, so an HTTP binding is `http://0.0.0.0:7070/` and a UDP binding is `udp://0.0.0.0:6969`. The same field has two forms. The lasting fix is in that crate; this issue makes the logs right now with a local helper and removes the helper when the fixed crate is used.

## Scope

### In Scope

- A small display helper in the workspace that prints a `ServiceBinding` as `<protocol>://<socket address>`, with rustdoc that says why it exists and when it is deleted, and unit tests for UDP, HTTP, HTTPS, IPv4 and IPv6.
- Use the helper at every log site that prints a service binding, request-handling lines included (about 25 sites; the specification pull request lists them).
- The issue on `torrust/torrust-net-primitives` for `Display`. It is filed with the EPIC specification; this issue links it.
- A recorded follow-up: when the fixed crate is used here, delete the helper and print the type directly.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- The health check API's JSON, which keeps the slash until the crate changes.
- Which lines carry `service_binding` (SI-6, SI-7).
- The crate fix itself.

## Architectural Decisions

- Related decisions: D3b, D3c of the EPIC. After SI-1 is merged, the logging ADR it creates governs this work.
- ADRs to create: none. The specification pull request decides in which package the helper lives; it must be reachable from every package that logs a binding without adding a dependency cycle.

During implementation, stop and create an ADR when a decision affects project architecture or design patterns, selects an approach among meaningful alternatives, or has consequences future contributors need to understand. If a decision of the EPIC does not hold in the code, stop, record the finding and ask the maintainer.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | List the log sites | Every `service_binding` log site with its package |
| T2 | TODO | Add the helper with tests | Helper, rustdoc and unit tests |
| T3 | TODO | Use it in the UDP packages | One commit |
| T4 | TODO | Use it in the HTTP packages, the REST API and the health check API | One commit for each package |
| T5 | TODO | Recapture | No `service_binding` value ends with a slash |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | List the log sites | Commit after focused validation and review. |
| T2 | Add the helper with tests | Commit after focused validation and review. |
| T3 | Use it in the UDP packages | Commit after focused validation and review. |
| T4 | Use it in the HTTP packages, the REST API and the health check API | Commit after focused validation and review. |
| T5 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-5-service-binding-without-trailing-slash/ISSUE.md`
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

## Acceptance Criteria

- [ ] AC1: No `service_binding` value in a capture at `info` or `debug` ends with `/`, in lifecycle and in request lines.
- [ ] AC2: The helper has unit tests for each protocol and for IPv6.
- [ ] AC3: The health check API JSON is unchanged.
- [ ] AC4: The upstream issue is linked from this specification and from the helper's rustdoc.
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
| M1 | Read a full run | Start the release tracker, send one UDP and one HTTP announce, stop it; search the log for `service_binding=` | One form for all values, no trailing slash | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Health check JSON | `curl http://127.0.0.1:1313/health_check` | Same JSON as before the change | TODO | `manual-verification-evidence.md` section V2 |

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

- A log site is missed. Mitigation: T5 searches the capture, and SI-21 checks it for good.
- The helper outlives its reason. Mitigation: its rustdoc and the EPIC's table of external follow-ups name the condition to delete it.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D3b, D3c)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
