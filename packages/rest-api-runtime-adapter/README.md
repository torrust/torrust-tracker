# Torrust Tracker REST API Runtime Adapter

`torrust-tracker-rest-api-runtime-adapter` implements the ports of the REST API application layer with the real tracker components, and converts domain values to protocol DTOs. It is glue code for the Torrust Tracker, not a reusable library.

## Where it fits

Adapter layer of the contract-first REST API, between `torrust-tracker-rest-api-application` and the tracker core packages. Workspace dependencies: `torrust-tracker-configuration`, `torrust-tracker-core`, `torrust-tracker-http-core`, `torrust-tracker-primitives`, `torrust-tracker-rest-api-application`, `torrust-tracker-rest-api-protocol`, `torrust-tracker-swarm-coordination-registry`, `torrust-tracker-udp-core`, `torrust-tracker-udp-server`. Used by: `torrust-tracker`, `torrust-tracker-axum-rest-api-server`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`v1` holds the port implementations and the domain-to-DTO conversions.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-rest-api-runtime-adapter)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
