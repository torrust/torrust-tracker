#!/usr/bin/env bash

set -euo pipefail

usage() {
	cat <<'EOF'
Usage: show-unresolved-thread-bodies.sh --threads-file <path>

Print the full details of each unresolved review thread, including comment bodies.
Use this after running get-pr-review-threads.sh to read Copilot (or other reviewer)
suggestions before triaging them.

Options:
  --threads-file <path>   Path to review threads JSON file written by
                          get-pr-review-threads.sh (required)
  -h, --help              Show this help

Output format (human-readable):
  === Thread <id> ===
  Path:     <file path>
  Outdated: <true|false>
  URL:      <comment url>
  Author:   <login>
  Body:
  <comment body>
  ---
EOF
}

THREADS_FILE=""

while [[ $# -gt 0 ]]; do
	case "$1" in
		--threads-file)
			THREADS_FILE=${2:-}
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

if [[ ! -f "${THREADS_FILE}" ]]; then
	echo "Error: file not found: ${THREADS_FILE}" >&2
	exit 2
fi

jq -r '
  .data.repository.pullRequest.reviewThreads.nodes[]
  | select(.isResolved == false)
  | "=== Thread \(.id) ===",
    "Path:     \(.path)",
    "Outdated: \(.isOutdated)",
    (.comments.nodes[]
      | "URL:      \(.url)",
        "Author:   \(.author.login)",
        "Body:",
        .body,
        "---"
    )
' "${THREADS_FILE}"
