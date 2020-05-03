# UDPT
_UDPT_ is a UDP based torrent tracker which fully implements [BEP-15](http://www.bittorrent.org/beps/bep_0015.html).

This project was written in Rust, it is a complete rewrite of a previous C/C++ UDPT project (which is still currently available in the `v2.1` tag of the repository).

## Features
* [X] UDP torrent tracking server
* [X] In memory database
* [X] Choice of Dynamic/Static/Private tracker modes
* [X] Ability to block a torrent from being tracked
* [X] HTTP REST API for management
* [X] Logging
* [ ] Windows Service or Linux/Unix daemon

## Getting started
The easiest way is to get built binaries from [Releases](https://github.com/naim94a/udpt/releases), 
but building from sources should be fairly easy as well:

```commandline
git clone https://github.com/naim94a/udpt.git
cd udpt
cargo build --release
```

## Contributing
Please report any bugs you find to our issue tracker. Ideas and feature requests are welcome as well!

Any pull request targeting existing issues would be very much appreciated. 

### Why was UDPT rewritten in rust?
For a few reasons,
1. Rust makes it harder to make mistakes than C/C++, It provides memory safety without runtime cost.
2. Rust allows easier cross-platform development with it's powerful standard library.
3. Integrated tests and benchmarks.


UDPT was originally developed for fun in 2012 by [@naim94a](https://github.com/naim94a).
