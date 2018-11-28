# UDPT
_UDPT_ is a UDP based torrent tracker which fully implements [BEP-15](http://www.bittorrent.org/beps/bep_0015.html) & [BEP-41](http://www.bittorrent.org/beps/bep_0041.html).

This project was written in Rust, it is a complete rewrite of a previous C/C++ UDPT project (which is still currently available in the `master` branch of the repository).

## Features
* UDP torrent tracking server
* In memory database
* Choice of Dynamic/Static/Private tracker modes
* Ability to block a torrent from being tracked
* HTTP REST API for management
* Logging
* Windows Service or Linux/Unix daemon

## Getting started
This rewrite is currently still under development and shouldn't be used at the moment. 
If you'd like to contribute in making everything in the "Features" list come true, please feel free to submit a pull-request.

Since we are using Rust, getting started is fairly easy:
```commandline
git clone https://github.com/naim94a/udpt.git
cd udpt
git checkout udpt-rs
cargo build
```

## Why was this project rewritten in rust?
For a few reasons,
1. Rust makes it harder to make mistakes than C/C++.
2. Rust allows easier cross-platform development with it's powerful standard library.
3. Integrated tests and benchmarks.

## Contributing
Pull Requests are always welcome, please just check with us before developing - creating a issue would be great!

---
The UDPT project would like to thank JetBrains for providing us with a Open Source license for the best IDEs there are out there!
[![JetBrains](.github/jetbrains-variant-4.svg)]((https://www.jetbrains.com/?from=udpt))