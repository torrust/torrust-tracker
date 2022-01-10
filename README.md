# Torrust Tracker
![Test](https://github.com/torrust/torrust-tracker/actions/workflows/test.yml/badge.svg)

## Project Description
Torrust Tracker is a lightweight but incredibly powerful and feature-rich BitTorrent tracker made using Rust.


### Features
* [X] UDP server
* [X] HTTP (optional SSL) server
* [X] Private & Whitelisted mode
* [X] API Hooks
* [X] Torrent whitelisting
* [X] Peer authentication using time-bound keys

### Implemented BEPs
* [BEP 15](http://www.bittorrent.org/beps/bep_0015.html): UDP Tracker Protocol for BitTorrent
* [BEP 23](http://bittorrent.org/beps/bep_0023.html): Tracker Returns Compact Peer Lists
* [BEP 27](http://bittorrent.org/beps/bep_0027.html): Private Torrents
* [BEP 41](http://bittorrent.org/beps/bep_0041.html): UDP Tracker Protocol Extensions
* [BEP 48](http://bittorrent.org/beps/bep_0048.html): Tracker Protocol Extension: Scrape

## Getting Started
You can get the latetst binaries from [releases](https://github.com/torrust/torrust-tracker/releases) or follow the install instructions below.

### Install

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
1. After building __Torrust Tracker__, navigate to the folder.
```bash
cd torrust-tracker/target
```

2. Create a file called `configuration.toml` with the following contents and change the [configuration](https://torrust.com/torrust-tracker/CONFIG.html) according to your liking:
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

### Tracker URL
Your tracker will be `udp://tracker-ip:port` or `https://tracker-ip:port` depending on your tracker mode.
In private mode, tracker keys are added after the tracker URL like: `https://tracker-ip:port/tracker-key`.

### Credits
This project was a joint effort by [Nautilus Cyberneering GmbH](https://nautilus-cyberneering.de/) and [DUTCH BITS](https://dutchbits.nl).
