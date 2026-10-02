---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - docs/research/20261002-scrape-downloaded-semantics/README.md
    - docs/issues/open/2406-scrape-ignores-persisted-torrent-downloads/ISSUE.md
---

<!-- skill-link: create-adr -->

# Scrape Reports Announce Swarm Statistics Without Side Effects

## Scope

Repository-level. The decision defines the observable contract of both HTTP and UDP scrape, which
spans `tracker-core`, `http-core`, `axum-http-server`, and `udp-server`.

## Description

With `persistent_torrent_completed_stat` enabled, the tracker persists each torrent's completed
download count. Announce lazily reloads that count when the torrent is absent from memory, so the
announce response reports the historical count. Scrape read only memory and reported zero for the
same torrent until an announce happened (#2406).

The maintainer considered limiting scrape to the active in-memory swarm for scalability and
privacy. Research ([scrape `downloaded` semantics](../research/20261002-scrape-downloaded-semantics/README.md))
found that BEP 48 defines `downloaded` as a lifetime counter, that scrape "has no effect on a
peer's participation in a swarm", and that database-backed trackers report persisted counts for
peerless torrents.

## Agreement

A scrape reports, for each requested torrent, the same swarm statistics an announce would return
at that moment, without any side effect:

- `complete` and `incomplete` reflect active peers in memory.
- `downloaded` follows the announce rule: the in-memory value when the swarm is present;
  otherwise the persisted value when `persistent_torrent_completed_stat` is enabled; otherwise
  zero.
- Scrape does not insert swarms, emit swarm events, or change retention state. A missing
  persisted row yields zero.
- Announce and scrape share one lookup policy for the persisted count, so they cannot diverge.
- A database failure during the lookup is propagated as a scrape error, as announce does.

Whether persisted counts should be exposed in responses at all is a policy that must apply to
announce and scrape together. Any future exposure setting must govern both.

### Alternatives Considered

- **Cache a peerless swarm on scrape.** Avoids repeated database reads, but `TorrentAdded`
  requires a peer announcement, peerless cleanup would emit unmatched `TorrentRemoved` events, and
  with `remove_peerless_torrents = false` scraped entries would never expire. Rejected for this
  bug fix.
- **Scrape reports only in-memory state.** Keeps the pre-#2406 behavior, but scrape and announce
  then disagree about the same swarm, and counts reset to zero after a restart or cleanup.
  Rejected.

### Consequences

- Scraping a torrent absent from memory costs one database read per info-hash when persistence is
  enabled, as a first announce does. Scrape-abuse protections are out of scope here.
- Scrape responses can fail with a database error when persistence is enabled.

## Date

2026-10-02

## References

- Issue [#2406](https://github.com/torrust/torrust-tracker/issues/2406)
- Issues #1510 and #1543 (startup loading rejected; lazy per-torrent loading introduced)
- [BEP 48](https://www.bittorrent.org/beps/bep_0048.html)
- [Events are objective facts](20260727000000_events_are_objective_facts.md)
