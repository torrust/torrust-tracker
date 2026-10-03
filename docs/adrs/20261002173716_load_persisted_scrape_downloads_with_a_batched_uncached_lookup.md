---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - "issue #2406"
    - docs/research/20261002-scrape-downloaded-semantics/README.md
    - packages/tracker-core/src/scrape_handler.rs
    - packages/tracker-core/src/torrent/persisted_downloads.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - packages/tracker-core/src/databases/driver/mod.rs
    - packages/tracker-core/tests/integration.rs
---

<!-- skill-link: create-adr -->

# Load Persisted Scrape Downloads With a Batched, Uncached Lookup

## Scope

Repository-level. The lookup lives in `tracker-core`, but its database-query bound is deliberately
decoupled from the HTTP and UDP scrape request limits owned by other packages.

## Description

Issue #2406 decided the domain behavior: with `persistent_torrent_completed_stat` enabled, scrape
reports a torrent's persisted `downloaded` count when its swarm is absent from memory, exactly as
an announce would, and without side effects. That behavior contract is not recorded here. It lives
in the `ScrapeHandler` Rustdoc (`packages/tracker-core/src/scrape_handler.rs`) and is specified
by the executable tests named there. The domain research is in
[scrape `downloaded` semantics](../research/20261002-scrape-downloaded-semantics/README.md).

This ADR records how the persisted counts are loaded, which had several viable implementations.

## Agreement

- **Shared lookup.** Announce and scrape read persisted counts through one `PersistedDownloads`
  lookup (`packages/tracker-core/src/torrent/persisted_downloads.rs`), so the persistence rule
  cannot diverge between them.
- **One batch query per scrape.** Scrape collects the authorized, deduplicated info-hashes absent
  from memory and loads their counts with `TorrentMetricsStore::load_torrents_downloads`, one
  `IN (...)` query instead of one query per torrent (N+1).
- **Bounded query size.** Drivers bind at most `MAX_INFO_HASHES_PER_QUERY` (100) info-hashes per
  query and split larger inputs. A full UDP scrape (about 74 info-hashes, BEP 15) fits in one
  query. The bound is independent of protocol request limits (see #2417): those exist for other
  reasons and may grow without changing the database query size.
- **No cache.** Scrape requests are far less frequent than announces, so one query per scrape
  request is acceptable. Add an in-memory cache only if metrics show the scrape database load
  matters.
- **Errors propagate.** A database failure surfaces as `ScrapeError::Database`, mirroring
  `AnnounceError::Database`.

### Alternatives Considered

- **Cache a peerless swarm on scrape.** Avoids repeated database reads, but `TorrentAdded`
  requires a peer announcement, peerless cleanup would emit unmatched `TorrentRemoved` events, and
  with `remove_peerless_torrents = false` scraped entries would never expire. It would also give
  scrape a side effect. Rejected.
- **One query per absent info-hash.** Simplest, but a scrape of N torrents costs N database round
  trips. Rejected.
- **Eager loading of all torrents at startup** (XBT, Ocelot, torrust-actix). Rejected in #1510
  for large databases.

### Consequences

- Scraping torrents absent from memory costs one batch query per request (one per 100
  info-hashes) when persistence is enabled. Scrape-abuse protections are out of scope here.
- Scrape responses can fail with a database error when persistence is enabled.

The decisions are guarded by tests: `it_should_load_the_persisted_downloads_of_all_torrents_absent_from_memory_in_one_query`,
`it_should_look_up_a_repeated_info_hash_absent_from_memory_only_once`,
`it_should_report_an_in_memory_swarm_without_reading_the_persisted_downloads`, and
`it_should_fail_when_loading_the_persisted_downloads_fails` in `scrape_handler.rs`, and
`it_should_load_the_persisted_downloads_of_more_torrents_than_fit_in_one_query` in the shared
driver tests (`packages/tracker-core/src/databases/driver/mod.rs`).

## Date

2026-10-02

## References

- Issue [#2406](https://github.com/torrust/torrust-tracker/issues/2406)
- Issues #1510 and #1543 (startup loading rejected; lazy per-torrent loading introduced)
- Issue [#2417](https://github.com/torrust/torrust-tracker/issues/2417) (HTTP scrape info-hash limit)
- [Events are objective facts](20260727000000_events_are_objective_facts.md)
