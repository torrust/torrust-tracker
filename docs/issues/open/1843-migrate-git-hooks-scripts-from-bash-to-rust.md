---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1843
spec-path: docs/issues/open/1843-migrate-git-hooks-scripts-from-bash-to-rust.md
branch: "1843-migrate-git-hooks-scripts-from-bash-to-rust"
related-pr: null
last-updated-utc: 2026-05-27 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - contrib/dev-tools/git/hooks/pre-push.sh
    - contrib/dev-tools/git/install-git-hooks.sh
    - .githooks/pre-commit
    - .githooks/pre-push
    - .github/workflows/copilot-setup-steps.yml
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/issues/open/1774-automate-cleanup-completed-issues-skill-script.md
    - .github/skills/dev/git-workflow/create-feature-branch/SKILL.md
    - .github/agents/committer.agent.md
---

<!-- skill-link: create-issue -->

# Issue #1843 — Migrate git hooks scripts from Bash to Rust

## Goal

Replace the three Bash scripts that implement pre-commit checks, pre-push checks, and git hook
installation with a single Rust binary that improves testability, type safety, and
maintainability, and that adds real-time feedback during long-running checks so developers and
automation agents can see hook progress without cancelling valid runs.

## Background

The repository ships three Bash scripts under `contrib/dev-tools/git/`:

| Script                                       | Purpose                                                                                                                       |
| -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `contrib/dev-tools/git/hooks/pre-commit.sh`  | Runs fast quality checks (`cargo machete --with-metadata`, linter, doc tests). Supports `--format`, `--verbosity`, log files. |
| `contrib/dev-tools/git/hooks/pre-push.sh`    | Runs comprehensive checks (machete, linters, nightly build, tests, E2E). Supports the same flags.                             |
| `contrib/dev-tools/git/install-git-hooks.sh` | Copies hooks from `.githooks/` to `.git/hooks/` on developer setup.                                                           |

These scripts have grown beyond simple orchestration. Both `pre-commit.sh` and `pre-push.sh` now
implement:

- Structured argument parsing (`--format=text|json`, `--verbosity=concise|verbose`, `--verbose`,
  `-h|--help`)
- A multi-step runner with per-step timing, log-file management, and early exit on failure
- Two output modes: human-readable text (concise and verbose) and machine-readable JSON
- ANSI stripping, JSON escaping, and safe name normalization for log files
- Environment variable support (`TORRUST_GIT_HOOKS_LOG_DIR`)

This logic is duplicated across the two scripts (they share the same ~250-line framework,
differing only in the `STEPS` array). Both scripts are already referenced extensively across
the codebase: `.githooks/` dispatcher scripts, CI workflows, agent configurations, and
multiple skill files.

### Feedback UX problems

Beyond the maintainability problems above, the current scripts have a feedback UX gap:

- `git commit` and `git push` look hung when hooks run long checks (pre-push takes ~15 min).
- Default output collects all step logs and prints only at the end; nothing is visible mid-run.
- Lack of real-time progress causes both developers and AI agents to cancel valid runs.
- There is no way to distinguish a slow-but-active check from a stalled or failed one.
- In non-interactive (agent/CI) shells the auto-selected JSON format delivers a single blob at
  exit, providing no intermediate signal.

The Rust rewrite is the right moment to fix this: the binary can emit structured progress events
as each step starts and ends, plus periodic heartbeat events during long steps.

### Redundant execution problems

Beyond feedback, the hooks also run unnecessarily:

- If only Markdown or documentation files are staged, pre-commit still runs the full Rust suite
  (`cargo machete`, `linter all`, `cargo test --doc`) — an expensive operation that adds no
  signal for a docs-only change.
- Both hooks re-execute even when they already passed for exactly the same set of changes.
  Amending a commit message or retrying a push after a network error re-runs all steps.
- There is currently no local record that a given set of staged changes or a given commit has
  already been validated, so developers pay the full cost on every attempt.

A Rust binary can analyse staged file types and cache pass results efficiently; shell scripts
cannot do this reliably.

Engineering policy #3 in `AGENTS.md` states:

> Use shell scripts for simple orchestration only. When logic becomes non-trivial, stateful,
> safety-critical, or worth testing independently, prefer Rust.

The current scripts clearly exceed that threshold. Migrating them to Rust will:

A global CLI output contract ADR (`docs/adrs/20260519000000_define_global_cli_output_contract.md`)
was also recently adopted, prescribing that **new** repository binaries must use structured JSON
output on both stdout and stderr, with no plain text permitted. The new git hooks runner binary
must be designed in conformance with this contract from day one. In particular:

- The runner likely classifies as `no-stdout-result` (pass/fail via exit code, all diagnostics
  to stderr as NDJSON) — analogous to `e2e_tests_runner`.
- The existing `--format=text|json` switch needs to be reconsidered: under the ADR, all output
  is always JSON. The binary should accept a `--verbosity` flag that controls _how much_ JSON
  is emitted, not _whether_ it is JSON.
- This is a design decision to settle in T1/T3 and must be documented in the spec before
  implementation begins.

The current scripts clearly exceed the simple-orchestration threshold. Migrating them to Rust will:

- Eliminate duplicated logic between the two scripts through a shared library
- Make the step-runner framework independently testable with unit and integration tests
- Provide compile-time guarantees for argument parsing and output formatting
- Simplify future extension (new output formats, additional hooks, config-file support)

The thin `.githooks/pre-commit` and `.githooks/pre-push` dispatcher scripts **must remain Bash**
(git requires hook executables to be directly invocable by the shell), but their bodies reduce
to a single delegate call to the Rust binary.

## Scope

### In Scope

- Create a new Rust binary crate at `contrib/dev-tools/git/` (or a fitting sub-path; see T1).
- Implement a `pre-commit` subcommand replicating the steps from `pre-commit.sh`, with output
  redesigned to comply with the global CLI output contract ADR.
- Implement a `pre-push` subcommand replicating the steps from `pre-push.sh`, with output
  redesigned to comply with the global CLI output contract ADR.
- Implement an `install-hooks` subcommand replicating the behaviour of `install-git-hooks.sh`.
- Design and implement a structured progress event model (NDJSON on stderr) that emits:
  - A hook-start event immediately when the binary is invoked (step list, expected count).
  - A step-start event before each step begins.
  - A step-end event with elapsed time and pass/fail status when each step finishes.
  - Periodic heartbeat events (every 20–30 seconds) during long-running steps, including
    current step name and elapsed duration.
  - A final result event summarising overall pass/fail and total elapsed time.
- Implement line-buffered output so each event is flushed immediately and is visible in
  real time rather than buffered until exit.
- Comply with the global CLI output contract ADR (§1, §2, §5) from day one: emit nothing on
  stdout (`no-stdout-result` class); write all output to stderr as NDJSON; communicate
  pass/fail via exit code only (0 = success, 1 = runtime failure, 2 = usage error). The
  `--format=text|json` switch present in the existing Bash scripts is not ported; format is
  always NDJSON. If T1 determines that the developer-tool exemption should be claimed (cf.
  `profiling` binary), document the rationale before implementation begins.
- Implement explicit diagnostics that distinguish an active-but-slow step from a failed one.
- Expose a `--verbosity=<concise|verbose>` flag controlling how much detail is included in
  progress events (e.g. whether step commands are echoed); keep `TORRUST_GIT_HOOKS_LOG_DIR`.
- Implement staged file type analysis for `pre-commit`: inspect the list returned by
  `git diff --cached --name-only` and classify the changeset as Markdown-only,
  documentation-only, or mixed/Rust. When the changeset is Markdown-only, run only the
  markdown-relevant linter steps (e.g., `linter markdown` and `linter cspell`); skip
  `cargo machete`, Rust linters, and `cargo test --doc`. Emit a `step_skip` event for each
  skipped step so the output record is complete.
- Implement pre-commit idempotency: compute the staged tree SHA (`git write-tree`) before
  running steps; if a pass record for that tree SHA already exists in
  `.git/torrust-hooks/pre-commit-cache`, exit 0 immediately without re-running steps. Write a
  pass record to the cache when all steps succeed. The cache key must also include a hash of the
  active step configuration so that adding or changing a step automatically invalidates old
  records.
- Implement pre-push idempotency: for each commit SHA in the set about to be pushed, check
  whether a pass record exists in `.git/torrust-hooks/pre-push-cache`. If all commits have
  passing records, exit 0 immediately. Write pass records per commit SHA when the hook succeeds.
- Add unit tests for the step-runner, argument parsing, event schema, output flushing, staged
  file classification, and cache read/write/invalidation logic.
- Add the new crate to the workspace `members` list in the root `Cargo.toml`.
- Update `.githooks/pre-commit` and `.githooks/pre-push` to delegate to the Rust binary
  (falling back gracefully with an informative error if the binary is not built).
- Remove `pre-commit.sh`, `pre-push.sh`, and `install-git-hooks.sh` once the Rust binary
  is verified end-to-end.
- Update all references across skills, agent configs, `AGENTS.md`, CI workflows, and
  documentation to point to the new binary invocation.

### Out of Scope

- Changing the set of steps run by pre-commit or pre-push checks (when the full suite applies).
- Adding a separate human-friendly pretty-printer binary or wrapper script.
- Migrating other `contrib/dev-tools/` scripts (e.g., analysis tools).
- Remote or CI-shared caching; the idempotency cache is strictly local (`.git/torrust-hooks/`).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

The plan is split into two phases. **Phase 1** replaces the three Bash scripts with the Rust
binary, implementing only what will exist in the new version — the same check steps, NDJSON
output only (the old `--format=text|json` switch is not ported), `--verbosity`, and
`TORRUST_GIT_HOOKS_LOG_DIR`. When Phase 1 is complete the binary is put into service and the
Bash scripts are removed. **Phase 2** adds new capabilities on top of the already-deployed binary.

### Phase 1 — Core migration (same steps, NDJSON output, switch over and remove old scripts)

| ID  | Status | Task                                                                                  | Notes / Expected Output                                                                                                                                                                                                                                                                                                          |
| --- | ------ | ------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Decide crate location, name, CLI output design, and ADR classification                | Candidate: `contrib/dev-tools/git/git-hooks-runner/`; binary name `torrust-git-hooks`; settle binary class (`no-stdout-result` vs `stdout-result-data`) under the global CLI output contract ADR; decide whether developer-tool exemption applies (cf. `profiling` binary); confirm with maintainer                              |
| T2  | TODO   | Scaffold new crate and add to workspace                                               | `Cargo.toml` `members` includes the new crate; `cargo build -p <crate-name>` succeeds                                                                                                                                                                                                                                            |
| T3  | TODO   | Design full NDJSON event schema (Phase 1 + Phase 2 events)                            | Define all `kind` values including Phase 2 events (`heartbeat`, `step_skip`); document field names, types, and which phase implements each; store schema doc in crate or `docs/`; Phase 1 implements: `hook_start`, `step_start`, `step_end`, `hook_result` only                                                                 |
| T4  | TODO   | Implement shared step-runner library (argument parsing, timing, basic event emission) | Emits `hook_start`, `step_start`, `step_end`, `hook_result` on stderr as NDJSON; line-buffered; `--verbosity=concise\|verbose`; `TORRUST_GIT_HOOKS_LOG_DIR`; no heartbeat (Phase 2); unit-tested                                                                                                                                 |
| T5  | TODO   | Implement `pre-commit` subcommand                                                     | Same 3 steps as `pre-commit.sh`; no `--format` flag; exits 0/1/2; unit-tested                                                                                                                                                                                                                                                    |
| T6  | TODO   | Implement `pre-push` subcommand                                                       | Same 8 steps as `pre-push.sh`; no `--format` flag; exits 0/1/2; unit-tested                                                                                                                                                                                                                                                      |
| T7  | TODO   | Implement `install-hooks` subcommand                                                  | Mirrors `install-git-hooks.sh`; copies `.githooks/*` to `.git/hooks/` and makes them executable                                                                                                                                                                                                                                  |
| T8  | TODO   | Implement ADR-compliant output contract                                               | Emit NDJSON on stderr in all modes (ADR §1, §5); exit code contract 0/1/2 (ADR §2); structured NDJSON writer — no `print!`/`eprint!`/`println!`/`eprintln!` (ADR §8); `--verbosity` controls detail level only. If T1 grants the developer-tool exemption, extend to render events in a human-readable form when stderr is a TTY |
| T9  | TODO   | Add Phase 1 unit and integration tests                                                | Cover: argument parsing, verbosity combinations, basic NDJSON schema validity, graceful failure, log-file creation, `TORRUST_GIT_HOOKS_LOG_DIR` override, exit code contract                                                                                                                                                     |
| T10 | TODO   | Update `.githooks/pre-commit` and `.githooks/pre-push`                                | Thin wrappers that build/locate the binary and delegate; emit a clear error if binary is missing                                                                                                                                                                                                                                 |
| T11 | TODO   | Remove `pre-commit.sh`, `pre-push.sh`, `install-git-hooks.sh`                         | Delete the three Bash files after the Rust binary is verified end-to-end — **migration is complete; binary is now in service**                                                                                                                                                                                                   |
| T12 | TODO   | Update `AGENTS.md` references                                                         | Replace script paths with binary invocation (`torrust-git-hooks pre-commit`) in descriptions and the mandatory quality gate section                                                                                                                                                                                              |
| T13 | TODO   | Update all skill files                                                                | `run-pre-commit-checks`, `run-pre-push-checks`, `setup-dev-environment`, `add-rust-dependency`, `update-dependencies` — replace `.sh` invocations with the binary command                                                                                                                                                        |
| T14 | TODO   | Update agent config files                                                             | `committer.agent.md`, `implementer.agent.md` — replace script paths; document how agents should consume NDJSON progress events                                                                                                                                                                                                   |
| T15 | TODO   | Update CI workflow                                                                    | `.github/workflows/copilot-setup-steps.yml` caches/file references updated to new binary path or build step                                                                                                                                                                                                                      |
| T16 | TODO   | Verify Phase 1 quality gates                                                          | `linter all`, full test suite, pre-commit and pre-push hooks exercise the new binary end-to-end; all Phase 1 ACs met                                                                                                                                                                                                             |

### Phase 2 — Enhancements (new features not present in the original Bash scripts)

| ID  | Status | Task                                                         | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                       |
| --- | ------ | ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T17 | TODO   | Implement heartbeat emitter                                  | Background ticker fires every 20–30s while a step is running; emits `heartbeat` NDJSON event (step name, elapsed seconds); extends T3 schema                                                                                                                                                                                                                                                  |
| T18 | TODO   | Implement staged file type analysis and smart step selection | `git diff --cached --name-only`; classify changeset (Markdown-only / docs-only / mixed); skip inapplicable steps; emit `step_skip` NDJSON events for skipped steps; extends T3 schema                                                                                                                                                                                                         |
| T19 | TODO   | Implement pre-commit idempotency cache                       | Compute staged tree SHA (`git write-tree`) + step-config hash; check/write `.git/torrust-hooks/pre-commit-cache`; exit 0 immediately on cache hit                                                                                                                                                                                                                                             |
| T20 | TODO   | Implement pre-push idempotency cache                         | Check/write per-commit-SHA records in `.git/torrust-hooks/pre-push-cache`; exit 0 immediately when all pushed commits have passing records                                                                                                                                                                                                                                                    |
| T21 | TODO   | Add Phase 2 unit and integration tests                       | Cover: heartbeat timing and event shape, staged file classification, smart step selection, cache read/write/invalidation, cache-and-smart-skip interaction                                                                                                                                                                                                                                    |
| T22 | TODO   | Implement branch-name validation                             | When the branch uses an issue-number prefix (e.g. `42-some-description`), verify that `docs/issues/open/` contains a matching spec file or directory. If none found, emit a warning event and optionally block the commit. Prevents committing under a wrong, closed, or non-existent issue number. See `docs/issues/open/1774-automate-cleanup-completed-issues-skill-script.md` for context |
| T23 | TODO   | Verify Phase 2 quality gates                                 | `linter all`, full test suite; all Phase 2 ACs met                                                                                                                                                                                                                                                                                                                                            |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-18 00:00 UTC - Agent - Spec drafted based on codebase analysis and user request
- 2026-05-27 00:00 UTC - Agent - Develop branch updated (merged e75c25ac..6d90e1fb); noted global CLI output contract ADR and pre-commit step description update (`cargo machete --with-metadata`)
- 2026-05-27 00:00 UTC - Agent - Incorporated hook output UX improvement ideas: progressive output, heartbeat events, NDJSON streaming, TTY auto-detection, flush behaviour, and active-vs-failed diagnostics
- 2026-05-27 00:00 UTC - Agent - Incorporated two further ideas: smart step skipping for Markdown-only staged changesets; idempotent hook execution via local SHA-keyed cache
- 2026-05-27 00:00 UTC - Agent - Aligned spec with global CLI output contract ADR: NDJSON on stderr in all modes; removed TTY/human-text assumption; fixed AC9, T8, M1–M3 exit codes; added ADR §8 lint guard and §9 agent capture risks
- 2026-05-27 00:00 UTC - Agent - Restructured implementation plan into Phase 1 (core migration, switch over, remove old scripts) and Phase 2 (enhancements); heartbeat moved to Phase 2 (T17); T3 now designs full schema upfront; Phase 1 tests scoped to Phase 1 features; Phase 2 adds T21 tests and T22 verify

## Acceptance Criteria

- [ ] AC1: A Rust binary (`torrust-git-hooks` or agreed name) exists under `contrib/dev-tools/git/`
- [ ] AC2: `torrust-git-hooks pre-commit [--verbosity=...]` runs the same steps as the former `pre-commit.sh` and exits with code 0 on success, 1 on runtime failure, or 2 on usage error (ADR §2); stdout is always empty
- [ ] AC3: `torrust-git-hooks pre-push [--verbosity=...]` runs the same steps as the former `pre-push.sh` and exits with code 0 on success, 1 on runtime failure, or 2 on usage error (ADR §2); stdout is always empty
- [ ] AC4: `torrust-git-hooks install-hooks` installs `.githooks/*` into `.git/hooks/` with correct permissions
- [ ] AC5: The first output event appears within 1 second of hook invocation (hook-start event; not buffered until exit)
- [ ] AC6: Each step emits a step-start event before the step's subprocess begins and a step-end event when it finishes
- [ ] AC7: During any step running longer than 30 seconds, a heartbeat event is emitted every 20–30 seconds with step name and elapsed time
- [ ] AC8: The output event schema is documented (NDJSON `kind` values, field names, and types)
- [ ] AC9: No plain text is emitted on stdout or stderr at any verbosity level; all output is NDJSON on stderr; stdout is always empty (ADR §1, §5). TTY state does not affect the output format.
- [ ] AC10: `.githooks/pre-commit` and `.githooks/pre-push` delegate to the Rust binary and emit a clear error if the binary has not been built
- [ ] AC11: The three former Bash scripts are removed from the repository
- [ ] AC12: All references in `AGENTS.md`, skills, agent configs, and CI workflows are updated to the binary invocation
- [ ] AC13: The new crate is included in the workspace and `cargo build --workspace` succeeds
- [ ] AC14: Unit tests cover argument parsing, verbosity, NDJSON schema, heartbeat logic, and step-runner; `cargo test -p <crate>` passes
- [ ] AC15: `linter all` exits `0`
- [ ] AC16: Pre-commit and pre-push hooks run end-to-end using the Rust binary on the developer machine
- [ ] AC17: Manual verification scenarios are executed and documented (status + evidence)
- [ ] AC18: Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] AC19: When only `*.md` files (and documentation-adjacent files) are staged, `pre-commit` skips Rust-specific steps and runs only markdown-relevant linters; a `step_skip` NDJSON event is emitted for each skipped step
- [ ] AC20: A second `torrust-git-hooks pre-commit` invocation with an unchanged staged tree (same `git write-tree` SHA and step config) exits 0 immediately without re-running any step
- [ ] AC21: A `torrust-git-hooks pre-push` invocation where all commits in the push already have passing cache records exits 0 immediately without re-running any step
- [ ] AC22: When the current branch has an issue-number prefix (e.g. `42-some-description`), the `pre-commit` subcommand verifies that a matching spec exists in `docs/issues/open/`. If none is found, it emits a warning event and blocks the commit with exit code 1.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p <crate-name>` (unit and integration tests for the new crate)
- `cargo test --doc --workspace`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features`
- Pre-commit hook (exercises the new binary end-to-end)
- Pre-push hook (exercises the new binary end-to-end)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                               | Command/Steps                                                                      | Expected Result                                                                                                                     | Status | Evidence |
| --- | ------------------------------------------------------ | ---------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Pre-commit NDJSON concise output (pass path)           | `torrust-git-hooks pre-commit --verbosity=concise`                                 | NDJSON `hook_start` event on stderr within 1 s; `step_start`/`step_end` per step; `hook_result` with `status: "pass"`; stdout empty | TODO   |          |
| M2  | Pre-commit NDJSON verbose output (pass path)           | `torrust-git-hooks pre-commit --verbosity=verbose`                                 | NDJSON events include step command details and full step output; `hook_result` with `status: "pass"`; stdout empty                  | TODO   |          |
| M3  | Pre-commit NDJSON output verified via pipe (pass path) | `torrust-git-hooks pre-commit 2>stderr.ndjson; cat stderr.ndjson`                  | Every line in `stderr.ndjson` is a valid JSON object; `hook_result` event present; stdout file empty                                | TODO   |          |
| M4  | Pre-commit NDJSON output (fail path)                   | Introduce a deliberate lint error; run `torrust-git-hooks pre-commit 2>&1 \| cat`  | `step_end` event with `status: "fail"`; `hook_result` fail; non-zero exit                                                           | TODO   |          |
| M5  | Pre-push interactive output (pass path)                | `torrust-git-hooks pre-push --verbosity=concise` in a TTY                          | All steps emit start/end events with elapsed time; overall PASS                                                                     | TODO   |          |
| M6  | Heartbeat during long-running step                     | Run `torrust-git-hooks pre-push`; observe a step that takes > 30 s                 | `heartbeat` NDJSON event(s) appear before step ends                                                                                 | TODO   |          |
| M7  | First event appears immediately on hook start          | `time torrust-git-hooks pre-commit --verbosity=concise 2>&1 \| head -1`            | First line appears within 1 second of invocation                                                                                    | TODO   |          |
| M8  | `install-hooks` installs correctly                     | `torrust-git-hooks install-hooks`                                                  | Hooks copied to `.git/hooks/`; each is executable                                                                                   | TODO   |          |
| M9  | `TORRUST_GIT_HOOKS_LOG_DIR` override                   | `TORRUST_GIT_HOOKS_LOG_DIR=.tmp torrust-git-hooks pre-commit 2>/dev/null`          | Log files created under `.tmp/`; no files in `/tmp`                                                                                 | TODO   |          |
| M10 | `.githooks/pre-commit` dispatcher delegates to binary  | `git commit` in a clean state                                                      | Hook exits 0; Rust binary output visible during run                                                                                 | TODO   |          |
| M11 | `.githooks/pre-commit` error when binary not built     | Delete/rename the binary, then trigger `git commit`                                | Clear human-readable error message; hook exits non-zero                                                                             | TODO   |          |
| M12 | Active-step diagnostic distinguishable from failure    | Start `torrust-git-hooks pre-push`; while a long step runs, observe output         | Output shows step is still running (heartbeat); no false failure                                                                    | TODO   |          |
| M13 | Non-interactive auto-detection in pipeline             | `torrust-git-hooks pre-commit 2>stderr.txt; cat stderr.txt`                        | `stderr.txt` contains valid NDJSON lines (not plain text)                                                                           | TODO   |          |
| M14 | Smart step skip — Markdown-only staged changeset       | Stage only a `*.md` file; run `torrust-git-hooks pre-commit --verbosity=verbose`   | Only markdown/cspell steps run; cargo steps show `step_skip` events; overall PASS                                                   | TODO   |          |
| M15 | Pre-commit idempotency cache hit                       | Run `torrust-git-hooks pre-commit` (pass); run again without changing staged files | Second run exits 0 in under 1 second; output indicates cache hit                                                                    | TODO   |          |
| M16 | Pre-push idempotency cache hit                         | Run `torrust-git-hooks pre-push` (pass); retry the push for the same commits       | Second run exits 0 immediately; output indicates cache hit                                                                          | TODO   |          |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |
| AC9   | TODO                   |          |
| AC10  | TODO                   |          |
| AC11  | TODO                   |          |
| AC12  | TODO                   |          |
| AC13  | TODO                   |          |
| AC14  | TODO                   |          |
| AC15  | TODO                   |          |
| AC16  | TODO                   |          |
| AC17  | TODO                   |          |
| AC18  | TODO                   |          |
| AC19  | TODO                   |          |
| AC20  | TODO                   |          |
| AC21  | TODO                   |          |

## Risks and Trade-offs

- **Global CLI output contract compliance**: the ADR (`docs/adrs/20260519000000_define_global_cli_output_contract.md`)
  mandates that new binaries use JSON-only output. The NDJSON progress event model (T3/T8)
  satisfies both this requirement and the real-time feedback goal: each event line is valid JSON
  and is flushed immediately. The `profiling` binary is explicitly excluded from the ADR as a
  developer-only tool; the git hooks runner may qualify for the same exemption — this must be
  settled in T1 to avoid retrofitting output design mid-implementation.
- **Heartbeat must be distinguishable from step output**: agents and scripts that consume NDJSON
  must filter by `kind` to separate heartbeat events from step-end results. The schema (T3) must
  define all `kind` values before implementation so consumers can be written unambiguously.
- **Existing JSON consumers**: the `.githooks/` dispatchers and any agent configuration that
  currently parses the script's JSON blob will need updating. There is no guaranteed schema
  backward-compatibility; the new NDJSON streaming model is a deliberate break. All consumers
  are within this repository and can be migrated as part of T11–T15.
- **Binary not built on first clone**: unlike a shell script, the Rust binary must be compiled
  before the hooks work. The `.githooks/` dispatchers must detect a missing binary and emit a
  helpful message (e.g., "run `cargo build -p torrust-git-hooks` first"). Alternatively,
  `install-git-hooks.sh` (or its replacement `install-hooks` subcommand) can trigger a build
  as part of setup. This trade-off must be decided during T1/T8.
- **CI setup step**: `copilot-setup-steps.yml` currently caches and references the Bash scripts
  directly. With a binary, the setup step must build the crate before installing hooks. This
  adds to CI setup time.
- **Cross-platform compatibility**: the Bash scripts rely on `bash`, `sed`, `mktemp`, and
  `date` — all POSIX-ish. The Rust binary will be more portable but must handle Windows paths
  and permissions correctly for the `install-hooks` subcommand if Windows support is desired.
  For now, Linux/macOS parity is sufficient.
- **Shared step-runner duplication in JSON schema**: the existing JSON schema is undocumented.
  During T3–T5, the schema should be explicitly documented so AC5 is unambiguously verifiable.
- **Smart step selection — file-pattern to step-subset mapping**: the mapping between file
  patterns and the steps they require must be maintained in code. If a new lint step is added
  (e.g., a YAML linter), the pattern mapping must be updated or the new step will be silently
  skipped on documentation-only commits. A test that enumerates all steps and asserts each has
  an explicit pattern classification mitigates this risk.
- **Pre-commit cache invalidation**: the cache key includes both the staged tree SHA and a hash
  of the active step configuration. A binary upgrade or step list change will therefore
  automatically invalidate all cached records. However, a developer who manually edits a step
  configuration without updating the hash derivation could get false cache hits. The step-config
  hash should be derived from a canonical serialisation of the steps, not a hand-maintained
  constant.
- **Pre-push cache storage in `.git/`**: `.git/torrust-hooks/` is not committed and is not
  shared between clones. A fresh clone has an empty cache, so the first push always runs the
  full suite. This is the correct and safe default; no cross-machine cache sharing is intended.
- **Cache and smart-skip interact**: if the staged tree SHA matches a cache record, the hook
  exits early before file-type analysis. Ensure the cache record stores which step subset was
  actually run (full or markdown-only) so a cached markdown-only result is not accepted as a
  substitute for a full-suite result when Rust files are subsequently staged.
- **ADR §8 — workspace lint guards**: once the repository-wide output contract migration is
  complete, `clippy::print_stdout` and `clippy::print_stderr` will be denied at workspace level.
  The new crate must use a structured NDJSON writer rather than `print!`, `println!`, `eprint!`,
  or `eprintln!` calls from the outset, to avoid future lint failures without needing a rewrite.
- **ADR §9 — AI agent output capture**: when an AI agent drives the binary, it should redirect
  output to `.tmp/<command>.stdout` and `.tmp/<command>.stderr` (workspace-local, git-ignored)
  to preserve the stdout/stderr channel split. Since the binary is `no-stdout-result`, the
  stdout file will always be empty; all NDJSON progress events will be in the stderr file.

## References

- Affected scripts:
  - [`contrib/dev-tools/git/hooks/pre-commit.sh`](../../../contrib/dev-tools/git/hooks/pre-commit.sh)
  - [`contrib/dev-tools/git/hooks/pre-push.sh`](../../../contrib/dev-tools/git/hooks/pre-push.sh)
  - [`contrib/dev-tools/git/install-git-hooks.sh`](../../../contrib/dev-tools/git/install-git-hooks.sh)
- Dispatcher scripts: [`.githooks/pre-commit`](../../../.githooks/pre-commit), [`.githooks/pre-push`](../../../.githooks/pre-push)
- CI: [`.github/workflows/copilot-setup-steps.yml`](../../../.github/workflows/copilot-setup-steps.yml)
- Engineering policy: `AGENTS.md` § Engineering Policies, rule #3
- Related closed issue: `docs/issues/closed/1780-refactor-pre-push-checks-performance-and-verbosity.md`
- Related closed issue: `docs/issues/closed/1769-refactor-pre-commit-checks-performance-and-verbosity.md`
- Global CLI output contract ADR: [`docs/adrs/20260519000000_define_global_cli_output_contract.md`](../../../docs/adrs/20260519000000_define_global_cli_output_contract.md)
