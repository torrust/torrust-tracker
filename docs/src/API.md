# Torrust Tracker REST API

## Endpoints in API (v1)
__Notice:__
This API should not be exposed directly to the internet, it is intended for internal use only. 

All endpoints require an authorization token which must be set in the configuration before running the tracker. 
The default configuration uses `?token=MyAccessToken`.

---
### `GET /api/torrents`
Get a list of all currently tracked torrents.

`Example: GET /api/torrents?token=MyAccessToken`
```json 
[
    {
        "info_hash": "67000d5d41a7da853b78621dc8316e3e0e89ab3c",
        "completed": 0,
        "seeders": 1,
        "leechers": 0
    },
    {
        "info_hash": "e940a7a57294e4c98f62514b32611e38181b6cae",
        "completed": 1,
        "seeders": 2,
        "leechers": 6
    }
]
```
---
### `GET /api/torrent/:info_hash`
Get the detailed info of a torrent hash.

`Example: GET /api/torrent/67000d5d41a7da853b78621dc8316e3e0e89ab3c?token=MyAccessToken`
```json 
{
    "info_hash": "67000d5d41a7da853b78621dc8316e3e0e89ab3c",
    "completed": 0,
    "seeders": 2,
    "leechers": 0,
    "peers": [
        [
            {
                "id": "2d7142343338302d616e33476269721111111111",
                "client": "qBittorrent"
            },
            {
                "ip": "80.80.100.20:1122",
                "updated": 1630621883, // epoch in seconds
                "uploaded": 0,
                "downloaded": 0,
                "left": 0,
                "event": "Started"
            }
        ]
    ]
}
```
---
### `POST /api/whitelist/:info_hash`
__**Notice:**__
whitelist is only used in Listed or PrivateListed mode.

Add a torrent hash to the whitelist.

`Example: POST /api/whitelist/67000d5d41a7da853b78621dc8316e3e0e89ab3c?token=MyAccessToken`
```json 
{
    "status": "ok"
}
```
---
### `DELETE /api/whitelist/:info_hash`
__**Notice:**__
whitelist is only used in Listed or PrivateListed mode.

Remove a torrent hash from the whitelist.

`Example: DELETE /api/whitelist/67000d5d41a7da853b78621dc8316e3e0e89ab3c?token=MyAccessToken`
```json 
{
    "status": "ok"
}
```
---
### `POST /api/key/:seconds_valid`

Generate a new temporary key which can be used like: ```udp://torrust-tracker.com/:key```

`Example: POST /api/key/3600?token=MyAccessToken`
```json 
{
    "key": "2cjvmxYTzG1ESiJBSAn5dxNornXB5CeD",
    "valid_until": 1630627378 // epoch in seconds
}
```
---
### `DELETE /api/key/:key`

Delete a key and immediately revoke its use.

`Example: DELETE /api/key/2cjvmxYTzG1ESiJBSAn5dxNornXB5CeD?token=MyAccessToken`
```json 
{
    "status": "ok"
}
```
