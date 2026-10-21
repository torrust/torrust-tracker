---
doc-type: manual-verification-evidence
issue-spec: docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/ISSUE.md
last-updated-utc: 2026-10-21 18:50
---

# Manual Verification Evidence

## Purpose

Record the reproduction of the `tcp4`/`tcp6` counter mismatch for an IPv6 client behind a reverse proxy,
before the fix is decided. This is evidence from commands actually run against the tracker.

## Preserving Verification Artifacts

The tracker configuration, the statistics before and after the request, and the full tracker log are stored in
[evidence/](evidence/). The configuration is also embedded below.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-21 18:49 to 18:50
- Artifact under test: `main` at `torrust/develop` as of the draft branch `docs/draft-http-core-tcp-family-counters`
  (no product code changed for this run).
- Operating system / environment: Linux, local development build (`cargo build --bin torrust-tracker`,
  `optimized + debuginfo` profile), tracker run in the foreground from the repository root.
- Prerequisites and setup performed:
  - Copied the layout of the schema 3.0.0 config into [evidence/tracker.toml](evidence/tracker.toml) and set
    `on_reverse_proxy = true` on the HTTP tracker, which is the only setting that makes the client address come from
    `X-Forwarded-For` (`packages/http-protocol/src/v1/services/peer_ip_resolver.rs`).
  - The HTTP tracker is bound to IPv4 (`127.0.0.1:7070`), so the server binding is IPv4.
  - The REST API is on `127.0.0.1:1212` with the development token from the config.
  - A first attempt put `on_reverse_proxy` under `[core.net]`. Startup failed with `unknown field: found net`
    (see the failed attempt in the Failures and Follow-up section). The working config puts it under
    `[http_trackers.network]`.

### Tracker configuration (embedded verbatim)

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
inactive_peer_cleanup_interval = 120
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "./.tmp/tcp-family-repro/sqlite3.db"

[core.tracker_policy]
max_peer_timeout = 60
persistent_torrent_completed_stat = true
remove_peerless_torrents = true

[[http_trackers]]
bind_address = "127.0.0.1:7070"
tracker_usage_statistics = true

[http_trackers.network]
on_reverse_proxy = true

[http_api]
bind_address = "127.0.0.1:1212"

[http_api.access_tokens]
admin = "MyAccessToken"
```

## Verification Processes

### V1 - Announce with an IPv6 forwarded client address to an IPv4-bound HTTP tracker

- Goal: Determine whether an announce whose client address is IPv6 (through `X-Forwarded-For`) is counted under
  `tcp6_announces_handled` or `tcp4_announces_handled`, which is the scenario the spec's Bug-Fix Process describes.
- Initial state: tracker started with the config above; statistics read before the request
  ([evidence/before.txt](evidence/before.txt)). `tcp4_announces_handled` and `tcp6_announces_handled` were both 0.
- Status: `DONE`

#### Steps Performed

1. Start the tracker (run in the foreground, then health check):

   ```sh
   ./target/debug/torrust-tracker --config-toml-path "$PWD/.tmp/tcp-family-repro/tracker.toml"
   curl -s -o /dev/null -w "api:%{http_code}\n" "http://127.0.0.1:1212/api/health_check"
   ```

   Result: `api:200`.

2. Read the baseline statistics:

   ```sh
   curl -s "http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken"
   ```

3. Send one announce with an IPv6 client address in the forwarded header:

   ```sh
   IH="%01%02%03%04%05%06%07%08%09%0a%0b%0c%0d%0e%0f%10%11%12%13%14"
   PID="-qB0000-123456789012"
   curl -s -o announce.bin -w "announce:%{http_code}\n" \
     -H "X-Forwarded-For: 2001:db8::1" \
     "http://127.0.0.1:7070/announce?info_hash=$IH&peer_id=$PID&port=6881&uploaded=0&downloaded=0&left=0&compact=1&event=started"
   ```

   Result: `announce:200`. The response body is a bencoded announce response (`d8:completei1e10:incompletei0e8:intervali120e...`).

4. Read the statistics again, after the request ([evidence/after.txt](evidence/after.txt)).

5. Stop the tracker with `kill <PID>` and confirm the process is gone.

#### Observed Result

Baseline (step 2), only the TCP and the relevant UDP fields are shown:

```text
{'tcp4_connections_handled': 0, 'tcp4_announces_handled': 0, 'tcp4_scrapes_handled': 0, 'tcp6_connections_handled': 0, 'tcp6_announces_handled': 0, 'tcp6_scrapes_handled': 0}
```

After the IPv6 forwarded announce (step 4):

```text
{'tcp4_connections_handled': 1, 'tcp4_announces_handled': 1, 'tcp4_scrapes_handled': 0, 'tcp6_connections_handled': 0, 'tcp6_announces_handled': 0, 'tcp6_scrapes_handled': 0}
```

The UDP counters are unchanged at 0 (`udp4_*`, `udp6_*`, see [evidence/after.txt](evidence/after.txt)).

The tracker log ([evidence/tracker.log](evidence/tracker.log)) records the announce request
(`request{method=GET uri=/announce?info_hash=...&event=started...}`) and no error. The log does not print the
resolved client IP at this level, so the client family is established by the request header, not by the log.

#### Conclusion

The announce from the IPv6 client `2001:db8::1` was counted as `tcp4_announces_handled`, and
`tcp6_announces_handled` stayed at 0. This matches the mismatch the spec describes: the getters
select on the server-binding family (IPv4 here), not on the client family (IPv6). The documentation says the
counters count requests "from IPv4/IPv6 peers", so under that wording the observed result is wrong.

Whether it is a defect or a documentation error depends on the meaning chosen in T1 (maintainer decision). The
reproduction does not depend on that decision: the observed counter value is the same either way.

## Classification

`Reproduced`: the reverse-proxy scenario produces the counter outcome described in the spec.

## Failures and Follow-up

- Failed attempt 1: `[core.net] on_reverse_proxy = true` was rejected at startup with
  `Tracker configuration could not be loaded ... unknown field: found net, expected ... for key "default.core.net"`.
  Remediation: moved the setting under `[http_trackers.network]`. The schema-3 location is
  `packages/configuration/src/v3_0_0/mod.rs` (see the `on_reverse_proxy` examples there). This was a configuration
  error in the run, not a tracker defect.
- No rerun was needed for the reproduction.
