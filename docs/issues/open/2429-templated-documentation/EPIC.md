---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2429
spec-path: docs/issues/open/2429-templated-documentation/EPIC.md
epic-owner: null
last-updated-utc: "2026-10-03 14:53"
semantic-links:
  skill-links:
    - create-issue
    - create-markdown-template
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-markdown-template/SKILL.md
    - docs/templates
    - docs/index.md
    - docs/adrs/index.md
    - docs/skills/semantic-skill-link-convention.md
    - docs/schemas/frontmatter-v1.schema.json
    - contrib/dev-tools/checks/frontmatter-validator
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - "issue #2003"
    - "issue #2264"
    - "issue #2278"
---

<!-- skill-link: create-issue -->

# EPIC #2429 - Templated Documentation

## Goal

Make every fact a Markdown document restates from elsewhere a rendering of committed data, so the tree alone reproduces each document and a gate fails when a restated fact goes stale. A template holds a document's prose and shape, committed datasets hold every copied fact, and the committed rendering stays plain Markdown.

The tree already shows the failure this removes. `docs/adrs/index.md` lists 28 of the 29 root ADRs: `docs/adrs/20260612000000_adopt_sccache_for_ci_bare_builds.md` has no row, and the table steps from `20260603000000` at `docs/adrs/index.md:26` to `20260617093046` at `docs/adrs/index.md:27`. Nothing flagged it: the index is a hand-made copy of a fact the tree holds.

## Why This Is Needed

Copies drift: a restated list, count or quoted line is right when written and wrong once its source changes, and no check compares the two. More evidence at `develop` `eb96d2957`:

- The template table at `docs/index.md:158` lists 12 of the 14 templates (not `MANUAL-VERIFICATION-EVIDENCE.md` or `PR-REVIEW-RETROSPECTIVE.md`), though `.github/skills/dev/planning/create-markdown-template/SKILL.md:58` has each entered there, and in `docs/templates/README.md`, by hand.
- `docs/skills/semantic-skill-link-convention.md:153` lists the EPIC fields without `schema-version` or `epic` and admits the status `open`, against the validator's EPIC profile at `contrib/dev-tools/checks/frontmatter-validator/src/profile.rs:197`.
- EPIC subissue tables copy each subissue's GitHub state into a hand-kept `Status` column.

The public friction register on EPIC #2003 tracks these drift classes.

One artifact already works this way: `docs/schemas/frontmatter-v1.schema.json` is generated from the Rust model (`contrib/dev-tools/checks/frontmatter-validator/src/profile.rs:49`), and a `check` mode fails when the committed bytes differ from a fresh generation.

At `develop` `eb96d2957` the tree tracks 970 Markdown files, among them 272 primary specifications (`ISSUE.md` or `EPIC.md` under `docs/issues/`), 143 PR-review audit files, 46 skills, 33 ADRs (29 at the root) and 14 templates: each a count that documents quote and the next change makes wrong.

## Scope

### In Scope

- Everything stated in Design.
- Progressive conversion of the document classes, most important documents first.

### Out of Scope

- The shared runner, cache, CI integration and execution architecture (EPIC #2003).
- Frontmatter profiles, the semantic-link model and path-reference syntax (EPIC #2264).
- Rewriting closed or historical records.
- Reading the forge at render time; forge facts enter as committed, stamped captures.

## Design

This section is binding; subissues change it only through a recorded update to this EPIC.

### Four Stages, Each Committed and Gated

1. **Inputs.** Authored data (lists, catalogs, owner and placement tables) lives in hand-edited TOML whose schema the data generator validates first, so a broken edit fails there, not at a reader.
2. **Data generator.** A Rust program reads the inputs and the tree (frontmatter, paths, links, committed captures and check results) and emits JSON datasets; it alone derives facts (counts, classifications, name-to-path resolution). The tree census is a dataset like any other: generated, committed and proved current by the gate.
3. **Document generator.** It renders each template over the committed JSON only (never the TOML, the forge or the network), with render-time functions such as `require`, and compares with the committed rendering. A stale, missing or unpaired rendering fails; `write` refreshes it.
4. **Lint.** The tracker's own `linter` profile and the frontmatter validator run on the committed renderings; templates are linted for their prose under a profile that knows the delimiters.

Generator, renderer and Markdown formatters compose to a fixed point: on a clean tree, running all four stages changes nothing.

### The Editing Boundary

People edit the TOML inputs, the templates' prose, the data generator's code (where a derived fact is defined) and the render-time functions. Datasets and renderings are generated; the gate rejects a hand edit to either. A render template, re-rendered on every gate run, keeps a name distinct from the authoring templates that `.github/skills/dev/planning/create-markdown-template/SKILL.md:20` calls starting points, "not generators".

### One Envelope on Every Structured File

Every TOML input and JSON dataset declares itself before any payload is read:

- `namespace` then `version` are its first two top-level keys in source order; the file name alone selects the carrier.
- The namespace names the schema, never the instance: dot-separated, at least two lowercase alphanumeric atoms, hyphens only inside an atom, at most 255 bytes, shaped `com.torrust.tracker.<area>.<document>`.
- The version is `[major, minor, patch]` in shortest-spelled non-negative integers; a reader requires an equal major, accepts a minor at or above its floor, and treats patch as documentary.
- A reader accepts a file only after validating carrier, namespace, version relation and payload; an envelope failure is a refused precondition, not a repository finding.
- Generated datasets carry it too: the data generator writes it and the gate checks it.

The envelope owns namespace and version, each program owns its payload schema, no layer infers a missing fact from another, and a schema change is a version change.

### One Dataset Crate as the Only Shared Interface

One Rust crate owns every JSON dataset: a record type per namespace (the type is the schema, projected to JSON Schema as the frontmatter validator already does), the envelope type with its read-time version check, and the serde round trip. Serialization is deterministic (fixed key order and number spelling, one trailing newline), so a dataset diff is a data diff. The data generator writes through it, the document generator and every checker read through it, and no stage parses JSON alone.

The TOML input schema stays out: it is the data generator's private contract, and sharing it would make the document generator depend on inputs it never reads. Layering: TOML input schema (private) → dataset crate (the only shared interface) → document generator and checkers.

### Premises and Labels

A sentence whose truth depends on the data states its premise with `require`, so a dataset that no longer supports the prose fails the render; a quoted line states the bytes it quotes. Every rendered record mints exactly one label, a stable identifier derived from the record (an ADR identifier, an acceptance-criterion ID, a `review-finding:pr-<number>-<id>` reference), never typed by hand.

### Conversion Is an Editorial Pass

Converting a document to a template is a review of that document: each one is read, corrected and improved as it is converted, so a rendering that differs from the former text because the document got better is the intent, not a defect. Equivalence with the previous bytes is never a gate condition; the fixed-point gate applies to a converted document from its conversion onward.

## Document Classes

Conversion follows importance, not class.

- **Class 1, inventories.** `docs/index.md`, `docs/adrs/index.md`, `docs/issues/README.md`, `docs/packages.md`, `docs/templates/README.md`: renderings of the tree census.
- **Class 2, records.** ADRs, specifications, audit records, retrospectives, security and evidence records: frontmatter, tables, thread states and related lists render; prose stays authored. Forge facts come from captures stamped with their time, so a status is "as of" a named instant.
- **Class 3, prose guides.** Skills, agent profiles, `AGENTS.md` and guides: only what they cite renders (tables of paths, counts, quoted lines).

## Engineering Choices Left to Implementation

The subissue that meets each of these records its choice and reason: where inputs, datasets and templates live; how many templates, per package or per concept; a rendering as a second file or as generated regions; the dependency graph within and between packages; whether envelope, schema and namespace handling get their own crate; where the generators live and how the gate invokes them; the delimiters; the conversion order within a subissue; the migrated layout.

## Relationship to Existing EPICs

- **EPIC #2003** owns the execution model: the `check` modes are checks in its taxonomy and the `write` modes actions, placed by its dev-tool precedent until it selects the shared architecture.
- **EPIC #2264** owns what records render against; the validator has two strict profiles, issue and EPIC (`contrib/dev-tools/checks/frontmatter-validator/src/profile.rs:16`). Its rows "Extend strict profiles and author guidance", "Normalize the semantic-link model" and "Decide path-reference scope and syntax" supply the other record types, the citation targets and the status of prose paths.
- **EPIC #2278** owns the audit-record contract: its rows "Port the audit validator to Rust with parity fixtures" and "Extend the audit validator to the adopted invariants" fix the thread states and invariants the audit template states as premises.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue | Local Spec | Status | Notes |
| ----- | ----- | ---------- | ------ | ----- |
| 1 | #[To be assigned] - Add the dataset crate and the envelope | Not drafted | TODO | No dependencies. |
| 2 | #[To be assigned] - Add the data generator, its TOML input schema and the tree census | Not drafted | TODO | Depends on the dataset crate. |
| 3 | #[To be assigned] - Add the document generator and the staleness gate | Not drafted | TODO | Depends on the dataset crate. |
| 4 | #[To be assigned] - Convert the root ADRs | Not drafted | TODO | Each of the 29 becomes a template over the ADR census, reviewed and improved in its conversion; the ADR index renders from that census. Depends on both generators. |
| 5 | #[To be assigned] - Convert the specifications and audit records | Not drafted | TODO | Depends on the root ADRs and EPIC #2264's strict profiles; audit records also on EPIC #2278's validator rows. |
| 6 | #[To be assigned] - Convert the remaining inventories and the prose guides | Not drafted | TODO | Depends on the root ADRs; guides also on EPIC #2264's path-reference decision. |

## Delivery Strategy

The interface lands first; conversion then goes by importance. The root ADRs lead, and their census also renders `docs/adrs/index.md`, retiring its omission by construction; specifications and audit records follow EPIC #2264's profiles and EPIC #2278's validator rows; the remaining inventories and the prose guides come last. A conversion PR is reviewed as an improvement of its documents, not as a mechanical move. Each subissue merges through its own PR.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an
   issue-local retrospective for reusable lessons, material design changes, or
   meaningful deviations from the plan; otherwise record why one was unnecessary
   in the issue progress log.

### Phase 1: Interface and Generators

- Outcome and exit: a clean tree is a fixed point, and a mutated dataset or rendering fails the gate.

### Phase 2: Root ADRs

- Outcome and exit: every root ADR renders from its template, and `docs/adrs/index.md` from their census; adding an ADR without refreshing fails the gate.

### Phase 3: Specifications and Audit Records

- Outcome and exit: converted once their profiles and validator rows exist, or deferred with reasons.

### Phase 4: Remaining Inventories and Prose Guides

- Outcome and exit: converted, or deferred with reasons.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [ ] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [ ] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-03 13:16 UTC - Specification drafter - Drafted the EPIC: binding design, engineering choices left to subissues, six proposed subissues.
- 2026-10-03 13:56 UTC - Specification drafter - Revised after review: root ADRs convert first, and conversion is an editorial pass.
- 2026-10-03 14:02 UTC - Specification drafter - Restored the template's completion policy and workflow checkpoints verbatim, and the subissue status drift class.
- 2026-10-03 14:20 UTC - Specification drafter - Issue #2429 created; spec moved from drafts to open.

## Acceptance Criteria

- [ ] AC1: Only the dataset crate reads or writes datasets, byte-deterministically.
- [ ] AC2: Every input and dataset carries the envelope, checked before its payload is read.
- [ ] AC3: The TOML input schema is private to the data generator.
- [ ] AC4: The gate fails on a stale dataset or a stale, missing or unpaired rendering; `write` refreshes both.
- [ ] AC5: An unsupported `require` premise fails the render.
- [ ] AC6: All four stages on a clean tree change nothing.
- [ ] AC7: The root ADRs are converted, and `docs/adrs/index.md` renders from their census and lists every root ADR.
- [ ] AC8: Each engineering choice is recorded with its reason where it was made.
- [ ] AC9: Template guidance distinguishes render templates from authoring templates.
- [ ] AC10: Every completed subissue carries verification evidence and its reviews.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1-AC6 | TODO | Interface subissues, mutation evidence |
| AC7 | TODO | Root-ADR subissue |
| AC8-AC10 | TODO | Subissue specifications and records |

## Risks and Trade-offs

- A data change diffs twice. Mitigation: deterministic output keeps both diffs minimal.
- A template that computes a fact recreates drift. Mitigation: only the data generator derives facts.
- Hand edits to renderings. Mitigation: the gate fails and names the template and dataset to edit.
- An editorial pass changes a decision's meaning. Mitigation: each ADR's conversion is reviewed in its own PR.

## References

- Related issues: #2003, #2264, #2278
- Related PRs: none
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (the generators' output contract)
