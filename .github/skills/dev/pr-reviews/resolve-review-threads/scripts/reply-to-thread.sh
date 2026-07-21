#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'EOF'
Usage: reply-to-thread.sh --thread-id <id> (--body <text> | --body-file <path>)

Post a reply comment on a pull-request review thread.

Options:
  --thread-id <id>      Node ID of the review thread (e.g. PRRT_kwDOxxx) (required)
  --body <text>         Reply body text (required unless --body-file is given)
  --body-file <path>    Read reply body from file instead of --body
  -h, --help            Show this help

Output:
  - JSON line to stdout: {"status":"ok","thread_id":"...","reply_url":"..."}
  - Diagnostics to stderr
EOF
}

THREAD_ID=""
BODY=""
BODY_FILE=""

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

printf '{"status":"ok","thread_id":"%s","reply_url":"%s"}\n' "${THREAD_ID}" "${REPLY_URL}"
