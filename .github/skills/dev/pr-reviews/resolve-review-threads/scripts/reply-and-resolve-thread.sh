#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'EOF'
Usage: reply-and-resolve-thread.sh --thread-id <id> (--body <text> | --body-file <path>) [--dry-run]

Post a reply on a pull-request review thread and then resolve it.
The reply is always posted before the thread is resolved.

Options:
  --thread-id <id>      Node ID of the review thread (e.g. PRRT_kwDOxxx) (required)
  --body <text>         Reply body text (required unless --body-file is given)
  --body-file <path>    Read reply body from file instead of --body
  --dry-run             Print what would happen without posting or resolving
  -h, --help            Show this help

Output:
  - JSON line to stdout: {"status":"ok","thread_id":"...","reply_url":"...","resolved":true}
  - Diagnostics to stderr
EOF
}

THREAD_ID=""
BODY=""
BODY_FILE=""
DRY_RUN="false"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --thread-id)
            THREAD_ID=${2:-}
            shift 2
            ;;
        --body)
            BODY=${2:-}
            shift 2
            ;;
        --body-file)
            BODY_FILE=${2:-}
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

if [[ -z "${THREAD_ID}" ]]; then
    echo "Error: --thread-id is required." >&2
    usage >&2
    exit 2
fi

if [[ -n "${BODY_FILE}" ]]; then
    if [[ ! -f "${BODY_FILE}" ]]; then
        echo "Error: --body-file '${BODY_FILE}' does not exist." >&2
        exit 2
    fi
    BODY=$(cat "${BODY_FILE}")
fi

if [[ -z "${BODY}" ]]; then
    echo "Error: --body or --body-file is required." >&2
    usage >&2
    exit 2
fi

if [[ "${DRY_RUN}" == "true" ]]; then
    printf '{"status":"dry-run","thread_id":"%s","body_length":%d}\n' "${THREAD_ID}" "${#BODY}"
    exit 0
fi

echo "Posting reply to thread ${THREAD_ID}..." >&2

# shellcheck disable=SC2016
REPLY_URL=$(gh api graphql \
    -F threadId="${THREAD_ID}" \
    -F body="${BODY}" \
    -f query='mutation($threadId: ID!, $body: String!) {
      addPullRequestReviewThreadReply(input: {
        pullRequestReviewThreadId: $threadId
        body: $body
      }) {
        comment {
          url
        }
      }
    }' \
    --jq '.data.addPullRequestReviewThreadReply.comment.url')

echo "Resolving thread ${THREAD_ID}..." >&2

# shellcheck disable=SC2016
gh api graphql \
    -F threadId="${THREAD_ID}" \
    -f query='mutation($threadId: ID!) {
      resolveReviewThread(input: { threadId: $threadId }) {
        thread {
          id
          isResolved
        }
      }
    }' >/dev/null

printf '{"status":"ok","thread_id":"%s","reply_url":"%s","resolved":true}\n' "${THREAD_ID}" "${REPLY_URL}"
