#!/usr/bin/env bash
set -euo pipefail

IMAGE="lscr.io/linuxserver/qbittorrent:5.1.4"
CONTAINER_NAME="qbt-login-probe"
DEFAULT_PASSWORD="adminadmin"
KEEP_ARTIFACTS=0
HOST_PORT=""

usage() {
  cat <<'EOF'
qBittorrent login probe utility.

Starts an isolated qBittorrent container with an explicit /config mount, then
runs login probes against /api/v2/auth/login with different CSRF headers.

Use this script when the WebUI does not load in a browser, login returns 401,
or you need to confirm how qBittorrent validates Host, Referer, and Origin.

Usage:
  qbittorrent-login-probe.sh [options]

Options:
  --image <image>        qBittorrent image to run.
                         Default: lscr.io/linuxserver/qbittorrent:5.1.4
  --name <container>     Container name.
                         Default: qbt-login-probe
  --password <password>  Password candidate to test.
                         Default: adminadmin
  --host-port <port>     Publish WebUI on a fixed host port.
                         Use 8080 for browser access.
  --keep                 Keep container and temp directory for manual inspection.
  -h, --help             Show this help.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --image)
      IMAGE="$2"
      shift 2
      ;;
    --name)
      CONTAINER_NAME="$2"
      shift 2
      ;;
    --password)
      DEFAULT_PASSWORD="$2"
      shift 2
      ;;
    --host-port)
      HOST_PORT="$2"
      shift 2
      ;;
    --keep)
      KEEP_ARTIFACTS=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
done

WORKDIR="$(mktemp -d /tmp/qbt-login-probe.XXXXXX)"
CONFIG_ROOT="$WORKDIR/config"
DOWNLOADS_DIR="$WORKDIR/downloads"

cleanup() {
  if [[ "$KEEP_ARTIFACTS" -eq 0 ]]; then
    docker rm -f "$CONTAINER_NAME" >/dev/null 2>&1 || true
    rm -rf "$WORKDIR"
  else
    echo "Keeping artifacts for inspection:"
    echo "  WORKDIR=$WORKDIR"
    echo "  CONTAINER=$CONTAINER_NAME"
  fi
}
trap cleanup EXIT

mkdir -p \
  "$CONFIG_ROOT/qBittorrent" \
  "$CONFIG_ROOT/qBittorrent/BT_backup" \
  "$CONFIG_ROOT/.cache/qBittorrent" \
  "$DOWNLOADS_DIR"

cat > "$CONFIG_ROOT/qBittorrent/qBittorrent.conf" <<'EOF'
[BitTorrent]
Session\AddTorrentStopped=false
Session\DefaultSavePath=/downloads
Session\TempPath=/downloads/temp
[Preferences]
WebUI\LocalHostAuth=false
WebUI\Port=8080
WebUI\Username=admin
WebUI\AuthSubnetWhitelistEnabled=true
WebUI\AuthSubnetWhitelist=0.0.0.0/0,::/0
EOF

docker rm -f "$CONTAINER_NAME" >/dev/null 2>&1 || true

PORT_MAPPING="0:8080"
if [[ -n "$HOST_PORT" ]]; then
  PORT_MAPPING="${HOST_PORT}:8080"
fi

docker run -d --rm \
  --name "$CONTAINER_NAME" \
  -e WEBUI_PORT=8080 \
  -e PUID=1000 \
  -e PGID=1000 \
  -e TZ=UTC \
  -e QBT_LEGAL_NOTICE=confirm \
  -v "$CONFIG_ROOT:/config" \
  -v "$DOWNLOADS_DIR:/downloads" \
  -p "$PORT_MAPPING" \
  "$IMAGE" >/dev/null

for _ in $(seq 1 60); do
  if docker port "$CONTAINER_NAME" 8080/tcp >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

HOST_PORT="$(docker port "$CONTAINER_NAME" 8080/tcp | awk -F: '{print $2}')"
BASE_URL="http://127.0.0.1:${HOST_PORT}"

echo "Probe container: $CONTAINER_NAME"
echo "Image: $IMAGE"
echo "Base URL: $BASE_URL"
echo "Workdir: $WORKDIR"

for _ in $(seq 1 60); do
  if docker logs "$CONTAINER_NAME" 2>&1 | grep -q "WebUI will be started shortly\|A temporary password is provided for this session:"; then
    break
  fi
  sleep 1
done

echo
echo "=== Container logs (tail) ==="
docker logs "$CONTAINER_NAME" 2>&1 | tail -60

TEMP_PASSWORD="$(docker logs "$CONTAINER_NAME" 2>&1 | sed -n 's/.*A temporary password is provided for this session:[[:space:]]*//p' | tail -1)"
PASSWORDS=("$DEFAULT_PASSWORD")
if [[ -n "$TEMP_PASSWORD" ]]; then
  PASSWORDS+=("$TEMP_PASSWORD")
fi

probe_login() {
  local label="$1"
  local password="$2"
  shift 2
  local outfile
  outfile="$(mktemp /tmp/qbt-probe-body.XXXXXX)"

  local status
  status="$(curl -sS -o "$outfile" -w '%{http_code}' \
    -X POST "$BASE_URL/api/v2/auth/login" \
    -H 'Content-Type: application/x-www-form-urlencoded' \
    "$@" \
    --data "username=admin&password=${password}")"

  local body
  body="$(cat "$outfile")"
  rm -f "$outfile"

  echo "$label | password='${password}' | HTTP=${status} | body='${body}'"
}

echo
echo "=== Login probes ==="
for password in "${PASSWORDS[@]}"; do
  probe_login "no-referer" "$password"
  probe_login "referer-base" "$password" -H "Referer: $BASE_URL"
  probe_login "origin-base" "$password" -H "Origin: $BASE_URL"
  probe_login "host+referer-localhost-8080" "$password" -H "Host: localhost:8080" -H "Referer: http://localhost:8080"
  probe_login "host+origin-localhost-8080" "$password" -H "Host: localhost:8080" -H "Origin: http://localhost:8080"
  probe_login "host+referer-127-8080" "$password" -H "Host: 127.0.0.1:8080" -H "Referer: http://127.0.0.1:8080"
  probe_login "host+origin-127-8080" "$password" -H "Host: 127.0.0.1:8080" -H "Origin: http://127.0.0.1:8080"
done

echo
echo "Done."
