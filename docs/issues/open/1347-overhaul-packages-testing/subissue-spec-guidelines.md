---
doc-type: guidance
epic: 1347
spec-path: docs/issues/open/1347-overhaul-packages-testing/subissue-spec-guidelines.md
last-updated-utc: 2026-09-14
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/testing/refactoring-patterns/README.md
---

# EPIC #1347 Subissue Specification Guidelines

Requirements and lessons for drafting the next package-testing subissue specification. Distilled
from the completed #2136, #2140, and #2149 subissues; the primary evidence base is Issue #2149
(119 commits, 19 file-local plans, ~82 recorded approval gates, four external review rounds, and
its `implementation-retrospective.md`).

Apply these guidelines when creating each new EPIC #1347 subissue. They supplement, and never
override, the `create-issue` skill and the repository issue template.

## Practices To Keep (Validated by #2149)

1. **Unit-first with separated coverage scopes.** Record aggregate/global, unit-only (`--lib`),
   and integration-only (`--test <name>`) reports separately, from clean `cargo llvm-cov` runs.
   Never combine their percentages or use one scope to claim another's protection.
2. **Per-module ownership decisions.** Every uncovered line either gets a test, or gets a named
   owner (another package, another boundary, deferred lifecycle work) recorded in the plan.
3. **Prose-first Arrange-Act-Assert review with the test-smell checklist** after every added or
   changed test, per `write-unit-test` and the refactoring-pattern catalog.
4. **Bounded mutation sampling** on one changed high-risk seam: minutes, not hours; no score
   target; record configuration, outcome, and limitations.
5. **Challenge every no-change decision.** A no-change or deferral decision must list the
   uncovered lines and answer, per line group: is this genuinely hard to test, owned elsewhere,
   or merely unselected? (#2149's server-states plan was correctly reopened by exactly this
   question.)
6. **Small signed commits per reviewed increment** — this made a mid-implementation power loss
   nearly free to recover from.

## Required Corrections (Problems Observed in #2149)

### 1. Toolchain-pinned validation commands

The single most expensive defect: ~25 plan rows recorded `cargo fmt --all -- --check ... passed`
from **stable** rustfmt, which only warns about the repository's unstable
`imports_granularity`/`group_imports` options, while CI enforces them with **nightly** rustfmt.
Four external review rounds were consumed before the toolchain divergence was identified.

The spec's validation section must therefore:

- list exact commands **with toolchains** — `cargo +nightly fmt --all -- --check`, never the
  bare `cargo fmt --all -- --check`; and
- require every recorded evidence row to name the toolchain that produced it.

### 2. Complete module inventory as the first task's output

Issue #2149 started with 9 tasks and gained T10-T18 on day 4 only because the maintainer asked "did we
check every module?". The first task of the next subissue must produce a complete inventory: one
row per package source file with its unit-only coverage, where every row terminates in exactly
one of *test-added*, *no-change-with-reason*, or *deferred-with-owner*. Completion of the issue
is measured against that inventory, not against an open-ended queue.

The inventory must also flag **property-test candidates** per seam rather than assessing the
technique once for the whole package. Flag a module when either trigger applies:

- an ADR, module doc, or comment states an invariant in universal terms ("for any sequence of
  pushes, at most one active task is aborted per admission"); or
- the module contains arithmetic, encoding, or conversion logic with natural round-trip, bounds,
  or order-insensitivity properties (incremental averages, make/validate token pairs,
  serialize/parse pairs).

Evidence for the rule: in #2149, `ActiveRequests::force_push`'s ADR-protected exactly-one-eviction
invariant was tested example-based; a reviewer-crafted `break`-to-`continue` mutation survived
those examples for several rounds, and a property over random finished/active buffer states would
likely have caught it immediately. A flagged candidate still requires a recorded selection
decision — including the dependency-addition justification if the workspace lacks a
property-testing crate — and discrete branch policy (2-3 enumerable cases) remains better served
by example-based tests.

### 3. Lightweight per-file decision records

Issue #2149's 19 plan files total ~4,200 lines, roughly 90% of which repeats shared Non-Goals,
validation commands, two-phase rules, and completion criteria (~4 documentation lines per test
line, 81 docs commits versus 36 test commits). For the next subissue:

- keep **one** shared `test-refactor-plans/README.md` holding the two-phase sequence, shared
  guardrails, shared non-goals, and the toolchain-pinned validation commands; and
- replace per-file plans with per-file **decision records** of roughly 40 lines: current state,
  selected contracts, ownership boundaries, review outcomes, and evidence. No repeated
  boilerplate sections.

### 4. Module-level approval gates

Issue #2149 recorded ~82 maintainer approval round trips, one per micro-increment. Replace with two
gates per module: approve the module's decision record before changing its tests, and review the
module's completed result. Intra-module increments proceed without stopping unless the decision
record flags a specific risk. Keep the EPIC's final gate: stop for maintainer review after the
last test-producing increment, before final verification and the PR.

### 5. Verification terms defined in the spec

Two rework cycles came from interpretable terms. The spec must state up front:

- **Manual verification** is a human-oriented interaction with the built artifact (for example,
  starting the tracker executable with an isolated configuration and exercising it with
  `tracker_client`). Running automated tests never satisfies it.
- The issue-local deliverables `manual-verification-evidence.md`,
  `implementation-retrospective.md`, and `agent-review-reports.md` (when independent reviewers
  are used) are named in the workflow checkpoints from day one, not discovered from templates at
  the end.

### 6. No cross-cutting guidance edits on the implementation branch

Issue #2149's branch amended seven skill/pattern/agent files across ten commits. The insights were
valuable; the vehicle was wrong — it grew the review surface mid-review. New process or
test-design insights go to an issue-local `lessons.md`; promoting them into
`.github/skills/` or `docs/testing/` is a separate small PR after the subissue merges.

### 7. Reconciliation task before final verification

Final review of #2149 found stale plan frontmatter (`status: proposed` on completed plans),
`TODO` item labels inside completed plans, and EPIC tables still naming an interim checkpoint.
Add an explicit penultimate task: verify that plan frontmatter, checklists, the plan index, the
issue task table, and the EPIC tables agree; grep for `status: proposed`, stray `TODO`/
`IN_PROGRESS` labels inside completed records, and stale checkpoint names.

### 8. Interruption handoff note

When stopping mid-task for any reason, record a one-paragraph handoff in the issue folder: the
next action, the validation state at stop, and any uncommitted-work locations. Recovery from a
power loss during #2149 succeeded but depended on conversation history rather than durable state.

### 9. Rebase-stable commit citations in evidence

While the branch can still be rebased, cite fix or increment commits in issue-local evidence,
plan records, and PR replies by their **unique Conventional Commit subject** (locatable with
`git log --fixed-strings --grep='<subject>'`), never by branch SHA. #2149's review audit cited
eight fix commits by SHA; two same-day rebases onto `develop` made every citation unreachable and
cost a dedicated review round. SHAs become safe only after merge pins the history. After any
rebase, verify no recorded SHA remains load-bearing.

## Spec-Drafting Checklist for the Next Subissue

- [ ] First task produces the complete per-file module inventory with unit-only coverage.
- [ ] Inventory flags property-test candidates per seam (universal invariants; round-trip/bounds
      arithmetic or encoding), each with a recorded selection decision.
- [ ] Validation commands are toolchain-pinned; evidence rows must name their toolchain.
- [ ] One shared plans README; per-file decision records (~40 lines) instead of full plans.
- [ ] Two approval gates per module (decision record, completed result) plus the final
      pre-PR maintainer review.
- [ ] Manual verification defined as artifact interaction; deliverable artifacts named in the
      checkpoints (`manual-verification-evidence.md`, `implementation-retrospective.md`,
      `agent-review-reports.md`).
- [ ] No-change and deferral decisions must enumerate uncovered lines with per-group reasons.
- [ ] Cross-cutting guidance changes excluded from scope; `lessons.md` collects candidates.
- [ ] Reconciliation task scheduled before final verification.
- [ ] Evidence and replies cite commits by unique subject, not branch SHA, until merge.
- [ ] Bounded mutation assessment scheduled after the final test increment.
- [ ] Separate aggregate/unit-only/integration-only EPIC table updates scheduled at completion.

## References

- Evidence base: `docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/`
  (`ISSUE.md`, `implementation-retrospective.md`, `coverage-evidence.md`,
  `test-refactor-plans/README.md`).
- Review-cost evidence: `docs/pr-review-feedback/pr-2174-review-feedback.md`.
- Related repo-wide follow-up (tooling parity and unified review workflow) is tracked as its own
  issue and is deliberately not part of this EPIC guidance.
