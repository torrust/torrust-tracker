# Event-Metrics Normalization Evidence

## Planned Baseline Probes

| ID  | Issue phase   | Configuration                                                                               | Expected baseline                                                                             | Status |
| --- | ------------- | ------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- | ------ |
| B1  | #2035         | HTTP listeners on distinct fixed ports with statistics disabled then enabled                | Aggregate HTTP announces are `1`.                                                             | TODO   |
| B2  | #2039         | UDP listeners on distinct fixed ports with statistics disabled then enabled                 | Aggregate UDP announces are currently `2`; #2039 must change this to `1`.                     | TODO   |
| B3  | #2035 + #2039 | HTTP and UDP listeners both configured as `0.0.0.0:0` with disabled then enabled statistics | Deferred until bootstrap identity propagation and listener-side filtering are both available. | TODO   |

## Evidence Records

Add the exact tracker configuration, commands, observed REST statistics, and
post-change comparison for each probe here. Do not overwrite baseline evidence.

### C1 baseline — fixed-port policy behavior (M1, M2, M5)

**Revision:** `e6b99635` (pre-implementation `develop`)

**Result:** DONE

The historical revision was run in an isolated temporary Git worktree with the
same fixed-port configuration now preserved at
[`evidence/fixed-port-manual.toml`](evidence/fixed-port-manual.toml), except for
an isolated temporary SQLite path.

One HTTP and one UDP announce was sent to each disabled and enabled listener,
using the same info hash and commands as the C1 post-change probe.

#### Observed output

- Before traffic: `tcp4_announces_handled: 0`, `udp4_announces_handled: 0`
- After traffic:
  - `tcp4_announces_handled: 2`
  - `udp4_announces_handled: 2`
  - `udp4_requests: 4`
  - `udp4_connections_handled: 2`
  - `udp4_responses: 4`

The disabled listeners incorrectly updated the shared aggregates. Compared with
the C1 post-change counts of `1`, the expected correction is verified.

### C1 post-change — fixed-port HTTP and UDP policy filtering (M1, M2, M5)

**Task:** T2-T6: canonical identity, always-published facts, and listener-side filtering

**Phase:** Post-change
**Result:** DONE

#### Configuration

- File: [`evidence/fixed-port-manual.toml`](evidence/fixed-port-manual.toml)
- HTTP: `127.0.0.1:17091` (disabled) and `127.0.0.1:17092` (enabled)
- UDP: `127.0.0.1:17093` (disabled) and `127.0.0.1:17094` (enabled)
- REST API: `127.0.0.1:17100`

#### Runtime endpoints

- `HttpTracker:0` → `http://127.0.0.1:17091/`
- `HttpTracker:1` → `http://127.0.0.1:17092/`
- `UdpTracker:0` → `udp://127.0.0.1:17093`
- `UdpTracker:1` → `udp://127.0.0.1:17094`

Startup logs confirmed the listed configuration instance identities and final
listener bindings.

#### Commands

Started the tracker with:

```sh
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/docs/issues/open/2039-normalize-per-instance-event-metrics-policy/evidence/fixed-port-manual.toml" \
  cargo +nightly run --bin torrust-tracker
```

Queried `GET /api/v1/stats` using the configured bearer token before and after
one `tracker_client http announce` request to each HTTP endpoint and one
`tracker_client udp announce` request to each UDP endpoint. Every announce used
the info hash `9c8b2213e30bff212b0c360d26f9a02131642200` and event `started`.

#### Observed output

- Baseline: `tcp4_announces_handled: 0`, `udp4_announces_handled: 0`
- After four successful announces:
  - `tcp4_announces_handled: 1`
  - `udp4_announces_handled: 1`
  - `udp4_connections_handled: 1`
  - `udp4_requests: 2`

Exactly one enabled listener contributed to each shared aggregate announce
counter. The disabled listeners remained functional but did not update those
aggregates.

#### Automated coverage

`cargo +nightly test --test aggregate_stats_fixed_ports --test aggregate_stats_port_zero -- --test-threads=1` passed.

The pre-change baseline was not captured before implementation began. The known
pre-change regression is retained in the issue background and covered by the
updated integration tests; this manual record is post-change evidence only.

### C1 post-change — repeated port-zero identity (M4)

**Task:** T2-T6: canonical identity, always-published facts, and listener-side filtering

**Phase:** Post-change
**Result:** DONE

#### Configuration

- File: [`evidence/port-zero-manual.toml`](evidence/port-zero-manual.toml)
- HTTP and UDP listeners: `127.0.0.1:0`
- Configuration order: disabled instance `0`, then enabled instance `1`
- REST API: `127.0.0.1:17100`

#### Runtime endpoints

- `HttpTracker:0` (disabled) → `http://127.0.0.1:35969/`
- `HttpTracker:1` (enabled) → `http://127.0.0.1:35285/`
- `UdpTracker:0` (disabled) → `udp://127.0.0.1:49864`
- `UdpTracker:1` (enabled) → `udp://127.0.0.1:48087`

The final bindings were mapped from startup logs to their configuration instance
identities; they were not inferred from the shared configured address.

#### Commands

Started the tracker with:

```sh
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/docs/issues/open/2039-normalize-per-instance-event-metrics-policy/evidence/port-zero-manual.toml" \
  cargo +nightly run --bin torrust-tracker
```

Queried `GET /api/v1/stats` before and after one `tracker_client http announce`
and one `tracker_client udp announce` request to each log-discovered endpoint.
Every announce used info hash `9c8b2213e30bff212b0c360d26f9a02131642200` and
event `started`.

#### Observed output

- Baseline: `tcp4_announces_handled: 0`, `udp4_announces_handled: 0`
- After four successful announces:
  - `tcp4_announces_handled: 1`
  - `udp4_announces_handled: 1`
  - `udp4_connections_handled: 1`
  - `udp4_requests: 2`

Despite identical configured socket addresses, only listeners identified as
configuration instance `1` updated aggregate metrics. This proves policy
filtering uses canonical configuration identity rather than a configured
address.

#### Automated coverage

`cargo +nightly test --test aggregate_stats_port_zero -- --test-threads=1` passed.

### C2 automated — banning remains independent of metrics policy (M3)

**Task:** T7: preserve banning independence

**Phase:** Post-change
**Result:** DONE

#### Scenario

`tests/aggregate_stats_fixed_ports.rs` starts the full application with the
first UDP listener metrics-disabled at `0.0.0.0:17093`. It sends eleven invalid
UDP announce requests with connection ID `0` from one client socket, then sends
a twelfth request from the same socket.

#### Observed output

- Each of the first eleven invalid-cookie requests receives the expected UDP
  cookie-error response.
- The twelfth request receives no response because the shared ban service has
  banned the client IP.
- The REST statistics endpoint reports `udp_banned_ips_total: 1`.

#### Automated coverage

`cargo +nightly test --test aggregate_stats_fixed_ports -- --test-threads=1`
passed, including
`udp_metrics_disabled_tracker_should_still_enforce_cookie_error_bans`.

This is full-application regression coverage, rather than an operator-run
manual probe. A manually reproducible forged-cookie client remains unavailable
from the public `tracker_client` CLI.

### C3 final application confirmation (M1, M2, M4, M5)

**Result:** DONE

The final committed application test suite repeated fixed-port and repeated
port-zero enabled/disabled traffic scenarios:

```sh
cargo +nightly test --test aggregate_stats_fixed_ports --test aggregate_stats_port_zero -- --test-threads=1
```

Both tests passed. They assert HTTP and UDP aggregate announce counts of `1`;
the fixed-port test also asserts retained UDP operational metrics for the
enabled listener: requests `2`, connections `1`, responses `2`, errors `0`,
and banned requests `0` before the independent banning scenario.

## Purpose

This file records the progressive manual baseline and post-change probes
required by the draft specification. Each code-changing task must have one
entry before its change and one entry after it.

## Entry Format

| Field              | Record                                                     |
| ------------------ | ---------------------------------------------------------- |
| Task               | Implementation task identifier and title                   |
| Phase              | `baseline` or `post-change`                                |
| Configuration      | Complete isolated tracker configuration or its stable path |
| Endpoints          | Final listener bindings used by the probe                  |
| Commands           | Exact commands or client interactions                      |
| Observed output    | Relevant counters, responses, and ban behavior             |
| Expected delta     | Intended difference from baseline, if any                  |
| Automated coverage | Focused tests run for the task                             |
| Result             | `DONE`, `FAILED`, or `BLOCKED`, with diagnosis             |

## Task Evidence Matrix

| Task | Baseline | Post-change | Result                                                                         |
| ---- | -------- | ----------- | ------------------------------------------------------------------------------ |
| T2   | DONE     | DONE        | Fixed-port baseline and fixed-port/port-zero post-change evidence recorded.    |
| T3   | DONE     | DONE        | Fixed-port baseline and post-change evidence recorded.                         |
| T4   | DONE     | DONE        | Fixed-port baseline and post-change evidence recorded.                         |
| T5   | DONE     | DONE        | Fixed-port baseline and post-change evidence recorded.                         |
| T6   | DONE     | DONE        | Fixed-port baseline and post-change evidence recorded.                         |
| T7   | Missing  | PARTIAL     | Full-application automated M3 coverage recorded; manual probe remains blocked. |
| T8   | Missing  | DONE        | REST announce and deterministic UDP operational-counter assertions verified.   |
| T9   | N/A      | DONE        | Focused and full-application regressions added.                                |
| T10  | N/A      | DONE        | Fixed-port and repeated-port-zero regressions pass.                            |

## Required Probe Outcomes

Every applicable baseline and post-change record must state whether:

- traffic from an enabled listener changes aggregate metrics;
- traffic from a disabled listener changes aggregate metrics; and
- UDP cookie errors from a disabled listener reach shared ban enforcement.

The post-change record must also state how the probe identifies repeated
port-zero listeners without relying on their configured socket address.
