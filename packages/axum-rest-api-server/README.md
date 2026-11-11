# Torrust Tracker Axum REST API Server

`torrust-tracker-axum-rest-api-server` is the Axum transport layer of the Torrust Tracker management REST API: HTTP routing, request extraction, response serialization and middleware. The API contract lives in `torrust-tracker-rest-api-protocol` and the use cases in `torrust-tracker-rest-api-application`.

## Where it fits

Server (`axum-*`) package of the contract-first REST API. Workspace dependencies: `torrust-tracker-axum-server`, `torrust-tracker-configuration`, `torrust-tracker-core`, `torrust-tracker-http-core`, `torrust-tracker-primitives`, `torrust-tracker-rest-api-application`, `torrust-tracker-rest-api-client`, `torrust-tracker-rest-api-protocol`, `torrust-tracker-rest-api-runtime-adapter`, `torrust-tracker-swarm-coordination-registry`, `torrust-tracker-udp-core`, `torrust-tracker-udp-server`. Used by: `torrust-tracker`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`server` starts and stops the API server, `routes` builds the router and `v1` holds the versioned handlers.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-axum-rest-api-server)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
