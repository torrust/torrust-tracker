---
name: create-adr
description: Guide for creating Architectural Decision Records (ADRs) in the torrust-tracker project. Covers the timestamp-based file naming convention, free-form structure, index registration in the docs/adrs/README.md index table, and commit workflow. Use when documenting architectural decisions, recording design choices, or adding decision records. Triggers on "create ADR", "add ADR", "new decision record", "architectural decision", "document decision", or "add decision".
metadata:
  author: torrust
  version: "1.0"
---

# Creating Architectural Decision Records

## Quick Reference

```bash
# 1. Generate the filename prefix
date -u +"%Y%m%d%H%M%S"
# e.g. 20241115093012

# 2. Create the ADR file
# Format: YYYYMMDDHHMMSS_snake_case_title.md
touch docs/adrs/20241115093012_your_decision_title.md

# 3. Update the index
# Add entry to docs/adrs/index.md

# 4. Validate and commit
linter markdown
linter cspell
git commit -S -m "docs(adrs): add ADR for {short description}"
```

## When to Create an ADR

Create an ADR when making a decision that:

- Affects the project's architecture or design patterns
- Chooses one approach over alternatives that were considered
- Has consequences worth documenting for future contributors
- Answers "why was this done this way?"

Do **not** create an ADR for trivial implementation choices or style preferences covered by linting.

## File Naming Convention

**Format**: `YYYYMMDDHHMMSS_snake_case_title.md`

Generate the timestamp prefix:

```bash
date -u +"%Y%m%d%H%M%S"
```

**Examples**:

- `20240227164834_use_plural_for_modules_containing_collections.md`
- `20241115093012_adopt_axum_for_http_server.md`

Location: `docs/adrs/`

## ADR Structure

There is no rigid template — derive structure from context. Use
[docs/templates/ADR.md](../../../docs/templates/ADR.md) as a starting point.

Optional sections to add when relevant:

- **Alternatives Considered**: other options explored and why they were rejected
- **Consequences**: positive and negative effects of the decision

## Step-by-Step Process

### Step 1: Generate Filename

```bash
PREFIX=$(date -u +"%Y%m%d%H%M%S")
TITLE="your_decision_title"   # snake_case
echo "docs/adrs/${PREFIX}_${TITLE}.md"
```

### Step 2: Write the ADR

- **Description**: Explain the problem thoroughly — enough context for future contributors
- **Agreement**: State clearly what was decided and why
- **Date**: Today's date (`date -u +"%Y-%m-%d"`)
- **References**: Issues, PRs, external docs

### Step 3: Update the Index

Add a row to the index table in `docs/adrs/index.md`:

```markdown
| [YYYYMMDDHHMMSS](YYYYMMDDHHMMSS_your_title.md) | YYYY-MM-DD | Short Title | One-sentence description. |
```

- The first column links to the ADR file using the timestamp as display text.
- The short description should allow a reader to understand the decision without opening the file.

### Step 4: Validate and Commit

```bash
linter markdown
linter cspell
linter all   # full check

git add docs/adrs/
git commit -S -m "docs(adrs): add ADR for {short description}"
git push {your-fork-remote} {branch}
```

## Example ADR

For a real example, see
[20240227164834_use_plural_for_modules_containing_collections.md](../../../docs/adrs/20240227164834_use_plural_for_modules_containing_collections.md).
