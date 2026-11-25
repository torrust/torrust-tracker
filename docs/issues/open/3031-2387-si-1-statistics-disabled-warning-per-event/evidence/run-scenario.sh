#!/bin/bash
# usage: run.sh <config> <out.log>   (run from the repo root; real release binary)
set -u
CFG=$1; OUT=$2; HASH=443c7602b4fde83d1154d6d9da48808418b181b6
rm -f "$(dirname "$CFG")/sqlite3.db"
./target/release/torrust-tracker --config-toml-path "$CFG" > "$OUT.raw" 2>&1 &
PID=$!
for i in $(seq 1 60); do curl -fsS http://127.0.0.1:1313/health_check >/dev/null 2>&1 && break; sleep 0.5; done
sleep 1
CLIENT=./target/debug/tracker_client
$CLIENT udp announce 127.0.0.1:6868 $HASH --event started --format text > "$OUT.udp.txt" 2>&1
$CLIENT http announce http://127.0.0.1:7070 $HASH --event started > "$OUT.http.txt" 2>&1
curl -fsS -H 'Authorization: Bearer MyAccessToken' "http://127.0.0.1:1212/api/v1/metrics?token=MyAccessToken&format=prometheus" > "$OUT.metrics.txt" 2>&1
sleep 1
kill -TERM $PID; wait $PID; echo "exit=$?" > "$OUT.exit"
sed 's/\x1b\[[0-9;]*m//g' "$OUT.raw" > "$OUT"
echo "lines=$(wc -l < $OUT) warn=$(grep -c ' WARN ' $OUT) error=$(grep -c ' ERROR ' $OUT)"
