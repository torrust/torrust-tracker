# Test fixtures

`https-health-check-cert.pem` and `https-health-check-key.pem` are a self-signed certificate and
its private key (CN and IP SAN `127.0.0.1`) made only for automatic tests. The key is public by
design, protects nothing and must never be used outside tests. They are byte-identical to the
fixtures in `packages/axum-rest-api-server/tests/fixtures`.
