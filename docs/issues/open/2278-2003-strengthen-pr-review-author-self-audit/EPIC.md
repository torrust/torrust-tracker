---
schema-version: 1
doc-type: epic
status: planned
epic: 2003
github-issue: 2278
spec-path: docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-09-22"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
    - fetch-review-threads
  related-artifacts:
    - "issue #2003"
    - "issue #2219"
    - "issue #2233"
    - "issue #2264"
    - "issue #2266"
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

# EPIC #2278 - Strengthen PR Review Author Self-Audit and Evidence Generation

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

This is a child EPIC of #2003. It was created as task #2278 through PR #2279 and converted into an
EPIC once its first task, the improvement matrix, showed that the remaining work spans several
independently reviewable artifacts. The conversion changed the document shape, not the goal or the
approved scope.

## Goal

Make the pull-request author's review-processing workflow converge reliably by requiring claims to
be derived from the current tree and durable sources before they are recorded, replied, or resolved.
Add narrowly scoped deterministic validation or generation helpers where objective evidence can
replace transcription, while preserving the canonical audit trail and human review judgment.

## Why This Is Needed

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

The maintainer's review of the EPIC #2003 comment thread
(<https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5767266486>) adds a
second evidence source: a register of `F1`-`F81` frictions filed across review rounds since
2026-08-27. Its Theme E items show that the author-side skill and the audit template now state
contradictory rules for the same case (outdated threads, the field roster, the consolidated-response
condition, review-body rows), and that the append-only and re-raise rules the retrospectives rely on
are not written where the record lives. An author cannot self-audit against a contract that
disagrees with itself, so reconciling those contradictions comes first.

This EPIC concerns the PR author process. It does not select a model for any review role; model
capability and availability change too frequently to be a durable repository policy. The process
must instead make each role's evidence and verification boundary explicit, so any capable model or
human can perform the work reproducibly.

## Decision Record

[`retrospective-improvement-matrix.md`](retrospective-improvement-matrix.md) is the approved
decision input for every subissue. It maps each proposal from the three retrospectives and each
author-side item of the #2003 friction register to `Adopt`, `Defer`, `Reject`, or `Out of scope`,
with owner and rationale, and records the boundaries with EPIC #2264. Subissues implement the
matrix; they do not reopen its dispositions without a maintainer decision recorded in this EPIC's
progress log.

Decisions the matrix fixes and that subissues must respect:

- one audit row per re-raised thread; re-raise rows are not collapsed into the original;
- `Resolution reference` keeps its commit-subject or durable-URL form; reply URLs alone are not a
  substitute for a repository change;
- no new per-finding fields; every change reuses or reconciles the existing roster;
- automation covers objective relationships only; narrative `Current-tree verification` text stays
  a manual, evidence-derived claim;
- the audit validator validates the audit body only; frontmatter and `review-finding:` target
  existence belong to EPIC #2264.

## Scope

### In Scope

- Reconcile the author-side contract so `process-pr-review` and `PR-REVIEW-TEMPLATE.md` state one
  rule per case, and single-source the audit field roster.
- Add an explicit author self-audit gate before each audit commit, reply, thread resolution, and
  re-review request, with re-derivation triggers after a second re-raise and after context
  compaction, and an evidence-first ordering that prevents intent from being recorded as fact.
- Align `fetch-review-threads` with the author workflow so both see all threads.
- Port the audit validator to Rust with parity fixtures, then extend it to the objective invariants
  the matrix adopts.
- Stop `agent-review-report-contract` from being cited as audit evidence and fix its pin granularity.
- Evaluate and implement only the evidence-generation helpers that demonstrably remove manual
  transcription.
- Decide, with maintainer approval, whether and how evidence requirements scale for small, low-risk
  changes.
- Retain the canonical audit, immutable finding references, append-only evidence intent,
  finding-specific replies, and the GraphQL completion check.

### Out of Scope

- Selecting, requiring, or routing work to a named LLM model or vendor. The
  `tiered-model-routing-design.md` from #2233 remains design input and is not revisited here.
- Weakening, deleting, or bulk-rewriting historical audit records.
- Removing finding-specific replies or resolving threads without a durable disposition.
- Building a shared guardrail runner, cache, policy engine, or CI integration, including invoking
  the audit validator or `agent-review-report-contract` from `testing.yaml`; these remain EPIC
  #2003 architecture decisions.
- Changing reviewer behavior, severity vocabulary, or GitHub's review interface. The reviewer-side
  `review-pr` items in the #2003 register are recorded in the matrix as input for a separate
  reviewer-side issue.
- Frontmatter validation of audit records, `review-finding:` target-existence checks, and any new
  marker syntax; these belong to EPIC #2264 and its subissues.
- Planning-template, semantic-link, linter, and CI-workflow items from the register; their owners
  are named in the matrix.
- Making an audit helper a prerequisite for the manual self-audit gate. The workflow must remain
  usable with direct Git and GitHub evidence while automation is unavailable.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Each subissue is one pull request. Register items are the `F<k>` numbers from the #2003 friction
register; retrospective items are cited by PR.

| Order | Issue | Local Spec | Status | Notes |
| ----- | ----- | ---------- | ------ | ----- |
| 1 | #2295 - Single-source the audit field roster and mark copied template sections | `docs/issues/closed/2295-2278-single-source-audit-roster/ISSUE.md` | DONE | F58, F77, F63. Completed by merged PR #2300; GitHub issue closed. Unblocks 2, 4, and 7. |
| 2 | #[To be assigned] - Reconcile skill and template rule contradictions | `docs/issues/open/{number}-2278-reconcile-audit-contract-rules/ISSUE.md` | TODO | F60, F73, F76, F79, F80, F61, F62, F66. Docs only. Depends on 1. |
| 3 | #[To be assigned] - Align `fetch-review-threads` with the author workflow | `docs/issues/open/{number}-2278-fetch-all-review-threads/ISSUE.md` | TODO | F17, F32, F18. Skill and scripts. No dependencies. |
| 4 | #[To be assigned] - Add the author self-audit gate to `process-pr-review` | `docs/issues/open/{number}-2278-author-self-audit-gate/ISSUE.md` | TODO | F65; PR #2270 and #2271 adopted items. Docs only. Depends on 1 and 2. |
| 5 | #[To be assigned] - Make `agent-review-report-contract` state what it reads | `docs/issues/open/{number}-2278-contract-checker-evidence-boundary/ISSUE.md` | TODO | F56 (false-evidence part), F57. Small Rust change. No dependencies. |
| 6 | #[To be assigned] - Port the audit validator to Rust with parity fixtures | `docs/issues/open/{number}-2278-port-audit-validator-to-rust/ISSUE.md` | TODO | F7. Behaviour parity only, `no-stdout-result` output contract. Depends on #2266 recording its integration-point decision. |
| 7 | #[To be assigned] - Extend the audit validator to the adopted invariants | `docs/issues/open/{number}-2278-extend-audit-validator-invariants/ISSUE.md` | TODO | F64, F75, F74, F58, F63. Depends on 1 and 6. |
| 8 | #[To be assigned] - Generate finding-detail skeletons from source comments | `docs/issues/open/{number}-2278-generate-finding-detail-skeleton/ISSUE.md` | TODO | PR #2270 tooling proposal 3. Depends on 6. |
| 9 | #[To be assigned] - Decide proportionate evidence for low-risk changes | `docs/issues/open/{number}-2278-proportionate-review-evidence/ISSUE.md` | TODO | PR #2272 proposals 3 and 4; F55 as a self-audit step. Decision note first, maintainer approval, then the limited change. Depends on 4. |

Subissue specifications are drafted one at a time as `docs/issues/drafts/2278-{slug}/ISSUE.md`
and created only after maintainer approval, following the `create-issue` skill.

## Delivery Strategy

Deliver in dependency order with the smallest reviewable change per pull request. Subissues 1, 3,
and 5 have no dependencies and may run in parallel. Subissue 6 waits for #2266 so both check crates
give the same answer to placement and output-contract questions.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an issue-local
   retrospective for reusable lessons, material design changes, or meaningful deviations from the
   plan; otherwise record why one was unnecessary in the issue progress log.

Every subissue pull request is itself processed with `process-pr-review`. Those audits are the
EPIC's own validation evidence: a subissue that changes the author workflow is exercised by the
review of the pull request that delivers it.

### Phase 1: Consistent contract

- Outcome: subissues 1, 2, and 3 merged. The skill, template, and helper skill agree on every case
  the matrix lists, and the roster exists once.
- Exit criteria: a side-by-side reading of skill and template finds no contradictory rule for the
  same case; `fetch-review-threads` returns resolved and outdated threads.

### Phase 2: Author gate and honest checkers

- Outcome: subissues 4 and 5 merged. The self-audit gate is normative and no check that ignores the
  audit can be cited as audit evidence.
- Exit criteria: a pull request processed with the revised skill records the self-audit before each
  reply; `agent-review-report-contract` documents or honors its input.

### Phase 3: Deterministic validation

- Outcome: subissues 6, 7, and 8 merged. The Rust validator enforces the adopted objective
  invariants with fixtures; mechanical fields are generated rather than transcribed.
- Exit criteria: mutation fixtures for each invariant fail with an actionable diagnostic; the
  Python prototype is removed.

### Phase 4: Proportionate evidence and closure

- Outcome: subissue 9 decided and applied; EPIC validation scenarios recorded.
- Exit criteria: the scenarios below are satisfied by real subissue review evidence, and the
  acceptance criteria are checked against it.

## Validation Scenarios

These scenarios close the EPIC. They are satisfied by the audits of this EPIC's own subissue pull
requests where possible; a fixture is used only when no real review produced the case.

| ID | Scenario | Expected Result |
| -- | -------- | --------------- |
| M1 | Substantive finding self-audit | The reply claim names evidence observed after the change; no audit or reply claim is copied from intent or an earlier draft. |
| M2 | Audit correction and re-raise | The validator identifies objective defects before a reply, and the author records the re-raise relationship against the audit-local finding ID. |
| M3 | Low-risk evidence decision | The documented rules give a reproducible decision, preserve required traceability, and escalate a similar-looking higher-risk counterexample. |
| M4 | Automation unavailable | With the helper unavailable, the author collects direct Git and GitHub evidence and correctly blocks reply or resolution when evidence cannot be established. |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted, approved, and merged as task #2278 through PR #2279
- [x] GitHub issue #2278 created and linked as a sub-issue of #2003
- [x] Improvement matrix approved by the maintainer
- [x] Converted to a child EPIC with maintainer approval of the conversion steps
- [ ] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-21 16:15 UTC - GitHub Copilot - Drafted from the author-side review-process evidence in PR #2270, PR #2271, and PR #2272 retrospectives; awaiting maintainer review before GitHub issue creation.
- 2026-09-21 16:25 UTC - GitHub Copilot - Maintainer approved the specification; created GitHub sub-issue #2278 under EPIC #2003 and moved this specification to its numbered open-issue folder.
- 2026-09-21 18:21 UTC - GitHub Copilot - PR #2279 merged into `develop` as merge commit `ffa3528cfd9ed170eeb910c6273bf675637dd3d1`.
- 2026-09-21 18:31 UTC - GitHub Copilot - Completed T1 with `retrospective-improvement-matrix.md`; awaiting maintainer review of its dispositions.
- 2026-09-22 06:28 UTC - GitHub Copilot - Extended the matrix with the author-side items of the EPIC #2003 friction register (comment 5767266486) and recorded the boundaries with EPIC #2264 and #2266.
- 2026-09-22 06:59 UTC - GitHub Copilot - Maintainer approved the matrix dispositions and the conversion of #2278 into a child EPIC split into nine one-PR subissues. Converted `ISSUE.md` to `EPIC.md` in place.
- 2026-09-22 09:40 UTC - GitHub Copilot - PR #2288 merged; GitHub issue #2278 relabelled `task` to `EPIC` and its body replaced with this specification. Created subissue #2295 (order 1) after maintainer approval of its specification.
- 2026-09-22 13:16 UTC - GitHub Copilot - Implemented #2295 T1-T5 on its implementation branch; manual verification evidence recorded and the subissue is awaiting its implementation PR.
- 2026-09-22 17:26 UTC - GitHub Copilot - PR #2300 merged and automatically closed subissue #2295. Archived its specification under `docs/issues/closed/`; subissues 2, 4, and 7 are unblocked.

## Acceptance Criteria

- [ ] AC1: Every proposal in the PR #2270, #2271, and #2272 retrospectives and every author-side #2003 register item has a recorded disposition, rationale, and owner in the matrix.
- [ ] AC2: `process-pr-review`, its helper skills, and `PR-REVIEW-TEMPLATE.md` state one rule for every case the matrix lists; the field roster appears once, one field per line, and matches the detail skeleton. (Subissues 1, 2, 3.)
- [ ] AC3: The author workflow requires current-source self-audit before every audit commit, reply, thread resolution, and re-review request; states the evidence source for each claim; never accepts intent as verification; and defines the second-re-raise and context-compaction triggers. (Subissue 4.)
- [ ] AC4: No check that ignores the audit record can be cited as audit evidence. (Subissue 5.)
- [ ] AC5: A Rust audit validator verifies its documented objective invariants with offline fixture tests, emits actionable diagnostics under the `no-stdout-result` contract, does not mutate repository or GitHub state, and does not validate frontmatter or `review-finding:` targets. (Subissues 6, 7.)
- [ ] AC6: Mechanical audit fields are generated from source identifiers rather than transcribed. (Subissue 8.)
- [ ] AC7: A documented maintainer decision addresses proportionate evidence for low-risk changes, including eligibility, escalation, and preserved audit requirements, without relying on a named model or vendor. (Subissue 9.)
- [ ] AC8: Scenarios M1-M4 are satisfied by recorded evidence without relaxing reply, traceability, or GraphQL completion requirements.
- [ ] All subissues are created, linked, and their statuses reflect actual state.
- [ ] Every completed subissue includes automated verification, manual verification, post-implementation acceptance review, and an implementation completion review.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | `retrospective-improvement-matrix.md`; maintainer approval recorded in the 2026-09-22 06:59 UTC progress entry. |
| AC2 | TODO | #2295 is DONE via merged PR #2300; subissues 2 and 3 remain. |
| AC3 | TODO | Subissue 4. |
| AC4 | TODO | Subissue 5. |
| AC5 | TODO | Subissues 6, 7. |
| AC6 | TODO | Subissue 8. |
| AC7 | TODO | Subissue 9. |
| AC8 | TODO | Subissue PR audits under `docs/pr-reviews/`; fixtures where no real case arose. |

## Risks and Trade-offs

- A validator that attempts to prove narrative current-tree claims will create false confidence or
  brittle parsing. Limit automation to objective relationships and require named manual evidence
  for semantic judgment.
- Adding fields or mandatory commands can recreate the fixed-cost burden this EPIC addresses. Every
  requirement must replace a manual transcription or prevent a demonstrated failure mode.
- A risk tier can be gamed or misclassified. Eligibility must be objective, conservative, and
  include escalation triggers; uncertain cases use the full path.
- Two check crates answering placement and output questions differently would decide architecture
  before #2003 does. Subissue 6 follows the #2266 decision rather than making its own.
- Nine small pull requests each carry review overhead. The EPIC accepts that cost because each is
  independently revertible and each review is validation evidence for the workflow under change.
- The process must be independent of model identity. Evidence, command output, and acceptance
  criteria are the durable controls, not presumed model capability.

## References

- Parent EPIC: #2003.
- Sibling child EPIC with the shared check-crate shape: #2264 (subissues #2266, #2280, #2281).
- EPIC #2003 friction register summary: <https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5767266486>.
- Prior process work: #2219 and #2233.
- Source retrospectives: PR #2270, PR #2271, and PR #2272 under `docs/pr-reviews/`.
- Spec-only PR for the original task: #2279.
- Related ADR: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
