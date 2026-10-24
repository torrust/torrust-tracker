#!/usr/bin/env bash
# Disposable SI-22 T0 reproduction (see the spec's "Disposable Verification Scripts").
# Counts `completed` announces that the tracker answered successfully but never
# persisted, when the tracker binary receives SIGTERM during or after a burst.
#
# Usage:
#   reproduce-lost-completions.sh race     # SIGTERM while announces are in flight
#   reproduce-lost-completions.sh control  # SIGTERM after every response arrived
#
# Environment overrides: PEERS, CONCURRENCY, SIGTERM_DELAY, CONTROL_WAIT, HTTP_PORT,
# HEALTH_PORT.
set -euo pipefail

PEERS="${PEERS:-2000}"
CONCURRENCY="${CONCURRENCY:-64}"
SIGTERM_DELAY="${SIGTERM_DELAY:-0.05}"
CONTROL_WAIT="${CONTROL_WAIT:-1}"
export HTTP_PORT="${HTTP_PORT:-47070}"
HEALTH_PORT="${HEALTH_PORT:-47313}"
# 20 ASCII bytes, so the info-hash needs no percent-encoding.
export INFO_HASH="SI22SI22SI22SI22SI22"

announce() {
    local event="$1" peer="$2" peer_id response
    peer_id="$(printf -- '-SI2200-%012d' "$peer")"
    if ! response="$(curl --silent --fail --max-time 10 \
        "http://127.0.0.1:${HTTP_PORT}/announce?info_hash=${INFO_HASH}&peer_id=${peer_id}&port=$((10000 + peer))&uploaded=0&downloaded=0&left=0&event=${event}" \
        | tr -d '\000')"; then
        echo "fail"
        return
    fi
    if [[ "$response" == *"8:interval"* ]]; then
        echo "ok"
    else
        echo "fail"
    fi
}

if [[ "${1:-}" == "announce" ]]; then
    announce "$2" "$3"
    exit 0
fi

mode="${1:?usage: $0 race|control}"
if [[ "$mode" != "race" && "$mode" != "control" ]]; then
    echo "usage: $0 race|control" >&2
    exit 2
fi
if [[ ! "$PEERS" =~ ^[1-9][0-9]*$ || ${#PEERS} -gt 5 || "$PEERS" -gt 55535 \
    || ! "$CONCURRENCY" =~ ^[1-9][0-9]*$ || ${#CONCURRENCY} -gt 5 ]]; then
    echo "PEERS must be 1..55535 and CONCURRENCY must be 1..99999" >&2
    exit 2
fi
root="$(cd "$(dirname "$0")/../../../.." && pwd)"
mkdir -p "$root/.tmp/si22-t0"
work="$(mktemp -d "$root/.tmp/si22-t0/$(date -u +%Y%m%dT%H%M%SZ)-$mode-XXXXXX")"
db="$work/tracker.sqlite3"
log="$work/tracker.log"
tracker_pid=""
burst_pid=""

cleanup() {
    if [[ -n "$burst_pid" ]]; then
        kill -KILL -- "-$burst_pid" 2>/dev/null || true
        wait "$burst_pid" 2>/dev/null || true
    fi
    if [[ -n "$tracker_pid" ]]; then
        kill -KILL "$tracker_pid" 2>/dev/null || true
        wait "$tracker_pid" 2>/dev/null || true
    fi
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

cat > "$work/tracker.toml" <<EOF
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
inactive_peer_cleanup_interval = 3600
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "$db"

[core.tracker_policy]
max_peer_timeout = 3600
persistent_torrent_completed_stat = true
remove_peerless_torrents = false

[[http_trackers]]
bind_address = "127.0.0.1:${HTTP_PORT}"
tracker_usage_statistics = true

[health_check_api]
bind_address = "127.0.0.1:${HEALTH_PORT}"
EOF

env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH \
    "$root/target/debug/torrust-tracker" -c "$work/tracker.toml" > "$log" 2>&1 &
tracker_pid=$!

ready=false
readiness_deadline=$((SECONDS + 30))
while (( SECONDS < readiness_deadline )); do
    kill -0 "$tracker_pid" 2>/dev/null || break
    if curl --silent --fail --max-time 1 "http://127.0.0.1:${HEALTH_PORT}/health_check" > /dev/null \
        && grep -q "Tracker shutdown signal handlers installed." "$log"; then
        ready=true
        break
    fi
    sleep 0.1
done
if [[ "$ready" != true ]]; then
    echo "Tracker startup failed or timed out; see $log" >&2
    exit 1
fi

seq 1 "$PEERS" | setsid xargs -P "$CONCURRENCY" -n 1 "$0" announce started > "$work/started.txt" &
burst_pid=$!
wait "$burst_pid"
burst_pid=""
started_ok="$(grep -c '^ok$' "$work/started.txt" || true)"
if [[ "$started_ok" -ne "$PEERS" ]]; then
    echo "Invalid baseline: only $started_ok of $PEERS started announces succeeded" >&2
    exit 1
fi

seq 1 "$PEERS" | setsid xargs -P "$CONCURRENCY" -n 1 "$0" announce completed > "$work/completed.txt" &
burst_pid=$!

if [[ "$mode" == "race" ]]; then
    sleep "$SIGTERM_DELAY"
    kill -TERM "$tracker_pid"
    wait "$burst_pid" || true
else
    wait "$burst_pid"
    sleep "$CONTROL_WAIT"
    kill -TERM "$tracker_pid"
fi
burst_pid=""

tracker_status=0
shutdown_deadline=$((SECONDS + 20))
while kill -0 "$tracker_pid" 2>/dev/null && (( SECONDS < shutdown_deadline )); do
    sleep 0.1
done
if kill -0 "$tracker_pid" 2>/dev/null; then
    echo "Tracker exceeded the 20-second observation budget; see $log" >&2
    exit 1
fi
wait "$tracker_pid" || tracker_status=$?
tracker_pid=""

completed_ok="$(grep -c '^ok$' "$work/completed.txt" || true)"
completed_failed="$(grep -c '^fail$' "$work/completed.txt" || true)"
persisted_torrent="$(sqlite3 "$db" "SELECT COALESCE(SUM(completed), 0) FROM torrents;")"
persisted_global="$(sqlite3 "$db" "SELECT COALESCE(SUM(value), 0) FROM torrent_aggregate_metrics WHERE metric_name = 'torrents_downloads_total';")"

{
    echo "mode=$mode peers=$PEERS concurrency=$CONCURRENCY sigterm_delay=$SIGTERM_DELAY control_wait=$CONTROL_WAIT"
    echo "work_dir=$work"
    echo "tracker_exit_status=$tracker_status"
    echo "started_ok=$started_ok"
    echo "completed_ok=$completed_ok completed_failed=$completed_failed"
    echo "persisted_torrent_completed=$persisted_torrent"
    echo "persisted_global_downloads=$persisted_global"
    echo "answered_minus_persisted=$((completed_ok - persisted_torrent))"
} | tee "$work/summary.txt"
