#!/usr/bin/env bash
# Integration tests for the prospective Clippy-allow rationale validator.
#
# Run manually after modifying the validator:
#   bash contrib/dev-tools/checks/tests/test-require-documented-clippy-allows.sh

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
TEST_DIRECTORY=$(mktemp -d "${TMPDIR:-/tmp}/test-require-documented-clippy-allows.XXXXXX")
trap 'rm -rf "${TEST_DIRECTORY}"' EXIT

create_fixture() {
    local fixture_name=$1
    local fixture_root="${TEST_DIRECTORY}/${fixture_name}"

    mkdir -p "${fixture_root}/contrib/dev-tools/checks" "${fixture_root}/src"
    cp "${PROJECT_ROOT}/contrib/dev-tools/checks/require-documented-clippy-allows.sh" \
        "${fixture_root}/contrib/dev-tools/checks/"
    chmod +x "${fixture_root}/contrib/dev-tools/checks/require-documented-clippy-allows.sh"
    git -C "${fixture_root}" init --quiet --initial-branch=develop
    git -C "${fixture_root}" config user.email "tests@example.com"
    git -C "${fixture_root}" config user.name "Validator tests"
    printf '%s\n' '#[allow(clippy::legacy)]' 'fn legacy() {}' >"${fixture_root}/src/lib.rs"
    git -C "${fixture_root}" add src/lib.rs
    git -C "${fixture_root}" commit --quiet -m 'test: establish legacy allowance'
    git -C "${fixture_root}" switch --quiet -c feature
    printf '%s\n' "${fixture_root}"
}

run_validator() {
    local fixture_root=$1

    CLIPPY_ALLOW_BASE_REF=develop \
        "${fixture_root}/contrib/dev-tools/checks/require-documented-clippy-allows.sh"
}

it_should_reject_an_undocumented_item_allowance() {
    local fixture_root
    fixture_root=$(create_fixture "undocumented-item")
    printf '%s\n' '#[allow(clippy::too_many_lines)]' 'fn new_item() {}' >>"${fixture_root}/src/lib.rs"

    if run_validator "${fixture_root}" >"${fixture_root}/output.txt" 2>&1; then
        printf 'Expected an undocumented item allowance to fail.\n' >&2
        return 1
    fi

    grep -F -q 'src/lib.rs:3: #[allow(clippy::too_many_lines)] requires an adjacent' "${fixture_root}/output.txt"
}

it_should_accept_a_documented_item_allowance() {
    local fixture_root
    fixture_root=$(create_fixture "documented-item")
    printf '%s\n' '// clippy-allow: intentional: The public wire schema uses these established names.' \
        '#[allow(clippy::struct_field_names)]' 'struct WireSchema { field_name: String }' >>"${fixture_root}/src/lib.rs"

    run_validator "${fixture_root}"
}

it_should_accept_a_documented_crate_allowance() {
    local fixture_root
    fixture_root=$(create_fixture "documented-crate")
    printf '%s\n' '// clippy-allow: false-positive: The generated compatibility module is intentionally named.' \
        '#![allow(clippy::module_name_repetitions)]' >>"${fixture_root}/src/lib.rs"

    run_validator "${fixture_root}"
}

it_should_require_a_removal_condition_for_temporary_allowances() {
    local fixture_root
    fixture_root=$(create_fixture "temporary-without-removal-condition")
    printf '%s\n' '// clippy-allow: temporary: The refactor has not reached this module yet.' \
        '#[allow(clippy::too_many_arguments)]' 'fn transitional_item() {}' >>"${fixture_root}/src/lib.rs"

    if run_validator "${fixture_root}" >"${fixture_root}/output.txt" 2>&1; then
        printf 'Expected a temporary allowance without a removal condition to fail.\n' >&2
        return 1
    fi

    grep -F -q 'temporary Clippy allows require a stable issue reference or explicit removal condition.' \
        "${fixture_root}/output.txt"
}

it_should_reject_an_incomplete_temporary_removal_condition() {
    local fixture_root
    fixture_root=$(create_fixture "temporary-with-incomplete-removal-condition")
    printf '%s\n' '// clippy-allow: temporary: remove when' \
        '#[allow(clippy::too_many_arguments)]' 'fn transitional_item() {}' >>"${fixture_root}/src/lib.rs"

    if run_validator "${fixture_root}" >"${fixture_root}/output.txt" 2>&1; then
        printf 'Expected an incomplete temporary removal condition to fail.\n' >&2
        return 1
    fi

    grep -F -q 'temporary Clippy allows require a stable issue reference or explicit removal condition.' \
        "${fixture_root}/output.txt"
}

it_should_accept_a_temporary_allowance_with_a_stable_issue_reference() {
    local fixture_root
    fixture_root=$(create_fixture "temporary-with-issue")
    printf '%s\n' '// clippy-allow: temporary: remove after #2158 inventories this legacy API.' \
        '#[allow(clippy::too_many_arguments)]' 'fn transitional_item() {}' >>"${fixture_root}/src/lib.rs"

    run_validator "${fixture_root}"
}

it_should_accept_a_temporary_allowance_with_an_explicit_removal_condition() {
    local fixture_root
    fixture_root=$(create_fixture "temporary-with-removal-condition")
    printf '%s\n' '// clippy-allow: temporary: remove when the refactor reaches this module.' \
        '#[allow(clippy::too_many_arguments)]' 'fn transitional_item() {}' >>"${fixture_root}/src/lib.rs"

    run_validator "${fixture_root}"
}

it_should_reject_a_modified_legacy_allowance_without_a_rationale() {
    local fixture_root
    fixture_root=$(create_fixture "modified-legacy")
    sed -i 's/legacy/changed_legacy/' "${fixture_root}/src/lib.rs"

    if run_validator "${fixture_root}" >"${fixture_root}/output.txt" 2>&1; then
        printf 'Expected a modified legacy allowance to fail.\n' >&2
        return 1
    fi

    grep -F -q 'src/lib.rs:1: #[allow(clippy::changed_legacy)] requires an adjacent' "${fixture_root}/output.txt"
}

it_should_reject_an_undocumented_item_allowance
it_should_accept_a_documented_item_allowance
it_should_accept_a_documented_crate_allowance
it_should_require_a_removal_condition_for_temporary_allowances
it_should_reject_an_incomplete_temporary_removal_condition
it_should_accept_a_temporary_allowance_with_a_stable_issue_reference
it_should_accept_a_temporary_allowance_with_an_explicit_removal_condition
it_should_reject_a_modified_legacy_allowance_without_a_rationale

printf 'All Clippy-allow rationale validator tests passed.\n'