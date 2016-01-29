
# UDPT
The UDPT project is a BitTorrent Tracking software.
It uses the UDP protocol (instead of the HTTP protocol) to track
peers downloading the same software. UDPT was written according
to [BEP 15](http://www.bittorrent.org/beps/bep_0015.html) of the BitTorrent standard.

UDPT is designed to run on both Windows and Linux-based platform (It may run on Apple systems too).

### License
UDPT is released under the [GPL](http://www.gnu.org/licenses/gpl-3.0.en.html) license, a copy is included in this repository. 
We use [SQLite3](http://www.sqlite.org/) which is public-domain, and [Boost](http://www.boost.org/) which is released under the [boost license](http://www.boost.org/LICENSE_1_0.txt).

### Building
We didn't really work on creating any installer, at the moment you can just run udpt from anywhere on your filesystem.
Building udpt is pretty straightforward, just download the project or clone the repo:

UDPT requires the SQLite3, boost_program_options and boost_thread develpment packages to be installed.

<pre>
    $ git clone https://github.com/naim94a/udpt.git
    $ cd udpt
    $ make
</pre>

And finally:

<pre>
    $ ./udpt
</pre>

### Links
* Documentation can be found at our wiki: https://github.com/naim94a/udpt/wiki
* If you have any suggestions or find any bugs, please report them here: https://github.com/naim94a/udpt/issues
* Project Page: http://www.github.com/naim94a/udpt

### Author(s)
UDPT was developed by [Naim A.](http://www.github.com/naim94a) at for fun at his free time. 
The development started on November 20th, 2012.
