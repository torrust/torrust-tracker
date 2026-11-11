# Torrust Tracker REST API Application

`torrust-tracker-rest-api-application` is the application (use-case) layer of the Torrust Tracker management REST API: port traits, use-case services and the mapping of domain errors. It does not know about HTTP or about the tracker internals.

## Where it fits

Application layer of the contract-first REST API. Workspace dependencies: `torrust-tracker-primitives`, `torrust-tracker-rest-api-protocol`. Used by: `torrust-tracker-axum-rest-api-server`, `torrust-tracker-rest-api-runtime-adapter`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`v1` holds the ports and use-case services.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-rest-api-application)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
