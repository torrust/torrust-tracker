#!/usr/bin/env bash
# Integration tests for the project dictionary formatter and pre-commit orchestration.

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
TEST_DIRECTORY=$(mktemp -d "${TMPDIR:-/tmp}/test-format-project-words.XXXXXX")
trap 'rm -rf "${TEST_DIRECTORY}"' EXIT

create_fixture() {
    local fixture_name=$1
    local fixture_root="${TEST_DIRECTORY}/${fixture_name}"

    mkdir -p "${fixture_root}/contrib/dev-tools/git/hooks" "${fixture_root}/bin" "${fixture_root}/logs"
    cp "${PROJECT_ROOT}/contrib/dev-tools/git/format-project-words.sh" "${fixture_root}/contrib/dev-tools/git/"
    cp "${PROJECT_ROOT}/contrib/dev-tools/git/hooks/pre-commit.sh" "${fixture_root}/contrib/dev-tools/git/hooks/"
    chmod +x \
        "${fixture_root}/contrib/dev-tools/git/format-project-words.sh" \
        "${fixture_root}/contrib/dev-tools/git/hooks/pre-commit.sh"

    printf '%s\n' "${fixture_root}"
}

create_successful_command_stubs() {
    local fixture_root=$1

    cat >"${fixture_root}/bin/cargo" <<'EOF'
#!/usr/bin/env bash
printf 'cargo %s\n' "$*" >>"${TEST_COMMAND_LOG}"
EOF
    cat >"${fixture_root}/bin/linter" <<'EOF'
#!/usr/bin/env bash
printf 'linter %s\n' "$*" >>"${TEST_COMMAND_LOG}"
EOF
    chmod +x "${fixture_root}/bin/cargo" "${fixture_root}/bin/linter"
}

it_should_sort_and_remove_exact_duplicates_when_dictionary_requires_formatting() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "formatter-changed")
    printf 'zebra\nAlpha\nalpha\nAlpha\n' >"${fixture_root}/project-words.txt"

    # Act
    if "${fixture_root}/contrib/dev-tools/git/format-project-words.sh" >"${fixture_root}/formatter-output.txt" 2>&1; then
        printf 'Expected formatter to report a changed dictionary.\n' >&2
        return 1
    fi

    # Assert
    diff -u "${fixture_root}/project-words.txt" <(printf 'Alpha\nalpha\nzebra\n')
    grep -F -q 'Formatted project-words.txt with LC_ALL=C sort -u.' "${fixture_root}/formatter-output.txt"
}

it_should_report_success_when_dictionary_is_already_formatted() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "formatter-unchanged")
    printf 'Alpha\nalpha\nzebra\n' >"${fixture_root}/project-words.txt"

    # Act
    "${fixture_root}/contrib/dev-tools/git/format-project-words.sh" >"${fixture_root}/formatter-output.txt"

    # Assert
    grep -F -q 'project-words.txt is already formatted.' "${fixture_root}/formatter-output.txt"
}

it_should_report_a_temp_file_creation_failure() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "formatter-mktemp-failure")
    printf 'Alpha\nzebra\n' >"${fixture_root}/project-words.txt"
    cat >"${fixture_root}/bin/mktemp" <<'EOF'
#!/usr/bin/env bash
exit 1
EOF
    chmod +x "${fixture_root}/bin/mktemp"

    # Act
    if PATH="${fixture_root}/bin:${PATH}" "${fixture_root}/contrib/dev-tools/git/format-project-words.sh" >"${fixture_root}/formatter-output.txt" 2>&1; then
        printf 'Expected formatter to fail when it cannot create its temporary dictionary.\n' >&2
        return 1
    fi

    # Assert
    grep -F -q 'Error: failed to create a temporary project dictionary:' "${fixture_root}/formatter-output.txt"
}

it_should_abort_pre_commit_and_request_restaging_when_dictionary_is_formatted() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "hook-changed")
    printf 'zebra\nAlpha\nAlpha\n' >"${fixture_root}/project-words.txt"
    create_successful_command_stubs "${fixture_root}"

    # Act
    if (
        cd "${fixture_root}" || exit
        PATH="${fixture_root}/bin:${PATH}" \
            TEST_COMMAND_LOG="${fixture_root}/commands.log" \
            TORRUST_GIT_HOOKS_LOG_DIR="${fixture_root}/logs" \
            ./contrib/dev-tools/git/hooks/pre-commit.sh >"${fixture_root}/hook-output.txt" 2>&1
    ); then
        printf 'Expected pre-commit hook to abort after formatting the dictionary.\n' >&2
        return 1
    fi

    # Assert
    diff -u "${fixture_root}/project-words.txt" <(printf 'Alpha\nzebra\n')
    grep -F -q "Stage 'project-words.txt' and retry the commit" "${fixture_root}/hook-output.txt"
    [[ ! -e "${fixture_root}/commands.log" ]]
}

it_should_not_mislabel_log_creation_failures_as_dictionary_changes() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "hook-log-mktemp-failure")
    printf 'Alpha\nzebra\n' >"${fixture_root}/project-words.txt"
    create_successful_command_stubs "${fixture_root}"
    cat >"${fixture_root}/bin/mktemp" <<'EOF'
#!/usr/bin/env bash
if [[ "$1" == *pre-commit-* ]]; then
    exit 1
fi
exec /usr/bin/mktemp "$@"
EOF
    chmod +x "${fixture_root}/bin/mktemp"

    # Act
    if (
        cd "${fixture_root}" || exit
        PATH="${fixture_root}/bin:${PATH}" \
            TEST_COMMAND_LOG="${fixture_root}/commands.log" \
            TORRUST_GIT_HOOKS_LOG_DIR="${fixture_root}/logs" \
            ./contrib/dev-tools/git/hooks/pre-commit.sh >"${fixture_root}/hook-output.txt" 2>&1
    ); then
        printf 'Expected pre-commit hook to fail when it cannot create a step log.\n' >&2
        return 1
    fi

    # Assert
    grep -F -q "Error: failed to create a temporary log file in '${fixture_root}/logs'." "${fixture_root}/hook-output.txt"
    ! grep -F -q "The formatter changed project-words.txt." "${fixture_root}/hook-output.txt"
}

it_should_report_infrastructure_failures_with_their_exit_code_in_json() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "hook-log-mktemp-failure-json")
    printf 'Alpha\nzebra\n' >"${fixture_root}/project-words.txt"
    create_successful_command_stubs "${fixture_root}"
    cat >"${fixture_root}/bin/mktemp" <<'EOF'
#!/usr/bin/env bash
if [[ "$1" == *pre-commit-* ]]; then
    exit 2
fi
exec /usr/bin/mktemp "$@"
EOF
    chmod +x "${fixture_root}/bin/mktemp"

    # Act
    if (
        cd "${fixture_root}" || exit
        PATH="${fixture_root}/bin:${PATH}" \
            TEST_COMMAND_LOG="${fixture_root}/commands.log" \
            TORRUST_GIT_HOOKS_LOG_DIR="${fixture_root}/logs" \
            ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json >"${fixture_root}/hook-output.txt" 2>&1
    ); then
        printf 'Expected pre-commit hook to fail when it cannot create a step log.\n' >&2
        return 1
    fi

    # Assert
    grep -F -q '"exit_code": 2' "${fixture_root}/hook-output.txt"
}

it_should_continue_pre_commit_checks_when_dictionary_is_already_formatted() {
    # Arrange
    local fixture_root
    fixture_root=$(create_fixture "hook-unchanged")
    printf 'Alpha\nzebra\n' >"${fixture_root}/project-words.txt"
    create_successful_command_stubs "${fixture_root}"

    # Act
    (
        cd "${fixture_root}" || exit
        PATH="${fixture_root}/bin:${PATH}" \
            TEST_COMMAND_LOG="${fixture_root}/commands.log" \
            TORRUST_GIT_HOOKS_LOG_DIR="${fixture_root}/logs" \
            ./contrib/dev-tools/git/hooks/pre-commit.sh >"${fixture_root}/hook-output.txt"
    )

    # Assert
    [[ $(wc -l <"${fixture_root}/commands.log") -eq 4 ]]
    grep -F -q 'SUCCESS: All pre-commit checks passed!' "${fixture_root}/hook-output.txt"
}

it_should_sort_and_remove_exact_duplicates_when_dictionary_requires_formatting
it_should_report_success_when_dictionary_is_already_formatted
it_should_report_a_temp_file_creation_failure
it_should_abort_pre_commit_and_request_restaging_when_dictionary_is_formatted
it_should_not_mislabel_log_creation_failures_as_dictionary_changes
it_should_report_infrastructure_failures_with_their_exit_code_in_json
it_should_continue_pre_commit_checks_when_dictionary_is_already_formatted

printf 'All formatter and pre-commit hook tests passed.\n'