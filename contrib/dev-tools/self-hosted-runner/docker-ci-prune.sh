#!/usr/bin/env bash
# Daily cleanup of Docker state on the self-hosted CI runner (see docs/self-hosted-runner.md).
#
# Prunes the Docker build cache and stale Docker objects. Runs as root from docker-ci-prune.service.

set -euo pipefail

CACHE_LIMIT=${DOCKER_CI_PRUNE_CACHE_LIMIT:-120GB}

log() {
    printf '%s\n' "$*"
}

prune_docker_storage() {
    docker builder prune --force --max-used-space "${CACHE_LIMIT}"
    docker container prune --force --filter until=24h
    docker image prune --force --filter until=168h
    docker volume prune --force
}

main() {
    prune_docker_storage
}

main "$@"
