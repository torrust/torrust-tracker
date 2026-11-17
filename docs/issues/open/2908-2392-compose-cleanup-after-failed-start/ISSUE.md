---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: 2392
github-issue: 2908
spec-path: docs/issues/open/2908-2392-compose-cleanup-after-failed-start/ISSUE.md
branch: "2392-audit-follow-up-issues-spec"
related-pr: null
last-updated-utc: "2026-11-17 12:35"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/closed/2394-2392-audit-shared-test-resources/inventory.md
    - src/console/ci/compose.rs
    - src/console/ci/qbittorrent_e2e/services_setup.rs
---

<!-- skill-link: create-issue -->

# Issue #2908 - Tear Down the qBittorrent E2E Stack When It Fails to Start

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

When `docker compose up` fails part-way, the qBittorrent E2E runner still removes every container,
network and volume of its Compose project (EPIC rule 1: release resources on success, failure and
drop paths).

## Background

Found in the review of the shared-resource audit (#2394, PR #2891, finding C9 in its
`inventory.md`). `DockerCompose::up` (`src/console/ci/compose.rs:73-93`) runs
`docker compose up --wait --detach` and returns the `RunningCompose` guard, whose drop runs
`docker compose down --volumes`, only when the command succeeds. On failure it returns an error and
no guard, and `services_setup.rs:47` propagates the error:

1. The runner starts the stack under its random project name (`qbt-e2e-<10 characters>`).
2. Some services start, but `up --wait` fails, for example because the tracker never becomes
   healthy or a database image cannot start.
3. The runner exits with the error. The containers, the project network and the volumes that were
   created stay on the Docker daemon, still running.

The project name and the host ports (`0`) are unique per run, so a leaked stack never collides with
another run. It accumulates instead: on the self-hosted runner, each failed run leaves running
containers (tracker, qBittorrent seeder and leecher, and MySQL or PostgreSQL) that hold memory and
disk until someone removes them. `contrib/dev-tools/self-hosted-runner/docker-ci-prune.sh` prunes
stopped resources, not running containers.

The plain E2E runner does not have this problem: `TrackerContainer`'s drop removes its container by
name, also after a failed `docker run` (`src/console/ci/e2e/tracker_container.rs:17-26`).

## Scope

### In Scope

- Run `docker compose down --volumes` for the project when `up` fails, or create the guard before
  `up` so its drop covers the failure path.
- Keep `--keep-containers` behavior: a kept stack is not torn down on purpose.

### Out of Scope

- The plain E2E runner (already cleans up).
- Pruning resources leaked by earlier runs (a one-off cleanup on the runner).
- Compose project names and host ports (already per run; #2394 C5, P10).

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none expected.

## Design and Ownership Review

- Ownership: the guard that owns the project's teardown exists from the moment `up` may have
  created anything, so normal, failure and drop paths all reach `down --volumes`.
- `--keep-containers` disarms the guard only after a successful start; decide and record whether a
  failed start with `--keep-containers` keeps the stack for debugging.
- Deadline: `down` must not wait without bound; record the limit it runs under.
- Design-review checkpoint: after the first passing vertical slice (the guard covers a failed
  `up`), review the design before any other change.

## Test Development Loop

The fix adds a test, so it follows the progressive test-development loop of the `write-unit-test`
skill (`.claude/skills/write-unit-test/SKILL.md`): work in the smallest passing increment, review
its design with the mandatory prose-first Arrange-Act-Assert comparison before the next increment,
record the review in `manual-verification-evidence.md`, and commit the reviewed increment.

## Bug-Fix Process

Follow `.claude/skills/fix-bug/SKILL.md`:

1. Real-artifact reproduction: start the runner with a stack that cannot become healthy (for
   example a tracker image that exits at once), then list the project's containers with
   `docker ps -a --filter label=com.docker.compose.project=<project>`; record it in
   `manual-verification-evidence.md` (V1).
2. Regression boundary: decide in spec review between a unit test with a fake `docker` command at
   the `DockerCompose` seam and a Docker-dependent test; record why.
3. Red before the change, green after; final like-for-like recheck of step 1 (V2).

## Regression Test Strategy

The smallest deterministic boundary is the `DockerCompose` cleanup decision: given a failed `up`,
`down --volumes` runs for the same project. Prefer a test at that seam that does not need Docker.
The real-artifact recheck (V2) confirms that nothing of the project remains.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce the leak | Leaked containers listed (V1). |
| T2 | TODO | Decide the test boundary and guard design | Recorded in this spec, approved by the maintainer. |
| T3 | TODO | Tear down after a failed `up` | Regression test red then green; design review recorded. |
| T4 | TODO | Recheck | No container, network or volume of the project remains (V2); `linter all` passes. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Evidence and design | Documentation-only commit after maintainer review. |
| T3 | Cleanup change and its test | Commit after focused validation and the test-design review. |
| T4 | Recheck evidence | Documentation-only commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2392-compose-cleanup-after-failed-start/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-16 22:06 UTC - Claude Code (Claude Opus 5.5), #2394 session - Drafted from Copilot's
  review of PR #2891 after the maintainer classified the failed-`up` cleanup as a new problem.
  Confirmed in the code; not reproduced against Docker yet (T1).
- 2026-11-17 12:35 UTC - Claude Code (Claude Opus 5.5), EPIC #2392 coordinator - The
  maintainer approved the draft in the review of PR #2891. Created GitHub issue #2908 and
  moved the spec from `docs/issues/drafts/` to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: After a failed `docker compose up`, no container, network or volume of the run's project
  remains.
- [ ] AC2: A regression test fails without the fix and passes with it.
- [ ] AC3: `--keep-containers` behaves as recorded in the design.
- [ ] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`
- The regression test

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| V1 | Reproduce | Run `qbittorrent_e2e_runner` with a stack that cannot become healthy; list the project's containers | Containers remain | TODO | `manual-verification-evidence.md` |
| V2 | Recheck | Same steps after the fix | Nothing remains | TODO | `manual-verification-evidence.md` |

Evidence policy: every recorded command result names the toolchain or runtime that produced it
(for example `cargo +stable` and the Docker version), and the evidence file is created from
`docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | V2 |
| AC2 | TODO | Regression test |
| AC3 | TODO | Diff review |

## Risks and Trade-offs

- **Teardown hides the failure.** Removing the stack also removes the container logs a developer
  needs to debug the failed start; capture the logs before `down`, or honor `--keep-containers`.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If the work invalidates an assumption of this spec, changes its design materially, or finds
  something reusable, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this folder; otherwise add a progress-log
  entry that says why none was needed.
- An independent reviewer that receives this folder-style specification records its result in
  `agent-review-reports.md` (`docs/templates/AGENT-REVIEW-REPORTS.md`).

## References

- Parent EPIC: #2392
- Audit: #2394, `docs/issues/closed/2394-2392-audit-shared-test-resources/inventory.md` (C9)
- Found in: PR #2891 (Copilot review)
