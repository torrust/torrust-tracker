#!/bin/bash
THREADS_FILE=${1:-/tmp/pr_threads_1733.json}

jq '.data.repository.pullRequest.reviewThreads.nodes[] | select(.isResolved==false) | {id, isOutdated, path, url: .comments.nodes[0].url}' "$THREADS_FILE"
