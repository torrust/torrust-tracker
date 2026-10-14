#!/usr/bin/env bash
# Disposable reproduction probe for the UDP banned-IPs metric bug.
# See ISSUE.md "Disposable Verification Scripts" and manual-verification-evidence.md.
# Requires: OpenBSD netcat (nc), xxd, curl, jq, and a tracker started with the
# configuration embedded in manual-verification-evidence.md.
# Sends raw BEP 15 packets from a chosen loopback source IP and reads the
# banned-IPs value from the REST API (JSON and Prometheus).
set -euo pipefail

TRACKER="127.0.0.1:16969"
API="http://127.0.0.1:11212/api/v1"
TOKEN="MyAccessToken"

# 98-byte announce request with connection ID 0 (an invalid cookie).
# conn_id | action=1 | tx_id | info_hash | peer_id | downloaded | left | uploaded | event | ip | key | num_want | port
announce_hex() {
    printf '%s' \
        "0000000000000000" "00000001" "$1" \
        "9c38422213e30bff212b30c360d26f9a02136422" \
        "2d71423030303030303030303030303030303031" \
        "0000000000000000" "0000000000000000" "0000000000000000" \
        "00000000" "00000000" "00000000" "ffffffff" "1ae1"
}

# 16-byte connect request: protocol_id | action=0 | tx_id
connect_hex() {
    printf '%s' "0000041727101980" "00000000" "$1"
}

# send <source-ip> <hex-payload>: prints the response as hex, or "<no response>".
send() {
    local response
    local host="${TRACKER%:*}" port="${TRACKER##*:}"
    response=$(printf '%s' "$2" | xxd -r -p | nc -u -w 2 -s "$1" "$host" "$port" | xxd -p | tr -d '\n')
    if [ -z "$response" ]; then echo "<no response>"; return; fi
    echo "response (hex): $response"
    # action 3 = error: print its message; action 0 = connect: print the connection ID.
    case "${response:0:8}" in
        00000003) echo "response (error message): $(printf '%s' "${response:16}" | xxd -r -p)" ;;
        00000000) echo "response (connect): connection_id=${response:16:16}" ;;
    esac
}

usage() {
    echo "usage: $0 {invalid-announce <src-ip> [tx]|connect <src-ip> [tx]|stats}" >&2
    exit 2
}

# require_source_ip <argument-count>: the packet commands need a source IP.
require_source_ip() {
    if [ "$1" -lt 2 ]; then usage; fi
}

if [ "$#" -lt 1 ]; then usage; fi

case "$1" in
    invalid-announce) require_source_ip "$#"; send "$2" "$(announce_hex "${3:-0000abcd}")" ;;
    connect) require_source_ip "$#"; send "$2" "$(connect_hex "${3:-00001234}")" ;;
    stats)
        echo "JSON  /stats   udp_banned_ips_total: $(curl -s "${API}/stats?token=${TOKEN}" | jq '.udp_banned_ips_total')"
        echo "Prom  /stats   $(curl -s "${API}/stats?token=${TOKEN}&format=prometheus" | grep '^udp_banned_ips_total')"
        echo "Prom  /metrics $(curl -s "${API}/metrics?token=${TOKEN}&format=prometheus" | grep '^udp_tracker_server_ips_banned_total' || echo 'udp_tracker_server_ips_banned_total <absent>')"
        echo "Prom  /metrics $(curl -s "${API}/metrics?token=${TOKEN}&format=prometheus" | grep '^udp_tracker_server_requests_banned_total' || echo 'udp_tracker_server_requests_banned_total <absent>')"
        ;;
    *) usage ;;
esac
