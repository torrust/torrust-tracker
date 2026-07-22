#!/usr/bin/env bash
# Format the repository cspell dictionary with deterministic ordering and exact de-duplication.

set -uo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
PROJECT_ROOT=$(cd -- "${SCRIPT_DIR}/../../.." && pwd)
DICTIONARY_PATH="${PROJECT_ROOT}/project-words.txt"

if [[ ! -f "${DICTIONARY_PATH}" ]]; then
    printf 'Error: project dictionary not found: %s\n' "${DICTIONARY_PATH}" >&2
    exit 2
fi

temporary_dictionary=$(mktemp)
trap 'rm -f "${temporary_dictionary}"' EXIT

if ! LC_ALL=C sort --unique "${DICTIONARY_PATH}" >"${temporary_dictionary}"; then
    printf 'Error: failed to format project dictionary: %s\n' "${DICTIONARY_PATH}" >&2
    exit 2
fi

if cmp --silent "${DICTIONARY_PATH}" "${temporary_dictionary}"; then
    printf 'project-words.txt is already formatted.\n'
    exit 0
fi

if ! cat "${temporary_dictionary}" >"${DICTIONARY_PATH}"; then
    printf 'Error: failed to update project dictionary: %s\n' "${DICTIONARY_PATH}" >&2
    exit 2
fi

printf 'Formatted project-words.txt with LC_ALL=C sort --unique.\n'
printf "Stage 'project-words.txt' and retry the commit.\n"
exit 1