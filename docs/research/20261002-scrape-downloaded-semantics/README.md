---
doc-type: research
status: complete
last-updated-utc: 2026-10-02
semantic-links:
  related-artifacts:
    - docs/issues/open/2406-scrape-ignores-persisted-torrent-downloads/ISSUE.md
    - docs/adrs/20261002173716_scrape_reports_announce_swarm_stats_without_side_effects.md
---

# Scrape `downloaded` Semantics for Torrents Without Active Peers

## Status

Complete — research conducted on 2026-10-02 for issue #2406.

## Question

Should a scrape response report a torrent's historical `downloaded` (UDP `completed`) count when
the torrent has no active peers in the tracker's in-memory swarm, for example after a restart or
after peerless-torrent cleanup? Or should scrape cover only the currently active swarm?

## 1. Specifications

[BEP 48](https://www.bittorrent.org/beps/bep_0048.html) (Draft):

> complete — The number of active peers that have completed downloading.
> incomplete — The number of active peers that have not completed downloading.
> downloaded — The number of peers that have ever completed downloading.

BEP 48 also states that "scrape exchanges have no effect on a peer's participation in a swarm".
It does not define the response for unknown or peerless torrents, and says nothing about
persistence.

[BEP 15](https://www.bittorrent.org/beps/bep_0015.html) lists `seeders`, `completed`, and
`leechers` per info-hash without definitions, and limits a request to about 74 info-hashes.

[BEP 3](https://www.bittorrent.org/beps/bep_0003.html) has no scrape section.

The historical `BitTorrentSpecification` wiki (wiki.theory.org, "Tracker 'scrape' Convention")
says `files` contains "one key/value pair for each torrent for which there are stats" and
defines `downloaded` as the "total number of times the tracker has registered a completion".

Conclusion: `downloaded` is defined as a lifetime counter, while `complete` and `incomplete` are
scoped to active peers. No specification requires persistence.

## 2. Other Trackers

Shallow clones inspected on 2026-10-02.

| Tracker | Commit | Tracks `downloaded` | Peerless torrent in scrape | Survives restart | Purge policy |
| --- | --- | --- | --- | --- | --- |
| opentracker (in-memory) | `e51780b` | Yes (`down_count`) | Non-zero while the entry is in memory | Only with optional `-l <state-file>` | Peerless without downloads after 45 min; with downloads after 24 h idle |
| XBT (MySQL) | `c5bcfe6` | Yes (`completed`) | Persisted count with 0/0 peers; full scrape skips peerless torrents | Yes, loaded at startup | Torrents never removed from memory |
| Ocelot (MySQL, private) | `972c8d6` | Yes (`Snatched`) | Persisted count; unknown info-hashes omitted | Yes, loaded at startup | Removed only when the site deletes the torrent |
| aquatic (in-memory) | `a2ddc4b` | No (always 0) | Zeros | n/a | Swarm removed when empty |
| chihaya (memory/redis) | `878b42e` | No (HTTP omits it) | Zeros | n/a | Swarm deleted when empty |
| webtorrent bittorrent-tracker | `0cec688` | No (`downloaded` = current `complete`) | 0 | No | No deletion code found |
| torrust-actix (in-memory, optional SQL) | `4c5f4cf` | Yes (`TorrentEntry.completed`) | Persistence on: entry kept, real count returned. Default (off): entry purged, zeros returned; scrape never reads the database | Only with `database.persistent = true` (default `false`); all torrents loaded at startup | Persistence off: entry removed when peerless. Persistence on: peers cleared, entry and count kept |

Notes:

- opentracker keeps peerless torrents with `down_count > 0` longer than torrents without
  downloads (`ot_clean.c`), showing intent to preserve the counter for a bounded time.
- XBT (`anonymous_scrape = false`) and Ocelot (passkey) require authentication for scrape by
  default.
- XBT and Ocelot load every torrent at startup. Torrust rejected that approach in #1510 because
  the demo database held about 1.9 billion torrents.
- [torrust-actix](https://github.com/Power2All/torrust-actix) ties scrape to persistence: with
  `database.persistent = true` it loads every torrent with its `completed` count at startup
  (`src/main.rs`, `TorrentTracker::load_torrents`, 100,000-row pages) and keeps peerless entries in
  memory, so scrape returns the historical count; with persistence off, peerless entries are
  removed and scrape returns zeros. Scrape itself only reads memory (`get_torrent_counts` in
  `src/tracker/impls/torrent_tracker_torrents.rs`). Its issue #9 states the intent that torrents
  without peers "only retain their completed downloaded number".

## 3. Related Discussions

- [chihaya#221](https://github.com/chihaya/chihaya/issues/221): keeping inactive swarms was
  judged low-value because chihaya does not track total snatches.
- [aquatic#249](https://github.com/greatest-ape/aquatic/issues/249): HTTP full scrape is
  forbidden for performance/DDoS reasons.
- [aquatic#27](https://github.com/greatest-ape/aquatic/issues/27): relational persistence is
  described as a private-tracker concern.
- No discussion of the privacy of exposing historical `downloaded` through scrape was found.
- [torrust-actix#27](https://github.com/Power2All/torrust-actix/issues/27): per-torrent
  persistence of seeders, leechers, and completed was requested; the maintainer made it optional
  because of database load.

## 4. Relationship to Torrust Announce Behavior

With `persistent_torrent_completed_stat` enabled, a Torrust announce for a torrent absent from
memory loads the persisted count before building the response. The announce response therefore
already exposes the historical count. Before #2406, scrape for the same torrent at the same
moment reported zero, so scrape and announce disagreed about the same swarm.

## 5. Summary

- The specifications support reporting historical `downloaded` for peerless torrents.
- Database-backed trackers do so; in-memory open trackers report only what remains in memory or
  do not track the counter.
- torrust-actix, the closest relative, follows the persistence setting: when an operator enables
  persistence, scrape reports the historical count for peerless torrents; it achieves this by
  eager startup loading, which Torrust rejected in #1510, rather than a lazy lookup.
- Treating scrape as "announce statistics without side effects" (BEP 48) makes scrape consistent
  with the announce response, whichever exposure policy the announce response follows.

## 6. Unverified Items

- Whether opentracker operators commonly use the state file in production.
- How clients use `downloaded` beyond UI display.
- webtorrent swarm deletion was checked only in `server.js` and `lib/server/swarm.js`.
- torrust-actix: only the SQLite loader and saver were read in full; MySQL and PostgreSQL were
  checked by grep. With default `insert_vacant = false`, new torrents may not get a persisted
  row (inferred, not run).
