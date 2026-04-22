#!/usr/bin/env bash
# Pre-push verification script
# Run comprehensive checks before pushing changes, including nightly toolchain
# validation and end-to-end tests.
#
# Usage:
#   ./contrib/dev-tools/git/hooks/pre-push.sh
#
# Expected runtime: ~15 minutes on a modern developer machine.
# AI agents: set a per-command timeout of at least 30 minutes before invoking this script.
#
# All steps must pass (exit 0) before pushing.

set -euo pipefail

# ============================================================================
# STEPS
# ============================================================================
# Each step: "description|success_message|command"

declare -a STEPS=(
    "Checking for unused dependencies (cargo machete)|No unused dependencies found|cargo +stable machete"
    "Running all linters|All linters passed|linter all"
    "Checking format with nightly toolchain|Nightly format check passed|cargo +nightly fmt --check"
    "Checking workspace with nightly toolchain|Nightly check passed|cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features"
    "Building documentation with nightly toolchain|Nightly documentation built|cargo +nightly doc --no-deps --bins --examples --workspace --all-features"
    "Running documentation tests|Documentation tests passed|cargo +stable test --doc --workspace"
    "Running all tests|All tests passed|cargo +stable test --tests --benches --examples --workspace --all-targets --all-features"
    "Running E2E tests|E2E tests passed|cargo +stable run --bin e2e_tests_runner -- --config-toml-path ./share/default/config/tracker.e2e.container.sqlite3.toml"
)

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

run_step() {
    local step_number=$1
    local total_steps=$2
    local description=$3
    local success_message=$4
    local command=$5

    echo "[Step ${step_number}/${total_steps}] ${description}..."

    local step_start=$SECONDS
    local -a cmd_array
    read -ra cmd_array <<< "${command}"
    "${cmd_array[@]}"
    local step_elapsed=$((SECONDS - step_start))

    echo "PASSED: ${success_message} ($(format_time "${step_elapsed}"))"
    echo
}

trap 'echo ""; echo "=========================================="; echo "FAILED: Pre-push checks failed!"; echo "Fix the errors above before pushing."; echo "=========================================="; exit 1' ERR

# ============================================================================
# MAIN
# ============================================================================

TOTAL_START=$SECONDS
TOTAL_STEPS=${#STEPS[@]}

echo "Running pre-push checks..."
echo

for i in "${!STEPS[@]}"; do
    IFS='|' read -r description success_message command <<< "${STEPS[$i]}"
    run_step $((i + 1)) "${TOTAL_STEPS}" "${description}" "${success_message}" "${command}"
done

TOTAL_ELAPSED=$((SECONDS - TOTAL_START))
echo "=========================================="
echo "SUCCESS: All pre-push checks passed! ($(format_time "${TOTAL_ELAPSED}"))"
echo "=========================================="
echo
echo "You can now safely push your changes."
