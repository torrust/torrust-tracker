# Torrust Tracker Axum Server

`torrust-tracker-axum-server` is the base Axum server wrapper used by the HTTP services of the Torrust Tracker. It adds request timeouts, graceful shutdown handling and TLS support on top of `axum-server`. It does not implement any tracker route.

## Where it fits

Infrastructure used by the `axum-*` server packages. Workspace dependencies: `torrust-tracker-configuration`. Used by: `torrust-tracker`, `torrust-tracker-axum-health-check-api-server`, `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`custom_axum_server` wraps the server with timeouts, `signals` handles graceful shutdown and `tls` loads the TLS configuration.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-axum-server)

## Notes

This package is tracker-scoped infrastructure for HTTP services in the Torrust tracker.
It is the base Axum server wrapper used by the tracker's HTTP service packages, so it
is fine for it to depend on tracker configuration types when that keeps the service API
cohesive.

The TLS helper in `tls.rs` currently depends on:

- `TslConfig` from `torrust-tracker-configuration` — the tracker supervisor's public
  TLS configuration DTO
- `LocatedError` / `DynError` from `torrust-located-error` — already extracted into a
  generic package

If this server wrapper is reused outside the tracker in the future, the package
boundary can be revisited and a more generic home for `TslConfig` can be evaluated
then.

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
