# Manual Verification - Issue #2023

**Status:** TODO

This file is the reproducible runtime evidence record for the implemented observability changes.
Create one evidence section for each scenario in the matrix below. Do not combine configured and
absent cases: each configuration change must have its own configuration, requests, and output.

## Evidence Requirements

Each completed scenario section must include:

- date/time in UTC, commit SHA, OS, and Rust toolchain;
- the complete effective local v3 tracker configuration, with sensitive values redacted;
- the exact tracker start and stop commands;
- every request command, including the health-check request, announce request, and metrics request;
- unedited relevant startup log lines and API/Prometheus response output;
- expected versus actual result, including the configured bind address, post-bind service binding,
  and public URL where applicable.

Retain ignored runtime artifacts in `.tmp/issue-2023-public-url-observability/<case>/`, including
the configuration file, tracker log, health response, announce output, and metrics response. Link
or name each retained artifact from its evidence section.

## Scenario Matrix

| ID | Configuration case | Required evidence | Status |
| --- | --- | --- | --- |
| M1 | Configured public URL with HTTP tracker `bind_address = "0.0.0.0:0"` | Effective configuration; startup logs; health-check response showing distinct `binding`, `service_binding`, and `public_url`. | TODO |
| M2 | Configured public URL after an HTTP announce | Announce command/output; Prometheus metrics response showing `public_url` together with existing `server_binding_*` labels. | TODO |
| M3 | Configured public URL startup logs | Relevant structured startup log lines showing separate `service_binding` and `public_url` fields. | TODO |
| M4 | No configured public URL | Effective configuration; startup logs; health-check response with `public_url: null`; metrics response without a `public_url` label. | TODO |

## M1 - Configured Public URL Health Check

**Status:** TODO

### Environment

| Item | Value |
| --- | --- |
| Date/time (UTC) | |
| Commit | |
| OS | |
| Rust toolchain | |
| Artifact directory | |

### Effective Configuration

```toml
# Paste the complete effective isolated v3 configuration here.
```

### Commands and Output

```sh
# Paste tracker start, health-check request, and tracker stop commands here.
```

```text
# Paste relevant startup log and health-check response output here.
```

### Expected and Actual Result

| Expected | Actual |
| --- | --- |
| | |

## M2 - Configured Public URL Metrics

**Status:** TODO

### Commands and Output

```sh
# Paste HTTP announce and Prometheus metrics request commands here.
```

```text
# Paste the announce output and matching metric samples here.
```

### Expected and Actual Result

| Expected | Actual |
| --- | --- |
| | |

## M3 - Configured Public URL Startup Logs

**Status:** TODO

### Commands and Output

```text
# Paste only the relevant structured startup log lines here.
```

### Expected and Actual Result

| Expected | Actual |
| --- | --- |
| | |

## M4 - Absent Public URL

**Status:** TODO

### Effective Configuration

```toml
# Paste the complete effective isolated v3 configuration with public_url omitted.
```

### Commands and Output

```sh
# Paste tracker start, health-check, HTTP announce, metrics, and tracker stop commands here.
```

```text
# Paste the relevant startup log, health-check, announce, and metric outputs here.
```

### Expected and Actual Result

| Expected | Actual |
| --- | --- |
| | |
