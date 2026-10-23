---
semantic-links:
  related-artifacts:
    - .github/agents/task-reviewer.agent.md
    - docs/issues/closed/2603-write-unit-test-skill-rule-mutations-and-removed-tests/ISSUE.md
---

# Agent Review Reports - Teach the write-unit-test Skill to Derive Mutations From Each Rule Clause and to Record Removed Tests

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-21 12:19 UTC - Task Reviewer (before push)

- Invocation scope: the whole branch `2603-write-unit-test-skill-rule-mutations-and-removed-tests` before any push or pull request: its three signed commits on `develop` at `Merge torrust/torrust-tracker#2604: docs(issues): add issue specification for #2603`, ending with `docs(issues): [#2603] record the walk-throughs of the two skill rules`, against #2603's acceptance criteria AC1 to AC4 (left open for the maintainer), the plan, the Commit Points, and the agreed skill wording. Read-only.
- Inputs: the spec (scope, acceptance criteria, plan, "Proposed Skill Text", the decisions on Q1 to Q4, Risks, and the progress log); `manual-verification-evidence.md` V1 and V2; the diff of `.github/skills/dev/testing/write-unit-test/SKILL.md` read in the context of the sections around it; the #2415 implementation retrospective, Task Reviewer report, and evidence V1 and V4; the conversion commit on `develop`, found by its subject.
- Evidence:
  - Re-ran the V2 commands (steps 1 to 5) with `git` and `grep`: the output is identical to V2's "Observed Result" (one test removed, `manager.rs` 17 to 16, the replacement present before the commit, no record in the commit message).
  - Compared the bodies of the removed test and its replacement: the same Act and Assert, one on the legacy root token and one on the reservation token.
  - `linter all`: exit 0. The frontmatter validator on the 2603 folder and the skill file: exit 0. `linter lychee` resolved the anchor `#prove-a-regression-test-guards-the-bug`.
  - The `SKILL.md` diff has 0 deleted lines, so the existing section "Prove a Regression Test Guards the Bug" is untouched; three files changed (the skill, `ISSUE.md`, the new evidence file).
  - Read from source, not run: the logic of the drop-rule example against the strengthened test in `src/bootstrap/jobs/manager.rs` (cancelling every token trips the assertion on the dropped token; cancelling only the other tokens trips the assertion that the drop left the kept token uncancelled).
  - All three commits are GPG-signed (`%G?` is `U`: a good signature, key not locally trusted); `last-updated-utc` is not later than the commit that carries it; no branch commit id is cited in the changed files.
- Findings:
  - AC1 to AC4 and `linter all`: pass. AC4 is qualified: V1 was not blind and cites the mutation outcomes instead of re-running them, which V1 and the spec's M1 row both say. The skill text matches the agreed wording apart from two deviations that the progress log disclosed (a `###` section, and a one-sentence citation of #2415 in each new section). No scope creep and no duplication of the existing mutation guidance.
  - S1 (should-fix): the workflow checkpoint "(Optional) Spec-only PR merged into `develop` before implementation" was not ticked although PR #2604 had merged.
  - N1: the front matter `branch:` still named the spec-only branch, and `related-pr` was `null`.
  - N2: V2 step 4 checks that the replacement test exists before the conversion commit, not that it covers the same behavior; the equivalence rests on #2415's evidence and was not re-compared there.
  - N3: the section title "Only With a Record" and its checklist entry are broader than the section's condition (a refactor whose acceptance criterion is that the existing tests pass). This is the wording the maintainer approved.
- Verdict: REVIEW PASSED (reported as "REVIEW PASSED WITH FINDINGS"; nothing blocks the push).
- Follow-up actions:
  - S1: ticked in the spec (done).
  - N1: `branch:` now names the implementation branch `2603-write-unit-test-skill-rule-mutations-and-removed-tests`; `related-pr` is set when the pull request is opened.
  - N2: V2 now says step 4 checks existence only, that #2415's evidence states the equivalence, and that the Task Reviewer compared the two bodies (done).
  - N3: none. The maintainer approved the wording; the section's first sentence states the narrower condition.
