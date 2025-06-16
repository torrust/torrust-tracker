# Torrust Tracker

[![container_wf_b]][container_wf] [![coverage_wf_b]][coverage_wf] [![deployment_wf_b]][deployment_wf] [![testing_wf_b]][testing_wf]

**Torrust Tracker** is a [BitTorrent][bittorrent] Tracker that matchmakes peers and collects statistics. Written in [Rust Language][rust] with the [Axum] web framework. **This tracker aims to be respectful to established standards, (both [formal][BEP 00] and [otherwise][torrent_source_felid]).**

> This is a [Torrust][torrust] project and is in active development. It is community supported as well as sponsored by [Nautilus Cyberneering][nautilus].

## Key Features

- [x] High Quality and Modern Rust Codebase.
- [x] [Documentation][docs] Generated from Code Comments.
- [x] [Comprehensive Suit][coverage] of Unit and Functional Tests.
- [x] Good Performance in Busy Conditions.
- [x] Support for `UDP`, `HTTP`, and `TLS` Sockets.
- [x] Native `IPv4` and `IPv6` support.
- [x] Private & Whitelisted mode.
- [x] Tracker Management API.
- [x] Support [newTrackon][newtrackon] checks.
- [x] Persistent `SQLite3` or `MySQL` Databases.

## Tracker Demo

Experience the **Torrust Tracker** in action with our comprehensive demo environment! The [Torrust Demo][torrust-demo] repository provides a complete setup showcasing the tracker's capabilities in a real-world scenario.

The demo takes full advantage of the tracker's powerful metrics system and seamless integration with [Prometheus][prometheus]. This allows you to monitor tracker performance, peer statistics, and system health in real-time. You can build sophisticated Grafana dashboards to visualize all aspects of your tracker's operation.

![Sample Grafana Dashboard](./docs/media/demo/torrust-tracker-grafana-dashboard.png)

**Demo Features:**

- Complete Docker Compose setup.
- Pre-configured Prometheus metrics collection.
- Sample Grafana dashboards for monitoring.
- Real-time tracker statistics and performance metrics.
- Easy deployment for testing and evaluation.

Visit the [Torrust Demo repository][torrust-demo] to get started with your own tracker instance and explore the monitoring capabilities.

## Roadmap

Core:

- [ ] New option `want_ip_from_query_string`. See <https://github.com/torrust/torrust-tracker/discussions/532#issuecomment-1836642956>.
- [ ] Peer and torrents specific statistics. See <https://github.com/torrust/torrust-tracker/discussions/139>.

Persistence:

- [ ] Support other databases like PostgreSQL.

Performance:

- [ ] More optimizations. See <https://github.com/torrust/torrust-tracker/discussions/774>.

Protocols:

- [ ] WebTorrent.

Integrations:

- [x] Monitoring (Prometheus).

Utils:

- [ ] Tracker client. WIP.
- [ ] Tracker checker. WIP.

Others:

- [ ] Intensive testing for Windows.
- [ ] Docker images for other architectures.

<https://github.com/orgs/torrust/projects/10/views/6>

## Implemented BitTorrent Enhancement Proposals (BEPs)
>
> _[Learn more about BitTorrent Enhancement Proposals][BEP 00]_

- [BEP 03]: The BitTorrent Protocol.
- [BEP 07]: IPv6 Support.
- [BEP 15]: UDP Tracker Protocol for BitTorrent.
- [BEP 23]: Tracker Returns Compact Peer Lists.
- [BEP 27]: Private Torrents.
- [BEP 48]: Tracker Protocol Extension: Scrape.

## Architecture

![Torrust Tracker Layers with main packages](./docs/media/packages/torrust-tracker-layers-with-packages.png)

There is also extra [documentation about the packages](./docs/packages.md).

## Getting Started

### Container Version

The Torrust Tracker is [deployed to DockerHub][dockerhub], you can run a demo immediately with the following commands:

#### Docker

```sh
docker run -it torrust/tracker:develop
```

> Please read our [container guide][containers.md] for more information.

#### Podman

```sh
podman run -it docker.io/torrust/tracker:develop
```

> Please read our [container guide][containers.md] for more information.

### Development Version

- Please ensure you have the _**[latest stable (or nightly) version of rust][rust]___.
- Please ensure that your computer has enough RAM. _**Recommended 16GB.___

#### Checkout, Test and Run

```sh
# Checkout repository into a new folder:
git clone https://github.com/torrust/torrust-tracker.git

# Change into directory and create an empty database file:
cd torrust-tracker
mkdir -p ./storage/tracker/lib/database/
touch ./storage/tracker/lib/database/sqlite3.db

# Check all tests in application:
cargo test --tests --benches --examples --workspace --all-targets --all-features

# Run the tracker:
cargo run
```

#### Customization

```sh
# Copy the default configuration into the standard location:
mkdir -p ./storage/tracker/etc/
cp ./share/default/config/tracker.development.sqlite3.toml ./storage/tracker/etc/tracker.toml

# Customize the tracker configuration (for example):
vim ./storage/tracker/etc/tracker.toml

# Run the tracker with the updated configuration:
TORRUST_TRACKER_CONFIG_TOML_PATH="./storage/tracker/etc/tracker.toml" cargo run
```

_Optionally, you may choose to supply the entire configuration as an environmental variable:_

```sh
# Use a configuration supplied on an environmental variable:
TORRUST_TRACKER_CONFIG_TOML=$(cat "./storage/tracker/etc/tracker.toml") cargo run
```

_For deployment, you **should** override the `api_admin_token` by using an environmental variable:_

```sh
# Generate a Secret Token:
gpg --armor --gen-random 1 10 | tee ./storage/tracker/lib/tracker_api_admin_token.secret
chmod go-rwx ./storage/tracker/lib/tracker_api_admin_token.secret

# Override secret in configuration using an environmental variable:
TORRUST_TRACKER_CONFIG_TOML=$(cat "./storage/tracker/etc/tracker.toml") \
  TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_API__ACCESS_TOKENS__ADMIN=$(cat "./storage/tracker/lib/tracker_api_admin_token.secret") \
  cargo run
```

> Please view our [crate documentation][docs] for more detailed instructions.

### Services

The following services are provided by the default configuration:

- UDP _(tracker)_
  - `udp://127.0.0.1:6969/announce`.
- HTTP _(tracker)_
  - `http://127.0.0.1:7070/announce`.
- API _(management)_
  - `http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken`.

## Documentation

You can read the [latest documentation][docs] from <https://docs.rs/>.

Some specific sections:

- [Management API (Version 1)][API]
- [Tracker (HTTP/TLS)][HTTP]
- [Tracker (UDP)][UDP]

There is also extra documentation in the [docs](./docs) folder.

## Benchmarking

- [Benchmarking](./docs/benchmarking.md)

## Contributing

We are happy to support and welcome new people to our project. Please consider our [contributor guide][guide.md].</br>
This is an open-source community-supported project. We welcome contributions from the community!

**How can you contribute?**

- Bug reports and feature requests.
- Code contributions. You can start by looking at the issues labeled "[good first issues]".
- Documentation improvements. Check the [documentation][docs] and [API documentation][API] for typos, errors, or missing information.
- Participation in the community. You can help by answering questions in the [discussions].

## License

**Copyright (c) 2023 The Torrust Developers.**

This program is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License][AGPL_3_0] as published by the [Free Software Foundation][FSF], version 3.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License][AGPL_3_0] for more details.

You should have received a copy of the *GNU Affero General Public License* along with this program. If not, see <https://www.gnu.org/licenses/>.

Some files include explicit copyright notices and/or license notices.

### Legacy Exception

For prosperity, versions of Torrust Tracker that are older than five years are automatically granted the [MIT-0][MIT_0] license in addition to the existing [AGPL-3.0-only][AGPL_3_0] license.

## Contributor Agreement

The copyright of the Torrust Tracker is retained by the respective authors.

**Contributors agree:**

- That all their contributions be granted a license(s) **compatible** with the [Torrust Trackers License](#license).
- That all contributors signal **clearly** and **explicitly** any other compilable licenses if they are not: _[AGPL-3.0-only with the legacy MIT-0 exception](#license)_.

**The Torrust-Tracker project has no copyright assignment agreement.**

_We kindly ask you to take time and consider The Torrust Project [Contributor Agreement][agreement.md] in full._

## Acknowledgments

This project was a joint effort by [Nautilus Cyberneering GmbH][nautilus] and [Dutch Bits]. Also thanks to [Naim A.] and [greatest-ape] for some parts of the code. Further added features and functions thanks to [Power2All].

[container_wf]: ../../actions/workflows/container.yaml
[container_wf_b]: ../../actions/workflows/container.yaml/badge.svg
[coverage_wf]: ../../actions/workflows/coverage.yaml
[coverage_wf_b]: ../../actions/workflows/coverage.yaml/badge.svg
[deployment_wf]: ../../actions/workflows/deployment.yaml
[deployment_wf_b]: ../../actions/workflows/deployment.yaml/badge.svg
[testing_wf]: ../../actions/workflows/testing.yaml
[testing_wf_b]: ../../actions/workflows/testing.yaml/badge.svg

[bittorrent]: http://bittorrent.org/
[rust]: https://www.rust-lang.org/
[axum]: https://github.com/tokio-rs/axum
[newtrackon]: https://newtrackon.com/
[coverage]: https://app.codecov.io/gh/torrust/torrust-tracker
[torrust]: https://torrust.com/

[dockerhub]: https://hub.docker.com/r/torrust/tracker/tags

[torrent_source_felid]: https://github.com/qbittorrent/qBittorrent/discussions/19406

[BEP 00]: https://www.bittorrent.org/beps/bep_0000.html
[BEP 03]: https://www.bittorrent.org/beps/bep_0003.html
[BEP 07]: https://www.bittorrent.org/beps/bep_0007.html
[BEP 15]: https://www.bittorrent.org/beps/bep_0015.html
[BEP 23]: https://www.bittorrent.org/beps/bep_0023.html
[BEP 27]: https://www.bittorrent.org/beps/bep_0027.html
[BEP 48]: https://www.bittorrent.org/beps/bep_0048.html

[containers.md]: ./docs/containers.md

[docs]: https://docs.rs/torrust-tracker/latest/
[api]: https://docs.rs/torrust-tracker/latest/torrust_tracker/servers/apis/v1
[http]: https://docs.rs/torrust-tracker/latest/torrust_tracker/servers/http
[udp]: https://docs.rs/torrust-tracker/latest/torrust_tracker/servers/udp

[good first issues]: https://github.com/torrust/torrust-tracker/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[discussions]: https://github.com/torrust/torrust-tracker/discussions

[guide.md]: https://github.com/torrust/.github/blob/main/info/contributing.md
[agreement.md]: https://github.com/torrust/.github/blob/main/info/licensing/contributor_agreement_v01.md

[AGPL_3_0]: ./docs/licenses/LICENSE-AGPL_3_0
[MIT_0]: ./docs/licenses/LICENSE-MIT_0
[FSF]: https://www.fsf.org/

[nautilus]: https://github.com/orgs/Nautilus-Cyberneering/
[Dutch Bits]: https://dutchbits.nl
[Naim A.]: https://github.com/naim94a/udpt
[greatest-ape]: https://github.com/greatest-ape/aquatic
[Power2All]: https://github.com/power2all
[torrust-demo]: https://github.com/torrust/torrust-demo
[prometheus]: https://prometheus.io/
