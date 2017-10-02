*************
HTTP Rest API
*************
.. note:: The REST API is only useful in *non-dynamic mode*.

Security Considerations
-----------------------
The REST API of UDPT was meant for use of the tracker's owner.
It will reject any connecting with a source address other than *127.0.0.1*.

User's of the REST API should make sure that unauthorized people won't gain access.

Attempting to access the api server with a BitTorrent client will result in the following message:
"this is a UDP tracker, not a HTTP tracker."

API Methods
-----------
* Adding torrents

    .. code-block:: bash

        curl "http://127.0.0.1:8081/api?action=add&hash=9228628504cc40efa57bf38e85c9e3bd2c572b5b"

* Removing torrents

    .. code-block:: bash

        curl "http://127.0.0.1:8081/api?action=remove&hash=9228628504cc40efa57bf38e85c9e3bd2c572b5b"

With both methods, the response should be:

.. code-block:: json

    {"success":true}

In case of error, you will receive:

.. code-block:: json

    {"error":"failure reason"}

With one of the following reasons:

* failed to add torrent to DB
* invalid info_hash.
* Hash length must be 40 characters.
* failed to remove torrent from DB
* unknown action
