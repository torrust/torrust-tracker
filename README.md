# Torrust Tracker

[![container_wf_b]][container_wf] [![coverage_wf_b]][coverage_wf] [![deployment_wf_b]][deployment_wf] [![testing_wf_b]][testing_wf]

__Torrust Tracker__, is a [BitTorrent][bittorrent] Tracker (a service that matchmakes peers and collects statistics) written in [Rust Language][rust] and [axum] (a modern web application framework). ___This tracker aims to be respectful to established standards, (both [formal][BEP 00] and [otherwise][torrent_source_felid]).___

> This is a [Torrust][torrust] project and is in active development. It is community supported as well as sponsored by [Nautilus Cyberneering][nautilus].

- _We have a [container guide][containers.md] for those who wish to get started with __Docker__ or __Podman___

## Key Features

- [x] High Quality and Modern Rust Codebase.
- [x] [Documentation] Generated from Code Comments.
- [x] [Comprehensive Suit][coverage] of Unit and Functional Tests.
- [x] Good Performance in Busy Conditions.
- [x] Support for `UDP`, `HTTP`, and `TLS` Sockets.
- [x] Native `IPv4` and `IPv6` support.
- [x] Private & Whitelisted mode.
- [x] Tracker Management API.
- [x] Support [newTrackon][newtrackon] checks.
- [x] Persistent `SQLite3` or `MySQL` Databases.

## Implemented BitTorrent Enhancement Proposals (BEPs)
> _[Learn more about BitTorrent Enhancement Proposals][BEP 00]_

- [BEP 03] : The BitTorrent Protocol.
- [BEP 07] : IPv6 Support.
- [BEP 15] : UDP Tracker Protocol for BitTorrent.
- [BEP 23] : Tracker Returns Compact Peer Lists.
- [BEP 27] : Private Torrents.
- [BEP 48] : Tracker Protocol Extension: Scrape.


## Getting Started

### Container Version

The Torrust Tracker is [deployed to DockerHub][dockerhub_torrust_tracker], you can run a demo immediately with the following commands:

#### Docker:

```sh
docker run -it torrust/tracker:develop
```
> Please read our [container guide][containers.md] for more information.

#### Podman:

```sh
podman run -it torrust/tracker:develop
```
> Please read our [container guide][containers.md] for more information.

### Development Version

- Please assure you have the ___[latest stable (or nightly) version of rust][rust]___.
- Please assure that you computer has enough ram. ___Recommended 16GB.___

#### Checkout, Test and Run:

```sh
# Checkout repository into a new folder:
git clone https://github.com/torrust/torrust-tracker.git

# Change into directory and create a empty database file:
cd torrust-tracker
mkdir -p ./storage/tracker/lib/database/
touch ./storage/tracker/lib/database/sqlite3.db

# Check all tests in application:
cargo test --tests --benches --examples --workspace --all-targets --all-features

# Run the tracker:
cargo run
```
#### Customization:

```sh
# Copy the default configuration into the standard location:
mkdir -p ./storage/tracker/etc/
cp ./share/default/config/tracker.development.sqlite3.toml ./storage/tracker/etc/tracker.toml

# Customize the tracker configuration (for example):
vim ./storage/tracker/etc/tracker.toml

# Run the tracker with the updated configuration:
TORRUST_TRACKER_PATH_CONFIG="./storage/tracker/etc/tracker.toml" cargo run
```

_Optionally, you may choose to supply the entire configuration as an environmental variable:_

```sh
# Use a configuration supplied on an environmental variable:
TORRUST_TRACKER_CONFIG=$(cat "./storage/tracker/etc/tracker.toml") cargo run
```

_For deployment you __should__ override the `api_admin_token` by using an environmental variable:_

```sh
# Generate a Secret Token:
gpg --armor --gen-random 1 10 | tee ./storage/tracker/lib/tracker_api_admin_token.secret
chmod go-rwx ./storage/tracker/lib/tracker_api_admin_token.secret

# Override secret in configuration using an environmental variable:
TORRUST_TRACKER_CONFIG=$(cat "./storage/tracker/etc/tracker.toml") \
  TORRUST_TRACKER_API_ADMIN_TOKEN=$(cat "./storage/tracker/lib/tracker_api_admin_token.secret") \
  cargo run
```

> Please view our [crate documentation][documentation] for more detailed instructions.

### Services
The following services are provided by the default configuration:

- UDP _(tracker)_
  - `udp://127.0.0.1:6969/announce`.
- HTTP _(tracker)_
  - `http://127.0.0.1:6969/announce`.
- API _(management)_
  - `http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken`.


## Documentation

- [Management API (Version 1)][api]
- [Tracker (HTTP/TLS)][http]
- [Tracker (UDP)][udp]

## Contributing

This is an open-source community supported project.</br>
We welcome contributions from the community!

__How can you contribute?__

- Bug reports and feature requests.
- Code contributions. You can start by looking at the issues labeled "[good first issues]".
- Documentation improvements. Check the [documentation] and [API documentation] for typos, errors, or missing information.
- Participation in the community. You can help by answering questions in the [discussions].

## License

The project is licensed under a dual license. See [COPYRIGHT].

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

[dockerhub_torrust_tracker]: https://hub.docker.com/r/torrust/tracker/tags

[torrent_source_felid]: https://github.com/qbittorrent/qBittorrent/discussions/19406

[BEP 00]: https://www.bittorrent.org/beps/bep_0000.html
[BEP 03]: https://www.bittorrent.org/beps/bep_0003.html
[BEP 07]: https://www.bittorrent.org/beps/bep_0007.html
[BEP 15]: https://www.bittorrent.org/beps/bep_0015.html
[BEP 23]: https://www.bittorrent.org/beps/bep_0023.html
[BEP 27]: https://www.bittorrent.org/beps/bep_0027.html
[BEP 48]: https://www.bittorrent.org/beps/bep_0048.html

[containers.md]: ./docs/containers.md

[api]: https://docs.rs/torrust-tracker/3.0.0-alpha.11-develop/torrust_tracker/servers/apis/v1
[http]: https://docs.rs/torrust-tracker/3.0.0-alpha.11-develop/torrust_tracker/servers/http
[udp]: https://docs.rs/torrust-tracker/3.0.0-alpha.11-develop/torrust_tracker/servers/udp

[good first issues]: https://github.com/torrust/torrust-tracker/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[documentation]: https://docs.rs/torrust-tracker/
[API documentation]: https://docs.rs/torrust-tracker/3.0.0-alpha.11-develop/torrust_tracker/servers/apis/v1
[discussions]: https://github.com/torrust/torrust-tracker/discussions

[COPYRIGHT]: ./COPYRIGHT

[nautilus]: https://github.com/orgs/Nautilus-Cyberneering/
[Dutch Bits]: https://dutchbits.nl
[Naim A.]: https://github.com/naim94a/udpt
[greatest-ape]: https://github.com/greatest-ape/aquatic
[Power2All]: https://github.com/power2all
