#!/usr/bin/env bash
# Check whether project Git hooks from .githooks/ are installed in .git/hooks/.
#
# Usage:
#   ./contrib/dev-tools/git/check-git-hooks.sh
#
# Exits 0 if all hooks are installed and executable.
# Exits 1 if any hook is missing or not executable.
#
# Run after cloning or whenever you want to verify your hook installation.

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
HOOKS_SRC="${REPO_ROOT}/.githooks"
HOOKS_DST="$(git rev-parse --git-path hooks)"

if [ ! -d "${HOOKS_SRC}" ]; then
    echo "ERROR: .githooks/ directory not found at ${HOOKS_SRC}"
    exit 1
fi

all_installed=true

for hook in "${HOOKS_SRC}"/*; do
    hook_name="$(basename "${hook}")"
    dest="${HOOKS_DST}/${hook_name}"

    if [[ -x "${dest}" ]]; then
        echo "installed:     ${hook_name}"
    else
        echo "NOT installed: ${hook_name}"
        all_installed=false
    fi
done

echo ""

if [[ "${all_installed}" == "true" ]]; then
    echo "=========================================="
    echo "SUCCESS: All hooks are installed."
    echo "=========================================="
    exit 0
else
    echo "=========================================="
    echo "FAILURE: Some hooks are missing."
    echo "Run: ./contrib/dev-tools/git/install-git-hooks.sh"
    echo "=========================================="
    exit 1
fi
