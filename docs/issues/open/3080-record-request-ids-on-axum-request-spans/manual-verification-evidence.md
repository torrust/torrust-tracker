---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3080-record-request-ids-on-axum-request-spans/ISSUE.md
last-updated-utc: "2026-11-27 12:00"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of Issue #3080 ("Record request IDs on axum request spans") against the compiled `torrust-tracker` binary and focused unit/integration test suites.

## Preserving Verification Artifacts

`.tmp/` is git-ignored and not preserved in the repository. The temporary reproduction configuration and shell driver used during V1 and V2 are embedded verbatim below, alongside the verbatim terminal commands, HTTP responses, and relevant tracker log lines.

### Embedded Reproduction Configuration (`.tmp/repro-3080.toml`)

Based on `share/default/config/tracker.container.no-persistence.toml` with `trace_filter = "debug"` and `[http_api]` enabled:

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "debug"
trace_style = "full"

[core]
listed = false
private = false
tracker_usage_statistics = true

[core.tracker_policy]
persistent_torrent_completed_stat = false

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[application.shutdown]
deadline_in_secs = 25
http_drain_in_secs = 15
udp_request_drain_in_secs = 5
event_listener_drain_in_secs = 5

[[udp_trackers]]
bind_address = "0.0.0.0:6969"

[[http_trackers]]
bind_address = "0.0.0.0:7070"

[http_api]
bind_address = "0.0.0.0:1212"

[http_api.access_tokens]
admin = "MyAccessToken"

[health_check_api]
bind_address = "0.0.0.0:1313"
```

### Embedded Reproduction Driver (`.tmp/run-repro.sh`)

```bash
#!/usr/bin/env bash
set -euo pipefail

PHASE="${1:-before}"
LOG_RAW="/workspace/.tmp/repro-${PHASE}.raw.log"
LOG_CLEAN="/workspace/.tmp/repro-${PHASE}.log"

echo "=== Toolchain ==="
rustc --version
cargo --version

echo "=== Start tracker (${PHASE}) $(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) ==="
/tmp/target/debug/torrust-tracker --config-toml-path /workspace/.tmp/repro-3080.toml >"$LOG_RAW" 2>&1 &
TRACKER_PID=$!

cleanup() {
  if kill -0 "$TRACKER_PID" 2>/dev/null; then
    kill -TERM "$TRACKER_PID" 2>/dev/null || true
    wait "$TRACKER_PID" || true
  fi
}
trap cleanup EXIT

for _ in $(seq 1 50); do
  if curl -fsS http://127.0.0.1:1212/api/health_check >/dev/null 2>&1 \
     && curl -fsS http://127.0.0.1:7070/health_check >/dev/null 2>&1 \
     && curl -fsS http://127.0.0.1:1313/health_check >/dev/null 2>&1; then
    break
  fi
  sleep 0.2
done

echo "=== Ready $(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) ==="

echo "--- Request 1: REST API /api/v1/torrents start $(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) ---"
curl -sS -i -H "x-request-id: repro-3080-api-${PHASE}" -H "Authorization: Bearer MyAccessToken" "http://127.0.0.1:1212/api/v1/torrents"
echo ""
echo "--- Request 1 end $(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) ---"

echo "--- Request 2: HTTP Tracker /announce start $(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) ---"
curl -sS -D - -o /dev/null -H "x-request-id: repro-3080-http-${PHASE}" "http://127.0.0.1:7070/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0"
echo "--- Request 2 end $(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) ---"

echo "--- Request 3: Health Check API /health_check start $(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) ---"
curl -sS -i -H "x-request-id: repro-3080-hc-${PHASE}" "http://127.0.0.1:1313/health_check"
echo ""
echo "--- Request 3 end $(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) ---"

kill -TERM "$TRACKER_PID"
wait "$TRACKER_PID" || true
trap - EXIT
echo "=== Stopped $(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) ==="

sed -E 's/\x1B\[[0-9;]*[A-Za-z]//g' "$LOG_RAW" > "$LOG_CLEAN"
echo "=== Tracker Log ==="
cat "$LOG_CLEAN"
```

## Defect Under Test

- Symptom: Axum request spans (`request{method=... uri=... version=...}`) never include `request_id`, even when clients send `x-request-id` and all three Axum servers call `span.record("request_id", request_id)` in `on_request` and `on_response`.
- Hypothesis: `RedactedMakeSpan::make_span` in `packages/axum-server/src/request_log.rs` creates the `"request"` span without declaring a `request_id` field, so `tracing` silently discards subsequent `span.record("request_id", ...)` calls.
- Regression-test boundary: Unit test `it_should_record_the_request_id_in_the_span_of_a_request` in `packages/axum-server/src/request_log.rs` (`mod tests`), exercising `RedactedMakeSpan.make_span(&request)` plus `span.record("request_id", ...)` directly at the causal seam with `capture_logs_in_scope()`.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-27 11:46 to 11:57 UTC.
- Artifact under test: `upstream/develop` (`786b46f`, already on `develop`: `Merge torrust/torrust-tracker#3081: docs(issues): refresh the daemon JSON logging spec and record its decisions`), plus uncommitted changes on branch `3080-record-request-ids-on-axum-request-spans` corresponding to planned commits `test(axum-server): [#3080] add regression test for request_id on request span` and `fix(axum-server): [#3080] declare empty request_id field on RedactedMakeSpan`.
- Operating system / environment: Docker container `rust:1-bookworm` on Windows 11 host (Docker Engine 29.9.3), `rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo 1.100.0 (3acc79c0c 2026-11-05)`, stable Rust toolchain.
- Prerequisites and setup performed: Docker volumes `torrust-cargo-registry`, `torrust-cargo-git`, and `torrust-target` (`CARGO_TARGET_DIR=/tmp/target`); temporary config `.tmp/repro-3080.toml` and script `.tmp/run-repro.sh`.

## Verification Processes

### V1 - Before-Fix Real-Artifact Reproduction

- Goal: Observe that `request{...}` spans emitted by the unmodified `torrust-tracker` binary omit `request_id` on inner handler `DEBUG` log events across REST API, HTTP Tracker, and Health Check API.
- Initial state: Unmodified `upstream/develop` (`786b46f`); `cargo build --bin torrust-tracker` completed inside `rust:1-bookworm` from `2026-11-27T11:46:09.476Z` to `2026-11-27T11:49:05.674Z`.
- Status: `DONE`
- Outcome: **Reproduced**

#### Steps Performed

1. Executed inside `rust:1-bookworm`:

   ```bash
   docker run --rm -v "d:/meowluckly/torrust-tracker:/workspace" -w /workspace \
     -v torrust-cargo-registry:/usr/local/cargo/registry \
     -v torrust-cargo-git:/usr/local/cargo/git \
     -v torrust-target:/tmp/target -e CARGO_TARGET_DIR=/tmp/target \
     rust:1-bookworm bash -c "tr -d '\r' < /workspace/.tmp/run-repro.sh | bash -s -- before"
   ```

2. Sequential step timestamps recorded by the driver:
   - Tracker started: `2026-11-27T11:49:30.761Z`, ready: `2026-11-27T11:49:31.231Z`
   - Request 1 (REST API `/api/v1/torrents`): `2026-11-27T11:49:31.233Z` to `2026-11-27T11:49:31.249Z`
   - Request 2 (HTTP Tracker `/announce`): `2026-11-27T11:49:31.251Z` to `2026-11-27T11:49:31.272Z`
   - Request 3 (Health Check API `/health_check`): `2026-11-27T11:49:31.274Z` to `2026-11-27T11:49:31.314Z`
   - Tracker stopped: `2026-11-27T11:49:31.362Z`

#### Observed Result

Relevant HTTP response headers and verbatim tracker log lines (from `.tmp/repro-before.log`, ANSI escapes stripped):

```text
HTTP/1.1 200 OK
x-request-id: repro-3080-api-before

2026-11-27T11:49:31.243188Z  INFO request{method=GET uri=/api/v1/torrents version=HTTP/1.1}: API: request server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ method=GET uri=/api/v1/torrents request_id=repro-3080-api-before
2026-11-27T11:49:31.244234Z DEBUG request{method=GET uri=/api/v1/torrents version=HTTP/1.1}: torrust_tracker_axum_rest_api_server::v1::context::torrent::handlers: pagination: Query(QueryParams { offset: None, limit: None, info_hashes: [] })
2026-11-27T11:49:31.245048Z  INFO request{method=GET uri=/api/v1/torrents version=HTTP/1.1}: API: response latency_ms=1 status_code=200 OK server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ request_id=repro-3080-api-before

HTTP/1.1 200 OK
x-request-id: repro-3080-http-before

2026-11-27T11:49:31.262490Z  INFO request{method=GET uri=/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0 version=HTTP/1.1}: HTTP TRACKER: request server_socket_addr=0.0.0.0:7070 service_binding=http://0.0.0.0:7070/ method=GET uri=/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0 request_id=repro-3080-http-before
2026-11-27T11:49:31.263426Z DEBUG request{method=GET uri=/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0 version=HTTP/1.1}: torrust_tracker_axum_http_server::v1::handlers::announce: Received HTTP announce request
2026-11-27T11:49:31.264075Z DEBUG request{method=GET uri=/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0 version=HTTP/1.1}: torrust_tracker_http_core::services::announce: Sending TcpAnnounce event: TcpAnnounce { ... }

HTTP/1.1 200 OK
x-request-id: repro-3080-hc-before

2026-11-27T11:49:31.282486Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HEALTH CHECK API: request method=GET uri=/health_check request_id=repro-3080-hc-before
2026-11-27T11:49:31.309500Z DEBUG request{method=GET uri=/health_check version=HTTP/1.1}:health_check_handler: torrust_tracker_axum_health_check_api_server::handlers: return=Response { status: 200, version: HTTP/1.1, headers: {"content-type": "application/json"}, body: Body(UnsyncBoxBody) }
2026-11-27T11:49:31.310327Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: HEALTH CHECK API: response latency_ms=27 status_code=200 OK request_id=repro-3080-hc-before
```

#### Conclusion

All three Axum servers returned the supplied `x-request-id` header, yet every `request{...}` span on the `request`, `response`, handler `DEBUG`, and core `DEBUG` log lines omitted `request_id`. The bug is reproduced.

### T2 - Regression Test Red Run (Before Fix)

- Goal: Prove the new maintained regression test `it_should_record_the_request_id_in_the_span_of_a_request` fails on the unfixed `RedactedMakeSpan`.
- Artifact under test: `upstream/develop` (`786b46f`) plus `it_should_record_the_request_id_in_the_span_of_a_request` in `packages/axum-server/src/request_log.rs`; production code unchanged. Stable Rust toolchain (`rustc 1.100.0`).
- Status: `DONE`

#### Steps Performed

1. Added `it_should_record_the_request_id_in_the_span_of_a_request` using `capture_logs_in_scope()` in `packages/axum-server/src/request_log.rs`.
2. Ran `cargo test -p torrust-tracker-axum-server --lib request_log::tests` inside `rust:1-bookworm` (`2026-11-27T11:50:16.313Z` to `2026-11-27T11:50:58.684Z`).

#### Observed Result

```text
running 39 tests
...
test request_log::tests::it_should_record_the_request_id_in_the_span_of_a_request ... FAILED
test request_log::tests::it_should_record_the_redacted_uri_and_not_the_raw_one_in_the_span_of_a_request ... ok
...
failures:

---- request_log::tests::it_should_record_the_request_id_in_the_span_of_a_request stdout ----

thread 'request_log::tests::it_should_record_the_request_id_in_the_span_of_a_request' (2867) panicked at packages/axum-server/src/request_log.rs:304:9:
expected the request span to contain request_id="test-request-id-0001", got span fields `method=GET uri=/health_check version=HTTP/1.1` in line: 2026-11-27T11:50:58.669275Z  INFO request{method=GET uri=/health_check version=HTTP/1.1}: torrust_tracker_axum_server::request_log::tests: an event inside the request span

failures:
    request_log::tests::it_should_record_the_request_id_in_the_span_of_a_request

test result: FAILED. 38 passed; 1 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.01s
```

#### Conclusion

The test compiled cleanly and failed specifically because `request{method=GET uri=/health_check version=HTTP/1.1}` did not contain `request_id="test-request-id-0001"`.

### T3 - Green Tests and Linting (After Fix)

- Goal: Verify all Axum server tests, Clippy lints, and formatting pass after declaring `request_id = Empty` in `RedactedMakeSpan::make_span` (including a `let ... else` cleanup in the regression test to satisfy `clippy::map-unwrap-or`).
- Artifact under test: `upstream/develop` (`786b46f`) plus the changes corresponding to `test(axum-server): [#3080] add regression test for request_id on request span` and `fix(axum-server): [#3080] declare empty request_id field on RedactedMakeSpan`. Stable Rust toolchain (`rustc 1.100.0`) with `clippy` and `rustfmt` components installed in container.
- Status: `DONE`

#### Steps Performed

Executed sequentially in `rust:1-bookworm` (`2026-11-27T11:54:54.020Z` to `2026-11-27T11:56:15.165Z`):

1. `cargo test -p torrust-tracker-axum-server` (`11:54:56.138Z`)
2. `cargo test -p torrust-tracker-axum-http-server -p torrust-tracker-axum-rest-api-server -p torrust-tracker-axum-health-check-api-server` (`11:55:00.563Z` approx.)
3. `cargo clippy -p torrust-tracker-axum-server --all-targets --all-features -- -D warnings` (`11:55:49.497Z`)
4. `cargo fmt --all -- --check` (`11:55:52.354Z` to `11:56:15.165Z`)

#### Observed Result

```text
torrust-tracker-axum-server:
  unit tests: 48 passed; 0 failed
  tests/idle_connections.rs: 9 passed; 0 failed

torrust-tracker-axum-health-check-api-server:
  unit tests: 13 passed; 0 failed
  tests/integration.rs: 13 passed; 0 failed
  tests/request_logging.rs: 1 passed; 0 failed

torrust-tracker-axum-http-server:
  unit tests: 53 passed; 0 failed
  tests/integration.rs: 77 passed; 0 failed
  tests/request_logging.rs: 1 passed; 0 failed

torrust-tracker-axum-rest-api-server:
  unit tests: 170 passed; 0 failed
  tests/integration.rs: 63 passed; 0 failed
  tests/request_logging.rs: 1 passed; 0 failed

cargo clippy -p torrust-tracker-axum-server --all-targets --all-features -- -D warnings:
  Finished `dev` profile [optimized + debuginfo] target(s) in 2.77s (exit 0)

cargo fmt --all -- --check (stable rustfmt 1.100.0):
  Exited 0 (emitted standard stable warnings that `imports_granularity` and `group_imports` require nightly)
```

#### Conclusion

All 448 executed tests across the four Axum packages passed, Clippy passed with `-D warnings`, and `cargo fmt --all -- --check` exited 0 on stable `rustfmt`.

### V2 - Final Like-for-Like Real-Artifact Recheck (After Fix)

- Goal: Repeat V1 identically against the rebuilt `torrust-tracker` binary containing the final `RedactedMakeSpan` fix.
- Artifact under test: Rebuilt `/tmp/target/debug/torrust-tracker` from the final working-tree state (`upstream/develop` `786b46f` + `fix(axum-server): [#3080] declare empty request_id field on RedactedMakeSpan`). Stable Rust toolchain (`rustc 1.100.0`).
- Status: `DONE`

#### Steps Performed

1. Rebuilt `torrust-tracker` with `cargo build --bin torrust-tracker` in `rust:1-bookworm` (`2026-11-27T11:56:48.523Z`, finished in `9.07s`).
2. Executed `.tmp/run-repro.sh after` inside `rust:1-bookworm`:
   - Tracker started: `2026-11-27T11:57:19.442Z`, ready: `2026-11-27T11:57:19.748Z`
   - Request 1 (REST API `/api/v1/torrents`): `2026-11-27T11:57:19.750Z` to `2026-11-27T11:57:19.764Z`
   - Request 2 (HTTP Tracker `/announce`): `2026-11-27T11:57:19.766Z` to `2026-11-27T11:57:19.781Z`
   - Request 3 (Health Check API `/health_check`): `2026-11-27T11:57:19.782Z` to `2026-11-27T11:57:19.818Z`
   - Tracker stopped: `2026-11-27T11:57:19.863Z`

#### Observed Result

Relevant HTTP response headers and verbatim tracker log lines (from `.tmp/repro-after.log`, ANSI escapes stripped):

```text
HTTP/1.1 200 OK
x-request-id: repro-3080-api-after

2026-11-27T11:57:19.759017Z  INFO request{method=GET uri=/api/v1/torrents version=HTTP/1.1 request_id="repro-3080-api-after"}: API: request server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ method=GET uri=/api/v1/torrents request_id=repro-3080-api-after
2026-11-27T11:57:19.759933Z DEBUG request{method=GET uri=/api/v1/torrents version=HTTP/1.1 request_id="repro-3080-api-after"}: torrust_tracker_axum_rest_api_server::v1::context::torrent::handlers: pagination: Query(QueryParams { offset: None, limit: None, info_hashes: [] })
2026-11-27T11:57:19.760627Z  INFO request{method=GET uri=/api/v1/torrents version=HTTP/1.1 request_id="repro-3080-api-after" request_id="repro-3080-api-after"}: API: response latency_ms=1 status_code=200 OK server_socket_addr=0.0.0.0:1212 service_binding=http://0.0.0.0:1212/ request_id=repro-3080-api-after

HTTP/1.1 200 OK
x-request-id: repro-3080-http-after

2026-11-27T11:57:19.773318Z  INFO request{method=GET uri=/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0 version=HTTP/1.1 request_id="repro-3080-http-after"}: HTTP TRACKER: request server_socket_addr=0.0.0.0:7070 service_binding=http://0.0.0.0:7070/ method=GET uri=/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0 request_id=repro-3080-http-after
2026-11-27T11:57:19.774267Z DEBUG request{method=GET uri=/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0 version=HTTP/1.1 request_id="repro-3080-http-after"}: torrust_tracker_axum_http_server::v1::handlers::announce: Received HTTP announce request
2026-11-27T11:57:19.774970Z DEBUG request{method=GET uri=/announce?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=6881&uploaded=0&downloaded=0&left=0 version=HTTP/1.1 request_id="repro-3080-http-after"}: torrust_tracker_http_core::services::announce: Sending TcpAnnounce event: TcpAnnounce { ... }

HTTP/1.1 200 OK
x-request-id: repro-3080-hc-after

2026-11-27T11:57:19.791103Z  INFO request{method=GET uri=/health_check version=HTTP/1.1 request_id="repro-3080-hc-after"}: HEALTH CHECK API: request method=GET uri=/health_check request_id=repro-3080-hc-after
2026-11-27T11:57:19.814324Z DEBUG request{method=GET uri=/health_check version=HTTP/1.1 request_id="repro-3080-hc-after"}:health_check_handler: torrust_tracker_axum_health_check_api_server::handlers: return=Response { status: 200, version: HTTP/1.1, headers: {"content-type": "application/json"}, body: Body(UnsyncBoxBody) }
2026-11-27T11:57:19.815131Z  INFO request{method=GET uri=/health_check version=HTTP/1.1 request_id="repro-3080-hc-after" request_id="repro-3080-hc-after"}: HEALTH CHECK API: response latency_ms=24 status_code=200 OK request_id=repro-3080-hc-after
```

#### Conclusion

Met. Every request span across REST API, HTTP Tracker, and Health Check API now records `request_id` (`request_id="repro-3080-api-after"`, `request_id="repro-3080-http-after"`, and `request_id="repro-3080-hc-after"`), and inner handler/core `DEBUG` log events carry that `request_id` in their `request{...}` span prefix.

### Prose-first Arrange-Act-Assert Record

- **Arrange (prose):** A request carrying `x-request-id` for `/health_check`, and scoped thread-local log capture.
  **Code:** `Request::builder().uri("/health_check").header("x-request-id", request_id)` and `capture_logs_in_scope()`.
- **Act (prose):** Create the request span with `RedactedMakeSpan`, enter it, record `request_id` on the span, and emit an event inside the span.
  **Code:** `RedactedMakeSpan.make_span(&request)`, `span.enter()`, `span.record("request_id", request_id)`, and `tracing::info!("an event inside the request span")`.
- **Assert (prose):** The captured line for the inner event carries a `request{...}` span whose fields include `request_id="..."`.
  **Code:** Extracts `span_fields` between `request{` and `}` on the captured event line and asserts `span_fields.contains(&expected_field)` with the expected field and actual line in the failure message.

## Failures and Follow-up

- During T3, an initial Clippy run flagged `clippy::map-unwrap-or` in `it_should_record_the_request_id_in_the_span_of_a_request`; replaced `.map(...).unwrap_or_else(...)` with `let ... else` and re-ran all T3 test, Clippy, and formatting checks before running V2.
- Both `on_request` and `on_response` across the three Axum servers still call `span.record("request_id", request_id)`, which causes `tracing_subscriber::fmt` to print `request_id="..."` twice on `response` and `on_eos` lines; left untouched to keep the fix scoped to `RedactedMakeSpan`.
