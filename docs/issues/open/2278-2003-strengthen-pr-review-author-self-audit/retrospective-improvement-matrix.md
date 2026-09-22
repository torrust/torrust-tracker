---
doc-type: review-process-improvement-matrix
issue: 2278
last-updated-utc: 2026-09-22 06:28
semantic-links:
  related-artifacts:
    - "issue #2278"
    - "issue #2003"
    - docs/pr-reviews/pr-2270-review/review-retrospective.md
    - docs/pr-reviews/pr-2271-review/review-retrospective.md
    - docs/pr-reviews/pr-2272-review/review-retrospective.md
---

# Retrospective Improvement Matrix

This matrix is the approved decision input for EPIC #2278 and its subissues. It maps every
improvement proposed by the
PR #2270, #2271, and #2272 review retrospectives, plus the author-side items of the EPIC #2003
friction register, to an implementation disposition. `Adopt` means the issue must deliver it.
`Defer` means a named later task must decide the policy before any workflow change. `Reject`
means the evidence does not justify the proposed change; the rationale states the retained
control. `Out of scope` means the item belongs to another artifact owner and is listed only so
the decision is traceable.

The `Owner task` columns predate the conversion of #2278 into an EPIC and are kept as written.
They map to the EPIC's subissues as follows: T2 covers subissues 1-4 (roster, contract rules,
`fetch-review-threads`, self-audit gate); T3 covers subissues 5-8 (contract checker boundary,
validator port, validator invariants, skeleton generator); T4 is subissue 9 (proportionate
evidence).

## Author Verification and Convergence

| Source | Proposal | Disposition | Owner artifact or task | Rationale |
| ------ | -------- | ----------- | ---------------------- | --------- |
| PR #2270 | Run the audit validator before every reply and audit commit. | Adopt | T2, `process-pr-review` | Objective audit inconsistencies must fail before they reach a reviewer. The run is required whenever the validator is available; when it is not, the self-audit gate performs the same checks by hand (row/detail parity and order, reply posted on the source thread, resolution subject present on the branch, log order) and records the commands used. The validator is never a prerequisite for the gate. |
| PR #2270 | Extend validation to issue progress logs, including chronological order, finding-ID references, and append-only changes. | Defer | T3 | Chronological order is objective; finding-ID and append-only checks need a defined ownership boundary and Git comparison input before enforcement. |
| PR #2270 | Generate mechanical audit fields from a source comment identifier. | Adopt | T3 | Source review ID, source URL, severity, and reply relationship are transcription-prone facts. |
| PR #2270 | Derive processing-log timestamps from the carrying commit. | Defer | T3 | Commit timestamps cannot represent GitHub reply or resolution events. T3 must choose a source-derived event-reference format that does not make false timing claims. |
| PR #2270 | Stop and re-derive affected fields after a finding is re-raised twice. | Adopt | T2 | A second re-raise is an observable failure to converge and warrants a stronger recovery step. |
| PR #2270 | Require corrections to re-derive claims from named Git or GitHub commands. | Adopt | T2, template | The verification text must be derived from evidence, not an intended edit. |
| PR #2270 | Revalidate after context compaction. | Adopt | T2 | Compaction specifically threatens verbatim IDs, subjects, URLs, and order. |
| PR #2271 | Self-audit every row before requesting re-review: rerun verification, confirm cited commits touch described paths, and verify re-raise targets, row/detail parity, and order. | Adopt | T2 and T3 | This is the author-side symmetry missing from the costly review rounds. T2 names the manual gate; T3 automates objective parts. |
| PR #2271 | Keep one repository fix per commit and audit updates in separate commits. | Adopt | T2 | Separate concerns make resolution references unambiguous and reviewable. |
| PR #2271 | Write verification command and result before narrative verification text. | Adopt | T2, template | It prevents intent from being recorded as current-tree fact. |
| PR #2271 | Use a commit only when `git show --stat <commit> -- <path>` proves it contains the described change. | Adopt | T2 and T3 | A commit subject existing on the branch is insufficient evidence that it resolved the row. |
| PR #2271 | Avoid self-referential counts and universal claims in audit prose. | Adopt | T2, template | Such claims rot as later review activity changes the record. |
| PR #2271 | Derive log events from Git and GitHub timestamps, not recollection. | Adopt | T2; T3 evaluates automation | The source of each event must be recorded; T3 may automate only when the event source is objective. |
| PR #2271 | Explain what `agent-review-report-contract` reads so it is not cited as audit evidence. | Adopt | T3, checker documentation | A passing fixed-target contract checker does not validate a PR-specific audit. |

## Audit Structure and Format

| Source | Proposal | Disposition | Owner artifact or task | Rationale |
| ------ | -------- | ----------- | ---------------------- | --------- |
| PR #2271 | Validate audit sections, row/detail parity and order, field roster, audit-local re-raise targets, and cited commit subjects. | Adopt | T3 validator and fixture tests | These are deterministic relationships with a demonstrated review cost. |
| PR #2272 | Diff `Status Values` and `Completion Rules` against the template byte-for-byte. | Adopt | T3 validator and fixture tests | These template-controlled sections are stable structural contract text; early detection prevents recursive format review. |
| PR #2272 | Remove manual `HH:MM` processing-log precision and record source event references instead. | Defer | T3 | The desired replacement must preserve chronological evidence and distinguish commit, push, reply, and resolution sources. |
| PR #2270 | Collapse re-raise bookkeeping into an original row's `Re-raised in` list. | Reject | Retain current audit contract | PR #2271 requires audit-local relationships and the audit trace must retain each thread's independently actionable outcome. T4 may reduce processing effort, but not erase thread-level provenance. |
| PR #2270 | Add an `audit-integrity` category. | Defer | T4 | It may improve future measurement, but category vocabulary is part of the durable audit contract and is not needed to enforce author self-audit. |
| PR #2270 | Replace commit-subject resolution references with reply URLs alone. | Reject | Retain current audit contract | PR #2271's evidence and the current template use commit subjects to connect a correction to versioned content; reply URLs cannot establish that a repository change exists. |
| PR #2271 | Do not add more per-finding fields. | Adopt | T2, T3, T4 | All changes must reuse or derive the current field roster unless a maintainer approves a contract change. |

## Proportionate Evidence and Reply Handling

| Source | Proposal | Disposition | Owner artifact or task | Rationale |
| ------ | -------- | ----------- | ---------------------- | --------- |
| PR #2272 | Batch `Suggestion` or `Nit` findings on process-only artifacts from the same review into one reply-and-resolve pass. | Defer | T4 | Batching may reduce fixed overhead, but must retain a finding-specific reply URL or explicitly define an equivalent durable relationship. |
| PR #2272 | Add a lighter audit path for small, low-risk changes. | Defer | T4 | The evidence establishes disproportionate cost, but not safe eligibility criteria. T4 must compare no-tier and risk-tier alternatives with mandatory escalation. |

## EPIC #2003 Friction Register Items

Source: the maintainer's register summary at
<https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5767266486>, recomputed
against `develop` `ffa3528c`. Only items whose fix lives in the author-side workflow (the
`process-pr-review` skill, its helper skills, the audit template, or the audit validator) are
dispositioned here. `F<k>` numbers are the register's own.

### Already covered by the specification

| Item | Gist | Disposition | Owner task |
| ---- | ---- | ----------- | ---------- |
| F64 | No rule fixes the findings-table row order. | Adopt | T3 validator (row/detail parity and order) |
| F65 | "Update progressively" has no completion-checklist counterpart. | Adopt | T2 self-audit gate and checklist item |
| F74 | A resolution reference must contain the fix, not merely exist on the branch. | Adopt | T3 validator (`git show --stat <commit> -- <path>`) |
| F75 | `RE_RAISE_OF` does not say whose finding ID it takes. | Adopt | T2 wording; T3 validator (audit-local targets) |
| F56 | The contract binary is cited as evidence it cannot provide. | Adopt (false-evidence part only) | T3 |

### Skill and template contradictions

Each of these is a case where the author can follow one normative source and be contradicted by
the other. They are the direct cause of "correct claim, wrong rule" review rounds and are adopted
for T2 as a single contract reconciliation, not as new fields.

| Item | Gist | Disposition | Owner task | Rationale |
| ---- | ---- | ----------- | ---------- | --------- |
| F60 | Template prescribes `NO_ACTION`/`SUPERSEDED` for an outdated thread; skill prescribes `FIXED`/`RESOLVED` when a change fixed it. | Adopt | T2 template and skill | The skill rule is correct: an outdated thread whose concern was fixed is `FIXED`. The template must say the same. |
| F58 | The skill's field roster omits `Concern` and `Solution`, which the template skeleton requires. | Adopt | T2 skill and template; T3 validator enforces the skeleton | One canonical roster must exist; the validator can only check a roster that both documents agree on. |
| F77 | The roster is one prose sentence naming 17 fields. | Adopt | T2 skill | Restate as one field per line so it can be diffed against the skeleton. |
| F63 | Nothing marks which template sections a record copies verbatim and which are guidance. | Adopt | T2 template; T3 validator byte-diffs the copied sections | The PR #2272 template-drift check needs an explicit list of copied sections. Use plain HTML comments local to `PR-REVIEW-TEMPLATE.md`; do not introduce a new typed marker, which would belong to EPIC #2264's marker catalog. |
| F61 | `## Processing Log` carries no append-only rule while sibling logs do. | Adopt | T2 template | PR #2270 F11/F27 were in-place rewrites; the rule must be stated where the log lives. |
| F62 | Nothing says how to repair an append-only record already modified in place. | Adopt | T2 skill | State the recovery: restore the prior entries, append a correction entry naming what was rewritten, never rewrite again. |
| F73 | That a re-raise takes its own row is unstated. | Adopt | T2 template | The matrix rejects collapsing re-raise rows; the template must state the rule it relies on. |
| F76 | `Resolution reference` has no admissible value for a `NO_ACTION` or `FOLLOW_UP` row. | Adopt | T2 skill and template | Define the admissible value per disposition (commit subject, reply URL, or follow-up PR URL) without adding a field. |
| F79 | Step 8 conditions the consolidated-response rule; the checklist states it flatly. | Adopt | T2 skill | Make the checklist item carry the same condition. |
| F80 | A review-body finding has no stated `Source URL` or `Thread state` in the template. | Adopt | T2 template | State the review URL as source and `NON_RESOLVABLE` as thread state for a review-body row. |
| F66 | Copilot's collapsed "Suppressed comments" block is named nowhere. | Adopt | T2 skill step 2 | The normalization step must say whether suppressed comments are findings; otherwise two authors will decide differently. |

### Helper skills the self-audit depends on

| Item | Gist | Disposition | Owner task | Rationale |
| ---- | ---- | ----------- | ---------- | --------- |
| F17, F32 | `fetch-review-threads` still filters to `isResolved == false` while `process-pr-review` requires all threads. | Adopt | T2 `fetch-review-threads` skill and scripts | Re-raise detection and the self-audit need resolved and outdated threads; the helper must not hide them. |
| F18 | The worked GraphQL query omits `resolvedBy` and `line`. | Adopt | T2 `fetch-review-threads` | Cheap, and `resolvedBy` is self-audit evidence for who closed a thread. |

### Enforcement and tooling

| Item | Gist | Disposition | Owner task | Rationale |
| ---- | ---- | ----------- | ---------- | --------- |
| F7 | `validate-audit-record.py` is a Python prototype with no tests and no lint gate. | Adopt | T3 | Repository policy prefers Rust for non-trivial developer tooling. T3 ports the validator to Rust as a replaceable check crate following the shape #2266 selects (see the #2264 boundary section below), with fixture tests proving parity with the Python behaviour before any new invariant is added. |
| F55 | The audit record is mandatory but nothing checks that one exists. | Defer | T2 (manual step); CI part out of scope | Pre-commit has no PR context, so it cannot know whether a PR needs an audit. The only in-scope form is a self-audit checklist step; a PR-time check is an EPIC #2003 CI decision. |
| F57 | The contract crate pins one field as a bare marker and another as a full placeholder. | Adopt | T3, when touching `agent-review-report-contract` | Fix the pin granularity in the same change that resolves the false-evidence gap; do not open a separate issue for it. |
| F56 (CI part) | The contract binary is never invoked by `testing.yaml`. | Out of scope | EPIC #2003 architecture decision | CI integration is excluded by this issue's Out of Scope. |

### Considered and left to other owners

| Items | Owner | Reason |
| ----- | ----- | ------ |
| F14, F15, F16, F20, F21, F22, F23, F28, F31, F36, F41, F42, F43, F48, F50, F67, F68 | `review-pr` skill (reviewer side) | This issue changes the author workflow only. The register's observation that the reviewer-side skill has lagged is recorded here as input for a separate reviewer-side issue. |
| F19 (reviewer-side gate re-run) | `review-pr` skill | Same reason. |
| F1, F2, F3, F4, F40, F44, F47, F70, F72, F78 | `create-issue`, EPIC and issue templates | Planning-template items; not review processing. |
| F51, F52, F53, F54, F69, F81 | EPIC #2264 | Semantic-link and frontmatter conventions. |
| F8, F24, F25, F26, F37, F38, F39, F45, F46, and the two CI findings | Repository rules, `torrust-linting`, CI | Not review processing; F37 and F38 are external. |

## Boundaries with EPIC #2264 and Its Subissues

EPIC #2264 (semantic links and frontmatter) and its open children #2266, #2280, and #2281 build a
Rust `frontmatter-validator` crate as a replaceable read-only check at the pre-commit tier, with
NDJSON diagnostics on stderr, no stdout, exit codes `0`/`1`/`2`, and fixture tests. The audit
validator this issue owns has the same shape but a different domain. To avoid two divergent
answers to the same questions:

| Concern | Owner | Rule for this issue |
| ------- | ----- | ------------------- |
| Frontmatter of `PR-REVIEW.md` (fields, scalar types, `related-artifacts` forms) | #2266 | The audit validator does not parse or validate frontmatter. It validates the audit body only. |
| Existence of a `review-finding:pr-<n>-<id>` target cited from another document | #2264 row 6 (semantic-link validation) | Out of scope here. The audit record is the referent; this issue only guarantees the audit's own finding IDs are well formed and unique. |
| Package placement and invocation shape for a read-only Markdown check | #2266 T2 decides for `frontmatter-validator`; #2003 owns the final architecture | T3 follows whatever #2266 T2 records (currently the `clippy-allow-reasons` precedent: a non-published workspace crate under `contrib/dev-tools/checks/`, run via `cargo run --package`). Do not merge the audit validator into `frontmatter-validator`; the domains are unrelated and coupling them would decide placement before #2003 does. |
| Output contract | `docs/adrs/20260519000000_define_global_cli_output_contract.md` | The Rust port must classify as `no-stdout-result` like the frontmatter validator. The Python prototype prints a JSON summary to stdout; parity tests must cover diagnostics, not that line. |
| Template section markers (F63) | This issue, locally | HTML comments in `PR-REVIEW-TEMPLATE.md` only. A repository-wide "copied section" convention would be a #2264 row 4/5 decision. |
| Quoted `issue #<n>` in audit frontmatter (register F51 sweep) | #2264 | New audits comply with the provisional v1 convention; the sweep of existing files is not this issue's work. |

Sequencing consequence: the validator port (T3) should start after #2266 records its T2
integration-point decision, so both checks give the same answer. Everything else in this issue is
independent of #2264.

## Boundaries Retained

The following retrospective cautions are retained as constraints on T2-T5:

- Do not remove the canonical audit, append-only evidence intent, finding-specific replies, or
  final GraphQL completion check.
- Do not require a retrospective for ordinary low-round reviews.
- Do not turn reviewer byte-level re-derivation into the required reply format; a reply remains a
  disposition, one verifying command or inspection, and a resolution reference.
- Do not select a named model or vendor as a process control. The durable control is evidence the
  author can re-derive from the current tree, Git history, and GitHub source data.
