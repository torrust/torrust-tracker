#!/usr/bin/env bash
# Deterministic integration tests for the repository-local merge workflow wrapper.

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
TEST_DIRECTORY=$(mktemp -d "${TMPDIR:-/tmp}/test-merge-pull-request.XXXXXX")
trap 'rm -rf "${TEST_DIRECTORY}"' EXIT

create_fixture() {
    local fixture_name=$1
    local fixture_root="${TEST_DIRECTORY}/${fixture_name}"

    mkdir -p "${fixture_root}/contrib/dev-tools/git"
    cp "${PROJECT_ROOT}/contrib/dev-tools/git/merge-pull-request.sh" "${fixture_root}/contrib/dev-tools/git/"
    cp "${PROJECT_ROOT}/contrib/dev-tools/git/github-merge.py" "${fixture_root}/contrib/dev-tools/git/"
    chmod +x "${fixture_root}/contrib/dev-tools/git/merge-pull-request.sh"

    (
        cd "${fixture_root}"
        git init --quiet --initial-branch=develop
        git config user.name "Merge workflow test"
        git config user.email "merge-workflow-test@example.com"
        printf 'fixture\n' >README.md
        git add .
        git commit --quiet -m 'Initial fixture'
        git config githubmerge.repository torrust/torrust-tracker
        git config githubmerge.branch develop
        git config user.signingkey 0123456789ABCDEF
    )

    printf '%s\n' "${fixture_root}"
}

it_should_pass_deterministic_preflight_when_repository_state_is_supported() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "valid-preflight")
    local output_file="${TEST_DIRECTORY}/valid-preflight-output.txt"

    # Act
    (
        cd "${fixture_root}"
        ./contrib/dev-tools/git/merge-pull-request.sh --dry-run 2022 >"${output_file}"
    )

    # Assert
    grep -F -q 'Dry-run preflight passed for torrust/torrust-tracker PR 2022 targeting develop.' "${output_file}"
}

it_should_refuse_a_dirty_working_tree_without_invoking_the_vendored_tool() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "dirty-working-tree")
    printf 'unrelated work\n' >"${fixture_root}/unrelated.txt"
    local output_file="${TEST_DIRECTORY}/dirty-working-tree-output.txt"

    # Act
    if (
        cd "${fixture_root}"
        ./contrib/dev-tools/git/merge-pull-request.sh --dry-run 2022 >"${output_file}" 2>&1
    ); then
        printf 'Expected dirty-worktree preflight to fail.\n' >&2
        return 1
    fi

    # Assert
    grep -F -q 'ERROR: Working tree is not clean; preserve or stash unrelated work before merging.' "${output_file}"
    [[ -f "${fixture_root}/unrelated.txt" ]]
}

it_should_refuse_a_repository_configuration_that_is_not_the_upstream_tracker() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "wrong-repository")
    (
        cd "${fixture_root}"
        git config githubmerge.repository example/other-repository
    )
    local output_file="${TEST_DIRECTORY}/wrong-repository-output.txt"

    # Act
    if (
        cd "${fixture_root}"
        ./contrib/dev-tools/git/merge-pull-request.sh --dry-run 2022 >"${output_file}" 2>&1
    ); then
        printf 'Expected repository preflight to fail.\n' >&2
        return 1
    fi

    # Assert
    grep -F -q "ERROR: githubmerge.repository must be 'torrust/torrust-tracker'." "${output_file}"
}

it_should_invoke_the_vendored_tool_with_the_fixed_target_branch_after_preflight() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "vendored-tool-invocation")
    local stub_directory="${TEST_DIRECTORY}/vendored-tool-bin"
    mkdir -p "${stub_directory}"
    cat >"${stub_directory}/python3" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >"${TEST_PYTHON_ARGUMENTS}"
EOF
    chmod +x "${stub_directory}/python3"

    # Act
    (
        cd "${fixture_root}"
        PATH="${stub_directory}:${PATH}" \
            TEST_PYTHON_ARGUMENTS="${fixture_root}/python-arguments.txt" \
            ./contrib/dev-tools/git/merge-pull-request.sh 2022
    )

    # Assert
    grep -F -q 'contrib/dev-tools/git/github-merge.py 2022 develop' "${fixture_root}/python-arguments.txt"
}

it_should_reject_a_non_positive_pull_request_number_before_performing_work() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "invalid-pull-request")
    local output_file="${TEST_DIRECTORY}/invalid-pull-request-output.txt"

    # Act
    if (
        cd "${fixture_root}"
        ./contrib/dev-tools/git/merge-pull-request.sh --dry-run 0 >"${output_file}" 2>&1
    ); then
        printf 'Expected invalid pull request input to fail.\n' >&2
        return 1
    fi

    # Assert
    grep -F -q 'ERROR: PULL_REQUEST must be a positive integer.' "${output_file}"
}

it_should_pass_deterministic_preflight_when_repository_state_is_supported
it_should_refuse_a_dirty_working_tree_without_invoking_the_vendored_tool
it_should_refuse_a_repository_configuration_that_is_not_the_upstream_tracker
it_should_invoke_the_vendored_tool_with_the_fixed_target_branch_after_preflight
it_should_reject_a_non_positive_pull_request_number_before_performing_work

printf 'All merge workflow wrapper tests passed.\n'