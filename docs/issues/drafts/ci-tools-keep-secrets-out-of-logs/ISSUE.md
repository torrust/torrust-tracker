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
last-updated-utc: "2026-11-23 18:08"
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
| `src/console/ci/compose.rs:111`, `:317` | `tracing::info!("Running docker compose command: {:?}", command)`: the `Debug` of a `std::process::Command`, which prints the environment set with `.envs(...)` |
| `src/console/ci/e2e/tracker_checker.rs:88` | `tracing::info!("Tracker Checker config:\n{config}")`: the checker configuration (issue #3004 noted it and left it out of its scope) |
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

A unit test per redaction helper (configuration text, `docker run` arguments, compose environment) that builds the input from a unique marker value and asserts the marker is not in the logged text. The E2E run itself is the manual check.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Baseline | The sites re-checked; one E2E run's log lines recorded with test values. |
| T2 | TODO | Configuration logs | `runner.rs:80`, `tracker_checker.rs:88` log a redacted form. |
| T3 | TODO | Command logs | `docker.rs:89`, `compose.rs:111`, `:317` log no environment value. |
| T4 | TODO | Database URL constants | `config_builder.rs:16-17` removed or rebuilt. |
| T5 | TODO | Final recheck | The E2E run repeated; no secret in the log. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Configuration log redaction with its tests | Commit after focused validation and required review. |
| T3 | Command log redaction with its tests | Commit after focused validation and required review. |
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

## Acceptance Criteria

- [ ] AC1: No log line of the CI tools under `src/console/ci/` holds an access token or a database password from the configuration or the environment.
- [ ] AC2: The log still says which configuration file, container and command the tool used.
- [ ] AC3: No constant in the CI tools holds a database URL with a password, or the reason it must stay is recorded.
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
| M1 | E2E runner log | Run the E2E runner locally with a configuration whose admin token and database password are unique markers; read the log | No marker in the log | TODO | `manual-verification-evidence.md` section V1 |

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- Less debugging detail in CI logs. Mitigation: log the redacted configuration, not nothing.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #2993 (EPIC), #3004
- Related ADRs: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`
