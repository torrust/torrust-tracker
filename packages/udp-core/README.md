# Torrust Tracker UDP Core

`torrust-tracker-udp-core` contains the UDP-specific tracker logic of the Torrust Tracker: connection cookies, connect, announce and scrape services and statistics, independent of the socket layer. You usually do not need it directly: use the [Torrust Tracker](https://github.com/torrust/torrust-tracker). To build your own UDP tracker, add a delivery layer on top.

## Where it fits

Core package (`*-core`) between `torrust-tracker-core` and the UDP server. Workspace dependencies: `torrust-tracker-configuration`, `torrust-tracker-core`, `torrust-tracker-events`, `torrust-tracker-primitives`, `torrust-tracker-swarm-coordination-registry`, `torrust-tracker-udp-protocol`. Used by: `torrust-tracker`, `torrust-tracker-axum-rest-api-server`, `torrust-tracker-rest-api-runtime-adapter`, `torrust-tracker-udp-server`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`connection_cookie`, `services`, `container`, `event` and `statistics`.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-udp-core)
- [UDP ban-service benchmarking](docs/benchmarking/banning.md)
- [Architectural Decision Records](docs/adrs/README.md)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
