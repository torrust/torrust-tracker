---
doc-type: issue
issue-type: bug
status: planned
priority: p3
github-issue: 1042
spec-path: docs/issues/open/1042-tracker-checker-http-improve-error-message-json-config.md
branch: 1042-tracker-checker-http-improve-error-message-json-config
related-pr: null
last-updated-utc: 2026-05-12 08:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

# Issue #1042 — Tracker Checker (HTTP): Improve Error Message When JSON Config Is Not Well-Formatted

## Overview

When the Tracker Checker is supplied with a malformed JSON configuration (e.g. a trailing comma),
it panics with a generic `invalid config format` message followed by a buried "Caused by" chain.
The goal is to surface the specific JSON parse error at the top level so the user can fix the
configuration immediately without inspecting the full backtrace.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1042>
- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>

## Motivation

The current output on a malformed config is:

```text
thread 'main' panicked at console/tracker-client/src/bin/tracker_checker.rs:6:22:
Some checks fail: invalid config format

Caused by:
    JSON parse error: trailing comma at line 7 column 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The useful detail (`JSON parse error: trailing comma at line 7 column 5`) is buried in the
"Caused by" chain. A developer who does not know to look for that will see only
`invalid config format` and have no idea where the problem is.

The fix should make the detailed JSON parse error visible immediately — either by improving
the context message, removing the generic context so the underlying error propagates directly,
or by printing the error cleanly to stderr before exiting non-zero (instead of panicking).

## How to Reproduce

Run the checker with invalid JSON (note the trailing comma in the `http_trackers` array):

```console
TORRUST_CHECKER_CONFIG='{
    "udp_trackers": [],
    "http_trackers": [
        "http://127.0.0.1:7070",
        "http://127.0.0.1:7070/",
        "http://127.0.0.1:7070/announce",
    ],
    "health_checks": []
}' cargo run --bin tracker_checker
```

Current output:

```text
thread 'main' panicked at console/tracker-client/src/bin/tracker_checker.rs:6:22:
Some checks fail: invalid config format

Caused by:
    JSON parse error: trailing comma at line 7 column 5
```

## Current Behaviour

In `console/tracker-client/src/console/clients/checker/app.rs`, both code paths that call
`parse_from_json` wrap the error with `.context("invalid config format")`:

```rust
fn setup_config(args: Args) -> Result<Configuration> {
    match (args.config_path, args.config_content) {
        (Some(config_path), _) => load_config_from_file(&config_path),
        (_, Some(config_content)) => parse_from_json(&config_content).context("invalid config format"),
        _ => Err(anyhow::anyhow!("no configuration provided")),
    }
}

fn load_config_from_file(path: &PathBuf) -> Result<Configuration> {
    let file_content = std::fs::read_to_string(path)
        .with_context(|| format!("can't read config file {}", path.display()))?;
    parse_from_json(&file_content).context("invalid config format")
}
```

And the binary entry-point panics on error:

```rust
app::run().await.expect("Some checks fail");
```

## Proposed Behaviour

Replace the generic context string with a message that includes the source of the configuration
and directs the user to the specific problem.

Do not panic on configuration errors. Print a structured JSON error to stderr and exit with a
non-zero status code.

**Error JSON format and exit codes follow the Tracker CLI I/O Contract:**

- References:
  - [ADR: Define Tracker CLI I/O Contract and Error Handling](../../console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md)
  - [Tracker CLI I/O Contract](../../console/tracker-client/docs/contracts/tracker-cli-io-contract.md)

**Error payload structure:**

```json
{
  "error": {
    "kind": "invalid_configuration",
    "source": "<delivery_source>",
    "message": "<json_parse_detail>"
  }
}
```

- `kind`: Always `"invalid_configuration"` for config errors
- `source`: How the configuration was delivered (e.g., `"TORRUST_CHECKER_CONFIG"`, `"/etc/tracker/config.json"`)
- `message`: The detailed parse error from serde_json (e.g., `"JSON parse error: trailing comma at line 7 column 5"`)

**Key architectural principle:** Decouple the **delivery mechanism** (how config arrived) from
**error presentation** (what configuration was invalid). This allows future refactoring of how
config is injected (new sources like stdin) without affecting error messaging.

**Exit code policy:**

- `2` for configuration errors (invalid JSON, missing config, invalid config values)
- `1` reserved for non-config general checker failures

**Example stderr output:**

```text
{"error":{"kind":"invalid_configuration","source":"TORRUST_CHECKER_CONFIG","message":"JSON parse error: trailing comma at line 7 column 5"}}
```

The key requirement is that the specific serde/JSON error message is immediately visible without
needing `RUST_BACKTRACE=1`.

## Key Files

| File                                                           | Role                                                                             |
| -------------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `console/tracker-client/src/console/clients/checker/app.rs`    | `setup_config`, `load_config_from_file` — context wrapping                       |
| `console/tracker-client/src/console/clients/checker/config.rs` | `parse_from_json` + `ConfigurationError` — already has good per-variant messages |
| `console/tracker-client/src/bin/tracker_checker.rs`            | Binary entry point with `expect` panic                                           |

## Goals

- [ ] The specific JSON parse error is visible to the user without `RUST_BACKTRACE=1`
- [ ] The error output clearly identifies whether the bad configuration came from an environment
      variable or from a file
- [ ] On configuration errors, the binary prints JSON error output to stderr and exits non-zero
- [ ] Checker errors follow a standardized JSON schema: `{ "error": { "kind", "source", "message" } }`
- [ ] Configuration errors use process exit code `2`
- [ ] Valid configurations are unaffected
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies
- [ ] Existing tests pass

## Implementation Plan

### Task 1: Refactor error handling in `setup_config` and `load_config_from_file`

In `console/tracker-client/src/console/clients/checker/app.rs`:

- Remove generic `.context("invalid config format")` wrapping
- Pass the delivery source (e.g., environment variable name or file path) to error handlers
- Allow the underlying JSON parse error to propagate directly or wrap it with source-aware context

### Task 2: Replace `expect` panic with clean error exit

In `console/tracker-client/src/bin/tracker_checker.rs`:

- Replace `app::run().await.expect("Some checks fail")` with structured error handling
- On `Err`, serialize the error to JSON with the contract-compliant envelope
- Write JSON error to stderr
- Exit with code `2` for configuration errors, `1` for other errors

### Task 3: Add configuration source tracking to error context

Ensure that configuration source information (delivery mechanism) is captured and included in
error payloads without altering how the final configuration is presented.

### Task 4: Add unit tests

In `console/tracker-client/src/console/clients/checker/`:

- Test `parse_from_json` with invalid JSON (trailing comma, syntax errors, type mismatches)
- Verify that parse errors propagate without generic wrapping
- Test error serialization to the contract envelope format

### Task 5: Add integration tests

In `console/tracker-client/tests/` or appropriate test module:

- End-to-end test: TORRUST_CHECKER_CONFIG with invalid JSON → stderr contains JSON error,
  exit code is 2
- End-to-end test: Config file with invalid JSON → stderr contains JSON error with file path,
  exit code is 2
- End-to-end test: Valid config → checker runs normally, exit code is 0 (even if tracker checks fail)
- Verify JSON error envelope conforms to the Tracker CLI I/O Contract schema

## Acceptance Criteria

- [ ] AC1: Running the checker with a trailing comma in `TORRUST_CHECKER_CONFIG` shows the JSON
      parse error message (e.g. `trailing comma at line N column M`) without `RUST_BACKTRACE=1`
- [ ] AC2: Running the checker with a trailing comma in a config file shows both the file path
      and the JSON parse error message
- [ ] AC3: Configuration errors are reported as JSON to stderr following the Tracker CLI I/O Contract
- [ ] AC4: Configuration errors use exit code `2`
- [ ] AC5: Running the checker with a valid configuration produces the same output as before
- [ ] AC6: Unit tests pass for parse error handling and error serialization
- [ ] AC7: Integration tests pass for end-to-end error scenarios (env var and file sources)
- [ ] AC8: `linter all` exits with code `0`
- [ ] AC9: `cargo machete` reports no unused dependencies
- [ ] AC10: Existing tests pass

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

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/open/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Implementation completed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-11 20:00 UTC - Agent - Spec created from GitHub issue #1042 content
- 2026-05-12 00:00 UTC - Agent - Incorporated maintainer decisions: JSON error output, no panic, both env and file config paths
- 2026-05-12 08:00 UTC - Agent - Incorporated answered follow-ups: standardized checker error schema and exit code `2` for configuration errors

## Open Questions

No open questions at this time.

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Clients extracted to new package: <https://github.com/torrust/torrust-tracker/issues/1067>
- Tracker CLI I/O contract: `console/tracker-client/docs/contracts/tracker-cli-io-contract.md`
- Tracker CLI ADR: `console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md`
