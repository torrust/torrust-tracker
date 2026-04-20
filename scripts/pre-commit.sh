#!/bin/bash
# Pre-commit verification script
# Run all mandatory checks before committing changes.
#
# Usage:
#   ./scripts/pre-commit.sh
#
# Expected runtime: ~3 minutes on a modern developer machine.
# AI agents: set a per-command timeout of at least 5 minutes before invoking this script.
#
# All steps must pass (exit 0) before committing.

set -euo pipefail

# ============================================================================
# STEPS
# ============================================================================
# Each step: "description|success_message|command"

declare -a STEPS=(
    "Checking for unused dependencies (cargo machete)|No unused dependencies found|cargo machete"
    "Running all linters|All linters passed|linter all"
    "Running documentation tests|Documentation tests passed|cargo test --doc --workspace"
    "Running all tests|All tests passed|cargo test --tests --benches --examples --workspace --all-targets --all-features"
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
    eval "${command}"
    local step_elapsed=$((SECONDS - step_start))

    echo "PASSED: ${success_message} ($(format_time "${step_elapsed}"))"
    echo
}

trap 'echo ""; echo "=========================================="; echo "FAILED: Pre-commit checks failed!"; echo "Fix the errors above before committing."; echo "=========================================="; exit 1' ERR

# ============================================================================
# MAIN
# ============================================================================

TOTAL_START=$SECONDS
TOTAL_STEPS=${#STEPS[@]}

echo "Running pre-commit checks..."
echo

for i in "${!STEPS[@]}"; do
    IFS='|' read -r description success_message command <<< "${STEPS[$i]}"
    run_step $((i + 1)) "${TOTAL_STEPS}" "${description}" "${success_message}" "${command}"
done

TOTAL_ELAPSED=$((SECONDS - TOTAL_START))
echo "=========================================="
echo "SUCCESS: All pre-commit checks passed! ($(format_time "${TOTAL_ELAPSED}"))"
echo "=========================================="
echo
echo "You can now safely stage and commit your changes."
