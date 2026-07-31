# Progressive Verification Evidence

Record baseline and post-change manual verification for each code-changing task
in the registry metadata migration.

## Task Evidence

| Task | Baseline Status | Post-change Status          | Evidence                                                                                                     |
| ---- | --------------- | --------------------------- | ------------------------------------------------------------------------------------------------------------ |
| T2   | NOT RECORDED    | Automated PASS; manual TODO | API boundary reviewed and approved before implementation.                                                    |
| T3   | NOT RECORDED    | Automated PASS; manual TODO | Server-lib tests cover acknowledgement, duplicate rejection, metadata snapshots, and deterministic ordering. |
| T4   | NOT RECORDED    | Automated PASS; manual TODO | `register().await` is the insertion acknowledgement; integration helpers await exact identities.             |
| T5   | NOT RECORDED    | Automated PASS; manual TODO | `cargo publish --dry-run` and final publication of `torrust-server-lib` 0.2.0 succeeded.                     |
| T6   | NOT RECORDED    | Automated PASS; manual TODO | Port-zero integration discovers every HTTP/UDP canonical instance identity.                                  |
| T7   | NOT RECORDED    | Automated PASS; manual TODO | Health contract tests assert preserved URL, binding, and service-type fields for HTTP, REST API, and UDP.    |
| T8   | NOT RECORDED    | Automated PASS; manual TODO | Integration helpers query roles/identities instead of raw map entries or bind IPs.                           |
| T9   | NOT RECORDED    | Automated PASS; manual TODO | Focused server, health-contract, repeated-port-zero, and scaffold tests passed.                              |
| T11  | NOT RECORDED    | Automated PASS; manual TODO | Startup spans use canonical tracing fields and post-bind events include the final service binding.            |

## Automated Local Verification

The issue's evidence protocol asks for manual baseline and post-change probes
before each edit. This work started before those baselines were recorded, so no
manual baseline is available. The following are reproducible **automated**
post-change checks. The completed manual post-change probe is recorded below;
all M1-M3 services and identity-discovery scenarios are now covered.

### T3-T5 - Generic registry API and released crate

- Baseline: Not recorded before implementation.
- Post-change revision: `torrust-server-lib` commit `d17fdb1`.
- Commands: `cargo publish --dry-run`, `cargo publish`, `cargo machete --with-metadata`, `linter all`, and `cargo test --doc --workspace`.
- Observed result: dry-run packaged and verified 18 files; `torrust-server-lib` 0.2.0 published to crates.io. Dependency, lint, and doc-test checks passed.
- Comparison: The released API replaces raw map access with metadata snapshots and acknowledged insertion.
- Result: `DONE`.

### T6-T8 - Runtime identities, health report, and integration discovery

- Baseline: Not recorded before implementation.
- Post-change revision: tracker branch `2041-migrate-runtime-service-registry-metadata`.
- Commands: `cargo test -p torrust-tracker-axum-health-check-api-server --test integration` and `cargo test --test aggregate_stats_port_zero --test scaffold`.
- Observed result: all seven health-contract tests passed; repeated port-zero HTTP/UDP blocks registered distinct non-zero final bindings for exact canonical identities; scaffold and port-zero integration scenarios passed.
- Comparison: Helper behavior now waits for exact canonical identities and finds endpoints by role, rather than registry size, map ordering, or bind-IP conventions.
- Result: `DONE`.

### T9 - Focused regression coverage

- Baseline: Not recorded before implementation.
- Post-change revision: tracker branch `2041-migrate-runtime-service-registry-metadata`.
- Commands: `cargo check --workspace --all-targets`; focused server package tests; `cargo test --test aggregate_stats_fixed_ports --test aggregate_stats_port_zero --test scaffold`; and `linter all`.
- Observed result: all invoked checks passed. Health JSON tests assert `service_binding`, `binding`, and `service_type`; port-zero coverage asserts exact identity-to-final-binding correlation.
- Comparison: Regression coverage now protects the metadata and readiness contracts introduced by this issue.
- Result: `DONE`.

### T11 - Structured runtime identity logging

#### Baseline

- Not recorded before implementation.

#### Post-change

- Revision: tracker branch `2041-migrate-runtime-service-registry-metadata`,
  after documentation commit `d7684051`.
- Changed behavior: HTTP, UDP, and REST startup spans skip automatic
  `RuntimeServiceMetadata` capture and explicitly emit `service_role` and
  `instance_index`. HTTP, UDP, REST, and health API startup paths emit a
  post-bind event with `service_binding`.
- Commands: `cargo test -p torrust-tracker-axum-http-server -p
  torrust-tracker-udp-server -p torrust-tracker-axum-rest-api-server -p
  torrust-tracker --lib`; `cargo test -p
  torrust-tracker-axum-health-check-api-server --test integration`; `cargo test
  --test aggregate_stats_port_zero --test scaffold`; `linter all`; and `git
  diff --check`.
- Observed result: HTTP server (21 tests), REST API server (1 test), UDP server
  (125 tests), tracker library (58 tests), health integration (7 tests), and
  port-zero/scaffold integration tests passed. All linters and whitespace checks
  passed.
- Shutdown note: an attempted `timeout 20s cargo run ...` probe did not stop
  the tracker because `timeout` sends SIGTERM while the current tracker entry
  point listens for SIGINT via Ctrl+C. `src/main.rs` and the relevant shutdown
  orchestration are unchanged from `develop`; sending SIGINT stopped the process.
  Manual logging verification must therefore start the tracker normally and use
  Ctrl+C. SIGTERM support is outside #2041 and belongs to shutdown-overhaul
  issue #1488.
- Manual command: `cargo run --quiet`, followed by Ctrl+C after startup.
- Observed startup output included explicit, queryable fields without a
  `metadata=RuntimeServiceMetadata` rendering. Representative entries were:
  `start_job{service_role="udp_tracker" instance_index=0}` followed by
  `Started UDP tracker service_binding=udp://0.0.0.0:6868`;
  `start_job{version=V1 service_role="http_tracker" instance_index=1}` followed
  by `Started HTTP tracker service_binding=http://0.0.0.0:7171/`; and
  `start_job{version=V1 service_role="tracker_rest_api" instance_index=0}`
  followed by `Started tracker API service_binding=http://0.0.0.0:1212/`. The
  health API emitted `service_role="health_check_api" instance_index=0
  service_binding=http://127.0.0.1:1313/`.
- Observed shutdown result: Ctrl+C logged `Torrust tracker shutting down ...`,
  each managed job completed gracefully, and the process ended with `Torrust
  tracker successfully shutdown.`
- Comparison: startup logging no longer depends on nested Rust `Debug` output
  for metadata identity. The canonical fields and final binding are explicit.
- Result: `DONE`.

## Manual Post-Change Verification

The manual baseline was not captured before implementation. The following
post-change probe was performed against a locally started tracker and records
the actual configuration, commands, and output.

### M1-M3 - Port-zero service startup and health report

#### Baseline

- Not recorded before implementation.

#### Post-change

- Revisions: tracker commit `28b60a78` and follow-up invariant refactor
  `e9515303`.
- Configuration: `.tmp/issue-2041-manual.toml` configured two HTTP and two UDP
  listeners at `0.0.0.0:0`, a REST API at `127.0.0.1:18081`, and a health API
  at `127.0.0.1:18080`. TLS/HTTPS was not configured for this probe.
- Start command:
  `TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/issue-2041-manual.toml" cargo run --bin torrust-tracker`.
- Startup output: distinct final bindings were assigned and logged with their
  canonical metadata: `UdpTracker(0)=0.0.0.0:49980`,
  `UdpTracker(1)=0.0.0.0:57094`, `HttpTracker(0)=0.0.0.0:59065`,
  `HttpTracker(1)=0.0.0.0:44209`, `RestApi(0)=127.0.0.1:18081`, and
  `HealthCheckApi(0)=127.0.0.1:18080`.
- Health query: `curl --fail --silent --show-error http://127.0.0.1:18080/health_check`.
- Observed health report: `status` was `Ok`. It reported five checkable
  services in deterministic protocol/binding order: both UDP listeners with
  `service_type="udp_tracker"`, both HTTP listeners with
  `service_type="http_tracker"`, and the REST API with
  `service_type="tracker_rest_api"`. Every report entry preserved matching
  `service_binding` URL and `binding` socket address. The health API itself was
  correctly omitted because it is metadata-only and must not recursively check
  itself.
- Service probes:
  - `curl --fail --silent --show-error http://127.0.0.1:59065/health_check` → `{"status":"Ok"}`.
  - `curl --fail --silent --show-error http://127.0.0.1:44209/health_check` → `{"status":"Ok"}`.
  - `curl --fail --silent --show-error http://127.0.0.1:18081/api/health_check` → `{"status":"Ok"}`.
  - `cargo run -p torrust-tracker-client --bin tracker_client -- udp announce udp://127.0.0.1:49980/announce 0123456789abcdef0123456789abcdef01234567` → successful IPv4 announce response.
  - The same announce command against `udp://127.0.0.1:57094/announce` → successful IPv4 announce response.
- Comparison: exact configuration identities were correlated with non-zero,
  distinct final bindings without bind-IP classification, registry-map order,
  or a startup delay. The health-report JSON retained the compatibility fields.
- Result: `DONE` for HTTP, UDP, REST API, health API, repeated port-zero
  identity, and health compatibility.

### M1 - HTTPS port-zero listener

#### Baseline

- Not recorded before implementation.

#### Post-change

- Revision: tracker commits `28b60a78` and `e9515303`.
- Temporary TLS material: generated a one-day self-signed RSA certificate and
  key in the ignored `.tmp/` directory. The certificate contained SAN entries
  for `localhost` and `127.0.0.1`, allowing a local direct probe.
- Temporary configuration: added a schema-2.0
  `[http_trackers.tsl_config]` section to the second repeated HTTP
  `0.0.0.0:0` listener in `.tmp/issue-2041-manual.toml`. It referenced the
  temporary certificate and key. The configuration was restored afterwards.
- Certificate command:
  `openssl req -x509 -out .tmp/issue-2041-manual.crt -keyout .tmp/issue-2041-manual.key -newkey rsa:2048 -nodes -sha256 -days 1 -subj '/CN=localhost' -addext 'subjectAltName=DNS:localhost,IP:127.0.0.1' -addext 'keyUsage=digitalSignature' -addext 'extendedKeyUsage=serverAuth'`.
- Start command:
  `TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/issue-2041-manual.toml" cargo run --bin torrust-tracker`.
- Startup output: `HttpTracker(0)` bound as
  `http://0.0.0.0:58997`; `HttpTracker(1)` bound as
  `https://0.0.0.0:60057`. The latter used the temporary certificate and key.
  The same run also bound `UdpTracker(0)=0.0.0.0:42524`,
  `UdpTracker(1)=0.0.0.0:54809`, `RestApi(0)=127.0.0.1:18081`, and
  `HealthCheckApi(0)=127.0.0.1:18080`.
- Registry/health-report query:
  `curl --fail --silent --show-error http://127.0.0.1:18080/health_check`.
  The report contained the HTTPS entry with
  `service_binding="https://0.0.0.0:60057/"`,
  `binding="0.0.0.0:60057"`, and `service_type="http_tracker"`.
- Direct TLS probe:
  `curl --fail --silent --show-error --insecure https://127.0.0.1:60057/health_check`.
  The response was `{"status":"Ok"}`.
- Known unrelated limitation observed: the aggregate health report had
  `status="Error"` for the HTTPS listener because
  `packages/axum-http-server/src/server.rs` constructs the check URL with a
  hard-coded `http://` scheme. Its report detail attempted
  `http://0.0.0.0:60057/health_check` despite correctly exposing the service's
  HTTPS binding. This is pre-existing behavior explicitly outside this issue's
  scope; it is tracked by the draft issue
  `docs/issues/drafts/fix-https-tracker-health-check-protocol.md`.
- Comparison: same-role repeated HTTP configuration instances were
  distinguished by canonical `HttpTracker` identities and their separately
  assigned final HTTP and HTTPS bindings. The direct TLS probe confirms that
  the HTTPS listener itself was operational.
- Result: `DONE`. The registry-metadata behavior and the M1 service-startup
  requirement are verified. The unrelated aggregate HTTPS health-check defect
  is documented separately.

## Scenario Record Template

```markdown
### T{N} - {Task title}

#### Baseline

- Configuration:
- Command/query:
- Observed result:

#### Post-change

- Commit or revision:
- Command/query:
- Observed result:
- Comparison:
- Result: `DONE` / `FAILED`
```
