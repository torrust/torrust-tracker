# Torrust Tracker Torrent Repository

A library to provide a torrent repository to the [Torrust Tracker](https://github.com/torrust/torrust-tracker).

Its main responsibilities include:

- Managing Torrent Entries: It stores, retrieves, and manages torrent entries, which are torrents being tracked.
- Persistence: It supports lading tracked torrents from a persistent storage, ensuring that torrent data can be restored across restarts.
- Pagination and sorting: It provides paginated and stable/sorted access to torrent entries.
- Peer management: It manages peers associated with torrents, including removing inactive peers and handling torrents with no peers (peerless torrents).
- Policy handling: It supports different policies for handling torrents, such as persisting, removing, or custom policies for torrents with no peers.
- Metrics: It can provide metrics about the torrents, such as counts or statuses, likely for monitoring or statistics.

This repo is a core component for managing the state and lifecycle of torrents and their peers in a BitTorrent tracker, with peer management, and flexible policies.

## Documentation

[Crate documentation](https://docs.rs/torrust-tracker-torrent-repository).

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
