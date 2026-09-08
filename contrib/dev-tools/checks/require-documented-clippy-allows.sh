#!/usr/bin/env bash
# Enforces rationale comments for newly added or modified Clippy suppressions.
# Tests: tests/test-require-documented-clippy-allows.sh
#
# Usage:
#   CLIPPY_ALLOW_BASE_REF=<remote>/<branch> ./contrib/dev-tools/checks/require-documented-clippy-allows.sh

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
BASE_REF=${CLIPPY_ALLOW_BASE_REF:-}
ISSUE_REFERENCE_REGEX='#[0-9]+'
REMOVAL_CONDITION_REGEX='remove[[:space:]](when|after|by)[[:space:]]+[^[:space:]]+'
UNTIL_CONDITION_REGEX='until[[:space:]]+[^[:space:]]+'
RATIONALE_REGEX='^[[:space:]]*//[[:space:]]clippy-allow:[[:space:]](intentional|false-positive|temporary):[[:space:]]+.+'

resolve_base_ref() {
    local candidate

    if [[ -n "${BASE_REF}" ]]; then
        printf '%s\n' "${BASE_REF}"
        return
    fi

    for candidate in origin/develop torrust/develop develop; do
        if git -C "${PROJECT_ROOT}" rev-parse --verify --quiet "${candidate}" >/dev/null; then
            printf '%s\n' "${candidate}"
            return
        fi
    done

    printf 'Error: could not find a develop reference. Set CLIPPY_ALLOW_BASE_REF to the PR base reference.\n' >&2
    return 2
}

require_rationale() {
    local file_path=$1
    local line_number=$2
    local attribute_line rationale_line=""

    attribute_line=$(sed -n "${line_number}p" "${PROJECT_ROOT}/${file_path}")
    if [[ "${line_number}" -gt 1 ]]; then
        rationale_line=$(sed -n "$((line_number - 1))p" "${PROJECT_ROOT}/${file_path}")
    fi

    if [[ ! "${rationale_line}" =~ ${RATIONALE_REGEX} ]]; then
        printf '%s:%s: %s requires an adjacent // clippy-allow: <intentional|false-positive|temporary>: <rationale> comment.\n' \
            "${file_path}" \
            "${line_number}" \
            "${attribute_line}" >&2
        return 1
    fi

    if [[ "${rationale_line}" == *'clippy-allow: temporary:'* ]] && \
        [[ ! "${rationale_line}" =~ ${ISSUE_REFERENCE_REGEX} ]] && \
        [[ ! "${rationale_line}" =~ ${REMOVAL_CONDITION_REGEX} ]] && \
        [[ ! "${rationale_line}" =~ ${UNTIL_CONDITION_REGEX} ]]; then
        printf '%s:%s: temporary Clippy allows require a stable issue reference or explicit removal condition.\n' \
            "${file_path}" \
            "${line_number}" >&2
        return 1
    fi
}

base_ref=$(resolve_base_ref)
base_commit=$(git -C "${PROJECT_ROOT}" merge-base HEAD "${base_ref}")
failed=0

while IFS=: read -r file_path line_number; do
    if ! require_rationale "${file_path}" "${line_number}"; then
        failed=1
    fi
done < <(
    git -C "${PROJECT_ROOT}" diff --unified=0 "${base_commit}" -- '*.rs' |
        awk '
            /^\+\+\+ b\// { file_path = substr($0, 7) }
            /^@@/ {
                split($0, hunk, " ")
                split(hunk[3], range, ",")
                line_number = substr(range[1], 2)
            }
            /^\+[^+]/ {
                if ($0 ~ /^\+#!?\[allow\(clippy::/) {
                    printf "%s:%s\n", file_path, line_number
                }
                line_number++
            }
        '
)

if [[ "${failed}" -ne 0 ]]; then
    exit 1
fi

printf 'All newly added or modified Clippy allows have documented rationales.\n'