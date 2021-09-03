# Tracking Modes
Torrust Tracker currently supports 4 different tracking modes. 

## Public Mode
`mode: public`

In this mode the tracker allows any torrent, even if unknown to be tracked.

## Listed Mode
`mode: listed`

In listed mode, anyone can use the tracker like in public mode. 
Except that torrents must be whitelisted ahead of being announced. 
Torrents can be added to a whitelist using the REST API.

## Private Mode
`mode: private`

Private tracking requires all peers to be authenticated.
Peers can authenticate themselves using a key: `udp://torrust-tracker.com/:key`.
Keys can be created using the REST API.

## PrivateListed Mode
`mode: private_listed`

This mode is a combination of listen and private mode. All peers must authenticate themselves, 
AND the tracker will only track whitelisted torrents. 
