#!/usr/bin/env bash
# Lint the Containerfile with hadolint.
#
# This sensor is a standalone check: it can be triggered by any orchestrator
# (pre-commit hook, CI, Copilot file hooks, manual invocation). It only runs
# hadolint when the Containerfile has been staged for commit (git diff check).
# See EPIC #2003 for the long-term harness/sensor architecture design.
#
# Usage:
#   ./contrib/dev-tools/checks/lint-containerfile.sh

set -uo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
PROJECT_ROOT=$(cd -- "${SCRIPT_DIR}/../../.." && pwd)
CONTAINERFILE="${PROJECT_ROOT}/Containerfile"
CONFIG="${PROJECT_ROOT}/.hadolint.yaml"
HADOLINT_IMAGE="hadolint/hadolint@sha256:27086352fd5e1907ea2b934eb1023f217c5ae087992eb59fde121dce9c9ff21e"

# Skip if Containerfile wasn't changed (staged)
if ! git diff --cached --name-only --diff-filter=ACM | grep -q '^Containerfile$'; then
    echo "Containerfile unchanged, skipping hadolint"
    exit 0
fi

if [[ ! -f "${CONTAINERFILE}" ]]; then
    echo "Error: Containerfile not found at '${CONTAINERFILE}'." >&2
    exit 2
fi

if [[ ! -f "${CONFIG}" ]]; then
    echo "Warning: hadolint config '${CONFIG}' not found, running without." >&2
    docker run --rm -i --entrypoint hadolint "${HADOLINT_IMAGE}" - < "${CONTAINERFILE}"
    exit $?
fi

docker run --rm -i \
    -v "${CONFIG}:/.hadolint.yaml" \
    --entrypoint hadolint \
    "${HADOLINT_IMAGE}" \
    --config /.hadolint.yaml \
    - < "${CONTAINERFILE}"
