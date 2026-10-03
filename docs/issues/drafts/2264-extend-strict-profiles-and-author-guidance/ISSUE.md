---
schema-version: 1
doc-type: issue
issue-type: feature
status: draft
priority: p2
epic: 2264
github-issue: null
spec-path: docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md
branch: "{issue-number}-extend-strict-profiles-and-author-guidance-spec"
related-pr: null
last-updated-utc: "2026-10-03 11:31"
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
    - write-unit-test
    - create-adr
    - create-refactor-plan
  related-artifacts:
    - "issue #2264"
    - "issue #2266"
    - "issue #2280"
    - "issue #2281"
    - contrib/dev-tools/checks/frontmatter-validator
    - docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-v1-contract.md
    - docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-inventory.md
    - docs/schemas/frontmatter-v1.schema.json
    - docs/schemas/README.md
    - docs/skills/semantic-skill-link-convention.md
    - docs/templates
    - docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md
    - docs/issues/drafts/mine-pr-review-audit-records/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Extend Strict Profiles and Author Guidance

**Parent EPIC:** #2264 - Refactor Semantic Link and Frontmatter Conventions

**Predecessors:** [#2266 - Implement the Rust Frontmatter Model and Initial Validator](../../closed/2266-2264-implement-rust-frontmatter-model-and-validator/ISSUE.md), [#2280 - Generate V1 Schema and Verify Drift](../../closed/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md), and [#2281 - Add Frontmatter Validator Command and Pre-Commit Rollout](../../closed/2281-2264-frontmatter-validator-command/ISSUE.md)

## Goal

Extend the canonical Rust frontmatter model from the two strict v1 profiles (issue and EPIC) to approved profiles for the remaining known, template-produced document classes; give Agent Skills and agent profiles a strict profile for the repository-owned `metadata.semantic-links` extension; protect `docs/templates/` against drift from the contract; and make author guidance and editor/agent discovery derive from the canonical model instead of duplicating it.

## Background

The approved [v1 contract](../../closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-v1-contract.md) applies strict profiles only when `schema-version: 1` is present and `doc-type` is `issue` or `epic`. Its "Document-Type Coverage" table leaves every other class outside the strict boundary: Agent Skills and agent profiles are externally governed and inspected only through `metadata.semantic-links`; test and refactor plans, manual-verification records, and review, security, analysis, research, and other evidence records keep only the universal envelope ("Deferred"); unrecognized or one-off classes stay permissive. Its "Deferred Profiles" section assigns the profile definitions for ADRs, Agent Skills, agent profiles, refactor plans, review records, security analysis, research, manual verification evidence, and other evidence records to this EPIC's later profile work, which is this subissue.

The validator delivered by #2266, #2280, and #2281 enforces exactly that boundary today:

- `src/profile.rs` recognizes two strict kinds. `StrictProfileKind` has only `Issue` and `Epic`, `strict_document_type` dispatches on `schema-version: 1` plus `doc-type`, and a v1 record with any other `doc-type` stays `Profile::Permissive` (the unit test `it_should_keep_an_unknown_v1_document_type_permissive` pins this). Externally owned documents always return `Profile::Permissive`.
- The generated schema root `FrontmatterV1` is an untagged union of the `Issue` and `Epic` types, so `docs/schemas/frontmatter-v1.schema.json` describes only those two profiles to editors and agents.
- `src/lib.rs` checks only the shape of an external document's `metadata.semantic-links` (string sequences). The frozen v1 reference syntax (`StrictSemanticLinks`) and the repository-aware path and skill resolution in `src/repository.rs` apply only to strict issue and EPIC records.
- The command's `EXCLUDED_PREFIXES` in `src/bin/frontmatter-validator/discovery.rs` skips `docs/templates/` in every mode, because template placeholders cannot pass strict validation. Decision D6 of #2281 states the consequence: nothing detects template drift from the v1 contract, and this row owns that follow-up.

The prose that authors read already disagrees with the model. The "Markdown Frontmatter" section of `docs/skills/semantic-skill-link-convention.md` duplicates the issue and EPIC field lists. It omits `schema-version` and `epic`, lists an `open` status that the v1 lifecycle rejects, and shows an unquoted `last-updated-utc`. A spec written from that section fails the pre-commit validator. The EPIC's Representation Decision requires prose to refer to the canonical model rather than duplicate field lists that can drift.

Baseline measured at the base of this draft's branch on `develop` (2026-10-03), excluding `docs/templates/` and the crate fixtures where noted:

| Class or location | Tracked records | Frontmatter today |
| --- | --- | --- |
| ADR collections (`docs/adrs/`, `packages/*/docs/adrs/`, without README/index files) | 32 | `semantic-links` only; none declares a `doc-type` |
| Agent Skills (`SKILL.md`) | 46 | External schema; 11 carry `metadata.semantic-links` |
| Agent profiles (`*.agent.md`) | 10 | External schema; none carries `metadata.semantic-links` |
| `doc-type: manual-verification-evidence` | 33 | Template-produced; no `schema-version` |
| `doc-type: file-test-plan` / `test-refactor-plan` | 36 / 24 | No template in `docs/templates/` |
| `doc-type: refactor-plan` | 8 | Template-produced; no `schema-version` |
| Other `doc-type` values besides `issue` and `epic` | 26 families, 19 of them with one record | One-off evidence, analysis, research, and guidance classes |
| `schema-version: 1` records outside templates and fixtures | 54 issue, 11 EPIC | No other `doc-type` carries `schema-version: 1` |
| `docs/templates/` | 14 templates and a README | 4 declare a `doc-type` (`issue`, `epic`, `manual-verification-evidence`, `refactor-plan`); 2 declare `schema-version: 1` |

The last two rows matter for compatibility. No record outside issue and EPIC specs opts into `schema-version: 1`, so adding profiles under that version changes the outcome of no existing record. Two templates (`MANUAL-VERIFICATION-EVIDENCE.md` and `REFACTOR-PLAN.md`) write an unquoted `last-updated-utc` placeholder, which a strict profile with the v1 timestamp rule would reject.

## Scope

### In Scope

- An issue-local profile catalog (`profile-catalog.md`) that proposes, per candidate class, the producer, opt-in, fields, scalar types, allowed values, location, and severity, and that the maintainer approves before any profile is implemented.
- Canonical Rust types, strict definitions, accepted and rejected fixtures, and generated-schema coverage for every approved repository-owned profile.
- A strict extension profile for Agent Skills and agent profiles that validates only `metadata.semantic-links`, leaving their externally governed top-level metadata untouched.
- Template drift protection: validate `docs/templates/` in a placeholder-aware template mode instead of excluding it.
- Template updates so each profiled class's template emits `schema-version: 1`, its `doc-type`, and contract-conformant placeholders.
- Author guidance derived from the canonical model: replace duplicated field lists with references to the generated schema, and document each profile's opt-in, location, and severity in the crate README, `docs/schemas/README.md`, and the producing skills.
- A whole-tree baseline before and after the change, with any new errors in draft and open records triaged.

### Out of Scope

- Changing the shape of `related-pr` or any other existing issue/EPIC field. The multiple-related-PRs proposal is EPIC row 2.3; this issue neither depends on it nor preempts it.
- Moving normative content into new convention documents or retiring `docs/skills/semantic-skill-link-convention.md`; EPIC row 4 owns the split. This issue corrects duplicated content in place only.
- New semantic-link relation or target types, new `related-artifacts` forms, or path-reference syntax (EPIC rows 5-8).
- Link-importance metadata (EPIC row 9).
- CI, pre-push, nightly, or new integration tiers, and any #2003 architecture choice; the existing command and pre-commit step stay the only integration.
- Bulk migration of legacy records, and rewriting closed or historical records to remove diagnostics.
- Strict profiles for one-off classes that no template or skill produces; they stay permissive under the contract.
- Changing the externally governed top-level schema of Agent Skills or agent profiles.

## Architectural Decisions

Proposed decisions, pending maintainer approval. Each records the recommendation and the alternative weighed; the approval checkpoint below records the outcome.

- **D1 - Workflow.** Spec-first. This specification merges through a spec-only PR from `{issue-number}-extend-strict-profiles-and-author-guidance-spec`; implementation starts after that merge on `{issue-number}-extend-strict-profiles-and-author-guidance`. Inside implementation, T1 stops for maintainer approval of `profile-catalog.md`, in the way #2265 stopped for approval of the v1 contract, because the field tables are the contract this issue implements.
- **D2 - Versioning.** Recommended: new profiles join `schema-version: 1` as additive profiles, selected by `doc-type`. The contract reserves a new integer for an incompatible profile; an additive profile changes no existing outcome, because the baseline shows no non-issue/EPIC record with `schema-version: 1`. The closed contract stays unchanged as the historical record, as #2281 D10 did when it superseded one contract row; the approved catalog is the amendment, and EPIC row 4 carries it into the owned convention documents. Alternative: a per-profile version field, which adds a second version axis before any profile needs one.
- **D3 - Class identification.** Recommended: a record opts into a strict profile only through `schema-version: 1` plus its `doc-type`, emitted by its template. The structural layer derives no class from the file path, which keeps the contract's explicit opt-in and survives file moves. A repository-aware check may additionally require a location-bound class to live in its collection, for example `doc-type: adr` under an ADR collection. Alternative: path-based dispatch like #2281 D5, which would make the 32 existing ADRs strict without any edit and turn their current shape into new errors.
- **D4 - Profile catalog candidates.** A class qualifies when a repository template or skill produces it, so every profile has an authoring path that the template drift check (D6) can keep honest. If the approved catalog is too large for one reviewable PR, T1 splits the remainder into a follow-up row in the EPIC, as #2266 split into #2280 and #2281. Initial candidates, final list approved at T1:
  - `adr`, produced by `docs/templates/ADR.md` and the `create-adr` skill;
  - `manual-verification-evidence`, produced by `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`;
  - `refactor-plan`, produced by `docs/templates/REFACTOR-PLAN.md` and the `create-refactor-plan` skill;
  - `implementation-retrospective` and `agent-review-reports`, produced by their templates; the second has an existing consumer, `contrib/dev-tools/checks/agent-review-report-contract`, which the profile must not contradict;
  - `security-report`, produced by `docs/templates/SECURITY-REPORT.md`, whose field family is unrelated to the issue lifecycle;
  - PR review records, which the draft `docs/issues/drafts/mine-pr-review-audit-records/ISSUE.md` (task T2) needs as a strict profile and defers to this EPIC: this issue either delivers that profile or records the owner's agreement that the draft delivers it under this catalog;
  - `file-test-plan` and `test-refactor-plan` only if T1 finds a producing template or skill; otherwise they remain permissive candidates.
- **D5 - Agent Skills and agent profiles.** Recommended: an extension profile. The external top-level schema stays authoritative, as the contract requires; the profile applies the frozen v1 reference syntax and the repository-aware path and skill resolution to `metadata.semantic-links` only, and never requires or rejects another top-level key. Skills and agent profiles are live documents, so findings are errors. T5 fixes or records the findings in the 11 skills that carry the extension today.
- **D6 - Template drift protection.** Recommended: remove `docs/templates/` from `EXCLUDED_PREFIXES` and validate templates in a template mode. Field presence, the unknown-field rule, and quoting rules stay strict. A value may be a documented placeholder only under `docs/templates/`. An enumerated placeholder such as `<task|bug|feature|enhancement>` must list exactly the profile's allowed values. A placeholder for a field that requires a double-quoted string must itself be double-quoted. The pre-commit gate runs the validator on staged Markdown and the workspace doc tests, but not the crate's unit tests, so this places the check where a template edit is committed. Alternative: a crate test that substitutes placeholders and validates the result, which detects drift only when the tests run.
- **D7 - Location and severity for new profiles.** Recommended: reuse #2281 D8. Structural failures are errors, except in historical locations (`docs/issues/closed/` and `docs/refactor-plans/closed/`), where they are advisory warnings. Records without `schema-version: 1` stay permissive. No `legacy-shape` error is added for the new classes: a new record becomes strict through its template, and existing records are not migrated by this issue.
- **D8 - Derived guidance.** Recommended:
  - the generated schema covers every approved profile, selectable by `doc-type`, and stays under the #2280 drift check;
  - prose lists no field tables: the "Markdown Frontmatter" section of `docs/skills/semantic-skill-link-convention.md` points to the generated schema and the crate README instead of repeating fields;
  - the crate README lists each profile's class, opt-in, location, and severity, and extends the migration checklist per class;
  - `docs/schemas/README.md` lists the Rust-only invariants of each new profile;
  - each producing template and skill names the profile it produces and how to validate it.
- Related ADRs: [`docs/adrs/20260519000000_define_global_cli_output_contract.md`](../../../adrs/20260519000000_define_global_cli_output_contract.md). The validator stays `no-stdout-result`; new diagnostic categories follow the #2281 D9 record catalog.
- ADRs to create: none expected. Create one only if the approved D2 versioning rule is judged a durable, repository-wide decision beyond the profile catalog.

## EPIC Open Questions This Issue Touches

- "Which existing frontmatter variations are intentional compatibility cases, and which are errors that should be rejected prospectively?" Answered per class by the T1 catalog: variations in historical and legacy records stay compatible, and template-produced records are strict prospectively (D7).
- "How should the universal envelope identify document classes that currently have no `doc-type`?" Answered by D3: through a template-emitted `doc-type` under `schema-version: 1`, not by path.
- "Should experimental fields use a reserved namespace, remain warning-only, or require a schema change before use?" Answered for the new profiles by inheriting the v1 rule: an `x-` field is preserved with an `experimental-field` warning.
- "Which constraints belong in Rust deserialization, post-deserialization domain validation, or repository-aware validation?" Answered per profile by the T1 catalog under the EPIC's three-layer split; collection membership (D3) and template mode (D6) are repository-aware.
- "Which generated schema format best serves editors and AI agents without becoming a second source of truth?" Kept as answered by #2280 (Draft 2020-12 generated from Rust); this issue extends coverage (D8) and defers any format change.
- "Should historical closed issues and PR review records be exempt from new reference syntax?" Partly: no new reference syntax is introduced here, and historical records keep advisory severity (D7). The reference-syntax question is deferred to EPIC rows 5-6.
- "Which validation semantics must be decided here, and which execution, caching, output, and migration choices should be deferred to EPIC #2003?" Answered for this issue: profile semantics are decided here; execution stays the existing command and pre-commit step, and everything else is deferred to #2003.

The remaining Open Questions (semantic-link target types, path syntax, knowledge-graph edges, OKF profile disposition, comment markers in other languages, validator placement, harness scope, link importance, and external-link policy) are outside this issue and stay with EPIC rows 5-10.

## Design and Ownership Review

- **Library:** owns each profile's canonical type, strict definition, allowed values, and schema projection, and stays free of filesystem and git I/O.
- **Repository-aware layer:** owns location classification and severity for the new classes (D7), collection membership (D3), template mode (D6), and path and skill resolution for the external extension (D5).
- **Command adapter:** owns discovery and the exclusion list; removing `docs/templates/` from that list is its only change.

The command already runs short synchronous `git` processes and needs no new child processes, asynchronous I/O, network readiness, or persistent resources. Tests that need a repository keep the #2281 pattern of a disposable repository owned by a `TempDir`.

**Vertical-slice checkpoint.** After T3 (the ADR profile end to end: type, fixtures, schema, template, and skill guidance), stop and review the shared profile machinery before adding further profiles. Record the result in the progress log.

## Bug-Fix Process

The issue is a feature, but T7 corrects stale guidance: the field lists in `docs/skills/semantic-skill-link-convention.md` disagree with the v1 model, and a spec written from them fails validation. It follows [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

1. the hypothesis: the duplicated lists predate the v1 contract and were never updated with it;
2. the reproduction, attempted while drafting and before maintainer review: two disposable specs written from the convention's issue field list, one exactly as listed and one with `schema-version: 1` added. The outcome is **Infeasible** in the drafting environment, which has no Rust toolchain. The substitute evidence, a field-by-field comparison with `profile.rs` and a trace of the validator's dispatch, predicts a `legacy-shape` error for the first spec and a `missing-required-field` error for `epic` for the second. `manual-verification-evidence.md` section B1 records the specs, the commands, and the comparison; running the commands with the Rust toolchain moves the outcome to **Reproduced**;
3. the fix: replace the lists with references to the generated schema and the crate README (D8);
4. the like-for-like recheck: the same spec written from the referenced sources passes.

## Regression Test Strategy

Prose cannot carry a unit test, so the regression guard is structural: after T7 no hand-maintained field list remains to drift, and the template mode (D6) fails when a template, the other authoring source, drifts from a profile. A focused template-mode unit test covers an enumerated placeholder that lists `open` for `status` and must be rejected.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Propose the profile catalog and obtain approval | Reproduce the baseline counts with recorded commands. For each D4 candidate, record producers, observed fields and scalar forms, the proposed field table, opt-in, location, and severity, and classify each variation. Write `profile-catalog.md` and stop for maintainer approval. Split the catalog into a follow-up EPIC row if it is too large for one PR. |
| T2 | TODO | Generalize the strict profile machinery | Extend `StrictProfileKind`, `StrictProfileDefinition`, and the `FrontmatterV1` schema root to more than two profiles, and add the D7 location policy for the new classes. Issue and EPIC behavior and the tracked schema bytes stay unchanged. |
| T3 | TODO | Add the ADR profile end to end | Canonical type and definition, accepted and rejected fixtures, schema regeneration, the D3 collection check, `docs/templates/ADR.md` emitting v1, and `create-adr` skill guidance. Stop for the vertical-slice checkpoint. |
| T4 | TODO | Add the remaining approved repository-owned profiles | One coherent change per profile: type, fixtures, schema regeneration, template update, and producing-skill guidance. |
| T5 | TODO | Add the Agent Skill and agent-profile extension profile | Apply the frozen reference syntax and repository-aware resolution to `metadata.semantic-links` (D5); fix or record findings in existing skills. |
| T6 | TODO | Protect templates against drift | Remove `docs/templates/` from `EXCLUDED_PREFIXES`, implement template mode (D6) with fixtures for each drift kind, and fix the drift the first run reports in `docs/templates/`. |
| T7 | TODO | Derive author guidance from the model | Apply D8 to `docs/skills/semantic-skill-link-convention.md`, the crate README and its migration checklist, `docs/schemas/README.md`, and the producing skills, following the Bug-Fix Process for the stale field lists. |
| T8 | TODO | Run the whole-tree baseline and triage | Run `--all` before and after the change; record error and warning counts per class; fix genuine new errors in draft and open records; rewrite no historical record. |
| T9 | TODO | Prove failures and review | Fixture and mutation evidence, manual scenarios M1-M7, post-implementation acceptance review, independent Task Reviewer report, and the implementation completion review. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | `profile-catalog.md` with the baseline and the approval record | One `docs(issues)` commit after maintainer approval. |
| T2 | Generalized profile machinery and location policy, with no behavior change for issue and EPIC records | Commit after focused library tests and the schema drift check show no artifact change. |
| T3 | ADR profile, fixtures, regenerated schema, template, and skill guidance | Commit after focused tests and the vertical-slice review. |
| T4 | Each remaining approved profile | One commit per profile after its focused tests and schema regeneration. |
| T5 | Extension profile and any existing-skill fixes | Separate commits for the validator change and for each coherent skill fix. |
| T6 | Template mode and exclusion removal; separately, template fixes | Commit the validator change after its tests, then each template fix after the validator passes on it. |
| T7 | Guidance changes | One `docs` commit per guidance document group after `linter all`. |
| T8-T9 | Baseline record, evidence, and review records | Commit after the full quality gate and review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For every test-producing increment, use the `write-unit-test` skill and record the mandatory prose-first Arrange-Act-Assert design review before maintainer review and commit, confirming that each test exposes the one causal initial-state difference, such as the profile, the location, or the single offending field or placeholder. Commit through the `commit-changes` skill with Conventional Commits, the `frontmatter` scope for validator changes, and the issue reference.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md`
- [x] Stale-guidance reproduction attempted while drafting and classified in issue-local `manual-verification-evidence.md` section B1 (`Infeasible` in the drafting environment)
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Profile catalog (`profile-catalog.md`) approved by the maintainer before T2 starts
- [ ] Vertical-slice design review recorded after T3
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-10-03 10:54 UTC - da2ce7 - Drafted this specification from EPIC #2264 row 3, the v1 contract's deferred classes, and the validator as delivered by #2266, #2280, and #2281; awaits maintainer approval before a GitHub issue is created - This specification
- 2026-10-03 11:31 UTC - da2ce7 - Attempted the stale-guidance reproduction before review: `Infeasible` in the drafting environment (no Rust toolchain); recorded the disposable specs, validator commands, and field-by-field substitute evidence - `manual-verification-evidence.md` section B1

## Acceptance Criteria

- [ ] AC1: `profile-catalog.md` records the baseline, every D4 candidate's disposition, and the maintainer's approval before any profile is implemented.
- [ ] AC2: Every approved repository-owned profile has a canonical Rust type and strict definition, at least one accepted fixture, and rejected fixtures for an unknown field, a missing required field, a wrong scalar type, and an invalid allowed value where the profile has an enum.
- [ ] AC3: Issue and EPIC validation outcomes are unchanged: the existing library and command tests pass unmodified, and the `--all` counts for issue and EPIC records match the T8 baseline.
- [ ] AC4: `docs/schemas/frontmatter-v1.schema.json` is regenerated from the model, covers every approved profile selectable by `doc-type`, and passes the #2280 drift check; the parity test covers each new profile's required fields.
- [ ] AC5: A `schema-version: 1` record whose `doc-type` names no approved profile remains permissive.
- [ ] AC6: Agent Skills and agent profiles receive reference-syntax and resolution diagnostics for `metadata.semantic-links` only; an unknown top-level key in them produces no diagnostic.
- [ ] AC7: `docs/templates/` is validated in template mode in every command mode; a drifted template (missing field, unknown field, enumerated placeholder that differs from the profile's values, or unquoted placeholder for a quoted field) fails, and every tracked template passes.
- [ ] AC8: Each profiled class's template emits `schema-version: 1` and its `doc-type`, and its producing skill names the profile and the validation command.
- [ ] AC9: No hand-maintained frontmatter field list remains in `docs/skills/semantic-skill-link-convention.md`; author guidance points to the generated schema and the crate README.
- [ ] AC10: No historical record is rewritten to remove a diagnostic, and the T8 baseline records the before and after counts per class.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- Focused `frontmatter-validator` library and command tests, including each new profile's fixtures and template-mode cases.
- `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- check` (stable Rust toolchain).
- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain).
- `cargo clippy --package frontmatter-validator -- -D warnings` (stable Rust toolchain).
- `linter all` and the repository pre-commit gate; pre-push checks when applicable.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Author a new ADR from the template | Copy `docs/templates/ADR.md` into `docs/adrs/` as a new ADR, fill it in, stage it, and run the pre-commit gate. | The validator accepts it as a strict `adr` record. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Break an ADR's frontmatter | In the same ADR, add an unprefixed unknown field, then remove a required field; run `frontmatter-validator` on the file each time. | Each run exits `1` with one NDJSON `diagnostic` naming the category and field. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Misplace a location-bound record | Move the ADR record outside the ADR collections and validate it. | A repository-aware diagnostic names the collection rule (D3). | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Drift a template | Edit `docs/templates/ISSUE.md` so the `issue-type` placeholder lists a value the profile lacks, stage it, and run the pre-commit gate; revert. | The commit is rejected with a template-mode diagnostic; after the revert the gate passes. | TODO | `manual-verification-evidence.md` section V4 |
| M5 | Break a skill's semantic links | Add a nonexistent skill name and an absolute URL to a skill's `metadata.semantic-links`, then add an unknown top-level key; validate the file. | The reference and resolution findings are reported; the unknown top-level key is not. | TODO | `manual-verification-evidence.md` section V5 |
| M6 | Confirm historical records stay advisory | Validate `docs/issues/closed/` and `docs/refactor-plans/closed/` with the new profiles. | Profile findings there are warnings, and the run does not fail because of them. | TODO | `manual-verification-evidence.md` section V6 |
| M7 | Author from the guidance alone | Write a new issue spec using only the guidance that `docs/skills/semantic-skill-link-convention.md` points to, and validate it. | The spec passes without consulting the Rust source (the Bug-Fix Process recheck). | TODO | `manual-verification-evidence.md` section V7 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the feature, not a simulated result and not merely running automated tests.
- Every recorded validation command result identifies the toolchain that produced it when one can affect behavior, for example `nightly Rust toolchain` for `cargo +nightly fmt --all -- --check`.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios, and record actual commands, output, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. Fixture and command tests are maintained Rust tests in the crate; if a disposable script becomes useful for the T8 baseline, record its issue-local path, purpose, rationale, and removal owner here before creating it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | `profile-catalog.md` and its approval log entry |
| AC2 | TODO | Fixture list and focused test output |
| AC3 | TODO | Unmodified test run and T8 baseline counts |
| AC4 | TODO | Schema diff and drift-check output |
| AC5 | TODO | Unit test for an unknown v1 `doc-type` |
| AC6 | TODO | M5 and extension-profile tests |
| AC7 | TODO | M4 and template-mode tests |
| AC8 | TODO | Template and skill diffs |
| AC9 | TODO | Convention diff and M7 |
| AC10 | TODO | T8 baseline record |

## Risks and Trade-offs

- **The catalog grows too large for one PR.** Mitigation: the D4 producer criterion and the T1 split into a follow-up EPIC row.
- **Explicit opt-in leaves gaps.** A record written without its template escapes its profile (D3). Mitigation: templates and skills are the authoring path, and the template drift check keeps them current; a `legacy-shape` rule for the new classes can follow once the catalog has settled.
- **Template mode adds a placeholder grammar.** A loose grammar would hide drift. Mitigation: placeholders are accepted only under `docs/templates/`, and enumerated placeholders are compared with the profile's allowed values.
- **External schemas can change upstream.** Mitigation: D5 validates only the repository-owned extension and never a top-level key.
- **Interaction with EPIC row 2.3.** If the multiple-related-PRs change lands first, the template mode validates `docs/templates/ISSUE.md` against the amended issue profile without change here; if this issue lands first, row 2.3 updates the template within its own scope. Neither issue depends on the other.
- **Overlap with the PR review audit draft.** Mitigation: D4 records which issue delivers the PR review record profile, so the profile is defined once.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue's directory if any of these holds: the approved catalog differs materially from the D4 candidates; the D2 versioning or D3 opt-in rule changed during implementation; template mode needed a placeholder grammar beyond D6; the catalog was split into a follow-up row; or the T8 baseline found new errors in draft or open records that the plan did not anticipate.
- Otherwise add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2264 (`docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md`)
- Approved v1 contract: `docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-v1-contract.md`
- Predecessors: #2266, #2280, #2281
- Canonical crate: `contrib/dev-tools/checks/frontmatter-validator/`
- Coordinated draft: `docs/issues/drafts/mine-pr-review-audit-records/ISSUE.md`
- Sibling proposal (EPIC row 2.3): `docs/issues/drafts/2264-support-multiple-related-prs-in-issue-frontmatter/ISSUE.md`
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
