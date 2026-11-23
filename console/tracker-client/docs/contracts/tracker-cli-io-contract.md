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

### Terminal refusal

`tracker_client` is a `stdout-result-data` command of the [global CLI output
contract](../../../../docs/adrs/20260519000000_define_global_cli_output_contract.md). When stdout is
a terminal it refuses to run, before it reads the arguments: it writes one record to stderr, nothing
to stdout, and exits `2`:

```json
{ "kind": "tty_refusal", "message": "stdout is a TTY; pipe the output to consume result data" }
```

### One JSON object on stdout

- On success, stdout holds exactly one JSON object followed by a newline. On failure it is empty.
- `check` writes `{"results": [...]}` in both `--format` values. `--format text` is the same
  object pretty-printed. (Before issue #2768 it wrote a top-level array: a breaking change.)
- `--help` and `--version` do not write to stdout: their text is a stderr record
  (`{"kind":"help","message":"..."}`, `{"kind":"version","message":"..."}`), exit `0`.

### No plain text

No plain text on either channel. `tracing` events are JSON lines on stderr (`level`, `target`,
`fields`). Every stderr line is one JSON object. A response a tracker returns that the client cannot
decode is an `unrecognized_response` record on stderr (with the decoded or raw body) followed by the
runtime-failure record; stdout stays empty and the exit code is `1`. A panic is a
`{"kind":"panic","message":"...","location":"..."}` record on stderr (the panic hook replaces the
default plain-text message); the process then ends through the usual runtime-failure path, exit `1`.

### No secrets

No output carries a secret (section 7 of the global CLI output contract,
`docs/adrs/20260519000000_define_global_cli_output_contract.md`). A private HTTP tracker takes its
key in the URL path (`/announce/<key>`, `/scrape/<key>`), so wherever a tracker URL is written, on
stdout or stderr, the key segment is written as `****`; the password of a URL user info is masked
the same way. The rest of the URL (scheme, host, port, path prefix, query) is kept, so a reader can
tell which tracker it was:

```text
{"error":{"kind":"runtime_failure","source":"runtime","message":"Failed to get a response: connection error for url (http://127.0.0.1:7070/announce/****?info_hash=...): client error (Connect): tcp connect error: Connection refused (os error 111)"}}
```

A failed HTTP request is described by its kind (one of `timeout`, `connection`, `status`,
`decode`, `body`, `redirect`, `builder`, `request` and `unknown`), the masked URL and the chain of
causes, never by the `Debug` text of the HTTP library error. The same masking applies to
configuration and usage errors that echo a keyed URL the caller typed, in any scheme case and even
when the URL does not parse. The masking functions live in
`packages/tracker-client/src/http/client/redaction.rs`.

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

For monitor commands (for example `tracker_client check monitor udp`):

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
- 2: invalid command line, invalid tracker checker configuration/input, or terminal refusal

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
  introduce a schema version and migration note. `tracker_client` has no schema version yet: the
  `check` result change of issue #2768 is recorded as a documented breaking change here and in the
  README, and a schema version is left to a later issue of the EPIC.

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
{"event":"probe","sequence":1,"url":"udp://127.0.0.1:6969","status":"timeout","elapsed_ms":null}

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
