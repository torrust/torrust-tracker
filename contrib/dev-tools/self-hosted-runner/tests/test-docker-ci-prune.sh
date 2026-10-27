#!/usr/bin/env bash
# Tests for the self-hosted runner Docker cleanup script.
#
# Script under test: ../docker-ci-prune.sh
#
# NOTE: These tests are NOT automatically run by the pre-commit hook or CI.
# Run them manually after modifying the script:
#   bash contrib/dev-tools/self-hosted-runner/tests/test-docker-ci-prune.sh

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
SCRIPT_UNDER_TEST="${PROJECT_ROOT}/contrib/dev-tools/self-hosted-runner/docker-ci-prune.sh"
TEST_DIRECTORY=$(mktemp -d "${TMPDIR:-/tmp}/test-docker-ci-prune.XXXXXX")
trap 'rm -rf "${TEST_DIRECTORY}"' EXIT

# Builds a directory of stub commands that record their calls in $TEST_COMMAND_LOG.
#   df:      prints the next value of $TEST_DISK_USAGE_SEQUENCE (the last one repeats).
#   pgrep:   reports a running job when $TEST_JOB_RUNNING is 1.
#   docker:  `docker info` fails when $TEST_DOCKER_DOWN is 1.
create_fixture() {
    local fixture_name=$1
    local fixture_root="${TEST_DIRECTORY}/${fixture_name}"

    mkdir -p "${fixture_root}/bin"
    : >"${fixture_root}/commands.log"
    printf '%s\n' "${TEST_DISK_USAGE_SEQUENCE:-40}" | tr ' ' '\n' >"${fixture_root}/disk-usage"

    cat >"${fixture_root}/bin/docker" <<'EOF'
#!/usr/bin/env bash
printf 'docker %s\n' "$*" >>"${TEST_COMMAND_LOG}"
if [ "$*" = "info" ] && [ "${TEST_DOCKER_DOWN:-0}" = "1" ]; then
    exit 1
fi
EOF
    cat >"${fixture_root}/bin/systemctl" <<'EOF'
#!/usr/bin/env bash
printf 'systemctl %s\n' "$*" >>"${TEST_COMMAND_LOG}"
EOF
    cat >"${fixture_root}/bin/pgrep" <<'EOF'
#!/usr/bin/env bash
printf 'pgrep %s\n' "$*" >>"${TEST_COMMAND_LOG}"
[ "${TEST_JOB_RUNNING:-0}" = "1" ]
EOF
    cat >"${fixture_root}/bin/sleep" <<'EOF'
#!/usr/bin/env bash
EOF
    cat >"${fixture_root}/bin/df" <<'EOF'
#!/usr/bin/env bash
usage=$(head -n 1 "${TEST_DISK_USAGE_FILE}")
if [ "$(wc -l <"${TEST_DISK_USAGE_FILE}")" -gt 1 ]; then
    tail -n +2 "${TEST_DISK_USAGE_FILE}" >"${TEST_DISK_USAGE_FILE}.next"
    mv "${TEST_DISK_USAGE_FILE}.next" "${TEST_DISK_USAGE_FILE}"
fi
printf 'Use%%\n %s%%\n' "${usage}"
EOF
    chmod +x "${fixture_root}/bin/"*

    printf '%s\n' "${fixture_root}"
}

# Runs the script with the fixture's stubs first in PATH; sets $status and saves the output in the fixture.
run_script() {
    local fixture_root=$1

    status=0
    PATH="${fixture_root}/bin:${PATH}" \
        TEST_COMMAND_LOG="${fixture_root}/commands.log" \
        TEST_DISK_USAGE_FILE="${fixture_root}/disk-usage" \
        DOCKER_CI_PRUNE_READY_ATTEMPTS=3 \
        "${SCRIPT_UNDER_TEST}" >"${fixture_root}/script-output.txt" 2>&1 || status=$?
}

assert_command_logged() {
    local fixture_root=$1
    local expected=$2

    if ! grep -qxF -- "${expected}" "${fixture_root}/commands.log"; then
        printf 'Expected command not run: %s\nCommands run:\n' "${expected}" >&2
        cat "${fixture_root}/commands.log" >&2
        exit 1
    fi
}

assert_command_not_logged() {
    local fixture_root=$1
    local unexpected=$2

    if grep -qxF -- "${unexpected}" "${fixture_root}/commands.log"; then
        printf 'Unexpected command run: %s\nCommands run:\n' "${unexpected}" >&2
        cat "${fixture_root}/commands.log" >&2
        exit 1
    fi
}

count_commands() {
    local fixture_root=$1
    local expected=$2

    grep -cxF -- "${expected}" "${fixture_root}/commands.log" || true
}

assert_equals() {
    local expected=$1
    local actual=$2
    local description=$3

    if [ "${expected}" != "${actual}" ]; then
        printf 'Expected %s to be "%s", got "%s".\n' "${description}" "${expected}" "${actual}" >&2
        exit 1
    fi
}

readonly CACHE_PRUNE="docker builder prune --force --max-used-space 120GB"
readonly RESTART_DOCKER="systemctl restart docker.service"

it_should_prune_build_cache_containers_images_and_volumes_when_disk_usage_is_low() {
    # Arrange
    local fixture_root status
    TEST_DISK_USAGE_SEQUENCE="40" fixture_root=$(create_fixture "low-usage")

    # Act
    run_script "${fixture_root}"

    # Assert
    assert_equals 0 "${status}" "exit status"
    assert_command_logged "${fixture_root}" "${CACHE_PRUNE}"
    assert_command_logged "${fixture_root}" "docker container prune --force --filter until=24h"
    assert_command_logged "${fixture_root}" "docker image prune --force --filter until=168h"
    assert_command_logged "${fixture_root}" "docker volume prune --force"
}

it_should_not_restart_docker_when_disk_usage_is_below_the_restart_threshold() {
    # Arrange
    local fixture_root status
    TEST_DISK_USAGE_SEQUENCE="69" fixture_root=$(create_fixture "below-threshold")

    # Act
    run_script "${fixture_root}"

    # Assert
    assert_equals 0 "${status}" "exit status"
    assert_command_not_logged "${fixture_root}" "${RESTART_DOCKER}"
}

it_should_restart_docker_and_prune_again_when_usage_stays_high_and_no_job_is_running() {
    # Arrange
    local fixture_root status
    TEST_DISK_USAGE_SEQUENCE="75 40" fixture_root=$(create_fixture "restart-when-idle")

    # Act
    run_script "${fixture_root}"

    # Assert
    assert_equals 0 "${status}" "exit status"
    assert_command_logged "${fixture_root}" "${RESTART_DOCKER}"
    assert_equals 2 "$(count_commands "${fixture_root}" "${CACHE_PRUNE}")" "number of build cache prunes"
}

it_should_wait_for_docker_before_pruning_again_after_the_restart() {
    # Arrange
    local fixture_root status order
    TEST_DISK_USAGE_SEQUENCE="75 40" fixture_root=$(create_fixture "wait-for-docker")

    # Act
    run_script "${fixture_root}"

    # Assert
    order=$(grep -nxE "${RESTART_DOCKER}|docker info|${CACHE_PRUNE}" "${fixture_root}/commands.log" | cut -d: -f2- | tr '\n' ';')
    assert_equals "${CACHE_PRUNE};${RESTART_DOCKER};docker info;${CACHE_PRUNE};" "${order}" "order of prune, restart, readiness check, prune"
}

it_should_not_restart_docker_while_a_job_is_running() {
    # Arrange
    local fixture_root status
    TEST_DISK_USAGE_SEQUENCE="75" fixture_root=$(create_fixture "job-running")

    # Act
    TEST_JOB_RUNNING=1 run_script "${fixture_root}"

    # Assert
    assert_equals 0 "${status}" "exit status"
    assert_command_not_logged "${fixture_root}" "${RESTART_DOCKER}"
    assert_equals 1 "$(count_commands "${fixture_root}" "${CACHE_PRUNE}")" "number of build cache prunes"
}

it_should_fail_when_usage_stays_above_the_failure_threshold_after_the_restart() {
    # Arrange
    local fixture_root status
    TEST_DISK_USAGE_SEQUENCE="90 88" fixture_root=$(create_fixture "still-full")

    # Act
    run_script "${fixture_root}"

    # Assert
    assert_equals 1 "${status}" "exit status"
    assert_command_logged "${fixture_root}" "${RESTART_DOCKER}"
}

it_should_fail_when_usage_is_above_the_failure_threshold_and_a_job_prevents_the_restart() {
    # Arrange
    local fixture_root status
    TEST_DISK_USAGE_SEQUENCE="90" fixture_root=$(create_fixture "full-and-busy")

    # Act
    TEST_JOB_RUNNING=1 run_script "${fixture_root}"

    # Assert
    assert_equals 1 "${status}" "exit status"
    assert_command_not_logged "${fixture_root}" "${RESTART_DOCKER}"
}

it_should_fail_without_pruning_again_when_docker_does_not_come_back_after_the_restart() {
    # Arrange
    local fixture_root status
    TEST_DISK_USAGE_SEQUENCE="75 40" fixture_root=$(create_fixture "docker-down")

    # Act
    TEST_DOCKER_DOWN=1 run_script "${fixture_root}"

    # Assert
    assert_equals 1 "${status}" "exit status"
    assert_equals 1 "$(count_commands "${fixture_root}" "${CACHE_PRUNE}")" "number of build cache prunes"
}

main() {
    local failures=0
    local test_name

    for test_name in $(declare -F | awk '{print $3}' | grep '^it_should_'); do
        if ("${test_name}"); then
            printf 'PASS %s\n' "${test_name}"
        else
            printf 'FAIL %s\n' "${test_name}"
            failures=$((failures + 1))
        fi
    done

    if [ "${failures}" -gt 0 ]; then
        printf '%s test(s) failed.\n' "${failures}" >&2
        exit 1
    fi
}

main "$@"
