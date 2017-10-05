.. title:: UDPT Configuration

***********************
UDPT Configuration File
***********************

UDPT's configuration file is called *udpt.conf* by convention and should be located at `/etc/udpt.conf` by default.

udpt.conf is an INI style configuration.

Sections
========

* **Section [db]** - Controls the backend database.

    +-------------------+------------+-------------------------+-------------------------------------------------------+
    | **Key**           | **Type**   | **Example**             | **Description**                                       |
    +-------------------+------------+-------------------------+-------------------------------------------------------+
    | driver            | enum       | "*sqlite*" is currently | Selects the storage backend.                          |
    |                   |            | the only option.        |                                                       |
    +-------------------+------------+-------------------------+-------------------------------------------------------+
    | param             | string     | /var/lib/udpt.db        | Parameter for the storage backend. For SQLite, it's   |
    |                   |            |                         | a filename. *:memory:* can be used for in-memory DB.  |
    +-------------------+------------+-------------------------+-------------------------------------------------------+

* **Section [tracker]** - Settings for the UDP tracker component.

    +-------------------+---------------+----------------+----------------------------------------------------------+
    | **Key**           | **Type**      | **Example**    | **Description**                                          |
    +-------------------+---------------+----------------+----------------------------------------------------------+
    | is_dynamic        | boolean       | *yes*          | Sets the tracker mode to dynamic.                        |
    +-------------------+---------------+----------------+----------------------------------------------------------+
    | port              | int           | *6969*         | UDP port to listen on.                                   |
    +-------------------+---------------+----------------+----------------------------------------------------------+
    | threads           | int           | *4*            | Amount of threads the UDP tracker should use.            |
    +-------------------+---------------+----------------+----------------------------------------------------------+
    | allow_remotes     | boolean       | *yes*          | Allows clients to report IPs other than themselves.      |
    +-------------------+---------------+----------------+----------------------------------------------------------+
    | allow_iana_ips    | boolean       | *no*           | Sets if packets from reserved IPs should be allowed.     |
    +-------------------+---------------+----------------+----------------------------------------------------------+
    | announce_interval | int           | *3600*         | Time for client to wait before asking for peers again.   |
    +-------------------+---------------+----------------+----------------------------------------------------------+
    | cleanup_interval  | int           | *60*           | Timeout to run a database cleanup.                       |
    +-------------------+---------------+----------------+----------------------------------------------------------+

* **Section [apiserver]** - Settings for the backend HTTP REST service.

    +-----------+-----------+---------------+-------------------------------------------+
    | **Key**   | **Type**  | **Example**   | **Description**                           |
    +-----------+-----------+---------------+-------------------------------------------+
    | enable    | boolean   | *yes*         | Sets if the HTTP API should be enabled.   |
    +-----------+-----------+---------------+-------------------------------------------+
    | port      | int       | *8081*        | The port the HTTP API should listen on.   |
    +-----------+-----------+---------------+-------------------------------------------+
    | threads   | int       | *2*           | Thread allocation for the HTTP API.       |
    +-----------+-----------+---------------+-------------------------------------------+

* **Section [logging]** - Logging preferences.

    +-----------+-----------+-------------------+---------------------------------------------------+
    | **Key**   | **Type**  | **Example**       | **Description**                                   |
    +-----------+-----------+-------------------+---------------------------------------------------+
    | filename  | string    | /var/log/udpt.log | The file the log should be appended to.           |
    +-----------+-----------+-------------------+---------------------------------------------------+
    | level     | enum      | info              | Log Level, should be one of:                      |
    |           |           |                   | *fatal*, *error*, *warning*, *info* or *debug*.   |
    +-----------+-----------+-------------------+---------------------------------------------------+

* **Section [daemon]** - Daemon settings.

    +---------------+---------------+-------------------+-------------------------------------------------------+
    | **Key**       | **Type**      | **Example**       | **Description**                                       |
    +---------------+---------------+-------------------+-------------------------------------------------------+
    | chdir         | string        | /opt/udpt         | The directory to chdir to when running as a daemon.   |
    +---------------+---------------+-------------------+-------------------------------------------------------+

Example
=======

.. code-block:: ini

    # This will use a volatile in-memory database.
    [db]
    driver = sqlite
    params = :memory:

    # UDP Tracker configuration
    [tracker]
    is_dynamic = yes
    port = 6969
    threads = 5
    allow_remotes = yes
    allow_iana_ips = no
    announce_interval = 1800
    cleanup_interval = 120

    # API Server
    [apiserver]
    enable = yes
    port = 8081
    threads = 2

    # Daemon chdir settings
    [daemon]
    chdir = /home/udpt/

    # Logging
    [logging]
    filename = /var/log/udpt.log
    level = warning

.. seealso:: :doc:`udpt(8) <udpt>`
