#!/bin/bash
# Verification run for #2757. Run from anywhere inside the checkout; strip ANSI codes from the output.
# The controls edit deny.toml and the package Cargo.toml and restore them with `git checkout`;
# run it on a clean tree. `cargo-chef` 0.1.79 must be on PATH for the container recipe step (optional).
set -u
cd "$(git rev-parse --show-toplevel)" || exit 1
P=torrust-tracker-axum-health-check-api-server
SELF=docs/issues/open/2757-2482-gate-axum-health-check-api-server-test-environment/evidence
run() { echo; echo "\$ $*"; bash -c "set -o pipefail; $*" 2>&1; echo "[exit $?]"; }
echo "start $(date -u '+%Y-%m-%d %H:%M:%S UTC')"; run "git log --oneline -1"
echo; echo "=== Baseline at the merge base (T1, F1): the library without features"
BASE=$(git merge-base HEAD torrust/develop)
rm -rf /tmp/row7-base-check; git worktree add -q --detach /tmp/row7-base-check "$BASE"
run "cd /tmp/row7-base-check && git log --oneline -1 && cargo check -p $P --lib 2>&1 | grep -E '^error|could not compile' | head -3"
run "cd /tmp/row7-base-check && cargo tree -p $P -e normal --depth 1 | grep -c '^[├└]'"
run "cd /tmp/row7-base-check && cargo test -p $P 2>&1 | grep -E 'test result|Running'"
git worktree remove --force /tmp/row7-base-check
echo; echo "=== V1 direct normal dependencies"
run "cargo tree -p $P -e normal --depth 1 | grep -c '^[├└]'"
run "cargo tree -p $P -e normal --depth 1 | grep -c configuration"
run "cargo tree -p $P -e normal --depth 1 -F testing | grep -c '^[├└]'"
echo; echo "=== V2 tests and the library without features"
run "cargo check -p $P --lib 2>&1 | tail -1"
run "cargo test -p $P 2>&1 | grep -E 'test result|Running'"
run "cargo test -p $P --all-features 2>&1 | grep -E 'test result|Running'"
echo; echo "=== V3 shipped features"
run "cargo run -q --package shipped-features-check -- --package torrust-tracker"
echo; echo "=== V4 deny, machete, packaging, nightly"
run "cargo deny check bans 2>&1 | tail -1"
run "cargo machete --with-metadata 2>&1 | tail -2"
run "cargo +stable package --workspace --no-verify 2>&1 | tail -1"
run "cargo +nightly check --tests --benches --examples --workspace --all-targets --all-features 2>&1 | tail -1"
echo; echo "=== Control: deny without the self wrapper entry"
python3 - <<'PY'
s = open("deny.toml").read()
t = s.replace('    # Its own path-only self dev-dependency enabling the `testing` feature (ADR 20261021190318, rule A.3).\n    "torrust-tracker-axum-health-check-api-server",\n  ] },\n  { crate = "torrust-tracker-axum-http-server"', '  ] },\n  { crate = "torrust-tracker-axum-http-server"', 1)
assert s != t
open("deny.toml", "w").write(t)
PY
run "RAYON_NUM_THREADS=1 cargo deny check bans 2>&1 | grep -E 'unmatched-wrapper|error\\[banned\\]'"
git checkout -q deny.toml
echo; echo "=== Control: no self dev-dependency (the lib tests do not compile)"
sed -i '/^torrust-tracker-axum-health-check-api-server = { path/d' packages/axum-health-check-api-server/Cargo.toml
run "cargo test -p $P 2>&1 | grep -E '^error|could not compile' | head -3"
git checkout -q packages/axum-health-check-api-server/Cargo.toml Cargo.lock
run "git status --short deny.toml Cargo.lock packages/axum-health-check-api-server/Cargo.toml"
echo; echo "=== V6 AC8: coupling report labels and the diagram"
run "grep -c 'optional (testing)' docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-11-09.md"
run "python3 $SELF/check_diagram.py"
echo; echo "end $(date -u '+%Y-%m-%d %H:%M:%S UTC')"
