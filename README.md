# Torrust Tracker
![Test](https://github.com/torrust/torrust-tracker/actions/workflows/test_build_release.yml/badge.svg)

## Project Description
Torrust Tracker is a lightweight but incredibly powerful and feature-rich BitTorrent tracker made using Rust.


### Features
* [X] Multiple UDP server and HTTP(S) server blocks for socket binding possible
* [X] Full IPv4 and IPv6 support for both UDP and HTTP(S)
* [X] Private & Whitelisted mode
* [X] Built-in API
* [X] Torrent whitelisting
* [X] Peer authentication using time-bound keys
* [X] newTrackon check supported for both HTTP, UDP, where IPv4 and IPv6 is properly handled
* [X] SQLite3 Persistent loading and saving of the torrent hashes and completed count
* [X] MySQL support added as engine option
* [X] Periodically saving added, interval can be configured

### Implemented BEPs
* [BEP 3](https://www.bittorrent.org/beps/bep_0003.html): The BitTorrent Protocol
* [BEP 7](https://www.bittorrent.org/beps/bep_0007.html): IPv6 Support
* [BEP 15](http://www.bittorrent.org/beps/bep_0015.html): UDP Tracker Protocol for BitTorrent
* [BEP 23](http://bittorrent.org/beps/bep_0023.html): Tracker Returns Compact Peer Lists
* [BEP 27](http://bittorrent.org/beps/bep_0027.html): Private Torrents
* [BEP 41](http://bittorrent.org/beps/bep_0041.html): UDP Tracker Protocol Extensions
* [BEP 48](http://bittorrent.org/beps/bep_0048.html): Tracker Protocol Extension: Scrape

## Getting Started
You can get the latest binaries from [releases](https://github.com/torrust/torrust-tracker/releases) or follow the install from scratch instructions below.

### Install From Scratch
1. Clone the repo.
```bash
git clone https://github.com/torrust/torrust-tracker.git
cd torrust-tracker
```

2. Build the source code.
```bash
cargo build --release
```

### Usage
* Run the torrust-tracker once to create the `config.toml` file:
```bash
./target/release/torrust-tracker
```


* Edit the newly created config.toml file according to your liking, see [configuration documentation](https://torrust.github.io/torrust-documentation/torrust-tracker/config/). Eg:
```toml
log_level = "info"
mode = "public"
db_path = "data.db"
persistence = false
cleanup_interval = 600
cleanup_peerless = true
external_ip = "0.0.0.0"
announce_interval = 120
announce_interval_min = 900
peer_timeout = 900
on_reverse_proxy = false

[[udp_trackers]]
enabled = false
bind_address = "0.0.0.0:6969"

[[udp_trackers]]
enabled = true
bind_address = "[::]:6969"

[[http_trackers]]
enabled = true
bind_address = "0.0.0.0:6969"
ssl_enabled = false
ssl_bind_address = "0.0.0.0:6868"
ssl_cert_path = ""
ssl_key_path = ""

[http_api]
enabled = true
bind_address = "127.0.0.1:1212"

[http_api.access_tokens]
admin = "MyAccessToken"
```


* Run the torrust-tracker again:
```bash
./target/release/torrust-tracker
```

### Tracker URL
Your tracker announce URL will be **udp://{tracker-ip:port}** and/or **http://{tracker-ip:port}/announce** and/or **https://{tracker-ip:port}/announce** depending on your bindings.
In private & private_listed mode, tracker keys are added after the tracker URL like: **https://{tracker-ip:port}/announce/{key}**.

### Built-in API
Read the API documentation [here](https://torrust.github.io/torrust-documentation/torrust-tracker/api/).

### Credits
This project was a joint effort by [Nautilus Cyberneering GmbH](https://nautilus-cyberneering.de/) and [Dutch Bits](https://dutchbits.nl).
Also thanks to [Naim A.](https://github.com/naim94a/udpt) and [greatest-ape](https://github.com/greatest-ape/aquatic) for some parts of the code.
Further added features and functions thanks to [Power2All](https://github.com/power2all).
