---
name: manual-torrent-cleanup-e2e
description: Manually verify that the tracker removes inactive peers while retaining a peerless torrent when configured to do so. Use when testing torrent cleanup, inactive peer expiration, or shutdown behavior of the cleanup runner. Triggers on "torrent cleanup test", "inactive peer expiration", or "manual cleanup e2e".
compatibility: Requires cargo, curl, jq, rg, and ss on Linux.
metadata:
  author: torrust
  version: "1.0"
---

# Manual Torrent Cleanup E2E

## Purpose

This skill verifies the observable behavior of the torrent cleanup job:

- an announced peer appears through the REST API;
- the peer is removed after its configured timeout;
- the torrent remains queryable when peerless-torrent removal is disabled; and
- a direct-binary SIGTERM cooperatively cancels cleanup.

This is a manual regression and lifecycle check. It complements focused unit
tests and does not replace the automated test suite.

## Prerequisites

Run commands from the repository root. Use a disposable configuration below to
avoid development ports and persistent state.

```bash
mkdir -p .tmp
cat > .tmp/torrent-cleanup-e2e.toml <<'EOF'
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
inactive_peer_cleanup_interval = 1
listed = false
private = false
tracker_usage_statistics = false

[core.tracker_policy]
max_peer_timeout = 1
persistent_torrent_completed_stat = false
remove_peerless_torrents = false

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "127.0.0.1:16969"

[http_api]
bind_address = "127.0.0.1:11212"

[http_api.access_tokens]
admin = "TorrentCleanupVerificationToken"

[health_check_api]
bind_address = "127.0.0.1:11313"
EOF
```

## 1. Build and start the direct binary

In terminal A:

```bash
cargo build --bin torrust-tracker
TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/torrent-cleanup-e2e.toml" \
  ./target/debug/torrust-tracker 2>&1 | tee .tmp/torrent-cleanup-e2e.log
```

Confirm that the isolated services are listening and identify the direct
executable PID. Do not signal a shell or `cargo run` parent.

```bash
pgrep -af 'target/debug/torrust-tracker'
ss -ulnp | rg ':16969'
ss -tlnp | rg ':(11212|11313)'
```

Expected: one direct tracker executable owns UDP `16969`, REST API `11212`, and
health API `11313`.

## 2. Announce a peer and capture initial state

In terminal B:

```bash
INFO_HASH=2169216921692169216921692169216921692169
API_URL="http://127.0.0.1:11212/api/v1/torrent/$INFO_HASH"
TOKEN='TorrentCleanupVerificationToken'

cargo run -p torrust-tracker-client --bin tracker_client -- \
  udp announce udp://127.0.0.1:16969/announce "$INFO_HASH"

curl --fail --silent --show-error -H "Authorization: Bearer $TOKEN" "$API_URL" \
  | jq '{info_hash, peers}'
```

Expected: the response has the requested `info_hash` and exactly one peer.

## 3. Verify cleanup

Wait at least five seconds without sending another announce, then query the same
torrent again:

```bash
curl --fail --silent --show-error -H "Authorization: Bearer $TOKEN" "$API_URL" \
  | jq '{info_hash, peers}'

rg -n 'Cleaning up torrents|peers=0' .tmp/torrent-cleanup-e2e.log | tail -n 20
```

Expected:

- the endpoint still returns HTTP 200 for the original torrent;
- its `peers` array is empty; and
- the log records the cleanup cadence and a post-cleanup `peers=0` aggregate.

`remove_peerless_torrents = false` is essential: otherwise the REST API cannot
distinguish a removed peer from a removed torrent.

## 4. Verify cooperative SIGTERM cancellation

Use the direct PID identified in step 1:

```bash
kill -TERM <tracker-pid>
```

After the tracker exits, inspect its logs and the isolated ports:

```bash
rg -n -C 3 'Stopping torrent cleanup job|job=torrent_cleanup|deadline expired' \
  .tmp/torrent-cleanup-e2e.log
pgrep -af 'target/debug/torrust-tracker' || true
ss -ulnp | rg ':16969' || true
ss -tlnp | rg ':(11212|11313)' || true
```

Expected: the log contains `Stopping torrent cleanup job ...` and `Job completed
after cooperative cancellation job=torrent_cleanup`, not a deadline-abort
outcome. No isolated listener or direct tracker process remains.

## Troubleshooting

- If a listener is already in use, choose a fresh three-port set consistently in
  the configuration and commands.
- `metadata.purpose` must remain the schema value `configuration`.
- Use a fresh 40-character hexadecimal `INFO_HASH` if a previous run remains in
  the in-memory state.
- If the REST request is rejected, confirm the bearer token matches
  `[http_api.access_tokens]` in the disposable configuration.

## Related

- Cleanup runner: `src/bootstrap/jobs/torrent_cleanup.rs`
- Local runtime workflow: `.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`
- REST API workflow: `.github/skills/usage/use-rest-api/SKILL.md`

## Skill Links

- `src/bootstrap/jobs/torrent_cleanup.rs` — `skill-link: manual-torrent-cleanup-e2e`
  marks the runner whose behavior and logs this procedure verifies.
