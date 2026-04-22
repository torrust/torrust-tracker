#!/usr/bin/env bash
# Install project Git hooks from .githooks/ into .git/hooks/.
#
# Usage:
#   ./contrib/dev-tools/git/install-git-hooks.sh
#
# Run once after cloning the repository. Re-run to update hooks after
# they change.

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel)"
HOOKS_SRC="${REPO_ROOT}/.githooks"
HOOKS_DST="$(git rev-parse --git-path hooks)"
mkdir -p "${HOOKS_DST}"

if [ ! -d "${HOOKS_SRC}" ]; then
    echo "ERROR: .githooks/ directory not found at ${HOOKS_SRC}"
    exit 1
fi

installed=0

for hook in "${HOOKS_SRC}"/*; do
    hook_name="$(basename "${hook}")"
    dest="${HOOKS_DST}/${hook_name}"

    cp "${hook}" "${dest}"
    chmod +x "${dest}"

    echo "Installed: ${hook_name} → .git/hooks/${hook_name}"
    installed=$((installed + 1))
done

echo ""
echo "=========================================="
echo "SUCCESS: ${installed} hook(s) installed."
echo "=========================================="
