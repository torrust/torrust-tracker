---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 2392
github-issue: 2396
spec-path: docs/issues/open/2396-2392-e2e-unique-tracker-image-tag/ISSUE.md
branch: "2392-test-isolation-spec"
related-pr: 2397
last-updated-utc: "2026-09-30 14:58"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - src/console/ci/e2e/runner.rs
    - docs/self-hosted-runner.md
---

<!-- skill-link: create-issue -->

# Issue #2396 - Use a Unique Tracker Image Tag per CI Run

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

Each CI run tests the tracker image it built. Runs sharing one Docker host cannot overwrite each
other's image tag, and the per-run images do not accumulate on persistent runners.

## Background

Two workflows build the tracker image, load it into the local Docker daemon under a fixed tag, and
then run container tests against that tag:

| Workflow | Tag | Consumers |
| --- | --- | --- |
| `container.yaml` (`test` job) | `torrust-tracker:local` | Persistence regression script, `e2e_tests_runner`, three `qbittorrent_e2e_runner` steps |
| `testing.yaml` (`docker-e2e` job) | `torrust-tracker:e2e-local` | `e2e_tests_runner`, three `qbittorrent_e2e_runner` steps |

What can go wrong, step by step:

1. Run A builds its image and tags it `torrust-tracker:local`.
2. Run B, on the same Docker host, builds a different commit and moves the same tag to its image.
3. Run A's later test steps start containers from `torrust-tracker:local` and test run B's image.
4. Run A reports a result for code it did not test, and may pass.

This is worse than a flaky failure: the result is wrong and nothing signals it. It cannot happen on
GitHub-hosted runners, where each job has its own virtual machine, and it does not happen today on
the self-hosted runner because each server runs one runner instance (#2386). It becomes real as soon
as a server runs two instances, or when a developer runs two local E2E runs that build with the
runner's default tag. #2386 lists this shared tag as a concurrency limit.

The security scan workflow also uses `torrust-tracker:local`, but it runs only on `ubuntu-latest`,
so it is out of scope.

## Scope

### In Scope

- Tag the image in `container.yaml` and `testing.yaml` with a per-run value (for example derived
  from `github.run_id`, `github.run_attempt`, and the matrix target), and pass that tag to every
  consumer step.
- Remove the per-run image at the end of the job, including when earlier steps fail, so images do
  not accumulate on persistent self-hosted runners.
- Decide whether `e2e_tests_runner`'s default `torrust-tracker:local` tag (used when it builds the
  image itself) also needs a per-run value for local runs, and record the decision.

### Out of Scope

- The security scan workflow (runs on isolated GitHub-hosted runners).
- Fixed E2E host ports (sibling subissue #2395).
- The BuildKit cache and the persistent Cargo target directory.
- Published image tags on Docker Hub.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`.
- ADRs to create: none known.

## Design and Ownership Review

Not applicable to runtime code. Ownership invariant for the workflow: the job that creates the
per-run tag removes it in an always-run cleanup step.

## Bug-Fix Process

Not applicable. The wrong outcome is latent: it needs two runs on one Docker host, which the current
one-instance-per-server setup prevents. If the maintainer prefers to treat it as a bug, reproduce it
first by building two different images under the same tag on one Docker host and running the E2E
runner against the first.

## Regression Test Strategy

Not applicable (not treated as a bug). Verification is manual (M1-M3) plus the normal CI runs.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Demonstrate the shared-tag hazard | On one Docker host, build two images under one tag and show the second replaces the first (M1). |
| T2 | TODO | Per-run tag in `container.yaml` | One tag value defined once per job and used by the build and every consumer step. |
| T3 | TODO | Per-run tag in `testing.yaml` | Same as T2 for the `docker-e2e` job. |
| T4 | TODO | Always-run image cleanup | Each job removes its per-run image even after failed steps; verify on the self-hosted runner (M3). |
| T5 | TODO | Local runner default and docs | Record the decision on the runner's default tag; remove the shared tag from the concurrency limits in `docs/self-hosted-runner.md`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2, T4 | `container.yaml` per-run tag and cleanup | Commit after a passing CI run. |
| T3, T4 | `testing.yaml` per-run tag and cleanup | Commit after a passing CI run. |
| T5 | Runner default decision and documentation | Separate commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/e2e-unique-tracker-image-tag/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked to the EPIC, and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, workflow lint, CI runs)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-30 14:40 UTC - GitHub Copilot - Drafted as a subissue of the test isolation EPIC from
  the concurrency limit recorded in #2386 and a review of `container.yaml` and `testing.yaml`.
- 2026-09-30 14:58 UTC - GitHub Copilot - Maintainer approved the draft. Created #2396 as a
  sub-issue of EPIC #2392 and moved the specification to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: `container.yaml` and `testing.yaml` build and test a per-run image tag; no consumer step
  uses a fixed tag.
- [ ] AC2: Each job removes its per-run image, including after a failed step.
- [ ] AC3: The decision on `e2e_tests_runner`'s default tag is recorded and implemented.
- [ ] AC4: `docs/self-hosted-runner.md` no longer lists the shared image tag as a concurrency limit.
- [ ] `linter all` exits with code `0`
- [ ] CI `Container` and `Testing` workflows pass on GitHub-hosted and self-hosted runners
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`

## Verification Plan

### Automatic Checks

- `linter all` (includes YAML linting)
- CI `Container` and `Testing` workflow runs on the pull request

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Shared-tag hazard | Build two different images with `docker build -t torrust-tracker:local`, then inspect the tag's image ID. | The tag points at the second image. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Per-run tag in CI | Inspect the CI logs of a pull-request run. | Build and every consumer step use the same per-run tag. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Cleanup on the self-hosted runner | After a passing run and a deliberately failed run, list images on the runner host. | No per-run tracker images remain. | TODO | `manual-verification-evidence.md` section V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | Workflow diff, M2 |
| AC2 | TODO | M3 |
| AC3 | TODO | Decision record |
| AC4 | TODO | Documentation diff |

## Risks and Trade-offs

- **Disk growth on persistent runners.** Per-run images must be removed; M3 checks it on the
  self-hosted runner.
- **Build cache reuse.** A new tag per run must not reduce BuildKit layer cache hits; compare the
  build step duration before and after.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add a progress-log
  entry explaining why no material discovery occurred.

## References

- Parent EPIC: #2392, `docs/issues/open/2392-test-isolation/EPIC.md`
- Concurrency limits: #2386, `docs/self-hosted-runner.md`
