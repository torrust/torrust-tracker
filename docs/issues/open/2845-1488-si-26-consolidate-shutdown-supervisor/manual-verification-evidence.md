---
schema-version: 1
doc-type: evidence
status: partial
last-updated-utc: "2026-11-15 20:30"
---

# Manual Verification Evidence - #2845 (SI-26)

Run on 2026-11-15 against the branch `2845-consolidate-shutdown-supervisor`.

## M1 - Legacy path gone: DONE

`git grep -n "register_legacy\|legacy_cancellation_token\|LegacyJob" -- . ':!docs/issues'` matches only two
historical notes in the event-flow ADR that say the symbols were removed.

## M2 - Orderly shutdown, all services: DONE

Binary `target/debug/torrust-tracker` with `share/default/config/tracker.development.sqlite3.toml` (HTTP, REST,
UDP, health check, ban reset interval 86400 s), sqlite in a temp directory. Signalled the PID after 4 s.

| Signal | Exit code | Log (from the run) |
| ------ | --------- | ------------------ |
| SIGTERM | 0 | `udp_ban_cleanup` requested to stop (waits for no other job); `health_check_api` "will be asked to stop after the jobs it waits for"; `udp_ban_cleanup` completed; then `health_check_api` requested to stop ("every job it waited for has completed") and completed. |
| SIGINT | 0 | Same sequence. |

The UDP port 6969 was released after both runs.

## M3 - Failure path: NOT RUN

Covered by unit tests: the table test of `token_aware_server` (server-join and runtime errors, drain timeout,
controller always joined) and the unchanged component tests. The real-binary deadline-abort evidence is SI-28.

## M4 - Ban cleanup still resets bans: covered by unit test

`it_should_publish_the_new_banned_ips_total_when_it_resets_the_bans` (unchanged loop, now through
`ban_cleanup_loop`) passes.
