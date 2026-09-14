---
doc-type: manual-verification-evidence
issue: 2149
package: torrust-tracker-udp-server
measured-commit: 8683d543
measured-utc: 2026-09-14
---

# UDP Server Manual Verification Evidence

## M1 - Local Tracker UDP Announce

The built tracker executable was started with the isolated configuration at
`.tmp/2149-manual-runtime.toml`, binding UDP to `127.0.0.1:16969`:

```text
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/2149-manual-runtime.toml" \
  target/debug/torrust-tracker
```

The tracker log recorded `Started UDP tracker service_binding=udp://127.0.0.1:16969`. The unified
client then sent a real UDP announce:

```text
cargo run -q -p torrust-tracker-client --bin tracker_client -- \
  udp announce udp://127.0.0.1:16969/announce \
  2149214921492149214921492149214921492149 \
  --event started --uploaded 0 --downloaded 0 --left 1000 --port 6881 \
  --peer-id ABCDEFGHIJKLMNOPQRST --key 1 --peers-wanted 0
```

**Observed result:** the command returned:

```json
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":1,"seeders":0,"peers":[]}}
```

The tracker was then stopped with `Ctrl-C`; its log records cancellation for the tracker core and
both UDP-server event listeners. The isolated configuration, SQLite database, and log remain only
under ignored `.tmp/` storage.

## M2 - Full Package Regression

The complete package test command was invoked directly:

```text
cargo test -p torrust-tracker-udp-server
```

**Observed result:** 170 package unit tests, 11 package integration tests, and one documentation
test passed. Expected test fixture logs include error/warning paths that are deliberately exercised
by negative protocol and connection-cookie contracts; no test failed.

## Conclusion

The manual local-tracker announce exercised the finished executable and unified client over UDP.
The separate full package regression command passed after the final direct-event processor test
cleanup.
