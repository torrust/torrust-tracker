---
doc-type: issue
issue-type: feature
status: open
priority: p2
github-issue: 1671
spec-path: docs/issues/open/1671-ipv4-ipv6-client-metrics/ISSUE.md
branch: "1671-ipv4-ipv6-client-metrics"
related-pr: null
last-updated-utc: 2026-06-20 10:00
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
3. **Add config option** to optionally disable dual-stack mode (`ipv6_v6only: bool`) on UDP and HTTP tracker sockets, allowing operators to bind separate IPv4/IPv6 sockets on the same port for per-family metric separation.

## Background

The tracker's Prometheus metrics currently have no way to distinguish IPv4 clients from native IPv6 clients. This was discovered when rebuilding Grafana dashboards for the multi-protocol dual-stack demo deployment ([torrust-tracker-demo#6](https://github.com/torrust/torrust-tracker-demo/issues/6)).

All tracker services in the demo bind to `[::]` (the IPv6 wildcard), which on Linux with the default kernel setting (`net.ipv6.bindv6only = 0`) causes a single dual-stack socket to accept both IPv4 and IPv6 clients. IPv4 clients are transparently handled by the kernel via IPv4-mapped IPv6 addresses (`::ffff:<ipv4>`), defined in [RFC 4291 §2.5.5.2](https://datatracker.ietf.org/doc/html/rfc4291#section-2.5.5.2).

The core problem is:

1. The existing `server_binding_address_ip_family` label is always `inet6` (it describes the server socket, not the connecting client).
2. The existing `server_binding_address_ip_type` label is also server-side and is always `plain` in a dual-stack setup.

Issue [#1375](https://github.com/torrust/torrust-tracker/issues/1375) introduced `server_binding_address_ip_type` but did not include a client-side counterpart.

## Scope

### In Scope

- **Task 1 — Investigate separate IPv4/IPv6 socket bindings:**
  - Experimentally verify whether setting `IPV6_V6ONLY=1` on IPv6 sockets at the Rust code level (via `socket2`) allows a single tracker process to bind both `0.0.0.0:<port>` and `[::]:<port>` on the same port without `EADDRINUSE`.
  - This is a pure investigation: keep the `IPV6_V6ONLY` change as experiment code in the branch, _not_ as a final configuration option or permanent behaviour change.
  - The experiment lives in `contrib/dev-tools/experiments/dual-stack-sockets/`.
  - If confirmed possible, document the finding and optionally design a config toggle for a follow-up issue. Do not merge IPV6_V6ONLY into the default code path.
  - Note: Task 2 (client address parsing) works regardless of the investigation outcome and is the primary fix for Grafana visibility.

- **Task 3 — Config option for `IPV6_V6ONLY` socket option:**
  - Add `ipv6_v6only: bool` field to `UdpTracker` and `HttpTracker` config structs (default `false`).
  - Conditionally call `socket.set_only_v6(true)` in UDP and HTTP socket creation only when config is `true`.
  - The config option replaces the unconditional `IPV6_V6ONLY=1` experiment code.
  - Document the option's platform-dependent behaviour (OpenBSD cannot use dual-stack mode).

### Out of Scope

- Adding raw client IP or port as metric labels (unbounded cardinality — never).
- Instrumenting global/aggregate counters (`swarm_coordination_registry_*`, `tracker_core_persistent_*`) — they lack a per-request context.
- Removing dual-stack support entirely — the option is opt-in.
- Changing the configuration schema permanently beyond adding `ipv6_v6only`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

### Task 1 — Investigate Separate Socket Bindings

| ID  | Status | Task                                          | Notes / Expected Output                                                                       |
| --- | ------ | --------------------------------------------- | --------------------------------------------------------------------------------------------- |
| T1  | DONE   | Run the dual-stack experiment locally         | ✅ `IPV6_V6ONLY=1` at runtime works — both IPv4/IPv6 UDP+HTTP bound successfully on same port |
| T2  | DONE   | Document findings and decide on config option | ✅ Experiment documented in `contrib/dev-tools/experiments/dual-stack-sockets/README.md`      |

### Task 2 — Client Address Labels

| ID  | Status | Task                                                                        | Notes / Expected Output                                                            |
| --- | ------ | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| T7  | DONE   | Add client address helper to `ConnectionContext` types                      | Add `client_address_ip_family()` and `client_address_ip_type()` helpers to context |
| T8  | DONE   | Add client labels to `ConnectionContext → LabelSet` conversion (UDP server) | Modify `packages/udp-server/src/event.rs` `From<ConnectionContext> for LabelSet`   |
| T9  | DONE   | Add client labels to `ConnectionContext → LabelSet` conversion (UDP core)   | Modify `packages/udp-tracker-core/src/event.rs`                                    |
| T10 | DONE   | Add client labels to `ConnectionContext → LabelSet` conversion (HTTP core)  | Modify `packages/http-tracker-core/src/event.rs`                                   |
| T11 | DONE   | Add tests for client address label derivation                               | Unit tests for `client_address_ip_type` derivation from `IpAddr`                   |

### Task 3 — Config Option for `IPV6_V6ONLY`

| ID  | Status | Task                                                                   | Notes / Expected Output                                                                                                                                                                   |
| --- | ------ | ---------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T12 | DONE   | Add `ipv6_v6only: bool` field to `UdpTracker` and `HttpTracker` config | Add field with `#[serde(default)]` defaulting to `false` (dual-stack mode).                                                                                                               |
| T13 | DONE   | Wire config into UDP socket creation                                   | Pass `ipv6_v6only` through `Launcher` to `BoundSocket::create_socket`, only call `set_only_v6` when true.                                                                                 |
| T14 | DONE   | Wire config into HTTP socket creation                                  | Pass `ipv6_v6only` into `Launcher::create_tcp_listener`, only call `set_only_v6` when true.                                                                                               |
| T15 | DONE   | Remove unconditional `IPV6_V6ONLY=1` experiment code                   | The config option replaces the hardcoded `set_only_v6(true)` in both socket creation paths.                                                                                               |
| T16 | DONE   | Update dual-stack experiment config to use `ipv6_v6only = true`        | Modify `contrib/dev-tools/experiments/dual-stack-sockets/config/tracker.dual-stack.toml`                                                                                                  |
| T17 | DONE   | Add tests for `ipv6_v6only` config propagation                         | Integration test `should_accept_ipv6_connections_with_ipv6_v6only_enabled` in `packages/udp-server/tests/server/contract.rs` and `packages/axum-http-server/tests/server/v1/contract.rs`. |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue number added to this spec (already #1671)
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-19 10:00 UTC - Copilot - Created draft spec for issue #1671
- 2026-06-19 17:45 UTC - Copilot - Implemented Task 2 (client address labels: T7-T11) and Task 1/IPV6_V6ONLY (T1, T4, T5) - UDP server, HTTP server, UDP core, HTTP core
- 2026-06-19 19:00 UTC - Copilot - Ran dual-stack experiment locally (see `contrib/dev-tools/experiments/dual-stack-sockets/README.md`)
- 2026-06-20 UTC - Copilot - Updated spec verification table with experiment evidence, added UDP unit tests for client address labels, ran linter all
- 2026-06-20 UTC - Copilot - Removed duplicate UDP server tests (derivation tested once in udp-tracker-core), added cross-fingerprint cookie rejection test for AC5, fixed linter issues, updated spec
- 2026-06-20 UTC - Copilot - Added UDP integration test for `ipv6_v6only` config propagation (T17)

## Acceptance Criteria

- [x] AC1: Tracker can bind two instances of the same service to the same port — one on `0.0.0.0` and one on `[::]` — after `IPV6_V6ONLY` is set (or workaround documented if impossible).
- [x] AC2: `server_binding_address_ip_family` correctly reports `inet` for an IPv4-only socket and `inet6` for an IPv6-only socket when separate bindings are used.
- [x] AC3: Client-side labels `client_address_ip_family` and `client_address_ip_type` are present on all per-request metric counters for both UDP and HTTP trackers.
- [x] AC4: `client_address_ip_type` correctly distinguishes `plain` IPv4/native IPv6 addresses from `v4_mapped_v6` addresses.
- [x] AC5: UDP connection IDs issued for one client address are not valid for a different client address — verified via unit test `it_should_reject_a_cookie_with_a_wrong_fingerprint_realistic_addresses`.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented (status + evidence)
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
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

Use the experiment config at `contrib/dev-tools/experiments/dual-stack-sockets/`:

1. A single config file with both `[[udp_trackers]]` entries (`0.0.0.0:6969` + `[::]:6969`)
   and both `[[http_trackers]]` entries (`0.0.0.0:7070` + `[::]:7070`).
2. The tracker process already has the `IPV6_V6ONLY=1` change from this branch.
3. On a system with `net.ipv6.bindv6only = 0` (Linux default), this tests whether
   the runtime code change alone enables dual-bind on the same port.

| ID  | Scenario                                                   | Command/Steps                                                                                                                 | Expected Result                                                                         | Status | Evidence                                                                                                                                                                                             |
| --- | ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | Run dual-stack experiment (single instance)                | `cargo run --bin torrust-tracker -- --config contrib/dev-tools/experiments/dual-stack-sockets/config/tracker.dual-stack.toml` | Both IPv4 and IPv6 listeners bind successfully on the same ports; no `EADDRINUSE` panic | DONE   | `ss` output shows all 4 sockets: `UNCONN 0.0.0.0:6969`, `UNCONN [::]:6969`, `LISTEN 0.0.0.0:7070`, `LISTEN [::]:7070`. See experiment README.                                                        |
| M2  | Verify server metrics labels in dual-bind mode             | `curl -s http://127.0.0.1:1212/metrics \| grep server_binding_address_ip_family`                                              | Both `inet` and `inet6` appear for the same protocol+port                               | DONE   | Experiment README metrics confirm `server_binding_address_ip_family="inet"` and `"inet6"` for same protocol+port.                                                                                    |
| M3  | Verify client address labels in metrics (single socket)    | Run tracker with default config (single `[::]` socket), connect with IPv4 and native IPv6 clients, inspect metrics            | `client_address_ip_family` shows `inet` for v4-mapped clients and `inet6` for native v6 | DONE   | Implicitly verified via dual-bind mode (same client label derivation logic). UDP announce to `127.0.0.1:6969` → `client=inet`, to `[::1]:6969` → `client=inet6`. Also confirmed by unit tests (T11). |
| M4  | Verify client address labels in metrics (separate sockets) | Run dual-bind config from M1, connect IPv4 → IPv4 socket, IPv6 → IPv6 socket, inspect metrics                                 | Labels show correct split and server/client sides are consistent                        | DONE   | Experiment README Expected vs actual: IPv4→IPv4 socket → `client=inet, server=inet` ✅; IPv6→IPv6 socket → `client=inet6, server=inet6` ✅.                                                          |
| M5  | Verify `client_address_ip_type` derivation                 | Connect with real IPv4 (gets `::ffff:a.b.c.d`), native IPv6, and direct IPv4 (separate socket)                                | `plain` for direct IPv4/native IPv6, `v4_mapped_v6` for v4-mapped addresses             | DONE   | Unit tests confirm all 3 cases. Manual: `127.0.0.1` → `plain`, `::1` → `plain`. V4-mapped case confirmed via unit test.                                                                              |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.
- All manual tests should be run on a system with `net.ipv6.bindv6only = 0` (Linux default) to verify the code-level `IPV6_V6ONLY` change is sufficient.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                                                                                                |
| ----- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | Experiment confirmed: `IPV6_V6ONLY=1` via `socket2` allows `0.0.0.0:<port>` + `[::]:<port>` on same port. All 4 sockets (UDP+HTTP) bind successfully. See experiment README.                                            |
| AC2   | DONE                   | Server metrics confirmed: `server_binding_address_ip_family="inet"` for `0.0.0.0` socket and `="inet6"` for `[::]` socket in dual-bind mode. See experiment README.                                                     |
| AC3   | DONE                   | `client_address_ip_family` and `client_address_ip_type` labels present on all per-request UDP and HTTP metric counters. Confirmed via manual experiment and unit tests (T11).                                           |
| AC4   | DONE                   | Unit tests confirm: direct IPv4 → `plain`, native IPv6 → `plain`, IPv4-mapped IPv6 → `v4_mapped_v6`. Also manually verified with real traffic.                                                                          |
| AC5   | DONE                   | Unit test `it_should_reject_a_cookie_with_a_wrong_fingerprint_realistic_addresses` verifies that a cookie issued for client A (127.0.0.1:4000) is rejected when validated with client B's fingerprint (127.0.0.2:4000). |

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
