# UDPT

- Project Page: [github.com/naim94a/udpt](https://github.com/naim94a/udpt)
- Documentation: [naim94a.github.io/udpt](https://naim94a.github.io/udpt)

UDPT is a lightweight torrent tracker that uses the UDP protocol for tracking and fully implements [BEP-15](http://bittorrent.org/beps/bep_0015.html). 
This project was developed with security & simplicity in mind, so it shouldn't be difficult to get a server started.

Unlike most HTTP torrent-trackers, you can save about 50% bandwidth using a UDP tracker.

## Features
- UDP tracking protocol
- Simple [TOML](https://en.wikipedia.org/wiki/TOML) configuration
- [HTTP REST API](./api.md)
- Logging
- Choice to run in *static* or *dynamic* modes
- Blacklist torrents using the REST API
- Can be built/run on many platforms
- (Re)written in [Rust](https://www.rust-lang.org/)

## Licenses
UDPT available under the [MIT license](https://github.com/naim94a/udpt/blob/master/LICENSE).

## About
Originally written in C++ by [@naim94a](https://github.com/naim94a) in 2012 for fun.
