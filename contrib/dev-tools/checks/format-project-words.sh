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

if ! temporary_dictionary=$(mktemp "${DICTIONARY_PATH}.XXXXXX"); then
    printf 'Error: failed to create a temporary project dictionary: %s\n' "${DICTIONARY_PATH}" >&2
    exit 2
fi

trap 'rm -f "${temporary_dictionary}"' EXIT

if ! cp -p "${DICTIONARY_PATH}" "${temporary_dictionary}"; then
    printf 'Error: failed to preserve project dictionary metadata: %s\n' "${DICTIONARY_PATH}" >&2
    exit 2
fi

if ! LC_ALL=C sort -u "${DICTIONARY_PATH}" >"${temporary_dictionary}"; then
    printf 'Error: failed to format project dictionary: %s\n' "${DICTIONARY_PATH}" >&2
    exit 2
fi

if cmp -s "${DICTIONARY_PATH}" "${temporary_dictionary}"; then
    printf 'project-words.txt is already formatted.\n'
    exit 0
fi

if ! mv "${temporary_dictionary}" "${DICTIONARY_PATH}"; then
    printf 'Error: failed to update project dictionary: %s\n' "${DICTIONARY_PATH}" >&2
    exit 2
fi

printf 'Formatted project-words.txt with LC_ALL=C sort -u.\n'
printf "Stage 'project-words.txt' and retry the commit.\n"
exit 1
