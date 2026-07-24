#!/usr/bin/env bash
# Lint the Containerfile with hadolint.
#
# Tests: (no automated tests yet — EPIC #2003)
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

# Skip if Containerfile wasn't changed (staged).
# Use a separate check so that a non-zero exit from `git diff` (e.g. running
# outside a git work tree) is not silently swallowed by `!`.
if git diff --cached --name-only --diff-filter=ACM 2>/dev/null | grep -q '^Containerfile$'; then
    : # Containerfile is staged — proceed
elif [[ $? -eq 1 ]]; then
    # grep exited 1: Containerfile not found in staged changes
    echo "Containerfile unchanged, skipping hadolint"
    exit 0
else
    # git diff or grep failed (e.g. not a git repository)
    echo "Error: cannot check staged changes (not a git repository?)." >&2
    exit 2
fi

# Lint the staged version of the Containerfile to avoid false positives
# from unstaged working-tree changes. This ensures the sensor checks exactly
# what will be committed, not the current working tree.
# Use `git show` piped directly to avoid shell mangling from `echo`.
if [[ ! -f "${CONFIG}" ]]; then
    echo "Warning: hadolint config '${CONFIG}' not found, running without." >&2
    git show :./"${CONTAINERFILE##*/}" 2>/dev/null | docker run --rm -i --entrypoint hadolint "${HADOLINT_IMAGE}" -
    exit $?
fi

git show :./"${CONTAINERFILE##*/}" 2>/dev/null | docker run --rm -i \
    -v "${CONFIG}:/.hadolint.yaml" \
    --entrypoint hadolint \
    "${HADOLINT_IMAGE}" \
    --config /.hadolint.yaml \
    -

# Capture the exit code from the pipeline (last command: hadolint)
exit "${PIPESTATUS[0]}"
