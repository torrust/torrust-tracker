# Torrust Tracker HTTP Core

`torrust-tracker-http-core` contains the HTTP-specific tracker logic of the Torrust Tracker: request validation, announce and scrape services and statistics, independent of the web framework. You usually do not need it directly: use the [Torrust Tracker](https://github.com/torrust/torrust-tracker). To build your own HTTP tracker, add a delivery layer on top.

## Where it fits

Core package (`*-core`) between `torrust-tracker-core` and the HTTP server. Workspace dependencies: `torrust-tracker-configuration`, `torrust-tracker-core`, `torrust-tracker-events`, `torrust-tracker-http-protocol`, `torrust-tracker-primitives`, `torrust-tracker-swarm-coordination-registry`. Used by: `torrust-tracker`, `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`, `torrust-tracker-rest-api-runtime-adapter`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`services` holds the announce and scrape services, `container` wires them, and `event` and `statistics` expose the HTTP events and metrics.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-http-core)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
