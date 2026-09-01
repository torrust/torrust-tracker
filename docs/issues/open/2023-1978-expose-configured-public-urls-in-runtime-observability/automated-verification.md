---
doc-type: verification-evidence
issue: 2023
spec-path: docs/issues/open/2023-1978-expose-configured-public-urls-in-runtime-observability/ISSUE.md
recorded-at-utc: 2026-08-31 00:00
---

# Automated Verification - Issue #2023

| Command                                                                                                                                                                | Result                                                                                                                                                                                                                 |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cargo test -p torrust-tracker-axum-health-check-api-server --test integration api::it_should_return_good_health_for_api_service`                                      | Passed. The contract configures `0.0.0.0:0` and a public URL; it verifies the health check separately returns the wildcard post-bind `service_binding`, its OS-assigned nonzero port, and the configured `public_url`. |
| `cargo test -p torrust-tracker-http-core -p torrust-tracker-udp-core`                                                                                                  | Passed: 27 HTTP-core and 39 UDP-core unit tests, including configured and absent `public_url` metric-label assertions.                                                                                                 |
| `cargo test -p torrust-tracker-http-core -p torrust-tracker-udp-core -p torrust-tracker-udp-server -p torrust-tracker-axum-health-check-api-server --test integration` | Passed: health-check and UDP-server integration contracts.                                                                                                                                                             |
| `linter all`                                                                                                                                                           | Passed: Markdown, YAML, TOML, cspell, Clippy, rustfmt, ShellCheck.                                                                                                                                                     |
| `cargo test --workspace`                                                                                                                                               | Passed, including workspace unit, integration, and documentation tests.                                                                                                                                                |
