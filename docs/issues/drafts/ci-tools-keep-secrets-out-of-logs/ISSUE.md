---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p3
epic: null
github-issue: null
spec-path: docs/issues/drafts/ci-tools-keep-secrets-out-of-logs/ISSUE.md
branch: "{issue-number}-ci-tools-keep-secrets-out-of-logs"
related-pr: null
last-updated-utc: "2026-11-23 19:45"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - handle-secrets
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/handle-secrets/SKILL.md
    - docs/issues/open/2993-secrets-and-client-error-text/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Keep secrets out of the CI tools' logs

Related to EPIC #2993 (Keep secrets out of errors, logs and client replies), which leaves the CI tools out of its scope (maintainer, 2026-11-23). Not a sub-issue of it.

## Goal

The CI tools under `src/console/ci/` do not write the tracker configuration, environment values or database URLs that hold a secret (an admin token, a database password) to their log, and the log still says what the tool is doing.

## Background

The inventory of EPIC #2993 (section "Verified by running experiments") and its recheck on 2026-11-23 against `develop` `6bc417424` found these sites:

| Site | What it logs |
| --- | --- |
| `src/console/ci/e2e/runner.rs:80` | `tracing::info!("tracker config:\n{tracker_config}")`: the whole configuration TOML, with its access tokens and database password |
| `src/console/ci/e2e/docker.rs:89` | `tracing::debug!("Docker run args: {:?}", args)`: the `docker run` arguments, including each `--env KEY=value`, so the configuration passed through the environment |
| `src/console/ci/compose.rs:111`, `:317` | `tracing::info!("Running docker compose command: {:?}", command)`: the `Debug` of a `std::process::Command`, which prints the environment set with `.envs(...)`. Its only caller today passes image names, ports and paths (`src/console/ci/qbittorrent_e2e/services_setup.rs:123-160`), so the risk is latent |
| `src/console/ci/e2e/tracker_checker.rs:88` | `tracing::info!("Tracker Checker config:\n{config}")`: the checker configuration. In the E2E runner it is the JSON of the running services (addresses parsed from the `Started on:` lines, `runner.rs:117-120`), so it holds no token or password today; it is listed because the function logs whatever configuration it gets (issue #3004 noted it and left it out of its scope) |
| `src/console/ci/qbittorrent_e2e/tracker/config_builder.rs:16-17` | constants that hold database URLs with a user and a password (`mysql://db_user:db_user_secret_password@...`, `postgresql://postgres:postgres@...`) |

The values are test values of the CI environment, so the exposure is low today. The pattern is the problem: the CI logs are public on GitHub Actions, and the same code would print a real secret if someone pointed the tool at a real configuration. The `handle-secrets` skill forbids secrets in logs.

Not checked yet: whether the `config_builder.rs` constants are still used, since the v3 configuration refuses `path` for a network database (`packages/configuration/src/v3_0_0/database.rs:203-204`).

## Scope

### In Scope

- Each site above logs a redacted form or only the non-secret part (for example the configuration with `to_redacted_json` or the same masking, the `docker run` arguments with `--env` values replaced by `***`, the compose command without its environment).
- The `config_builder.rs` database URLs: removed if unused, or built from the v3 network database fields.
- A check of the other `src/console/ci/` tools for the same pattern.

### Out of Scope

- The tracker binary and its servers (EPIC #2993).
- `tracker_client` output (#3004).
- Secrets in GitHub Actions workflow files (the runner and workflow skills cover them).

## Architectural Decisions

- Related ADRs: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`.
- ADRs to create: None known.

## Design and Ownership Review

Not applicable: the work changes log statements and constants in CI tools. It adds no child process, asynchronous I/O, network readiness, resource cleanup or reusable test fixture.

## Bug-Fix Process

Follows `.claude/skills/fix-bug/SKILL.md`: re-check the sites on the starting commit; reproduce by running the E2E runner locally and recording the log lines (with test values); choose the test boundary; red test; fix; green test; repeat the run.

## Regression Test Strategy

A unit test per redaction helper (configuration text, `docker run` arguments, compose environment) that builds the input from a unique marker value and asserts the marker is not in the logged text. These tests carry the cases a real run cannot reach: a database password (the E2E runner forces SQLite), a credential in the compose environment (none is passed today) and the checker configuration (the E2E runner passes service addresses only). The manual scenarios show that each logging site runs.

## Implementation Plan

Test-development loop (required by `docs/templates/ISSUE.md`, "Commit Points", and the `create-issue` skill): the red tests are not written as one batch. Use the `write-unit-test` skill and work one behavior area at a time, in the order of the commit points below. For each area: write the smallest behavior-focused test, record its red output, make the production change, record the green output, then perform and record an explicit test-design review before starting the next area. The review uses the prose-first Arrange-Act-Assert comparison (write temporary prose for each section, refactor until the code expresses it, remove redundant prose) and confirms that the test exposes the one causal initial-state difference, that its fixture owns only incidental mechanics, and that the production Act and the independently specified expected result stay visible. Record each review in the task's evidence. Commit each reviewed increment before the next one. After the last increment, stop for maintainer review before final verification, the final commit and the pull request.

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Baseline | The sites re-checked; one E2E run's log lines recorded with test values. |
| T2 | TODO | Configuration logs | `runner.rs:80`, `tracker_checker.rs:88` log a redacted form. |
| T3 | TODO | Command logs | `docker.rs:89`, `compose.rs:111`, `:317` log no environment value. |
| T4 | TODO | Database URL constants | `config_builder.rs:16-17` removed or rebuilt. |
| T4b | TODO | Maintainer checkpoint | Stop for maintainer review after the last test increment, before final verification, the final commit and the pull request. |
| T5 | TODO | Final recheck | The manual scenarios repeated; no secret in the log. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Configuration log redaction with its tests | Commit after the red and green runs and the recorded test-design review. |
| T3 | Command log redaction with its tests | Commit after the red and green runs and the recorded test-design review. |
| T4 | Database URL constants | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/ci-tools-keep-secrets-out-of-logs/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-23 18:08 UTC - Claude Code (Claude Opus 5.5), EPIC #2993 coordinator session - Drafted from the EPIC #2993 recheck after the maintainer put the CI tools out of the EPIC's scope. Not approved; no GitHub issue created.
- 2026-11-23 19:29 UTC - Claude Code (Claude Opus 5.5), EPIC #2993 coordinator session - Review of PR 3014: scoped M1 to the runner configuration line and the admin token (the runner forces SQLite), and recorded that the checker log and the compose environment hold no credential today, so their markers are injected by unit tests.

## Acceptance Criteria

- [ ] AC1: No log line of the CI tools under `src/console/ci/` holds an access token or a database password from the configuration or the environment.
- [ ] AC2: The log still says which configuration file, container and command the tool used.
- [ ] AC3: No constant in the CI tools holds a database URL with a password, or the reason it must stay is recorded.
- [ ] AC4: Each test increment has a recorded test-design review, and the maintainer checkpoint after the last increment is recorded in the progress log.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- The unit tests of the redaction helpers
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | E2E runner configuration log | Run the E2E runner (`src/console/ci/e2e/runner.rs`) locally with an input configuration whose `[http_api.access_tokens]` admin token is a unique marker, and read the INFO line `tracker config:` (`runner.rs:80`). The line is written before the container starts. The runner then forces the SQLite driver (`runner.rs:95-96`), so a network-database input with a password marker cannot complete the run; the password case is covered by the T2 unit test, not by this scenario | The configuration line appears, redacted; no marker | TODO | `manual-verification-evidence.md` section V1 |
| M2 | `docker run` arguments | The runner fixes tracing at INFO (`src/console/ci/e2e/runner.rs:74`), so the DEBUG event of `docker.rs:89` does not appear in M1. Run it with the level raised to DEBUG (a temporary local change, recorded and not committed, unless T3 adds a level option) and the same markers in the `--env` values | The `Docker run args` line appears; no marker | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Compose commands | The compose environment of the qBittorrent E2E runner holds only image names, ports and paths (`src/console/ci/qbittorrent_e2e/services_setup.rs:123-160`), so there is no credential to show today. Run the qBittorrent E2E runner and read both `Running docker compose command` lines (`compose.rs:111`, `:317`); the T3 unit test injects a marker through `DockerCompose::with_env` | Both lines appear and print no environment value | TODO | `manual-verification-evidence.md` section V3 |

Each scenario is run on the starting commit first and again after the last production change, and the evidence shows that each logging site ran. The baseline of M1 and M2 shows the marker. M3 has no marker (no credential is passed to compose today), so its baseline records the environment values the two lines print, and the final run shows they are no longer printed.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- Less debugging detail in CI logs. Mitigation: log the redacted configuration, not nothing.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #2993 (EPIC), #3004
- Related ADRs: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`
