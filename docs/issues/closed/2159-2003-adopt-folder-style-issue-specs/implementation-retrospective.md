<!-- markdownlint-disable MD003 -->
semantic-links:
  skill-links:
    - docs/issues/closed/2159-2003-adopt-folder-style-issue-specs/ISSUE.md
    - docs/issues/closed/2159-2003-adopt-folder-style-issue-specs/migration-inventory.md
    - docs/issues/closed/2159-2003-adopt-folder-style-issue-specs/ISSUE.md
    - docs/issues/closed/2159-2003-adopt-folder-style-issue-specs/migration-inventory.md
    - docs/adrs/20260918093757_adopt_folder_style_documentation_artifact_records.md
---

# Implementation Retrospective

## Discovery

Moving a record into a folder changes the base directory of each relative Markdown link. Bulk link repair
must therefore be restricted to the selected migration batch. A broad rewrite also changes
already-folder-style records and can introduce invalid paths.

## Outcome

Each archive batch used `git mv`, explicit primary filenames, and focused link validation. The
final full check confirmed that all selected durable-record families have zero flat primary
records and that local Markdown links resolve.

## Follow-Up

No migration tool is required. The recorded batch process is sufficient for the current one-time
archive migration; any future record-family migration should inventory source paths and validate
each bounded batch before committing.

## Test Rationale

The tracked shell audit-contract test was replaced with the dependency-free Rust workspace check
`agent-review-report-contract`. It arranges the repository-relative artifact and contract-text
pairs, reads each target file, and asserts the required presence or absence of each text contract.
This preserves focused workflow validation while complying with the repository rule that tracked
test code is Rust.

The prose-first comparison names the former eight shell scenarios as the Rust check's contract:
template structure, reviewer edit access, report persistence, Committer authority, PR-review
routing and redirects, audit schema, finding details, and portable finding references. The
constant assertion sets arrange each expected fact, file reads act on the named artifact, and the
failure collection asserts every result before returning a nonzero exit code.

The Rust check keeps Arrange, Act, Assert visible in its structure: the constant requirement sets
name the expected initial contract, `contains` and `contains_normalized` read the named artifacts,
and the collected failures state each assertion outcome. Its scenarios cover the report template,
reviewer edit access, report persistence policy, Committer authority, PR-review routing and
compatibility redirects, audit schema, finding details, and portable finding references.

## Prose-First Test Comparison

**Arrange:** the constant requirement sets name every expected or forbidden text and each reviewer
profile that must have `edit` access across the eight former shell scenarios.

**Act:** `contains`, `contains_normalized`, and `has_edit_tool` read the specified repository
artifacts and evaluate the corresponding contract.

**Assert:** `main` collects a failure for each unmet contract, emits all failures, and returns a
nonzero exit code when any assertion fails. The final Rust check maps these phases directly to its
requirement sets, file-read helpers, and `failures` collection.

The retired routing scenario arranged the handler, prompt, both compatibility redirects, both
helper skills, the process skill, and the orchestration record. The Rust `verify_review_routing`
function names those same artifacts, applies the same required and forbidden text checks, and
adds each mismatch to `failures`. The retired process-skill metadata checks inspected only the
frontmatter `related-artifacts` list; Rust now uses `has_related_artifact` to preserve that same
boundary rather than searching the entire file.

| Retired shell scenario | Rust replacement |
| ---------------------- | ---------------- |
| Reusable report template contract | `REQUIRED_TEXT` template entries and `require_frontmatter` |
| Reviewer edit access | `EDIT_CAPABLE_REVIEWERS` and `has_edit_tool` |
| Create, append, or skip policy | `verify_persistence_policy` loops over every reviewer |
| Committer authority | `verify_committer_authority` separates the auditor from Task and PR reviewers |
| PR-review routing | `verify_review_routing` checks the handler, prompt, helpers, redirects, workflow, and orchestration |
| New-audit analysis fields | `verify_audit_schema` checks template and workflow requirements |
| Tracking and finding details | `verify_finding_details` checks the compact row and narrative detail contract |
| Portable finding references | `verify_portable_references` checks convention, template, and workflow rules |

`contains_normalized` deliberately replaces only newline characters with spaces, exactly matching
the retired shell helper's `tr '\n' ' '` behavior. It does not collapse tabs or repeated spaces.
