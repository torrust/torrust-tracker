# Torrust Tracker Events

`torrust-tracker-events` provides the asynchronous event bus used by the Torrust Tracker packages to communicate without depending on each other: broadcaster, sender, receiver and shutdown helpers.

## Where it fits

Domain package used by the core and server packages. Used by: `torrust-tracker`, `torrust-tracker-core`, `torrust-tracker-http-core`, `torrust-tracker-swarm-coordination-registry`, `torrust-tracker-udp-core`, `torrust-tracker-udp-server`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`bus` and `broadcaster` publish events, `sender` and `receiver` are the channel ends, and `shutdown` and `drain` stop listeners cleanly.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-events)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
