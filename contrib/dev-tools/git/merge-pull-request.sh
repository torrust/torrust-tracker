#!/usr/bin/env bash
# Repository-local entry point for the vendored GitHub pull-request merge tool.
#
# The wrapped tool intentionally remains interactive for merge inspection, signing, and pushing.
# This wrapper only validates Torrust Tracker's non-destructive preconditions and fixes the
# upstream repository and target branch. See .github/skills/dev/git-workflow/merge-pull-request/SKILL.md.

set -euo pipefail

readonly EXPECTED_REPOSITORY="torrust/torrust-tracker"
readonly TARGET_BRANCH="develop"
SCRIPT_DIRECTORY="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
readonly SCRIPT_DIRECTORY
readonly VENDORED_TOOL="${SCRIPT_DIRECTORY}/github-merge.py"

print_usage() {
    cat >&2 <<'EOF'
Usage: ./contrib/dev-tools/git/merge-pull-request.sh [--dry-run] PULL_REQUEST

Validate the local maintainer merge-workflow prerequisites, then invoke the vendored merge tool
for torrust/torrust-tracker targeting develop.

Options:
  --dry-run  Validate only. Do not access GitHub, create temporary branches, merge, sign, or push.
  -h, --help Show this help.
EOF
}

require_clean_working_tree() {
    if [[ -n "$(git status --porcelain)" ]]; then
        echo "ERROR: Working tree is not clean; preserve or stash unrelated work before merging." >&2
        exit 1
    fi
}

require_repository_configuration() {
    local repository
    repository=$(git config --get githubmerge.repository || true)

    if [[ "${repository}" != "${EXPECTED_REPOSITORY}" ]]; then
        echo "ERROR: githubmerge.repository must be '${EXPECTED_REPOSITORY}'." >&2
        exit 1
    fi
}

require_target_branch() {
    local current_branch
    current_branch=$(git branch --show-current)

    if [[ "${current_branch}" != "${TARGET_BRANCH}" ]]; then
        echo "ERROR: Run this workflow from the '${TARGET_BRANCH}' branch; current branch is '${current_branch:-detached HEAD}'." >&2
        exit 1
    fi
}

require_signing_key() {
    if ! git config --get user.signingkey >/dev/null; then
        echo "ERROR: Configure user.signingkey before starting a merge attempt." >&2
        exit 1
    fi
}

main() {
    local dry_run=false

    case "${1:-}" in
        --dry-run)
            dry_run=true
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
    esac

    if [[ $# -ne 1 || ! "${1}" =~ ^[1-9][0-9]*$ ]]; then
        echo "ERROR: PULL_REQUEST must be a positive integer." >&2
        print_usage
        exit 2
    fi

    local pull_request=$1

    if ! git rev-parse --show-toplevel >/dev/null 2>&1; then
        echo "ERROR: Run this command inside a Git working tree." >&2
        exit 1
    fi

    require_clean_working_tree
    require_repository_configuration
    require_target_branch
    require_signing_key

    if [[ "${dry_run}" == true ]]; then
        printf 'Dry-run preflight passed for %s PR %s targeting %s.\n' "${EXPECTED_REPOSITORY}" "${pull_request}" "${TARGET_BRANCH}"
        exit 0
    fi

    exec python3 "${VENDORED_TOOL}" "${pull_request}" "${TARGET_BRANCH}"
}

main "$@"