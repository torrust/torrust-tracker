.. title:: Building UDPT from source

*************
Building UDPT
*************

Obtaining source code
=====================
UDPT_'s source code is hosted on GitHub, and obviously uses Git as source control.

You can use the following command to clone the UDPT repository to your machine.::

    git clone https://github.com/naim94a/udpt.git

It’s as simple as that!

If you prefer, you can head over to the project on GitHub and download a tarball and execute::

    curl -Lo udpt-master.tgz https://github.com/naim94a/udpt/archive/master.tar.gz
    tar -xvf udpt

Building for Linux & FreeBSD
============================
The author of UDPT used ArchLinux for development, so the documentation assumes you are using ArchLinux. You should be able to get the following instructions to work on most Linux distributions.

.. note:: Package names vary between distributions, for example: The SQLite package (with development headers/libs) on ArchLinux is *sqlite*, where on Debian you’ll need *libsqlite3-dev*.

Installing dependencies
-----------------------
In order to build UDPT you will need:

*Required Dependencies:*

* sqlite_ - Provides the storage backend, at the moment SQLite handles the In-Memory database too.
* libevent_ - Provides an asynchronous HTTP API (It can actually do more than that, be we just use it’s HTTP capabilities).
* boost_ - We just use boost-program_options, it provides configuration file parsing and command line parsing.
* cmake_ - Generates Makefiles for various platforms (and configures)
* gcc - C/C++ compiler, you can use clang if you prefer.
* binutils - Linker

*Optional Dependencies:*

* git_ - Get the source code, make changes and contribute!
* gtest_ - Google’s C++ Test framework.

On ArchLinux::

    pacman -S sqlite libevent boost cmake git gcc binutils git gtest

On Debian::

    sudo apt-get install libsqlite3-dev libevent-dev libboost-dev cmake git gcc binutils libgtest-dev

On FreeBSD (11.1R)::

    pkg install sqlite3 boost-all libevent cmake git binutils llvm40 googletest

Compiling
---------
Okay, now that you have the source code and all the dependencies, you may execute the following::

    cd udpt
    mkdir build-release && cd build-release
    cmake -DCMAKE_BUILD_TYPE=Release ..
    make -j4

This should leave you with a udpt executable file, and optionally a udpt_tests executable. If any of the above instructions failed, please submit a issue to our `issue tracker`_.

If everything succeeded, head over to :doc:`udpt.conf` and get your tracker running!

Building with Docker
====================
Complete working Docker workflow is possible allowing you to start hacking and building without any other requirements or dependencies. All the required libs and build tools are handled by Docker build process.

Using the provided Dockerfile you can build a complete working image with all the required dependencies.

If you're not familiar with Docker, better use Docker Compose to both build and run your source easy and effortlessly.

From the ``docker-compose.yml`` directory, run::

    docker-compose up --build

Skip the ``--build`` switch to launch the last built container image without rebuilding again.

The provided ``docker-compose.yml`` file is configured to:

* Expose daemon's ports to host (using port's defaults). API server is only exposed on 127.0.0.1 to the Docker host.
* Mount your host's ``udpt.conf`` from your source tree inside the container at ``/etc/udpt.conf`` (read-only).
* Start with the ``--interactive`` switch to avoid forking to background, as required with Docker.

To run udpt inside a Docker container, you need to:

* Configure logging to ``/dev/stdout`` to send the program's messages to Docker's standard logging.
* Configure API server to listen to 0.0.0.0 inside the container to be able to contact it from your development host, that is from outside the container.

See the ``docker-compose.yml`` to view and tweak the launch parameters.

Stop the container by ``CTRL+C``'ing it.

Building for Windows
====================
.. note:: This documentation is a work-in-progress. Stay tuned!

.. _UDPT: https://github.com/naim94a/udpt
.. _sqlite: https://www.sqlite.org/
.. _libevent: https://github.com/libevent/libevent
.. _boost: http://www.boost.org/
.. _cmake: https://www.cmake.org/
.. _git: https://git-scm.com/
.. _gtest: https://github.com/google/googletest
.. _issue tracker: https://github.com/naim94a/udpt/issues
