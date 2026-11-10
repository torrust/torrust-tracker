# Torrust Tracker Client Library

`torrust-tracker-client-lib` is a Rust client library for BitTorrent trackers over UDP and HTTP: announce and scrape requests with generic clients. It is published as `torrust-tracker-client-lib`.

## Where it fits

Client package. Workspace dependencies: `torrust-tracker-http-protocol`, `torrust-tracker-udp-protocol`. Used by: `torrust-tracker-client`, `torrust-tracker-udp-server`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`udp` and `http` hold the clients for each protocol and `peer_id` builds peer ids.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-client-lib)

## License

This crate is licensed `LGPL-3.0-only`.

**Copyright (c) 2024 The Torrust Developers.**

This program is free software: you can redistribute it and/or modify it under the terms of the [GNU Lesser General Public License][LGPL_3_0] as published by the [Free Software Foundation][FSF], version 3.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Lesser General Public License][LGPL_3_0] for more details.

You should have received a copy of the _GNU Lesser General Public License_ along with this program. If not, see <https://www.gnu.org/licenses/>.

Some files include explicit copyright notices and/or license notices.

### Legacy Exception

For prosperity, versions of Torrust BitTorrent Tracker Client that are older than five years are automatically granted the [MIT-0][MIT_0] license in addition to the existing [LGPL-3.0-only][LGPL_3_0] license.

[LGPL_3_0]: https://www.gnu.org/licenses/lgpl-3.0.html
[MIT_0]: https://github.com/torrust/torrust-tracker/blob/develop/docs/licenses/LICENSE-MIT_0
[FSF]: https://www.fsf.org/
