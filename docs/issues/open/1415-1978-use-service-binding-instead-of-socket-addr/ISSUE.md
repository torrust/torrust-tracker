---
doc-type: issue
issue-type: enhancement
status: done
priority: p2
github-issue: 1415
spec-path: docs/issues/open/1415-1978-use-service-binding-instead-of-socket-addr/ISSUE.md
branch: "1415-use-service-binding"
related-pr: null
last-updated-utc: 2026-07-22 16:10
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-health-check-api-server/
    - packages/axum-http-server/
    - packages/axum-rest-api-server/
    - packages/http-core/src/event.rs
    - packages/udp-core/src/event.rs
    - packages/udp-server/src/server/launcher.rs
    - src/bootstrap/
    - manual-verification.md
---

# Issue #1415 - Use `ServiceBinding` instead of bare `SocketAddr` for service identity

> **EPIC position**: Subissue #5 of 11 in #1978. Independent of the remaining configuration
> subissues and does not add a configuration field.

## Goal

Use the existing `ServiceBinding` type from `torrust-net-primitives` wherever a service's
identity must include both protocol and bind address. This removes identity-related bare
`SocketAddr` plumbing while retaining the established public health-check and metrics contracts.

## Background

A `SocketAddr` alone cannot identify the protocol of a service. `ServiceBinding` models this
identity as a protocol plus bind address, is already used in domain events, and exposes
`protocol()` and `bind_address()`.

Completed work already made that identity visible to operators:

- #1409 / PR #1416 added health-check fields for a service binding and service type.
- #1403 / PR #1414 added the split `server_binding_*` Prometheus labels.
- #1417 adds optional public URLs to the v3 configuration schema, but runtime use of those URLs
  is not part of this issue.

The baseline verification in [`manual-verification.md`](manual-verification.md) confirms the
current health-check and metrics outputs. It also exposes an unresolved runtime-log gap: HTTP
tracker and REST API request logs still emit `server_socket_addr`, which loses protocol context.

## Scope

### In Scope

- Identify every remaining use of bare `SocketAddr` as a service identity in server launchers,
  request/startup logging, health-check registration, metrics, and domain events.
- Replace each identified identity flow with `ServiceBinding` without changing unrelated socket
  I/O interfaces.
- Preserve the established health-check `service_binding`, `binding`, and `service_type` fields.
- Preserve the established `server_binding_*` metric labels and ensure they are derived from the
  same `ServiceBinding` identity.
- Add focused regression tests for changed identity flows and the externally observable output.
- Run and record both baseline and post-implementation manual checks in
  [`manual-verification.md`](manual-verification.md).

### Out of Scope

- Adding URL path segments such as `/announce` to service identity.
- Resolving wildcard bind addresses to a concrete host IP.
- Adding, consuming, or exposing `public_url` configuration. Runtime observability integration
  is tracked by [#2023](../2023-1978-expose-configured-public-urls-in-runtime-observability.md).
- Adding an `internal_service_url`; it remains a future concept distinct from both
  `ServiceBinding` and `public_url`.
- Changing BitTorrent protocol parsing, TLS configuration, or `torrust-net-primitives`.
- Renaming or removing the existing health-check and metric fields unless separately approved.

## Current Baseline

The following was verified locally on 2026-07-22 before implementation:

- `GET /health_check` returns `service_binding` values such as
  `http://0.0.0.0:7070/` and `udp://0.0.0.0:6969`.
- An HTTP announce increments `http_tracker_core_requests_received_total` with
  `server_binding_ip`, `server_binding_port`, and `server_binding_protocol` labels.
- HTTP tracker and REST API request logs still include `server_socket_addr=0.0.0.0:<port>`.

The exact commands and complete relevant outputs are recorded in
[`manual-verification.md`](manual-verification.md).

## Implementation Plan

| ID  | Status | Task                                                   | Notes                                                                                                                                                                            |
| --- | ------ | ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Capture baseline manual verification                   | Health check, HTTP announce, and Prometheus metrics recorded before code changes.                                                                                                |
| T2  | DONE   | Inventory bare service-identity `SocketAddr` flows     | Audited server production paths; HTTP and REST API request/response logs plus UDP error logs were the remaining observable bare-address flows.                                   |
| T3  | DONE   | Replace remaining identity flows with `ServiceBinding` | Preserved public response and metric contracts.                                                                                                                                  |
| T4  | DONE   | Update runtime logging                                 | Retained `server_socket_addr` and added `service_binding` to HTTP, REST API, and UDP error logs.                                                                                 |
| T5  | DONE   | Run focused regression tests                           | Existing server-package tests cover the changed paths. Field-level log assertions are deferred to #1430 because global tracing state and concurrent output make them unreliable. |
| T6  | DONE   | Complete automatic and post-change manual verification | Recorded final commands and output in `manual-verification.md`.                                                                                                                  |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec reviewed and clarified with user/maintainer
- [x] GitHub issue exists and is linked to EPIC #1978
- [x] Baseline manual verification executed and recorded
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests)
- [x] Post-implementation manual verification executed and recorded
- [x] Acceptance criteria reviewed after implementation
- [ ] Issue closed and specification moved to `docs/issues/closed/`

### Progress Log

- 2026-07-13 21:00 UTC - josecelano - Initial specification drafted.
- 2026-07-14 00:00 UTC - josecelano - Narrowed scope to the existing `ServiceBinding` type;
  excluded new types, external crate changes, and URL path segments.
- 2026-07-22 11:00 UTC - agent - Started implementation branch `1415-use-service-binding`.
- 2026-07-22 12:50 UTC - agent - Ran baseline manual verification against a local tracker.
  Recorded health-check, announce, metrics, and relevant log evidence in
  `manual-verification.md`; converted the specification to folder form for evidence storage.
- 2026-07-22 13:15 UTC - agent - Confirmed that a wildcard bind on port `0` retains its wildcard
  address while the OS assigns the actual port after binding. Recorded `public_url` runtime
  observability as a separate draft follow-up.
- 2026-07-22 13:25 UTC - agent - Defined the #1415 runtime-log contract before implementation:
  HTTP tracker and REST API request/response logs add the protocol-aware `service_binding` field.
  The expected output is documented in `manual-verification.md`.
- 2026-07-22 13:30 UTC - josecelano - Confirmed that `server_socket_addr` is an existing public
  log contract and remains valid. #1415 keeps it for compatibility and adds `service_binding` as
  complementary protocol-aware information.
- 2026-07-22 13:35 UTC - agent - Recorded approved public-URL runtime observability follow-up as
  issue #2023.
- 2026-07-22 15:25 UTC - agent - Audited remaining production service-identity flows. Added
  `service_binding` alongside `server_socket_addr` to HTTP tracker and REST API request/response
  logs and UDP error logs. Verified the HTTP, REST API, and UDP output manually and passed
  focused, workspace, and lint checks. Field-level regression tests are still pending.
- 2026-07-22 15:35 UTC - josecelano - Accepted manual verification as the log-output evidence.
  Automated assertions for tracing output are deferred to #1430 because the global tracing
  subscriber and concurrent test output make deterministic field-level capture unreliable.
- 2026-07-22 16:10 UTC - josecelano - Clarified the post-bind identity contract: the retained
  `server_socket_addr` is derived from `ServiceBinding::bind_address()`. Both fields therefore
  report the same actual bound address, including an OS-assigned port when configuration uses
  port `0`.

## Acceptance Criteria

- [x] AC1: Every changed flow that represents a service identity uses `ServiceBinding` rather
      than a bare `SocketAddr`.
- [x] AC2: Changed HTTP tracker, REST API, and UDP error logs retain
      `server_socket_addr=<post-bind-address>` and add
      `service_binding=<protocol>://<post-bind-address>/`.
- [x] AC3: The health-check endpoint continues to expose protocol-aware `service_binding` data
      for each registered service.
- [x] AC4: An HTTP announce continues to produce metrics containing the protocol-aware
      `server_binding_*` label set.
- [x] AC5: No configuration field or `torrust-net-primitives` change is required.
- [x] AC6: `linter all` exits with code `0` and relevant tests pass.
- [x] AC7: The health-check, metric, and runtime-log post-implementation manual checks pass and
      their commands and output are
      recorded in `manual-verification.md`.

## Verification Plan

### Automatic Checks

- `linter all`
- Focused package tests for each changed package
- `cargo test --workspace`

### Manual Checks

| ID  | Scenario                                                               | Expected Result                                                                                                              | Evidence                                                                                                     |
| --- | ---------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| M1  | Run the tracker locally and call `GET /health_check`.                  | Every relevant service detail includes its protocol-aware `service_binding`.                                                 | [`manual-verification.md#m1-health-check`](manual-verification.md#m1-health-check)                           |
| M2  | Announce to the local HTTP tracker, then query Prometheus metrics.     | The HTTP announce metric contains `server_binding_ip`, `server_binding_port`, and `server_binding_protocol="http"`.          | [`manual-verification.md#m2-http-announce-and-metrics`](manual-verification.md#m2-http-announce-and-metrics) |
| M3  | Send an HTTP announce and make a REST API request; inspect their logs. | Changed records retain `server_socket_addr=<post-bind-address>` and add `service_binding=<protocol>://<post-bind-address>/`. | [`manual-verification.md#runtime-log-contract`](manual-verification.md#runtime-log-contract)                 |

## Risks and Trade-offs

- **Accidental API churn**: health-check and metrics representations already exist. Preserve
  their names and serialized shape unless a later design decision explicitly changes them.
- **Over-broad replacement**: `SocketAddr` remains appropriate for low-level binding and client
  network I/O. Replace it only where it models a service identity.
- **Log-consumer compatibility**: request and response logs are operational output. This issue
  preserves `server_socket_addr` and adds `service_binding`, avoiding a breaking log-schema
  change while providing protocol-aware service identity.
- **Post-bind address source**: `server_socket_addr` is derived from
  `ServiceBinding::bind_address()` in the changed flows. The two log fields always describe the
  same actual bound host and port; only `service_binding` adds protocol and URL formatting. If
  configuration requests port `0`, both fields use the OS-assigned port rather than `0`.
- **Tracing testability**: field-level assertions for concurrent tracing output are deferred to
  #1430. The manual verification evidence is the acceptance evidence for this issue's log schema.

## References

- #1409 and PR #1416 - health-check service binding output
- #1403 and PR #1414 - per-service labelled metrics
- #1417 - optional public service URL configuration
- #1430 - tracing log-capture test reliability
- [Manual verification evidence](manual-verification.md)
