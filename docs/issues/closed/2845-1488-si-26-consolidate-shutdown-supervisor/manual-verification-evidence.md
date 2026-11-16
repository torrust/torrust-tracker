---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2845-1488-si-26-consolidate-shutdown-supervisor/ISSUE.md
last-updated-utc: "2026-11-16 09:50"
---

<!-- cspell:ignore Hltun instan instanc sendall addre remainin urandom QQQIIIiH QQQII recvfrom rtx -->

# Manual Verification Evidence - #2845 (SI-26)

M1 and M2: branch `2845-consolidate-shutdown-supervisor`, binary `target/debug/torrust-tracker` built from it.
M3 and M4: binary built from `develop` at `a25a27e23`, which includes the merge of PR #2870 (`1c3daab4b`).

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

## M3 - Failure path: DONE

Run on 2026-11-16 at 08:45 UTC from the repository root, on a binary built from `develop` (see above), with the SI-21 V6 recipe: the shortened shutdown budgets of V6 and a partial REST request (a
header block that never ends) held open for 10 s, so the REST drain cannot finish. The script signals the
binary PID with `SIGTERM` and waits up to 15 s.

```bash
#!/usr/bin/env bash
# M3: failure path on the real binary (the SI-21 V6 recipe). A partial REST request outlives the
# REST drain, so the REST API component fails; expect exit 1 within the shutdown deadline P.
set -u
BIN=target/debug/torrust-tracker
W=$(mktemp -d)
sed -e "s#./storage/tracker/lib/database/sqlite3.db#$W/sqlite3.db#" \
    -e 's#0.0.0.0:6868#127.0.0.1:26868#; s#0.0.0.0:6969#127.0.0.1:26969#' \
    -e 's#0.0.0.0:7070#127.0.0.1:27070#; s#0.0.0.0:7171#127.0.0.1:27171#' \
    -e 's#0.0.0.0:1212#127.0.0.1:21212#' \
    -e 's#^deadline_in_secs = 25#deadline_in_secs = 4#; s#^http_drain_in_secs = 15#http_drain_in_secs = 2#' \
    -e 's#^udp_request_drain_in_secs = 5#udp_request_drain_in_secs = 1#; s#^event_listener_drain_in_secs = 5#event_listener_drain_in_secs = 1#' \
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

echo; echo "## run with SIGTERM and a partial REST request held open"
echo "bindings before start: $(listening)"
TORRUST_TRACKER_CONFIG_TOML_PATH="$W/tracker.toml" "$BIN" > "$W/tracker.log" 2>&1 &
PID=$!
for _ in $(seq 1 50); do curl -s -o /dev/null http://127.0.0.1:21313/health_check && break; sleep 0.2; done
echo "tracker PID $PID; bindings after start: $(listening)"
# Partial request: the header block never ends, so the connection stays open for 10 s.
python3 -I -c '
import socket, time
s = socket.create_connection(("127.0.0.1", 21212))
s.sendall(b"GET /api/v1/stats?token=MyAccessToken HTTP/1.1\r\nHost: 127.0.0.1\r\n")
time.sleep(10)
' &
HOLDER=$!
sleep 0.5
START=$(date +%s.%N)
kill -s TERM "$PID"
for _ in $(seq 1 300); do kill -0 "$PID" 2>/dev/null || break; sleep 0.05; done
wait "$PID"; STATUS=$?
echo "exit status $STATUS, $(python3 -I -c "print(round($(date +%s.%N) - $START, 2))") s after the signal (P = 4 s)"
echo "bindings after exit: $(listening)"
wait "$HOLDER" 2>/dev/null
echo "### job manager outcome lines and shutdown summary, in log order"
sed -E 's/\x1b\[[0-9;]*m//g' "$W/tracker.log" \
  | grep -E 'bootstrap::jobs::manager: (Job completed|Job failed|Job aborted)|job=(http_api|health_check_api)|incomplete|timed out|Shutdown|shutdown' \
  | sed -E 's/^([0-9T:.-]+Z) +[A-Z]+ +([^:]+(::[^:]+)*): /\1 \2: /; s/torrust_tracker_lib::bootstrap::jobs::manager: /manager: /' \
  | cut -c1-200 | sed -E 's/[[:space:]]+$//'
rm -rf "$W"
```

Output (unchanged except for the temporary directory name; the filter also keeps one line of the
configuration dump that contains "shutdown"):

```text
## config overrides (diff against share/default/config/tracker.development.sqlite3.toml, trailing spaces trimmed)
19c19
< path = "./storage/tracker/lib/database/sqlite3.db"
---
> path = "<tmp>/sqlite3.db"
30,33c30,33
< deadline_in_secs = 25
< http_drain_in_secs = 15
< udp_request_drain_in_secs = 5
< event_listener_drain_in_secs = 5
---
> deadline_in_secs = 4
> http_drain_in_secs = 2
> udp_request_drain_in_secs = 1
> event_listener_drain_in_secs = 1
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

## run with SIGTERM and a partial REST request held open
bindings before start: none
tracker PID 1662110; bindings after start: tcp 127.0.0.1:21212 tcp 127.0.0.1:21313 tcp 127.0.0.1:27070 tcp 127.0.0.1:27171 udp 127.0.0.1:26868 udp 127.0.0.1:26969
exit status 1, 2.04 s after the signal (P = 4 s)
bindings after exit: none
### job manager outcome lines and shutdown summary, in log order
    "shutdown": {
2026-11-16T08:45:12.034362Z torrust_tracker: Tracker shutdown signal handlers installed.
2026-11-16T08:45:12.745583Z manager: Requested the job to stop: it waits for no other job job=http_api
2026-11-16T08:45:12.745586Z manager: Job will be asked to stop after the jobs it waits for have completed job=health_check_api waiting_for=swarm_coordination_registry_event_listener, tracker_core_in_m
2026-11-16T08:45:12.745616Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:27070 drain_timeout=2s}: torrust_tracker_axum_server::signals: !! Shutting down HTTP server on socket address: 127
2026-11-16T08:45:12.745621Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:27171 drain_timeout=2s}: torrust_tracker_axum_server::signals: !! Shutting down HTTP server on socket address: 127
2026-11-16T08:45:12.745628Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:27070 drain_timeout=2s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in add
2026-11-16T08:45:12.745646Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:21212 drain_timeout=2s}: torrust_tracker_axum_server::signals: !! Shutting down tracker API server on socket addre
2026-11-16T08:45:12.745613Z manager: Waiting for job to finish under the shared shutdown deadline timeout_seconds=4
2026-11-16T08:45:12.745638Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:27171 drain_timeout=2s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in add
2026-11-16T08:45:12.745652Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:21212 drain_timeout=2s}: torrust_tracker_axum_server::signals: Remaining alive connections: 1 (1.9999997s remainin
2026-11-16T08:45:12.745658Z manager: Job completed after cooperative cancellation job=udp_ban_cleanup
2026-11-16T08:45:12.745670Z manager: Job completed after cooperative cancellation job=udp_instance_0_127.0.0.1:26868
2026-11-16T08:45:12.745674Z manager: Job completed after cooperative cancellation job=peers_inactivity_update
2026-11-16T08:45:12.745677Z manager: Job completed after cooperative cancellation job=torrent_cleanup
2026-11-16T08:45:12.745683Z manager: Job completed after cooperative cancellation job=udp_instance_1_127.0.0.1:26969
2026-11-16T08:45:12.745711Z manager: Job completed after cooperative cancellation job=udp_core_event_listener
2026-11-16T08:45:12.745722Z manager: Job completed after cooperative cancellation job=http_instance_0_127.0.0.1:27070
2026-11-16T08:45:12.745726Z manager: Job completed after cooperative cancellation job=udp_server_stats_event_listener
2026-11-16T08:45:12.745729Z manager: Job completed after cooperative cancellation job=udp_server_banning_event_listener
2026-11-16T08:45:12.745808Z manager: Job completed after cooperative cancellation job=http_instance_1_127.0.0.1:27171
2026-11-16T08:45:12.745839Z manager: Job completed after cooperative cancellation job=swarm_coordination_registry_event_listener
2026-11-16T08:45:12.745854Z manager: Job completed after cooperative cancellation job=tracker_core_in_memory_event_listener
2026-11-16T08:45:12.745858Z manager: Job completed after cooperative cancellation job=tracker_core_persistent_completed_statistics_event_listener
2026-11-16T08:45:12.745861Z manager: Job completed after cooperative cancellation job=http_core_event_listener
2026-11-16T08:45:13.746743Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:21212 drain_timeout=2s}: torrust_tracker_axum_server::signals: Remaining alive connections: 1 (998.914324ms remain
2026-11-16T08:45:14.746807Z  WARN graceful_shutdown_on_cancellation{address=127.0.0.1:21212 drain_timeout=2s}: torrust_tracker_axum_server::signals: Shutdown timeout of 2s reached in address 127.0.0.1
2026-11-16T08:45:14.746988Z manager: Job reported a failure job=http_api error=tracker API graceful drain timed out
2026-11-16T08:45:14.747010Z manager: Requested the job to stop: every job it waited for has completed job=health_check_api last_completed_job=http_api
2026-11-16T08:45:14.747021Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:21313 drain_timeout=2s}: torrust_tracker_axum_server::signals: !! Shutting down health check API server on socket
2026-11-16T08:45:14.747027Z  INFO graceful_shutdown_on_cancellation{address=127.0.0.1:21313 drain_timeout=2s}: torrust_tracker_axum_server::signals: All connections closed, shutting down server in add
2026-11-16T08:45:14.747063Z manager: Job completed after cooperative cancellation job=health_check_api
2026-11-16T08:45:14.747076Z torrust_tracker: Torrust tracker shutdown was incomplete: a component failed, panicked, or was aborted at the shutdown deadline. incomplete_components=http_api
```

Result:

- Exit status 1, 2.04 s after the signal, inside P = 4 s; every binding is released.
- `http_api` is reported as failed with "tracker API graceful drain timed out", the same wording SI-21 V6
  recorded, once its 2 s drain ran out; every other component completed after cooperative cancellation.
- `health_check_api` is asked to stop only after `http_api` completed (`last_completed_job=http_api`) and
  stops at once, so it does not delay the exit past P.
- The final line names `http_api` as the incomplete component, as in SI-21 V6.

## M4 - Ban cleanup still resets bans: DONE

Run on 2026-11-16 from 08:44 to 09:44 UTC, on the same binary, with `ip_bans_reset_interval_in_secs = 3600`
(the smallest value the configuration accepts) and `max_connection_id_errors_per_ip = 10` (the
development default). The script starts the binary on ports no other session uses, probes the UDP tracker
on port 36969 with `udp_probe.py`, sends twelve announces with a made-up connection ID, then polls the
`udp_banned_ips_total` metric of the REST API every 60 s until the reset, and stops the binary with
`SIGTERM`.

`udp_probe.py`:

```python
"""UDP tracker probes for M4: a BEP 15 connect, and announces with a wrong connection ID."""
import os
import socket
import struct
import sys

HOST = "127.0.0.1"


def connect(port, timeout=2.0):
    """Send a connect request; print 'answered' with the action, or 'no answer' on timeout."""
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.settimeout(timeout)
    tx = struct.unpack(">I", os.urandom(4))[0]
    s.sendto(struct.pack(">QII", 0x41727101980, 0, tx), (HOST, port))
    try:
        data, _ = s.recvfrom(2048)
        action, rtx = struct.unpack(">II", data[:8])
        print(f"connect: answered (action={action}, transaction id matches={rtx == tx})")
    except socket.timeout:
        print(f"connect: no answer within {timeout} s")


def bad_announces(port, count):
    """Send announces with a made-up connection ID; each one is a connection ID error."""
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.settimeout(1.0)
    answers = 0
    for _ in range(count):
        tx = struct.unpack(">I", os.urandom(4))[0]
        packet = struct.pack(">QII", 0x0123456789ABCDEF, 1, tx) + b"A" * 20 + b"-qB0000-000000000000" \
            + struct.pack(">QQQIIIiH", 0, 0, 0, 0, 0, 0, -1, 6881)
        s.sendto(packet, (HOST, port))
        try:
            s.recvfrom(2048)
            answers += 1
        except socket.timeout:
            pass
    print(f"bad announces: sent {count}, answered {answers}")


if __name__ == "__main__":
    port = int(sys.argv[2])
    if sys.argv[1] == "connect":
        connect(port)
    else:
        bad_announces(port, int(sys.argv[3]))
```

`m4.sh` (run as `bash m4.sh udp_probe.py`):

```bash
#!/usr/bin/env bash
# M4: the migrated udp_ban_cleanup component still resets the bans, on the real binary.
# The configuration rejects a reset interval below 3600 s, so the run takes just over an hour.
set -u
BIN=target/debug/torrust-tracker
PROBE="$1"   # path to udp_probe.py
W=$(mktemp -d)
sed -e "s#./storage/tracker/lib/database/sqlite3.db#$W/sqlite3.db#" \
    -e 's#^ip_bans_reset_interval_in_secs = 86400#ip_bans_reset_interval_in_secs = 3600#' \
    -e 's#0.0.0.0:6868#127.0.0.1:36868#; s#0.0.0.0:6969#127.0.0.1:36969#' \
    -e 's#0.0.0.0:7070#127.0.0.1:37070#; s#0.0.0.0:7171#127.0.0.1:37171#' \
    -e 's#0.0.0.0:1212#127.0.0.1:31212#' \
    share/default/config/tracker.development.sqlite3.toml > "$W/tracker.toml"
printf '\n[health_check_api]\nbind_address = "127.0.0.1:31313"\n' >> "$W/tracker.toml"
echo "## config overrides (diff against share/default/config/tracker.development.sqlite3.toml, trailing spaces trimmed)"
diff share/default/config/tracker.development.sqlite3.toml "$W/tracker.toml" | sed -E 's/[[:space:]]+$//'

ts() { date -u +%H:%M:%S; }
banned() {
  curl -s "http://127.0.0.1:31212/api/v1/stats?token=MyAccessToken&format=prometheus" \
    | grep -E '^udp_banned_ips_total ' || echo "udp_banned_ips_total <unavailable>"
}
probe() { python3 -I "$PROBE" "$@"; }

echo
TORRUST_TRACKER_CONFIG_TOML_PATH="$W/tracker.toml" "$BIN" > "$W/tracker.log" 2>&1 &
PID=$!
for _ in $(seq 1 50); do curl -s -o /dev/null http://127.0.0.1:31313/health_check && break; sleep 0.2; done
START=$(date +%s)
echo "$(ts) tracker PID $PID started"
echo "$(ts) before: $(banned); $(probe connect 36969)"
echo "$(ts) $(probe bad 36969 12)"
echo "$(ts) after the bad announces: $(banned); $(probe connect 36969)"
echo "$(ts) waiting for the reset (polling every 60 s, at most 3720 s after start)"
while :; do
  sleep 60
  ELAPSED=$(( $(date +%s) - START ))
  STATE=$(banned)
  if [[ "$STATE" == "udp_banned_ips_total 0" ]]; then
    echo "$(ts) reset seen ${ELAPSED} s after start: $STATE; $(probe connect 36969)"
    break
  fi
  if (( ELAPSED > 3720 )); then
    echo "$(ts) no reset ${ELAPSED} s after start: $STATE; $(probe connect 36969)"
    break
  fi
done
kill -s TERM "$PID"
wait "$PID"; echo "$(ts) exit status $? after SIGTERM"
echo "### UDP ban log lines, in log order"
sed -E 's/\x1b\[[0-9;]*m//g' "$W/tracker.log" \
  | grep -iE 'IP-ban|ban' | grep -vE 'Requested the job|Job will be asked|Job completed' \
  | sed -E 's/^([0-9T:.-]+Z) +[A-Z]+ +([^:]+(::[^:]+)*): /\1 \2: /' | cut -c1-170 | sed -E 's/[[:space:]]+$//' | head -40
rm -rf "$W"
```

Output (unchanged except for the temporary directory name):

```text
## config overrides (diff against share/default/config/tracker.development.sqlite3.toml, trailing spaces trimmed)
19c19
< path = "./storage/tracker/lib/database/sqlite3.db"
---
> path = "<tmp>/sqlite3.db"
22c22
< ip_bans_reset_interval_in_secs = 86400
---
> ip_bans_reset_interval_in_secs = 3600
41c41
< bind_address = "0.0.0.0:6868"
---
> bind_address = "127.0.0.1:36868"
45c45
< bind_address = "0.0.0.0:6969"
---
> bind_address = "127.0.0.1:36969"
49c49
< bind_address = "0.0.0.0:7070"
---
> bind_address = "127.0.0.1:37070"
53c53
< bind_address = "0.0.0.0:7171"
---
> bind_address = "127.0.0.1:37171"
57c57
< bind_address = "0.0.0.0:1212"
---
> bind_address = "127.0.0.1:31212"
60a61,63
>
> [health_check_api]
> bind_address = "127.0.0.1:31313"

08:44:50 tracker PID 1660358 started
08:44:50 before: udp_banned_ips_total 0; connect: answered (action=0, transaction id matches=True)
08:44:50 bad announces: sent 12, answered 11
08:44:51 after the bad announces: udp_banned_ips_total 1; connect: no answer within 2.0 s
08:44:53 waiting for the reset (polling every 60 s, at most 3720 s after start)
09:44:53 reset seen 3603 s after start: udp_banned_ips_total 0; connect: answered (action=0, transaction id matches=True)
09:44:53 exit status 0 after SIGTERM
### UDP ban log lines, in log order
    "ip_bans_reset_interval_in_secs": 3600,
2026-11-16T08:44:49.973569Z initialize: METRICS: type="counter" name="udp_tracker_server_requests_banned_total" unit=Some(Count) description=Some(MetricDescription("Total
2026-11-16T08:44:49.973572Z initialize: METRICS: type="gauge" name="udp_tracker_server_ips_banned_total" unit=Some(Count) description=Some(MetricDescription("Number of IP
2026-11-16T08:44:49.973938Z UDP TRACKER: Starting UDP tracker server event listener (banning)
2026-11-16T08:44:49.973938Z UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=3600
2026-11-16T09:44:53.896768Z UDP TRACKER: Stopping UDP IP-ban cleanup job ...
2026-11-16T09:44:53.896973Z UDP TRACKER: Received cancellation request, draining UDP tracker server event listener (banning).
2026-11-16T09:44:53.896997Z UDP TRACKER: Event listener drained its queue after cancellation listener="udp_tracker_server_banning" outcome="emptied" processed=0 lost_to_l
```

Result:

- Before the bad announces the IP is not banned: `udp_banned_ips_total 0`, and a connect is answered.
- After them it is banned: the eleventh connection ID error passes the limit of ten, the twelfth
  announce gets no answer, `udp_banned_ips_total` is 1, and a connect gets no answer.
- 3603 s after start, at the first poll after the 3600 s interval, the ban is reset:
  `udp_banned_ips_total 0`, and a connect is answered again.
- `udp_ban_cleanup` started with `reset_interval_in_secs=3600`, stopped on `SIGTERM` ("Stopping UDP IP-ban
  cleanup job"), and the binary exited with status 0.
- The reset itself writes no log line; the metric and the answered connect are the evidence.
