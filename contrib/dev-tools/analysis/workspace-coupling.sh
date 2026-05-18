#!/usr/bin/env bash
#
# workspace-coupling.sh
#
# Generates a workspace coupling report for the Torrust Tracker repository.
#
# For every workspace package that has workspace-level dependencies the script:
#   1. Lists the declared workspace dependencies (normal / dev / build).
#   2. Scans the package's src/ directory for `use DEP_MODULE::` statements and
#      fully-qualified `DEP_MODULE::` path references, then lists the distinct
#      top-level import paths found.
#
# A short import list (1-3 items) is a signal that the dependency may be weak
# and worth reviewing (e.g. moving a single constant to eliminate the edge).
#
# Requirements: cargo, jq, ripgrep (rg)
#
# Usage:
#   ./contrib/dev-tools/analysis/workspace-coupling.sh [OUTPUT_FILE]
#
# If OUTPUT_FILE is omitted the report is written to:
#   docs/media/packages/workspace-coupling-report.md
#
# Exit codes: 0 on success, non-zero on error.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
OUTPUT_FILE="${1:-$WORKSPACE_ROOT/docs/media/packages/workspace-coupling-report.md}"

echo "Workspace root : $WORKSPACE_ROOT" >&2
echo "Output file    : $OUTPUT_FILE" >&2
echo "" >&2

# ---------------------------------------------------------------------------
# 1. Load workspace metadata
# ---------------------------------------------------------------------------
cd "$WORKSPACE_ROOT"
METADATA=$(cargo metadata --format-version 1 2>/dev/null)

# Build a JSON array of workspace member names (used as a lookup set later).
WORKSPACE_NAME_SET=$(echo "$METADATA" | jq -c '
  .workspace_members as $members |
  [.packages[] | select(.id as $id | $members | index($id) != null) | .name]
')

# Sorted list of workspace member names, one per line.
WORKSPACE_MEMBER_NAMES=$(echo "$WORKSPACE_NAME_SET" | jq -r 'sort | .[]')

# Count total workspace members for the header.
TOTAL=$(echo "$WORKSPACE_NAME_SET" | jq 'length')

# ---------------------------------------------------------------------------
# 2. Helper: convert a crate name to its Rust module identifier
#    (hyphens → underscores).
# ---------------------------------------------------------------------------
crate_to_module() { echo "$1" | tr '-' '_'; }

# ---------------------------------------------------------------------------
# 3. Render the report
# ---------------------------------------------------------------------------
{
  echo "# Workspace Coupling Report"
  echo ""
  echo "Generated: $(date -u '+%Y-%m-%d %H:%M UTC')"
  echo ""
  echo "Workspace packages: $TOTAL"
  echo ""
  echo "---"
  echo ""
  echo "## How to read this report"
  echo ""
  echo "Each section covers one workspace package that has at least one workspace-level"
  echo "dependency. For every dependency the items actually imported from it are listed:"
  echo ""
  echo "- **Normal dep** — required for compilation of the library/binary."
  echo "- **Dev dep** — required only in tests and benchmarks."
  echo "- **Build dep** — required only in \`build.rs\`."
  echo ""
  echo "Items are extracted by scanning the package's \`src/\` directory for"
  echo "\`use MODULE::\` statements and \`MODULE::\` fully-qualified path references."
  echo "The scan is text-based; it may miss items imported through re-exports or macros,"
  echo "but it is accurate enough to identify thin-dependency patterns."
  echo ""
  echo "**Signal**: a dependency with only 1–3 distinct import paths may be a candidate"
  echo "for elimination (move the item, break the edge)."
  echo ""
  echo "---"
  echo ""
  echo "## Packages with no workspace dependencies"
  echo ""
  echo "These packages are leaves (no workspace dep) and are prime extraction candidates."
  echo ""

  # List leaf packages.
  LEAF_LIST=""
  while IFS= read -r PKG_NAME; do
    DEP_COUNT=$(echo "$METADATA" | jq --arg name "$PKG_NAME" \
      --argjson ws_names "$WORKSPACE_NAME_SET" '
      .packages[] | select(.name == $name) |
      [.dependencies[] | select(.name as $n | $ws_names | index($n) != null)] | length
    ')
    if [ "$DEP_COUNT" -eq 0 ]; then
      echo "- \`$PKG_NAME\`"
      LEAF_LIST="${LEAF_LIST}${PKG_NAME}\n"
    fi
  done <<< "$WORKSPACE_MEMBER_NAMES"

  echo ""
  echo "---"
  echo ""
  echo "## Package coupling details"
  echo ""
} > "$OUTPUT_FILE"

# ---------------------------------------------------------------------------
# 4. Per-package sections (only packages that have workspace deps)
# ---------------------------------------------------------------------------
while IFS= read -r PKG_NAME; do
  # Extract this package's workspace dependencies (all kinds).
  PKG_MANIFEST=$(echo "$METADATA" | jq -r --arg name "$PKG_NAME" '
    .packages[] | select(.name == $name) | .manifest_path
  ')
  PKG_DIR="$(dirname "$PKG_MANIFEST")"
  PKG_SRC_DIR="$PKG_DIR/src"

  # Build a sorted list of workspace deps as JSON objects {name, kind}.
  WORKSPACE_DEPS=$(echo "$METADATA" | jq -c --arg name "$PKG_NAME" \
    --argjson ws_names "$WORKSPACE_NAME_SET" '
    .packages[] | select(.name == $name) |
    [
      .dependencies[] |
      select(.name as $n | $ws_names | index($n) != null) |
      {name: .name, kind: (.kind // "normal")}
    ] | sort_by(.kind, .name)
  ')

  DEP_COUNT=$(echo "$WORKSPACE_DEPS" | jq 'length')
  if [ "$DEP_COUNT" -eq 0 ]; then
    continue
  fi

  {
    echo "### \`$PKG_NAME\`"
    echo ""
    echo "Workspace deps: $DEP_COUNT"
    echo ""
  } >> "$OUTPUT_FILE"

  # For each workspace dependency, scan the source for imports.
  while IFS= read -r DEP_JSON; do
    DEP_NAME=$(echo "$DEP_JSON" | jq -r '.name')
    DEP_KIND=$(echo "$DEP_JSON" | jq -r '.kind')
    DEP_MODULE=$(crate_to_module "$DEP_NAME")

    {
      echo "#### \`$DEP_NAME\` [$DEP_KIND]"
      echo ""
    } >> "$OUTPUT_FILE"

    if [ -d "$PKG_SRC_DIR" ]; then
      # Search for: `use DEP_MODULE::` and bare `DEP_MODULE::Foo` references.
      # Extract the path up to the next space, semicolon, brace, or comma.
      IMPORTS=$(
        rg --no-filename --no-line-number \
          "${DEP_MODULE}::[A-Za-z_]" \
          "$PKG_SRC_DIR" 2>/dev/null \
        | grep -oP "${DEP_MODULE}::[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)?" \
        | sort -u \
        || true
      )

      if [ -n "$IMPORTS" ]; then
        echo "$IMPORTS" | while IFS= read -r IMPORT; do
          echo "- \`$IMPORT\`"
        done >> "$OUTPUT_FILE"
      else
        # Check if there are any references at all (maybe macro-only usage)
        ANY=$(
          rg --no-filename --no-line-number \
            "${DEP_MODULE}" \
            "$PKG_SRC_DIR" 2>/dev/null | head -1 \
          || true
        )
        if [ -n "$ANY" ]; then
          echo "_Items not extracted — dependency used without a direct \`use\` path (macro, re-export, or glob import)._" >> "$OUTPUT_FILE"
        else
          echo "_No \`${DEP_MODULE}::\` references found in \`src/\` — may be used only in \`Cargo.toml\` feature flags or \`build.rs\`._" >> "$OUTPUT_FILE"
        fi
      fi
    else
      echo "_Source directory \`src/\` not found at \`$PKG_SRC_DIR\`._" >> "$OUTPUT_FILE"
    fi

    echo "" >> "$OUTPUT_FILE"

  done < <(echo "$WORKSPACE_DEPS" | jq -c '.[]')

done <<< "$WORKSPACE_MEMBER_NAMES"

# ---------------------------------------------------------------------------
# 5. Observations placeholder
# ---------------------------------------------------------------------------
{
  echo "---"
  echo ""
  echo "## Observations"
  echo ""
  echo "_(To be filled in after reviewing the report above.)_"
  echo ""
  echo "### Known thin dependencies (pre-existing)"
  echo ""
  echo "- \`torrust-tracker-clock\` → \`torrust-tracker-primitives\`: only"
  echo "  \`DurationSinceUnixEpoch\` imported. Addressed by SI-02."
  echo "- \`torrust-tracker-configuration\` → \`torrust-tracker-clock\`: only"
  echo "  \`DEFAULT_TIMEOUT\` imported. Addressed by SI-03."
  echo ""
  echo "### New findings"
  echo ""
  echo "_(Record any new thin-dependency or cluster-dependency findings here, with a"
  echo "reference to the subissue opened for each.)_"
  echo ""
} >> "$OUTPUT_FILE"

echo "Done." >&2
echo "Report: $OUTPUT_FILE" >&2
