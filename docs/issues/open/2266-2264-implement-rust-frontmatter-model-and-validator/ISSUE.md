---
doc-type: issue
issue-type: feature
status: planned
priority: p1
epic: 2264
github-issue: 2266
spec-path: docs/issues/open/2266-2264-implement-rust-frontmatter-model-and-validator/ISSUE.md
branch: "2264-2003-refactor-semantic-link-conventions-spec"
related-pr: 2269
last-updated-utc: "2026-09-21 16:01"
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - issue #2264
    - issue #2265
    - docs/skills/semantic-skill-link-convention.md
    - docs/templates/ISSUE.md
    - docs/templates/EPIC.md
    - docs/AGENTS.md
    - issue #2003
---

# Issue #2266 - Implement the Rust Frontmatter Model and Initial Validator

**Parent EPIC:** #2264 - Refactor Semantic Link and Frontmatter Conventions

**Predecessor:** [#2265 - Inventory and Resolve Markdown Frontmatter Contracts](../../closed/2265-2264-inventory-markdown-frontmatter-contracts/ISSUE.md)

## Goal

Implement canonical Rust types for the approved Markdown frontmatter v1 contract, generate a
machine-readable schema from those types, and provide a small read-only validator that gives humans
and AI agents deterministic field, scalar-type, allowed-value, and compatibility diagnostics.

## Background

The repository currently describes frontmatter types in prose. AI agents must infer whether a
field is required, nullable, date-like, enum-like, profile-specific, or an open extension. The
predecessor issue resolves those ambiguities and supplies accepted and rejected fixtures.

Rust types are the canonical executable source of truth because they can be reused by future Rust
automation selected by EPIC #2003. A generated schema exposes the same model to editors, agents,
and external tools without creating a separately maintained contract.

The first implementation uses the predecessor's provisional reference model:

- `skill-links` deserialize to validated skill names;
- `related-artifacts` deserialize to a narrow union of repository-relative file or directory
  paths, `issue #<number>` references, and established `review-finding:pr-<number>-<id>`
  references;
- a document's own issue and EPIC relationships remain dedicated metadata fields;
- no new `path:`, `adr:`, `rust-item:`, or other typed reference syntax is introduced.

Repository-aware checks such as target existence are separate from YAML deserialization and
structural profile validation.

The repository already has a precedent for this shape of work: `contrib/dev-tools/checks/clippy-allow-reasons`
is a non-published workspace crate invoked by `pre-commit.sh` through `cargo run --package`. It
was approved under the same EPIC #2003 early-work exception and shows an integration point that
needs no new runner. This issue should evaluate that precedent first and record why it does or
does not fit before choosing another location.

## Scope

### In Scope

- Select a minimal, replaceable Rust package or existing integration point consistent with EPIC
  #2003's approved early-work boundary.
- Parse YAML frontmatter from tracked Markdown files that contain it.
- Represent the approved universal envelope and strict issue and EPIC profiles with canonical Rust
  types.
- Preserve explicitly approved compatibility cases and unknown document classes according to the
  predecessor's rollout policy.
- Represent the provisional v1 `skill-links` and `related-artifacts` value types without extending
  the semantic-link ontology.
- Separate syntax, structural/profile, and repository-aware validation layers.
- Generate a machine-readable schema from the canonical Rust model and verify generated output does
  not drift. The drift check must be reproducible without network access and must not add a new
  required toolchain component beyond the workspace's existing Rust toolchain.
- Emit stable, actionable diagnostics containing at least the source path, diagnostic category,
  severity, field path when applicable, and explanation. Classify the binary as
  `no-stdout-result` under the repository CLI output contract ADR: stdout remains empty, stderr
  contains NDJSON records, and exit codes are `0` for pass, `1` for validation/runtime failure,
  and `2` for invalid invocation. TTY refusal does not apply to this output class.
- Accept explicit file or directory paths for focused use, a `--staged` mode for pre-commit, and a
  manual whole-tree mode. Pre-commit is the only integration tier in this issue; CI integration is
  deferred to #2003.
- Add accepted/rejected fixture tests and mutation evidence for representative field, scalar,
  allowed-value, reference, and lifecycle/path failures.
- Integrate the check into pre-commit without adding CI integration, a shared runner, cache, policy
  engine, or orchestration framework.
- Document the temporary integration point and how the model and fixtures can move into the
  architecture later selected by #2003.

### Out of Scope

- Defining semantic-link relation types or a complete knowledge graph.
- Adding new typed path-reference syntax.
- Requiring frontmatter on every Markdown document.
- Strictly rejecting every unknown document class in the first rollout.
- Bulk migration or rewriting of historical documents.
- Selecting #2003's final automation binary, package structure, JSONL event contract, caching, or
  local/CI policy composition.
- Integrating the validator into CI, pre-push, nightly, release, or agent policy profiles.
- Validating ordinary Markdown links, which remain Lychee's responsibility.
- Validating the externally owned top-level frontmatter of `SKILL.md` and agent-profile files
  beyond the repository-owned `metadata.semantic-links` extension.
- Auto-fixing or rewriting frontmatter. The check is read-only; any formatter is a separate,
  later decision.

## Architectural Decisions

- Canonical representation: Rust types derived from the approved inventory contract.
- External representation: generated machine-readable schema; never edited as a second source of
  truth.
- Validation layers: frontmatter/YAML syntax, typed structural profile, then repository semantics.
- Initial architecture: an independently testable and replaceable check using an existing
  integration tier; final placement remains owned by EPIC #2003.
- Output: `no-stdout-result` under the CLI output contract ADR; no plain-text output at any
  verbosity and no TTY refusal.
- Dependencies: choose the YAML and schema-generation crates under the repository's dependency
  freshness policy and record the choice in the issue; prefer crates already in the workspace
  lockfile when they meet the need.
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`.
- ADRs to create: Create a root ADR only if implementation requires a durable repository-wide
  choice beyond the decisions already approved in the parent and predecessor specifications.

## Design and Ownership Review

The parser owns frontmatter extraction and YAML syntax reporting. Canonical model types own scalar
representation and profile structure. Domain validators own cross-field invariants. Repository
adapters own filesystem and repository lookups. The command surface owns discovery, diagnostic
rendering, and exit status, but not the final orchestration contract reserved for #2003.

No network access, asynchronous readiness, or persistent resources are required. Tests use
repository-local immutable fixtures or isolated temporary directories and must leave no tracked
runtime state.

**Split checkpoint.** After the first passing issue-profile vertical slice (T3 plus the issue
profile part of T4), stop and review the boundaries above. If the slice shows that schema
generation (T6) or the existing-tier integration (T7) is independently reviewable and would
otherwise make this issue's PR too large, split them into follow-up issues at that point rather
than continuing. Record the decision in the progress log either way.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Confirm predecessor contract | Use the approved `frontmatter-inventory.md`, `frontmatter-v1-contract.md`, and `frontmatter-fixtures/` from issue #2265 as the implementation input and compatibility baseline. |
| T2 | TODO | Select the replaceable integration point | Evaluate the `clippy-allow-reasons` precedent first; record package/location, invocation, and staged/whole-tree modes without selecting #2003's long-term architecture. |
| T3 | TODO | Implement extraction and universal envelope | Parse present frontmatter and report malformed delimiters or YAML, invalid envelope fields, and scalar errors. Missing frontmatter is an error only for a strict profile that requires it. |
| T4 | TODO | Implement strict issue and EPIC profiles | Encode required fields, enums, nullability, cross-field invariants, and prospective versus legacy behavior. |
| T5 | TODO | Implement provisional references | Parse skill names, repository-relative paths, `issue #<number>`, and `review-finding:` references; keep target-existence checks in the repository-aware layer. |
| T6 | TODO | Generate the external schema | Generate the predecessor-selected format and dialect from Rust types to a documented tracked path, record unsupported Rust invariants, and add deterministic offline drift verification. |
| T7 | TODO | Add validator diagnostics and rollout | Add explicit-path, `--staged`, and manual whole-tree invocation; integrate only with pre-commit; preserve unknown-class compatibility as approved. |
| T8 | TODO | Prove failures and portability | Exercise fixture and mutation cases, run manual agent/human scenarios, and document relocation into a future #2003 architecture. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T2-T3 | Package/integration decision plus frontmatter extraction and envelope vertical slice | Commit after focused tests and the required boundary review. |
| T4 | Strict issue and EPIC profiles | Commit after accepted/rejected fixture tests and prose-first test-design review. |
| T5 | Provisional reference value types and repository-aware boundary | Commit after focused parsing and validation tests. |
| T6 | Generated schema and drift check | Commit after deterministic regeneration verification. |
| T7-T8 | Validator command, existing-tier integration, mutation evidence, and documentation | Commit after focused and full validation plus maintainer review. |

For every test-producing increment, follow the `write-unit-test` skill and record the required
prose-first Arrange-Act-Assert design review before commit. Use signed Conventional Commits.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in
      `docs/issues/drafts/implement-rust-frontmatter-model-and-validator/ISSUE.md`
- [x] Parent EPIC received GitHub issue #2264 and this specification received `epic: 2264`
- [x] Predecessor inventory and v1 contract approved
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2266 created and linked as a sub-issue of #2264
- [x] Specification folder moved to `docs/issues/open/2266-2264-implement-rust-frontmatter-model-and-validator/`
  and open-state metadata plus live references updated
- [x] Planning/evidence PR #2269 opened and `related-pr` updated
- [x] Planning/evidence PR #2269 merged into `develop` before implementation
- [ ] First issue-profile vertical slice completed and design boundaries reviewed
- [ ] Implementation completed
- [ ] Automatic verification completed
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Independent reviewer reports recorded when applicable
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-18 11:05 UTC - GitHub Copilot - Drafted the canonical Rust model, generated schema, and
  initial validator issue as the blocked successor to the frontmatter inventory - This specification
- 2026-09-18 12:40 UTC - GitHub Copilot - Review pass: aligned the reference union with the
  predecessor, named the `clippy-allow-reasons` integration precedent, bound output to the CLI
  output contract ADR, added staged/whole-tree modes, a read-only guarantee, a pre-commit manual
  scenario, and an explicit split checkpoint after the first vertical slice - This specification
- 2026-09-18 12:50 UTC - GitHub Operator - Created issue #2266 as a native child of #2264 and
  promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2266
- 2026-09-18 15:25 UTC - GitHub Copilot - Opened spec-only PR #2269 and recorded it in frontmatter -
  https://github.com/torrust/torrust-tracker/pull/2269
- 2026-09-19 07:58 UTC - GitHub - Merged spec-only PR #2269 into `develop`; this issue remains
  blocked by #2265 - https://github.com/torrust/torrust-tracker/pull/2269
- 2026-09-19 14:05 UTC - GitHub Copilot - Issue #2265 proposed the inventory, v1 contract, and
  fixtures; this issue remained blocked pending maintainer approval -
  `docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/`
- 2026-09-21 14:33 UTC - Jose Celano - Maintainer approved the predecessor inventory, v1 contract,
  and fixtures; T1 is complete and implementation is unblocked -
  `docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/`
- 2026-09-21 16:01 UTC - GitHub Copilot - Archived completed predecessor issue #2265; this issue
  remains planned with the archived inventory, contract, and fixtures as its input -
  `docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/`

## Acceptance Criteria

- [ ] AC1: Canonical Rust types represent the approved universal envelope and strict issue and EPIC
      profiles without duplicating an independently authored schema.
- [ ] AC2: Every accepted predecessor fixture parses and validates with its expected result; every
      rejected fixture produces the expected diagnostic category.
- [ ] AC3: Syntax, structural/profile, and repository-aware validation remain separate and are
      independently testable.
- [ ] AC4: The provisional reference model accepts only approved skill names, repository-relative
      paths, `issue #<number>` references, and `review-finding:` references in their respective
      fields and introduces no new typed relation syntax.
- [ ] AC5: A deterministic machine-readable schema is generated from the Rust model and a check
      fails when the tracked generated artifact drifts.
- [ ] AC6: Diagnostics identify the source path, category, severity, field path when applicable,
      and an actionable explanation; output and exit codes follow the CLI output contract ADR.
- [ ] AC7: The initial rollout validates every present Markdown frontmatter block, strictly enforces
      approved known profiles, and handles unknown classes and legacy documents according to the
      approved compatibility policy.
- [ ] AC8: Mutation evidence proves detection of malformed YAML, wrong scalar types, missing required
      fields, unknown allowed values, invalid provisional references, and representative
      lifecycle/path inconsistencies.
- [ ] AC9: The check uses an existing integration tier and remains replaceable; no shared runner,
      cache, policy engine, or final #2003 architecture is introduced.
- [ ] AC10: Documentation identifies the canonical Rust source, generated artifact, temporary
      invocation, rollout behavior, and future #2003 relocation boundary.
- [ ] AC11: The check is read-only and never modifies a document; a run against an unchanged tree
  leaves all tracked Markdown content and the Git index byte-identical.
- [ ] AC12: The staged-files and whole-tree modes produce consistent diagnostics for the same
      document.
- [ ] `linter all` and relevant Rust tests exit with code `0`.
- [ ] Manual verification scenarios are executed and documented.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- Focused Rust unit tests for extraction, deserialization, profile validation, references, and
  diagnostics.
- Integration tests over accepted/rejected Markdown fixtures and isolated repository fixtures.
- Deterministic generated-schema drift check.
- Mutation checks for each failure class listed in AC8.
- `cargo test --doc --workspace`
- `linter all`
- Applicable pre-push checks before completion.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Validate a current draft | Run the documented validator command against a conforming draft issue or EPIC. | The command exits successfully and reports the recognized profile without requiring network access or interactive input. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Correct an invalid field | In an isolated fixture, introduce a wrong scalar or allowed value, run the validator, correct it from the diagnostic, and rerun. | The first run identifies the exact source and field with recovery guidance; the corrected run succeeds. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Exercise compatibility | Run the validator against representative unknown and historical document classes selected by the predecessor issue. | Results match the approved warning, acceptance, or exemption policy and do not trigger bulk migration. | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Regenerate external schema | Run the documented generation command, inspect the resulting schema, and rerun the drift check. | The generated artifact exposes the Rust contract, is deterministic, and is unchanged on the second generation. | TODO | `manual-verification-evidence.md` section V4 |
| M5 | Run through pre-commit | Stage a conforming and a non-conforming spec in an isolated fixture and run `./contrib/dev-tools/git/hooks/pre-commit.sh`. | The hook reports the check as a named step; the non-conforming spec blocks the commit with the validator's NDJSON diagnostic; the conforming spec passes; the validator binary itself writes nothing to stdout. | TODO | `manual-verification-evidence.md` section V5 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1-AC4 | TODO | Rust model, focused tests, and predecessor fixtures |
| AC5 | TODO | Generated schema and drift test |
| AC6-AC8 | TODO | Diagnostic tests and mutation evidence |
| AC9-AC10 | TODO | Integration and portability documentation |
| AC11-AC12 | TODO | Read-only and mode-consistency tests plus M5 evidence |

## Risks and Trade-offs

- Combining model, schema generation, and validation can become too large. Preserve vertical slices
  and split the issue before implementation if the inventory reveals multiple independent document
  profile migrations.
- YAML implicit typing can turn date-like or boolean-like text into unexpected scalars. Fixtures
  must cover the approved quoting and coercion policy explicitly.
- Generated schemas cannot express every repository-aware invariant. Keep those checks in typed
  Rust validators and document that boundary rather than weakening the canonical model.
- Early integration may differ from #2003's final architecture. Keep domain types, fixtures, and
  diagnostics independent from hook or CI orchestration so relocation does not alter semantics.
- Strict enforcement can create a large legacy backlog. Follow the approved prospective and
  compatibility modes instead of weakening diagnostics or rewriting history opportunistically.

## Implementation Completion Review

After implementation, create `implementation-retrospective.md` if package placement, generated
schema limitations, YAML behavior, compatibility findings, or #2003 integration materially differ
from the specification. Otherwise record why the implementation confirmed the planned design.

## References

- Parent EPIC: #2264
- Parent automation EPIC: #2003
- Predecessor specification: #2265
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- Integration precedent: `contrib/dev-tools/checks/clippy-allow-reasons/` (approved early work
  under EPIC #2003 via #2157)
