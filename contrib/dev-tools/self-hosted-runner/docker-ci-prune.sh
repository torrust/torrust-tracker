#!/usr/bin/env bash
# Daily cleanup of Docker state on the self-hosted CI runner (see docs/self-hosted-runner.md).
#
# Prunes the Docker build cache and stale Docker objects. Runs as root from docker-ci-prune.service.
#
# BuildKit inside dockerd can keep references to build state that no job owns (issue 2673); the prune
# then frees nothing and only a daemon restart releases them. So when the disk is still above
# RESTART_ABOVE_PERCENT after the prune, the script restarts docker.service and prunes again, but only
# while no job is running: with live restore disabled a restart stops running containers.
#
# The script exits non-zero when the disk stays above FAIL_ABOVE_PERCENT, so systemd marks the unit as
# failed and `systemctl --failed` shows it.

set -euo pipefail

CACHE_LIMIT=${DOCKER_CI_PRUNE_CACHE_LIMIT:-120GB}
DISK_PATH=${DOCKER_CI_PRUNE_DISK_PATH:-/}
RESTART_ABOVE_PERCENT=${DOCKER_CI_PRUNE_RESTART_ABOVE_PERCENT:-70}
FAIL_ABOVE_PERCENT=${DOCKER_CI_PRUNE_FAIL_ABOVE_PERCENT:-85}
READY_ATTEMPTS=${DOCKER_CI_PRUNE_READY_ATTEMPTS:-30}
READY_INTERVAL_SECONDS=${DOCKER_CI_PRUNE_READY_INTERVAL_SECONDS:-2}

log() {
    printf '%s\n' "$*"
}

prune_docker_storage() {
    docker builder prune --force --max-used-space "${CACHE_LIMIT}"
    docker container prune --force --filter until=24h
    docker image prune --force --filter until=168h
    docker volume prune --force
}

disk_usage_percent() {
    df --output=pcent "${DISK_PATH}" | tail -n 1 | tr -dc '0-9'
}

# The runner agent starts one Runner.Worker process per job.
job_is_running() {
    pgrep -x Runner.Worker >/dev/null
}

wait_for_docker() {
    local attempt

    for ((attempt = 1; attempt <= READY_ATTEMPTS; attempt++)); do
        if docker info >/dev/null 2>&1; then
            return 0
        fi
        sleep "${READY_INTERVAL_SECONDS}"
    done

    log "ERROR: Docker did not answer after ${READY_ATTEMPTS} attempts following the restart."
    return 1
}

restart_docker() {
    log "Disk usage is still above ${RESTART_ABOVE_PERCENT}% after the prune; restarting docker.service to release orphaned build state."
    systemctl restart docker.service
    wait_for_docker
}

main() {
    local usage

    prune_docker_storage
    usage=$(disk_usage_percent)
    log "Disk usage after the prune: ${usage}%."

    if [ "${usage}" -ge "${RESTART_ABOVE_PERCENT}" ]; then
        if job_is_running; then
            log "Skipping the docker.service restart: a job is running. The next run tries again."
        else
            restart_docker
            prune_docker_storage
            usage=$(disk_usage_percent)
            log "Disk usage after the restart and a second prune: ${usage}%."
        fi
    fi

    if [ "${usage}" -ge "${FAIL_ABOVE_PERCENT}" ]; then
        log "ERROR: Disk usage is ${usage}%, at or above the ${FAIL_ABOVE_PERCENT}% failure threshold."
        return 1
    fi
}

main "$@"
