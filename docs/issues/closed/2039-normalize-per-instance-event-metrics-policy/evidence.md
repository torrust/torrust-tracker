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
[`evidence-artifacts/fixed-port-manual.toml`](evidence-artifacts/fixed-port-manual.toml), except for
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

- File: [`evidence-artifacts/fixed-port-manual.toml`](evidence-artifacts/fixed-port-manual.toml)
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
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/docs/issues/closed/2039-normalize-per-instance-event-metrics-policy/evidence-artifacts/fixed-port-manual.toml" \
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

The aggregate-policy binaries passed: `metrics-fixed-ports`,
`metrics-port-zero`, `metrics-udp-error-enabled-port-zero`,
`metrics-udp-error-disabled-port-zero`, and
`banning-udp-metrics-disabled-port-zero`.

The fixed-port pre-change baseline was subsequently captured from isolated
revision `e6b99635` and is recorded above. The port-zero baseline is not
available because the prerequisite bootstrap identity work was not present in
that revision; its post-change regression test and manual evidence verify the
required final behavior.

### C1 post-change — repeated port-zero identity (M4)

**Task:** T2-T6: canonical identity, always-published facts, and listener-side filtering

**Phase:** Post-change
**Result:** DONE

#### Configuration

- File: [`evidence-artifacts/port-zero-manual.toml`](evidence-artifacts/port-zero-manual.toml)
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
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/docs/issues/closed/2039-normalize-per-instance-event-metrics-policy/evidence-artifacts/port-zero-manual.toml" \
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

`cargo +nightly test --test metrics-port-zero -- --test-threads=1` passed.

### C2 baseline — cookie errors from a metrics-disabled listener (M3)

**Revision:** `e6b99635` (pre-implementation `develop`)

The tracked probe was run from an isolated historical worktree against
`UdpTracker:0` at `127.0.0.1:17093`.

#### Observed output

- Before traffic: `udp_banned_ips_total: 0` and `udp4_errors_handled: 0`
- The probe received eleven cookie-error responses and the twelfth request
  timed out after ban enforcement.
- After traffic: `udp_banned_ips_total: 1`, `udp_requests_banned: 1`,
  `udp4_requests: 12`, `udp4_announces_handled: 11`,
  `udp4_responses: 11`, and `udp4_errors_handled: 11`.

The historical shared metrics listener aggregated cookie errors and request
events from the metrics-disabled listener. The intended post-change behavior
retains shared banning while excluding those usage metrics.

### C2 post-change — banning remains independent of metrics policy (M3)

**Task:** T7: preserve banning independence

**Phase:** Post-change
**Result:** DONE

#### Scenario

Started the final tracker build with
[`evidence-artifacts/fixed-port-manual.toml`](evidence-artifacts/fixed-port-manual.toml), then ran:

```sh
python3 evidence-artifacts/invalid_cookie_probe.py 127.0.0.1 17093
```

The probe retains one UDP socket and sends eleven invalid connection-ID
announces through metrics-disabled `UdpTracker:0`, followed by a twelfth request
from the same source address.

#### Observed output

- Each of the first eleven invalid-cookie requests receives the expected UDP
  cookie-error response.
- The probe printed: `PASS: the twelfth invalid request timed out after shared
ban enforcement`.
- The REST statistics endpoint reports `udp_banned_ips_total: 1`.
- REST UDP aggregate metrics remained zero (`udp4_requests: 0`,
  `udp4_announces_handled: 0`, and `udp4_errors_handled: 0`) because every
  probe request originated at the metrics-disabled listener.

#### Automated coverage

`cargo +nightly test --test banning-udp-metrics-disabled-port-zero -- --test-threads=1`
passed.

The tracked Python probe supplies the forged-cookie capability unavailable from
the public `tracker_client` CLI. The full-application regression remains
additional automated coverage.

### C3 final application confirmation (M1, M2, M4, M5)

**Result:** DONE

The final application test suite repeated fixed-port and repeated
port-zero enabled/disabled traffic scenarios:

```sh
cargo +nightly test \
  --test metrics-fixed-ports \
  --test metrics-port-zero \
  --test metrics-udp-error-enabled-port-zero \
  --test metrics-udp-error-disabled-port-zero \
  --test banning-udp-metrics-disabled-port-zero \
  -- --test-threads=1
```

All five explicit test binaries passed. They assert HTTP and UDP aggregate announce counts of `1`;
the fixed-port test also asserts retained UDP operational metrics for the
enabled listener: requests `2`, connections `1`, responses `2`, errors `0`,
and banned requests `0` before the independent banning scenario.

#### Manual fixed-port result

Using `evidence-artifacts/fixed-port-manual.toml`, one announce to each disabled and
enabled HTTP/UDP listener produced final REST values
`tcp4_announces_handled: 1`, `udp4_announces_handled: 1`,
`udp4_requests: 2`, `udp4_connections_handled: 1`, and
`udp4_responses: 2`.

#### Manual repeated-port-zero result

The final startup logs mapped `HttpTracker:0` and `UdpTracker:0` to the
disabled ephemeral bindings, and identity `1` to the enabled bindings. One
announce to each of the four final endpoints produced the same REST values:
`tcp4_announces_handled: 1`, `udp4_announces_handled: 1`,
`udp4_requests: 2`, `udp4_connections_handled: 1`, and
`udp4_responses: 2`.

## Purpose

This file records the baseline and final application probes required by the
issue specification. Intermediate observations remain useful diagnostics, but
the baseline-to-final comparison is the completion evidence.

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

| Task | Baseline | Post-change | Result                                                                       |
| ---- | -------- | ----------- | ---------------------------------------------------------------------------- |
| T2   | DONE     | DONE        | Fixed-port baseline and fixed-port/port-zero final evidence recorded.        |
| T3   | DONE     | DONE        | Fixed-port baseline and final evidence recorded.                             |
| T4   | DONE     | DONE        | Fixed-port baseline and final evidence recorded.                             |
| T5   | DONE     | DONE        | Fixed-port baseline and final evidence recorded.                             |
| T6   | DONE     | DONE        | Fixed-port baseline and final evidence recorded.                             |
| T7   | DONE     | DONE        | Manual M3 baseline/final probe and full-application coverage recorded.       |
| T8   | N/A      | DONE        | REST announce and deterministic UDP operational-counter assertions verified. |
| T9   | N/A      | DONE        | Focused and isolated full-application regressions added.                     |
| T10  | N/A      | DONE        | Fixed-port routing and port-zero policy binaries pass.                       |

## Required Probe Outcomes

Every applicable baseline and post-change record must state whether:

- traffic from an enabled listener changes aggregate metrics;
- traffic from a disabled listener changes aggregate metrics; and
- UDP cookie errors from a disabled listener reach shared ban enforcement.

The post-change record must also state how the probe identifies repeated
port-zero listeners without relying on their configured socket address.
