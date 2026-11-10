# Torrust Tracker UDP Protocol

`torrust-tracker-udp-protocol` provides the types and functions for the BitTorrent UDP tracker protocol (BEP 15): connect, announce and scrape requests and responses with their binary parsing and writing. It has no socket or tracker logic, so other BitTorrent tools can reuse it.

## Where it fits

Protocol package (`*-protocol`) used by `torrust-tracker-udp-core`, the UDP server and the client library. Used by: `torrust-tracker-client`, `torrust-tracker-client-lib`, `torrust-tracker-udp-core`, `torrust-tracker-udp-server`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`request` and `response` are the entry points, and `connect`, `announce` and `scrape` hold the message types.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-udp-protocol)

## Origin and In-House Maintenance

This crate was originally derived from Aquatic's `udp_protocol` crate:

- https://github.com/greatest-ape/aquatic/tree/master/crates/udp_protocol

Torrust keeps an in-house copy because upstream maintenance appears inactive and the tracker
still needs dependency updates, security maintenance, and ongoing protocol-related evolution.

Relevant upstream context:

- https://github.com/greatest-ape/aquatic/issues/224
- https://github.com/greatest-ape/aquatic/pull/235

## Licensing and Notices

The original source is Apache-2.0 licensed. The in-house package keeps the required origin and
change notices in code headers, consistent with the license terms.

An explicit copy of Apache-2.0 is included at [LICENSE-APACHE](./LICENSE-APACHE).

## Acknowledgment

Special thanks to [greatest-ape](https://github.com/greatest-ape)
(Joakim Frostegård) for his contributions to the BitTorrent ecosystem and the original
implementation this crate builds upon.

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE) (`AGPL-3.0-only`).
