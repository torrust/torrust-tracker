# Torrust Tracker

[![Build Status](https://app.travis-ci.com/torrust/torrust-tracker.svg?branch=master)](https://app.travis-ci.com/torrust/torrust-tracker)

__Torrust Tracker__ is a feature rich UDP based torrent tracker built with Rust.

## Features
* [X] UDP torrent tracking server
* [X] SQLite database
* [X] 4 Different tracker modes
* [X] HTTP REST API for easy use
* [X] Torrent whitelisting
* [X] Peer authentication using time-bound keys

## BEPs
* [BEP 15](http://www.bittorrent.org/beps/bep_0015.html): UDP Tracker Protocol for BitTorrent
* [BEP 41](http://bittorrent.org/beps/bep_0041.html): UDP Tracker Protocol Extensions

## Contributing
Please report any bugs you find to our issue tracker. Ideas and feature requests are welcome as well!
Any pull request targeting existing issues would be very much appreciated.

## Credits
Torrust Tracker was built by [@WarmBeer](https://github.com/WarmBeer) 
as a fork from [UDPT](https://github.com/naim94a/udpt): [@naim94a](https://github.com/naim94a) 
and heavily modified with parts from [Aquatic](https://github.com/greatest-ape/aquatic): [@greatest-ape](https://github.com/greatest-ape).
