---
doc-type: frontmatter-inventory
status: completed
github-issue: 2265
spec-path: docs/issues/open/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-inventory.md
last-updated-utc: "2026-09-19 12:10"
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - issue #2265
    - docs/skills/semantic-skill-link-convention.md
    - docs/templates/ISSUE.md
    - docs/templates/EPIC.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/cleanup-completed-issues/SKILL.md
    - contrib/dev-tools/checks/agent-review-report-contract/src/main.rs
---

# Frontmatter Inventory

This inventory records the tracked repository state at commit `092de794`, before the v1 contract
selects prospective rules. Counts are evidence, not a majority-vote definition of correctness.

## Reproduction

The inventory uses only Git and POSIX shell tools already needed by the repository workflow. Run
the commands from the repository root on the commit being inventoried.

```sh
git ls-files '*.md' | while IFS= read -r file; do
    [[ $(sed -n '1p' "$file") == '---' ]] && printf '%s\n' "$file"
done | wc -l

git ls-files '*.md' | while IFS= read -r file; do
    [[ $(sed -n '1p' "$file") == '---' ]] || continue
    awk 'NR == 1 { next } /^---$/ { exit } /^[^[:space:]#][^:]*:/ { sub(/:.*/, ""); print }' "$file"
done | sort | uniq -c | sort -nr

git ls-files '*.md' | while IFS= read -r file; do
    [[ $(sed -n '1p' "$file") == '---' ]] || continue
    awk 'NR == 1 { next } /^---$/ { exit } /^status:[[:space:]]*/ { sub(/^status:[[:space:]]*/, ""); print }' "$file"
done | sort | uniq -c | sort -nr
```

The first command returns the frontmatter count. Compare it with `git ls-files '*.md' | wc -l`
to determine the number without frontmatter. The remaining commands list top-level keys and status
values. Nested lists require profile-aware YAML parsing and are therefore recorded below from the
same pinned tree rather than inferred from the key-only command.

## Baseline

| Measure | Count | Interpretation |
| --- | ---: | --- |
| Tracked Markdown files | 740 | Complete tracked corpus, including `.github/` and package documentation. |
| Files beginning with frontmatter | 611 | The validator must parse present frontmatter wherever it occurs. |
| Files without frontmatter | 129 | Frontmatter cannot be universally required in v1. |
| `semantic-links` top-level key | 520 | The most common repository-owned extension. |
| `doc-type` top-level key | 301 | Only a subset of frontmatter-bearing documents has a typed profile indicator. |
| Issue `doc-type` | 212 | The largest prospective strict profile. |
| EPIC `doc-type` | 12 | The second prospective strict profile. |

The issue-location sample contains 31 frontmatter-bearing draft documents, 48 open documents, and
276 closed documents. Closed issue records use at least thirteen observed status values, so their
existing metadata is evidence for compatibility rules rather than a strict v1 input profile.

## Producers And Normative Sources

| Source | Role | Current rule or mutation | Contract consequence |
| --- | --- | --- | --- |
| `docs/skills/semantic-skill-link-convention.md` | Normative convention | Requires metadata for new or updated issue and EPIC specs; defines `skill-links` and most `related-artifacts` forms. | Primary legacy rule; supersede its profile table with the approved v1 contract. |
| `docs/templates/ISSUE.md` | Issue producer | Emits `epic`, nullable issue/PR fields, `branch`, and a draft timestamp placeholder. | `epic` is prospective issue metadata despite being absent from the convention's required-field table. |
| `docs/templates/EPIC.md` | EPIC producer | Emits the EPIC profile and nullable ownership/issue fields. | Defines the minimum prospective EPIC shape. |
| `.github/skills/dev/planning/create-issue/SKILL.md` | Issue lifecycle producer | Requires metadata including `status`, `epic`, `github-issue`, `spec-path`, and `last-updated-utc`; moves draft specs into open folders. | Confirms the issue lifecycle owns location/path invariants. |
| `.github/skills/dev/planning/cleanup-completed-issues/SKILL.md` | Archive mutator | Moves open specs to closed and rewrites `status`, `spec-path`, and `last-updated-utc`. | Closed files must be checked under a compatibility mode, not rewritten wholesale. |
| `docs/AGENTS.md` and `.github/skills/dev/planning/write-markdown-docs/SKILL.md` | Normative guidance | Describe issue/EPIC frontmatter as required and other document classes as recommended or optional. | Supports strict known profiles with permissive unknown classes. |
| `.github/skills/*/SKILL.md` and `.github/agents/*.agent.md` | External-schema producers | Own their top-level Agent Skills or agent-profile metadata. | v1 validates only the repository-owned `metadata.semantic-links` extension when present. |

## Existing Consumers

| Consumer | Fields or syntax used | Load-bearing behavior |
| --- | --- | --- |
| `contrib/dev-tools/checks/agent-review-report-contract/src/main.rs` | Opening/closing delimiters and selected `related-artifacts` list items | Requires valid-looking frontmatter for four named review artifacts and exact related-artifact entries. It does not parse YAML or enforce a general schema. |
| `.github/skills/dev/planning/cleanup-completed-issues/SKILL.md` | `status`, `spec-path`, `last-updated-utc`, `semantic-links.related-artifacts` | Prescribes archive-time rewrites and live-reference repair. |
| `.github/skills/dev/planning/create-issue/SKILL.md` | Issue/EPIC metadata and `branch` | Prescribes draft/open lifecycle updates. |
| Markdown/skill guidance | `semantic-links` | Human and agent discovery guidance only; no repository-wide executable consumer exists yet. |

## Observed Shapes

### Top-Level Keys

The recurring issue/EPIC field family is `doc-type`, `status`, `github-issue`, `spec-path`, and
`last-updated-utc`; issue documents additionally commonly carry `issue-type`, `priority`,
`branch`, `related-pr`, and `epic`. Other recurring families belong to Agent Skills (`name`,
`description`, `metadata`, `argument-hint`) and review/security/evidence documents (`issue`,
`package`, `target-file`, `source`, date fields, and domain-specific fields).

Observed `doc-type` values include 212 `issue`, 12 `epic`, 23 test-refactor-plan variants, 12
manual-verification evidence records, and numerous one-off evidence, analysis, research, and
security values. These are known document classes that need future profiles; v1 must not reject
them as unknown.

### Scalar Forms

| Field | Observed forms | Classification |
| --- | --- | --- |
| `github-issue`, `epic`, `related-pr` | YAML integers and `null` | Canonical for strict profiles. |
| `last-updated-utc` | `YYYY-MM-DD HH:MM` (151), date-only (98), `null` (28), template placeholder (4) | Date-time is prospective canonical; date-only and null are legacy evidence. |
| `status` | Issue lifecycle values, review states, security dispositions, and placeholders | Profile-specific; not a universal enum. |
| `semantic-links` | Mapping with optional `skill-links` and `related-artifacts`; a small number of inline YAML lists | Mapping is canonical; both YAML list spellings are structurally valid. |

### Lifecycle Values

Issue/EPIC lifecycle values observed in draft/open records are `draft`, `open`, `planned`,
`in-progress`, `blocked`, `in-review`, and `complete`. Closed records additionally contain
historical values such as `done`, `completed`, `closed`, `resolved`, `deferred`, and legacy spelling
`in_progress`. Security and review documents use unrelated status vocabularies such as
`non-affecting`, `superseded`, and `preliminary-not-a-legal-opinion`.

The v1 contract must therefore give issue and EPIC profiles their own prospective lifecycle enum.
It must not apply that enum to every document with a `status` key.

### Semantic Links

`skill-links` values observed in block-list form all conform to lowercase letters, digits, and
hyphens. The documented and observed `related-artifacts` forms are:

| Form | Examples | v1 disposition |
| --- | --- | --- |
| Repository-relative file or directory path | `docs/AGENTS.md`, `Cargo.toml`, `contrib/dev-tools/experiments/` | Canonical. Root-level paths are valid paths, not an exception. |
| Stable issue reference | `issue #2264` | Canonical for a related issue whose specification path can move. |
| Review finding reference | `review-finding:pr-2230-f1` | Canonical. |
| External URL | `https://github.com/dbrgn/tracing-test/issues/23` | Invalid for newly authored `related-artifacts`; retain as legacy evidence. |
| Quoted GitHub URL | `"https://github.com/torrust/torrust-tracker/issues/1488"` | Invalid for newly authored `related-artifacts`; retain as legacy evidence. |
| Bare non-path text | `manual-verification.md`, `issue torrust/torrust-tracker-deployer`, numeric identifiers | Invalid for newly authored `related-artifacts`; retain as legacy evidence. |

The inventory found 2,473 path-shaped entries, 38 `issue #<number>` entries, 8 review-finding
entries, and 140 entries outside that provisional union. The count treats root-level paths as valid
repository-relative paths and deliberately separates external/bare legacy strings from the approved
v1 union.

## Conflict Classification

| Variation | Classification | Rationale |
| --- | --- | --- |
| `epic` on issue specs | Canonical prospective | The issue template and creation skill produce it; active issue specifications use it. |
| Issue/EPIC profiles without `epic` in the older convention table | Invalid normative guidance | The table omits a field produced by current canonical templates. |
| Date-time `last-updated-utc` | Canonical prospective | Both issue and EPIC templates prescribe it and lifecycle skills rewrite it. |
| Date-only or `null` timestamps | Accepted legacy | Widely present in older evidence and research records; not adequate for newly authored strict issue/EPIC specs. |
| `done`, `completed`, and `closed` on historical issue records | Accepted legacy | Archive history predates a unified lifecycle vocabulary. |
| `in_progress` | Invalid prospective | Conflicts with the convention's hyphenated `in-progress` form; preserve historical documents. |
| Profile-specific review/security statuses | Accepted outside issue/EPIC profiles | They are meaningful within their own document families and cannot be judged by issue lifecycle rules. |
| Unknown top-level keys on unknown document classes | Unresolved for universal envelope | V1 needs a preservation and severity policy, not a universal closed-world schema. |
| `schema-version` | Unresolved | No observed field establishes an existing per-document versioning practice; decide from v1 migration requirements. |
| Historical location enforcement | Unresolved | Closed records demonstrate migration debt; decide whether advisory diagnostics or exemption better preserves evidence while enabling visibility. |

## Resulting Constraints For The V1 Contract

1. Parse every present frontmatter block, but do not require frontmatter on all Markdown files.
2. Apply strict prospective profiles only to issue and EPIC specifications; defer profiles for ADRs,
   skills, agents, review, security, research, and evidence records.
3. Keep `semantic-links` as the repository-owned universal extension, with the frozen v1 reference
   union documented above.
4. Treat unknown document types and fields permissively until a profile owns them.
5. Preserve historical documents without mass rewrite; the contract must explicitly choose advisory
   or exempt enforcement after weighing useful diagnostics against historical fidelity.
6. Select contract versioning deliberately; the inventory provides no basis for treating a new
  field as legacy-compatible by default.
