# Torrust Tracker Swarm Coordination Registry

`torrust-tracker-swarm-coordination-registry` is the in-memory registry of the torrents (swarms) a Torrust Tracker tracks and the peers in each swarm. It stores and retrieves torrent entries, manages peers (including removing inactive ones), supports paginated and sorted access, applies the policy for peerless torrents and publishes statistics and events.

## Where it fits

Domain package used by `torrust-tracker-core`. Workspace dependencies: `torrust-tracker-events`, `torrust-tracker-primitives`. Used by: `torrust-tracker`, `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`, `torrust-tracker-core`, `torrust-tracker-http-core`, `torrust-tracker-rest-api-runtime-adapter` and 2 more. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`swarm` holds the registry and its entries, `container` wires it, and `event` and `statistics` expose the events and metrics.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-swarm-coordination-registry)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](https://github.com/torrust/torrust-tracker/blob/develop/LICENSE) (`AGPL-3.0-only`).
