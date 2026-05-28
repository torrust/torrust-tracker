#!/usr/bin/env bash
# run-testing-baseline.sh
#
# semantic-links:
#   related-artifacts:
#     - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/ISSUE.md
#     - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
#     - .github/workflows/testing.yaml
#
# Reproducible baseline timing capture for testing-workflow-equivalent steps.
# Mirrors .github/workflows/testing.yaml (jobs: unit + docker-e2e).
#
# The CI workflow runs unit(nightly) + unit(stable) + docker-e2e in parallel.
# This script runs phases sequentially; CI wall time ≈ max(unit_stable, docker-e2e).
#
# Usage:
#   ./contrib/dev-tools/workflow-benchmarks/run-testing-baseline.sh [--cold]
#
# Options:
#   --cold   Use isolated CARGO_HOME and target dir, and clear the Docker builder
#            cache before measuring, approximating a shared-runner first run.
#            Omit to use the default ~/.cargo and target/ (warm / incremental).
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

LOG="$EVIDENCE_DIR/testing-baseline-$(date -u +%Y%m%dT%H%M%SZ)-${RUN_TYPE}.log"

time_phase() {
    local scope="$1" name="$2"
    shift 2
    echo "[$scope] ${name}_start"
    local t0 t1 rc
    t0=$(date +%s)
    set +e
    "$@"
    rc=$?
    set -e
    t1=$(date +%s)
    echo "[$scope] ${name}_seconds=$((t1 - t0))"
    echo "[$scope] ${name}_exit_code=$rc"
    return $rc
}

{
    echo "[meta] start_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    echo "[meta] workflow=testing"
    echo "[meta] run_type=${RUN_TYPE}"
    echo "[meta] repo_root=${REPO_ROOT}"

    if $COLD; then
        TMP_HOME="${REPO_ROOT}/.tmp/workflow-benchmarks/cargo-home"
        TMP_TARGET="${REPO_ROOT}/.tmp/workflow-benchmarks/target"
        echo "[cold] cache_reset_start"
        rm -rf "${TMP_HOME}" "${TMP_TARGET}"
        mkdir -p "${TMP_HOME}" "${TMP_TARGET}"
        docker builder prune -af >/dev/null
        docker image rm -f torrust-tracker:e2e-local >/dev/null 2>&1 || true
        export CARGO_HOME="${TMP_HOME}"
        export CARGO_TARGET_DIR="${TMP_TARGET}"
        echo "[cold] cache_reset_done"
        echo "[meta] cargo_home=${TMP_HOME}"
        echo "[meta] cargo_target_dir=${TMP_TARGET}"
    fi

    cd "${REPO_ROOT}"

    # --- unit job (shared phases) ---
    time_phase "${RUN_TYPE}" fetch \
        cargo fetch --verbose

    time_phase "${RUN_TYPE}" install_linter \
        cargo install --locked \
            --git https://github.com/torrust/torrust-linting \
            --bin linter

    # nightly-only in CI; run unconditionally to measure time
    time_phase "${RUN_TYPE}" format \
        cargo fmt --check

    time_phase "${RUN_TYPE}" lint \
        linter all

    time_phase "${RUN_TYPE}" test_docs \
        cargo test --doc --workspace

    time_phase "${RUN_TYPE}" test_unit \
        cargo test --tests --benches --examples --workspace --all-targets --all-features

    # --- docker-e2e job ---
    time_phase "${RUN_TYPE}" docker_build_e2e \
        docker build \
            --file "${REPO_ROOT}/Containerfile" \
            --target release \
            --tag torrust-tracker:e2e-local \
            "${REPO_ROOT}"

    time_phase "${RUN_TYPE}" e2e_tracker \
        cargo run --bin e2e_tests_runner -- \
            --config-toml-path "./share/default/config/tracker.e2e.container.sqlite3.toml" \
            --tracker-image "torrust-tracker:e2e-local" \
            --skip-build

    time_phase "${RUN_TYPE}" e2e_qbittorrent_sqlite \
        cargo run --bin qbittorrent_e2e_runner -- \
            --tracker-image "torrust-tracker:e2e-local" \
            --skip-build \
            --db-driver sqlite3 \
            --timeout-seconds 600

    time_phase "${RUN_TYPE}" e2e_qbittorrent_mysql \
        cargo run --bin qbittorrent_e2e_runner -- \
            --tracker-image "torrust-tracker:e2e-local" \
            --skip-build \
            --db-driver mysql \
            --timeout-seconds 600

    time_phase "${RUN_TYPE}" e2e_qbittorrent_postgresql \
        cargo run --bin qbittorrent_e2e_runner -- \
            --tracker-image "torrust-tracker:e2e-local" \
            --skip-build \
            --db-driver postgresql \
            --timeout-seconds 600

    echo "[meta] end_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
} | tee "$LOG"

echo ""
echo "Evidence log: $LOG"
