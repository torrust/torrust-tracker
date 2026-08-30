---
name: create-adr
description: Guide for creating Architectural Decision Records (ADRs) in the torrust-tracker project. Covers decision-scope placement, timestamp-based names, free-form structure, collection-specific index registration, and commit workflow. Use when documenting architectural decisions, recording design choices, or adding decision records. Triggers on "create ADR", "add ADR", "new decision record", "architectural decision", "document decision", or "add decision".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/templates/ADR.md
---

# Creating Architectural Decision Records

## Quick Reference

```bash
# 1. Generate the filename prefix
date -u +"%Y%m%d%H%M%S"
# e.g. 20241115093012

# 2. Choose the ADR collection by decision scope
# Repository-wide, multi-package, and inter-package: docs/adrs/
# Package-owned and extractable: packages/<package>/docs/adrs/

# 3. Create the ADR file
# Format: YYYYMMDDHHMMSS_snake_case_title.md
# Root ADR:
touch docs/adrs/20241115093012_your_decision_title.md
# Package-local ADR:
touch packages/<package>/docs/adrs/20241115093012_your_decision_title.md

# 4. Update the owning collection's index
# Add a root ADR to docs/adrs/index.md; add a local ADR only to its local index

# 5. Validate and commit
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

## ADR Placement

Choose the collection according to the scope of the decision, not the paths changed by the
implementation:

| Decision scope                                            | Location                        |
| --------------------------------------------------------- | ------------------------------- |
| Repository-wide, multi-package, or inter-package contract | `docs/adrs/`                    |
| Solely owned by an extractable package                    | `packages/<package>/docs/adrs/` |

Shared configuration, protocol behavior, dependency policy, workspace conventions, and other
cross-package contracts require a root ADR even if one package contains all immediate code changes.

Every package-local collection needs a `README.md` and an `index.md`. Register an ADR only in the
index for its owning collection; root indexes do not duplicate local entries. When a package-local
decision becomes repository-wide, create a root ADR that links to and supersedes the local ADR,
then retain the local ADR and its local index entry as historical context.

The tracker-client CLI I/O ADR and the later root global CLI output ADR demonstrate this
local-placement and root-supersession pattern.

## ADR Structure

There is no rigid template — derive structure from context. Use
[docs/templates/ADR.md](../../../docs/templates/ADR.md) as a starting point.

Optional sections to add when relevant:

- **Alternatives Considered**: other options explored and why they were rejected
- **Consequences**: positive and negative effects of the decision

### ADR Status

Do **not** add a `- Status:` header by default. An ADR merged into `develop` or `main` is
implicitly accepted — the PR review process is the acceptance gate.

Only add a `- Status:` header for special terminal states:

- `- Status: Superseded by [ADR link]` — this decision has been replaced by a newer ADR.
- Additional states (e.g. `Deprecated`) may be introduced as needed.

## Step-by-Step Process

### Step 1: Generate Filename

```bash
PREFIX=$(date -u +"%Y%m%d%H%M%S")
TITLE="your_decision_title"   # snake_case
echo "docs/adrs/${PREFIX}_${TITLE}.md"  # Or packages/<package>/docs/adrs/ for a local decision.
```

### Step 2: Write the ADR

- **Scope**: State whether the decision is root or package-local and why
- **Description**: Explain the problem thoroughly — enough context for future contributors
- **Agreement**: State clearly what was decided and why
- **Date**: Today's date (`date -u +"%Y-%m-%d"`)
- **References**: Issues, PRs, external docs

### Step 3: Update the Index

Add a row to the selected collection's `index.md` table:

```markdown
| [YYYYMMDDHHMMSS](YYYYMMDDHHMMSS_your_title.md) | YYYY-MM-DD | Short Title | One-sentence description. |
```

- The first column links to the ADR file using the timestamp as display text.
- The short description should allow a reader to understand the decision without opening the file.

### Step 3.5: Cross-link ADR and Affected Code

When an ADR affects a specific area of code, keep discovery bidirectional:

- Add a short "Affected Code" section in the ADR with links to key files
  (module entry points, traits, setup/wiring files).
- Add concise module-level doc comments in those code files pointing back to
  the ADR.

This keeps rationale discoverable whether a contributor starts from docs or
from code.

### Step 4: Validate and Commit

```bash
linter markdown
linter cspell
linter all   # full check

git add docs/adrs/  # Include a package-local ADR path instead when applicable.
git commit -S -m "docs(adrs): add ADR for {short description}"
git push {your-fork-remote} {branch}
```

If code comments were added to establish ADR links, include those files in the
same commit when practical.

## Example ADR

For a real example, see
[20240227164834_use_plural_for_modules_containing_collections.md](../../../docs/adrs/20240227164834_use_plural_for_modules_containing_collections.md).
