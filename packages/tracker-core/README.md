# Torrust Tracker Core

`torrust-tracker-core` contains the central logic of the Torrust Tracker, independent of the delivery protocol: announce and scrape handling, peer keys authentication, the whitelist, the database drivers and persistence of torrent statistics. You usually do not need it directly: use the [Torrust Tracker](https://github.com/torrust/torrust-tracker). To build your own tracker, add a delivery layer (HTTP or UDP) on top.

## Where it fits

Core package (`*-core`) below `torrust-tracker-http-core` and `torrust-tracker-udp-core`. Workspace dependencies: `torrust-tracker-configuration`, `torrust-tracker-events`, `torrust-tracker-primitives`, `torrust-tracker-swarm-coordination-registry`. Used by: `torrust-tracker`, `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`, `torrust-tracker-http-core`, `torrust-tracker-rest-api-runtime-adapter`, `torrust-tracker-udp-core` and 1 more. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`announce_handler`, `scrape_handler`, `authentication`, `whitelist`, `databases` and `container`.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-core)

## Testing

Run tests including tests for MySQL driver:

```console
TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo test
```

> NOTE: MySQL driver requires docker to run. We don't run them by default because we don't want to run them when we build container images. The Torrust Tracker container build runs unit tests for all dependencies, including this library.

Show coverage report:

```console
TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo +stable llvm-cov
```

Export coverage report to `lcov` format:

```console
TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true cargo +stable llvm-cov --lcov --output-path=./.coverage/lcov.info
```

If you use Visual Studio Code, you can use the [Coverage Gutters](https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters) extension to view the coverage lines.

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
