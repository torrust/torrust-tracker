---
doc-type: issue
issue-type: enhancement
status: planned
priority: p1
github-issue: 1769
spec-path: docs/issues/open/1769-refactor-pre-commit-checks-performance-and-verbosity.md
branch: "1769-refactor-pre-commit-checks-performance-and-verbosity"
related-pr: null
last-updated-utc: 2026-05-13 09:28
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - contrib/dev-tools/git/hooks/pre-push.sh
    - .github/workflows/testing.yaml
    - .github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #1769 - Refactor pre-commit checks for lower verbosity and faster feedback

## Goal

Improve local commit-time feedback by making pre-commit output concise by default and reducing unnecessary runtime, while preserving strong quality guarantees through pre-push and CI.

## Background

Current pre-commit flow in [contrib/dev-tools/git/hooks/pre-commit.sh](../../../contrib/dev-tools/git/hooks/pre-commit.sh):

1. `cargo machete`
2. `linter all`
3. `cargo test --doc --workspace`
4. `cargo test --tests --benches --examples --workspace --all-targets --all-features`

Current pre-push flow in [contrib/dev-tools/git/hooks/pre-push.sh](../../../contrib/dev-tools/git/hooks/pre-push.sh) already runs comprehensive validation and includes E2E. CI in [.github/workflows/testing.yaml](../../../.github/workflows/testing.yaml) also runs E2E matrix jobs.

Key finding:

- E2E is not part of pre-commit today. The pre-commit pain is mainly verbosity and broad test scope for frequent local commits.

Automation policy constraint:

- We do not want to couple workflow automation exclusively to GitHub-native services (for example Dependabot) when defining core maintenance processes.
- The process should remain portable: executable in different CI/CD infrastructures and usable with different AI providers.
- GitHub ecosystem tools can still be used as optional integrations, but not as the only execution path.

## Scope

### In Scope

- Add concise/verbose output modes to pre-commit with better failure summaries and log-path reporting.
- Measure current vs proposed pre-commit runtime and output quality.
- Define and document command ownership by tier (pre-commit, pre-push, CI).
- Adjust pre-commit step composition to optimize local cycle time without reducing merge safety.

### Out of Scope

- Removing comprehensive checks from pre-push/CI.
- E2E redesign.
- Changes unrelated to developer workflow/quality gates.

## Deep Analysis Summary

### A. Verbosity issues

- Current commands stream full output, producing noisy terminal sessions.
- Failures can be hard to spot in long logs.
- High-volume output contributes to tooling output transport instability for agent execution.

### B. Runtime issues

- Pre-commit runs broad workspace tests on every commit.
- Heavy checks are duplicated in pre-push/CI.
- For docs/small changes, local wait time is disproportionate to change risk.

Observed baseline run (2026-05-13, local):

| Step  | Command                                                                            | Elapsed |
| ----- | ---------------------------------------------------------------------------------- | ------- |
| 1     | `cargo machete`                                                                    | 0s      |
| 2     | `linter all`                                                                       | 7s      |
| 3     | `cargo test --doc --workspace`                                                     | 50s     |
| 4     | `cargo test --tests --benches --examples --workspace --all-targets --all-features` | 17s     |
| Total | pre-commit script                                                                  | 1m 14s  |

Observation from this run: documentation tests were slower than the broader test command. Profile decisions must be based on multi-run data, not assumptions.

### C. Boundary between pre-commit and heavier tiers

- Pre-commit should optimize for fast, high-signal local feedback.
- Pre-push and CI should remain comprehensive and authoritative for merge readiness.

## Proposed Changes

### Task 1: Add output modes and failure-focused summaries

CLI contract:

- [ ] Add `--format=<text|json>` where:
  - `--format=text` is the default (human-friendly terminal output)
  - `--format=json` emits a single JSON document to stdout
- [ ] Add `--verbosity=<concise|verbose>` where:
  - `--verbosity=concise` is the default
  - `--verbosity=verbose` streams full command output
- [ ] Keep `--verbose` as a compatibility alias for `--verbosity=verbose`.
- [ ] Define precedence explicitly:
  - when `--format=json`, output remains structured JSON regardless of verbosity value
  - for `--format=text`, verbosity controls concise vs full streaming output
- [ ] Define argument conflict/error behavior explicitly:
  - duplicate `--format`/`--verbosity` flags: last value wins
  - `--verbose` alias sets `--verbosity=verbose`
  - invalid values (for example `--format=xml`): fail with exit code `2` and usage hint
  - unknown flags: fail with exit code `2` and usage hint
  - output channel contract: structured output goes to stdout, diagnostics/errors to stderr

Modes matrix:

| Format | Verbosity              | Behavior                       |
| ------ | ---------------------- | ------------------------------ |
| `text` | `concise` (default)    | High-signal summary per step   |
| `text` | `verbose`              | Full streaming command output  |
| `json` | `concise` or `verbose` | Single JSON document to stdout |

- [ ] Add `--format` and `--verbosity` flags to [contrib/dev-tools/git/hooks/pre-commit.sh](../../../contrib/dev-tools/git/hooks/pre-commit.sh).
- [ ] In concise mode, capture per-step logs and print only:
  - step name, pass/fail, elapsed time
  - log path and a short failure tail when a step fails
- [ ] Keep full streaming output in `--verbosity=verbose` mode for `--format=text`.
- [ ] In `--format=json` mode, write a single JSON document to stdout (see examples below).

#### Example command calls

```sh
# Default behavior
./contrib/dev-tools/git/hooks/pre-commit.sh

# Explicit text + concise (equivalent to default)
./contrib/dev-tools/git/hooks/pre-commit.sh --format=text --verbosity=concise

# Text + verbose
./contrib/dev-tools/git/hooks/pre-commit.sh --format=text --verbosity=verbose

# Compatibility alias for verbose text output
./contrib/dev-tools/git/hooks/pre-commit.sh --format=text --verbose

# Structured output for agents/scripts
./contrib/dev-tools/git/hooks/pre-commit.sh --format=json
```

#### Example: concise default (all pass)

```sh
./contrib/dev-tools/git/hooks/pre-commit.sh
```

```text
Running pre-commit checks...

[Step 1/4] Checking for unused dependencies (cargo machete)  ... PASS  (0s)
[Step 2/4] Running all linters (linter all)                  ... PASS  (7s)
[Step 3/4] Running documentation tests                       ... PASS (50s)
[Step 4/4] Running all tests                                 ... PASS (17s)

==========================================
SUCCESS: All pre-commit checks passed! (1m 14s)
==========================================
```

#### Example: concise default (step fails)

```sh
./contrib/dev-tools/git/hooks/pre-commit.sh
```

```text
Running pre-commit checks...

[Step 1/4] Checking for unused dependencies (cargo machete)  ... PASS  (0s)
[Step 2/4] Running all linters (linter all)                  ... FAIL (11s)  log: /tmp/pre-commit-linter-all-20260513-083055.log
    error[E0001]: unused variable `x` at src/lib.rs:42
    error: aborting due to 1 previous error
    (2 lines shown — full log: /tmp/pre-commit-linter-all-20260513-083055.log)

==========================================
FAILED: Pre-commit checks failed!
Fix the errors above before committing.
==========================================
```

#### Example: `--format=json` mode (all pass)

```sh
./contrib/dev-tools/git/hooks/pre-commit.sh --format=json
```

```json
{
  "schema_version": 1,
  "status": "pass",
  "exit_code": 0,
  "elapsed_seconds": 74,
  "steps": [
    {
      "name": "Checking for unused dependencies",
      "command": "cargo machete",
      "status": "pass",
      "elapsed_seconds": 0
    },
    {
      "name": "Running all linters",
      "command": "linter all",
      "status": "pass",
      "elapsed_seconds": 7
    },
    {
      "name": "Running documentation tests",
      "command": "cargo test --doc --workspace",
      "status": "pass",
      "elapsed_seconds": 50
    },
    {
      "name": "Running all tests",
      "command": "cargo test --tests --benches --examples --workspace --all-targets --all-features",
      "status": "pass",
      "elapsed_seconds": 17
    }
  ]
}
```

#### Example: `--format=json` mode (step fails)

```sh
./contrib/dev-tools/git/hooks/pre-commit.sh --format=json
```

```json
{
  "schema_version": 1,
  "status": "fail",
  "exit_code": 1,
  "elapsed_seconds": 11,
  "failed_step": "Running all linters",
  "steps": [
    {
      "name": "Checking for unused dependencies",
      "command": "cargo machete",
      "status": "pass",
      "elapsed_seconds": 0
    },
    {
      "name": "Running all linters",
      "command": "linter all",
      "status": "fail",
      "elapsed_seconds": 11,
      "log_path": "/tmp/pre-commit-linter-all-20260513-083055.log",
      "failure_tail": [
        "error[E0001]: unused variable `x` at src/lib.rs:42",
        "error: aborting due to 1 previous error"
      ]
    }
  ]
}
```

### Task 2: Baseline timing and propose tuned pre-commit profile

- [ ] Measure current pre-commit runtime over at least 3 runs.
- [ ] Measure candidate profile runtime over at least 3 runs.
- [ ] Compare results and choose a profile with documented rationale.

Candidate profiles:

- Profile A (provisional until multi-run evidence is collected): `cargo machete` + `linter all` + `cargo test --doc --workspace`.
- Profile B: retain full tests but with concise default output.

Evaluation note:

- Because a real baseline run showed `cargo test --doc --workspace` as the slowest step, the final profile selection must be decided after the required multi-run timing table is completed.

### Task 3: Clarify check tiers and ownership

- [ ] Document which checks are mandatory at each tier:
  - pre-commit (fast local gate)
  - pre-push (comprehensive developer gate)
  - CI (merge authority)
- [ ] Keep E2E explicitly out of pre-commit and documented as pre-push/CI responsibility.

### Task 4: Update workflow docs and skills

- [ ] Update [.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md](../../../.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md) with new behavior and flags.
- [ ] Update references in [AGENTS.md](../../../AGENTS.md) and related skills if command expectations changed.
- [ ] Add troubleshooting notes for concise vs verbose mode.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                              | Notes / Expected Output                             |
| --- | ------ | --------------------------------- | --------------------------------------------------- |
| T1  | TODO   | Baseline current pre-commit stats | Runtime and output-size baseline collected.         |
| T2  | TODO   | Implement output mode refactor    | Concise default + verbose opt-in implemented.       |
| T3  | TODO   | Select and apply runtime profile  | Profile selected with measured trade-off rationale. |
| T4  | TODO   | Update docs/skills                | Workflow docs and skills aligned.                   |
| T5  | TODO   | Validate gates and regression     | `linter all` and relevant test checks pass.         |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-13 07:33 UTC - Copilot - Created focused pre-commit refactor draft split from combined proposal.
- 2026-05-13 08:42 UTC - Copilot - Executed `./contrib/dev-tools/git/hooks/pre-commit.sh` and captured baseline output (`1m 14s` total; docs `50s`, tests `17s`).
- 2026-05-13 09:26 UTC - Copilot - Opened GitHub issue #1769 and moved this spec to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: Pre-commit supports `--format=<text|json>` and `--verbosity=<concise|verbose>` with documented defaults and precedence rules.
- [ ] AC2: `--format=text --verbosity=concise` prints high-signal step summaries and log paths on failure; `--format=json` emits a single valid JSON document matching the schema in Task 1.
- [ ] AC2.1: Invalid flags/values fail with exit code `2`, print usage guidance, and write diagnostics to stderr.
- [ ] AC3: Chosen pre-commit profile is backed by timing data from multiple runs.
- [ ] AC4: Check-tier ownership is documented and consistent across scripts and docs.
- [ ] AC5: E2E remains excluded from pre-commit and explicitly mapped to pre-push/CI.
- [ ] AC6: `linter all` exits with code `0` after changes.
- [ ] AC7: Relevant checks pass for modified hook behavior.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                            |
| ----- | ---------------------- | --------------------------------------------------------------------------------------------------- |
| AC1   | TODO                   | Updated pre-commit script usage/flags (`--format`, `--verbosity`, `--verbose` alias)                |
| AC2   | TODO                   | Sample `--format=text --verbosity=concise` logs and `--format=json` output against schema in Task 1 |
| AC2.1 | TODO                   | Negative tests for invalid/unknown flags and stderr/exit-code checks                                |
| AC3   | TODO                   | Runtime comparison table                                                                            |
| AC4   | TODO                   | Updated docs/skills references                                                                      |
| AC5   | TODO                   | Hook/CI command mapping                                                                             |
| AC6   | TODO                   | `linter all` output                                                                                 |
| AC7   | TODO                   | Test/check outputs                                                                                  |

## Risks and Trade-offs

- Reducing local checks too far can miss early regressions.
  - Mitigation: keep pre-push/CI comprehensive and document boundaries clearly.
- Concise output can hide details during debugging.
  - Mitigation: preserve full verbose mode and always record log file paths.
- Hook complexity can grow over time (argument parsing, structured output, log orchestration).
  - Mitigation: if complexity becomes hard to maintain in shell, migrate the hook logic to a small Rust CLI and keep the shell hook as a thin entrypoint.
- Captured logs can include ANSI color codes and multiline errors that are harder to parse in JSON consumers.
  - Mitigation: strip ANSI sequences in `--format=json` mode and keep raw logs on disk.
- Script interruption (Ctrl+C) can leave partial state or truncated output.
  - Mitigation: add trap handling that emits a deterministic non-zero exit and a final status line/JSON payload where feasible.

## References

- Pre-commit hook: [contrib/dev-tools/git/hooks/pre-commit.sh](../../../contrib/dev-tools/git/hooks/pre-commit.sh)
- Pre-push hook: [contrib/dev-tools/git/hooks/pre-push.sh](../../../contrib/dev-tools/git/hooks/pre-push.sh)
- CI testing workflow: [.github/workflows/testing.yaml](../../../.github/workflows/testing.yaml)
- Skill reference: [.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md](../../../.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md)
- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1769
- Related split issue spec: [docs/issues/open/1768-refactor-update-dependencies-skill-automation.md](1768-refactor-update-dependencies-skill-automation.md)
