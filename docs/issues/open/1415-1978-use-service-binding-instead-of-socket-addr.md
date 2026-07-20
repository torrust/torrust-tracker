---
doc-type: issue
issue-type: enhancement
status: open
priority: p2
github-issue: 1415
spec-path: docs/issues/open/1415-1978-use-service-binding-instead-of-socket-addr.md
branch: "1415-listen-url"
related-pr: null
last-updated-utc: 2026-07-13 21:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/
    - packages/tracker-core/src/lib.rs
    - packages/http-core/src/container.rs
    - packages/udp-server/src/server/launcher.rs
    - src/bootstrap/jobs/
---


# Issue #1415 - Use `ServiceBinding` (protocol + address) instead of bare `SocketAddr` for service identity

> **EPIC position**: Subissue #5 of 9. Independent — no config changes, no overlap with other subissues. Can run in parallel with #1453, #1490, #889.

## Goal

Replace bare `SocketAddr` values with `ServiceBinding` (from `torrust-net-primitives`) wherever the socket address is used for service identity — in logs, events, health check info, and metrics. `ServiceBinding` already carries both the protocol scheme and the socket address, so consumers get the full protocol + address context without any new types or external crate changes.

## Background

The tracker currently passes `SocketAddr` values around for each service (UDP tracker, HTTP tracker, API, health check). However, the socket address alone lacks the **protocol/scheme** information. When logging startup messages, the code manually constructs URL-like strings:

```text
UDP TRACKER: Started on: udp://0.0.0.0:6868
HTTP TRACKER: Started on: http://0.0.0.0:7070
API: Started on http://0.0.0.0:1212
```

But this protocol context is not available as a first-class value in the places that need it:

1. **Health check API** (#1409) — needs to expose the service type and address
2. **Metrics** (#1403 / #1414) — needs protocol as a label for Prometheus metrics
3. **Events** — domain events should carry the service protocol, not just the socket address

The solution is to use the existing `ServiceBinding` type from `torrust-net-primitives` (which already carries `scheme` + `SocketAddr`) wherever bare `SocketAddr` is currently passed. No new types, no external crate changes, no URL path segments.

### What this issue does NOT do

- **Does not add URL path segments** (e.g. `/announce`). Path segments are hardcoded per protocol and not useful for service identity.
- **Does not resolve the bind address to a concrete IP**. `ServiceBinding` carries the configured bind address as-is (e.g. `0.0.0.0:7070`).
- **Does not provide a public-facing URL**. That is handled by #1417 (`public_url` config field).

### Future extension: internal connection URL

A future issue could build an "internal connection URL" from the OS-resolved IP + hardcoded path segment (e.g. `https://192.168.1.5:7070/announce`). This would be useful when the `public_url` is not configured but the service is bound to a concrete reachable IP. This is deferred — the `public_url` field (#1417) covers the primary use case.

## Scope

### In Scope

- Use `ServiceBinding` (from `torrust-net-primitives`) wherever bare `SocketAddr` is currently passed for service identity:
  - Server startup logging
  - Health check info structs
  - Metrics labels
  - Domain events (if applicable)
- No new types — `ServiceBinding` already has `protocol()` and `bind_address()` methods
- No changes to `torrust-net-primitives` external crate

### Out of Scope

- Adding URL path segments (e.g. `/announce`) — hardcoded per protocol, not useful for identity
- Resolving bind address to concrete IP — `ServiceBinding` carries the configured address as-is
- Adding a `public_url` config field (tracked in #1417)
- Building an "internal connection URL" from resolved IP + path segment (future extension)
- Changing the tracker protocol types (UDP/HTTP protocol parsing)
- TLS certificate configuration

## Implementation Plan

| ID  | Status | Task                                                                     | Notes                                                        |
| --- | ------ | ------------------------------------------------------------------------ | ------------------------------------------------------------ |
| T1  | TODO   | Identify all places where bare `SocketAddr` is used for service identity | Logs, health check, metrics, events, server launchers        |
| T2  | TODO   | Replace `SocketAddr` with `ServiceBinding` in those places               | `ServiceBinding` already has `protocol()` + `bind_address()` |
| T3  | TODO   | Update startup logging to use `ServiceBinding`                           | Replace manual URL string construction                       |
| T4  | TODO   | Update health check info to include `ServiceBinding`                     | For issue #1409                                              |
| T5  | TODO   | Update metrics to use `ServiceBinding` scheme as a label                 | For issue #1403/#1414                                        |
| T6  | TODO   | Run `linter all` and tests                                               |                                                              |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and spec moved to `docs/issues/open/`

### Progress Log

- 2026-07-13 21:00 UTC - josecelano - Initial spec drafted
- 2026-07-14 00:00 UTC - josecelano - Narrowed scope: use existing `ServiceBinding` instead of bare `SocketAddr`; no new types; no external crate changes; no URL path segments. Deferred "internal connection URL" to future extension.

## Acceptance Criteria

- [ ] AC1: `ServiceBinding` is used wherever bare `SocketAddr` was used for service identity
- [ ] AC2: Startup logs show protocol + address (e.g. `udp://0.0.0.0:6969`) via `ServiceBinding`
- [ ] AC3: Health check endpoint exposes `ServiceBinding` per service
- [ ] AC4: Metrics include the protocol scheme as a label (from `ServiceBinding`)
- [ ] AC5: No new config field required — `ServiceBinding` is derived from scheme + bind_address
- [ ] AC6: No changes to `torrust-net-primitives` external crate
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

| ID  | Scenario                          | Command/Steps                                 | Expected Result                            | Status | Evidence |
| --- | --------------------------------- | --------------------------------------------- | ------------------------------------------ | ------ | -------- |
| M1  | Verify listen URL in startup logs | Run tracker locally, check startup log output | Logs show `udp://0.0.0.0:6969` etc.        | TODO   |          |
| M2  | Verify listen URL in health check | `curl http://127.0.0.1:1313/health`           | Response includes `listen_url` per service | TODO   |          |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |
| AC5   | TODO   |          |

## Risks and Trade-offs

- **Scope creep**: This issue touches many packages (server launchers, health check, metrics). Mitigation: keep changes focused on replacing `SocketAddr` with `ServiceBinding` — no refactoring of how addresses are consumed.
- **No external crate changes**: `ServiceBinding` from `torrust-net-primitives` is used as-is. No coordinated release needed.

## References

- Related issues: #1409 (health check), #1403/#1414 (metrics)
- Related: `packages/tracker-core/src/lib.rs`
- Related: `packages/http-core/src/container.rs`
- Related: `packages/udp-server/src/server/launcher.rs`
