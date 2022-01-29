# Torrust Tracker
![Test](https://github.com/torrust/torrust-tracker/actions/workflows/test_build_release.yml/badge.svg)

## Project Description
Torrust Tracker is a lightweight but incredibly powerful and feature-rich BitTorrent tracker made using Rust.


### Features
* [X] UDP server
* [X] HTTP (optional SSL) server
* [X] Private & Whitelisted mode
* [X] Built-in API
* [X] Torrent whitelisting
* [X] Peer authentication using time-bound keys

### Implemented BEPs
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
1. Run the torrust-tracker once to create the `config.toml` file:
```bash
./target/release/torrust-tracker
```


2. Edit the newly created config.toml file according to your liking, see [configuration documentation](https://torrust.github.io/torrust-documentation/torrust-tracker/config/).


3. Run the torrust-tracker again:
```bash
./target/release/torrust-tracker
```

### Tracker URL
Your tracker announce URL will be **udp://{tracker-ip:port}** or **https://{tracker-ip:port}/announce** depending on your tracker mode.
In private & private_listed mode, tracker keys are added after the tracker URL like: **https://{tracker-ip:port}/announce/{key}**.

### Built-in API
Read the API documentation [here](https://torrust.github.io/torrust-documentation/torrust-tracker/api/).

### Credits
This project was a joint effort by [Nautilus Cyberneering GmbH](https://nautilus-cyberneering.de/) and [Dutch Bits](https://dutchbits.nl).
Also thanks to [Naim A.](https://github.com/naim94a/udpt) and [greatest-ape](https://github.com/greatest-ape/aquatic) for some parts of the code.
