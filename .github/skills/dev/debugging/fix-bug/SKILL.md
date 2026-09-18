---
name: fix-bug
description: Canonical workflow for investigating and fixing bugs in torrust-tracker. Use when work is substantively a bug, even if issue metadata or labels are missing or wrong. Requires source analysis, real-artifact reproduction or infeasibility evidence, regression-test selection, red/green validation, code fix, and like-for-like final recheck.
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - .github/skills/dev/planning/create-issue/SKILL.md
      - .github/skills/dev/testing/write-unit-test/SKILL.md
      - .github/agents/implementer.agent.md
      - docs/templates/ISSUE.md
      - issue #2226
---

# Fixing Bugs

Use this skill whenever the requested work is substantively a bug: observed behavior is broken,
incorrect, stale, misleading, unexpectedly failing, or regressed. Apply it even when the issue type,
GitHub label, branch name, or user wording does not say `bug`.

This skill is the canonical bug-fix workflow. Other artifacts should state when to load it and what
sections or evidence are required, but should not duplicate the operational process.

## Required Sequence

Complete these steps in order unless a step is infeasible. When a step is infeasible, record why,
what was attempted, and the strongest substitute evidence before moving on.

1. **Analyze the defect.** Read the issue, reproduction notes, nearby code, and relevant tests.
   Identify the code path that decides the broken behavior and write down the current local
   hypothesis.
2. **Reproduce against a real artifact.** Run the application, CLI, test binary, script, or command
   that demonstrates the bug as a user or operator would see it. Record the exact command,
   toolchain/runtime, relevant output, and logs in issue-local `manual-verification-evidence.md`.
3. **Select the regression-test boundary.** Choose the smallest deterministic maintained test that
   can fail for the defect and pass for the fix. Prefer a unit test at the causal seam. Use an
   integration or end-to-end test only when it is the clearer or only practical boundary, and record
   the rationale.
4. **Write the red regression test.** Add the maintained automatic test before changing production
   behavior when practical. Run it against the broken implementation and record the failing command
   and output.
5. **Fix the code.** Make the smallest production change that addresses the root cause. Keep the
   change scoped to the behavior under investigation.
6. **Verify green and recheck like-for-like.** Rerun the regression test and any focused affected
   tests. Then rerun the original real-artifact reproduction, or record the infeasibility constraint
   and strongest substitute evidence if the original reproduction cannot be repeated.

A passing automated test is not a substitute for the final real-artifact recheck. The recheck proves
that the artifact-level symptom observed at the start is gone.

## Evidence Requirements

Record evidence in the issue folder so later reviewers can audit what happened without relying on
chat history or retained agent state.

For every bug, create or update `manual-verification-evidence.md` from
`docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`. Include:

- the original symptom and the current hypothesis;
- the real-artifact reproduction command or interaction, with actual output and relevant logs;
- the toolchain, runtime, configuration, database backend, container image, service URL, or other
  environment details that can affect behavior;
- the regression-test boundary selected and why it is the smallest deterministic maintained test;
- red regression-test output when a maintained test is feasible;
- green regression-test output after the fix;
- the final like-for-like manual recheck output; and
- when reproduction or a maintained regression test is infeasible, the attempted commands,
  blocking constraint, and strongest substitute evidence.

Use issue-local notes, not memory, as the source of truth. Traceability and accountability are part
of the project quality bar.

## Regression-Test Boundary Rules

Prefer this order:

1. **Unit test** at the causal decision seam: fast, deterministic, isolated, and directly explains
   the defect.
2. **Collaboration or integration test** when the behavior is only observable through multiple
   project components or a collaborator-owned rule.
3. **End-to-end test** when the defect is about process boundaries, protocol integration,
   persistence backends, deployment behavior, or real client interaction.
4. **Manual-only evidence** only when no maintained automatic regression test is practical. Record
   why the code cannot be protected appropriately by a maintained test and what follow-up would make
   it testable.

When writing or changing unit tests, use `.github/skills/dev/testing/write-unit-test/SKILL.md`.
For regression tests, prove the test guards the bug by observing it fail against the bug or by
recording why that red check cannot be performed.

## Issue-Spec Requirements for Bugs

Bug specifications must include these sections, even when metadata or labels do not identify the
work as a bug:

- `Bug-Fix Process` — the planned or completed analysis, reproduction, regression-test, fix, and
  recheck steps, linked to this skill.
- `Regression Test Strategy` — the selected maintained test boundary, rationale for that boundary,
  and any infeasibility constraints.

The issue spec must also require issue-local `manual-verification-evidence.md` for the initial
reproduction and final recheck. If reproduction is not possible, the file records the attempted
commands, blocking constraint, and strongest substitute evidence.

## Worked Example

Use the stale activity-metrics cutoff bug as a review-only worked example:

- Issue: issue #2226
- Evidence: the issue-local evidence artifacts attached to issue #2226.

Do not change that issue's implementation scope when updating this workflow.

## Skill Links

Review this skill when changing:

- `.github/skills/dev/planning/create-issue/SKILL.md` — bug issue-spec requirements and evidence
  policy.
- `.github/skills/dev/testing/write-unit-test/SKILL.md` — maintained regression-test design and red
  test evidence.
- `.github/agents/implementer.agent.md` — bug workflow invocation by implementation agents.
- `docs/templates/ISSUE.md` — bug-only issue sections and manual evidence requirements.
- issue #2226 — worked example reference.
