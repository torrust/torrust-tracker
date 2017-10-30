# UDPT
**UDP**-**T**racker is a torrent tracker that implements [BEP15](http://www.bittorrent.org/beps/bep_0015.html),
the UDP torrent tracker protocol. 

The UDP tracker protocol is light compared to HTTP(s) based torrent 
trackers since  it doesnt have TCP's overhead.

This project was developed with simplicity and security in mind.
Development started November 20th, 2012 by [@naim94a](https://github.com/naim94a).

## Features
* UDP torrent tracking server
* SQLite3 database, with in-memory support (volatile)
* Choice of static or dynamic tracker modes
* HTTP REST API
* Logging
* Windows Service / Linux Daemon
* INI like configuration syntax

## Getting Started
The easiest way is to download binaries from the [Releases Section](https://github.com/naim94a/udpt/releases),
but releases don't get updated as often as the master branch...

### Getting the code
1. Make sure you have the following binaries, they are required to build UDPT: *All packages should be in most linux disto's official repositories*
    * cmake
    * make
    * g++, gcc, ld
    * boost_program-options, boost_system
    * libsqlite3
    * libevent
    * gtest - optional
    
2. Obtain the code: `git clone https://github.com/naim94a/udpt.git`

3. And start building!
    ```sh
    cd udpt
    mkdir build && cd build
    cmake ..
    make udpt
    ```

4. Finally, start the server:
    ```sh
    ./udpt -ic ../udpt.conf
    ```
    Now you can get people to use your tracker at: udp://*<YOUR_IP>*:6969/

You should note that the default configuration does not use a persistent database.

### Links
* UDPT's documentation can be found in the docs directory, or the rendered version at [naim94a.github.io/udpt](https://naim94a.github.io/udpt). 
* If you have any suggestions or find any bugs, please report them here: https://github.com/naim94a/udpt/issues
* Project Page: http://www.github.com/naim94a/udpt

## How to Contribute
**Donations** are the best way to contribute, we accept BitCoin:

<a href="bitcoin://1KMeZvcgnmWdHitu51yEFWBNcSTXL1eBk3">bitcoin:1KMeZvcgnmWdHitu51yEFWBNcSTXL1eBk3</a>

![bitcoin:1KMeZvcgnmWdHitu51yEFWBNcSTXL1eBk3](.github/bitcoin-qr.png)

[**Issues**](https://github.com/naim94a/udpt/issues), 
[**Pull-Requests**](https://github.com/naim94a/udpt/pulls) 
and suggestions are welcome as well.
See our [CONTRIBUTING](.github/CONTRIBUTING.md) page for more information.
