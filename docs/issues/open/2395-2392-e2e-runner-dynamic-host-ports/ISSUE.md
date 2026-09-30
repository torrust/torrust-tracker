---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 2392
github-issue: 2395
spec-path: docs/issues/open/2395-2392-e2e-runner-dynamic-host-ports/ISSUE.md
branch: "2392-test-isolation-spec"
related-pr: null
last-updated-utc: "2026-09-30 14:58"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
    - src/console/ci/e2e/runner.rs
    - src/console/ci/e2e/tracker_container.rs
    - src/console/ci/e2e/logs_parser.rs
    - src/console/ci/e2e/docker.rs
    - src/console/ci/compose.rs
    - packages/e2e-tools/src/bin/e2e_tests_runner.rs
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
---

<!-- skill-link: create-issue -->

# Issue #2395 - Use Dynamic Host Ports in the E2E Runner

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

Two `e2e_tests_runner` executions can run at the same time on one Docker host without colliding
on host ports. The runner lets Docker choose free host ports and discovers them, instead of
publishing fixed host ports. (Testing the right image in each run is sibling subissue #2396.)

## Background

`src/console/ci/e2e/runner.rs` starts the tracker container with fixed host port mappings:
`6969:6969/udp`, `7070:7070/tcp`, `1212:1212/tcp`, and `1313:1313/tcp`. It then parses the
tracker's startup log (`logs_parser.rs`) to find the running services and passes those addresses,
rewritten to `127.0.0.1`, to the tracker checker. This works only because host and container ports
are identical.

What goes wrong, step by step:

1. Run A starts a tracker container and binds host ports `6969`, `7070`, `1212`, and `1313`.
2. Run B starts while A is still running and asks Docker for the same host ports.
3. Docker cannot publish them, so run B fails before testing anything.

The runner already notes the limitation in a `code-review` comment: with port `0` it would not
know which ports to open. #2386 records the consequence: each self-hosted runner server can host
only one runner instance.

Container names are already unique (`tracker_` plus a random suffix), and the qBittorrent E2E
runner already solves this problem: its Compose stacks publish host port `0` and read the assigned
port with `docker compose port` (`src/console/ci/compose.rs`).

## Scope

### In Scope

- Publish the tracker container's service ports without fixed host ports and discover the assigned
  host port for each service with `docker port`.
- Translate the container-side addresses parsed from the startup log into host-side addresses
  before running the tracker checker.
- Keep the container-internal ports fixed; they live in the container's own network namespace and
  do not conflict between runs.
- Unit tests for the container-to-host address translation.

### Out of Scope

- The shared tracker image tag (sibling subissue #2396).
- The shared Cargo target directory between runner instances (speed concern, EPIC #1840).
- The qBittorrent E2E runner, which already uses dynamic host ports.
- Changing the tracker configuration used by E2E tests or adding new E2E scenarios.
- Changing runner instances on the self-hosted runner servers.

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: none known. Reusing the qBittorrent E2E approach needs no new decision.

## Design and Ownership Review

| Concern | Owner | Required invariant |
| --- | --- | --- |
| Port publishing | `runner.rs` / `RunOptions` | Publishes each service port with a Docker-assigned host port. |
| Port discovery | `TrackerContainer` / `Docker` | Reads the assigned host port after the container is healthy, with the existing health wait as the deadline. |
| Address translation | Small pure function | Maps each parsed container address to its host address; fails clearly if a service port was not published. |
| Cleanup | `TrackerContainer` (`Drop`) | Existing stop/remove behavior is unchanged. |

Review the design after the first service type (for example UDP) works end to end.

## Bug-Fix Process

Not applicable. The runner works as designed for one run at a time; this issue removes a
concurrency limitation.

## Regression Test Strategy

Not applicable (not a bug). The translation logic gets unit tests; concurrent execution is verified
manually (M2).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce the conflict | Record two concurrent runs with the current runner; the second fails on host port binding (M1). |
| T2 | TODO | Add host-port discovery | `docker port` wrapper for a container and a `port/protocol`; unit-test its output parsing. |
| T3 | TODO | Translate service addresses | Pure function mapping parsed container addresses to host addresses, with unit tests. |
| T4 | TODO | Publish dynamic host ports | Runner publishes container ports only, discovers host ports, and runs the checker on host addresses; remove the `code-review` note. |
| T5 | TODO | Verify concurrency and update docs | Two concurrent runs pass (M2); CI E2E jobs pass; remove fixed host ports from #2386's documented limits in `docs/self-hosted-runner.md`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Host-port discovery and its tests | Commit after focused validation and design review. |
| T3 | Address translation and its tests | Commit after focused validation and design review. |
| T4 | Runner switch to dynamic host ports | Commit after a passing single E2E run. |
| T5 | Evidence and documentation | Documentation-only commit. |

For test-producing work, use the `write-unit-test` skill and complete the prose-first
Arrange-Act-Assert design review after each passing increment, before maintainer review and commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/e2e-runner-dynamic-host-ports/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked to the EPIC, and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-30 13:38 UTC - GitHub Copilot - Drafted as a subissue of the test isolation
  EPIC. Source review confirmed the fixed host ports in `runner.rs` and that container names are
  already random.
- 2026-09-30 14:49 UTC - GitHub Copilot - Consistency review: issue type set to `task` to match
  the EPIC table; goal scoped to host ports, leaving image identity to the sibling subissue.
- 2026-09-30 14:58 UTC - GitHub Copilot - Maintainer approved the draft. Created #2395 as a
  sub-issue of EPIC #2392 and moved the specification to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: The E2E runner publishes no fixed host ports.
- [ ] AC2: The tracker checker receives host addresses that match the Docker-assigned ports.
- [ ] AC3: Two runner executions started at the same time on one Docker host both pass.
- [ ] AC4: CI E2E jobs in `testing.yaml` and `container.yaml` pass unchanged.
- [ ] AC5: `docs/self-hosted-runner.md` no longer lists fixed E2E host ports as a concurrency limit.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test` for the E2E runner unit tests
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Current conflict | Start two `cargo run -p torrust-tracker-e2e-tools --bin e2e_tests_runner -- --config-toml-path <config>` runs at once. | The second run fails to publish a host port. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Concurrent runs after the change | Repeat M1 with the new runner. | Both runs pass and use different host ports. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Single run after the change | Run the runner once as CI does. | The run passes; logs show the discovered host ports. | TODO | `manual-verification-evidence.md` section V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | `runner.rs` diff |
| AC2 | TODO | T3 unit tests, M3 |
| AC3 | TODO | M2 |
| AC4 | TODO | CI run links |
| AC5 | TODO | Documentation diff |

## Risks and Trade-offs

- **Checker output shows unfamiliar ports.** Logs will show assigned host ports instead of `6969`
  and `7070`; log both container and host ports to keep runs easy to read.
- **UDP port discovery.** `docker port` must be queried with the `/udp` protocol suffix; cover it in
  the T2 unit tests.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add a progress-log
  entry explaining why no material discovery occurred.

## References

- Parent EPIC: #2392, `docs/issues/open/2392-test-isolation/EPIC.md`
- Concurrency limits: #2386
- Dynamic host-port precedent: `src/console/ci/compose.rs` (qBittorrent E2E)
