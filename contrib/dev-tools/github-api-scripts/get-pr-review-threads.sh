#!/bin/bash
PR_NUMBER=${1:-1733}
OUTPUT_FILE=${2:-/tmp/pr_threads_${PR_NUMBER}.json}

gh api graphql -f query='query { repository(owner:"torrust", name:"torrust-tracker") { pullRequest(number:'"$PR_NUMBER"') { reviewThreads(first:100) { nodes { id isResolved isOutdated path comments(first:1){nodes{url body author{login} createdAt}} } } } } }' > "$OUTPUT_FILE"

echo "Review threads saved to $OUTPUT_FILE"
