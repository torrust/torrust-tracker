# Torrust Tracker Axum HTTP Server

`torrust-tracker-axum-http-server` is the BitTorrent HTTP tracker server (BEP 3 and BEP 23) of the Torrust Tracker, built on Axum. It handles the announce and scrape routes and delegates the tracker logic to `torrust-tracker-http-core`. Use it only to run an HTTP tracker; it is not a general-purpose Axum helper.

## Where it fits

Server (`axum-*`) package. Workspace dependencies: `torrust-tracker-axum-server`, `torrust-tracker-configuration`, `torrust-tracker-core`, `torrust-tracker-http-core`, `torrust-tracker-http-protocol`, `torrust-tracker-primitives`, `torrust-tracker-swarm-coordination-registry`. Used by: `torrust-tracker`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`server` starts and stops the HTTP tracker and `v1` holds its routes, extractors and handlers.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-axum-http-server)

## Testing and Coverage

This crate belongs to the Torrust Tracker Cargo workspace. Run its tests from the repository root:

```text
cargo test -p torrust-tracker-axum-http-server
```

Install [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov) to measure coverage. From
the repository root, run:

```text
cargo llvm-cov -p torrust-tracker-axum-http-server --all-features --summary-only
```

When working from this package directory, use its manifest explicitly:

```text
cargo llvm-cov --manifest-path Cargo.toml --all-features --summary-only
```

Use `--json` instead of `--summary-only` when you need per-file, function, and region detail.
Aggregate percentages are only a navigation aid: prioritize behavior risk and per-file gaps when
choosing tests. For Issue #2136's current measurement method and coverage interpretation, see
[`coverage-evidence.md`](https://github.com/torrust/torrust-tracker/blob/develop/docs/issues/closed/2136-1347-add-tests-axum-http-server/coverage-evidence.md).

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
