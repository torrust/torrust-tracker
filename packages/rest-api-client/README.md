# Torrust Tracker REST API Client

`torrust-tracker-rest-api-client` is a Rust client for the Torrust Tracker management REST API, built on `reqwest`. It uses the types of `torrust-tracker-rest-api-protocol`.

## Where it fits

Client package. Workspace dependencies: `torrust-tracker-rest-api-protocol`. Used by: `torrust-tracker`, `torrust-tracker-axum-rest-api-server`. See the [package catalog](https://github.com/torrust/torrust-tracker/blob/develop/docs/packages.md#package-catalog) for the full picture.

## Main API

`connection_info` describes the server and token, and `v1` holds the client with one method per endpoint.

## Stability

Versions are independent per package (see the [ADR](https://github.com/torrust/torrust-tracker/blob/develop/docs/adrs/20260629000000_adopt_independent_package_versioning.md)). While the version is `0.x`, the public API can change in any minor release. The crate is developed for the Torrust Tracker first, so use it directly only if you accept that.

## Documentation

- [API documentation](https://docs.rs/torrust-tracker-rest-api-client)

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
