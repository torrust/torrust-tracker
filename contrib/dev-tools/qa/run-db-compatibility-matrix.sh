#!/bin/bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"

POSTGRES_VERSIONS_STRING="${POSTGRES_VERSIONS:-14 15 16 17}"
MYSQL_VERSIONS_STRING="${MYSQL_VERSIONS:-8.0 8.4}"

read -r -a POSTGRES_VERSIONS <<< "$POSTGRES_VERSIONS_STRING"
read -r -a MYSQL_VERSIONS <<< "$MYSQL_VERSIONS_STRING"

run_step() {
    echo
    echo "==> $*"
    "$@"
}

run_step bash -lc "cd '$ROOT_DIR' && cargo check --workspace --all-targets"

echo
echo "==> SQLite runtime version"
sqlite3 --version

run_step bash -lc "cd '$ROOT_DIR' && cargo test -p torrust-tracker-configuration postgresql_user_password"
run_step bash -lc "cd '$ROOT_DIR' && cargo test -p bittorrent-http-tracker-protocol saturate_large_download_counts"
run_step bash -lc "cd '$ROOT_DIR' && cargo test -p torrust-udp-tracker-server saturate_large_download_counts_for_udp_protocol"
run_step bash -lc "cd '$ROOT_DIR' && cargo test -p bittorrent-tracker-core run_sqlite_driver_tests -- --nocapture"

for version in "${MYSQL_VERSIONS[@]}"; do
    echo
    echo "==> MySQL compatibility test on ${version}"
    docker pull "mysql:${version}"
    TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=1 \
    TORRUST_TRACKER_CORE_MYSQL_DRIVER_IMAGE_TAG="${version}" \
        bash -lc "cd '$ROOT_DIR' && cargo test -p bittorrent-tracker-core run_mysql_driver_tests -- --nocapture"
done

for version in "${POSTGRES_VERSIONS[@]}"; do
    echo
    echo "==> PostgreSQL compatibility test on ${version}"
    docker pull "postgres:${version}"
    TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=1 \
    TORRUST_TRACKER_CORE_POSTGRES_DRIVER_IMAGE_TAG="${version}" \
        bash -lc "cd '$ROOT_DIR' && cargo test -p bittorrent-tracker-core run_postgres_driver_tests -- --nocapture"
done

echo
echo "Database compatibility matrix finished successfully."
