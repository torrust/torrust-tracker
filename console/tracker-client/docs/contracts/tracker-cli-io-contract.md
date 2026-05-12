# Tracker CLI I/O Contract

Status: Active

Scope: console/tracker-client commands, with explicit emphasis on tracker checker behavior.

## Purpose

Define stable rules for:

- output format
- stdout/stderr channel usage
- error payload structure
- process exit codes

This contract is designed for automation-first CLI usage and progressive adoption.

## Core Rules

### JSON-first output

- JSON is the default output format for command results.
- Result payloads on stdout are machine-consumable.

### Channel usage

- stdout:
  - final command results
  - structured status/results intended for downstream processing
- stderr:
  - progress reporting
  - diagnostics and warnings
  - application error output

### Monitor/progress events

For monitor commands (for example `tracker_checker monitor udp`):

- Per-probe progress is emitted as NDJSON style: one JSON object per line.
- Per-probe progress events go to stderr.
- Final aggregate summary goes to stdout as JSON.

## Error Payload Schema

Application errors should use this envelope:

```json
{ "error": { "kind": "string", "source": "string", "message": "string" } }
```

Field meaning:

- kind: machine-readable error category (for example `invalid_configuration`)
- source: where the error originated (for example `TORRUST_CHECKER_CONFIG`, `config_path`, `runtime`)
- message: human-readable detail

## Exit Codes

Exit codes represent CLI app execution status.

- 0: command executed successfully (tracker failures can still be present in JSON results)
- 1: generic application/runtime failure
- 2: invalid tracker checker configuration/input

Important:

- Tracker endpoint failures do not map to non-zero process exit codes.
- Tracker endpoint failures are part of result JSON payloads.

## Distinguishing App Errors vs Tracker Failures

- App errors:
  - invalid CLI/config input
  - internal command failures
  - serialization/runtime failures
  - reported via stderr error JSON and non-zero exit code
- Tracker failures:
  - timeout
  - connection refused
  - non-success status from tracker endpoint
  - reported inside stdout result JSON, exit code remains 0

## Stability and Migration

- New features and subcommands must comply with this contract.
- Legacy behavior is migrated progressively.
- Contract changes should remain backward compatible; if a breaking change is required,
  introduce a schema version and migration note.

## Auditability Requirements

This contract is intended to be auditable.

- Prefer explicit structured payloads over ad-hoc text messages.
- Keep field names stable once published.
- If any required field changes, bump a schema version and document migration steps.

Recommended metadata fields for auditable outputs:

- `schema_version`
- `command`
- `timestamp`
- `run_id`

These fields can be added progressively as commands are migrated.

## Verification Strategy

### Current repository phase

- Contract conformance is validated by documentation reviews and issue-level acceptance criteria.
- New feature specs should include explicit checks for:
  - stdout/stderr channel behavior
  - JSON envelope conformance
  - exit-code semantics

### Post-extraction phase (target)

When `console/tracker-client` is extracted to its own repository, add dedicated E2E conformance
tests for this contract.

Recommended E2E coverage:

- golden stdout/stderr fixtures for representative command runs
- exit-code assertions (`0`, `1`, `2`)
- NDJSON per-line validation for monitor probe events
- JSON schema validation for final summaries and error envelopes

Until extraction, this remains a planned verification step.

## Examples

### Example 1: Successful run with tracker failures

```text
stdout:
{"udp_trackers":[{"url":"udp://127.0.0.1:6969","status":{"code":"timeout","message":"announce timeout"}}]}

stderr:
{"event":"probe","url":"udp://127.0.0.1:6969","status":"timeout","elapsed_ms":null}

exit code: 0
```

### Example 2: Invalid configuration

```text
stdout:

stderr:
{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"JSON parse error: trailing comma at line 7 column 5"}}

exit code: 2
```

### Example 3: Generic application failure

```text
stdout:

stderr:
{"error":{"kind":"runtime_failure","source":"runtime","message":"failed to initialize async runtime"}}

exit code: 1
```
