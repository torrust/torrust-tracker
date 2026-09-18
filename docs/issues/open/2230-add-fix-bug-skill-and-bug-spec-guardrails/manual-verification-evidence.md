---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/ISSUE.md
last-updated-utc: 2026-09-18 15:45
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the workflow change for issue #2230. This evidence
covers review scenarios for the new bug-fix skill, issue-template guardrails, and Implementer
trigger behavior.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-18 15:36-15:45 UTC
- Artifact under test: repository documentation, Agent Skill, issue template, and Implementer agent
- Operating system / environment: Linux workspace in VS Code
- Prerequisites and setup performed: branch `2230-add-fix-bug-skill-and-bug-spec-guardrails`; issue
  spec feedback commit `0b7780c9` already present

## Verification Processes

### V1 - Plan a Bug Regardless of Metadata

- Goal: Verify that a sample specification whose metadata is not `issue-type: bug` still visibly
  records the bug-fix workflow when its substance is broken behavior.
- Initial state: no issue-local sample bug draft existed for issue #2230.
- Status: `DONE`

#### Steps Performed

1. Created `docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/sample-substantive-bug-spec.md`.
2. Set the sample frontmatter to `issue-type: task` while describing a stale counter bug.
3. Added `Bug-Fix Process` and `Regression Test Strategy` sections that link to
   `.github/skills/dev/debugging/fix-bug/SKILL.md`.
4. Ran repository-supported Markdown and spelling validation.

#### Observed Result

```text
2026-09-18T15:37:51.928791Z  INFO markdown: Scanning markdown files...
2026-09-18T15:37:55.455001Z  INFO markdown: All markdown files passed linting! (3.526s)
2026-09-18T15:37:55.457318Z  INFO cspell: Running spell check on all files...
2026-09-18T15:37:59.691799Z  INFO cspell: All files passed spell checking! (4.234s)
```

The sample draft contains both required bug-only sections and explicitly states that `fix-bug`
applies because the described stale value is a bug even though the metadata says `issue-type: task`.

#### Conclusion

The sample visibly records all six bug-fix steps, selects a unit-first regression-test boundary, and
links evidence expectations to issue-local `manual-verification-evidence.md`. This satisfies manual
scenario M1.

### V2 - Start Bug Implementation

- Goal: Verify that the Implementer workflow loads `fix-bug` for a substantively bug-shaped spec
  even when the metadata says `task`, and that it requires reproduction-or-infeasibility evidence,
  red/green regression-test evidence when feasible, and final like-for-like recheck evidence.
- Initial state: sample spec existed with `issue-type: task` and stale-value bug substance.
- Status: `DONE`

#### Steps Performed

1. Asked the `Implementer` agent to perform a review-only inspection of:
   - `.github/agents/implementer.agent.md`
   - `.github/skills/dev/debugging/fix-bug/SKILL.md`
   - `docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/sample-substantive-bug-spec.md`
2. Instructed the agent not to edit files, run commands, or commit.
3. Asked whether it would load and apply `fix-bug` for the sample despite `issue-type: task`.

#### Observed Result

```text
Yes. If asked to implement the sample specification, the Implementer workflow would load and apply
.github/skills/dev/debugging/fix-bug/SKILL.md despite issue-type: task.

Controlling rule: .github/agents/implementer.agent.md, Required Workflow, Step 1 says to decide
whether the work is substantively a bug: broken, incorrect, stale, misleading, unexpectedly failing,
or regressed behavior and, if so, load and apply .github/skills/dev/debugging/fix-bug/SKILL.md,
even when the issue metadata or labels are missing or wrong.

Yes, I would require issue-local manual-verification-evidence.md before completion, including
reproduction-or-infeasibility evidence, red/green regression-test evidence when feasible, and final
like-for-like recheck evidence.
```

#### Conclusion

The Implementer agent recognizes the semantic bug trigger and preserves the required evidence model.
This satisfies manual scenario M2.

## Failures and Follow-up

An attempted focused linter invocation failed because the repository `linter markdown` and
`linter cspell` subcommands do not accept individual file arguments:

```text
error: unexpected argument '.github/skills/dev/debugging/fix-bug/SKILL.md' found

Usage: linter markdown
```

The supported repository commands `linter markdown` and `linter cspell` were run immediately after
and passed.
