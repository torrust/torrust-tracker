# Manual Verification Evidence

Recorded 2026-11-05 (UTC) by Claude Code (Claude Sonnet 5.5) at branch commit `bcd7982d0` (on top of `torrust/develop`
`47ed159be`), Rust dev profile build of `torrust-tracker`. The development token `MyAccessToken` is public and documented.
Same setup as the #2719 evidence: persistence off is `tracker.container.no-persistence.toml` plus an appended `[http_api]`
section; persistence on is `tracker.development.sqlite3.toml` with the SQLite path moved to a scratch directory. Prometheus and
`promtool` 3.5.0 are `prom/prometheus:v3.5.0` (image id `sha256:a3bc50fcb50fb1409287c32773d8fe799f6cad5511c4ba3e2e851784e7a05026`).

## M1 - Raw text, persistence off and on

```bash
./target/debug/torrust-tracker --config-toml-path on.toml &
curl -si 'http://localhost:1212/api/v1/stats?format=prometheus&token=MyAccessToken'
```

```text
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 5000

# HELP torrents Total number of torrents.
# TYPE torrents gauge
torrents 0
# HELP seeders Total number of seeders for all torrents.
# TYPE seeders gauge
seeders 0
# HELP completed Deprecated ambiguous completed-download total. Use completed_in_session and completed_persisted instead.
# TYPE completed counter
completed 0
...
```

Both bodies have 32 `# HELP`, 32 `# TYPE` and 32 sample lines, the last byte is `0x0a`, and the sample lines have the same
names, order and values as before (`completed_persisted_enabled 1` with persistence on, `0` off).

## M2 - `promtool check metrics` 3.5.0

Both bodies: exit code `3`, no "no help text" and no type warning. The 28 remaining warnings are naming-convention lint only:

- 24 `counter metrics should have "_total" suffix` (`completed`, `completed_in_session`, `completed_persisted`, `tcp4_*_handled`,
  `tcp6_*_handled`, `udp4_*`, `udp6_*`, `udp_requests_aborted`, `udp_requests_banned`, `udp_requests_discarded`).
- 3 `metric names should not contain abbreviated units` (`udp_avg_{connect,announce,scrape}_processing_time_ns`).
- 1 `non-counter metrics should not have "_total" suffix` (`udp_banned_ips_total`, a gauge).

Exit code `0` is not reachable without renaming metrics. The names are part of API v1, so they stay; the maintainer decided on
2026-11-05 to keep them, amend AC4, and note the rename for the next API version (see the spec, "Next API version").

## M3 - Real Prometheus 3.5.0 scraping the tracker

Prometheus scraped `http://127.0.0.1:1212/api/v1/stats` with `params: {format: [prometheus], token: [MyAccessToken]}` at a 1 s
interval (persistence on).

| Check                                                      | Result  |
| ---------------------------------------------------------- | ------- |
| target health (`/api/v1/targets`)                          | `up`, no `lastError` |
| `scrape_samples_scraped`                                   | `32`    |
| types in `/api/v1/metadata` for the 32 names               | 24 `counter`, 8 `gauge` |

Before the change (#2719 evidence V5) the same scrape was `up` with 32 samples and every type `unknown`.

## Regression tests

Red at `299aaafc0` (`2 failed`: the exact-text test and the new HELP/TYPE structure test), green at `5af51fc73`
(`cargo test -p torrust-tracker-axum-rest-api-server -p torrust-tracker-rest-api-protocol`: 160 + 58 + 3 passed, 0 failed).
