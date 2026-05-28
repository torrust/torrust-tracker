#!/usr/bin/env bash
# run-container-baseline.sh
#
# semantic-links:
#   related-artifacts:
#     - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/ISSUE.md
#     - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
#     - .github/workflows/container.yaml
#
# Reproducible baseline timing capture for container-workflow-equivalent steps.
# Mirrors .github/workflows/container.yaml (job: test, matrix: debug + release).
#
# The CI workflow runs debug and release in parallel (matrix strategy).
# This script runs them sequentially.  Total CI wall time ≈ max(debug, release).
#
# Usage:
#   ./contrib/dev-tools/workflow-benchmarks/run-container-baseline.sh [--cold]
#
# Options:
#   --cold   Clear Docker builder cache and remove the tracked local image
#            before measuring, approximating a shared-runner first run.
#            Omit to measure the warm (cached) case.
#
# Output:
#   Structured timing lines on stdout and a dated log under:
#   docs/issues/open/1841-1840-workflow-performance-baseline-analysis/evidence/
#
# Re-use after later optimisations:
#   Run this script once --cold and once without --cold after each change and
#   compare the evidence logs to quantify the improvement.

set -euo pipefail

COLD=false
for arg in "$@"; do
    case "$arg" in
        --cold) COLD=true ;;
        *) echo "Unknown argument: $arg" >&2; exit 1 ;;
    esac
done

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
EVIDENCE_DIR="$REPO_ROOT/docs/issues/open/1841-1840-workflow-performance-baseline-analysis/evidence"
mkdir -p "$EVIDENCE_DIR"

RUN_TYPE="warm"
$COLD && RUN_TYPE="cold"

LOG="$EVIDENCE_DIR/container-baseline-$(date -u +%Y%m%dT%H%M%SZ)-${RUN_TYPE}.log"

time_phase() {
    local scope="$1" name="$2"
    shift 2
    echo "[$scope] ${name}_start"
    local t0 t1 rc
    t0=$(date +%s)
    "$@"
    rc=$?
    t1=$(date +%s)
    echo "[$scope] ${name}_seconds=$((t1 - t0))"
    echo "[$scope] ${name}_exit_code=$rc"
    return $rc
}

{
    echo "[meta] start_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "[meta] workflow=container"
    echo "[meta] run_type=${RUN_TYPE}"
    echo "[meta] repo_root=${REPO_ROOT}"

    if $COLD; then
        echo "[cold] cache_reset_start"
        docker builder prune -af >/dev/null
        docker image rm -f torrust-tracker:local >/dev/null 2>&1 || true
        echo "[cold] cache_reset_done"
    fi

    # --- debug target (first matrix entry) ---
    # --progress plain writes per-layer step output to stdout so it is captured
    # in the evidence log alongside the phase timing lines.  Without this flag
    # Docker (BuildKit) emits the interactive progress to stderr only.
    time_phase "${RUN_TYPE}" build_debug \
        docker build \
            --progress plain \
            --file "${REPO_ROOT}/Containerfile" \
            --target debug \
            --tag torrust-tracker:local \
            "${REPO_ROOT}"

    time_phase "${RUN_TYPE}" inspect_debug \
        docker image inspect torrust-tracker:local

    # --- release target (second matrix entry) ---
    time_phase "${RUN_TYPE}" build_release \
        docker build \
            --progress plain \
            --file "${REPO_ROOT}/Containerfile" \
            --target release \
            --tag torrust-tracker:local \
            "${REPO_ROOT}"

    time_phase "${RUN_TYPE}" inspect_release \
        docker image inspect torrust-tracker:local

    echo "[meta] end_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
} | tee "$LOG"

echo ""
echo "Evidence log: $LOG"
