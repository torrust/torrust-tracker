---
schema-version: 1
doc-type: evidence
status: partial
last-updated-utc: "2026-11-15 21:13"
---

<!-- cspell:ignore Hltun waiti -->

# Manual Verification Evidence - #2845 (SI-26)

Branch `2845-consolidate-shutdown-supervisor`, binary `target/debug/torrust-tracker` built from it.

## M1 - Legacy path gone: DONE

```sh
git grep -n "register_legacy\|legacy_cancellation_token\|LegacyJob" -- . ':!docs/issues'
```

Output: two lines, both in
`docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md` (lines 159 and 367),
notes saying the symbols were removed by SI-26.

## M2 - Orderly shutdown, all services: DONE

Run on 2026-11-15 at 21:13 UTC from the repository root with the script below. It moves every binding to
ports that no other local session uses (another session was running a tracker on the default ports), starts
the binary, waits for `/health_check`, signals the binary PID, waits up to 30 s, and lists the bindings
before start, after start and after exit with `ss -Hltun`.

```bash
#!/usr/bin/env bash
# M2: SIGTERM and SIGINT runs of the real binary on ports no other session uses.
set -u
BIN=target/debug/torrust-tracker
W=$(mktemp -d)
sed -e "s#./storage/tracker/lib/database/sqlite3.db#$W/sqlite3.db#" \
    -e 's#0.0.0.0:6868#127.0.0.1:26868#; s#0.0.0.0:6969#127.0.0.1:26969#' \
    -e 's#0.0.0.0:7070#127.0.0.1:27070#; s#0.0.0.0:7171#127.0.0.1:27171#' \
    -e 's#0.0.0.0:1212#127.0.0.1:21212#' \
    share/default/config/tracker.development.sqlite3.toml > "$W/tracker.toml"
printf '\n[health_check_api]\nbind_address = "127.0.0.1:21313"\n' >> "$W/tracker.toml"
echo "## config overrides (diff against share/default/config/tracker.development.sqlite3.toml)"
diff share/default/config/tracker.development.sqlite3.toml "$W/tracker.toml"
PORTS=':(26868|26969|27070|27171|21212|21313) '
listening() { ss -Hltun | grep -E "$PORTS" | awk '{print $1, $5}' | sort | tr '\n' ' '; echo; }

for SIG in TERM INT; do
  echo; echo "## run with SIG$SIG"
  echo "bindings before start: $(listening)"
  TORRUST_TRACKER_CONFIG_TOML_PATH="$W/tracker.toml" "$BIN" > "$W/$SIG.log" 2>&1 &
  PID=$!
  for _ in $(seq 1 50); do curl -s -o /dev/null http://127.0.0.1:21313/health_check && break; sleep 0.2; done
  echo "tracker PID $PID; bindings after start: $(listening)"
  START=$(date +%s.%N)
  kill -s "$SIG" "$PID"
  for _ in $(seq 1 300); do kill -0 "$PID" 2>/dev/null || break; sleep 0.1; done
  wait "$PID"; STATUS=$?
  echo "exit status $STATUS, $(python3 -I -c "print(round($(date +%s.%N) - $START, 2))") s after the signal"
  echo "bindings after exit: $(listening)"
  echo "### log lines for udp_ban_cleanup and health_check_api, in log order"
  sed -E 's/\x1b\[[0-9;]*m//g' "$W/$SIG.log" | grep -E 'job=(udp_ban_cleanup|health_check_api)|IP-ban cleanup job' | sed -E 's/^([0-9T:.-]+Z) +INFO +([^:]+(::[^:]+)*): /\1 \2: /' | cut -c1-170
done
rm -rf "$W"
```

Output (unchanged except for the temporary directory name):

```text
## config overrides (diff against share/default/config/tracker.development.sqlite3.toml)
19c19
< path = "./storage/tracker/lib/database/sqlite3.db"
---
> path = "<tmp>/sqlite3.db"
41c41
< bind_address = "0.0.0.0:6868"
---
> bind_address = "127.0.0.1:26868"
45c45
< bind_address = "0.0.0.0:6969"
---
> bind_address = "127.0.0.1:26969"
49c49
< bind_address = "0.0.0.0:7070"
---
> bind_address = "127.0.0.1:27070"
53c53
< bind_address = "0.0.0.0:7171"
---
> bind_address = "127.0.0.1:27171"
57c57
< bind_address = "0.0.0.0:1212"
---
> bind_address = "127.0.0.1:21212"
60a61,63
> 
> [health_check_api]
> bind_address = "127.0.0.1:21313"

## run with SIGTERM
bindings before start: 
tracker PID 2146446; bindings after start: tcp 127.0.0.1:21212 tcp 127.0.0.1:21313 tcp 127.0.0.1:27070 tcp 127.0.0.1:27171 udp 127.0.0.1:26868 udp 127.0.0.1:26969 
exit status 0, 0.11 s after the signal
bindings after exit: 
### log lines for udp_ban_cleanup and health_check_api, in log order
2026-11-15T21:13:06.852974Z UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=86400
2026-11-15T21:13:07.065742Z torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=udp_ban_cleanup
2026-11-15T21:13:07.065748Z UDP TRACKER: Stopping UDP IP-ban cleanup job ...
2026-11-15T21:13:07.065779Z torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=health_check_api waiti
2026-11-15T21:13:07.065850Z torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_ban_cleanup
2026-11-15T21:13:07.066049Z torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=health_check_api last_comp
2026-11-15T21:13:07.066091Z torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=health_check_api

## run with SIGINT
bindings before start: 
tracker PID 2146627; bindings after start: tcp 127.0.0.1:21212 tcp 127.0.0.1:21313 tcp 127.0.0.1:27070 tcp 127.0.0.1:27171 udp 127.0.0.1:26868 udp 127.0.0.1:26969 
exit status 0, 0.11 s after the signal
bindings after exit: 
### log lines for udp_ban_cleanup and health_check_api, in log order
2026-11-15T21:13:07.243431Z UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=86400
2026-11-15T21:13:07.455804Z torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: it waits for no other job job=udp_ban_cleanup
2026-11-15T21:13:07.455812Z UDP TRACKER: Stopping UDP IP-ban cleanup job ...
2026-11-15T21:13:07.455823Z torrust_tracker_lib::bootstrap::jobs::manager: Job will be asked to stop after the jobs it waits for have completed job=health_check_api waiti
2026-11-15T21:13:07.455913Z torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=udp_ban_cleanup
2026-11-15T21:13:07.456015Z torrust_tracker_lib::bootstrap::jobs::manager: Requested the job to stop: every job it waited for has completed job=health_check_api last_comp
2026-11-15T21:13:07.456080Z torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=health_check_api
```

Result: both signals exit with status 0 about 0.11 s after the signal. Every binding is released.
`udp_ban_cleanup` is asked to stop at once and completes before `health_check_api` is asked to stop,
which logs "every job it waited for has completed".

## M3 - Failure path: TODO

Not run against the binary. The unit tests of `src/bootstrap/jobs/token_aware_server.rs` cover the join and
error mapping, and the real-binary deadline-abort evidence belongs to SI-28 (#2847).

## M4 - Ban cleanup still resets bans: TODO

Not run against the binary. The configuration rejects `ip_bans_reset_interval_in_secs` below 3600
("The IP bans reset interval must be at least 3600 seconds"), so one reset interval takes an hour.
The unit test `it_should_publish_the_new_banned_ips_total_when_it_resets_the_bans` in
`src/bootstrap/jobs/udp_tracker_server.rs` exercises the reset loop on a paused clock and passes; it is
not a substitute for this scenario.
