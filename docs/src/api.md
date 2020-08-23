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

## Examples
Listing all tracked torrents
```bash
$ curl http://127.0.0.1:1212/t/?token=MyAccessToken
[{"info_hash":"1234567890123456789012345678901234567890","is_flagged":true,"completed":0,"seeders":0,"leechers":0}]
```

Getting information for a specific torrent
```bash
$ curl http://127.0.0.1:1212/t/1234567890123456789012345678901234567890?token=MyAccessToken
{"info_hash":"1234567890123456789012345678901234567890","is_flagged":false,"completed":0,"seeders":0,"leechers":1,"peers":[[{"id":"2d7142343235302d3458295942396f5334af686b","client":"qBittorrent"},{"ip":"192.168.1.6:52391","uploaded":0,"downloaded":0,"left":0,"event":"Started","updated":672}]]}
```

Adding a torrent (non-dynamic trackers) or Unflagging a torrent:
```bash
$ curl -X POST http://127.0.0.1:1212/t/1234567890123456789012345678901234567890?token=MyAccessToken -d "{\"is_flagged\": false}" -H "Content-Type: application/json"
{"status":"ok"}
```

Removing a torrent:
```bash
$ curl -X DELETE http://127.0.0.1:1212/t/1234567890123456789012345678901234567890?token=MyAccessToken
{"status":"ok"}
```

Flagging a torrent:
```bash
$ curl -X POST http://127.0.0.1:1212/t/1234567890123456789012345678901234567890?token=MyAccessToken -d "{\"is_flagged\": true}" -H "Content-Type: application/json"
{"info_hash":"1234567890123456789012345678901234567890","is_flagged":true,"completed":0,"seeders":0,"leechers":0,"peers":[]}
```
