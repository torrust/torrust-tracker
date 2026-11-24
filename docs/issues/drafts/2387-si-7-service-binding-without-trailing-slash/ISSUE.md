---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-7-service-binding-without-trailing-slash/ISSUE.md
branch: "{issue-number}-2387-si-7-service-binding-without-trailing-slash-spec"
related-pr: null
last-updated-utc: "2026-11-24 12:46"
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

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-7 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries) and stops for the maintainer to review and approve it. Only after that approval it creates the GitHub issue and opens the specification pull request, before any code.

- Implements: D3b, D3c, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-2.
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
- The issue on `torrust/torrust-net-primitives` for `Display`. It is filed: <https://github.com/torrust/torrust-net-primitives/issues/1>.
- A recorded follow-up: when the fixed crate is used here, delete the helper and print the type directly.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- The health check API's JSON, which keeps the slash until the crate changes.
- Which lines carry `service_binding` (SI-8, SI-10).
- The crate fix itself.

## Findings of the coordinator's check (2026-11-24)

The EPIC coordinator compared this draft with the code at `develop` `6a058dd4d` on 2026-11-24. Nothing changed since the draft was written; these are cases the draft did not name. The session that completes the draft checks each one and decides how its specification uses it.

- There are exactly 25 event sites with a `service_binding` field: `packages/axum-rest-api-server/src/routes.rs:93,118,129,145`, `server.rs:138,140`; `packages/axum-http-server/src/v1/routes.rs:99,122,133,149`, `server.rs:237,239`; `packages/udp-server/src/handlers/error.rs:103,105,119,121`, `server/states.rs:118,120`, `server/launcher.rs:88,132,134,148,150`; `packages/axum-health-check-api-server/src/handlers.rs:65`, `server.rs:115`.
- The five UDP launcher sites record a `&str`, so the value prints quoted (`service_binding="udp://…"`). D3a wants one unquoted form.
- Spans record `ServiceBinding` with `Debug`: `packages/axum-rest-api-server/src/routes.rs:41`, `server.rs:252` (`check_fn{service_binding=ServiceBinding {…}}` on every health probe), `packages/udp-server/src/handlers/mod.rs:80`. SI-12 owns spans.
- The health check JSON fills its field with `ServiceBinding::url()` (`packages/axum-health-check-api-server/src/handlers.rs:70`), so neither this issue nor the `torrust-net-primitives` fix changes it.
- A candidate home for the helper: `torrust-tracker-primitives`, which depends on `torrust-net-primitives` and is a dependency of the four server packages; `packages/axum-server` does not depend on it.
- `torrust-net-primitives` is pinned at 0.1.0 (`Cargo.toml:130`).

## Architectural Decisions

- Related decisions: D3b, D3c of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
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

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-7-service-binding-without-trailing-slash/ISSUE.md`
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
- 2026-11-23 17:20 UTC - Claude Code (Fable 5.2) - Restored the link to the upstream issue, which the renumbering commit had dropped (review finding F10 of da2ce7 on PR 3010, audit finding F13).
- 2026-11-23 17:29 UTC - Claude Code (Fable 5.2) - Corrected after the CodeRabbit review of PR 3010 (audit findings F15 to F22): this draft now names the maintainer's approval before the GitHub issue is created; the drafts of SI-4, SI-8, SI-9, SI-13 and SI-23 have one more correction each.
- 2026-11-24 12:46 UTC - Claude Code (Opus 5.5), EPIC coordinator - Compared with the code at `develop` `6a058dd4d`: corrected the claims the code contradicts, recorded the maintainer's answers of 2026-11-24 that concern this draft, and added "Findings of the coordinator's check" (order B′ of D1 places #2766 after SI-1 and before SI-2).

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

- A log site is missed. Mitigation: T5 searches the capture, and SI-20 checks it for good.
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
