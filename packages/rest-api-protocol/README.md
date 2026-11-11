# Torrust Tracker REST API Protocol

`torrust-tracker-rest-api-protocol` defines the contract of the Torrust Tracker management REST API: versioned data transfer objects, error schemas and authentication semantics. It contains no transport or business logic, so clients and servers can share it.

## Where it fits

Protocol layer of the contract-first REST API, used by the application layer, the Axum server and the REST API client. Used by: `torrust-tracker`, `torrust-tracker-axum-rest-api-server`, `torrust-tracker-rest-api-application`, `torrust-tracker-rest-api-client`, `torrust-tracker-rest-api-runtime-adapter`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`v1` holds the request and response DTOs and the error types.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-rest-api-protocol)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
