---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2003
github-issue: 2278
spec-path: docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/ISSUE.md
branch: "2278-2003-strengthen-pr-review-author-self-audit-spec"
related-pr: null
last-updated-utc: "2026-09-22 06:28"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
    - fetch-review-threads
  related-artifacts:
    - "issue #2003"
    - "issue #2219"
    - "issue #2233"
    - docs/pr-reviews/pr-2270-review/review-retrospective.md
    - docs/pr-reviews/pr-2271-review/review-retrospective.md
    - docs/pr-reviews/pr-2272-review/review-retrospective.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py
    - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - contrib/dev-tools/checks/agent-review-report-contract/src/main.rs
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/retrospective-improvement-matrix.md
---

<!-- skill-link: create-issue -->

# Issue #2278 - Strengthen PR Review Author Self-Audit and Evidence Generation

Parent EPIC: #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make the pull-request author's review-processing workflow converge reliably by requiring claims to
be derived from the current tree and durable sources before they are recorded, replied, or resolved.
Add narrowly scoped deterministic validation or generation helpers where objective evidence can
replace transcription, while preserving the canonical audit trail and human review judgment.

## Background

The retrospectives for PRs #2270, #2271, and #2272 show a common author-side failure despite the
reviewer's findings being correct: the audit record and replies contained factual claims that had
not been re-derived from the current tree, Git history, or GitHub data. Attempts to correct those
claims introduced further unverified claims, producing repeated `RE_RAISE_OF` findings and review
rounds focused on the audit rather than the delivered change.

PR #2270 recorded 25 of 43 findings about audit or progress-log accuracy. PR #2271 recorded 31 of
42 human findings about the audit record. PR #2272 showed that a correct four-line workflow change
still incurred recursive audit-only review because the process did not scale its evidence burden
to change risk. The existing `process-pr-review` skill and prototype validator establish useful
foundations, but the validator does not verify all structural relationships and the workflow does
not force a self-audit before a reply reaches GitHub.

This work concerns the PR author process. It does not select a model for any review role; model
capability and availability change too frequently to be a durable repository policy. The process
must instead make each role's evidence and verification boundary explicit, so any capable model or
human can perform the work reproducibly.

The maintainer's review of the EPIC #2003 comment thread
(<https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5767266486>) adds a
second evidence source: a register of `F1`-`F81` frictions filed across review rounds since
2026-08-27. Its Theme E items show that the author-side skill and the audit template now state
contradictory rules for the same case (outdated threads, the field roster, the consolidated-response
condition, review-body rows), and that the append-only and re-raise rules the retrospectives rely on
are not written where the record lives. An author cannot self-audit against a contract that
disagrees with itself, so reconciling those contradictions is a precondition for T2, not a separate
clean-up. The register items this issue owns, and those left to other owners, are dispositioned in
`retrospective-improvement-matrix.md`.

## Scope

### In Scope

- Consolidate the author-side evidence from the three retrospectives into a traceable disposition
  of each proposed improvement: adopt, defer with rationale, or reject with rationale. Reconcile
  the conflicting proposals explicitly: PR #2270 suggests collapsing re-raise rows and questions
  the `Resolution reference` field, while PR #2271 concludes the current field roster must not
  change.
- Add an explicit author self-audit gate before each reply, thread resolution, and re-review
  request. The gate must re-derive affected claims from the current tree, Git history, and GitHub
  source data rather than prior audit prose or session memory.
- Add two author-side triggers that force a full re-derivation of every affected audit field: a
  second re-raise of the same finding, and any session context compaction. In both cases the author
  stops editing the record until the re-derivation is complete.
- Define an evidence-first ordering for review processing: implement and validate the substantive
  change, inspect the committed tree, derive audit fields, validate the audit, then reply and
  resolve. The workflow must prevent an intended change from being recorded as an observed fact.
- Reconcile the author-side contract so `process-pr-review` and `PR-REVIEW-TEMPLATE.md` state one
  rule per case: the outdated-thread disposition, a single field roster written one field per line
  and matching the detail skeleton, the admissible `Resolution reference` value per disposition, the
  consolidated-response condition, the `Source URL` and `Thread state` of a review-body row, the
  one-row-per-re-raise rule, the Processing Log append-only rule and its in-place-repair procedure,
  and an explicit marker for the template sections a record copies verbatim. Define how Copilot's
  collapsed suppressed comments are normalized. These reconcile existing rules and add no field.
- Align `fetch-review-threads` with the author workflow: fetch all threads, including resolved and
  outdated, and include `resolvedBy` and `line` in the worked query, so re-raise detection and the
  self-audit see the same data the reviewer does.
- Extend or replace the existing audit validator with narrowly scoped deterministic checks and
  fixture-based tests for objective audit invariants, including row/detail parity and order,
  required-field roster, audit-local re-raise targets, source/reply-thread ownership, cited commit
  existence and whether that commit touches the path the entry describes, and template-controlled
  sections.
- Stop `agent-review-report-contract` from being cited as audit evidence: either make it honor an
  explicit path argument or document that it validates only fixed skill and template literals, so
  a passing run is never recorded as verification of an audit record. Fix its inconsistent pin
  granularity in the same change.
- Decide the audit validator's implementation language and test coverage: port the Python prototype
  to Rust with fixture tests, per the repository's developer-tooling policy, following the check-crate
  shape and output contract that #2266 selects for `frontmatter-validator`, and proving behaviour
  parity before adding invariants. The audit validator validates the audit body only; frontmatter
  and `review-finding:` target existence belong to EPIC #2264. An audit-existence check can only be
  a self-audit step here, because pre-commit has no PR context.
- Evaluate and implement only the evidence-generation helpers that demonstrably remove manual
  transcription. Candidate helpers include generating finding-detail skeletons from GitHub source
  identifiers and deriving processing-log event references or timestamps from Git and GitHub data.
- Define a proportionate-review evidence policy for small, low-risk changes. Compare a single
  workflow with documented evidence tiers, establish objective eligibility and escalation rules,
  and record the maintainer-approved decision before changing the audit contract. Include whether
  `Nit` and `Suggestion` findings on process-only artifacts from the same review may be batched
  into one reply-and-resolve pass while keeping one audit row per finding.
- Retain the existing canonical audit, immutable finding references, append-only evidence intent,
  finding-specific replies, and GraphQL completion check.

### Out of Scope

- Selecting, requiring, or routing work to a named LLM model or vendor. The existing
  `tiered-model-routing-design.md` from #2233 remains design input and is not revisited here.
- Weakening, deleting, or bulk-rewriting historical audit records.
- Removing finding-specific replies or resolving threads without a durable disposition.
- Building a general-purpose shared guardrail runner, cache, policy engine, or CI integration;
  those choices remain owned by EPIC #2003's later architecture decision.
- Changing reviewer behavior, severity vocabulary, or GitHub's review interface. The reviewer-side
  `review-pr` items in the EPIC #2003 register (verdict mapping, re-pushed-head procedure, ACK
  re-earning, superseded-PR path, draft-PR bar, first-round finding-ID scheme) are recorded in the
  matrix as input for a separate reviewer-side issue.
- Invoking `agent-review-report-contract` or the audit validator from `testing.yaml`; CI
  integration remains an EPIC #2003 architecture decision.
- Planning-template, semantic-link, linter, and CI-workflow items from the register; their owners
  are named in the matrix.
- Frontmatter validation of audit records, `review-finding:` target-existence checks, and any new
  marker syntax; these belong to EPIC #2264 and its subissues. The matrix records the boundary.
- Making an audit helper a prerequisite for the manual self-audit gate. The workflow must remain
  usable with direct Git and GitHub evidence while automation is unavailable.

## Architectural Decisions

- Related ADR: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
- Existing process decisions retained: #2219's unified author-owned audit and #2233's immutable
  finding references, re-raise normalization, and reviewer-format guidance.
- ADRs to create: None known. Create one before implementation only if the proportionate-review
  policy creates a repository-wide workflow tiering decision beyond `process-pr-review`.

## Design and Ownership Review

The work may enhance a helper that invokes `git` and `gh`, but it introduces no network service,
asynchronous I/O, reusable fixture lifecycle, or persistent runtime resource.

| Collaborator | Responsibility | Failure and lifetime boundary |
| ------------ | -------------- | ----------------------------- |
| `process-pr-review` skill | Orders author actions and names the manual self-audit evidence required before an external reply or resolution. | A missing or failed verification blocks the affected reply; the skill owns no subprocess or network resource. |
| Audit validator/helper | Reads an explicit audit path and explicit GitHub/Git evidence, then reports deterministic pass/fail diagnostics. | It must not edit the audit or GitHub state. A failed command, malformed input, unavailable source, or invalid relationship is a nonzero result with recovery guidance. |
| Fixture tests | Exercise parser and validation behavior using checked-in or generated disposable inputs without network access. | Fixtures are test-owned and leave no tracked runtime state. |
| GitHub CLI / Git | Supply source comments, thread relationships, and committed-tree evidence. | They remain external command boundaries; the helper must distinguish unavailable evidence from a validated claim and must never manufacture a substitute value. |

After the first passing validator/helper vertical slice, review the command interface, diagnostics,
fixture coverage, and skill integration before expanding the set of enforced invariants.

## Bug-Fix Process

Not applicable. This is prospective workflow and guardrail improvement, not a defect in tracker
runtime behavior.

## Regression Test Strategy

Not applicable. The issue does not change production behavior. Deterministic validator/helper
behavior will instead be protected with focused fixture tests that mutate one audit invariant at a
time and assert the diagnostic and nonzero exit status.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Build the retrospective improvement matrix | Added `retrospective-improvement-matrix.md`, mapping every proposed improvement from PRs #2270, #2271, and #2272, plus the author-side items of the EPIC #2003 friction register, to an adopted, deferred, rejected, or out-of-scope disposition with owner and rationale. Maintainer review is required before T2. |
| T2 | TODO | Specify and add the author self-audit gate and reconcile the contract | Update `process-pr-review` so each affected claim is re-derived from named current-tree, Git, or GitHub evidence before audit commit, reply, resolution, and re-review request. Add the second-re-raise and context-compaction re-derivation triggers, the one-commit-per-concern rule, and a completion-checklist item for the self-audit (register F65). Reconcile the skill and template contradictions listed in the matrix (F58, F60, F61, F62, F63, F66, F73, F75, F76, F77, F79, F80) without adding fields. Align `fetch-review-threads` to return all threads with `resolvedBy` and `line` (F17, F18, F32). Require a concise self-audit record that identifies the commands or inspections actually used. |
| T3 | TODO | Deliver the smallest useful deterministic audit guardrails | Implement or extend a read-only validator/helper according to T1's matrix. It must support offline fixture input, emit actionable deterministic diagnostics, and cover the selected structural and source-derived invariants (row/detail parity and order, F64; audit-local re-raise targets, F75; cited commit touches the described path, F74; roster and copied-section parity, F58 and F63) without asserting unverifiable prose claims. Port to Rust following the #2266 check-crate shape and `no-stdout-result` output contract, with parity fixtures first (F7); start after #2266 records its integration-point decision. Resolve the `agent-review-report-contract` false-evidence gap and pin granularity (F56, F57) in the same task. |
| T4 | TODO | Decide proportionate evidence handling | Compare no-tier and risk-tier alternatives against the three retrospectives. Define eligibility, mandatory escalation, retained traceability, and explicit non-eligibility conditions; either implement the approved limited change or record why the current single path remains preferable. |
| T5 | TODO | Validate realistic author workflows | Exercise the revised process against representative audit fixtures: an ordinary substantive finding, an audit-record correction, a re-raised finding, and a narrow low-risk change. Record manual evidence and confirm the workflow blocks reply or resolution whenever a claim lacks recorded current-source evidence. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Retrospective evidence matrix and approved scope decisions | Commit after documentation validation and maintainer review of dispositions. |
| T2 | Author self-audit workflow, contract reconciliation of skill and template, and `fetch-review-threads` alignment | Commit the skill/template reconciliation separately from the self-audit gate and from the helper-skill change so each rule change is independently reviewable. |
| T3 | Read-only validator/helper, fixture tests, and narrow workflow integration | Commit after focused tests, design review, and validation of diagnostics. |
| T4 | Proportionate-evidence decision and any resulting limited skill/template change | Commit after maintainer decision and focused documentation validation. |
| T5 | Manual verification evidence and completion-review updates | Commit separately when it improves traceability; do not create an empty commit for evidence already included in a coherent prior change. |

Use a Conventional Commit subject with the narrow affected scope and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted and moved to `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-21 16:15 UTC - GitHub Copilot - Drafted from the author-side review-process evidence in PR #2270, PR #2271, and PR #2272 retrospectives; awaiting maintainer review before GitHub issue creation.
- 2026-09-21 16:25 UTC - GitHub Copilot - Maintainer approved the specification; created GitHub sub-issue #2278 under EPIC #2003 and moved this specification to its numbered open-issue folder. Implementation remains deferred until this spec-only pull request merges.
- 2026-09-21 18:21 UTC - GitHub Copilot - PR #2279 merged into `develop` as merge commit `ffa3528cfd9ed170eeb910c6273bf675637dd3d1`; began implementation from the merged specification.
- 2026-09-21 18:31 UTC - GitHub Copilot - Completed T1 with `retrospective-improvement-matrix.md`; awaiting maintainer review of its adopted, deferred, and rejected dispositions before T2.
- 2026-09-22 06:28 UTC - GitHub Copilot - Extended T1 and the specification with the author-side items of the EPIC #2003 friction register (comment 5767266486): skill/template contradictions, helper-skill alignment, validator language and existence-check decisions; reviewer-side and non-review items recorded as out of scope with named owners.

## Acceptance Criteria

- [ ] AC1: An issue-local improvement matrix traces every proposal in the PR #2270, #2271, and #2272 retrospectives to an adopted, deferred, or rejected disposition with rationale and owner artifact.
- [ ] AC2: The author workflow requires current-source self-audit before every audit commit, reply, thread resolution, and re-review request; it states the evidence source for each claim, never accepts intent as verification, and defines the second-re-raise and context-compaction re-derivation triggers.
- [ ] AC3: The selected read-only validator/helper verifies its documented objective invariants with offline fixture tests, emits actionable nonzero diagnostics, and does not mutate repository or GitHub state.
- [ ] AC4: The workflow distinguishes mechanically verifiable fields from prose judgment, retaining manual verification where no sound deterministic check exists, and no check that ignores the audit record can be cited as audit evidence.
- [ ] AC5: A documented maintainer decision addresses proportionate evidence for low-risk changes, including eligibility, escalation, and preserved audit requirements; it does not rely on a named model or vendor.
- [ ] AC6: Manual scenarios demonstrate convergence for substantive, audit-correction, re-raise, and low-risk cases without relaxing reply, traceability, or final GraphQL completion requirements.
- [ ] AC7: `process-pr-review`, its helper skills, and `PR-REVIEW-TEMPLATE.md` state one rule for every case the matrix lists under skill and template contradictions; the field roster appears once, one field per line, and matches the detail skeleton; and every author-side register item in the matrix has a recorded disposition.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflows change.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- Focused validator/helper fixture tests, including accepted and rejected audit relationships.
- The existing review-report contract test when its protected artifacts change:
  `cargo run --quiet --package agent-review-report-contract`.
- `linter all`.
- Applicable pre-push checks.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Substantive finding self-audit | Process one representative finding against a current branch using the revised skill; inspect the committed change, derive the audit fields, run the validator, then prepare the reply. | The reply claim names evidence observed after the change; no audit or reply claim is copied from intent or an earlier draft. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Audit correction and re-raise | Use fixtures containing an incorrect audit reference and a later re-raise; correct both with the revised process. | The validator identifies objective defects before a reply, and the author records the re-raise relationship against the audit-local finding ID. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Low-risk evidence decision | Apply the documented eligibility and escalation rules to a narrow configuration or workflow change and to a similar-looking higher-risk counterexample. | The decision is reproducible, preserves required traceability, and escalates the counterexample rather than silently reducing evidence. | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Automation unavailable | Run the manual self-audit procedure with the optional helper unavailable. | The author can collect direct Git and GitHub evidence and correctly blocks reply or resolution when evidence cannot be established. | TODO | `manual-verification-evidence.md` section V4 |

Create `manual-verification-evidence.md` from
`docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual
prerequisites, actions, commands, output, relevant logs, and outcomes. Toolchain-sensitive command
results must identify the toolchain or runtime that produced them.

### Disposable Verification Scripts

No disposable verification script is planned. Use maintained fixture tests for deterministic
validator/helper behavior and issue-local manual evidence for author workflow scenarios.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | `retrospective-improvement-matrix.md` maps every proposal from PR #2270, #2271, and #2272, and every author-side EPIC #2003 register item, to a disposition, rationale, and owner; pending maintainer review gates T2. |
| AC2 | TODO | Updated skill/template and M1-M2 evidence. |
| AC3 | TODO | Focused fixture tests and diagnostic examples. |
| AC4 | TODO | Validator contract, skill evidence boundaries, and M4. |
| AC5 | TODO | Decision record and M3 evidence. |
| AC6 | TODO | Manual scenarios M1-M4 and final GraphQL completion check. |
| AC7 | TODO | Side-by-side reading of skill and template for each listed case; roster/skeleton diff; matrix register section. |

## Risks and Trade-offs

- A validator that attempts to prove narrative current-tree claims will create false confidence or
  brittle parsing. Limit automation to objective relationships and require named manual evidence
  for semantic judgment.
- Adding fields or mandatory commands can recreate the fixed-cost burden this issue addresses.
  Every requirement must replace a manual transcription or prevent a demonstrated failure mode.
- A risk tier can be gamed or misclassified. Eligibility must be objective, conservative, and
  include escalation triggers; uncertain cases use the full path.
- The existing Python validator is a prototype. A change may retain it only with fixture coverage
  and an explicit migration-compatible behavior contract; do not introduce a general tooling
  framework through this issue.
- The process must be independent of model identity. Evidence, command output, and acceptance
  criteria are the durable controls, not presumed model capability.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions,
material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: Required if the proportionate-evidence decision changes scope materially, the
  validator exposes unanticipated audit-contract gaps, or a manual scenario reveals nonconvergent
  author behavior. Otherwise record a concise no-retrospective rationale in the progress log.
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2003.
- Sibling child EPIC with shared check-crate shape: #2264 (subissues #2266, #2280, #2281).
- EPIC #2003 friction register summary: <https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5767266486>.
- Prior process work: #2219 and #2233.
- Source retrospectives: PR #2270, PR #2271, and PR #2272 under `docs/pr-reviews/`.
- Current author workflow: `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`.
