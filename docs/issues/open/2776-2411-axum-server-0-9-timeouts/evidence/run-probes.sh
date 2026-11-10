#!/bin/bash
# Runs every idle-connection probe, one at a time, against a fresh tracker.
# usage: run-probes.sh <scratch-dir> <tracker-binary> <baseline-probe.py> <probe-h2.py>
# <scratch-dir> holds tracker.toml (ports 27070 plain, 27171 TLS, 21212 REST API, 21313 health check),
# cert.pem and key.pem. The run takes about two minutes.
set -u
scratch=$1 tracker=$2 probe=$3 probe_h2=$4

cd "$scratch" || exit 1
rm -f sqlite3.db
TORRUST_TRACKER_CONFIG_TOML_PATH="$scratch/tracker.toml" "$tracker" >"$scratch/tracker.log" 2>&1 &
pid=$!
sleep 3

python3 -I "$probe" M1-http-plain 27070 plain
python3 -I "$probe" M1-rest-plain 21212 plain
python3 -I "$probe" M1-health 21313 plain
python3 -I "$probe" M2-tls-silent 27171 tls
python3 -I "$probe" M3-tls-no-handshake 27171 plain
python3 -I "$probe" M4-http-keep-alive 27070 persistent /health_check
python3 -I "$probe" M4-health-keep-alive 21313 persistent /health_check
python3 -I "$probe_h2" M4-h2-http-plain 27070 plain
python3 -I "$probe_h2" M4-h2-http-tls 27171 tls
python3 -I "$probe_h2" M4-h2-health-plain 21313 plain
python3 -I "$probe_h2" M4-h2-rest-plain 21212 plain

kill -INT "$pid"
