---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 2264
github-issue: null
spec-path: docs/issues/drafts/2264-support-multiple-related-prs-in-issue-frontmatter/ISSUE.md
branch: "{issue-number}-2264-support-multiple-related-prs-in-issue-frontmatter"
related-pr: null
last-updated-utc: "2026-09-30 12:24"
semantic-links:
  skill-links:
    - create-issue
    - cleanup-completed-issues
  related-artifacts:
    - docs/schemas/frontmatter-v1.schema.json
    - docs/schemas/README.md
    - contrib/dev-tools/checks/frontmatter-validator
    - docs/templates/ISSUE.md
    - docs/skills/semantic-skill-link-convention.md
    - .github/skills/dev/planning/cleanup-completed-issues/SKILL.md
    - docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md
    - docs/issues/open/2375-2003-unambiguous-issue-spec-names/ISSUE.md
    - docs/issues/closed/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Support Multiple Related Pull Requests in Issue Frontmatter

## Goal

Let an issue or EPIC spec record every pull request that belongs to it, not only one. A spec
should show all its PRs, and what each one did, without anyone having to search GitHub.

## Background

The v1 frontmatter contract defines `related-pr` as a single nullable positive integer: `integer |
null` in `docs/schemas/frontmatter-v1.schema.json` and `Option<u64>` in the frontmatter validator's
issue profile. The field is required and may be `null`.

One issue often has more than one PR:

- **Spec-only PR, then implementation PR.** The issue workflow merges the spec before any code.
  Issue #2370 (EPIC #1488 SI-15) has spec-only PR #2372 and implementation PR #2382, so its
  frontmatter can record only one of them. On 2026-09-30 the maintainer chose the implementation
  PR, following the SI-14 precedent (`related-pr: 2351`), and asked for this issue.
- **Reopened issues.** An issue can be reopened and closed by a later PR, which a single value
  would overwrite.
- **Follow-up and review-fix PRs** that land after the main PR.

Repository evidence as of 2026-09-30 (`^related-pr:` in `docs/issues/*/*/{ISSUE,EPIC}.md`):

| Folder | `null` | Integer | Other form |
| --- | --- | --- | --- |
| `open/` | 14 | 5 | 0 |
| `closed/` | 100 | 76 | 11 |
| `drafts/` | 25 | 0 | 0 |

All 11 "other form" values are full PR URLs in closed specs, written before the v1 contract. They
still pass because strict-profile findings on closed specs are only warnings. Most specs leave the
field `null`, so a missing PR is common today.

## Scope

### In Scope

- Decide and implement a representation that records zero, one, or many PRs per issue or EPIC spec.
- Update the canonical Rust model and validator, then regenerate the schema and verify drift, as
  #2280 set up.
- Update `docs/templates/ISSUE.md`, `docs/schemas/README.md`, the semantic-link convention example,
  and the skills that write the field: `create-issue` when opening a PR, `cleanup-completed-issues`
  at close-out, and the open-pull-request workflow.
- Define compatibility for existing values, including the 11 URL values in `closed/`, according to
  the #2264 migration policy.

### Out of Scope

- Backfilling `null` values in historical specs from GitHub history.
- Changing how PRs link to issues on GitHub (`Closes #N`, `Related to #N`).
- The general semantic-link model and relation vocabulary (#2264 subissues 5-6); this issue may
  reuse it but must not preempt it.

## Decisions Deferred to EPIC #2264

1. **Shape.**
   - Widen `related-pr` to accept an integer or a list. Existing values stay valid, but every
     reader handles two forms.
   - Add a list field (for example `related-prs`) and retire `related-pr`. One form, but it needs a
     schema version bump and a migration.
2. **Roles.** Whether each entry says what the PR did (for example `spec`, `implementation`,
   `follow-up`), or the list stays plain numbers and the progress log explains roles. Add roles
   only if a check, query, or skill will use them.
3. **Order and uniqueness.** Whether the list is chronological, and whether duplicates are an error.
4. **Versioning.** Whether the change needs `schema-version: 2` under the #2264 convention
   versioning rules, or is an additive v1 change.
5. **URL values.** Whether the 11 closed-spec URL values are migrated, accepted as legacy, or left
   as advisory warnings.

## Expected Outcome

- A spec can record every related PR, and #2370 can list both #2372 and #2382.
- The Rust model, generated schema, template, docs, and skills agree on one contract.
- The frontmatter validator accepts the new form, rejects invalid entries with stable diagnostics,
  and handles existing values under the chosen compatibility rule.

## Verification Plan

- Validator fixtures cover each accepted form, and the invalid ones: non-positive numbers, wrong
  types, and duplicates if forbidden.
- Schema generation shows no drift after the model change.
- Running the validator on the whole tree reports no new errors in `open/` and `drafts/`.
- The pre-commit gate passes.
- One real spec, for example #2370 if it is still open, records multiple PRs in the new form.

## References

- EPIC #2264: `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md` (child of
  EPIC #2003)
- v1 contract: `docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-v1-contract.md`
- Validator implementation: `docs/issues/closed/2266-2264-implement-rust-frontmatter-model-and-validator/ISSUE.md`
- Motivating case: `docs/issues/closed/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md`
- Directory naming (tagged segments pending): `docs/issues/open/2375-2003-unambiguous-issue-spec-names/ISSUE.md`
