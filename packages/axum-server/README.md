# Torrust Tracker Axum Server

A wrapper for the Axum server used by Torrust tracker HTTP services to add timeouts.

## Documentation

[Crate documentation](https://docs.rs/torrust-tracker-axum-server).

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

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
