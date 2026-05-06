#!/bin/bash
THREADS_FILE=${1:-/tmp/pr_threads_1733.json}

jq -r '.data.repository.pullRequest.reviewThreads.nodes[] | select(.isResolved==false) | .id' "$THREADS_FILE" | while read -r id; do
  gh api graphql -f query="mutation(\$id:ID!){resolveReviewThread(input:{threadId:\$id}){thread{id isResolved}}}" -F id="$id" && echo "resolved: $id"
done
