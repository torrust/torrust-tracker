#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

COMPOSE_FILE="$REPO_ROOT/compose.qbittorrent-e2e.sqlite3.yaml"
TRACKER_IMAGE="torrust-tracker:qbt-e2e-local"
QBITTORRENT_IMAGE="lscr.io/linuxserver/qbittorrent:5.1.4"
PROJECT_NAME="qbt-e2e-composecheck-$(date +%s)"
KEEP_STACK=0
SKIP_BUILD=0

usage() {
  cat <<'EOF'
Usage: check-qbittorrent-e2e-compose.sh [options]

Validate that the qBittorrent E2E compose stack can be rendered, started, and
inspected before debugging the Rust runner.

Options:
  --project-name <name>     Docker compose project name.
  --compose-file <path>     Compose file to validate and run.
  --tracker-image <image>   Tracker image tag.
  --qb-image <image>        qBittorrent image tag.
  --skip-build              Skip building tracker image when missing.
  --keep-stack              Keep containers up after checks.
  -h, --help                Show this help message.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --project-name)
      PROJECT_NAME="$2"
      shift 2
      ;;
    --compose-file)
      COMPOSE_FILE="$2"
      shift 2
      ;;
    --tracker-image)
      TRACKER_IMAGE="$2"
      shift 2
      ;;
    --qb-image)
      QBITTORRENT_IMAGE="$2"
      shift 2
      ;;
    --skip-build)
      SKIP_BUILD=1
      shift
      ;;
    --keep-stack)
      KEEP_STACK=1
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

if [[ ! -f "$COMPOSE_FILE" ]]; then
  echo "Compose file not found: $COMPOSE_FILE" >&2
  exit 1
fi

if ! command -v docker >/dev/null 2>&1; then
  echo "docker command not found" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
TRACKER_CONFIG_SOURCE="$REPO_ROOT/share/default/config/tracker.e2e.container.sqlite3.toml"
TRACKER_CONFIG_PATH="$TMP_DIR/tracker-config.toml"
TRACKER_STORAGE_PATH="$TMP_DIR/tracker-storage"
SHARED_PATH="$TMP_DIR/shared"
SEEDER_CONFIG_PATH="$TMP_DIR/seeder-config"
LEECHER_CONFIG_PATH="$TMP_DIR/leecher-config"
SEEDER_DOWNLOADS_PATH="$TMP_DIR/seeder-downloads"
LEECHER_DOWNLOADS_PATH="$TMP_DIR/leecher-downloads"

cleanup() {
  if [[ "$KEEP_STACK" -eq 0 ]]; then
    QBT_E2E_TRACKER_IMAGE="$TRACKER_IMAGE" \
    QBT_E2E_QBITTORRENT_IMAGE="$QBITTORRENT_IMAGE" \
    QBT_E2E_TRACKER_CONFIG_PATH="$TRACKER_CONFIG_PATH" \
    QBT_E2E_TRACKER_STORAGE_PATH="$TRACKER_STORAGE_PATH" \
    QBT_E2E_SHARED_PATH="$SHARED_PATH" \
    QBT_E2E_SEEDER_CONFIG_PATH="$SEEDER_CONFIG_PATH" \
    QBT_E2E_LEECHER_CONFIG_PATH="$LEECHER_CONFIG_PATH" \
    QBT_E2E_SEEDER_DOWNLOADS_PATH="$SEEDER_DOWNLOADS_PATH" \
    QBT_E2E_LEECHER_DOWNLOADS_PATH="$LEECHER_DOWNLOADS_PATH" \
      docker compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" down --volumes --remove-orphans || true
  fi

  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

if [[ ! -f "$TRACKER_CONFIG_SOURCE" ]]; then
  echo "Tracker config template not found: $TRACKER_CONFIG_SOURCE" >&2
  exit 1
fi

mkdir -p \
  "$TRACKER_STORAGE_PATH" \
  "$SHARED_PATH" \
  "$SEEDER_CONFIG_PATH" \
  "$LEECHER_CONFIG_PATH" \
  "$SEEDER_DOWNLOADS_PATH" \
  "$LEECHER_DOWNLOADS_PATH"
cp "$TRACKER_CONFIG_SOURCE" "$TRACKER_CONFIG_PATH"

if [[ "$SKIP_BUILD" -eq 0 ]] && ! docker image inspect "$TRACKER_IMAGE" >/dev/null 2>&1; then
  echo "Building tracker image: $TRACKER_IMAGE"
  docker build -f "$REPO_ROOT/Containerfile" --target release -t "$TRACKER_IMAGE" "$REPO_ROOT"
fi

echo "Validating compose config"
QBT_E2E_TRACKER_IMAGE="$TRACKER_IMAGE" \
QBT_E2E_QBITTORRENT_IMAGE="$QBITTORRENT_IMAGE" \
QBT_E2E_TRACKER_CONFIG_PATH="$TRACKER_CONFIG_PATH" \
QBT_E2E_TRACKER_STORAGE_PATH="$TRACKER_STORAGE_PATH" \
QBT_E2E_SHARED_PATH="$SHARED_PATH" \
QBT_E2E_SEEDER_CONFIG_PATH="$SEEDER_CONFIG_PATH" \
QBT_E2E_LEECHER_CONFIG_PATH="$LEECHER_CONFIG_PATH" \
QBT_E2E_SEEDER_DOWNLOADS_PATH="$SEEDER_DOWNLOADS_PATH" \
QBT_E2E_LEECHER_DOWNLOADS_PATH="$LEECHER_DOWNLOADS_PATH" \
  docker compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" config -q

echo "Bringing stack up"
QBT_E2E_TRACKER_IMAGE="$TRACKER_IMAGE" \
QBT_E2E_QBITTORRENT_IMAGE="$QBITTORRENT_IMAGE" \
QBT_E2E_TRACKER_CONFIG_PATH="$TRACKER_CONFIG_PATH" \
QBT_E2E_TRACKER_STORAGE_PATH="$TRACKER_STORAGE_PATH" \
QBT_E2E_SHARED_PATH="$SHARED_PATH" \
QBT_E2E_SEEDER_CONFIG_PATH="$SEEDER_CONFIG_PATH" \
QBT_E2E_LEECHER_CONFIG_PATH="$LEECHER_CONFIG_PATH" \
QBT_E2E_SEEDER_DOWNLOADS_PATH="$SEEDER_DOWNLOADS_PATH" \
QBT_E2E_LEECHER_DOWNLOADS_PATH="$LEECHER_DOWNLOADS_PATH" \
  docker compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" up -d

echo "Container status"
QBT_E2E_TRACKER_IMAGE="$TRACKER_IMAGE" \
QBT_E2E_QBITTORRENT_IMAGE="$QBITTORRENT_IMAGE" \
QBT_E2E_TRACKER_CONFIG_PATH="$TRACKER_CONFIG_PATH" \
QBT_E2E_TRACKER_STORAGE_PATH="$TRACKER_STORAGE_PATH" \
QBT_E2E_SHARED_PATH="$SHARED_PATH" \
QBT_E2E_SEEDER_CONFIG_PATH="$SEEDER_CONFIG_PATH" \
QBT_E2E_LEECHER_CONFIG_PATH="$LEECHER_CONFIG_PATH" \
QBT_E2E_SEEDER_DOWNLOADS_PATH="$SEEDER_DOWNLOADS_PATH" \
QBT_E2E_LEECHER_DOWNLOADS_PATH="$LEECHER_DOWNLOADS_PATH" \
  docker compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" ps -a

for service in qbittorrent-seeder qbittorrent-leecher; do
  echo "Resolving port mapping for ${service}:8080"
  QBT_E2E_TRACKER_IMAGE="$TRACKER_IMAGE" \
  QBT_E2E_QBITTORRENT_IMAGE="$QBITTORRENT_IMAGE" \
  QBT_E2E_TRACKER_CONFIG_PATH="$TRACKER_CONFIG_PATH" \
  QBT_E2E_TRACKER_STORAGE_PATH="$TRACKER_STORAGE_PATH" \
  QBT_E2E_SHARED_PATH="$SHARED_PATH" \
  QBT_E2E_SEEDER_CONFIG_PATH="$SEEDER_CONFIG_PATH" \
  QBT_E2E_LEECHER_CONFIG_PATH="$LEECHER_CONFIG_PATH" \
  QBT_E2E_SEEDER_DOWNLOADS_PATH="$SEEDER_DOWNLOADS_PATH" \
  QBT_E2E_LEECHER_DOWNLOADS_PATH="$LEECHER_DOWNLOADS_PATH" \
    docker compose -f "$COMPOSE_FILE" -p "$PROJECT_NAME" port "$service" 8080

done

echo "Compose check completed successfully"
if [[ "$KEEP_STACK" -eq 1 ]]; then
  echo "Stack kept running (project: $PROJECT_NAME)"
fi
