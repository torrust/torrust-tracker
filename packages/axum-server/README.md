# Torrust Tracker Axum Server

`torrust-tracker-axum-server` is the base Axum server wrapper used by the HTTP services of the Torrust Tracker. It adds connection timeouts, graceful shutdown handling and TLS support on top of `axum-server`. It does not implement any tracker route.

## Where it fits

Infrastructure used by the `axum-*` server packages. Workspace dependencies: `torrust-tracker-configuration`. Used by: `torrust-tracker`, `torrust-tracker-axum-health-check-api-server`, `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`custom_axum_server` builds the listeners with the connection timeouts (`Timeouts`, `from_tcp_with_timeouts`, `from_tcp_rustls_with_timeouts`), `signals` handles graceful shutdown and `tls` loads the TLS configuration.

The timeouts use `axum-server` 0.9. A connection that sends no complete first request is closed after 5 s, for plaintext and TLS (after the handshake). An idle connection after a response is closed after 5 s over HTTP/1, and after about 10 s over HTTP/2 when the client does not answer the server's keep-alive pings; a client that answers them keeps an idle HTTP/2 connection open and is not time-bounded (EPIC #2411 case A1). A TLS handshake that does not complete is closed after 10 s. The server closes the connection without a `408` ([ADR](../../docs/adrs/20261111095222_close_idle_connections_without_a_408_response.md)). The values are constants; `from_tcp_with` and `from_tcp_rustls_with` accept others, for tests.

Build every listener through `custom_axum_server`: building one with `axum_server::from_tcp` directly leaves idle connections open, which lets a flood of silent connections exhaust the process's file descriptors. Two guards enforce this:

- The root `clippy.toml` forbids the `axum-server` listener constructors and `axum::serve` (`disallowed-methods`). Only the two constructors in `custom_axum_server.rs` call them, each with a documented `allow`.
- Each service has a test that a connection that sends nothing is closed (plaintext, and TLS where it supports it). Add one when you add a listener.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-axum-server)

## Notes

This package is tracker-scoped infrastructure for HTTP services in the Torrust tracker.
It is the base Axum server infrastructure used by the tracker's HTTP service packages, so it
is fine for it to depend on tracker configuration types when that keeps the service API
cohesive.

The TLS helper in `tls.rs` currently depends on:

- `TslConfig` from `torrust-tracker-configuration` — the tracker supervisor's public
  TLS configuration DTO
- `LocatedError` / `DynError` from `torrust-located-error` — already extracted into a
  generic package

If this package is reused outside the tracker in the future, the package
boundary can be revisited and a more generic home for `TslConfig` can be evaluated
then.

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
