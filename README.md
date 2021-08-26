# Torrust Tracker
_Torrust Tracker_ is a UDP based torrent tracker which fully implements [BEP-15](http://www.bittorrent.org/beps/bep_0015.html).

Torrust Tracker is a fork from [UDPT](https://github.com/naim94a/udpt) and heavily modified with parts from [Aquatic](https://github.com/greatest-ape/aquatic).

## Features
* [X] UDP torrent tracking server
* [X] In memory database
* [X] Choice of Dynamic/Static/Private tracker modes
* [X] Ability to block a torrent from being tracked
* [X] HTTP REST API for management
* [X] Logging

## Getting started
The easiest way is to get built binaries from [Releases](https://github.com/torrust/torrust-tracker/releases), 
but building from sources should be fairly easy as well:

```commandline
git clone https://github.com/torrust/torrust-tracker.git
cd torrust-tracker
cargo build --release
```

## Contributing
Please report any bugs you find to our issue tracker. Ideas and feature requests are welcome as well!

Any pull request targeting existing issues would be very much appreciated. 

If you like this project and want to buy to original developer of UDPT a coffee, there's a link for that:

<a href="https://www.buymeacoffee.com/naim" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/arial-orange.png" alt="Buy Me A Coffee" height="60px" width="217px"></a>

Credits: [@naim94a](https://github.com/naim94a), [@greatest-ape](https://github.com/greatest-ape), [@WarmBeer](https://github.com/WarmBeer), [@DutchBits](https://github.com/dutchbits)
