#!/usr/bin/env bash
# Pre-commit verification script
# Run all mandatory checks before committing changes.
#
# Usage:
#   ./contrib/dev-tools/git/hooks/pre-commit.sh
#
# Expected runtime: ~1 minute on a modern developer machine (concise default profile).
# AI agents: set a per-command timeout of at least 3 minutes before invoking this script.
#
# All steps must pass (exit 0) before committing.
# The formatter is an intentionally small interim action while EPIC #2003 determines
# the repository's long-term automation architecture. It exits 1 after rewriting the
# dictionary so this hook aborts and the contributor can deliberately stage the change.
#
# TODO: Implement branch-name validation in the Rust git-hooks binary (#1843).
# When the branch uses an issue-number prefix (e.g. "42-some-description"), verify that
# docs/issues/open/ contains a matching spec file or directory starting with that number.
# This prevents committing under a wrong, closed, or non-existent issue number.
# See also: docs/issues/open/1843-migrate-git-hooks-scripts-from-bash-to-rust.md

set -uo pipefail

# ============================================================================
# STEPS
# ============================================================================
# Each step: "description|command"

declare -a STEPS=(
    "Formatting project dictionary|./contrib/dev-tools/git/format-project-words.sh"
    "Checking for unused dependencies (cargo machete --with-metadata)|cargo machete --with-metadata"
    "Checking workspace layer boundary bans (cargo deny check bans)|cargo deny check bans"
    "Running all linters|linter all"
    "Linting Containerfile with hadolint|if git diff --cached --name-only --diff-filter=ACM | grep -q '^Containerfile$'; then docker run --rm -i -v \"$(pwd)/.hadolint.yaml:/.hadolint.yaml\" --entrypoint hadolint hadolint/hadolint@sha256:27086352fd5e1907ea2b934eb1023f217c5ae087992eb59fde121dce9c9ff21e --config /.hadolint.yaml - < ./Containerfile; else echo 'Containerfile unchanged, skipping hadolint'; fi"
    "Running documentation tests|cargo test --doc --workspace"
)

FORMAT="text"
VERBOSITY="concise"
FAILURE_TAIL_LINES=10
LOG_DIR="${TORRUST_GIT_HOOKS_LOG_DIR:-/tmp}"

declare -a STEP_NAMES=()
declare -a STEP_COMMANDS=()
declare -a STEP_STATUSES=()
declare -a STEP_ELAPSED_SECONDS=()
declare -a STEP_LOG_PATHS=()

# ============================================================================
# HELPER FUNCTIONS
# ============================================================================

format_time() {
    local total_seconds=$1
    local minutes=$((total_seconds / 60))
    local seconds=$((total_seconds % 60))
    if [ "$minutes" -gt 0 ]; then
        echo "${minutes}m ${seconds}s"
    else
        echo "${seconds}s"
    fi
}

print_usage() {
    cat >&2 <<'EOF'
Usage: ./contrib/dev-tools/git/hooks/pre-commit.sh [--format=<text|json>] [--verbosity=<concise|verbose>] [--verbose]

Options:
  --format=<text|json>          Output format. Default: text
  --verbosity=<concise|verbose> Text output verbosity. Default: concise
  --verbose                     Compatibility alias for --verbosity=verbose
  -h, --help                    Show this help

Environment:
  TORRUST_GIT_HOOKS_LOG_DIR     Directory for per-step log files (shared by all git hooks). Default: /tmp
EOF
}

prepare_log_dir() {
    if ! mkdir -p "${LOG_DIR}"; then
        echo "Error: cannot create log directory '${LOG_DIR}'." >&2
        exit 2
    fi

    if [[ ! -d "${LOG_DIR}" || ! -w "${LOG_DIR}" ]]; then
        echo "Error: log directory '${LOG_DIR}' is not writable." >&2
        exit 2
    fi
}

json_escape() {
    local input=$1
    input=${input//\\/\\\\}
    input=${input//\"/\\\"}
    input=${input//$'\b'/\\b}
    input=${input//$'\f'/\\f}
    input=${input//$'\n'/\\n}
    input=${input//$'\r'/\\r}
    input=${input//$'\t'/\\t}
    input=$(printf '%s' "${input}" | tr -d '\000-\010\013\016-\037')
    printf '%s' "${input}"
}

strip_ansi() {
    sed -E 's/\x1B\[[0-9;]*[A-Za-z]//g'
}

sanitize_name_for_log() {
    local raw_name=$1
    local normalized
    normalized=$(printf '%s' "${raw_name}" | tr '[:upper:]' '[:lower:]' | tr -cs 'a-z0-9' '-')
    normalized=${normalized#-}
    normalized=${normalized%-}
    if [[ -z "${normalized}" ]]; then
        normalized="step"
    fi
    printf '%s' "${normalized}"
}

print_step_summary() {
    local step_number=$1
    local total_steps=$2
    local description=$3
    local status=$4
    local elapsed_seconds=$5
    local log_path=$6

    if [[ "${status}" == "pass" ]]; then
        printf '[Step %d/%d] %s ... PASS (%s)\n' "${step_number}" "${total_steps}" "${description}" "$(format_time "${elapsed_seconds}")"
        return
    fi

    printf '[Step %d/%d] %s ... FAIL (%s)  log: %s\n' \
        "${step_number}" \
        "${total_steps}" \
        "${description}" \
        "$(format_time "${elapsed_seconds}")" \
        "${log_path}"

    local -a tail_lines=()
    while IFS= read -r line; do
        tail_lines+=("${line}")
    done < <(tail -n "${FAILURE_TAIL_LINES}" "${log_path}" | strip_ansi)

    local shown_count=${#tail_lines[@]}
    for line in "${tail_lines[@]}"; do
        printf '    %s\n' "${line}"
    done

    printf '    (%d lines shown - full log: %s)\n' "${shown_count}" "${log_path}"
}

run_command() {
    local command=$1
    local log_path=$2

    if [[ "${FORMAT}" == "text" && "${VERBOSITY}" == "verbose" ]]; then
        bash -o pipefail -c "${command}" 2>&1 | tee "${log_path}"
        local command_exit_code=${PIPESTATUS[0]}
        return "${command_exit_code}"
    fi

    bash -o pipefail -c "${command}" >"${log_path}" 2>&1
}

run_step() {
    local step_number=$1
    local total_steps=$2
    local description=$3
    local command=$4

    if [[ "${FORMAT}" == "text" && "${VERBOSITY}" == "verbose" ]]; then
        printf '[Step %d/%d] %s...\n' "${step_number}" "${total_steps}" "${description}"
    fi

    local step_start=$SECONDS

    local safe_name
    safe_name=$(sanitize_name_for_log "${description}")
    local _tmp log_path
    if ! _tmp=$(mktemp "${LOG_DIR%/}/pre-commit-${safe_name}-XXXXXX"); then
        echo "Error: failed to create a temporary log file in '${LOG_DIR}'." >&2
        return 2
    fi
    log_path="${_tmp}.log"
    mv "$_tmp" "$log_path"

    run_command "${command}" "${log_path}"
    local command_exit_code=$?

    local step_elapsed=$((SECONDS - step_start))

    STEP_NAMES+=("${description}")
    STEP_COMMANDS+=("${command}")
    STEP_ELAPSED_SECONDS+=("${step_elapsed}")
    STEP_LOG_PATHS+=("${log_path}")

    if [[ "${command_exit_code}" -eq 0 ]]; then
        STEP_STATUSES+=("pass")
    else
        STEP_STATUSES+=("fail")
    fi

    local step_status=${STEP_STATUSES[$(( ${#STEP_STATUSES[@]} - 1 ))]}

    if [[ "${FORMAT}" == "text" ]]; then
        print_step_summary \
            "${step_number}" \
            "${total_steps}" \
            "${description}" \
            "${step_status}" \
            "${step_elapsed}" \
            "${log_path}"
        if [[ "${VERBOSITY}" == "verbose" ]]; then
            echo
        fi
    fi

    return "${command_exit_code}"
}

emit_json_result() {
    local overall_status=$1
    local exit_code=$2
    local total_elapsed=$3
    local failed_step_name=$4

    printf '{\n'
    printf '  "schema_version": 1,\n'
    printf '  "status": "%s",\n' "${overall_status}"
    printf '  "exit_code": %d,\n' "${exit_code}"
    printf '  "elapsed_seconds": %d' "${total_elapsed}"

    if [[ -n "${failed_step_name}" ]]; then
        printf ',\n  "failed_step": "%s"' "$(json_escape "${failed_step_name}")"
    fi

    printf ',\n  "steps": [\n'

    local steps_count=${#STEP_NAMES[@]}
    for ((index = 0; index < steps_count; index++)); do
        local name=${STEP_NAMES[$index]}
        local command=${STEP_COMMANDS[$index]}
        local status=${STEP_STATUSES[$index]}
        local elapsed=${STEP_ELAPSED_SECONDS[$index]}
        local log_path=${STEP_LOG_PATHS[$index]}

        printf '    {\n'
        printf '      "name": "%s",\n' "$(json_escape "${name}")"
        printf '      "command": "%s",\n' "$(json_escape "${command}")"
        printf '      "status": "%s",\n' "${status}"
        printf '      "elapsed_seconds": %d' "${elapsed}"

        if [[ "${status}" == "fail" ]]; then
            printf ',\n      "log_path": "%s",\n' "$(json_escape "${log_path}")"
            printf '      "failure_tail": ['

            local -a tail_lines=()
            while IFS= read -r line; do
                tail_lines+=("${line}")
            done < <(tail -n "${FAILURE_TAIL_LINES}" "${log_path}" | strip_ansi)

            local tail_count=${#tail_lines[@]}
            for ((tail_index = 0; tail_index < tail_count; tail_index++)); do
                if [[ "${tail_index}" -gt 0 ]]; then
                    printf ', '
                fi
                printf '"%s"' "$(json_escape "${tail_lines[$tail_index]}")"
            done
            printf ']'
        fi

        if [[ "${index}" -lt $((steps_count - 1)) ]]; then
            printf '\n    },\n'
        else
            printf '\n    }\n'
        fi
    done

    printf '  ]\n'
    printf '}\n'
}

parse_args() {
    for arg in "$@"; do
        case "${arg}" in
            --format=text)
                FORMAT="text"
                ;;
            --format=json)
                FORMAT="json"
                ;;
            --verbosity=concise)
                VERBOSITY="concise"
                ;;
            --verbosity=verbose)
                VERBOSITY="verbose"
                ;;
            --verbose)
                VERBOSITY="verbose"
                ;;
            -h|--help)
                print_usage
                exit 0
                ;;
            --format=*)
                echo "Error: invalid --format value in '${arg}'. Expected --format=text or --format=json." >&2
                print_usage
                exit 2
                ;;
            --verbosity=*)
                echo "Error: invalid --verbosity value in '${arg}'. Expected --verbosity=concise or --verbosity=verbose." >&2
                print_usage
                exit 2
                ;;
            *)
                echo "Error: unknown option '${arg}'." >&2
                print_usage
                exit 2
                ;;
        esac
    done
}

parse_args "$@"
prepare_log_dir

# ============================================================================
# MAIN
# ============================================================================

TOTAL_START=$SECONDS
TOTAL_STEPS=${#STEPS[@]}
overall_status="pass"
exit_code=0
failed_step_name=""
failed_step_exit_code=0

if [[ "${FORMAT}" == "text" ]]; then
    echo "Running pre-commit checks..."
    echo
fi

for i in "${!STEPS[@]}"; do
    IFS='|' read -r description command <<< "${STEPS[$i]}"
    if run_step $((i + 1)) "${TOTAL_STEPS}" "${description}" "${command}"; then
        step_exit_code=0
    else
        step_exit_code=$?
        overall_status="fail"
        exit_code=${step_exit_code}
        failed_step_name="${description}"
        failed_step_exit_code=${step_exit_code}
        break
    fi
done

TOTAL_ELAPSED=$((SECONDS - TOTAL_START))

if [[ "${FORMAT}" == "json" ]]; then
    emit_json_result "${overall_status}" "${exit_code}" "${TOTAL_ELAPSED}" "${failed_step_name}"
    exit "${exit_code}"
fi

if [[ "${overall_status}" == "pass" ]]; then
    echo "=========================================="
    echo "SUCCESS: All pre-commit checks passed! ($(format_time "${TOTAL_ELAPSED}"))"
    echo "=========================================="
    echo
    echo "You can now safely stage and commit your changes."
    exit 0
fi

echo
echo "=========================================="
echo "FAILED: Pre-commit checks failed!"
if [[ "${failed_step_name}" == "Formatting project dictionary" && "${failed_step_exit_code}" -eq 1 ]]; then
    echo "The formatter changed project-words.txt. Stage 'project-words.txt' and retry the commit."
fi
echo "Fix the errors above before committing."
echo "=========================================="
exit 1
