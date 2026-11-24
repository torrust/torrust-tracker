---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-8-single-listener-identity-field/ISSUE.md
branch: "{issue-number}-2387-si-8-single-listener-identity-field-spec"
related-pr: null
last-updated-utc: "2026-11-24 12:46"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - packages/udp-server/src/server/launcher.rs
    - packages/axum-server/src/signals.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Identify a listener with one field: remove `local_addr`, `address` and `bind_addr`

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-8 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries) and stops for the maintainer to review and approve it. Only after that approval it creates the GitHub issue and opens the specification pull request, before any code.

- Implements: D3a, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-7.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

Every log line about a bound listener names it with `service_binding`, together with `service_role` and `instance_index`. The fields `local_addr`, `address` and `bind_addr` no longer appear.

## Background

The listener is named five ways across components. `bind_address` (the configured value) and `service_binding` (the bound URL) are two different values and both stay, each in its place. The other three names are a socket address under different words, left from older code: the UDP receive loop still says `local_addr` (the finding that started this EPIC), and the shared HTTP shutdown span says `address`.

## Scope

### In Scope

- Replace `local_addr`, `address` and `bind_addr` with `service_binding` in every lifecycle line and lifecycle span that is after the bind.
- Rename `bind_addr` to `bind_address` where it is logged before the bind, with the configured socket address as value. One such line exists today: `UdpSocket::bind (binding)` in `packages/udp-server/src/server/bound_socket.rs`, at `DEBUG`, whose value is a URL built from the configured address.
- Where a line has the listener only in its message text, add the field and take the address out of the text.
- Keep `bind_address` only on lines before the bind.
- Add `service_role` and `instance_index` where a line has `service_binding` without them and is outside a span that carries them.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- The three start lines (SI-10).
- Span fields other than the listener identity (SI-12).
- Job manager lines (SI-13).

## Findings of the coordinator's check (2026-11-24)

The EPIC coordinator compared this draft with the code at `develop` `6a058dd4d` on 2026-11-24. Nothing changed since the draft was written; these are cases the draft did not name. The session that completes the draft checks each one and decides how its specification uses it.

- The drain lines already use `service_binding` (`packages/udp-server/src/server/launcher.rs:88-150`); only the receive-loop lines need the change.
- Every `local_addr` field: `packages/udp-server/src/server/launcher.rs:196,263,267,282,370,397,446,450,454`, `request_buffer.rs:102,115`, `bound_socket.rs:39`.
- Other names for a listener: `bind_to` (`launcher.rs:428`, a `WARN` whose value is the bound address); `local_udp_url` (`launcher.rs:193`); `server_socket_addr` in request lines (`packages/axum-http-server/src/v1/routes.rs:98,121,132`, `packages/axum-rest-api-server/src/routes.rs:92,117,128,144`, `packages/udp-server/src/handlers/error.rs:103-121`).
- A listener named only in message text: `packages/axum-server/src/signals.rs:33,41,54`, with messages built by the callers (`packages/axum-http-server/src/server.rs:292`, `packages/axum-rest-api-server/src/server.rs:230`, `packages/axum-health-check-api-server/src/server.rs:105`), and `Stopped server running on: http://{}` (`src/bootstrap/jobs/health_check_api.rs:67`).
- AC3 needs the identity in spawned tasks: the UDP receive loop (`run_udp_server_main`) and the shared HTTP shutdown (`graceful_shutdown_on_cancellation`) run in root spans without `service_role` and `instance_index`. `signals.rs` takes a `SocketAddr`, and `packages/axum-server` depends on neither primitives package.

## Architectural Decisions

- Related decisions: D3a of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: none.

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
| T1 | TODO | List the sites | From the inventory: each use of the three names and of a listener in message text |
| T2 | TODO | UDP tracker | Receive loop and drain lines; tests updated |
| T3 | TODO | Shared HTTP shutdown code and the HTTP services | `graceful_shutdown_on_cancellation` span and its lines |
| T4 | TODO | Recapture | No `local_addr=`, `address=` or `bind_addr=` in a capture at `debug` |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | List the sites | Commit after focused validation and review. |
| T2 | UDP tracker | Commit after focused validation and review. |
| T3 | Shared HTTP shutdown code and the HTTP services | Commit after focused validation and review. |
| T4 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-8-single-listener-identity-field/ISSUE.md`
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
- 2026-11-23 17:29 UTC - Claude Code (Fable 5.2) - Corrected after the CodeRabbit review of PR 3010 (audit findings F15 to F22): this draft now names the maintainer's approval before the GitHub issue is created; the drafts of SI-4, SI-8, SI-9, SI-13 and SI-23 have one more correction each.
- 2026-11-24 12:46 UTC - Claude Code (Opus 5.5), EPIC coordinator - Compared with the code at `develop` `6a058dd4d`: corrected the claims the code contradicts, recorded the maintainer's answers of 2026-11-24 that concern this draft, and added "Findings of the coordinator's check" (order B′ of D1 places #2766 after SI-1 and before SI-2).

## Acceptance Criteria

- [ ] AC1: A capture at `debug` contains none of the three removed field names.
- [ ] AC2: `bind_address` appears only on lines logged before the bind, and every pre-bind line about a listener names its address with `bind_address`. A line that is not about one listener, such as the build and run identity line, has no such field.
- [ ] AC3: Every line with `service_binding` can be attributed to a service by `service_role` and `instance_index`, as event or span fields.
- [ ] AC4: "Changes in 4.0.0" lists the removed names.
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
| M1 | Read a full run at `debug` | Start the release tracker at `debug`, send one UDP announce, stop it; search for the three names | None found | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Follow one listener | Select every line with one `service_binding` value | The start, the requests and the shutdown of that listener are selected | TODO | `manual-verification-evidence.md` section V2 |

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

- A value called `address` is not a listener (for example a client address). Mitigation: T1 classifies each site; only listener identities change.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D3a)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
