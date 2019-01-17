# Tracking Modes
UDPT currently supports Static & Dynamic tracking modes. 
Private tracking is planned, but isn't yet completely implemented.

## Dynamic Mode
In this mode a tracker allows any torrent, even if unknown to be tracked.
Trackers that run in this mode usually don't know about the contents of the tracked torrent.
In addition, trackers don't usually know anything about the peers.

UDPT supports dynamic mode, and allows blacklisting torrents to avoid copyright infringement.
Torrents can be blacklisted (or "flagged") using the REST API.

## Static Mode
In static mode, anyone can use the tracker like in dynamic mode. 
Except that torrents must be registered ahead of time.

UDPT supports static mode, and torrents can be added or removed using the REST API.

## Private Mode
Private tracking requires all peers to be authenticated.
Some implementations require torrents to be registered, and some do not.
This mode can be either static or dynamic with peer authentication.

UDPT doesn't currently implement private mode, although there are plans to implement private tracking in the future.
