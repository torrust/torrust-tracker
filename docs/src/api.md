# REST API
The REST API can help you manage UDPT with your own scripts.

__**Notice:**__
The API should only be used in trusted networks. 
APIs should not be exposed directly to the internet, they are intended for internal use only. 

## Endpoints

All Endpoints require a authorization token which must be set in the configuration before running the tracker.

| Method   | Route            | Description           |
| --       | --               | --                    |
| `GET`    | /t               | list all tracked torrents. Possible query parameters are: <br /> _offset_ - The offset in the db where to start listing torrents from.<br />_limit_ - Maximum amount of records to retrieve (max. 1000). |
| `GET`    | /t/_infohash_    | get information about a specific torrent: connected peers & stats |
| `DELETE` | /t/_infohash_    | drop a torrent from the database. |
| `POST`   | /t/_infohash_    | add/flag/unflag torrent |

The payload expected for adding a torrent can be empty, flagging or unflagging a torrent has the following payload:
```json
{
    "is_flagged": false
}
```
