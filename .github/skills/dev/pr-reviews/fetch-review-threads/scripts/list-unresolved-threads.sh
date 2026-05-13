#!/usr/bin/env bash

set -euo pipefail

usage() {
	cat <<'EOF'
Usage: list-unresolved-threads.sh --threads-file <path>

List unresolved review threads as JSON lines.

Options:
  --threads-file <path>   Path to review threads JSON file (required)
  -h, --help              Show this help
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

jq -c '.data.repository.pullRequest.reviewThreads.nodes[]
  | select(.isResolved == false)
  | {
	  id,
	  isOutdated,
	  path,
	  url: (.comments.nodes[0].url // null)
	}' "${THREADS_FILE}"
