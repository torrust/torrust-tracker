# Torrust Tracker

[![Build Status](https://app.travis-ci.com/torrust/torrust-tracker.svg?branch=master)](https://app.travis-ci.com/torrust/torrust-tracker)

__Torrust Tracker__ is a feature rich UDP based torrent tracker built with Rust.

[documentation](https://torrust.github.io/torrust-tracker/) | [torrust.com]()

## Features
* [X] UDP torrent tracking server
* [X] SQLite database
* [X] 4 Different tracker modes
* [X] HTTP REST API for easy use
* [X] Torrent whitelisting
* [X] Peer authentication using time-bound keys

## BEPs
* [X] [BEP 15](http://www.bittorrent.org/beps/bep_0015.html): UDP Tracker Protocol for BitTorrent
* [X] [BEP 41](http://bittorrent.org/beps/bep_0041.html): UDP Tracker Protocol Extensions


## Getting started
The easiest way is to get built binaries from [Releases](https://github.com/torrust/torrust-tracker/releases),
but building from sources is also possible:

```bash
git clone https://github.com/torrust/torrust-tracker.git
cd torrust-tracker
cargo build --release
```

## Usage
__Notice:__ Skip the first step if you've downloaded the binaries directly.

1. After building __Torrust Tracker__, navigate to the folder.
```bash
cd torrust-tracker/target
```

2. Create a file called `configuration.toml` with the following contents and change the [configuration](https://torrust.github.io/torrust-tracker/CONFIG.html) according to your liking:
```toml
mode = "public"
external_ip = "0.0.0.0" # set this to your external IP

[udp]
bind_address = "0.0.0.0:6969"
announce_interval = 120 # Two minutes

[http]
bind_address = "127.0.0.1:1212"

[http.access_tokens]
someone = "MyAccessToken"
```

3. And run __Torrust Tracker__:
```bash
./torrust-tracker -c configuration.toml
```

## Contributing
Please report any bugs you find to our issue tracker. Ideas and feature requests are welcome as well!
Any pull request targeting existing issues would be very much appreciated.

## Credits
Torrust Tracker was built by [@WarmBeer](https://github.com/WarmBeer)
as a fork from [UDPT](https://github.com/naim94a/udpt): [@naim94a](https://github.com/naim94a)
and heavily modified with parts from [Aquatic](https://github.com/greatest-ape/aquatic): [@greatest-ape](https://github.com/greatest-ape).
