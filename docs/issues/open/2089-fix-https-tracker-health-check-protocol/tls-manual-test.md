# TLS Certificate and Manual Verification

<!-- cspell:ignore keyid noout -->

This document describes the committed TLS fixture used by the HTTPS health-check
regression, how to recreate it if required, and how to perform the associated
manual checks.

## Purpose and Boundaries

The production HTTP-tracker health check uses `reqwest::Client::new()`. It
therefore keeps normal platform trust-store validation and must **not** bypass
certificate validation for local self-signed certificates.

The integration test at
`packages/axum-health-check-api-server/tests/server/contract.rs` supplies a
named, non-capturing `trusted_test_check_fn`. That callback trusts exactly the
static fixture certificate and verifies the aggregate health API can report an
HTTPS tracker as healthy.

Do not use `danger_accept_invalid_certs(true)`, `curl --insecure`, or a
production configuration change to validate the aggregate-health behavior.
Those approaches do not verify the required trust model.

## Committed Test Fixture

The test-only certificate and private key are stored in:

- `packages/axum-health-check-api-server/tests/fixtures/https-health-check-cert.pem`
- `packages/axum-health-check-api-server/tests/fixtures/https-health-check-key.pem`

The certificate is a self-signed TLS server certificate with:

| Property                 | Value                                 |
| ------------------------ | ------------------------------------- |
| Subject and issuer       | `CN=127.0.0.1`                        |
| Subject alternative name | `IP:127.0.0.1`                        |
| Basic constraint         | `CA:FALSE`                            |
| Key usage                | `digitalSignature`, `keyEncipherment` |
| Extended key usage       | TLS Web Server Authentication         |
| Validity                 | 2026-08-24 through 2036-08-21         |

The IP SAN is required because the test connects to `https://127.0.0.1:<port>`.
A common name alone is insufficient for modern TLS hostname verification.

## Recreate the Fixture

Recreation is normally unnecessary. If the fixture must be replaced, generate
a non-CA leaf certificate with the same loopback IP SAN. The command below
creates temporary files first, so only the reviewed final artifacts are copied
into the fixture directory.

```bash
tmpdir=$(mktemp -d)
cat > "$tmpdir/openssl.cnf" <<'EOF'
[req]
distinguished_name = req_distinguished_name
x509_extensions = v3_server
prompt = no

[req_distinguished_name]
CN = 127.0.0.1

[v3_server]
subjectAltName = IP:127.0.0.1
basicConstraints = critical,CA:FALSE
keyUsage = critical,digitalSignature,keyEncipherment
extendedKeyUsage = serverAuth
subjectKeyIdentifier = hash
authorityKeyIdentifier = keyid:always
EOF

openssl req -x509 -newkey rsa:2048 -sha256 -nodes \
  -keyout "$tmpdir/key.pem" \
  -out "$tmpdir/cert.pem" \
  -days 3650 \
  -config "$tmpdir/openssl.cnf"

cp "$tmpdir/cert.pem" packages/axum-health-check-api-server/tests/fixtures/https-health-check-cert.pem
cp "$tmpdir/key.pem" packages/axum-health-check-api-server/tests/fixtures/https-health-check-key.pem
rm -rf "$tmpdir"
```

Inspect a replacement before committing it:

```bash
openssl x509 \
  -in packages/axum-health-check-api-server/tests/fixtures/https-health-check-cert.pem \
  -noout -subject -issuer -dates \
  -ext subjectAltName \
  -ext basicConstraints \
  -ext keyUsage \
  -ext extendedKeyUsage
```

Confirm that the certificate is not a CA, includes `IP:127.0.0.1`, and is
usable for TLS server authentication. Treat the key as test-only material; do
not reuse it for any deployed listener.

## Aggregate HTTPS Regression Procedure

This is the authoritative end-to-end verification for the issue. It starts an
ephemeral HTTPS HTTP tracker, registers the named callback that adds the fixture
certificate as a root, starts the aggregate health API, and asserts its report.

1. Run the focused regression:

```bash
cargo test -p torrust-tracker-axum-health-check-api-server --test integration \
  it_should_return_good_health_for_https_service_with_a_trusted_test_certificate
```

1. Confirm it passes. The test asserts all of the following:
   - the aggregate report has `Status::Ok`;
   - `service_binding` uses `https://127.0.0.1:<ephemeral-port>`;
   - `service_type` is `http_tracker`;
   - the result is `200 OK`; and
   - the information message identifies the HTTPS `/health_check` URL.

1. Run the affected suites to ensure ordinary HTTP behavior remains intact:

```bash
cargo test -p torrust-tracker-axum-http-server
cargo test -p torrust-tracker-axum-health-check-api-server --test integration
```

1. Run the repository quality checks before committing changes:

```bash
linter all
TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh
```

## Direct TLS Listener Check

For a diagnostic check of the HTTPS listener alone, run the focused regression
above or start an equivalent temporary listener using the fixture paths. Probe
it with explicit certificate trust:

```bash
curl --fail --silent --show-error \
  --cacert packages/axum-health-check-api-server/tests/fixtures/https-health-check-cert.pem \
  https://127.0.0.1:<port>/health_check
```

The expected response is `{"status":"Ok"}`. This confirms TLS handshake,
loopback-IP validation, and the listener endpoint. It does **not** replace the
aggregate regression, because the production health-check client does not trust
this self-signed fixture.

## Production-like Manual Runtime Check

To exercise the unmodified production callback through the aggregate API, the
TLS listener needs a certificate already trusted by the platform trust store
used by `reqwest`. Use a real development trust anchor installed for the current
user or a publicly trusted certificate for a hostname that resolves to the test
listener. Configure that certificate in `tsl_config`, then:

1. Start the tracker with its temporary configuration.
2. Read the log to find the final HTTPS listener address and health API address.
3. Query `http://<health-api-address>/health_check`.
4. Verify the HTTPS tracker detail uses an `https://` service binding and has a
   `200 OK` result.
5. Stop the tracker and remove local-only certificate/configuration files.

Do not mark this scenario complete when using a self-signed certificate that
only `curl --cacert` trusts: that validates the listener, not the default
production health-check client's trust path.
