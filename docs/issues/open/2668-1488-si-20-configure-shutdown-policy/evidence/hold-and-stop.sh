#!/usr/bin/env bash
# Disposable verification script for issue #2668 (see ISSUE.md, "Disposable Verification Scripts").
# Run from the repository root after `mkdir -p .tmp/si20`.
#
# Usage: hold-and-stop.sh <config.toml> <signal: TERM|INT> <hold: none|idle|partial-header|api-body|announce-body>
# Starts the real tracker binary, holds one connection open as described, signals the binary PID,
# and prints the exit status and the seconds from the signal to the exit. Bounded by a 40 s watchdog.
set -u
CONFIG="$1"; SIGNAL="$2"; HOLD="$3"
BIN=./target/debug/torrust-tracker
LOG=".tmp/si20/run-${SIGNAL}-${HOLD}.log"

# Sockets of the previous run can linger in TIME_WAIT and make the next bind fail.
for _ in $(seq 1 140); do
  ss -tan | grep -qE ':(17070|11212|11313)\b' || break
  sleep 0.5
done

"$BIN" -c "$CONFIG" >"$LOG" 2>&1 &
PID=$!
echo "tracker pid=$PID (binary, not a cargo launcher)"

for _ in $(seq 1 100); do
  grep -q "shutdown signal handlers installed" "$LOG" && break
  sleep 0.1
done
for _ in $(seq 1 100); do
  curl -fsS -o /dev/null http://127.0.0.1:11313/health_check 2>/dev/null && break
  sleep 0.1
done

case "$HOLD" in
  none) ;;
  idle) exec 3<>/dev/tcp/127.0.0.1/17070 ;;
  partial-header) exec 3<>/dev/tcp/127.0.0.1/17070; printf 'GET /health_check HTTP/1.1\r\nHost: x\r\n' >&3 ;;
  api-body) exec 3<>/dev/tcp/127.0.0.1/11212
            printf 'POST /api/v1/keys?token=MyAccessToken HTTP/1.1\r\nHost: x\r\nContent-Type: application/json\r\nContent-Length: 1000\r\n\r\n{"key":' >&3 ;;
  announce-body) exec 3<>/dev/tcp/127.0.0.1/17070
            printf 'GET /announce?info_hash=%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00%%00&peer_id=-qB0000-000000000000&port=6881&left=0 HTTP/1.1\r\nHost: x\r\nContent-Length: 1000\r\n\r\n' >&3 ;;
esac
sleep 1
START=$(date +%s.%N)
kill -"$SIGNAL" "$PID"
( sleep 40; kill -KILL "$PID" 2>/dev/null ) &
WATCHDOG=$!
wait "$PID"; STATUS=$?
kill "$WATCHDOG" 2>/dev/null
END=$(date +%s.%N)
exec 3>&- 2>/dev/null
printf 'signal=%s hold=%s exit_status=%s seconds_to_exit=%.1f\n' "$SIGNAL" "$HOLD" "$STATUS" "$(echo "$END - $START" | bc)"
grep -E "shutting down|Waiting for job|Job (completed|aborted|reported|panicked)|deadline|successfully shutdown|drain|Remaining alive|Forcing" "$LOG" | sed -E 's/^[^ ]+ +//' | cut -c1-220
