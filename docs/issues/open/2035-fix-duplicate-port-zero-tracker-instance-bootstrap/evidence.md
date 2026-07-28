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
