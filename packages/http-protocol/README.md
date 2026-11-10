# Torrust Tracker HTTP Protocol

`torrust-tracker-http-protocol` provides the types and functions for the BitTorrent HTTP tracker protocol (BEP 3 and BEP 23): announce and scrape request parsing, percent-decoding and bencoded response building. It has no server or tracker logic, so other BitTorrent tools can reuse it.

## Where it fits

Protocol package (`*-protocol`) used by `torrust-tracker-http-core` and the HTTP client. Used by: `torrust-tracker-axum-http-server`, `torrust-tracker-client`, `torrust-tracker-client-lib`, `torrust-tracker-http-core`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`v1` holds the announce and scrape request and response types and `percent_encoding` decodes info-hashes and peer ids.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-http-protocol)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
