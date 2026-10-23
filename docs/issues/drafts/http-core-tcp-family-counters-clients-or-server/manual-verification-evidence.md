---
doc-type: manual-verification-evidence
issue-spec: docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/ISSUE.md
last-updated-utc: "2026-10-23 08:23"
---

# Manual Verification Evidence

## Purpose

Record the reproduction of the `tcp4`/`tcp6` counter mismatch for an IPv6 client behind a reverse proxy,
before the fix is decided. This is evidence from commands actually run against the tracker.

## Preserving Verification Artifacts

The tracker configuration, the raw statistics responses before and after the request
([evidence/before.json](evidence/before.json), [evidence/after.json](evidence/after.json)), and the full tracker log
are stored in [evidence/](evidence/). The configuration is also embedded below.

The first run (2026-10-21 18:49 to 18:50 UTC) kept only a filtered copy of the statistics. The run was repeated on
2026-10-22 to store the raw responses (review finding F6 of PR #2621); the counters were identical. Everything below
describes the repeat, and the stored log is the repeat's log.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-22 18:09:38 (tracker start) to 18:10:01 (shutdown, see `evidence/tracker.log`)
- Artifact under test: the draft branch `docs/draft-http-core-tcp-family-counters`, whose product code is that of
  `torrust/develop` at the merge base `80c45b606` ("Merge torrust/torrust-tracker#2616"). No product code changed
  on the branch, which adds only documentation.
- Operating system / environment: Linux, local development build (`cargo build --bin torrust-tracker`,
  `optimized + debuginfo` profile). All commands run from the repository root, and `E` below is the folder of the
  committed artifacts.
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
  ([evidence/before.json](evidence/before.json)). `tcp4_announces_handled` and `tcp6_announces_handled` were both 0.
- Status: `DONE`

#### Steps Performed

1. In terminal A, start the tracker in the foreground from the repository root:

   ```sh
   mkdir -p .tmp/tcp-family-repro
   ./target/debug/torrust-tracker --config-toml-path "$PWD/docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/evidence/tracker.toml" \
     > .tmp/tcp-family-repro/run.log 2>&1
   ```

   The log goes to `.tmp/tcp-family-repro/run.log`; the stored [evidence/tracker.log](evidence/tracker.log) is that file after the run.
   The `mkdir -p` is needed because the SQLite driver opens the configured path `./.tmp/tcp-family-repro/sqlite3.db`
   as given, so it resolves against the working directory, and it does not create the folder. The stored log comes from
   the 2026-10-22 18:09 UTC run, which read a working copy of `evidence/tracker.toml` from `.tmp/`; the configuration printed at the
   top of the log is the one in effect.

   Steps 2 to 4 run in terminal B, from the same repository root. Each block sets `E` itself, because a new shell does
   not inherit it.

2. Check health, read the baseline statistics and store the raw response:

   ```sh
   E=docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/evidence
   curl -s -o /dev/null -w "api:%{http_code}\n" "http://127.0.0.1:1212/api/health_check"
   curl -s "http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken" > "$E/before.json"
   ```

   Result: `api:200`.

3. Send one announce with an IPv6 client address in the forwarded header:

   ```sh
   IH="%01%02%03%04%05%06%07%08%09%0a%0b%0c%0d%0e%0f%10%11%12%13%14"
   PID="-qB0000-123456789012"
   curl -s -o .tmp/tcp-family-repro/announce.bin -w "announce:%{http_code}\n" \
     -H "X-Forwarded-For: 2001:db8::1" \
     "http://127.0.0.1:7070/announce?info_hash=$IH&peer_id=$PID&port=6881&uploaded=0&downloaded=0&left=0&compact=1&event=started"
   ```

   Result: `announce:200`. The response body is a bencoded announce response (`d8:completei1e10:incompletei0e8:intervali120e...`).

4. Read the statistics again, after the request, and store the raw response:

   ```sh
   E=docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/evidence
   curl -s "http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken" > "$E/after.json"
   ```

5. Stop the tracker with Ctrl-C in terminal A. The recorded run instead ended when its shell session closed and sent the
   tracker `SIGTERM`; the log ends with "Torrust tracker successfully shutdown." and the ports 7070 and 1212 were free
   afterwards.

#### Observed Result

The stored responses are the raw `curl` output. The excerpts below come from this filter, which only selects fields:

```sh
E=docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/evidence
jq -c 'with_entries(select(.key|test("^(tcp|udp4_announces|udp6_announces)")))' "$E/before.json" "$E/after.json"
```

Baseline (step 2):

```json
{"tcp4_connections_handled":0,"tcp4_announces_handled":0,"tcp4_scrapes_handled":0,"tcp6_connections_handled":0,"tcp6_announces_handled":0,"tcp6_scrapes_handled":0,"udp4_announces_handled":0,"udp6_announces_handled":0}
```

After the IPv6 forwarded announce (step 4):

```json
{"tcp4_connections_handled":1,"tcp4_announces_handled":1,"tcp4_scrapes_handled":0,"tcp6_connections_handled":0,"tcp6_announces_handled":0,"tcp6_scrapes_handled":0,"udp4_announces_handled":0,"udp6_announces_handled":0}
```

The UDP counters are unchanged at 0 (`udp4_*`, `udp6_*`, see [evidence/after.json](evidence/after.json)).

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
- Rerun (2026-10-22 18:09 UTC): the first run kept only a filtered copy of the statistics. The repeat stored the raw
  responses and the complete log; the counters matched the first run (`tcp4_announces_handled` 0 to 1, `tcp6` 0).
