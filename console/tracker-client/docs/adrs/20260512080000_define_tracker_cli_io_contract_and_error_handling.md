# ADR 20260512080000: Define Tracker CLI I/O Contract and Error Handling

- Status: Accepted
- Date: 2026-05-12
- Scope: console/tracker-client

## Context

The tracker client is a growing CLI surface with multiple commands (UDP client, HTTP client,
tracker checker, and monitor features under active development). The project intends to extract
this application into an independent repository.

Without an explicit contract, command outputs and error behavior can diverge, breaking user
automation and increasing maintenance cost.

At the same time, existing commands may not yet fully match the desired target behavior, so the
team needs a migration policy, not a flag day rewrite.

## Decision

Define a global Tracker CLI I/O contract for console/tracker-client.

### 1. Default output format

- JSON is the default output format.

### 2. Output channels

- stdout: normal command results and machine-consumable output.
- stderr: progress reporting, diagnostics, warnings, and error output.

For monitor-style streaming behavior:

- Progress/probe events may be emitted as one JSON object per line (NDJSON style).
- If emitted as progress, they go to stderr.
- Final command result summary goes to stdout as JSON.

### 3. Exit-code semantics

Exit codes represent tracker client app execution state, not tracker endpoint health status.

- 0: command executed successfully, even if one or more trackers reported failures/timeouts.
- 1: generic application/runtime failure (unexpected internal error).
- 2: invalid tracker checker configuration/input errors.

Tracker-specific failures (for example announce timeout, scrape timeout, non-200 HTTP from a
tracker) are represented in JSON result payloads, not in non-zero exit codes.

### 4. Progressive migration policy

- New features and new subcommands must follow this contract.
- Existing features that do not yet comply will be migrated progressively when touched by new
  feature work or dedicated refactors.
- No immediate broad rewrite is required.

### 5. Scope location

This policy is intentionally documented under console/tracker-client docs because the tracker
client is expected to be extracted into its own repository.

### 6. Auditability and testing strategy

- Contracts should be auditable through stable structured payloads and explicit field definitions.
- During the monorepo phase, conformance is enforced through issue specs and acceptance criteria.
- After tracker-client extraction to its own repository, add dedicated E2E contract tests for
  stdout/stderr behavior, exit codes, NDJSON events, and JSON schema conformance.

## Consequences

### Positive

- Predictable behavior for shell pipelines and automation.
- Clear separation between app-level failure and tracker-level status.
- Lower migration risk through incremental adoption.
- Documentation remains aligned with future repository extraction boundaries.
- Auditable CLI behavior suitable for compliance and regression verification.

### Negative

- Transitional inconsistency until all legacy paths are migrated.
- Additional implementation and review burden to keep channel/exit behavior consistent.
- Full E2E contract coverage is deferred until extraction, so short-term assurance relies on
  spec-driven validation.

## Implementation Notes

- Command specs should reference the tracker client I/O contract document.
- New command acceptance criteria should include channel correctness and exit-code behavior.
- Contract schema updates should be backward compatible or explicitly versioned.

## References

- [Tracker CLI I/O Contract](../contracts/tracker-cli-io-contract.md)
- [console/tracker-client/README.md](../../README.md)
