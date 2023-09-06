# Torrust Tracker

[![container_wf_b]][container_wf] [![coverage_wf_b]][coverage_wf] [![testing_wf_b]][testing_wf]

Torrust Tracker is a lightweight but incredibly high-performance and feature-rich BitTorrent tracker written in [Rust Language][rust].

It aims to provide a reliable and efficient solution for serving torrents to a vast number of peers while maintaining a high level of performance, robustness, extensibility, security, usability and with community-driven development.

_We have a [container guide][containers.md] to get started with Docker or Podman_

## Key Features

* [x] Multiple UDP server and HTTP(S) server blocks for socket binding are possible.
* [x] Full IPv4 and IPv6 support for both UDP and HTTP(S).
* [x] Private & Whitelisted mode.
* [x] Built-in API.
* [x] Torrent whitelisting.
* [x] Peer authentication using time-bound keys.
* [x] [newTrackon][newtrackon] check is supported for both HTTP and UDP, where IPv4 and IPv6 are properly handled.
* [x] SQLite3 and MySQL persistence, loading and saving of the torrent hashes and downloads completed count.
* [x] Comprehensive documentation.
* [x] A complete suite of tests. See our [code coverage report][coverage].

## Implemented BEPs

* [BEP 03]: The BitTorrent Protocol.
* [BEP 07]: IPv6 Support.
* [BEP 15]: UDP Tracker Protocol for BitTorrent.
* [BEP 23]: Tracker Returns Compact Peer Lists.
* [BEP 27]: Private Torrents.
* [BEP 48]: Tracker Protocol Extension: Scrape.


## Getting Started

Requirements:

* Rust Stable `1.68`
* You might have problems compiling with a machine or docker container with low resources. It has been tested with docker containers with 6 CPUs, 7.5 GM of memory and 2GB of swap.

You can follow the [documentation] to install and use Torrust Tracker in different ways, but if you want to give it a quick try, you can use the following commands:

```s
git clone https://github.com/torrust/torrust-tracker.git \
  && cd torrust-tracker \
  && cargo build --release \
  && mkdir -p ./storage/tracker/lib/database \
  && mkdir -p ./storage/tracker/lib/tls
```

### Configuration

The [default configuration folder: `/share/default/config`][share.default.config]:

- Contains the [development default][src.bootstrap.config.default] i.e: [`tracker.development.sqlite3.toml`][tracker.development.sqlite3.toml].

- Also contains the container defaults: [`sqlite3`][tracker.container.sqlite3.toml] and [`mysql`][tracker.container.mysql.toml].

To override the default configuration there is two options:

- Configure a different configuration path by setting the [`TORRUST_TRACKER_PATH_CONFIG`][src.bootstrap.config.path.config] environmental variable.

- Supply the entire configuration via the [`TORRUST_TRACKER_CONFIG`][src.bootstrap.config.config] environmental variable.


> NOTE: It is recommended for production you override the `api admin token` by placing your secret in the [`ENV_VAR_API_ADMIN_TOKEN`][src.bootstrap.config.admin.token] environmental variable.


### Services
After running the tracker these services will be available (as defined in the default configuration):

* UDP tracker: `udp://127.0.0.1:6969/announce`.
* HTTP tracker: `http://127.0.0.1:6969/announce`.
* API: `http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken`.

## Documentation

* [Crate documentation]
* [API `v1`]
* [HTTP Tracker]
* [UDP Tracker]

## Contributing

We welcome contributions from the community!

How can you contribute?

* Bug reports and feature requests.
* Code contributions. You can start by looking at the issues labeled "[good first issues]".
* Documentation improvements. Check the [documentation] and [API documentation] for typos, errors, or missing information.
* Participation in the community. You can help by answering questions in the [discussions].

## License

The project is licensed under a dual license. See [COPYRIGHT].

## Acknowledgments

This project was a joint effort by [Nautilus Cyberneering GmbH][nautilus] and [Dutch Bits]. Also thanks to [Naim A.] and [greatest-ape] for some parts of the code. Further added features and functions thanks to [Power2All].



[container_wf]: https://github.com/torrust/torrust-tracker/actions/workflows/container.yaml
[container_wf_b]: https://github.com/torrust/torrust-tracker/actions/workflows/container.yaml/badge.svg
[coverage_wf]: https://github.com/torrust/torrust-tracker/actions/workflows/coverage.yaml
[coverage_wf_b]: https://github.com/torrust/torrust-tracker/actions/workflows/coverage.yaml/badge.svg
[testing_wf]: https://github.com/torrust/torrust-tracker/actions/workflows/testing.yaml
[testing_wf_b]: https://github.com/torrust/torrust-tracker/actions/workflows/testing.yaml/badge.svg

[rust]: https://www.rust-lang.org/
[newtrackon]: https://newtrackon.com/
[coverage]: https://app.codecov.io/gh/torrust/torrust-tracker

[BEP 03]: https://www.bittorrent.org/beps/bep_0003.html
[BEP 07]: https://www.bittorrent.org/beps/bep_0007.html
[BEP 15]: http://www.bittorrent.org/beps/bep_0015.html
[BEP 23]: http://bittorrent.org/beps/bep_0023.html
[BEP 27]: http://bittorrent.org/beps/bep_0027.html
[BEP 48]: http://bittorrent.org/beps/bep_0048.html

[containers.md]: ./docs/containers.md

[share.default.config]: ./share/default/config/
[tracker.development.sqlite3.toml]: ./share/default/config/tracker.development.sqlite3.toml
[src.bootstrap.config.default]: ./src/bootstrap/config.rs#L18
[tracker.container.sqlite3.toml]: ./share/default/config/tracker.container.sqlite3.toml
[tracker.container.mysql.toml]: ./share/default/config/tracker.container.mysql.toml
[share.container.entry_script_sh.default]: ./share/container/entry_script_sh#L10

[src.bootstrap.config.path.config]: ./src/bootstrap/config.rs#L15
[src.bootstrap.config.config]: ./src/bootstrap/config.rs#L11
[src.bootstrap.config.admin.token]: ./src/bootstrap/config.rs#L12

[Crate documentation]: https://docs.rs/torrust-tracker/
[API `v1`]: https://docs.rs/torrust-tracker/3.0.0-alpha.4/torrust_tracker/servers/apis/v1
[HTTP Tracker]: https://docs.rs/torrust-tracker/3.0.0-alpha.4/torrust_tracker/servers/http
[UDP Tracker]: https://docs.rs/torrust-tracker/3.0.0-alpha.4/torrust_tracker/servers/udp

[good first issues]: https://github.com/torrust/torrust-tracker/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[documentation]: https://docs.rs/torrust-tracker/
[API documentation]: https://docs.rs/torrust-tracker/3.0.0-alpha.4/torrust_tracker/servers/apis/v1
[discussions]: https://github.com/torrust/torrust-tracker/discussions

[COPYRIGHT]: ./COPYRIGHT

[nautilus]: https://nautilus-cyberneering.de/
[Dutch Bits]: https://dutchbits.nl
[Naim A.]: https://github.com/naim94a/udpt
[greatest-ape]: https://github.com/greatest-ape/aquatic
[Power2All]: https://github.com/power2all
