#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'EOF'
Usage: check-thread-reply-status.sh --threads-file <path> [--login <username>]

For each unresolved review thread, report whether the given user (or the current
authenticated GitHub user) has already posted a reply.

Use this before running resolve-all-unresolved-threads.sh to confirm that every
thread has a reply. Threads without a reply should be handled with
reply-and-resolve-thread.sh instead of the bulk resolver.

Options:
  --threads-file <path>   Path to review threads JSON file (required)
  --login <username>      GitHub login to check for replies (default: current gh user)
  -h, --help              Show this help

Output:
  - JSON lines to stdout, one per unresolved thread:
      {"thread_id":"...","path":"...","url":"...","has_reply":true|false}
  - Summary line at the end:
      {"summary":true,"total":N,"with_reply":N,"without_reply":N}
  - Diagnostics to stderr
EOF
}

THREADS_FILE=""
LOGIN=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --threads-file)
            THREADS_FILE=${2:-}
            shift 2
            ;;
        --login)
            LOGIN=${2:-}
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

if [[ -z "${THREADS_FILE}" ]]; then
    echo "Error: --threads-file is required." >&2
    usage >&2
    exit 2
fi

if [[ -z "${LOGIN}" ]]; then
    LOGIN=$(gh api /user --jq .login)
    echo "Using current GitHub user: ${LOGIN}" >&2
fi

total=0
with_reply=0
without_reply=0

while IFS= read -r thread_json; do
    thread_id=$(echo "${thread_json}" | jq -r '.id')
    path=$(echo "${thread_json}" | jq -r '.path')
    has_reply=$(echo "${thread_json}" | jq --arg login "${LOGIN}" '
        .comments.nodes
        | map(select(.author.login == $login))
        | length > 0
    ')

    url_json=$(echo "${thread_json}" | jq '.url')
    jq -n \
        --arg thread_id "${thread_id}" \
        --arg path "${path}" \
        --argjson url "${url_json}" \
        --argjson has_reply "${has_reply}" \
        '{"thread_id":$thread_id,"path":$path,"url":$url,"has_reply":$has_reply}'

    total=$((total + 1))
    if [[ "${has_reply}" == "true" ]]; then
        with_reply=$((with_reply + 1))
    else
        without_reply=$((without_reply + 1))
        echo "  ⚠ No reply yet on thread ${thread_id} (${path})" >&2
    fi
done < <(jq -c '.data.repository.pullRequest.reviewThreads.nodes[]
    | select(.isResolved == false)
    | {
        id,
        path,
        url: (.comments.nodes[0].url // null),
        comments
    }' "${THREADS_FILE}")

printf '{"summary":true,"total":%d,"with_reply":%d,"without_reply":%d}\n' \
    "${total}" "${with_reply}" "${without_reply}"

if [[ "${without_reply}" -gt 0 ]]; then
    echo "Error: ${without_reply} thread(s) have no reply. Use reply-and-resolve-thread.sh before bulk-resolving." >&2
    exit 1
fi
