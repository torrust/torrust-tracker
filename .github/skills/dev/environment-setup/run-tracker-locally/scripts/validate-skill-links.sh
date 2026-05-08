#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../../../../.." && pwd)"
MARKER="skill-link: run-tracker-locally"

required_files=(
  "src/bootstrap/config.rs"
  "share/default/config/tracker.development.sqlite3.toml"
  "src/lib.rs"
  "README.md"
)

has_errors=0

for rel_path in "${required_files[@]}"; do
  full_path="${REPO_ROOT}/${rel_path}"

  if [[ ! -f "${full_path}" ]]; then
    echo "Missing required file: ${rel_path}" >&2
    has_errors=1
    continue
  fi

  if ! grep -Fq "${MARKER}" "${full_path}"; then
    echo "Missing marker '${MARKER}' in: ${rel_path}" >&2
    has_errors=1
  fi
done

if [[ "${has_errors}" -ne 0 ]]; then
  exit 1
fi

echo "Skill links validation passed"
