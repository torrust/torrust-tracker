# BitTorrent UDP Tracker Protocol

A library with the primitive types and functions used by BitTorrent UDP trackers.

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

## Acknowledgment

Special thanks to [greatest-ape](https://github.com/greatest-ape)
(Joakim Frostegård) for his contributions to the BitTorrent ecosystem and the original
implementation this crate builds upon.

## Documentation

[Crate documentation](https://docs.rs/bittorrent-udp-protocol).

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
