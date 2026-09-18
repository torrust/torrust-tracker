---
doc-type: issue
issue-type: task
status: planned
priority: p1
epic: 2264
github-issue: 2265
spec-path: docs/issues/open/2265-2264-inventory-markdown-frontmatter-contracts/ISSUE.md
branch: "2264-2003-refactor-semantic-link-conventions-spec"
related-pr: 2269
last-updated-utc: 2026-09-18 15:25
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md
    - docs/skills/semantic-skill-link-convention.md
    - docs/templates/ISSUE.md
    - docs/templates/EPIC.md
    - docs/AGENTS.md
    - docs/external-snapshots/open-knowledge-format/0.2/SPEC.md
    - docs/external-snapshots/open-knowledge-format/0.2/PROVENANCE.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/write-markdown-docs/SKILL.md
---

# Issue #2265 - Inventory and Resolve Markdown Frontmatter Contracts

**Parent EPIC:** #2264 - Refactor Semantic Link and Frontmatter Conventions

## Goal

Establish an evidence-based frontmatter contract for the repository by inventorying current
Markdown frontmatter producers and values, resolving conflicts between prose, templates, skills,
and actual documents, and defining the accepted input for the first typed implementation.

## Background

Frontmatter requirements currently live primarily in
`docs/skills/semantic-skill-link-convention.md`, but templates, skills, and documents have evolved
independently. Existing issue and EPIC specs use conflicting lifecycle values, field sets, date
representations, and scalar types. Encoding the prose directly would freeze contradictions rather
than create a reliable source of truth.

The later semantic-link work must not block basic metadata validation. This issue therefore defines
a deliberately narrow provisional v1 reference convention for frontmatter:

- `semantic-links.skill-links` contains skill names using the current lowercase, number, and hyphen
  convention;
- `semantic-links.related-artifacts` contains only the forms the current convention already
  sanctions: a repository-relative file or directory path, the `issue #<number>` form that ADRs use
  for specs whose path changes over their lifecycle, and the `review-finding:pr-<number>-<id>`
  typed reference;
- a document's own issue and EPIC relationships use dedicated metadata fields such as
  `github-issue` and `epic`;
- new typed forms such as `path:`, `adr:`, `rust-item:`, and `markdown-section:` are not introduced;
- the inventory records unknown tagged forms as evidence, but newly authored specs must not invent
  additional forms before the semantic-link model is approved.

This convention is a compatibility baseline, not the final semantic-link ontology. It also does not
cover the frontmatter of Agent Skill (`SKILL.md`) and agent-profile files, whose top-level schemas
are owned by external specifications; the inventory records only the repository-owned
`metadata.semantic-links` extension they carry.

## Scope

### In Scope

- Inventory frontmatter field names, value shapes, scalar types, and nesting across every tracked
  Markdown file that currently has frontmatter, including files under `.github/`.
- Record the current baseline counts (documents with frontmatter, documents without, per-field
  variant counts) so later rollout stages can measure migration progress.
- Inventory frontmatter producers and normative guidance, including templates, planning skills,
  lifecycle skills, the `cleanup-completed-issues` skill that rewrites frontmatter on archive, and
  repository instructions.
- Identify which frontmatter fields are consumed by existing scripts, tests, or skills, so the
  contract knows which fields are load-bearing and which are informational.
- Inventory every value form currently used in `semantic-links.skill-links` and
  `semantic-links.related-artifacts`.
- Classify each observed variation as canonical, accepted legacy, invalid, or unresolved.
- Define the universal envelope shared by Markdown documents with frontmatter.
- Define strict prospective profiles for issue and EPIC specifications.
- Identify the known document classes that need later strict profiles without requiring those
  profiles in this issue.
- Resolve lifecycle status vocabulary, timestamp representation, nullability, unknown fields,
  experimental fields, and YAML scalar coercion policy.
- Decide the enforcement mode per document location (for example `drafts/` and `open/` strict,
  `closed/` advisory or exempt) and the initial severity of each diagnostic category, so the
  successor knows what fails a check versus what warns.
- Decide how the contract itself is versioned (for example a `schema-version` field, a
  convention-document version, or implicit versioning through the Rust crate) before the successor
  encodes it.
- Select the generated schema format, dialect/version, intended consumers, and treatment of Rust
  invariants that the generated format cannot express.
- Define accepted and rejected examples for the Rust-model issue.
- Define compatibility and no-rewrite rules for historical documents.
- Compare the resulting contract with OKF v0.2 and recommend one explicit disposition: an OKF
  profile, an OKF-compatible projection, selective field adoption without conformance, or a
  separate Torrust format.
- Update the draft EPIC when inventory findings alter its subissue boundaries or dependencies.

### Out of Scope

- Implementing Rust types, schema generation, or a validator.
- Migrating existing documents to the selected contract.
- Defining new semantic-link relation or target types.
- Introducing typed path-reference syntax.
- Selecting the final automation runner, package layout, event contract, cache, or hook/CI policy
  owned by EPIC #2003.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None expected for the inventory itself. Escalate the canonical metadata model to
  an ADR only if review establishes a repository-wide architectural decision that should outlive
  the convention and issue specifications.

## Design and Ownership Review

No child processes, asynchronous I/O, network readiness, or reusable runtime fixtures are involved.
The issue owns evidence gathering and contract decisions. The successor Rust-model issue owns
executable representation and validation. EPIC #2003 owns final automation placement and
orchestration.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Inventory producers and normative rules | Map every frontmatter rule in current conventions, templates, skills, and instructions to its source and consumers. |
| T2 | TODO | Inventory repository usage | Produce `frontmatter-inventory.md` with fields, scalar types, values, document classes, reference forms, baseline counts, and the reproducible command. The command must use tools already required by the repository toolchain. |
| T3 | TODO | Classify conflicts | Record canonical, accepted-legacy, invalid, and unresolved variants with representative paths and rationale. |
| T4 | TODO | Evaluate OKF v0.2 compatibility | Compare type fields, lifecycle, identity, reserved filenames, frontmatter presence, links, provenance, trust, freshness, versioning, and extension behavior; recommend a concrete disposition. |
| T5 | TODO | Specify the v1 contract | Create `frontmatter-v1-contract.md` defining the universal envelope, strict issue/EPIC profiles, provisional reference union, unknown-field policy, lifecycle rules, per-location enforcement mode, diagnostic severities, contract versioning, generated schema format/dialect and consumers, and the maintained compatibility baseline. |
| T6 | TODO | Define implementation fixtures | Create `frontmatter-fixtures/` with accepted and rejected Markdown/YAML examples plus a manifest of expected diagnostic categories for the successor issue. |
| T7 | TODO | Review handoff | Reconcile the resulting contract with the parent EPIC and obtain maintainer approval before Rust implementation begins. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T3 | Current-state inventory and conflict classification | Commit after the inventory is reproducible and representative samples are manually checked. |
| T4-T7 | OKF disposition, approved v1 contract, fixtures, and successor handoff | Commit after maintainer review and documentation validation. |

Record a justified no-change decision without creating an empty commit. Use signed Conventional
Commits for each independently reviewed change set.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/inventory-markdown-frontmatter-contracts/ISSUE.md`
- [x] Parent EPIC received GitHub issue #2264 and this specification received `epic: 2264`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2265 created and linked as a sub-issue of #2264
- [x] Specification folder moved to `docs/issues/open/2265-2264-inventory-markdown-frontmatter-contracts/`
  and open-state metadata plus live references updated
- [ ] Planning/evidence PR opened and `related-pr` updated
- [ ] Planning/evidence PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-18 11:05 UTC - GitHub Copilot - Drafted the frontmatter inventory and contract-resolution
  issue as the first child of the unnumbered semantic-link/frontmatter EPIC - This specification
- 2026-09-18 12:40 UTC - GitHub Copilot - Review pass: added `issue #<number>` and directory paths
  to the provisional reference union, scoped `SKILL.md`/agent frontmatter to the repository-owned
  extension, and added baseline counts, load-bearing field discovery, per-location enforcement mode,
  severity, and contract versioning to the deliverables - This specification
- 2026-09-18 12:50 UTC - GitHub Operator - Created issue #2265 as a native child of #2264 and
  promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2265

## Acceptance Criteria

- [ ] AC1: Every current frontmatter producer and normative source is included in a reviewed
      responsibility inventory.
- [ ] AC2: Every tracked Markdown frontmatter field and every current `skill-links` and
      `related-artifacts` value form is represented in the inventory.
- [ ] AC3: Observed variations are classified as canonical, accepted legacy, invalid, or unresolved
      with evidence and rationale.
- [ ] AC4: The approved v1 contract defines the universal envelope and strict prospective issue and
      EPIC profiles, including field presence, scalar types, nullability, and allowed values.
- [ ] AC5: The approved contract defines lifecycle, timestamp, unknown-field, experimental-field,
      YAML coercion, and historical-document compatibility policies.
- [ ] AC6: The provisional reference model accepts repository-relative file and directory paths,
      `issue #<number>` references, and `review-finding:` references in `related-artifacts`, and
      skill names in `skill-links`, without introducing new reference forms.
- [ ] AC7: Accepted and rejected fixtures define expected diagnostic categories for the successor
      Rust-model issue.
- [ ] AC8: The parent EPIC and successor draft are reconciled with the approved findings.
- [ ] AC9: The OKF v0.2 comparison records whether Torrust adopts a profile, exposes a compatible
  projection, selectively adopts fields without conformance, or remains separate, with material
  differences and migration consequences documented.
- [ ] AC10: The contract states the enforcement mode per document location, the initial severity
  of each diagnostic category, how the contract is versioned, and the generated schema format,
  dialect/version, intended consumers, and expressiveness boundary.
- [ ] `linter all` exits with code `0`.
- [ ] Manual verification scenarios are executed and documented in issue-local
      `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual results.

## Verification Plan

### Automatic Checks

- Re-run the documented inventory command or maintained inventory test selected during T2.
- `linter markdown`
- `linter cspell`
- `linter lychee`
- `git diff --check`
- `linter all` before completion

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Trace representative profiles | Compare one draft issue, open issue, closed issue, EPIC, ADR, skill, agent, evidence record, and general document against the inventory. | Every observed field and scalar form appears in the inventory with the correct source path and classification. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Review provisional references | Inspect representative file paths, directory paths, skill names, `issue #<number>` values, `review-finding:` values, and unknown tagged values. | Each value is assigned to the provisional v1 union, a dedicated field, legacy evidence, or an unresolved decision without inventing a new relation type. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Walk accepted and rejected examples | Review each fixture with the maintainer and explain the expected structural or semantic result. | The examples are sufficient to implement the Rust model without guessing field types or compatibility behavior. | TODO | `manual-verification-evidence.md` section V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1-AC3 | TODO | `frontmatter-inventory.md` and classification sections |
| AC4-AC6 | TODO | `frontmatter-v1-contract.md` |
| AC7 | TODO | `frontmatter-fixtures/` and expectation manifest |
| AC8 | TODO | Updated parent EPIC and successor draft |
| AC9 | TODO | OKF v0.2 compatibility matrix and recommendation |
| AC10 | TODO | Enforcement-mode, severity, and versioning sections of the v1 contract |

## Risks and Trade-offs

- A broad inventory can become unbounded. Record distinct shapes and controlling producers rather
  than manually documenting every identical file.
- Existing usage does not automatically define correct behavior. Classification requires explicit
  rationale rather than majority voting by occurrence count.
- The provisional reference union may preserve awkward legacy syntax. Keep it narrow and versioned
  so the later semantic-link subissue can replace it deliberately.
- Strict prospective profiles may expose a migration backlog. Record that backlog separately and do
  not rewrite historical documents in this issue.
- Frontmatter under `.github/skills/` and `.github/agents/` follows externally owned schemas.
  Constraining their top-level keys here would couple the repository to those specifications;
  limit the contract to the repository-owned `metadata.semantic-links` extension.

## Implementation Completion Review

After implementation, record material contract discoveries and invalidated assumptions in
`implementation-retrospective.md`. If the inventory only confirms the planned categories, add a
progress-log entry explaining why a separate retrospective is unnecessary.

## References

- Parent EPIC: `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md`
- Parent automation EPIC: #2003
- Successor specification: `docs/issues/open/2266-2264-implement-rust-frontmatter-model-and-validator/ISSUE.md`
- Pinned external design input: [Open Knowledge Format v0.2 specification](../../../external-snapshots/open-knowledge-format/0.2/SPEC.md)
- Snapshot provenance and update policy: [OKF 0.2 provenance](../../../external-snapshots/open-knowledge-format/0.2/PROVENANCE.md)
- Current upstream source: <https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md>
