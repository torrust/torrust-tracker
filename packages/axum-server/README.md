# Torrust Tracker Axum Server

Shared Axum server infrastructure for the Torrust tracker HTTP services: the listener constructors
with the connection timeouts, graceful shutdown, and the TLS configuration helper.

## What it provides

- `custom_axum_server`: the one place that builds the Axum listeners of the HTTP tracker, the REST
  API and the health check API. `Timeouts` holds the values and `from_tcp_with_timeouts` and
  `from_tcp_rustls_with_timeouts` apply them. They rely on `axum-server` 0.9, including
  `Server::first_request_timeout`. A connection that sends no complete first request is closed after
  5 s, for plaintext and TLS (after the handshake). An idle connection after a response is closed
  after 5 s over HTTP/1 and about 10 s over HTTP/2, and a TLS handshake that does not complete is
  closed after 10 s. The server closes the connection without a `408`. The values are constants and
  can be injected with `from_tcp_with` and `from_tcp_rustls_with`, for tests.
- `signals`: graceful shutdown and the drain outcome of a server.
- `tls`: builds the Rustls configuration from the tracker's TLS settings.

New listeners must be built through `custom_axum_server`, so they get the same timeouts. Building
one with `axum_server::from_tcp` directly leaves idle connections open.

## Documentation

[Crate documentation](https://docs.rs/crate/torrust-tracker/latest).

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

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
