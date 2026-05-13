#!/usr/bin/env bash

set -euo pipefail

usage() {
		cat <<'EOF'
Usage: get-pr-review-threads.sh --pr-number <number> [--output-file <path>] [--owner <owner>] [--repo <repo>]

Fetch pull-request review threads and write full JSON response to an output file.

Options:
	--pr-number <number>   Pull request number (required)
	--output-file <path>   Output JSON file (default: /tmp/pr_threads_<PR_NUMBER>.json)
	--owner <owner>        Repository owner (default: torrust)
	--repo <repo>          Repository name (default: torrust-tracker)
	-h, --help             Show this help

Output:
	- Writes GraphQL response JSON to --output-file
	- Writes a small summary JSON object to stdout
	- Writes diagnostics to stderr
EOF
}

OWNER="torrust"
REPO="torrust-tracker"
PR_NUMBER=""
OUTPUT_FILE=""

while [[ $# -gt 0 ]]; do
		case "$1" in
				--pr-number)
						PR_NUMBER=${2:-}
						shift 2
						;;
				--output-file)
						OUTPUT_FILE=${2:-}
						shift 2
						;;
				--owner)
						OWNER=${2:-}
						shift 2
						;;
				--repo)
						REPO=${2:-}
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

if [[ -z "${PR_NUMBER}" ]]; then
		echo "Error: --pr-number is required." >&2
		usage >&2
		exit 2
fi

if [[ -z "${OUTPUT_FILE}" ]]; then
		OUTPUT_FILE="/tmp/pr_threads_${PR_NUMBER}.json"
fi

echo "Fetching review threads for ${OWNER}/${REPO} PR #${PR_NUMBER}..." >&2
# shellcheck disable=SC2016
gh api graphql \
	-F owner="${OWNER}" \
	-F repo="${REPO}" \
	-F pullNumber="${PR_NUMBER}" \
	-f query='query($owner: String!, $repo: String!, $pullNumber: Int!) {
		repository(owner: $owner, name: $repo) {
			pullRequest(number: $pullNumber) {
				reviewThreads(first: 100) {
					nodes {
						id
						isResolved
						isOutdated
						path
						isCollapsed
						comments(first: 20) {
							nodes {
								url
								body
								createdAt
								author {
									login
								}
							}
						}
					}
				}
			}
		}
	}' > "${OUTPUT_FILE}"

printf '{"status":"ok","pr_number":%s,"output_file":"%s"}\n' "${PR_NUMBER}" "${OUTPUT_FILE}"
