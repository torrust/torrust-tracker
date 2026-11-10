# Torrust Tracker Primitives

`torrust-tracker-primitives` defines the domain types shared by the Torrust Tracker packages: peers, announce and scrape data, swarm metadata, tracker policy and related value types, and re-exports `PeerId` and `DurationSinceUnixEpoch`. It has no I/O.

## Where it fits

Domain package at the bottom of the tracker dependency graph. Used by: `torrust-tracker`, `torrust-tracker-axum-health-check-api-server`, `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`, `torrust-tracker-configuration`, `torrust-tracker-core` and 6 more. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`peer::Peer`, `AnnounceData`, `ScrapeData`, `swarm_metadata::SwarmMetadata`, `TrackerPolicy`, `Readiness` and `PeerId`.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). The crate follows semantic versioning: breaking changes are released in a new major version. It is developed for the Torrust Tracker first.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-primitives)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
