---
doc-type: issue
issue-type: bug
status: in-progress
priority: p3
github-issue: 1042
spec-path: docs/issues/open/1042-tracker-checker-http-improve-error-message-json-config.md
branch: 1042-tracker-checker-improve-error-message-json-config
related-pr: null
last-updated-utc: 2026-05-12 10:00
semantic-links:
  related-artifacts:
    - console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md
    - console/tracker-client/docs/contracts/tracker-cli-io-contract.md
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

- [x] The specific JSON parse error is visible to the user without `RUST_BACKTRACE=1`
- [x] The error output clearly identifies whether the bad configuration came from an environment
      variable or from a file
- [x] On configuration errors, the binary prints JSON error output to stderr and exits non-zero
- [x] Checker errors follow a standardized JSON schema: `{ "error": { "kind", "source", "message" } }`
- [x] Configuration errors use process exit code `2`
- [x] Valid configurations are unaffected
- [x] `linter all` exits with code `0`
- [x] `cargo machete` reports no unused dependencies
- [x] Existing tests pass

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

- [x] AC1: Running the checker with a trailing comma in `TORRUST_CHECKER_CONFIG` shows the JSON
      parse error message (e.g. `trailing comma at line N column M`) without `RUST_BACKTRACE=1`
- [x] AC2: Running the checker with a trailing comma in a config file shows both the file path
      and the JSON parse error message
- [x] AC3: Configuration errors are reported as JSON to stderr following the Tracker CLI I/O Contract
- [x] AC4: Configuration errors use exit code `2`
- [x] AC5: Running the checker with a valid configuration produces the same output as before
- [x] AC6: Unit tests pass for parse error handling and error serialization
- [x] AC7: Integration tests pass for end-to-end error scenarios (env var and file sources)
- [x] AC8: `linter all` exits with code `0`
- [x] AC9: `cargo machete` reports no unused dependencies
- [x] AC10: Existing tests pass

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                    |
| ----- | ---------------------- | ----------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | Integration test `it_should_include_parse_detail_in_stderr_error_message_on_trailing_comma` passes          |
| AC2   | DONE                   | Integration test `it_should_include_file_path_in_stderr_source_field` passes                                |
| AC3   | DONE                   | JSON envelope `{"error":{"kind":"invalid_configuration","source":"...","message":"..."}}` written to stderr |
| AC4   | DONE                   | `std::process::exit(2)` for `AppError::InvalidConfig`; verified by integration tests                        |
| AC5   | DONE                   | 35 unit tests + 9 integration tests pass; no regressions                                                    |
| AC6   | DONE                   | 12 new unit tests in `config.rs` and `error.rs` all pass                                                    |
| AC7   | DONE                   | 9 integration tests in `tests/tracker_checker.rs` all pass                                                  |
| AC8   | DONE                   | `cargo clippy -- -D warnings` and `cargo fmt --check` exit 0                                                |
| AC9   | DONE                   | `cargo machete` — `anyhow` still used by other modules; no unused deps                                      |
| AC10  | DONE                   | All 35 pre-existing unit tests pass unchanged                                                               |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/open/`
- [x] Spec reviewed and approved by user/maintainer
- [x] Implementation completed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-11 20:00 UTC - Agent - Spec created from GitHub issue #1042 content
- 2026-05-12 00:00 UTC - Agent - Incorporated maintainer decisions: JSON error output, no panic, both env and file config paths
- 2026-05-12 08:00 UTC - Agent - Incorporated answered follow-ups: standardized checker error schema and exit code `2` for configuration errors

## Manual Verification

The following scenarios have been tested manually to verify the implementation meets the specification.

### Scenario 1: Valid Configuration with Tracker Demo URLs

**Command:**

```console
$ TORRUST_CHECKER_CONFIG='{
    "udp_trackers": [],
    "http_trackers": [
        "https://http1.torrust-tracker-demo.com:443/announce",
        "https://http1.torrust-tracker-demo.com:443/",
        "https://http1.torrust-tracker-demo.com:443"
    ],
    "health_checks": []
}' cargo run --bin tracker_checker
```

**Output:**

```json
[
  {
    "Http": {
      "Ok": {
        "url": "https://http1.torrust-tracker-demo.com/announce",
        "results": [
          ["Announce", { "Ok": null }],
          ["Scrape", { "Ok": null }]
        ]
      }
    }
  },
  {
    "Http": {
      "Ok": {
        "url": "https://http1.torrust-tracker-demo.com/",
        "results": [
          ["Announce", { "Ok": null }],
          ["Scrape", { "Ok": null }]
        ]
      }
    }
  },
  {
    "Http": {
      "Ok": {
        "url": "https://http1.torrust-tracker-demo.com/",
        "results": [
          ["Announce", { "Ok": null }],
          ["Scrape", { "Ok": null }]
        ]
      }
    }
  }
]
```

**Exit Code:** `0` (success)

**Status:** ✅ PASS — Valid configuration runs successfully and produces tracker check results.

---

### Scenario 2: Trailing Comma in JSON Config via Environment Variable

**Command:**

```console
$ TORRUST_CHECKER_CONFIG='{
    "udp_trackers": [],
    "http_trackers": [
        "https://http1.torrust-tracker-demo.com:443/announce",
        "https://http1.torrust-tracker-demo.com:443/",
        "https://http1.torrust-tracker-demo.com:443",
    ],
    "health_checks": []
}' cargo run --bin tracker_checker
```

**Output (stderr):**

```json
{
  "error": {
    "kind": "invalid_configuration",
    "source": "TORRUST_CHECKER_CONFIG",
    "message": "JSON parse error: trailing comma at line 7 column 5"
  }
}
```

**Exit Code:** `2` (configuration error)

**Status:** ✅ PASS — JSON parse error detail visible immediately, source identified as environment variable, exit code is 2.

---

### Scenario 3: Missing Closing Bracket in JSON Config via Environment Variable

**Command:**

```console
$ TORRUST_CHECKER_CONFIG='{
    "udp_trackers": [],
    "http_trackers": ["https://http1.torrust-tracker-demo.com:443/announce"
}' cargo run --bin tracker_checker
```

**Output (stderr):**

```json
{
  "error": {
    "kind": "invalid_configuration",
    "source": "TORRUST_CHECKER_CONFIG",
    "message": "JSON parse error: expected `,` or `]` at line 4 column 1"
  }
}
```

**Exit Code:** `2` (configuration error)

**Status:** ✅ PASS — Serde JSON parse error visible, source is env var, exit code is 2.

---

### Scenario 4: Invalid JSON from Configuration File

**Command:**

```console
$ cat > /tmp/invalid-tracker-config.json << 'EOF'
{
    "udp_trackers": [],
    "http_trackers": [
        "https://http1.torrust-tracker-demo.com:443/announce",
        "https://http1.torrust-tracker-demo.com:443/",
    ],
    "health_checks": []
}
EOF
$ TORRUST_CHECKER_CONFIG_PATH=/tmp/invalid-tracker-config.json cargo run --bin tracker_checker
```

**Output (stderr):**

```json
{
  "error": {
    "kind": "invalid_configuration",
    "source": "/tmp/invalid-tracker-config.json",
    "message": "JSON parse error: trailing comma at line 6 column 5"
  }
}
```

**Exit Code:** `2` (configuration error)

**Status:** ✅ PASS — File path shown in source field, JSON parse error detail visible, exit code is 2.

---

### Scenario 5: No Configuration Provided

**Command:**

```console
$ cargo run --bin tracker_checker
```

**Output (stderr):**

```json
{
  "error": {
    "kind": "invalid_configuration",
    "source": "TORRUST_CHECKER_CONFIG",
    "message": "no configuration provided"
  }
}
```

**Exit Code:** `2` (configuration error)

**Status:** ✅ PASS — Specific error message when no config provided, exit code is 2.

---

### Scenario 6: Invalid Configuration Content (Bad URL)

**Command:**

```console
$ TORRUST_CHECKER_CONFIG='{
    "udp_trackers": [],
    "http_trackers": [
        "not a valid url!"
    ],
    "health_checks": []
}' cargo run --bin tracker_checker
```

**Output (stderr):**

```json
{
  "error": {
    "kind": "invalid_configuration",
    "source": "TORRUST_CHECKER_CONFIG",
    "message": "Invalid URL: relative URL without a base"
  }
}
```

**Exit Code:** `2` (configuration error)

**Status:** ✅ PASS — Configuration validation errors surfaced with detail, exit code is 2.

---

## Summary of Manual Verification

All 6 manual test scenarios pass:

- ✅ Valid config runs successfully (exit 0)
- ✅ Trailing comma error captured with line/column detail (exit 2, stderr JSON, source=env)
- ✅ Malformed JSON error captured with detail (exit 2, stderr JSON, source=env)
- ✅ File-sourced invalid JSON shows file path in source field (exit 2, stderr JSON, source=path)
- ✅ Missing config handled gracefully (exit 2, stderr JSON)
- ✅ Invalid URL in config surfaced with validation detail (exit 2, stderr JSON)

All error outputs follow the Tracker CLI I/O Contract schema and are sent to stderr with exit code 2 (config errors).

## Open Questions

No open questions at this time.

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- Clients extracted to new package: <https://github.com/torrust/torrust-tracker/issues/1067>
- Tracker CLI I/O contract: `console/tracker-client/docs/contracts/tracker-cli-io-contract.md`
- Tracker CLI ADR: `console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md`
