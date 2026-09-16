---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: 2003
github-issue: 2233
spec-path: docs/issues/open/2233-2003-tune-unified-pr-review-process/ISSUE.md
branch: "2233-2003-tune-unified-pr-review-process"
related-pr: 2235
last-updated-utc: 2026-09-16 12:06
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
  related-artifacts:
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/closed/2219-2003-unify-pr-review-processing/ISSUE.md
    - docs/pr-reviews/pr-2232-review.md
    - docs/templates/REVIEW-FINDINGS.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh
---

<!-- skill-link: create-issue -->

# Issue #2233 - Tune the Unified PR Review Process from First-Use Evidence

Parent EPIC: #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Use evidence from the first full execution of the unified PR-review workflow to make the process
more self-consistent and resistant to stale documentation references, without changing the
established audit contract or rewriting historical audit records.

## Background

Issue #2219 introduced a single author-owned review-processing workflow, immutable
`review-finding:pr-<PR_NUMBER>-<FINDING_ID>` references, and an audit record for every pull
request. Its first full use on PR #2232 validated the core design, but produced five durable
improvements. The motivating findings are recorded in
`docs/pr-reviews/pr-2232-review.md`; they remain the source evidence for this issue.

The work is deliberately scoped as a follow-up. It does not revisit deterministic finding
references, the split audit layout, or GraphQL-first sourcing decided and delivered by #2219.

## Scope

### In Scope

- Align the reviewer-facing `review-pr` guidance with the unified author-side review contract,
  including finding syntax, severity vocabulary, re-raise semantics, re-pushed-head scoping, and
  an explicit checklist `N/A` convention.
- Add a maintained structural guardrail that verifies repository-relative paths in Markdown code
  spans, including an explicit documented allowlist for intentional historical or illustrative
  paths.
- Add a retirement obligation inventory rule to the governing documentation-replacement workflow.
- Add a mechanical rename-purity verification pattern to the governing migration guidance.
- Create a design note, without implementing it, for tiered model routing during review
  processing.

### Out of Scope

- Modifying historical audit records below `docs/pr-reviews/`.
- Enforcing the advisory reviewer finding format through a GitHub bot or linting review comments.
- Implementing tiered-model review agents; this issue produces only their decision record/design
  input.
- Re-litigating #2219 decisions about immutable finding references, audit layout, or
  GraphQL-first sourcing.

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
- ADRs to create: None known. T5 may identify a durable repository-wide agent-routing decision;
  create an ADR before implementation only if the design selects a cross-cutting architecture or
  policy with consequences beyond this issue.

## Design and Ownership Review

T2 adds repository-maintenance tooling invoked by the pre-commit documentation-test step. Its
responsibilities and ownership boundaries are:

| Collaborator | Responsibility | Failure and lifetime boundary |
| ------------ | -------------- | ----------------------------- |
| Pre-commit documentation-test step | Invokes the checker and propagates its exit status. | It owns process invocation only; a nonzero checker status fails the step and stops the gate under the hook's existing control flow. |
| Code-span path checker | Scans tracked Markdown code spans, validates recognized repository paths, and emits source document/span diagnostics. | It owns no persistent resources; malformed input, a missing path, or an invalid exception is a deterministic nonzero result. |
| Dependency-free checker test | Exercises current-path, missing-path, and allowlisted historical/illustrative-path behavior. | It owns disposable fixtures and leaves no tracked runtime state. |
| Tracked allowlist | Records reviewed exceptions with source span, reason, and retention/removal owner. | It is maintained with the checker; an entry missing any required field is invalid and fails validation. |

The checker has no network readiness, asynchronous I/O, or reusable test-fixture lifecycle. Its
execution is bounded by the existing pre-commit step; no awaited readiness operation is introduced.
After the first passing T2 vertical slice, review this map against the actual invocation and
diagnostic behavior before expanding the checker.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Align reviewer-side guidance | Updated the `review-pr` skill and associated advisory template with `[<Severity>][<FindingId>] <summary>`, `Blocker`, `Major`, `Minor`, `Nit`, and `Suggestion`; one-finding-per-thread, re-raise, current-head, and `N/A` conventions validated with Markdown, spelling, link, and review-contract checks. Evidence: `review-finding:pr-2232-f12`. |
| T2 | TODO | Validate Markdown code-span paths | Add a focused maintained check under `contrib/dev-tools/checks/` and its dependency-free test under `contrib/dev-tools/checks/tests/`; invoke it from the existing pre-commit documentation-test step. It must fail for a missing path and pass real current paths. Keep exceptions in a tracked allowlist next to the checker; each entry names the source document and literal code span, states why it is historical or illustrative, and names a retention/removal owner. Decide between shell and Rust using the repository shell-vs-Rust threshold. Evidence: `review-finding:pr-2232-f4`, `review-finding:pr-2232-f5`. |
| T3 | TODO | Preserve retirement obligations | Add a `Retiring or Replacing Review Workflow Documents` section to `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`. It must require an inventory of every normative rule in the retiring artifact and record whether each is preserved with its destination or deliberately dropped with a reason. Evidence: `review-finding:pr-2232-f6`. |
| T4 | TODO | Verify rename purity mechanically | Add a `Rename Migration Verification` section to `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`. It must require an explicitly selected merge base, old and new repository paths, and reviewed expected added/deleted lines per renamed file. The verifier must fail when the actual changed-line pairs differ from that approved expectation. Evidence: `review-finding:pr-2232-f4`. |
| T5 | TODO | Design tiered model routing | Write a design note that separates strong-model current-tree triage and bounded solution specification from lower-cost implementation and independent verification. Record the cost, quality, auditability, failure-containment, and portability trade-offs. Do not implement agents. Evidence: deferred automation candidate in `docs/pr-reviews/pr-2232-review.md`. |

Candidate T4 verification contract:

```sh
actual_patch=$(git diff -U0 "<base>:<old-path>" "<new-path>")
test "$actual_patch" = "$(cat "<approved-zero-context-patch>")"
```

The approved patch is one reviewable fixture per renamed file and includes the expected file
headers and every permitted added/deleted line. The verifier must use `test` or an equivalent
failure-propagating comparison so an empty expected/actual patch succeeds only when both are
empty and any unexpected content fails.

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Reviewer guidance and advisory finding template alignment | Commit after focused documentation validation and review. |
| T2 | Code-span path checker, dependency-free tests, allowlist, and pre-commit invocation wiring | Commit after focused negative/positive checks and required review. |
| T3-T4 | Retirement and migration workflow documentation | Commit after focused documentation validation and review. |
| T5 | Tiered-routing design note | Commit after documentation validation and review. |

Record a justified no-change decision in task evidence without creating an empty commit. Sign every
commit with GPG and use a Conventional Commit subject with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2233-2003-tune-unified-pr-review-process/ISSUE.md`
- [x] GitHub issue created and issue number added to this spec
- [x] Spec reviewed and approved by user/maintainer
- [x] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-16 08:15 UTC - Copilot - Formalized the already-open GitHub issue #2233 from the
  first-use evidence recorded in `docs/pr-reviews/pr-2232-review.md`.
- 2026-09-16 12:06 UTC - Copilot - PR #2235 merged the reviewed specification into `develop`;
  created the reserved implementation branch `2233-2003-tune-unified-pr-review-process`.

## Acceptance Criteria

- [ ] AC1: Reviewer-facing guidance consistently defines the advisory finding format, severity
  vocabulary, one-finding-per-thread rule, re-raise behavior, round-N+1 scoping after a re-push,
  and checklist `N/A` semantics.
- [ ] AC2: A maintained pre-commit-invoked code-span repository-path check catches missing
  `docs/`, `.github/`, `contrib/`, `packages/`, and `src/` paths in dependency-free tests and
  permits only tracked exceptions identifying their source span, reason, and retention/removal
  owner.
- [ ] AC3: `process-pr-review/SKILL.md` requires a normative-obligation inventory for retiring or
  replacing review-workflow documents and a failure-propagating rename-purity verification that
  compares each renamed file against its approved changed-line expectation.
- [ ] AC4: A tiered-model routing design note defines agent boundaries and cost, quality,
  auditability, failure-containment, and portability trade-offs without adding automation.
- [ ] AC5: Each planned task retains its motivating `review-finding:pr-2232-*` reference or the
  explicitly recorded deferred automation candidate.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented in issue-local
  `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflows change.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`.
- Focused checker test(s) added by T2, including missing-path and allowlisted-path cases.
- The focused structural review-report contract check when modifying its protected artifacts:
  `contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh`.
- Applicable pre-push checks.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Review guidance consumption | Follow the updated reviewer guidance while writing a sample re-raised finding for a re-pushed PR head. | The reviewer can state the finding, relationship, and round scope without ambiguity. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Path-check exception review | Review a current path failure and an intentionally historical audit path through the checker and allowlist process. | The invalid current path fails; the historical record remains untouched and is accepted only through an explicit exception. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Tiered-routing design review | Read the design note as a prospective reviewer and trace one finding from triage through bounded implementation and independent verification. | Ownership boundaries, evidence, and failure handling are explicit before automation is considered. | TODO | `manual-verification-evidence.md` section V3 |

Create `manual-verification-evidence.md` from
`docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual
prerequisites, actions, commands, output, relevant logs, and outcomes. Toolchain-sensitive command
results must identify the toolchain or runtime that produced them.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Updated reviewer guidance and manual scenario M1. |
| AC2 | TODO | Focused checker tests, allowlist review, and manual scenario M2. |
| AC3 | TODO | Updated `process-pr-review` workflow guidance and focused documentation validation. |
| AC4 | TODO | Tiered-routing design note and manual scenario M3. |
| AC5 | TODO | Final specification and task evidence review. |

## Risks and Trade-offs

- A broad path parser can produce false positives from prose, URLs, shell placeholders, or
  historical evidence. Restrict extraction to code spans and recognized repository prefixes, then
  require every allowlist entry to state why the path is intentionally non-current.
- Requiring reviewer formatting as a gate would exclude external reviewers and conflict with the
  author-owned workflow. The format remains advisory; author-side normalization remains mandatory.
- Tiered routing can save cost but could lose context or introduce decision/implementation drift.
  The design must retain immutable finding references, current-tree triage evidence, bounded
  acceptance criteria, and independent verification before a thread is resolved.
- Documentation rules can become duplicated across skills. Update the single governing workflow
  for each rule and link to it rather than restating competing procedures.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions,
material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`.
- Create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` for material discoveries; otherwise add a
  concise progress-log entry explaining why none was needed.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2003.
- Predecessor issue: #2219.
- Related PR: #2232.
- First-use audit: `docs/pr-reviews/pr-2232-review.md`.
- Motivating findings: `review-finding:pr-2232-f4`, `review-finding:pr-2232-f5`,
  `review-finding:pr-2232-f6`, and `review-finding:pr-2232-f12`.
