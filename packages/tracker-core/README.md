# BitTorrent Core Tracker library

A library with the core functionality needed to implement a BitTorrent tracker.

You usually don’t need to use this library directly. Instead, you should use the [Torrust Tracker](https://github.com/torrust/torrust-tracker). If you want to build your own tracker, you can use this library as the core functionality. In that case, you should add the delivery layer (HTTP or UDP) on top of this library.

> **Disclaimer**: This library is actively under development. We’re currently extracting and refining common types from the[Torrust Tracker](https://github.com/torrust/torrust-tracker) to make them available to the BitTorrent community in Rust. While these types are functional, they are not yet ready for use in production or third-party projects.

## Documentation

[Crate documentation](https://docs.rs/torrust-tracker-core).

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

If you use Visual Studio Code, you can use the [Coverage Gutters](https://marketplace.visualstudio.com/items?itemName=semasquare.vscode-coverage-gutters) extension to view the coverage lines.

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
