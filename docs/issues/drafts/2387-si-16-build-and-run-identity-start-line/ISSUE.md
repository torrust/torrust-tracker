---
schema-version: 1
doc-type: issue
issue-type: feature
status: draft
priority: p2
epic: 2387
github-issue: null
spec-path: docs/issues/drafts/2387-si-16-build-and-run-identity-start-line/ISSUE.md
branch: "{issue-number}-2387-si-16-build-and-run-identity-start-line-spec"
related-pr: null
last-updated-utc: "2026-11-24 12:46"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
    - docs/issues/open/2387-review-tracker-lifecycle-logs/decisions.md
    - .claude/skills/create-issue/SKILL.md
    - src/main.rs
    - src/app.rs
    - Containerfile
    - .claude/skills/add-rust-dependency/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Log the build and run identity as the first line

**Parent EPIC:** #2387 - Make Tracker Startup and Shutdown Logs Coherent for Operators. Sub-issue SI-16 of 23.

**Draft.** This draft fixes the goal, the scope and the decisions it implements. The session that takes this sub-issue completes it (file lists, exact messages, test boundaries) and stops for the maintainer to review and approve it. Only after that approval it creates the GitHub issue and opens the specification pull request, before any code.

- Implements: D10, in [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md).
- Depends on: SI-5.
- Working order and rules common to all sub-issues: [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md), section Delivery Strategy.
- Evidence of today's behavior: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md) and the two captures beside it.

## Goal

The first line of a run says which build is running and identifies the run: `APPLICATION: Starting Torrust Tracker version=… git_commit=… build_profile=… pid=… run_id=…`.

## Background

The log does not say which version or commit is running, so a log file cannot be matched to a build. When logs of several runs are in one place (a restart loop, two containers), nothing but the time tells them apart. A **run** is one execution of the tracker process; the word instance is kept for a configured service. Decision D10 compares five alternatives for the run identity and chooses `pid` and a random `run_id` on the start line now.

## Scope

### In Scope

- A build script that embeds the Git commit (and whether the tree was dirty) at build time, with a defined value when Git is not available (a source archive, a container build without `.git`).
- The start line with `version` (the crate version), `git_commit`, `build_profile` (`debug` or `release`), `pid` and `run_id` (a random UUID, generated once at start, in the same form as `request_id`).
- The line is the first one logged, after logging is initialized. The specification pull request decides whether `Logging initialized` stays or is replaced by it.
- `run_id` is kept where SI-18 and SI-22 can read it.
- Check the container build (`Containerfile`) so that the commit reaches the build.
- Update the catalogue of lifecycle lines and the section "Changes in 4.0.0" in `docs/logging.md` for every line this issue changes.

### Out of Scope

- `run_id` on every record (SI-22).
- Exposing the build identity in the REST API or the health check.
- Reproducible-build concerns beyond recording what the specification decides for a dirty tree.

## Findings of the coordinator's check (2026-11-24)

The EPIC coordinator compared this draft with the code at `develop` `6a058dd4d` on 2026-11-24. Nothing changed since the draft was written; these are cases the draft did not name. The session that completes the draft checks each one and decides how its specification uses it.

- No `build.rs` exists in the workspace, and nothing logs a version or commit today. `request_id` is a UUID (`MakeRequestUuid`).
- `uuid` is not a dependency of the root crate (only of four packages), so `run_id` adds a direct dependency; it is already in the lockfile.
- Container build: `.dockerignore` is default-deny and admits neither `.git` nor a root `build.rs`; `Containerfile` has no commit build argument; `cargo chef` needs the declared files; `.github/workflows/container.yaml` must pass the value.
- `Logging initialized` is logged inside `packages/configuration/src/v3_0_0/logging.rs:101`, under a `Once`; with `trace_filter = "off"` the setup returns before it (`:68`). A configuration failure happens before logging starts (`src/bootstrap/app.rs:56-62`), so such a run has no identity line.
- Log the line from the library, not from `src/main.rs`, so the test-only executable `packages/hung-component-tracker` (its own `src/main.rs:36-43`) prints it too (maintainer, 2026-11-24).

## Architectural Decisions

- Related decisions: D10 of the EPIC. After SI-2 is merged, the logging ADR it creates governs this work.
- ADRs to create: none expected. Adding a build script and possibly a build dependency follows the `add-rust-dependency` skill; prefer no new dependency.

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
| T1 | TODO | Decide how the commit is embedded | Alternatives (build script with `git`, an environment variable set by CI and the container build, a crate) with pros, cons and the choice, in the specification |
| T2 | TODO | Build script and fallback | Builds with and without `.git` |
| T3 | TODO | Start line | With a test of its fields |
| T4 | TODO | Container build | The image logs the right commit |
| T5 | TODO | Recapture | The first line of a native run and of a container run |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Decide how the commit is embedded | Commit after focused validation and review. |
| T2 | Build script and fallback | Commit after focused validation and review. |
| T3 | Start line | Commit after focused validation and review. |
| T4 | Container build | Commit after focused validation and review. |
| T5 | Recapture | Commit after focused validation and review. |

For test-producing work, use the `write-unit-test` skill and complete the design review it requires after each passing test increment. Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2387-si-16-build-and-run-identity-start-line/ISSUE.md`
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

- [ ] AC1: The first line of a run has the five fields.
- [ ] AC2: `git_commit` equals the commit the binary was built from, in a native build and in the container image.
- [ ] AC3: A build without Git succeeds and logs the defined fallback value.
- [ ] AC4: Two runs have different `run_id` values.
- [ ] AC5: The line is in the catalogue of `docs/logging.md`.
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
| M1 | Native run | Build and start the release tracker; compare `git_commit` with `git rev-parse HEAD` | Equal | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Container run | Build the image as documented and start it | The first line shows the same commit | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Two runs | Start, stop and start again | Different `run_id`; `pid` as the operating system reports it | TODO | `manual-verification-evidence.md` section V3 |

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
| AC5 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- A build script slows every build or breaks builds outside Git. Mitigation: T1 compares the alternatives; the fallback is tested.
- The commit of a dirty tree is misleading. Mitigation: the dirty state is part of the value.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- Send a short final report to the EPIC coordinator: the pull request links and anything that affects another sub-issue.

## References

- Parent EPIC: #2387, [EPIC.md](../../open/2387-review-tracker-lifecycle-logs/EPIC.md)
- Decisions: [decisions.md](../../open/2387-review-tracker-lifecycle-logs/decisions.md) (D10)
- Baseline: [baseline-lifecycle-logs.md](../../open/2387-review-tracker-lifecycle-logs/baseline-lifecycle-logs.md)
