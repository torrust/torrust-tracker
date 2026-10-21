---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: null
github-issue: 2603
spec-path: docs/issues/open/2603-write-unit-test-skill-rule-mutations-and-removed-tests/ISSUE.md
branch: "2603-write-unit-test-skill-rule-mutations-and-removed-tests"
related-pr: null
last-updated-utc: "2026-10-21 12:25"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/implementation-retrospective.md
    - docs/issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/agent-review-reports.md
    - docs/issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/manual-verification-evidence.md
---

<!-- skill-link: create-issue -->

# Issue #2603 - Teach the `write-unit-test` Skill to Derive Mutations From Each Rule Clause and to Record Removed Tests

## Goal

Add two short, evidence-backed rules to the `write-unit-test` skill: derive at least one mutation
from each clause of a stated rule, including its negative clauses, and record every removed or
merged test in a refactor whose acceptance criterion is that the existing tests still pass.

## Background

Both rules are proposals from the implementation retrospective of #2415 (SI-22.3, the
per-component cancellation tokens), which left them unapplied because changing a skill needs the
maintainer's approval. See the
[retrospective](../../closed/2415-2410-si-22-3-per-component-cancellation-tokens/implementation-retrospective.md)
(sections "Root Cause" and "Improvements for Future Work") and the independent
[Task Reviewer report](../../closed/2415-2410-si-22-3-per-component-cancellation-tokens/agent-review-reports.md).

**Observation 1: mutations were chosen by mechanism, not by clause.** The spec of #2415 says a
dropped unspawned reservation "must release its state without cancelling unrelated components",
which has two clauses. The T6a mutations were chosen by asking how each mechanism could break
(no release, no cancel, a foreign reservation accepted, and so on), so only the first clause had a
mutation. The release test asserted that the other component's token was cancelled after `cancel()`,
which also holds if the drop had cancelled it. The Task Reviewer found this (finding S2). The
issue's evidence file records the result: a mutation that cancels only the other components on drop
passed the old test and failed the strengthened one (V4 of
[manual-verification-evidence.md](../../closed/2415-2410-si-22-3-per-component-cancellation-tokens/manual-verification-evidence.md)).
The skill already asks for mutations, but its section "Prove a Regression Test Guards the Bug" is
written for bug regressions, so it did not prompt this for a feature or refactor whose rule comes
from a spec.

**Observation 2: a removed test was not recorded.** Converting the `JobManager` tests to
reservations deleted `it_should_report_cooperatively_cancelled_jobs`, because it became the same test
as a new one, and the manager's test count went from 17 to 16. The acceptance criterion was "all
existing tests pass", yet neither the evidence, the commit message, nor the progress log said so;
the Task Reviewer found it (finding S1) and the record was added afterwards. Nothing in the skill
mentions removing or merging tests.

Each observation is a single instance. Whether that is enough evidence for a rule is Open Question
Q1.

## Scope

### In Scope

- A new section of `.github/skills/dev/testing/write-unit-test/SKILL.md` that asks for at least one
  mutation per clause of a rule the test claims to guard, including negative clauses such as
  "without affecting", "is not lost", and "only by", with a short evidence format (clause, mutation,
  failing tests). It links to the existing mutation guidance instead of repeating it.
- A short rule in the same skill: before removing or merging a test in a refactor whose acceptance
  criterion is that the existing tests still pass, name the test that now covers its behavior (or
  give the reason it is dropped), and record the removal and the new test count in the evidence and
  in the commit message.
- Matching entries in the skill's "Quick Checklist".

### Out of Scope

- Changes to the Task Reviewer agent, the `review-task` skill, or `docs/templates/ISSUE.md` to check
  these rules (Open Question Q3).
- Adopting a mutation-testing tool such as `cargo-mutants`. The retrospective found that targeted,
  recorded mutations located both gaps; the problem was choosing them, not running them.
- Rewriting or renaming the existing section "Prove a Regression Test Guards the Bug".
- Any change to product code or tests.

## Architectural Decisions

- Related ADRs: none. This changes agent guidance only. Repository policy 6 (documentation single
  source of truth) governs how the new text relates to the existing mutation guidance: link, do not
  duplicate.
- ADRs to create: `None known`.

## Design and Ownership Review

Not applicable: no child processes, asynchronous I/O, resource cleanup, or reusable test fixtures
are involved.

## Bug-Fix Process

Not applicable: this is a process improvement, not a defect fix.

## Regression Test Strategy

Not applicable: no behavior changes. Verification is by walk-through against the real #2415 artifacts
(see the manual verification scenarios).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | DONE   | Settle the open questions Q1 to Q4 with the maintainer | Each answer is recorded under "Decisions on the Open Questions" and in the progress log; the proposed text needed no change. |
| T2  | DONE   | Add the per-clause mutation section to the `write-unit-test` skill | A section next to "Prove a Regression Test Guards the Bug", linking to it for the compile check and the evidence rule. Committed after the maintainer approved the wording. |
| T3  | DONE   | Add the removed-or-merged-test rule to the skill | A short section right after the skill's test-inventory section, plus its "Quick Checklist" entry (the row said "two entries"; the Commit Points and AC3 mean one per rule, which is what was done). Committed after the maintainer approved the wording. |
| T4  | DONE   | Walk the two rules through the #2415 artifacts (M1, M2) | Both scenarios recorded in `manual-verification-evidence.md` (V1, V2). |
| T5  | TODO   | Validate and review | `linter all`, the frontmatter check, completion review, and the independent Task Reviewer report. |

## Commit Points

| Task  | Coherent change set | Commit policy |
| ----- | ------------------- | ------------- |
| T2    | The per-clause mutation section and its checklist entry in the skill | Commit after `linter all` and the maintainer's review of the wording. |
| T3    | The removed-or-merged-test rule and its checklist entry in the skill | Commit after `linter all` and the maintainer's review of the wording. |
| T4-T5 | Evidence, progress log, and review reports | Commit after the walk-throughs are recorded. |

Documentation-only work, so the `write-unit-test` test-increment loop does not apply. Sign every
commit with GPG.

## Proposed Skill Text (Starting Point for T2 and T3)

The wording below matches the maintainer's decisions on Q1 to Q4. It is the starting point for T2
and T3, and the maintainer reviews the final wording before each commit. It is not final.

### Per-clause mutations (T2)

> ### Prove a Test Guards Each Clause of a Rule
>
> "Prove a Regression Test Guards the Bug" applies to a bug. It applies in the same way to any test
> that claims to guard a rule the repository states, such as a lifecycle rule in an issue
> specification or an invariant in an ADR.
>
> 1. **List the clauses.** Split the rule as written into its clauses, and include the negative ones
>    ("without cancelling unrelated components", "a stop request is not lost", "only by the manager
>    that issued it"). A negative clause is the one most often left without a mutation.
> 2. **One mutation per clause.** For each clause, write a mutation that breaks that clause and
>    nothing else, and run the tests as described above, including the check that it compiled. A
>    mutation that breaks several clauses at once cannot show which clause a test guards. On the rule
>    "a dropped reservation releases its state without cancelling unrelated components", making a
>    drop cancel every token breaks both clauses: the released reservation's own token is cancelled,
>    and so are the others. An assertion on the released token then fails the test and hides that the
>    other clause has no assertion. Making a drop cancel only the other tokens breaks just the second
>    clause, and exposes the missing assertion.
> 3. **Record it.** In the evidence, list each clause, its mutation, and the tests that failed. A
>    clause with no failing test is a gap in the tests, not a pass.
> 4. **Bound it.** Apply this to rules stated in a specification, ADR, or acceptance criterion, not
>    to every line of code.

### Removed or merged tests (T3)

> **Removing or merging a test in a no-behavior-change refactor.** When the acceptance criterion is
> that the existing tests still pass, a deleted or merged test changes the evidence. Before removing
> one, name the test that now covers its behavior, or state why the behavior is dropped. Record the
> removal, the replacement, and the new test count in the issue's evidence and in the commit message.

Checklist entries:

> - [ ] For a rule with several clauses (including negative ones): each clause has a mutation that breaks only it, and the failing tests are recorded
> - [ ] For a refactor that removes or merges a test: the replacement and the new test count are recorded in the evidence and the commit message

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/write-unit-test-skill-rule-mutations-and-removed-tests/ISSUE.md`, then moved to `docs/issues/open/2603-write-unit-test-skill-rule-mutations-and-removed-tests/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional) Spec-only PR merged into `develop` before implementation (PR #2604)
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 10:05 UTC - GitHub Copilot (Claude Sonnet 5.5) - Drafted from the two proposals in the #2415 implementation retrospective, at the maintainer's request. No GitHub issue exists yet.
- 2026-10-21 10:37 UTC - GitHub Copilot (Claude Sonnet 5.5) - The maintainer approved the draft. GitHub issue #2603 was created (label `task`), and this spec moved from `docs/issues/drafts/` to `docs/issues/open/2603-write-unit-test-skill-rule-mutations-and-removed-tests/`; `status` is `planned`.
- 2026-10-21 10:38 UTC - GitHub Copilot (Claude Sonnet 5.5) - The maintainer answered Q1 to Q4 as recommended (one instance is enough for both rules, a new sibling section, no reviewer change, keep the bound) and chose a spec-only pull request first: branch `2603-write-unit-test-skill-rule-mutations-and-removed-tests-spec`, `Related to #2603`. The skill change follows on the reserved branch `2603-write-unit-test-skill-rule-mutations-and-removed-tests`. T1 is `DONE`. No wording change.
- 2026-10-21 10:41 UTC - GitHub Copilot (Claude Sonnet 5.5) - Opened the spec-only PR #2604 (`Related to #2603`).
- 2026-10-21 11:06 UTC - GitHub Copilot (Claude Sonnet 5.5) - Copilot's review of PR #2604 has one finding (`review-finding:pr-2604-f1`): the per-clause mutation example named a clause the drop rule does not have. It is fixed in the proposed skill text, and the review is processed in `docs/pr-reviews/pr-2604-review/PR-REVIEW.md` (replied to and resolved, no unresolved thread). The correction changes the example only, not the rules or the maintainer's decisions on Q1 to Q4.
- 2026-10-21 11:34 UTC - GitHub Copilot (Claude Sonnet 5.5) - PR #2604 merged on 2026-10-21 at 11:32 UTC; issue #2603 stays open. Implementation started on the reserved branch `2603-write-unit-test-skill-rule-mutations-and-removed-tests`, cut from the merged `develop`.
- 2026-10-21 11:50 UTC - GitHub Copilot (Claude Sonnet 5.5) - T2 and T3 written in the skill and shown to the maintainer, who approved the wording (verdict `approve`, no changes). Two things differ from the agreed text, both disclosed at the review: each new section cites the #2415 observation in one sentence, and the removed-test rule is its own `###` section instead of a paragraph. Committed as `docs(skills): [#2603] ask for a mutation per clause of a rule a test guards` and `docs(skills): [#2603] ask for a record when a refactor removes or merges a test`. The T3 plan row asked for two checklist entries, but the Commit Points and AC3 mean one per rule; one each was added, and the row is corrected. No design change.
- 2026-10-21 11:57 UTC - GitHub Copilot (Claude Sonnet 5.5) - T4 done: M1 and M2 recorded in `manual-verification-evidence.md` (V1, V2). M2 flags the test the #2415 conversion removed (manager tests 17 to 16, no record in the commit message). M1 yields the mutation the original release test missed, but was not blind as the scenario asked: I wrote the #2415 finding up and knew it, and the mutation outcomes are cited from #2415's evidence, not re-run. Both limits are in V1. Next: pre-push checks, the Task Reviewer report, and the completion review.
- 2026-10-21 12:25 UTC - GitHub Copilot (Claude Sonnet 5.5) - Independent Task Reviewer report recorded in `agent-review-reports.md`: REVIEW PASSED (with findings), AC1 to AC4 and `linter all` pass, nothing blocks the push. Its findings are addressed: the spec-only-PR checkpoint is ticked (S1), `branch:` names the implementation branch (N1; `related-pr` follows when the pull request is opened), V2 now says its step 4 checks existence only (N2), and the checklist wording stays as the maintainer approved it (N3). Completion review: no separate retrospective, for the reason in the section above. No design change.

## Acceptance Criteria

- [ ] AC1: the `write-unit-test` skill asks for at least one mutation per clause of a rule a test claims to guard, names negative clauses explicitly, and defines what the evidence records. It links to the existing mutation guidance instead of repeating it.
- [ ] AC2: the skill asks that a removed or merged test, its replacement, and the new test count be recorded in the evidence and in the commit message when the acceptance criterion is that the existing tests still pass.
- [ ] AC3: the "Quick Checklist" of the skill has an entry for each rule.
- [ ] AC4: the two walk-throughs (M1 and M2) were run on the real #2415 artifacts and show that each rule would have prompted the missing action.
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all` (Markdown, link, and spelling checks of the skill and this folder)
- The staged-Markdown frontmatter check that the pre-commit hook runs on the changed skill file and on this folder

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | The per-clause rule on the #2415 release rule | Without looking at the Task Reviewer report, apply the new section to the sentence "a dropped unspawned reservation must release its state without cancelling unrelated components" in the #2415 spec. List the clauses and one mutation for each, then compare with the mutations recorded in V1 and V4 of the #2415 evidence. | The clause list contains the negative clause, and its mutation is one that passes the original release test and fails the strengthened one. | DONE (not blind, see V1) | `manual-verification-evidence.md` section V1 |
| M2 | The removed-test rule on the #2415 conversion | Take the diff of the commit `refactor(bootstrap): [#2415] give every application component its own reservation token` and apply the new rule. List the tests it removes and the record the rule asks for. | The rule flags the removal of `it_should_report_cooperatively_cancelled_jobs` and names the test that replaced it. | DONE | `manual-verification-evidence.md` section V2 |

Notes:

- These are walk-throughs of real, merged artifacts, not new runs of the tracker. They show the
  rules are specific enough to prompt the action; whether agents then follow them is not measured
  here (see Risks).
- Record every validation command result with the toolchain or runtime that produced it when it can
  affect behavior.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
  when executing these scenarios.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   | The section "Prove a Test Guards Each Clause of a Rule" in the skill; evidence V1 |
| AC2   | TODO                   | The section "Remove or Merge a Test Only With a Record" in the skill; evidence V2 |
| AC3   | TODO                   | Two entries in the skill's "Quick Checklist" |
| AC4   | TODO                   | Evidence V1 and V2, with the limits stated in V1 |

## Decisions on the Open Questions

The maintainer decided Q1 to Q4 on 2026-10-21, each as recommended.

- **Q1: is one observed instance enough for each rule?** The evidence is #2415 only: one weak
  assertion and one unrecorded removal. Recommendation: yes for the removed-test rule, which costs
  one line of evidence per removal, and yes for the per-clause rule, which extends a section the
  skill already has. Alternative: wait for a second occurrence. **Decision: yes, for both rules.**
- **Q2: where does the per-clause rule go?** Recommendation: a new sibling section after "Prove a
  Regression Test Guards the Bug", leaving the existing section unchanged. A `git grep` for the
  heading and its anchor on 2026-10-21 found no link to it, so renaming it would also be possible.
  Alternative: generalize that section in place and rename it. **Decision: a new sibling section.**
- **Q3: should a reviewer check these rules?** The Task Reviewer found both gaps by reading, so the
  skill text alone may be enough. Recommendation: no change to the reviewer in this issue. Alternative:
  add a line to the `review-task` skill and the Task Reviewer agent, as a follow-up. **Decision: no reviewer change in this issue.**
- **Q4: is the "bound it" clause right?** It limits the rule to rules stated in a specification, ADR,
  or acceptance criterion, to avoid turning every test into a mutation exercise. Alternative: no
  bound, leaving the author to judge. **Decision: keep the bound.**

## Risks and Trade-offs

- **Overcorrection.** The skill is already about 500 lines, and each added rule is one more thing
  to read. Mitigation: two short rules, each tied to one observed failure, with a bound on the first
  (Q4). Do not add rules that the evidence does not support.
- **Cost of per-clause mutations.** A rule with many clauses needs many mutations. Mitigation: the
  bound, and the fact that a mutation is a one-line change that is restored afterwards.
- **No measurement that agents follow the rules.** The walk-throughs show the rules can prompt the
  missing action, not that they will. The next issue that states a multi-clause rule is the real
  check; the completion review of that issue should say whether the rule helped.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions,
material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: none needed (assessed 2026-10-21, see the progress log). The independent review found only documentation items, the maintainer approved the wording unchanged, and the two small lessons (the M1 scenario could not be blind for the author, and the T3 row said "two" checklist entries where the Commit Points meant one per rule) are already recorded in `manual-verification-evidence.md` and the progress log.
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add a concise
  progress-log entry explaining why none was needed.
- The independent reviewer records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Source: the [implementation retrospective](../../closed/2415-2410-si-22-3-per-component-cancellation-tokens/implementation-retrospective.md)
  of #2415, merged in PR #2591.
- Skill to change: `.github/skills/dev/testing/write-unit-test/SKILL.md`
- Issue: #2603
- Related issues: #2415
