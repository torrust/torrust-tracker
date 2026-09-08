---
doc-type: issue
issue-type: feature
status: planned
priority: p2
epic: null
github-issue: 2151
spec-path: docs/issues/open/2151-add-tracker-config-path-argument/ISSUE.md
branch: "2151-add-tracker-config-path-argument"
related-pr: 2153
last-updated-utc: 2026-09-08 12:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2151-add-tracker-config-path-argument/rust-executable-test-plan.md
    - src/main.rs
    - src/app.rs
    - src/bootstrap/app.rs
    - src/bootstrap/config.rs
    - packages/configuration/src/lib.rs
    - packages/configuration/src/v3_0_0/mod.rs
    - tests/AGENTS.md
    - tests/lifecycle/native_tracker.rs
    - README.md
    - docs/benchmarking.md
    - docs/containers.md
    - docs/profiling.md
    - Containerfile
    - docs/issues/closed/1978-configuration-overhaul-epic/EPIC.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2151 - Add a Configuration-File Path Argument to the Tracker Executable

## Goal

Allow an operator to start `torrust-tracker` with an explicit configuration-file
path, without relying on a process environment variable:

```text
torrust-tracker --config-toml-path /etc/torrust/tracker/tracker.toml
```

The option must fit the existing configuration model predictably, preserve
environment-based invocations, and make native child-process configuration
explicit.

## Background

The main tracker executable does not currently parse command-line arguments.
`src/main.rs` calls `app::start()`, which initializes configuration through
`src/bootstrap/config.rs`. `torrust_tracker_configuration::Info` discovers a
complete TOML environment source or a path environment source before loading
the selected TOML source.

This works for shell, service-manager, and container deployments, but it makes
direct executable invocations less explicit. In-process tests cannot safely
mutate a process-global configuration environment variable concurrently. Native
child-process tests avoid that problem by setting
`TORRUST_TRACKER_CONFIG_TOML_PATH` on each spawned `Command`, which supplies an
independent environment snapshot. A command-line argument would make the source
visible in the child invocation and remove that avoidable environment dependency
at the executable boundary.

This issue selects only a configuration **file**. It does not introduce
command-line configuration for every setting or for a complete TOML document.
It is independent of the closed configuration-overhaul EPIC #1978: that EPIC
changed the configuration schema, whereas this issue changes executable
bootstrap source selection. The EPIC remains related context only.

## Current Configuration Injection Analysis

### Sources and current priority

The active v3 configuration implementation has these source classes, from
highest to lowest priority:

1. `TORRUST_TRACKER_CONFIG_OVERRIDE_*` per-value overrides, merged onto the
   selected base source.
2. Complete TOML from `TORRUST_TRACKER_CONFIG_TOML`, when set. It selects the
   base source and causes a path environment source to be ignored.
3. A TOML file selected by `TORRUST_TRACKER_CONFIG_TOML_PATH`, when the complete
   TOML variable is absent.
4. Bootstrap's default development file,
   `./share/default/config/tracker.development.sqlite3.toml`, when neither
   environment base source is supplied.
5. Rust schema defaults, which fill non-mandatory values after mandatory-value
   validation.

The container entrypoint may prepare its packaged file before startup. That is
deployment orchestration, not a configuration source read by the executable.

Before Rust defaults are joined, loading requires these explicit values from the
selected base TOML source or matching per-value overrides:

- `metadata.schema_version`
- `logging.trace_filter`
- `core.private`
- `core.listed`

Schema-version validation occurs after source merging and default filling.
Bootstrap then performs semantic and persistence-requirement validation before
composing services.

### File-source resolution caveats

`Configuration::load` reads the selected file with `figment::providers::Toml::file`,
which has two properties that matter for an explicit CLI path:

- A missing file is **not** an error. The provider yields an empty source, so
  loading fails later with `MissingMandatoryOption { path: "metadata.schema_version" }`
  rather than a path-specific diagnostic. This is what
  `it_should_return_a_typed_load_error_when_the_configured_source_file_is_missing`
  in `src/bootstrap/config.rs` currently observes.
- A **relative** path is searched in the current working directory and then in
  every parent directory up to the filesystem root. `-c tracker.toml` could load
  an unrelated file from a parent directory.

Both behaviors are acceptable-by-accident for the environment variable today but
are surprising for an explicit argument. See the decisions below.

## Proposed Source Precedence

Two kinds of source exist and must not be confused:

- **Base sources** are mutually exclusive. Exactly one is selected; the others
  are ignored entirely (not merged).
- **Per-value overrides** (`TORRUST_TRACKER_CONFIG_OVERRIDE_*`) are always
  merged on top of the selected base source, regardless of how it was selected.

Effective order after this change, highest priority first:

1. `TORRUST_TRACKER_CONFIG_OVERRIDE_<PATH>` per-value overrides (merged).
2. `--config-toml-path <PATH>` / `-c <PATH>` — **new**; selects the base file.
3. `TORRUST_TRACKER_CONFIG_TOML` — complete TOML content; selects the base.
4. `TORRUST_TRACKER_CONFIG_TOML_PATH` — selects the base file.
5. Bootstrap default development file.
6. Rust schema defaults for non-mandatory values (joined last).

Only item 2 is new. Items 1 and 3-6 keep their current relative order, so any
invocation that does not pass the argument behaves exactly as today.

Base-source selection table (`set` = supplied, `-` = absent):

| `--config-toml-path` | `CONFIG_TOML` | `CONFIG_TOML_PATH` | Base source used | Changed by this issue |
| -------------------- | ------------- | ------------------ | ---------------- | --------------------- |
| set                  | set           | set                | CLI file         | new                   |
| set                  | set           | -                  | CLI file         | new                   |
| set                  | -             | set                | CLI file         | new                   |
| set                  | -             | -                  | CLI file         | new                   |
| -                    | set           | set                | env TOML content | no                    |
| -                    | set           | -                  | env TOML content | no                    |
| -                    | -             | set                | env path file    | no                    |
| -                    | -             | -                  | default dev file | no                    |

In every row, `TORRUST_TRACKER_CONFIG_OVERRIDE_*` values still replace matching
keys in the selected base. Environment variable names in the table omit the
`TORRUST_TRACKER_` prefix for width.

Rationale for the argument outranking both environment base sources:

- An argument is the most explicit and most local expression of intent; it is
  visible in the invocation, whereas environment state is inherited and often
  invisible (container `ENV`, service-manager unit, parent shell).
- If the argument lost to `TORRUST_TRACKER_CONFIG_TOML`, a container image that
  sets that variable would silently ignore an operator's explicit `-c`, which is
  the failure mode this issue exists to remove.
- Overrides stay on top because their purpose is deployment-time injection of
  secrets and per-environment values that must apply to _any_ base file. An
  explicit `-c` says "use this file", not "ignore my secrets".

Examples:

```text
# CLI file wins over both environment base sources; override still applies.
TORRUST_TRACKER_CONFIG_TOML="$(cat a.toml)" \
TORRUST_TRACKER_CONFIG_TOML_PATH=b.toml \
TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_API__ACCESS_TOKENS__ADMIN=secret \
  torrust-tracker --config-toml-path /etc/torrust/tracker/tracker.toml
# → base: /etc/torrust/tracker/tracker.toml; admin token: "secret"

# No argument: current behavior, env TOML content beats env path.
TORRUST_TRACKER_CONFIG_TOML="$(cat a.toml)" \
TORRUST_TRACKER_CONFIG_TOML_PATH=b.toml \
  torrust-tracker
# → base: content of a.toml (b.toml ignored)
```

## Scope

### In Scope

- Add `-c` / `--config-toml-path <PATH>` to the main tracker executable.
- Implement the precedence defined in "Proposed Source Precedence": the CLI
  path outranks both environment base sources; per-value overrides still merge
  on top.
- Resolve the CLI path exactly (no parent-directory search) and fail with a
  path-specific error when it is missing or unreadable.
- Keep per-value overrides highest priority and preserve all current
  environment-based interfaces when the option is absent.
- Pass source selection as typed application/bootstrap/configuration input;
  do not mutate process environment variables to emulate the option.
- Add unit, executable-level integration, and manual coverage for precedence,
  default behavior, and diagnostics.
- Keep all tracked repository test code in Rust. The release CLI Python harness
  is temporary evidence only and must be removed after the approved
  `rust-executable-test-plan.md` preserves its behavior in Rust tests.
- Review the affected operational and native-test documentation.

### Out of Scope

- A full-TOML command-line argument or flags for individual configuration
  settings.
- Removing or deprecating `TORRUST_TRACKER_CONFIG_TOML`,
  `TORRUST_TRACKER_CONFIG_TOML_PATH`, or `TORRUST_TRACKER_CONFIG_OVERRIDE_*`.
- Changing the configuration schema, mandatory fields, Rust defaults, semantic
  validation, container entrypoint behavior, or service-manager integration.
  The `Containerfile` is reviewed only for documentation comments; its
  `ENV TORRUST_TRACKER_CONFIG_TOML_PATH` default stays and is legitimately
  outranked by a CLI argument passed through `entry.sh`'s `exec ... "$@"`.
- Replacing environment configuration in existing in-process tests.
- Migrating `torrust-tracker` to full compliance with the global CLI output
  contract (tracked separately in
  `docs/issues/drafts/cli-output-contract-migration.md`).

## Architectural Decisions

- The CLI option owns base-source selection, not per-value configuration. TOML
  remains the canonical configuration representation.
- An explicit CLI path wins over both existing base-source environment inputs
  (`TORRUST_TRACKER_CONFIG_TOML` and `TORRUST_TRACKER_CONFIG_TOML_PATH`); base
  sources are exclusive, never merged. `TORRUST_TRACKER_CONFIG_OVERRIDE_*`
  remains the highest priority so deployment secret injection continues to
  work. The full table is in "Proposed Source Precedence".
- The executable must pass an explicit typed path through startup layers rather
  than write an environment variable.
- Preserve `app::start()` as the environment-based compatibility wrapper;
  introduce a parameterized entry point where required. Existing callers
  (`src/console/profiling.rs`, `tests/common/workspace.rs`) are unchanged.
- Do **not** bind the argument to an environment variable with clap's `env`
  attribute (unlike `src/console/ci/e2e/runner.rs`). Doing so would rank
  `TORRUST_TRACKER_CONFIG_TOML_PATH` above `TORRUST_TRACKER_CONFIG_TOML`,
  silently inverting the current contract, and would move environment reading
  into the parser. Environment discovery stays in one place.
- The CLI path is an OS path, not a `String`. Carry it as `PathBuf` (or
  `camino::Utf8PathBuf`, already a dependency of the configuration crate) and
  decide how a non-UTF-8 path is reported.
- An absent option value is a CLI usage error: report that `--config-toml-path`
  requires a non-empty path and exit `2`. A supplied empty path is invalid and
  receives the same treatment; it must not fall back to an environment or
  default source.
- A supplied path must be resolved relative to the current working directory
  only (`Toml::file_exact` or an explicit exact-file check), never through
  parent-directory search. A missing, unreadable, non-file, or TOML-invalid
  CLI source is a startup error: report the supplied path and relevant failure,
  exit `1`, and start no listener.
- Keep `TORRUST_TRACKER_CONFIG_TOML_PATH` semantics unchanged: it continues to
  use the current parent-directory search and missing-file behavior. This is an
  intentional compatibility distinction. The existing startup log naming the
  selected environment file makes that behavior observable; documentation must
  not claim that environment paths are exact.
- Help text, usage errors, and startup failures must respect the global CLI
  output contract as far as this issue touches them: usage errors exit `2`,
  runtime failures exit `1`, and no new plain-text output is added on stdout.
  Existing non-compliance (`report_startup_failure` plain text on stderr) is
  not fixed here.
- Related ADRs:
  `docs/adrs/20260519000000_define_global_cli_output_contract.md`
  (`torrust-tracker` is classed `no-stdout-result`).
- ADRs to create: Assess the final source-priority contract after the first
  vertical slice. Create a root ADR in `docs/adrs/` if maintainers judge the
  cross-package executable/configuration contract to have lasting architectural
  consequences.

## Design and Ownership Review

This work changes child-process configuration and reusable test-fixture
behavior. The implementation must establish the following before and during the
first vertical slice:

| Collaborator           | Responsibility                                                                                                                                                |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Binary CLI parser      | Parse and reject invalid executable arguments; own no configuration loading and no environment reading.                                                       |
| Application/bootstrap  | Carry the optional selected path to configuration initialization without global side effects.                                                                 |
| Configuration package  | Select and load a base source, verify the selected file exists and is readable, merge per-value overrides, enforce mandatory values, and return typed errors. |
| Native tracker fixture | Own one temporary workspace, child process, output drains, absolute readiness deadline, and child reaping.                                                    |

- Each spawned child owns its configuration path, workspace, listeners, and
  output capture. It must never read or overwrite another child's source.
- Normal shutdown waits for the child; failure and drop paths kill and reap it
  before the temporary workspace is released.
- Every awaited readiness operation continues to be bounded by the fixture's
  absolute `STARTUP_DEADLINE`; adding CLI configuration must not introduce an
  unbounded wait.
- After the first passing CLI-only child-process vertical slice, stop for a
  design review of source ownership, cleanup behavior, deadline coverage, and
  any needed ADR before adding broader scenarios.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status      | Task                                                      | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| --- | ----------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE        | Establish baseline source behavior                        | Added `figment::Jail` tests for all four existing no-CLI base-source rows, a path-source override, missing-file mandatory-option result, and parent-directory search. `cargo test --package torrust-tracker-configuration` passed (118 tests).                                                                                                                                                                                                                |
| T2  | DONE        | Introduce typed source selection                          | Added `Info::new_with_explicit_config_toml_path(..., Option<PathBuf>)`. Explicit paths are eagerly read and loaded from retained contents, take precedence over environment base sources, preserve their exact `PathBuf` in diagnostics, and never use parent lookup. Legacy environment and default file behavior remains on `Toml::file`. `cargo test --package torrust-tracker-configuration --lib` passed (127 tests); Rust formatting and Clippy passed. |
| T3  | DONE        | Define the CLI boundary                                   | Added a main-owned `clap` parser for `-c` / `--config-toml-path` with no `env` binding. Parser tests cover short/long forms, missing/empty values, unknown arguments, help, exit codes, and absent environment binding. `cargo test --package torrust-tracker --bin torrust-tracker` passed (7 tests).                                                                                                                                                        |
| T4  | DONE        | Wire startup and precedence                               | Threaded `Option<PathBuf>` from `main` through `app::start_with_explicit_config_toml_path`, `bootstrap::app::setup`, and `initialize_configuration` into T2 without environment mutation. Direct bootstrap tests cover every CLI-present table row: CLI only, CLI plus full TOML, CLI plus path, and CLI plus both sources. Root library tests passed (84 tests).                                                                                             |
| T5  | DONE        | Review first vertical slice                               | Review completed after the parser-to-bootstrap vertical slice passed. Ownership is coherent: parsing is binary-only; configuration loading remains in the configuration package; no new async resource or readiness wait was introduced. Existing native-fixture lifetime/deadline invariants are unchanged. No ADR is required now; reconsider only if a lasting wider source-selection policy emerges.                                                      |
| T6  | DONE        | Preserve overrides and defaults                           | Existing explicit-file coverage proves a per-value override wins. Added table-driven tests that each mandatory field still fails before Rust defaults, and that an explicit file containing only mandatory fields receives the unchanged optional defaults. `cargo test --package torrust-tracker-configuration --lib` passed (129 tests); Rust formatting and Clippy passed.                                                                                 |
| T7  | DONE        | Add executable-boundary coverage                          | Native fixtures now pass `--config-toml-path`, remove both inherited base-source variables, and retain per-child CLI-path/storage identities. The lifecycle target starts two children concurrently, verifies distinct PIDs, health addresses, CLI paths, and storage paths, then sends SIGTERM and reaps both. `cargo test --test lifecycle-signals` passed (8 tests); tracker tests, Rust formatting, and Clippy passed.                                    |
| T8  | DONE        | Update documentation                                      | Updated the README, configuration crate/root API docs, container, benchmarking, profiling, source/test guidance, and local-run skill. CLI selection is primary for the main binary; environment examples remain valid. Documentation states final precedence, strict CLI-path behavior, profiling's environment-only boundary, and native fixture isolation. Skill-link validation, Markdown lint, spell checking, and diff checks passed.                    |
| T9  | DONE        | Validate and record evidence                              | The mandatory pre-commit gate, configuration (129), tracker, and lifecycle-signals (8) tests passed. Scripted M1-M5 release-binary scenarios (disposable Python verifier) passed with `.tmp/issue-2151-manual/` evidence, including unreadable-file and no-listener checks; this is not human-oriented manual verification (see T10). Acceptance criteria were independently reviewed and all passed. No separate retrospective was warranted.                |
| T10 | DONE        | Complete Rust executable coverage and manual verification | The approved `rust-executable-test-plan.md` preserved the release-verifier behavior in Rust executable-boundary tests, documented the Rust-only tracked test-code policy, and removed the Python harness. Real release-style M1-M5 verification is recorded in `manual-verification-evidence.md`; final focused tests, Clippy, `linter all`, and the pre-commit gate passed.                                                        |

Each task must be independently buildable and tested. T1 is a
behavior-preserving safety-net change; T2 is a configuration refactor; T3-T4
are the deployable feature; later tasks extend verification and documentation.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted and moved to `docs/issues/open/2151-add-tracker-config-path-argument/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue [#2151](https://github.com/torrust/torrust-tracker/issues/2151) created and issue number added to this spec
- [x] Spec-only PR [#2153](https://github.com/torrust/torrust-tracker/pull/2153) merged into `develop` before implementation
- [x] First passing CLI-only vertical slice reviewed for ownership, cleanup, deadline, and ADR decisions
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: progress log states why no retrospective was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-08 17:34 UTC - GitHub Copilot - Completed real release-artifact
  manual verification. V1-V4 recorded direct CLI-only, precedence, override,
  and invalid-source runs; V5 recorded two concurrently running isolated
  trackers, distinct health bindings, successful health responses, and clean
  SIGTERM shutdown. Evidence is retained in
  `manual-verification-evidence.md`; final automated validation and acceptance
  review remain pending under T10/R9.

- 2026-09-08 17:39 UTC - GitHub Copilot - Final validation passed: 129
  configuration-package tests, 18 `cli-configuration` executable tests, and
  13 `lifecycle-signals` tests; focused Clippy, `linter all`, and the required
  pre-commit gate also passed. Re-reviewed M1-M5 evidence and all acceptance
  criteria. No retrospective is warranted because the implementation and
  verification-policy discoveries are recorded in the issue plans and
  repository guidance.

- 2026-09-02 16:40 UTC - GitHub Copilot - Drafted the source-selection analysis locally; no tracked file, GitHub issue, or branch was created.
- 2026-09-07 08:55 UTC - GitHub Copilot - Reviewed the local draft against the issue-spec workflow and copied it to this folder-style draft; corrected metadata and added ownership, deadline, vertical-slice, and completion-review requirements.
- 2026-09-07 09:40 UTC - GitHub Copilot - Consistency review: documented `Toml::file` missing-file and parent-directory-search behavior (blocks AC5 as written), linked the global CLI output contract ADR, forbade clap `env` binding, specified the path type, and tightened M2/T7.
- 2026-09-07 09:45 UTC - Maintainer - Kept this as an independent bootstrap issue related to closed EPIC #1978, not an EPIC subissue. Decided that the CLI argument rejects an empty value as usage error and uses strict exact-file loading with descriptive startup errors; retained the environment-path compatibility behavior.
- 2026-09-07 10:05 UTC - Maintainer / GitHub Copilot - Confirmed precedence: argument > `TORRUST_TRACKER_CONFIG_TOML` > `TORRUST_TRACKER_CONFIG_TOML_PATH` > default; overrides always merge on top. Added a dedicated "Proposed Source Precedence" section with the eight-row selection table and tied T1/T4/AC2/AC3 to it.
- 2026-09-07 10:10 UTC - GitHub Copilot - Created GitHub issue [#2151](https://github.com/torrust/torrust-tracker/issues/2151) after maintainer approval. This spec will move to `docs/issues/open/2151-add-tracker-config-path-argument/`.
- 2026-09-07 10:30 UTC - GitHub Copilot - Moved the approved specification to this open-issue folder and prepared the spec-only delivery branch. Source-level issue markers are deferred to the implementation branch so the specification PR changes only `docs/issues/`.
- 2026-09-07 11:21 UTC - GitHub - Merged spec-only PR [#2153](https://github.com/torrust/torrust-tracker/pull/2153) into `develop` (merge commit `796c8157`).
- 2026-09-07 11:25 UTC - GitHub Copilot - Created implementation branch `2151-add-tracker-config-path-argument` from the merged `torrust/develop` baseline. Started T1 baseline source-behavior analysis.
- 2026-09-07 11:45 UTC - GitHub Copilot - Completed T1. Added deterministic `Info::new` to `Configuration::load` regression tests using `figment::Jail` with cleared environment state. Verified complete-TOML, path, and default base-source selection; path-source override precedence; the missing-file `MissingMandatoryOption` result; and relative environment-path parent search. `cargo test --package torrust-tracker-configuration` passed (118 tests); `cargo fmt --check` and `git diff --check` passed.
- 2026-09-07 12:10 UTC - GitHub Copilot - Completed T2. Added typed `PathBuf` source selection through `Info::new_with_explicit_config_toml_path`, retaining explicit source identity and eagerly captured TOML contents. Both v2 and v3 loaders use the captured contents for explicit files while retaining legacy file-provider behavior for environment/default sources. Added precedence, override, missing-file, directory, exact-relative-path, malformed-content redaction, non-UTF-8-path, and captured-content regression coverage. `cargo test --package torrust-tracker-configuration --lib` passed (127 tests); `linter rustfmt`, `linter clippy`, and `git diff --check` passed.
- 2026-09-07 15:05 UTC - GitHub Copilot - Completed T3 and the first T4 vertical slice. The binary parses an optional `PathBuf` with clap and passes it through the application and bootstrap boundaries to the typed configuration API, with no environment mutation. Parser, root-library, and configuration tests passed (7, 82, and 127 tests respectively), along with `cargo check`, Rust formatting, Clippy, and diff checks. T5 review found no new child-process ownership, cleanup, or deadline concern and no ADR need. T4 remains in progress until direct tests cover every CLI-present precedence-table row.
- 2026-09-07 15:15 UTC - GitHub Copilot - Completed T4 by adding the remaining direct bootstrap precedence tests for CLI plus full-TOML environment source and CLI plus path environment source. Together with the earlier CLI-only and CLI-plus-both-sources tests, all four CLI-present table rows are covered. Root library (84 tests), parser binary (7 tests), and configuration library (127 tests) passed; Rust formatting, Clippy, and diff checks passed.
- 2026-09-07 15:20 UTC - GitHub Copilot - Completed T6. The existing explicit-file override test remains the regression for per-value override precedence. Added table-driven explicit-file coverage proving each mandatory option is still required before defaults are joined, plus a minimal explicit-file test proving optional values receive the unchanged Rust defaults. `cargo test --package torrust-tracker-configuration --lib` passed (129 tests); `linter rustfmt`, `linter clippy`, and `git diff --check` passed.
- 2026-09-07 15:40 UTC - GitHub Copilot - Completed T7. Native executable fixtures now pass the isolated configuration through `--config-toml-path` and explicitly remove both inherited base-source environment variables. A concurrent child-process scenario starts two port-zero trackers, waits within each fixture deadline, asserts distinct PIDs, health-check addresses, CLI paths, and workspace-local storage paths, then sends SIGTERM and reaps both children. `cargo test --test lifecycle-signals` passed (8 tests); `cargo test --package torrust-tracker`, Rust formatting, Clippy, and diff checks passed.
- 2026-09-07 15:55 UTC - GitHub Copilot - Completed T8. Updated the README, container and benchmarking commands, profiling clarification, root/configuration API docs, source and test guidance, and `run-tracker-locally` skill. The CLI path is documented as the main-binary primary source; environment examples retain their compatible behavior. `validate-skill-links.sh`, `linter markdown`, `linter cspell`, and `git diff --check` passed.
- 2026-09-07 16:30 UTC - GitHub Copilot - Completed T9 verification. The pre-commit gate passed; configuration tests passed (129), tracker tests passed, and lifecycle-signals passed (8). The release binary was built and manual scenarios M1-M5 passed with logs under `.tmp/issue-2151-manual/`. The first verifier run used a 10-second shutdown wait and force-killed an otherwise healthy M1 child; it was corrected to use the fixture-aligned 30-second deadline, then all scenarios passed. No implementation deviation or reusable design discovery warrants a separate retrospective.
- 2026-09-07 16:50 UTC - Task Reviewer / GitHub Copilot - Independent acceptance review initially found M4 had no unreadable regular-file scenario, M1 incorrectly named a debug binary, T9 remained TODO, and CLI-source remediation text named only environment sources. Added a mode-`000` unreadable regular-file scenario and retained its command, file mode, exit status, and `Permission denied` result in `.tmp/issue-2151-manual/summary.txt`; added no-listener bind probes for malformed and parent-only relative sources; corrected M1 evidence; marked T9 done; and updated the guidance with a regression test. The release-binary manual suite and focused tracker tests passed after correction; all acceptance criteria now pass.
- 2026-09-07 17:00 UTC - Maintainer / GitHub Copilot - Preserved the reusable release-binary CLI verifier as `release-cli-verification.py` in this issue directory. It creates only ignored runtime configurations and logs beneath `.tmp/issue-2151-manual/`, keeping source and reproducible verification procedure together without tracking transient evidence.
- 2026-09-07 17:25 UTC - Maintainer / GitHub Copilot - Reclassified the tracked Python verifier as temporary evidence: repository test code must be Rust. Added `rust-executable-test-plan.md` for maintainer review before implementation. T10 will preserve appropriate executable behavior in Rust tests, document the policy, remove the Python harness, and repeat completion validation.
- 2026-09-08 09:00 UTC - Maintainer / GitHub Copilot - Refined the pending Rust test plan: move the reusable native child-process fixture from `tests/lifecycle/` to `tests/common/`; create `tests/configuration/cli_configuration.rs` for executable configuration contracts; retain `tests/lifecycle/signals.rs` for OS-signal contracts only. The shared fixture must offer narrowly configured child commands without duplicating process lifecycle ownership.
- 2026-09-08 09:15 UTC - GitHub Copilot - Completed R1. Moved the native child-process fixture to `tests/common/native_tracker.rs` and updated `tests/lifecycle/signals.rs` to import it through an explicit path module declaration. The signal suite passed unchanged (8 tests).
- 2026-09-08 09:35 UTC - GitHub Copilot - Created a provisional `cli-configuration` target and validated its shared-fixture import. Review found its sole scenario duplicated SIGTERM lifecycle coverage without asserting a configuration contract, so the uncommitted target was removed. R2 remains pending and must begin with a configuration-specific executable-boundary scenario.
- 2026-09-08 10:00 UTC - GitHub Copilot - Completed R2 and R3. Added the `cli-configuration` target with executable assertions that a CLI file wins over both child-only environment base sources and that a child-only per-value health-check override wins over the CLI file. The shared fixture owns the narrow child-only override configuration and continues to remove inherited source values by default. `cargo test --test cli-configuration` passed (7 tests); `cargo test --test lifecycle-signals` passed (9 tests); Rust formatting, Clippy, and diff checks passed.
- 2026-09-08 10:15 UTC - Maintainer / GitHub Copilot - Refined the R3 test bodies so Arrange defines source state, Act starts the child and waits for readiness, Teardown reaps it before assertions, and Assert compares a named observed endpoint with a named expected endpoint. Added R3a to prove at configuration-package level that ignored base sources are exclusive, not merely overridden on a conflicting key.
- 2026-09-08 10:30 UTC - Maintainer / GitHub Copilot - Grouped executable configuration scenarios by their contracts: `base_source_precedence` and `per_value_overrides`. The Cargo test entry point retains only target-level configuration and the shared-fixture import; `invalid_sources` remains the planned next module.
- 2026-09-08 11:00 UTC - GitHub Copilot - Completed R3a and R4. R3a proves ignored base sources are not merged at the configuration-package layer. R4 adds compiled-child invalid-source contracts for parser errors and explicit-source failures, with fixture-owned workspaces, child-only environment isolation, deadline-bounded wait/reap/output handling, and post-reap candidate-port probes. Independent review initially found expected-failure drop and timeout cleanup gaps; they were corrected and the re-review approved the lifecycle. Focused configuration (129), CLI configuration (15), and lifecycle (10) tests passed.
- 2026-09-08 12:00 UTC - GitHub Copilot - Completed R5. Added the Unix unreadable regular-file executable contract. The fixture creates a valid mode-`000` source and probes effective permission enforcement before spawning: normal users exercise a path-bearing `Permission denied` exit, while privileged runners explicitly skip without starting a child. Independent review found restoration could panic or be lost before workspace cleanup; restoration is now fallible in normal cleanup, non-panicking and synchronous in drop cleanup, and covered by regressions. CLI configuration (17) and lifecycle (11) tests passed; re-review approved the cleanup design.
- 2026-09-08 17:00 UTC - Maintainer / GitHub Copilot - Approved and implemented repository guidance distinguishing Rust automatic tests, real human-oriented manual verification, and disposable issue-local verification scripts. Added the manual-evidence template and reset M1-M5: prior Python-script output is historical disposable-script evidence, not manual verification. T10 now requires real release-binary runs with actual commands, output, and tracker logs recorded in `manual-verification-evidence.md`.
- 2026-09-08 17:10 UTC - GitHub Copilot - Completed R7: removed the disposable Python verifier after its durable behavior checks were preserved in Rust executable-boundary tests. Real release-binary manual verification for M1-M5 remains pending in `manual-verification-evidence.md`.

## Acceptance Criteria

- [x] AC1: `torrust-tracker` accepts `-c` and `--config-toml-path <PATH>`.
- [x] AC2: Every row of the base-source selection table in "Proposed Source
      Precedence" is covered by a test and behaves as specified; in particular
      a CLI path selects its file even when both `TORRUST_TRACKER_CONFIG_TOML`
      and `TORRUST_TRACKER_CONFIG_TOML_PATH` are set, and the ignored base
      sources are not merged.
- [x] AC3: `TORRUST_TRACKER_CONFIG_OVERRIDE_*` values still override matching
      values in a CLI-selected file; without the option, the four unchanged
      rows (env TOML content beats env path beats default) behave exactly as
      before.
- [x] AC4: Required values remain mandatory before Rust defaults are applied;
      optional defaults remain unchanged.
- [x] AC5: An absent or supplied empty CLI value is a descriptive usage error
      with exit code `2` and no listener. A missing, unreadable, non-file, or
      TOML-invalid CLI path produces an error naming the offending path with
      exit code `1` and no listener. A relative CLI path is never resolved
      through parent-directory search.
- [x] AC6: Two tracker child processes can run concurrently with distinct CLI
      paths, isolated storage, and port-zero bindings without configuration-source
      environment variables.
- [x] `linter all` exits with code `0` and relevant tests pass.
- [x] Manual verification scenarios are executed and documented (status + evidence).
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation states final interfaces and precedence without contradicting implementation.

## Verification Plan

### Automatic Checks

- Configuration-package tests for every source-selection branch and precedence pair.
- Root bootstrap tests proving the executable path is passed explicitly without
  process-environment mutation.
- Direct executable tests for CLI-only configuration and concurrent child isolation.
- `cargo test --package torrust-tracker-configuration`
- `cargo test --package torrust-tracker`
- `cargo test --test lifecycle-signals` when its fixture is updated.
- `linter all`
- Required pre-commit and pre-push checks before publishing implementation.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                        | Command/Steps                                                                                                                                                                                                                                                       | Expected Result                                                                                                                                                                        | Status | Evidence                                |
| --- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------- |
| M1  | CLI path only                   | Start the release binary with `--config-toml-path` pointing to an isolated valid file and no configuration-source variables.                                                                                                                                        | The tracker reads that file, starts configured services, and exits cleanly on SIGTERM.                                                                                                 | DONE   | `manual-verification-evidence.md` (V1). |
| M2  | CLI source precedence           | Prepare three valid configurations that differ only in `health_check_api.bind_address` (three distinct fixed loopback ports). Supply one via `TORRUST_TRACKER_CONFIG_TOML`, one via `TORRUST_TRACKER_CONFIG_TOML_PATH`, and the third via `--config-toml-path`.     | The `HEALTH CHECK API: Started on:` log line reports the port from the CLI-selected file.                                                                                              | DONE   | `manual-verification-evidence.md` (V2). |
| M3  | Per-value override              | Start with `--config-toml-path` and a distinguishable `TORRUST_TRACKER_CONFIG_OVERRIDE_*` value.                                                                                                                                                                    | The override wins for its path while other values come from the file.                                                                                                                  | DONE   | `manual-verification-evidence.md` (V3). |
| M4  | Invalid CLI source              | Start with (a) `--config-toml-path` with no value, (b) an empty supplied path, (c) a nonexistent absolute file, (d) a directory, (e) an unreadable regular file, (f) malformed TOML, and (g) a relative filename that exists only in a parent directory of the CWD. | Cases (a-b) exit `2` with a descriptive usage error. Cases (c-g) exit `1` with an error naming the path; case (g) must not load the parent-directory file. No case creates a listener. | DONE   | `manual-verification-evidence.md` (V4). |
| M5  | Parallel child isolation (Unix) | Launch two binaries concurrently with different CLI paths, isolated storage, and port-zero configuration.                                                                                                                                                           | Both start with their own configuration; neither reads or overwrites the other's source.                                                                                               | DONE   | `manual-verification-evidence.md` (V5). |

Manual verification is mandatory. Execute these release-style scenarios against
the built artifact and record actual setup, commands, output, and tracker logs
in `manual-verification-evidence.md`; the earlier disposable script output does
not satisfy this requirement. Record a failing scenario and its diagnosis in
the progress log before proceeding.

### Acceptance Verification

| AC ID                     | Status (`TODO`/`DONE`) | Evidence                                                                                                                                         |
| ------------------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| AC1                       | DONE                   | Parser tests; `cargo test --package torrust-tracker --bin torrust-tracker` (7 passed).                                                           |
| AC2                       | DONE                   | T1/T4 configuration and bootstrap tests; M2.                                                                                                     |
| AC3                       | DONE                   | T1/T2/T6 configuration tests; M3.                                                                                                                |
| AC4                       | DONE                   | T6 table-driven mandatory/default tests (129 configuration tests passed).                                                                        |
| AC5                       | DONE                   | Parser/configuration tests; M4 release-binary command, mode, exit, diagnostic, and no-listener evidence in `.tmp/issue-2151-manual/summary.txt`. |
| AC6                       | DONE                   | `cargo test --test lifecycle-signals` (8 passed); M5.                                                                                            |
| Quality and documentation | DONE                   | M1-M5 are recorded in `manual-verification-evidence.md`; 129 configuration tests, 18 CLI executable tests, 13 lifecycle tests, focused Clippy, `linter all`, and pre-commit passed.                                                          |

## Risks and Trade-offs

| Risk                                                                                                     | Mitigation                                                                                                                   |
| -------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| Priority ambiguity changes an established deployment contract.                                           | Lock the current and revised source tables in tests and document CLI precedence.                                             |
| Using clap `env` on the argument inverts `TORRUST_TRACKER_CONFIG_TOML` vs `_PATH` precedence.            | Forbidden by the architectural decisions; T1 baseline tests catch the inversion.                                             |
| Missing CLI file is reported as a missing mandatory option; relative path loads a parent-directory file. | Explicit validation / exact resolution for the CLI path (T2); M4 covers all invalid-source cases.                            |
| Stricter CLI path behavior differs from compatible environment-path behavior.                            | Document the difference, keep the environment startup log, and test both contracts separately.                               |
| New help/usage output violates the global CLI output contract.                                           | Follow the ADR for exit codes; do not add stdout plain text beyond clap defaults; defer full migration to the tracked issue. |
| CLI parsing leaks into the configuration package or is emulated through environment mutation.            | Keep parsing in the binary layer and use typed source selection.                                                             |
| Secret overrides lose priority.                                                                          | Add regression coverage where an override wins over the CLI file.                                                            |
| CLI behavior differs across direct binary, `cargo run`, containers, and service managers.                | Test the built executable; retain and document environment interfaces.                                                       |
| A refactor exposes complete TOML content in diagnostics.                                                 | Preserve redaction behavior and add no secret-bearing logs without an explicit security decision.                            |
| The issue grows into general configuration redesign.                                                     | Limit it to a file-path argument and source-selection plumbing.                                                              |

## Implementation Completion Review

After implementation, compare the result with this specification and record
invalidated assumptions, material design changes, unexpected validation findings,
and reusable lessons.

- Retrospective: `Not needed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue directory.
- No separate retrospective was needed: the implementation followed the source
  precedence and ownership decisions in this specification. The only discovery
  was a disposable verifier deadline mismatch, corrected without changing the
  product design; the progress log records it.

## References

- `packages/configuration/src/lib.rs` (`Info::new` source discovery)
- `packages/configuration/src/v3_0_0/mod.rs` (`Configuration::load`, overrides,
  mandatory values, and defaults)
- `src/bootstrap/config.rs` (default path and bootstrap loading)
- `src/console/ci/e2e/runner.rs` (existing `clap` configuration arguments;
  note its `env` binding is **not** to be copied)
- `docs/adrs/20260519000000_define_global_cli_output_contract.md` (exit codes
  and output channels for `torrust-tracker`)
- `docs/issues/drafts/cli-output-contract-migration.md` (deferred output-contract work)
- Closed EPIC [#1978](https://github.com/torrust/torrust-tracker/issues/1978)
  (`docs/issues/closed/1978-configuration-overhaul-epic/EPIC.md`; related
  configuration-schema context, not a parent)
- `tests/AGENTS.md` (in-process and child-process configuration isolation)
- `tests/lifecycle/native_tracker.rs` (child-specific environment setup)
- `docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md`
  (environment-isolation analysis)
