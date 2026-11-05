---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2726-rest-api-https-health-check/ISSUE.md
last-updated-utc: "2026-11-05 12:23"
---

# Manual Verification Evidence

## Purpose

Real evidence for #2726: the REST API health check probed `http://` while the API served TLS. Commands and
outputs below were actually run.

## Environment and Prerequisites

- Date (UTC): 2026-11-04 (initial reproduction) and 2026-11-05 12:23 (recheck).
- Artifact under test: the tracker binary built from `develop` (initial reproduction) and from the PR branch with the
  `fix(axum-rest-api-server): [#2726] check HTTPS REST APIs over TLS` commit (recheck).
- Environment: Linux, stable Rust toolchain, SQLite backend, loopback addresses only.
- Setup: a tracker with only the REST API and the health check API. The REST API serves TLS with the repository test
  certificate (`packages/axum-rest-api-server/tests/fixtures/https-health-check-cert.pem`, self-signed, `IP:127.0.0.1`,
  valid to 2036). Configuration (schema 3.0.0; the TLS-off control removes the `[http_api.tls_config]` section):

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
inactive_peer_cleanup_interval = 120
listed = false
private = false

[core.database]
driver = "sqlite3"
path = ".tmp/storage/sqlite3.db"

[http_api]
bind_address = "127.0.0.1:41212"

[http_api.tls_config]
ssl_cert_path = ".tmp/cert.pem"
ssl_key_path = ".tmp/key.pem"

[http_api.access_tokens]
admin = "MyAccessToken"

[health_check_api]
bind_address = "127.0.0.1:41313"
```

(`.tmp/cert.pem` and `.tmp/key.pem` are copies of the test certificate and key.)

## Verification Processes

### V1 - Reproduce on `develop` with a TLS REST API (before the fix)

- Goal: show that a healthy TLS REST API is reported as an error.
- Status: `DONE`

#### Steps Performed

1. `cargo build --bin torrust-tracker` and run it with the TLS configuration above.
2. `curl -sk https://127.0.0.1:41212/api/health_check`
3. `curl -s http://127.0.0.1:41313/health_check`

#### Observed Result

```text
HTTPS api: 200
{"status":"Error","message":"health check failed","details":[{"service_binding":"https://127.0.0.1:41212/","binding":"127.0.0.1:41212","service_type":"tracker_rest_api","public_url":null,"info":"checking api health check at: http://127.0.0.1:41212/api/health_check","result":{"Err":"error sending request for url (http://127.0.0.1:41212/api/health_check)"}}]}
HTTP 200
```

#### Conclusion

The service answers `200` over HTTPS but the report is `Error` because the probe uses `http://`. The report is still
served with HTTP `200`, so the container `HEALTHCHECK` is not affected.

### V2 - Control with TLS disabled (before the fix)

- Status: `DONE`

#### Observed Result

```text
{"status":"Ok","message":"","details":[{"service_binding":"http://127.0.0.1:41212/","binding":"127.0.0.1:41212","service_type":"tracker_rest_api","public_url":null,"info":"checking api health check at: http://127.0.0.1:41212/api/health_check","result":{"Ok":"200 OK"}}]}
HTTP 200
```

#### Conclusion

Plain HTTP is healthy, so the defect is specific to TLS.

### V3 - Red regression tests (before the fix)

- Goal: the maintained tests fail for the defect.
- Status: `DONE`
- Boundary: a unit test of the URL builder (the causal seam) and one test that starts the token-aware server with a TLS
  launcher, takes the registered service from the `Registar` and runs the check with a client that trusts the test
  certificate. Both live in `packages/axum-rest-api-server/src/server.rs`.
- Code state: `refactor(axum-rest-api-server): [#2726] extract the health check URL builder and a client-taking check`
  plus the two new tests, with the URL still built from the bind address.

#### Observed Result

```text
cargo test -p torrust-tracker-axum-rest-api-server --lib health_check
test server::tests::it_should_build_a_health_check_url_using_the_service_binding_protocol ... FAILED
  left: "http://127.0.0.1:1212/api/health_check"
 right: "https://127.0.0.1:1212/api/health_check"

cargo test -p torrust-tracker-axum-rest-api-server --lib it_should_report_a_tls_rest_api
test server::tests::it_should_report_a_tls_rest_api_as_healthy_when_the_check_trusts_the_test_certificate ... FAILED
  left: Err("error sending request for url (http://127.0.0.1:46101/api/health_check)")
 right: Ok("200 OK")
```

#### Conclusion

Both tests fail for the reported cause, on assertions, not on compilation.

### V4 - Green regression tests (after the fix)

- Status: `DONE`

#### Observed Result

```text
cargo test -p torrust-tracker-axum-rest-api-server
test server::tests::it_should_build_a_health_check_url_using_the_service_binding_protocol ... ok
test server::tests::it_should_report_a_tls_rest_api_as_healthy_when_the_check_trusts_the_test_certificate ... ok
test result: ok. 158 passed; 0 failed   (lib)
test result: ok. 58 passed; 0 failed    (integration)
```

### V5 - Like-for-like recheck with the real binary (after the fix)

- Status: `DONE`
- Note: production certificate validation is unchanged (`reqwest::Client::new()`). A tracker with a self-signed
  certificate that the host does not trust stays `Error`, as for the HTTP tracker in #2089. To show the fixed probe
  with production validation, the second run makes the process trust the test certificate with `SSL_CERT_FILE`.

#### Observed Result

```text
(a) TLS REST API, certificate NOT trusted by the host
curl -sk https://127.0.0.1:41212/api/health_check -> 200
"info":"checking api health check at: https://127.0.0.1:41212/api/health_check",
"result":{"Err":"error sending request for url (https://127.0.0.1:41212/api/health_check)"}   status Error, HTTP 200

(b) TLS REST API, SSL_CERT_FILE=.tmp/cert.pem (certificate trusted, production validation)
{"status":"Ok","message":"","details":[{"service_binding":"https://127.0.0.1:41212/","binding":"127.0.0.1:41212","service_type":"tracker_rest_api","public_url":null,"info":"checking api health check at: https://127.0.0.1:41212/api/health_check","result":{"Ok":"200 OK"}}]}
HTTP 200

(c) TLS disabled (regression control)
{"status":"Ok","message":"","details":[{"service_binding":"http://127.0.0.1:41212/","binding":"127.0.0.1:41212","service_type":"tracker_rest_api","public_url":null,"info":"checking api health check at: http://127.0.0.1:41212/api/health_check","result":{"Ok":"200 OK"}}]}
HTTP 200
```

#### Conclusion

The probe now follows the served protocol: `https://` for a TLS API and `http://` without TLS. With a trusted
certificate the REST API entry is healthy; (a) shows the production trust model was not weakened.

## Failures and Follow-up

None. The acceptance criterion for a real tracker (AC5) holds when the certificate is trusted by the host.
