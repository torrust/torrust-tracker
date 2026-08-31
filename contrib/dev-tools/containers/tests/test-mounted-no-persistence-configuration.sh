#!/usr/bin/env bash
# issue: #2107
# Before changing this regression, review the deferred persistence-transition
# test and entrypoint refactor plan in #2107.
# Release-image regression for mounted v3 configuration and SQLite transitions.
#
# Run locally after modifying the container entrypoint or Containerfile:
#   bash contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh
#
# Reuse an existing image, for example in CI:
#   IMAGE_TAG=torrust-tracker:local BUILD_IMAGE=false \
#     bash contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
TEST_DIRECTORY=$(mktemp -d "${TMPDIR:-/tmp}/test-mounted-no-persistence-configuration.XXXXXX")
IMAGE_TAG=${IMAGE_TAG:-torrust-tracker:test-mounted-no-persistence-configuration}
BUILD_IMAGE=${BUILD_IMAGE:-true}
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

build_release_image() {
    if [ "${BUILD_IMAGE}" = true ]; then
        docker build \
            --target release \
            --tag "${IMAGE_TAG}" \
            --file "${PROJECT_ROOT}/Containerfile" \
            "${PROJECT_ROOT}"
    fi
}

assert_mounted_no_persistence_configuration_is_preserved() {
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
    cmp "${NO_PERSISTENCE_CONFIGURATION}" "${MOUNTED_CONFIGURATION}"
}

assert_entrypoint_created_sqlite_storage_is_writable() {
    docker run --rm --entrypoint /bin/sh \
        --env USER_ID="$(id -u)" \
        --env TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER=sqlite3 \
        "${IMAGE_TAG}" \
        -c '/usr/local/bin/entry.sh true && /bin/su-exec torrust test -w /var/lib/torrust/tracker/database'
}

assert_sqlite_transitions_are_non_destructive() {
    mkdir -p "${TEST_DIRECTORY}/lib/database"

    # Create the first selected target, then preserve it while persistence is disabled.
    configure_sqlite_database old.sqlite3
    run_tracker
    test -f "${OLD_DATABASE}"
    old_database_checksum=$(sha256sum "${OLD_DATABASE}")

    cp "${NO_PERSISTENCE_CONFIGURATION}" "${MOUNTED_CONFIGURATION}"
    run_tracker
    test "${old_database_checksum}" = "$(sha256sum "${OLD_DATABASE}")"
    test ! -e "${NEW_DATABASE}"

    # Selecting a new target must not modify the original target.
    configure_sqlite_database new.sqlite3
    run_tracker
    test -f "${NEW_DATABASE}"
    test "${old_database_checksum}" = "$(sha256sum "${OLD_DATABASE}")"
    new_database_checksum=$(sha256sum "${NEW_DATABASE}")

    # Reusing the original target must not modify the unselected new target.
    configure_sqlite_database old.sqlite3
    run_tracker
    test "${old_database_checksum}" = "$(sha256sum "${OLD_DATABASE}")"
    test "${new_database_checksum}" = "$(sha256sum "${NEW_DATABASE}")"
}

build_release_image
assert_mounted_no_persistence_configuration_is_preserved
assert_entrypoint_created_sqlite_storage_is_writable
assert_sqlite_transitions_are_non_destructive

printf '%s\n' 'mounted-no-persistence-config-preserved-without-sqlite-artifacts'
printf '%s\n' 'unselected-sqlite-targets-remain-unchanged-across-transitions'