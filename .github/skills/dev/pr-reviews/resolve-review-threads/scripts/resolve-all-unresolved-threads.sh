#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'EOF'
Usage: resolve-all-unresolved-threads.sh --threads-file <path> [--dry-run]

Resolve all unresolved review threads from a fetched threads JSON file.

Options:
  --threads-file <path>   Path to review threads JSON file (required)
  --dry-run               Print thread IDs that would be resolved without mutating GitHub state
  -h, --help              Show this help

Output:
  - JSON lines to stdout describing each action/result
  - Diagnostics to stderr
EOF
}

THREADS_FILE=""
DRY_RUN="false"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --threads-file)
            THREADS_FILE=${2:-}
            shift 2
            ;;
        --dry-run)
            DRY_RUN="true"
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Error: unknown argument '$1'." >&2
            usage >&2
            exit 2
            ;;
    esac
done

if [[ -z "${THREADS_FILE}" ]]; then
    echo "Error: --threads-file is required." >&2
    usage >&2
    exit 2
fi

mapfile -t THREAD_IDS < <(jq -r '.data.repository.pullRequest.reviewThreads.nodes[]
  | select(.isResolved == false)
  | .id' "${THREADS_FILE}")

if [[ ${#THREAD_IDS[@]} -eq 0 ]]; then
    echo '{"status":"ok","message":"no unresolved threads"}'
    exit 0
fi

for thread_id in "${THREAD_IDS[@]}"; do
    if [[ "${DRY_RUN}" == "true" ]]; then
        printf '{"status":"dry-run","thread_id":"%s"}\n' "${thread_id}"
        continue
    fi

    # shellcheck disable=SC2016
    gh api graphql \
      -F threadId="${thread_id}" \
      -f query='mutation($threadId: ID!) {
        resolveReviewThread(input: { threadId: $threadId }) {
          thread {
            id
            isResolved
          }
        }
      }' >/dev/null

    printf '{"status":"resolved","thread_id":"%s"}\n' "${thread_id}"
done
