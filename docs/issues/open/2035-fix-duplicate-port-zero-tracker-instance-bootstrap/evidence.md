# Bootstrap Collision Evidence

## Purpose

Demonstrate the current HTTP bootstrap defect before implementing the fix: duplicate configured
`0.0.0.0:0` bindings overwrite the first instance container, so both started listeners use the
second configuration block.

## Environment

- Repository: `torrust/torrust-tracker`
- Working branch: `1419-allow-multiple-integration-tests`
- Execution date: `2026-07-28`
- Required tools: Rust/Cargo and a writable `/tmp` directory

No network access, external tracker, generated certificate, or source-file change was retained
after this reproduction.

## Reproduction Configuration

The following complete configuration was written to
`/tmp/torrust-1419-bootstrap-evidence/tracker.toml`:

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "2.0.0"

[logging]
threshold = "debug"

[core]
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "/tmp/torrust-1419-bootstrap-evidence/storage/sqlite3.db"

[[http_trackers]]
bind_address = "0.0.0.0:0"
tracker_usage_statistics = true

[[http_trackers]]
bind_address = "0.0.0.0:0"
tracker_usage_statistics = false

[http_api]
bind_address = "127.0.0.1:0"

[http_api.access_tokens]
admin = "evidence-token"

[health_check_api]
bind_address = "127.0.0.2:0"
```

## Temporary Instrumentation

The following temporary debug events were added solely for this reproduction. They were removed
immediately after recording the output and are not part of the working tree.

In `src/container.rs`, the HTTP configuration loop was temporarily changed to enumerate entries,
capture the return value from `HashMap::insert`, and emit:

```rust
tracing::debug!(
    index,
    bind_address = %http_tracker_config.bind_address,
    tracker_usage_statistics = http_tracker_config.tracker_usage_statistics,
    replaced = replaced.is_some(),
    "Initialized HTTP tracker instance container"
);
```

In `src/app.rs`, immediately after retrieving the HTTP container for a configuration entry, this
temporary event was emitted:

```rust
tracing::debug!(
    index = idx,
    bind_address = %http_tracker_config.bind_address,
    configured_tracker_usage_statistics = http_tracker_config.tracker_usage_statistics,
    container_tracker_usage_statistics = http_tracker_container.http_tracker_config.tracker_usage_statistics,
    "Starting HTTP tracker instance"
);
```

## Commands Executed

From the repository root, the configuration directory and file were created, then the tracker was
started with that file:

```sh
mkdir -p /tmp/torrust-1419-bootstrap-evidence/storage
printf '%s\n' \
  '[metadata]' \
  'app = "torrust-tracker"' \
  'purpose = "configuration"' \
  'schema_version = "2.0.0"' \
  '' \
  '[logging]' \
  'threshold = "debug"' \
  '' \
  '[core]' \
  'listed = false' \
  'private = false' \
  '' \
  '[core.database]' \
  'driver = "sqlite3"' \
  'path = "/tmp/torrust-1419-bootstrap-evidence/storage/sqlite3.db"' \
  '' \
  '[[http_trackers]]' \
  'bind_address = "0.0.0.0:0"' \
  'tracker_usage_statistics = true' \
  '' \
  '[[http_trackers]]' \
  'bind_address = "0.0.0.0:0"' \
  'tracker_usage_statistics = false' \
  '' \
  '[http_api]' \
  'bind_address = "127.0.0.1:0"' \
  '' \
  '[http_api.access_tokens]' \
  'admin = "evidence-token"' \
  '' \
  '[health_check_api]' \
  'bind_address = "127.0.0.2:0"' \
  > /tmp/torrust-1419-bootstrap-evidence/tracker.toml

TORRUST_TRACKER_CONFIG_TOML_PATH=/tmp/torrust-1419-bootstrap-evidence/tracker.toml cargo run
```

After recording the output, the tracker process was terminated and both temporary source edits
were removed. The final verification command was:

```sh
git diff -- src/app.rs src/container.rs
```

It produced no output, confirming the probe did not remain in production code.

## Observed Output

Cargo rebuilt the tracker successfully and started `target/debug/torrust-tracker`. The tracker
loaded both HTTP blocks exactly as configured. The following complete set of discriminator lines
was emitted during bootstrap and startup:

```text
Initialized HTTP tracker instance container index=0 bind_address=0.0.0.0:0 tracker_usage_statistics=true replaced=false
Initialized HTTP tracker instance container index=1 bind_address=0.0.0.0:0 tracker_usage_statistics=false replaced=true
Starting HTTP tracker instance index=0 bind_address=0.0.0.0:0 configured_tracker_usage_statistics=true container_tracker_usage_statistics=false
HTTP TRACKER: Started on: http://0.0.0.0:33439
Starting HTTP tracker instance index=1 bind_address=0.0.0.0:0 configured_tracker_usage_statistics=false container_tracker_usage_statistics=false
HTTP TRACKER: Started on: http://0.0.0.0:33983
```

The normal tracker output also showed that the REST API and health check API started successfully;
their output is not relevant to this defect and is omitted above. The compile progress, metrics,
database migration diagnostics, and unrelated service logs are likewise omitted because they do
not affect the configuration-collision result.

## Result

The `replaced=true` result proves that the second configuration entry overwrote the first in the
address-keyed map. The first startup record proves that configuration index `0` was started using
the surviving container from index `1`. Distinct runtime ports do not preserve the lost
configuration-instance identity.

This run used temporary instrumentation only. No production debug statements remain after the
evidence capture.

## Automated Regression Evidence

The application-level regression
`the_stats_api_endpoint_should_exclude_announces_from_a_tracker_with_statistics_disabled` now
captures the same defect without temporary production instrumentation. It configures two HTTP
trackers with `0.0.0.0:0`: the first disables usage statistics and the second enables them. It
announces once to each listener and expects the global `tcp4_announces_handled` counter to be `1`.

The regression is intentionally ignored until this issue is implemented so the regular integration
suite remains green. It was run explicitly from the repository root with:

```sh
cargo test --test stats the_stats_api_endpoint_should_exclude_announces_from_a_tracker_with_statistics_disabled -- --ignored
```

The command compiled successfully, started the isolated application, and failed with:

```text
assertion `left == right` failed
  left: 2
 right: 1
```

The observed `2` shows that both listeners inherited the second configuration block's enabled
statistics setting. After the bootstrap fix, remove the `#[ignore]` attribute and the same test
must pass with the expected count of `1`.

## Final Port-Zero Verification

### Environment

- Revision: `4560c0403dfb4c7d9da5e3a9bd8c56fe1bf4f85d`
- Execution date: `2026-08-20`
- Configuration:
  [`../../2039-normalize-per-instance-event-metrics-policy/evidence-artifacts/port-zero-manual.toml`](../../2039-normalize-per-instance-event-metrics-policy/evidence-artifacts/port-zero-manual.toml)
- REST API: `http://127.0.0.1:17100/api/v1/stats?token=MyAccessToken`

The configuration defines two HTTP and two UDP listeners on `127.0.0.1:0`.
For both protocols, configuration instance `0` disables usage statistics and
instance `1` enables them.

### Commands

Started the tracker with:

```sh
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/docs/issues/open/2039-normalize-per-instance-event-metrics-policy/evidence-artifacts/port-zero-manual.toml" \
  cargo run --bin torrust-tracker
```

After reading the identity and final bindings from startup logs, queried REST
statistics before and after one announce to each listener:

```sh
curl -fsS 'http://127.0.0.1:17100/api/v1/stats?token=MyAccessToken'
cargo run -q -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:60889 9c8b2213e30bff212b0c360d26f9a02131642200
cargo run -q -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:37067 9c8b2213e30bff212b0c360d26f9a02131642200
cargo run -q -p torrust-tracker-client --bin tracker_client -- udp announce udp://127.0.0.1:35064 9c8b2213e30bff212b0c360d26f9a02131642200
cargo run -q -p torrust-tracker-client --bin tracker_client -- udp announce udp://127.0.0.1:58877 9c8b2213e30bff212b0c360d26f9a02131642200
curl -fsS 'http://127.0.0.1:17100/api/v1/stats?token=MyAccessToken'
```

Finally, ran the invalid-cookie probe through the metrics-disabled UDP listener:

```sh
python3 docs/issues/open/2039-normalize-per-instance-event-metrics-policy/evidence-artifacts/invalid_cookie_probe.py 127.0.0.1 35064
curl -fsS 'http://127.0.0.1:17100/api/v1/stats?token=MyAccessToken'
```

### Runtime Bindings

Startup logs mapped the canonical configuration instances to final bindings:

| Instance        | Metrics policy | Final binding             |
| --------------- | -------------- | ------------------------- |
| `HttpTracker:0` | Disabled       | `http://127.0.0.1:60889/` |
| `HttpTracker:1` | Enabled        | `http://127.0.0.1:37067/` |
| `UdpTracker:0`  | Disabled       | `udp://127.0.0.1:35064`   |
| `UdpTracker:1`  | Enabled        | `udp://127.0.0.1:58877`   |

Each listener accepted its announce request. Before traffic, both aggregate
announce counters were `0`. After all four announces, REST statistics reported:

```text
tcp4_announces_handled: 1
udp4_announces_handled: 1
udp4_requests: 2
udp4_connections_handled: 1
udp4_responses: 2
udp4_errors_handled: 0
udp_banned_ips_total: 0
```

The invalid-cookie probe printed:

```text
PASS: the twelfth invalid request timed out after shared ban enforcement
```

After that probe, REST reported `udp_banned_ips_total: 1`. The existing usage
metric values remained unchanged: `udp4_requests: 2`,
`udp4_announces_handled: 1`, and `udp4_errors_handled: 0`.

### Result

The duplicate port-zero listeners retained their own configuration and
canonical identity through startup. Metrics from instance `0` were filtered
from the shared aggregates while instance `1` contributed normally. Objective
UDP cookie-error facts from the metrics-disabled listener still reached shared
banning enforcement.

### Wildcard Binding Confirmation

The preceding probe used loopback bindings to simplify local connections. The
original collision applies specifically to repeated wildcard bindings, so the
same probe was repeated with
[`evidence-artifacts/wildcard-port-zero-manual.toml`](evidence-artifacts/wildcard-port-zero-manual.toml),
which configures every public listener as `0.0.0.0:0`.

Startup logs mapped each configured identity to a distinct final wildcard bind
socket address. The probe clients used the corresponding loopback endpoints:

| Instance        | Metrics policy | Bind socket address | Client endpoint           |
| --------------- | -------------- | ------------------- | ------------------------- |
| `HttpTracker:0` | Disabled       | `0.0.0.0:41223`     | `http://127.0.0.1:41223/` |
| `HttpTracker:1` | Enabled        | `0.0.0.0:39525`     | `http://127.0.0.1:39525/` |
| `UdpTracker:0`  | Disabled       | `0.0.0.0:39302`     | `udp://127.0.0.1:39302`   |
| `UdpTracker:1`  | Enabled        | `0.0.0.0:44277`     | `udp://127.0.0.1:44277`   |

One announce to
each listener produced `tcp4_announces_handled: 1`,
`udp4_announces_handled: 1`, `udp4_requests: 2`,
`udp4_connections_handled: 1`, and `udp4_responses: 2`. The invalid-cookie
probe against `UdpTracker:0` printed the expected twelfth-request ban result;
afterward `udp_banned_ips_total: 1`, while those usage values remained
unchanged.

### Automated Verification

The final focused regression suite passed with one test in each target:

```sh
cargo test \
  --test metrics-port-zero \
  --test metrics-fixed-ports \
  --test banning-udp-metrics-disabled-port-zero \
  --test metrics-udp-error-enabled-port-zero \
  --test metrics-udp-error-disabled-port-zero \
  --test scaffold \
  -- --test-threads=1
```
