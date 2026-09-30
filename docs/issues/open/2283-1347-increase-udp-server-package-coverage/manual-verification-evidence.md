---
doc-type: manual-verification-evidence
issue: 2283
package: torrust-tracker-udp-server
measured-utc: 2026-09-30T07:42:00Z
---

# UDP Server Manual Verification Evidence

## V1 - Local Tracker UDP Announce

The built tracker executable ran with an ignored isolated configuration at
`.tmp/2283-manual-runtime.toml`, binding UDP to `127.0.0.1:16969` with an isolated SQLite
database under `.tmp/`.

```text
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/2283-manual-runtime.toml" \
  target/debug/torrust-tracker
```

The UDP socket was confirmed bound before sending this real announce through the unified client:

```text
cargo run -q -p torrust-tracker-client --bin tracker_client -- \
  udp announce udp://127.0.0.1:16969/announce \
  2283228322832283228322832283228322832283 \
  --event started --uploaded 0 --downloaded 0 --left 1000 --port 6881 \
  --peer-id ABCDEFGHIJKLMNOPQRST --key 1 --peers-wanted 0
```

**Observed result:**

```json
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":1,"seeders":0,"peers":[]}}
```

## V2 - Local Tracker UDP Scrape

The same running tracker was queried for the announced info hash:

```text
cargo run -q -p torrust-tracker-client --bin tracker_client -- \
  udp scrape 127.0.0.1:16969 2283228322832283228322832283228322832283
```

**Observed result:**

```json
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":0,"completed":0,"leechers":1}]}}
```

The tracker process was stopped after both responses were received. The configuration, database,
and process output remain local ignored verification artifacts.
