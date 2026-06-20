---
doc-type: issue
issue-type: feature
status: open
priority: p2
github-issue: 1671
spec-path: docs/issues/open/1671-ipv4-ipv6-client-metrics.md
branch: "1671-ipv4-ipv6-client-metrics"
related-pr: null
last-updated-utc: 2026-06-19 10:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/server/bound_socket.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-tracker-core/src/event.rs
    - packages/http-tracker-core/src/event.rs
    - packages/axum-http-server/src/server.rs
    - packages/configuration/src/v2_0_0/udp_tracker.rs
    - packages/configuration/src/v2_0_0/http_tracker.rs
---

<!-- skill-link: create-issue -->

# Issue #1671 - IPv4/IPv6 client metrics: support per-client IP family labels and separate socket bindings

## Goal

Enable the tracker to distinguish IPv4 clients from native IPv6 clients in Prometheus metrics by:

1. **(Investigate, then implement)** Verifying and enabling separate IPv4/IPv6 socket bindings so the tracker can bind two instances of the same service on the same port — one to `0.0.0.0:<port>` (IPv4-only) and one to `[::]:<port>` (IPv6-only).
2. **Add client address labels** to per-request metric counters so Grafana dashboards can split traffic by client IP family (`inet`/`inet6`) and address type (`plain`/`v4_mapped_v6`) without requiring separate socket bindings.

## Background

The tracker's Prometheus metrics currently have no way to distinguish IPv4 clients from native IPv6 clients. This was discovered when rebuilding Grafana dashboards for the multi-protocol dual-stack demo deployment ([torrust-tracker-demo#6](https://github.com/torrust/torrust-tracker-demo/issues/6)).

All tracker services in the demo bind to `[::]` (the IPv6 wildcard), which on Linux with the default kernel setting (`net.ipv6.bindv6only = 0`) causes a single dual-stack socket to accept both IPv4 and IPv6 clients. IPv4 clients are transparently handled by the kernel via IPv4-mapped IPv6 addresses (`::ffff:<ipv4>`), defined in [RFC 4291 §2.5.5.2](https://datatracker.ietf.org/doc/html/rfc4291#section-2.5.5.2).

The core problem is:

1. The existing `server_binding_address_ip_family` label is always `inet6` (it describes the server socket, not the connecting client).
2. The existing `server_binding_address_ip_type` label is also server-side and is always `plain` in a dual-stack setup.

Issue [#1375](https://github.com/torrust/torrust-tracker/issues/1375) introduced `server_binding_address_ip_type` but did not include a client-side counterpart.

## Scope

### In Scope

- **Task 1 — Separate socket bindings (investigative → implementation):**
  - Investigate whether the tracker needs to set `IPV6_V6ONLY` on IPv6 sockets before `bind()`.
  - Experimentally verify that two tracker instances can coexist on the same port (`0.0.0.0` + `[::]`) with `IPV6_V6ONLY = 1` (or `net.ipv6.bindv6only = 1` in a container).
  - If confirmed working, implement `IPV6_V6ONLY` socket option setting in the UDP and HTTP server socket creation paths.
  - If `IPV6_V6ONLY` alone is insufficient for dual-instance per-service bindings, document the limitation and note alternative approaches.
  - Confirm `server_binding_address_ip_family` correctly reports `inet`/`inet6` for separate sockets.

- **Task 2 — Client address labels:**
  - Add `client_address_ip_family` label (values: `inet`, `inet6`) to per-request metric counters.
  - Add `client_address_ip_type` label (values: `plain`, `v4_mapped_v6`) to per-request metric counters.
  - Instrument all per-request counters that already carry server binding labels (UDP + HTTP trackers).
  - Derive the client address type from the connecting client's socket address — reusing the existing `IpType` semantics.

### Out of Scope

- Adding raw client IP or port as metric labels (unbounded cardinality — never).
- Instrumenting global/aggregate counters (`swarm_coordination_registry_*`, `tracker_core_persistent_*`) — they lack a per-request context.
- Changing the configuration schema (bind addresses stay as single `SocketAddr` per instance; dual-instance is achieved via configuration, not a new config field).
- Modifying server binding labels or removing existing server-side labels.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

### Task 1 — Separate Socket Bindings

| ID  | Status | Task                                                              | Notes / Expected Output                                                                          |
| --- | ------ | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Investigate `IPV6_V6ONLY` socket option in socket creation paths  | Check `BoundSocket::new` (UDP) and `axum-http-server/src/server.rs` (TCP) for `IPV6_V6ONLY`      |
| T2  | TODO   | Set up Docker Compose dual-stack test environment                 | Container with both `0.0.0.0`:port and `[::]:port` bindings for same service                     |
| T3  | TODO   | Run investigation experiments and document findings               | Confirm whether `IPV6_V6ONLY` + dual-bind works; capture errors and metrics output                |
| T4  | TODO   | Implement `IPV6_V6ONLY` setting in UDP socket creation            | Modify `packages/udp-server/src/server/bound_socket.rs`                                          |
| T5  | TODO   | Implement `IPV6_V6ONLY` setting in HTTP/TCP socket creation       | Modify `packages/axum-http-server/src/server.rs`                                                 |
| T6  | TODO   | Verify separate socket metrics labels                             | Confirm `server_binding_address_ip_family` is `inet` for IPv4 socket, `inet6` for IPv6 socket     |

### Task 2 — Client Address Labels

| ID  | Status | Task                                                                       | Notes / Expected Output                                                             |
| --- | ------ | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| T7  | TODO   | Add client address helper to `ConnectionContext` types                     | Add `client_address_ip_family()` and `client_address_ip_type()` helpers to context  |
| T8  | TODO   | Add client labels to `ConnectionContext → LabelSet` conversion (UDP server) | Modify `packages/udp-server/src/event.rs` `From<ConnectionContext> for LabelSet`    |
| T9  | TODO   | Add client labels to `ConnectionContext → LabelSet` conversion (UDP core)   | Modify `packages/udp-tracker-core/src/event.rs`                                     |
| T10 | TODO   | Add client labels to `ConnectionContext → LabelSet` conversion (HTTP core)  | Modify `packages/http-tracker-core/src/event.rs`                                    |
| T11 | TODO   | Add tests for client address label derivation                              | Unit tests for `client_address_ip_type` derivation from `IpAddr`                    |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue number added to this spec (already #1671)
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-19 10:00 UTC - Copilot - Created draft spec for issue #1671

## Acceptance Criteria

- [ ] AC1: Tracker can bind two instances of the same service to the same port — one on `0.0.0.0` and one on `[::]` — after `IPV6_V6ONLY` is set (or workaround documented if impossible).
- [ ] AC2: `server_binding_address_ip_family` correctly reports `inet` for an IPv4-only socket and `inet6` for an IPv6-only socket when separate bindings are used.
- [ ] AC3: Client-side labels `client_address_ip_family` and `client_address_ip_type` are present on all per-request metric counters for both UDP and HTTP trackers.
- [ ] AC4: `client_address_ip_type` correctly distinguishes `plain` IPv4/native IPv6 addresses from `v4_mapped_v6` addresses.
- [ ] AC5: UDP connection IDs issued by the IPv4 socket are not visible to the IPv6 socket and vice versa (no cross-socket mismatches).
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features`
- Relevant unit tests for `ConnectionContext` and label derivation

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

#### Local Testing Setup

Use Docker Compose to create a controlled dual-stack environment:

1. A `docker-compose.yml` with two tracker instances per service:
   - `tracker-udp-ipv4`: binds to `0.0.0.0:6969`
   - `tracker-udp-ipv6`: binds to `[::]:6969`
   - `tracker-http-ipv4`: binds to `0.0.0.0:7070`
   - `tracker-http-ipv6`: binds to `[::]:7070`
2. Use `IPV6_V6ONLY=1` (via `IPV6_V6ONLY` socket option in code or container-level `sysctl`).
3. Health check endpoints and Prometheus metrics endpoint to verify label values.

| ID  | Scenario                                                                    | Command/Steps                                                                                                                                       | Expected Result                                                                           | Status | Evidence |
| --- | -------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Investigate: check current `IPV6_V6ONLY` status                            | Inspect socket options on a `[::]` bound socket before and after `bind()` in `BoundSocket::new`                                                     | Confirm `IPV6_V6ONLY` is currently `0` (not set) — document as investigation baseline     | TODO   |          |
| M2  | Investigate: attempt dual-bind without `IPV6_V6ONLY`                       | Run two tracker containers, one binding `0.0.0.0:6969`, one binding `[::]:6969`, without any `IPV6_V6ONLY` change                                   | Second bind fails with `EADDRINUSE` — document the expected OS-level error                | TODO   |          |
| M3  | Implement: dual-bind with `IPV6_V6ONLY=1` (UDP)                            | Run two tracker containers after implementing `IPV6_V6ONLY=1` for UDP sockets                                                                       | Both containers bind successfully; `server_binding_address_ip_family` correctly split     | TODO   |          |
| M4  | Implement: dual-bind with `IPV6_V6ONLY=1` (HTTP)                           | Run two tracker containers after implementing `IPV6_V6ONLY=1` for HTTP TCP sockets                                                                  | Both containers bind successfully; `server_binding_address_ip_family` correctly split     | TODO   |          |
| M5  | Verify UDP cross-socket isolation                                           | Send announce from an IPv4 client to the IPv4 socket; send the same announce to the IPv6 socket with the same connection ID                         | Announce to wrong socket fails with connection ID error                                   | TODO   |          |
| M6  | Verify client address labels in metrics (dual-stack socket)                | Run a single tracker on `[::]` (current default), connect with IPv4 and native IPv6 clients, inspect Prometheus metrics                              | `client_address_ip_family` shows `inet` for v4-mapped clients and `inet6` for native v6   | TODO   |          |
| M7  | Verify client address labels in metrics (separate sockets)                 | Run the dual-bind setup from M3+M4, connect IPv4 client → IPv4 socket, native IPv6 client → IPv6 socket, inspect metrics                             | Labels show correct split and server/client sides are consistent                          | TODO   |          |
| M8  | Verify `client_address_ip_type` derivation                                 | Connect with a real IPv4 address (gets v4-mapped as `::ffff:a.b.c.d`), native IPv6, and direct IPv4 (if using separate socket)                      | `plain` for direct IPv4/native IPv6, `v4_mapped_v6` for v4-mapped addresses               | TODO   |          |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.
- All manual tests should be run in a Docker container with controlled `net.ipv6.bindv6only` setting, not on the host system.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   |                    |
| AC2   | TODO                   |                    |
| AC3   | TODO                   |                    |
| AC4   | TODO                   |                    |
| AC5   | TODO                   |                    |

## Risks and Trade-offs

- **`IPV6_V6ONLY` approach may not work on all platforms**: macOS and some BSDs behave differently. Mitigation: target Linux as primary platform (consistent with CI and demo deployment); document platform-specific notes.
- **Dual-instance per-service is more complex than single-instance dual-stack**: Operating two tracker processes per service doubles operational overhead. Mitigation: Task 2 (client labels) works regardless and is the primary fix for Grafana visibility — dual-binding is complementary for cases where strict IPv4/IPv6 separation is needed (e.g., per-family rate limiting).
- **Setting `IPV6_V6ONLY` changes socket semantics for all IPv6 binds**: This is a one-line change but broad in effect. Mitigation: keep the change minimal and tested.
- **Client IP type derivation from `SocketAddr` is straightforward but must handle edge cases**: An `IpAddr::V4` address is always `plain`; an `IpAddr::V6` address is `v4_mapped_v6` if it starts with `::ffff:0:0/96`, else `plain`. Mitigation: use a well-defined helper function with unit tests.

## References

- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1671
- [#1375](https://github.com/torrust/torrust-tracker/issues/1375) — Original issue that added `server_binding_address_ip_type`
- [torrust-tracker-demo#6](https://github.com/torrust/torrust-tracker-demo/issues/6) — Rebuild Grafana Dashboards for new dual-stack deployment
- [ADR-001: Dual-stack socket vs separate sockets](https://github.com/torrust/torrust-tracker-demo/blob/main/docs/adr/ADR-001-dual-stack-socket-vs-separate-ipv4-ipv6-sockets.md)
- [Docker IPv6 documentation](https://github.com/torrust/torrust-tracker-demo/blob/main/docs/docker-ipv6.md)
- [RFC 4291 §2.5.5.2: IPv4-mapped IPv6 addresses](https://datatracker.ietf.org/doc/html/rfc4291#section-2.5.5.2)
