---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2845-1488-si-26-consolidate-shutdown-supervisor/ISSUE.md
last-updated-utc: "2026-11-16 07:39"
---

<!-- cspell:ignore Hltun instan instanc -->

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

Run on 2026-11-16 at 07:39 UTC from the repository root with the script below, on a binary built from this
branch. It moves every binding to ports that no other local session uses (another session was running a
tracker on the default ports), starts the binary, waits for `/health_check`, signals the binary PID, waits up
to 30 s, and lists the bindings before start, after start and after exit with `ss -Hltun`. It keeps every
job-manager stop line, for all components, and the IP-ban cleanup lines. This run replaces the one of
2026-11-15 21:13 UTC, which kept only the `udp_ban_cleanup` and `health_check_api` lines.

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
echo "## config overrides (diff against share/default/config/tracker.development.sqlite3.toml, trailing spaces trimmed)"
diff share/default/config/tracker.development.sqlite3.toml "$W/tracker.toml" | sed -E 's/[[:space:]]+$//'
PORTS=':(26868|26969|27070|27171|21212|21313) '
listening() {
  local bound
  bound=$(ss -Hltun | grep -E "$PORTS" | awk '{print $1, $5}' | sort | paste -sd' ' -)
  echo "${bound:-none}"
}

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
  echo "### job manager stop lines for every component, and the IP-ban cleanup lines, in log order"
  sed -E 's/\x1b\[[0-9;]*m//g' "$W/$SIG.log" \
    | grep -E 'bootstrap::jobs::manager: (Requested the job to stop|Job will be asked to stop|Job completed|Job failed|Job aborted)|IP-ban cleanup job' \
    | sed -E 's/^([0-9T:.-]+Z) +INFO +([^:]+(::[^:]+)*): /\1 \2: /; s/torrust_tracker_lib::bootstrap::jobs::manager: /manager: /' \
    | cut -c1-170 | sed -E 's/[[:space:]]+$//'
done
rm -rf "$W"
```

Output (unchanged except for the temporary directory name; the script itself trims trailing spaces and cuts
each log line to 170 characters):

```text
## config overrides (diff against share/default/config/tracker.development.sqlite3.toml, trailing spaces trimmed)
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
bindings before start: none
tracker PID 124997; bindings after start: tcp 127.0.0.1:21212 tcp 127.0.0.1:21313 tcp 127.0.0.1:27070 tcp 127.0.0.1:27171 udp 127.0.0.1:26868 udp 127.0.0.1:26969
exit status 0, 0.11 s after the signal
bindings after exit: none
### job manager stop lines for every component, and the IP-ban cleanup lines, in log order
2026-11-16T07:39:25.578830Z UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=86400
2026-11-16T07:39:25.789674Z manager: Requested the job to stop: it waits for no other job job=udp_ban_cleanup
2026-11-16T07:39:25.789678Z manager: Requested the job to stop: it waits for no other job job=udp_instance_0_127.0.0.1:26868
2026-11-16T07:39:25.789680Z manager: Requested the job to stop: it waits for no other job job=udp_instance_1_127.0.0.1:26969
2026-11-16T07:39:25.789679Z UDP TRACKER: Stopping UDP IP-ban cleanup job ...
2026-11-16T07:39:25.789682Z manager: Requested the job to stop: it waits for no other job job=http_instance_0_127.0.0.1:27070
2026-11-16T07:39:25.789685Z manager: Requested the job to stop: it waits for no other job job=http_instance_1_127.0.0.1:27171
2026-11-16T07:39:25.789688Z manager: Requested the job to stop: it waits for no other job job=torrent_cleanup
2026-11-16T07:39:25.789690Z manager: Requested the job to stop: it waits for no other job job=peers_inactivity_update
2026-11-16T07:39:25.789692Z manager: Requested the job to stop: it waits for no other job job=http_api
2026-11-16T07:39:25.789695Z manager: Job will be asked to stop after the jobs it waits for have completed job=health_check_api waiting_for=swarm_coordination_registry_eve
2026-11-16T07:39:25.789712Z manager: Job will be asked to stop after the jobs it waits for have completed job=swarm_coordination_registry_event_listener waiting_for=udp_i
2026-11-16T07:39:25.789714Z manager: Job will be asked to stop after the jobs it waits for have completed job=tracker_core_in_memory_event_listener waiting_for=udp_instan
2026-11-16T07:39:25.789748Z manager: Job will be asked to stop after the jobs it waits for have completed job=tracker_core_persistent_completed_statistics_event_listener
2026-11-16T07:39:25.789753Z manager: Job will be asked to stop after the jobs it waits for have completed job=http_core_event_listener waiting_for=http_instance_0_127.0.0
2026-11-16T07:39:25.789756Z manager: Job will be asked to stop after the jobs it waits for have completed job=udp_core_event_listener waiting_for=udp_instance_0_127.0.0.1
2026-11-16T07:39:25.789759Z manager: Job will be asked to stop after the jobs it waits for have completed job=udp_server_stats_event_listener waiting_for=udp_instance_0_1
2026-11-16T07:39:25.789761Z manager: Job will be asked to stop after the jobs it waits for have completed job=udp_server_banning_event_listener waiting_for=udp_instance_0
2026-11-16T07:39:25.789773Z manager: Job completed after cooperative cancellation job=udp_ban_cleanup
2026-11-16T07:39:25.789779Z manager: Job completed after cooperative cancellation job=peers_inactivity_update
2026-11-16T07:39:25.789782Z manager: Job completed after cooperative cancellation job=udp_instance_1_127.0.0.1:26969
2026-11-16T07:39:25.789785Z manager: Job completed after cooperative cancellation job=udp_instance_0_127.0.0.1:26868
2026-11-16T07:39:25.789794Z manager: Requested the job to stop: every job it waited for has completed job=udp_core_event_listener last_completed_job=udp_instance_0_127.0.
2026-11-16T07:39:25.789796Z manager: Requested the job to stop: every job it waited for has completed job=udp_server_stats_event_listener last_completed_job=udp_instance_
2026-11-16T07:39:25.789799Z manager: Requested the job to stop: every job it waited for has completed job=udp_server_banning_event_listener last_completed_job=udp_instanc
2026-11-16T07:39:25.789802Z manager: Job completed after cooperative cancellation job=torrent_cleanup
2026-11-16T07:39:25.789806Z manager: Job completed after cooperative cancellation job=http_instance_0_127.0.0.1:27070
2026-11-16T07:39:25.789809Z manager: Job completed after cooperative cancellation job=http_instance_1_127.0.0.1:27171
2026-11-16T07:39:25.789813Z manager: Requested the job to stop: every job it waited for has completed job=swarm_coordination_registry_event_listener last_completed_job=ht
2026-11-16T07:39:25.789816Z manager: Requested the job to stop: every job it waited for has completed job=tracker_core_in_memory_event_listener last_completed_job=http_in
2026-11-16T07:39:25.789819Z manager: Requested the job to stop: every job it waited for has completed job=tracker_core_persistent_completed_statistics_event_listener last
2026-11-16T07:39:25.789823Z manager: Requested the job to stop: every job it waited for has completed job=http_core_event_listener last_completed_job=http_instance_1_127.
2026-11-16T07:39:25.789826Z manager: Job completed after cooperative cancellation job=http_api
2026-11-16T07:39:25.789840Z manager: Job completed after cooperative cancellation job=udp_core_event_listener
2026-11-16T07:39:25.789844Z manager: Job completed after cooperative cancellation job=udp_server_stats_event_listener
2026-11-16T07:39:25.789846Z manager: Job completed after cooperative cancellation job=udp_server_banning_event_listener
2026-11-16T07:39:25.789849Z manager: Job completed after cooperative cancellation job=tracker_core_in_memory_event_listener
2026-11-16T07:39:25.789852Z manager: Job completed after cooperative cancellation job=tracker_core_persistent_completed_statistics_event_listener
2026-11-16T07:39:25.789855Z manager: Job completed after cooperative cancellation job=swarm_coordination_registry_event_listener
2026-11-16T07:39:25.789858Z manager: Job completed after cooperative cancellation job=http_core_event_listener
2026-11-16T07:39:25.789863Z manager: Requested the job to stop: every job it waited for has completed job=health_check_api last_completed_job=http_core_event_listener
2026-11-16T07:39:25.789924Z manager: Job completed after cooperative cancellation job=health_check_api

## run with SIGINT
bindings before start: none
tracker PID 125182; bindings after start: tcp 127.0.0.1:21212 tcp 127.0.0.1:21313 tcp 127.0.0.1:27070 tcp 127.0.0.1:27171 udp 127.0.0.1:26868 udp 127.0.0.1:26969
exit status 0, 0.11 s after the signal
bindings after exit: none
### job manager stop lines for every component, and the IP-ban cleanup lines, in log order
2026-11-16T07:39:25.962070Z UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=86400
2026-11-16T07:39:26.174397Z manager: Requested the job to stop: it waits for no other job job=udp_ban_cleanup
2026-11-16T07:39:26.174400Z manager: Requested the job to stop: it waits for no other job job=udp_instance_0_127.0.0.1:26868
2026-11-16T07:39:26.174402Z manager: Requested the job to stop: it waits for no other job job=udp_instance_1_127.0.0.1:26969
2026-11-16T07:39:26.174403Z manager: Requested the job to stop: it waits for no other job job=http_instance_0_127.0.0.1:27070
2026-11-16T07:39:26.174405Z manager: Requested the job to stop: it waits for no other job job=http_instance_1_127.0.0.1:27171
2026-11-16T07:39:26.174403Z UDP TRACKER: Stopping UDP IP-ban cleanup job ...
2026-11-16T07:39:26.174407Z manager: Requested the job to stop: it waits for no other job job=torrent_cleanup
2026-11-16T07:39:26.174410Z manager: Requested the job to stop: it waits for no other job job=peers_inactivity_update
2026-11-16T07:39:26.174411Z manager: Requested the job to stop: it waits for no other job job=http_api
2026-11-16T07:39:26.174414Z manager: Job will be asked to stop after the jobs it waits for have completed job=health_check_api waiting_for=swarm_coordination_registry_eve
2026-11-16T07:39:26.174420Z manager: Job will be asked to stop after the jobs it waits for have completed job=swarm_coordination_registry_event_listener waiting_for=udp_i
2026-11-16T07:39:26.174432Z manager: Job will be asked to stop after the jobs it waits for have completed job=tracker_core_in_memory_event_listener waiting_for=udp_instan
2026-11-16T07:39:26.174434Z manager: Job will be asked to stop after the jobs it waits for have completed job=tracker_core_persistent_completed_statistics_event_listener
2026-11-16T07:39:26.174436Z manager: Job will be asked to stop after the jobs it waits for have completed job=http_core_event_listener waiting_for=http_instance_0_127.0.0
2026-11-16T07:39:26.174440Z manager: Job will be asked to stop after the jobs it waits for have completed job=udp_core_event_listener waiting_for=udp_instance_0_127.0.0.1
2026-11-16T07:39:26.174454Z manager: Job will be asked to stop after the jobs it waits for have completed job=udp_server_stats_event_listener waiting_for=udp_instance_0_1
2026-11-16T07:39:26.174457Z manager: Job will be asked to stop after the jobs it waits for have completed job=udp_server_banning_event_listener waiting_for=udp_instance_0
2026-11-16T07:39:26.174469Z manager: Job completed after cooperative cancellation job=udp_ban_cleanup
2026-11-16T07:39:26.174475Z manager: Job completed after cooperative cancellation job=udp_instance_1_127.0.0.1:26969
2026-11-16T07:39:26.174478Z manager: Job completed after cooperative cancellation job=torrent_cleanup
2026-11-16T07:39:26.174481Z manager: Job completed after cooperative cancellation job=udp_instance_0_127.0.0.1:26868
2026-11-16T07:39:26.174490Z manager: Requested the job to stop: every job it waited for has completed job=udp_core_event_listener last_completed_job=udp_instance_0_127.0.
2026-11-16T07:39:26.174492Z manager: Requested the job to stop: every job it waited for has completed job=udp_server_stats_event_listener last_completed_job=udp_instance_
2026-11-16T07:39:26.174494Z manager: Requested the job to stop: every job it waited for has completed job=udp_server_banning_event_listener last_completed_job=udp_instanc
2026-11-16T07:39:26.174497Z manager: Job completed after cooperative cancellation job=peers_inactivity_update
2026-11-16T07:39:26.174514Z manager: Job completed after cooperative cancellation job=udp_core_event_listener
2026-11-16T07:39:26.174526Z manager: Job completed after cooperative cancellation job=udp_server_stats_event_listener
2026-11-16T07:39:26.174536Z manager: Job completed after cooperative cancellation job=udp_server_banning_event_listener
2026-11-16T07:39:26.174566Z manager: Job completed after cooperative cancellation job=http_instance_1_127.0.0.1:27171
2026-11-16T07:39:26.174570Z manager: Job completed after cooperative cancellation job=http_instance_0_127.0.0.1:27070
2026-11-16T07:39:26.174574Z manager: Requested the job to stop: every job it waited for has completed job=swarm_coordination_registry_event_listener last_completed_job=ht
2026-11-16T07:39:26.174576Z manager: Requested the job to stop: every job it waited for has completed job=tracker_core_in_memory_event_listener last_completed_job=http_in
2026-11-16T07:39:26.174581Z manager: Requested the job to stop: every job it waited for has completed job=tracker_core_persistent_completed_statistics_event_listener last
2026-11-16T07:39:26.174583Z manager: Requested the job to stop: every job it waited for has completed job=http_core_event_listener last_completed_job=http_instance_0_127.
2026-11-16T07:39:26.174613Z manager: Job completed after cooperative cancellation job=http_api
2026-11-16T07:39:26.174616Z manager: Job completed after cooperative cancellation job=swarm_coordination_registry_event_listener
2026-11-16T07:39:26.174618Z manager: Job completed after cooperative cancellation job=http_core_event_listener
2026-11-16T07:39:26.174621Z manager: Job completed after cooperative cancellation job=tracker_core_persistent_completed_statistics_event_listener
2026-11-16T07:39:26.174623Z manager: Job completed after cooperative cancellation job=tracker_core_in_memory_event_listener
2026-11-16T07:39:26.174627Z manager: Requested the job to stop: every job it waited for has completed job=health_check_api last_completed_job=tracker_core_in_memory_event
2026-11-16T07:39:26.174667Z manager: Job completed after cooperative cancellation job=health_check_api
```

Result:

- Both signals exit with status 0 about 0.11 s after the signal, and every binding is released.
- `udp_ban_cleanup` is asked to stop at once, with the servers, torrent cleanup, the inactivity update and the
  REST API, and completes before `health_check_api` is asked to stop.
- The other components stop in the order their `StopBefore` edges declare (shutdown-order ADR,
  `docs/adrs/20261021184946_order_shutdown_by_event_flow_and_drain_event_listeners.md`): the UDP-core and
  UDP-server listeners are asked to stop only after both UDP instances completed; the HTTP-core, swarm-registry
  and tracker-core listeners only after both HTTP instances (and torrent cleanup) completed; `health_check_api`
  last, after every other component completed.
- "Unchanged" here means the order matches the edges this PR did not touch; this run does not compare against
  a binary built from `develop`. The only new edge is `udp_ban_cleanup` before `health_check_api`.

## M3 - Failure path: TODO

Not run against the binary. The unit tests of `src/bootstrap/jobs/token_aware_server.rs` cover the join and
error mapping, and the real-binary deadline-abort evidence belongs to SI-28 (#2847).

## M4 - Ban cleanup still resets bans: TODO

Not run against the binary. The configuration rejects `ip_bans_reset_interval_in_secs` below 3600
("The IP bans reset interval must be at least 3600 seconds"), so one reset interval takes an hour.
The unit test `it_should_publish_the_new_banned_ips_total_when_it_resets_the_bans` in
`src/bootstrap/jobs/udp_tracker_server.rs` exercises the reset loop on a paused clock and passes; it is
not a substitute for this scenario.
