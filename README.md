# Torrust Tracker

[![Build & Release](https://github.com/torrust/torrust-tracker/actions/workflows/build_release.yml/badge.svg)](https://github.com/torrust/torrust-tracker/actions/workflows/build_release.yml) [![CI](https://github.com/torrust/torrust-tracker/actions/workflows/test_build_release.yml/badge.svg)](https://github.com/torrust/torrust-tracker/actions/workflows/test_build_release.yml) [![Publish crate](https://github.com/torrust/torrust-tracker/actions/workflows/publish_crate.yml/badge.svg)](https://github.com/torrust/torrust-tracker/actions/workflows/publish_crate.yml) [![Publish docker image](https://github.com/torrust/torrust-tracker/actions/workflows/publish_docker_image.yml/badge.svg)](https://github.com/torrust/torrust-tracker/actions/workflows/publish_docker_image.yml) [![Test](https://github.com/torrust/torrust-tracker/actions/workflows/test.yml/badge.svg)](https://github.com/torrust/torrust-tracker/actions/workflows/test.yml) [![Test docker build](https://github.com/torrust/torrust-tracker/actions/workflows/test_docker.yml/badge.svg)](https://github.com/torrust/torrust-tracker/actions/workflows/test_docker.yml) [![Coverage](https://github.com/torrust/torrust-tracker/actions/workflows/coverage.yaml/badge.svg)](https://github.com/torrust/torrust-tracker/actions/workflows/coverage.yaml)

Torrust Tracker is a lightweight but incredibly high-performance and feature-rich BitTorrent tracker written in [Rust](https://www.rust-lang.org/).

It aims to provide a reliable and efficient solution for serving torrents to a vast number of peers while maintaining a high level of performance, robustness, extensibility, security, usability and with community-driven development.

## Key Features

* [X] Multiple UDP server and HTTP(S) server blocks for socket binding are possible.
* [X] Full IPv4 and IPv6 support for both UDP and HTTP(S).
* [X] Private & Whitelisted mode.
* [X] Built-in API.
* [X] Torrent whitelisting.
* [X] Peer authentication using time-bound keys.
* [X] [newTrackon](https://newtrackon.com/) check is supported for both HTTP and UDP, where IPv4 and IPv6 are properly handled.
* [X] SQLite3 and MySQL persistence, loading and saving of the torrent hashes and downloads completed count.
* [X] Comprehensive documentation.
* [X] A complete suite of tests. See [code coverage](https://app.codecov.io/gh/torrust/torrust-tracker) report.

## Implemented BEPs

* [BEP 3](https://www.bittorrent.org/beps/bep_0003.html): The BitTorrent Protocol.
* [BEP 7](https://www.bittorrent.org/beps/bep_0007.html): IPv6 Support.
* [BEP 15](http://www.bittorrent.org/beps/bep_0015.html): UDP Tracker Protocol for BitTorrent.
* [BEP 23](http://bittorrent.org/beps/bep_0023.html): Tracker Returns Compact Peer Lists.
* [BEP 27](http://bittorrent.org/beps/bep_0027.html): Private Torrents.
* [BEP 48](http://bittorrent.org/beps/bep_0048.html): Tracker Protocol Extension: Scrape.

## Getting Started

Requirements:

* Rust Stable `1.68`
* You might have problems compiling with a machine or docker container with low resources. It has been tested with docker containers with 6 CPUs, 7.5 GM of memory and 2GB of swap.

You can follow the [documentation](https://docs.rs/torrust-tracker/) to install and use Torrust Tracker in different ways, but if you want to give it a quick try, you can use the following commands:

```s
git clone https://github.com/torrust/torrust-tracker.git \
  && cd torrust-tracker \
  && cargo build --release \
  && mkdir -p ./storage/database \
  && mkdir -p ./storage/ssl_certificates
```

And then run `cargo run` twice. The first time to generate the `config.toml` file and the second time to run the tracker with the default configuration.

After running the tracker these services will be available:

* UDP tracker: `udp://127.0.0.1:6969/announce`.
* HTTP tracker: `http://127.0.0.1:6969/announce`.
* API: `http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken`.

## Documentation

* [Crate documentation](https://docs.rs/torrust-tracker/).
* [API `v1`](https://docs.rs/torrust-tracker/3.0.0-alpha.3/torrust_tracker/servers/apis/v1).
* [HTTP Tracker](https://docs.rs/torrust-tracker/3.0.0-alpha.3/torrust_tracker/servers/http).
* [UDP Tracker](https://docs.rs/torrust-tracker/3.0.0-alpha.3/torrust_tracker/servers/udp).

## Contributing

We welcome contributions from the community!

How can you contribute?

* Bug reports and feature requests.
* Code contributions. You can start by looking at the issues labeled ["good first issues"](https://github.com/torrust/torrust-tracker/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22).
* Documentation improvements. Check the [documentation](https://docs.rs/torrust-tracker/) and [API documentation](https://docs.rs/torrust-tracker/3.0.0-alpha.3/torrust_tracker/servers/apis/v1) for typos, errors, or missing information.
* Participation in the community. You can help by answering questions in the [discussions](https://github.com/torrust/torrust-tracker/discussions).

## License

The project is licensed under a dual license. See [COPYRIGHT](./COPYRIGHT).

## Acknowledgments

This project was a joint effort by [Nautilus Cyberneering GmbH](https://nautilus-cyberneering.de/) and [Dutch Bits](https://dutchbits.nl). Also thanks to [Naim A.](https://github.com/naim94a/udpt) and [greatest-ape](https://github.com/greatest-ape/aquatic) for some parts of the code. Further added features and functions thanks to [Power2All](https://github.com/power2all).
