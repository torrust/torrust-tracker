# Define the Global CLI Output Contract

## Description

The Torrust Tracker repository ships several CLI binaries: the tracker server daemon
(`torrust-tracker`), operational tools (`http_health_check`, `e2e_tests_runner`,
`qbittorrent_e2e_runner`), and the interactive tracker client (`tracker_client`).

Without a repository-wide output contract, each binary can diverge in how it uses stdout,
stderr, exit codes, and output format. This causes friction for shell pipelines, container
health checks, CI orchestration, and AI agents that drive CLI commands programmatically.

The `console/tracker-client` package already has a local ADR
(`20260512080000_define_tracker_cli_io_contract_and_error_handling.md`) with a compatible
contract, deliberately scoped to that package because extraction to its own repository was
anticipated. That local ADR is superseded by this global one.

The Torrust Index project has an equivalent decision record (`ADR-T-010`) that served as
the primary reference for this decision.

**This ADR is prescriptive.** The current codebase does not yet fully comply. Adoption is
progressive via a dedicated follow-up issue; see the migration policy section below.

## Agreement

### 1. Output channels

- **stdout**: final command result data only.
  - On success: exactly one JSON object followed by a newline.
  - On failure: empty (nothing written to stdout).
- **stderr**: everything else — internal tracing diagnostics, user-facing progress events,
  help text, usage errors, panic records.
  - Each record is a complete JSON line (NDJSON: one JSON object per line).
  - Records should carry a `kind` field (or equivalent) to allow filtering.

No plain text on either channel, at any verbosity level.

### 2. Exit codes

| Code | Meaning                                                 |
| ---- | ------------------------------------------------------- |
| 0    | Command executed successfully                           |
| 1    | Runtime or internal failure                             |
| 2    | Usage error — invalid arguments, config, or TTY refusal |

Tracker endpoint failures (announce timeout, non-200 response, etc.) are represented in the
JSON result payload on stdout. They do not cause a non-zero exit code.

### 3. Binary classification

Every binary is assigned one of two output classes.

**`stdout-result-data`** — emits a JSON result object on stdout. TTY refusal applies (see
section 4). On failure, stdout is empty; the error appears on stderr as a JSON record.

**`no-stdout-result`** — emits nothing on stdout. Pass/fail is communicated via exit code.
All diagnostics go to stderr via the tracing subscriber or direct JSON stderr writes.

| Binary                   | Class                | Notes                                                                 |
| ------------------------ | -------------------- | --------------------------------------------------------------------- |
| `torrust-tracker`        | `no-stdout-result`   | Long-running daemon; tracing events to stderr                         |
| `http_health_check`      | `stdout-result-data` | Health status JSON on stdout; currently non-compliant (plain text)    |
| `e2e_tests_runner`       | `no-stdout-result`   | CI orchestrator; pass/fail via exit code                              |
| `qbittorrent_e2e_runner` | `no-stdout-result`   | CI orchestrator; pass/fail via exit code                              |
| `tracker_client`         | `stdout-result-data` | Announce/scrape results as JSON; monitor progress as NDJSON on stderr |

The `profiling` binary is a developer-only diagnostic harness and is excluded from the
normative scope of this contract.

### 4. TTY refusal

Commands in the `stdout-result-data` class must refuse to run when stdout is a terminal (TTY).

- Exit code: 2.
- A JSON diagnostic record is written to stderr explaining the refusal.

Rationale: when stdout is a TTY, result JSON would be mixed with the shell prompt, breaking
pipelines silently. Refusing makes the contract mechanically enforceable and the error
immediately visible. Users can suppress the check with `| cat` or `| jq`.

Example stderr record on TTY refusal (pretty-printed for readability; the actual wire format is
a single JSON line per the NDJSON contract):

```json
{
  "kind": "tty_refusal",
  "message": "stdout is a TTY; pipe the output to consume result data"
}
```

### 5. User-facing verbosity

Verbosity is command-specific. No global verbosity scheme is prescribed by this ADR.

The single invariant is: **all output at any verbosity level must be JSON**. Plain text is not
permitted on stdout or stderr regardless of the verbosity setting.

### 6. Shared CLI infrastructure

No shared infrastructure package is prescribed by this ADR. Implementors may refer to
the Torrust Index `cli-common` package as a reference implementation for common scaffolding
(TTY refusal, stdout emitter, panic hook, tracing setup). Start simple; extract common
patterns gradually as project needs arise.

### 7. Redaction policy

JSON diagnostics and result payloads must not expose secrets or credentials.

- Configuration values loaded from secret sources (environment variables, files) must be
  masked before inclusion in any JSON output (use `mask_secrets()` or equivalent).
- The mask value is a fixed string such as `"****"`.
- Field names that reference secrets may appear; only the values must be masked.

### 8. Workspace lint guards

Once migration is complete, the following `clippy` lints will be denied at workspace level:

- `clippy::print_stdout`
- `clippy::print_stderr`

These lints enforce that direct `print!`, `println!`, `eprint!`, and `eprintln!` calls do not
bypass the structured output contract. This interacts with issue #1786 (workspace lints
migration); coordination between that effort and the migration issue for this ADR is required.

### 9. AI agent output capture practice

AI agents reuse terminal sessions, which prevents reliable per-command stdout/stderr capture.

Recommended practice when an AI agent drives a CLI command that falls under this contract:

- Redirect stdout to `.tmp/<command>.stdout`
- Redirect stderr to `.tmp/<command>.stderr`

`.tmp/` is workspace-local and git-ignored (following the existing `TORRUST_GIT_HOOKS_LOG_DIR`
convention). Two separate files preserve the stdout/stderr channel split, which is important
because stdout carries result data and stderr carries diagnostics.

### 10. Migration policy

This ADR is prescriptive. The current codebase does not yet fully comply.

Migration rules:

- **New commands and features** must comply with this contract from the moment they are
  written.
- **Existing non-compliant commands** are migrated progressively when touched by new feature
  work or via a dedicated follow-up migration issue. No immediate broad rewrite is required.
- **Deprecated binaries** (`http_tracker_client`, `udp_tracker_client`, `tracker_checker`)
  should be **removed** rather than migrated.
- Until a binary is migrated, any non-compliance must be documented in the migration issue,
  not silently tolerated.

## Alternatives Considered

**Adopt plain-text output with a `--json` flag.** Rejected because machine-readable output
should be the default; opt-in JSON creates inconsistent automation surfaces and increases
the API surface without benefit.

**Make TTY refusal opt-in.** Rejected because opt-in enforcement is not enforcement. The
value of TTY refusal comes precisely from it being unconditional for stdout-result-data
commands.

**Define a single global verbosity flag (`-q`/`-v`/`-vv`).** Rejected because verbosity
requirements vary significantly by command. A global scheme would be either too coarse or
would require command-specific override logic anyway. The binding constraint — all output
is JSON — is prescribed here; verbosity levels are left to each command.

## Consequences

### Positive

- Shell pipelines, container health checks, and CI scripts can rely on a stable, parseable
  output format across all Torrust Tracker binaries.
- TTY refusal makes contract violations immediately visible rather than causing silent
  corruption.
- AI agents can capture and process command output reliably.
- The contract is aligned with the Torrust Index decision (ADR-T-010), enabling consistent
  tooling across the Torrust ecosystem.

### Negative

- Developers can no longer run `stdout-result-data` commands in a terminal without piping
  through `cat` or `jq`. This is intentional friction that enforces the contract.
- Migrating existing non-compliant binaries requires implementation work tracked separately.
- Until migration is complete, the ADR is accepted but partially unimplemented.

## Date

2026-05-19

## References

- Issue spec: `docs/issues/open/1798-global-cli-output-contract-adr.md`
- Tracker-client local ADR (superseded by this ADR):
  `console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md`
- Tracker-client I/O contract (narrowed to tracker-client–specific rules):
  `console/tracker-client/docs/contracts/tracker-cli-io-contract.md`
- Torrust Index ADR-T-010 (primary reference):
  <https://github.com/torrust/torrust-index/blob/develop/adr/010-global-command-line-output-contract.md>
- Torrust Tracker Deployer — console output research:
  - <https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/research/UX/console-output-logging-strategy.md>
  - <https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/research/UX/console-stdout-stderr-handling.md>
  - <https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/research/UX/user-output-vs-logging-separation.md>
- Related issue: [#1786](https://github.com/torrust/torrust-tracker/issues/1786) (workspace
  lints migration — interacts with print_stdout/print_stderr guards)
- ADR index: `docs/adrs/index.md`
