# T3 Persistence-Free Runtime Verification

**Date:** 2026-08-28 14:34-14:38 UTC

## Scope

This verification exercised the local v3 tracker after T3 removed the fixed
SQLite compatibility bridge. The supplied configuration omitted
`[core.database]`, enabled public HTTP/UDP tracker instances and tracker usage
statistics, and enabled the management and health APIs.

## Configuration

The local configuration was supplied with `TORRUST_TRACKER_CONFIG_TOML`:

```toml
[core]
listed = false
private = false
tracker_usage_statistics = true

[core.tracker_policy]
persistent_torrent_completed_stat = false

[[udp_trackers]]
bind_address = "127.0.0.1:16969"

[[http_trackers]]
bind_address = "127.0.0.1:17070"

[http_api]
bind_address = "127.0.0.1:11212"

[health_check_api]
bind_address = "127.0.0.1:11313"
```

The resolved configuration logged by the tracker contained `"database": null`.

## Commands And Results

```text
TORRUST_TRACKER_CONFIG_TOML="$(<.tmp/2107-no-persistence-verification.toml)" cargo run --bin torrust-tracker

cargo run -p torrust-tracker-client --bin tracker_client -- udp announce udp://127.0.0.1:16969/announce 9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d
{"AnnounceIpv4":{"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}

cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:17070/announce 9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d
{"complete":2,"incomplete":0,"interval":120,"min interval":120,"peers":[...]}

curl 'http://127.0.0.1:11212/api/v1/torrents?token=T3VerificationToken'
[{"info_hash":"9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d","seeders":2,"completed":0,"leechers":0}]
HTTP 200

curl 'http://127.0.0.1:11212/api/health_check?token=T3VerificationToken'
{"status":"Ok"}
HTTP 200

curl --header 'content-type: application/json' --data '{"key":null,"seconds_valid":60}' 'http://127.0.0.1:11212/api/v1/keys?token=T3VerificationToken'
{"status":"err","reason":"private capability is disabled by configuration"}
HTTP 409

curl --request POST 'http://127.0.0.1:11212/api/v1/whitelist/9e0217d0fa71c87332cd8bf9dbeabcb2c2cf3c4d?token=T3VerificationToken'
{"status":"err","reason":"listed capability is disabled by configuration"}
HTTP 409
```

The tracker logged startup of the tracker-core event listener, HTTP tracker,
UDP tracker, REST API, and health API. It accepted Ctrl-C and reported a
successful graceful shutdown after every managed job completed.

## Persistence Inspection

No database driver, migration, or database setup log was emitted by this run.
The resolved configuration reported `database: null`.

The workspace already contained SQLite files before this test, so their
presence cannot be attributed to this run:

```text
2026-07-16 17:15:08 +0100 storage/tracker/lib/database/sqlite3.db
2026-07-31 13:31:45 +0100 .tmp/issue-2041-manual.sqlite3
2026-08-28 13:58:34 +0100 .tmp/manual-t2-rest-contract.sqlite3.db
```

No new SQLite file was created by the isolated configuration. A clean
workspace/container artifact test remains required for M5 and M6.

## Result

M1 and M4 passed for the local source-tree runtime. This run demonstrates
actual public protocol and API operation with no configured persistence; it
does not replace the pending baseline or supported-container verification.
