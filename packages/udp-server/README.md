# Torrust Tracker UDP Server

`torrust-tracker-udp-server` is the BitTorrent UDP tracker server (BEP 15) of the Torrust Tracker. It receives datagrams, dispatches them to `torrust-tracker-udp-core`, bounds request concurrency and sends the responses. Use it only to run a UDP tracker.

## Where it fits

Server package. Workspace dependencies: `torrust-tracker-client-lib`, `torrust-tracker-configuration`, `torrust-tracker-core`, `torrust-tracker-events`, `torrust-tracker-primitives`, `torrust-tracker-swarm-coordination-registry`, `torrust-tracker-udp-core`, `torrust-tracker-udp-protocol`. Used by: `torrust-tracker`, `torrust-tracker-axum-rest-api-server`, `torrust-tracker-rest-api-runtime-adapter`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`server` starts and stops the UDP tracker, `handlers` processes requests, `banning` rate-limits clients, and `container`, `event` and `statistics` wire it into the tracker.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-udp-server)

## Design Decisions

- [UDP server ADR index](docs/adrs/index.md)
- [Bound UDP request concurrency with a task-per-request ring](docs/adrs/20260929181216_bound_udp_request_concurrency_with_task_per_request_ring.md)

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
