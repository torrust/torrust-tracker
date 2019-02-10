# REST API
The REST API can help you manage UDPT with your own scripts.

__**Notice:**__
The API should only be used in trusted networks. 
APIs should not be exposed directly to the internet, they are intended for internal use only. 

## Endpoints

All Endpoints require a authorization token which must be set in the configuration before running the tracker.

- Listing Torrents
    
    This can be useful if you want to see which torrents are currently registered in any of the tracking modes.
    
    `GET /t?offset=0&limit=1000&token=... HTTP/1.0`
    
    Optional Parameters:
    - `offset` - Offset of the torrent list to return. Default: 0.
    - `limit` - Limit of torrents to output. Between 1 and 4096. Default 1000.
    
- Getting a torrent's stats

    Allows collection of stats from active torrents.
    
    `GET /t/<info_hash>?token=... HTTP/1.0`
    
    This request will return information about the torrent, such as:
    - if the torrent is flagged
    - seeders & leechers
    - times the torrent's download was completed
    
- Performing actions on torrents
    
    `POST /t/<info_hash>?action=<action>&token=... HTTP/1.0`
    
    Valid actions are: `flag`, `unflag`, `add` & `remove`.
    
    `add` & `remove` are only valid for non-dynamic tracking modes.
