# Torrust Tracker Configuration

`torrust-tracker-configuration` provides the configuration types of the Torrust Tracker: loading from TOML files and environment variables, versioned schemas, validation and defaults. Depend on it to read or generate a tracker configuration; it does not start any service.

## Where it fits

Domain package used by almost every other tracker package. Workspace dependencies: `torrust-tracker-primitives`. Used by: `torrust-tracker`, `torrust-tracker-axum-health-check-api-server`, `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`, `torrust-tracker-axum-server`, `torrust-tracker-core` and 5 more. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`v3_0_0` holds the current configuration schema and `v2_0_0` the previous one, and `validator` checks a configuration.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). The crate follows semantic versioning: breaking changes are released in a new major version. It is developed for the Torrust Tracker first.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-configuration)
- [Migrate Configuration v2 to v3](docs/migrate-v2-to-v3.md)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
