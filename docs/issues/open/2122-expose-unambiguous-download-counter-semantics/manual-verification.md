# Manual Verification Evidence

**Date:** {YYYY-MM-DD HH:MM UTC}
**Tracker revision:** {commit SHA}
**Issue:** #2122

## Safety

Do not record API tokens, passwords, private keys, connection strings, or other
secrets. Replace each secret with `{REDACTED}` in commands, configuration, and
HTTP requests.

## Local Environment

- Operating system: {value}
- Tracker command: {exact command}
- Working directory: {path}
- Configuration source: {environment variable or file path}
- Configuration: {redacted TOML or link to an issue-local redacted fixture}

## M1 - Disabled Persistence

**Status:** `TODO`

### Commands

```text
{exact tracker start, client, HTTP request, and restart commands}
```

### Requests And Responses

```text
{exact HTTP method, redacted URL, request body, HTTP status, and response body}
```

### Result

{Record legacy `completed`, `completed_in_session`, `completed_persisted`, and
`completed_persisted_enabled` before and after restart. Confirm the persisted
Prometheus metric is absent.}

## M2 - Enabled Persistence

**Status:** `TODO`

### Commands

```text
{exact tracker start, client, HTTP request, and restart commands}
```

### Requests And Responses

```text
{exact HTTP method, redacted URL, request body, HTTP status, and response body}
```

### Result

{Record all legacy and new REST values before and after restart using the same
database. Confirm the persisted metric is exported and document an enabled
zero-value observation when feasible.}

## M3 - Legacy Migration

**Status:** `TODO`

### Commands

```text
{exact metrics request command}
```

### Requests And Responses

```text
{exact HTTP method, redacted URL, HTTP status, and relevant response body}
```

### Result

{Confirm the observed legacy and new REST fields and metric identifiers,
descriptions, values, and availability match the approved ADR.}
