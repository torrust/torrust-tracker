#!/usr/bin/env bash
# Integration test for mounted v3 configuration and persistence target transitions.
#
# Run manually after modifying the container entrypoint:
#   bash contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
TEST_DIRECTORY=$(mktemp -d "${TMPDIR:-/tmp}/test-mounted-no-persistence-configuration.XXXXXX")
IMAGE_TAG=torrust-tracker:test-mounted-no-persistence-configuration
trap 'rm -rf "${TEST_DIRECTORY}"' EXIT

mkdir -p "${TEST_DIRECTORY}/etc" "${TEST_DIRECTORY}/lib" "${TEST_DIRECTORY}/log"
NO_PERSISTENCE_CONFIGURATION="${PROJECT_ROOT}/share/default/config/tracker.container.no-persistence.toml"
SQLITE_CONFIGURATION="${PROJECT_ROOT}/share/default/config/tracker.container.sqlite3.toml"
MOUNTED_CONFIGURATION="${TEST_DIRECTORY}/etc/tracker.toml"
OLD_DATABASE="${TEST_DIRECTORY}/lib/database/old.sqlite3"
NEW_DATABASE="${TEST_DIRECTORY}/lib/database/new.sqlite3"

run_tracker() {
    local exit_status=0

    timeout --signal=INT --kill-after=3s 10s docker run --rm \
        --env USER_ID="$(id -u)" \
        --volume "${TEST_DIRECTORY}/etc:/etc/torrust/tracker:rw" \
        --volume "${TEST_DIRECTORY}/lib:/var/lib/torrust/tracker:rw" \
        --volume "${TEST_DIRECTORY}/log:/var/log/torrust/tracker:rw" \
        "${IMAGE_TAG}" || exit_status=$?

    test "${exit_status}" -eq 0 -o "${exit_status}" -eq 124
}

configure_sqlite_database() {
    local database_name=$1

    sed "s|path = \"/var/lib/torrust/tracker/database/sqlite3.db\"|path = \"/var/lib/torrust/tracker/database/${database_name}\"|" \
        "${SQLITE_CONFIGURATION}" >"${MOUNTED_CONFIGURATION}"
}

docker build \
    --target release \
    --tag "${IMAGE_TAG}" \
    --file "${PROJECT_ROOT}/Containerfile" \
    "${PROJECT_ROOT}"

cp "${NO_PERSISTENCE_CONFIGURATION}" "${MOUNTED_CONFIGURATION}"

docker run --rm --entrypoint /bin/sh \
    --env USER_ID="$(id -u)" \
    --env TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER=sqlite3 \
    --volume "${TEST_DIRECTORY}/etc:/etc/torrust/tracker:rw" \
    --volume "${TEST_DIRECTORY}/lib:/var/lib/torrust/tracker:rw" \
    --volume "${TEST_DIRECTORY}/log:/var/log/torrust/tracker:rw" \
    "${IMAGE_TAG}" \
    -c '/usr/local/bin/entry.sh true && test ! -e /var/lib/torrust/tracker/database'

test ! -e "${TEST_DIRECTORY}/lib/database"
cmp \
    "${NO_PERSISTENCE_CONFIGURATION}" \
    "${MOUNTED_CONFIGURATION}"

docker run --rm --entrypoint /bin/sh \
    --env USER_ID="$(id -u)" \
    --env TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER=sqlite3 \
    "${IMAGE_TAG}" \
    -c '/usr/local/bin/entry.sh true && /bin/su-exec torrust test -w /var/lib/torrust/tracker/database'

mkdir -p "${TEST_DIRECTORY}/lib/database"
configure_sqlite_database old.sqlite3
run_tracker
test -f "${OLD_DATABASE}"
old_database_checksum=$(sha256sum "${OLD_DATABASE}")

cp "${NO_PERSISTENCE_CONFIGURATION}" "${MOUNTED_CONFIGURATION}"
run_tracker
test "${old_database_checksum}" = "$(sha256sum "${OLD_DATABASE}")"
test ! -e "${NEW_DATABASE}"

configure_sqlite_database new.sqlite3
run_tracker
test -f "${NEW_DATABASE}"
test "${old_database_checksum}" = "$(sha256sum "${OLD_DATABASE}")"
new_database_checksum=$(sha256sum "${NEW_DATABASE}")

configure_sqlite_database old.sqlite3
run_tracker
test "${old_database_checksum}" = "$(sha256sum "${OLD_DATABASE}")"
test "${new_database_checksum}" = "$(sha256sum "${NEW_DATABASE}")"

printf '%s\n' 'mounted-no-persistence-config-preserved-without-sqlite-artifacts'
printf '%s\n' 'unselected-sqlite-targets-remain-unchanged-across-transitions'