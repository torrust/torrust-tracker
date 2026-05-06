GitHub API helper scripts for PR review management.

## Scripts

**get-pr-review-threads.sh**
Fetches all review threads for a PR and saves to a JSON file.
Usage: ./get-pr-review-threads.sh [PR_NUMBER] [OUTPUT_FILE]
Default PR: 1733, Default output: /tmp/pr*threads*${PR_NUMBER}.json

**list-unresolved-threads.sh**
Filters and displays all unresolved threads from the fetched threads JSON file.
Usage: ./list-unresolved-threads.sh [THREADS_FILE]
Default: /tmp/pr_threads_1733.json

**resolve-all-unresolved-threads.sh**
Resolves all unresolved threads in a PR via GitHub GraphQL API.
Usage: ./resolve-all-unresolved-threads.sh [THREADS_FILE]
Default: /tmp/pr_threads_1733.json
