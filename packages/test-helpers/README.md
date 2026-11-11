# Torrust Tracker Test Helpers

`torrust-tracker-test-helpers` provides helpers for testing the Torrust Tracker packages: test configurations, logging setup and random data generators. Use it as a development dependency only.

## Where it fits

Utility package used as a dev-dependency by many tracker packages. Workspace dependencies: `torrust-tracker-configuration`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`configuration` builds test configurations, `logging` initializes test logging and `random` generates random test data.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). The crate follows semantic versioning: breaking changes are released in a new major version. It is developed for the Torrust Tracker first.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-test-helpers)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
