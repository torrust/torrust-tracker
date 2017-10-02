.. title:: UDPT

UDPT
====

.. toctree::
    :maxdepth: 2
    :caption: Contents:
    :numbered:
    :hidden:

    building
    udpt
    udpt.conf
    restapi.rst

UDPT is a lightweight UDP torrent tracker written in C++, it implements BEP15_ of the BitTorrent protocol.
This project was developed with simplicity and security in mind.
Unlike most tracker, UDPT will save you bandwidth from all that TCP overhead.

Features
--------
* UDP protocol based tracker
* :doc:`Simple INI configuration file <udpt.conf>`
* :doc:`HTTP REST API <restapi>`
* SQLite3 database, support for in-*:memory:*.
* Logging
* Linux daemon / Windows Service
* Choice of *static* or *dynamic* tracker modes
* Works with Windows, Linux and FreeBSD

Licenses & 3rd party libraries
------------------------------
UDPT is released under GPLv3_, and uses GPLv3 compatible libraries:

* SQLite3, released under the public domain.
* Boost, released under the `BOOST LICENSE`_.
* libevent, released under the `3-clause BSD license`_.

Contributing
------------
Feel free to submit `issues <https://github.com/naim94a/udpt/issues>`_, `PRs <https://github.com/naim94a/udpt/pulls>`_ or donations!

.. seealso:: `CONTRIBUTING <https://github.com/naim94a/udpt/blob/master/.github/CONTRIBUTING.md>`_

.. image:: _static/bitcoin-qr.png
    :alt: bitcoin:1KMeZvcgnmWdHitu51yEFWBNcSTXL1eBk3

`bitcoin:1KMeZvcgnmWdHitu51yEFWBNcSTXL1eBk3 <bitcoin:1KMeZvcgnmWdHitu51yEFWBNcSTXL1eBk3>`_

Author
------
UDPT's development started November 20th, 2012 by Naim A. (`@naim94a`_).

.. _BEP15: http://www.bittorrent.org/beps/bep_0015.html
.. _GPLv3: https://www.gnu.org/licenses/gpl-3.0.en.html
.. _BOOST LICENSE: http://www.boost.org/LICENSE_1_0.txt
.. _3-clause BSD license: http://libevent.org/LICENSE.txt
.. _@naim94a: https://github.com/naim94a/
