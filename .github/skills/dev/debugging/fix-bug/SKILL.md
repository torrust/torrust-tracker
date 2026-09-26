---
name: fix-bug
description: Canonical workflow for investigating and fixing bugs in torrust-tracker. Use when work is substantively a bug, even if issue metadata or labels are missing or wrong. Requires source analysis, real-artifact reproduction or infeasibility evidence, regression-test selection, red/green validation, code fix, and like-for-like final recheck.
metadata:
  author: torrust
  version: "1.1"
  semantic-links:
    related-artifacts:
      - .github/skills/dev/planning/create-issue/SKILL.md
      - .github/skills/dev/testing/write-unit-test/SKILL.md
      - .github/agents/implementer.agent.md
      - docs/templates/ISSUE.md
      - "issue #2226"
      - "issue #2345"
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
4. **Prove the regression test is red.** Add the maintained automatic test before changing
   production behavior when practical, run it against the broken implementation, and record the
   failing command and output. If the fix already exists, use the `write-unit-test` skill's
   mutate-then-restore method: reintroduce the bug without staging it, observe the test fail, and
   restore the production file. Try the nearest plausible bug variants when stale or cached state
   could make a weaker test pass.
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
- red regression-test output for every maintained regression test;
- green regression-test output after the fix;
- the final like-for-like manual recheck output; and
- when real-artifact reproduction is infeasible, or no maintained regression test is practical,
  the attempted commands, blocking constraint, and strongest substitute evidence.

Identify PR-branch code states by Conventional Commit subject, never by branch commit id: every
rebase onto `develop` rewrites the ids and orphans the cited objects, so a merged evidence file
would point at commits no clone of `develop` carries. Commit ids are durable only for commits
already on `develop`, tags, or external repositories.

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

When writing or changing unit tests, use
[write-unit-test](../../testing/write-unit-test/SKILL.md). For regression tests, prove the test
guards the bug by observing it fail against the bug. Use mutate-then-restore when the fix is already
present. Record infeasibility only when no maintained automatic regression test is practical, not as
a substitute for proving an existing test fails against the bug.

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

### Explain the Bug in Plain Terms

The spec background must explain the bug so a reviewer understands it without reading the code:

- what happens, step by step from the input to the wrong outcome;
- which contract, documentation, or expectation it violates; and
- the impact, including whether anything observable is wrong today.

### Reproduce Before Review

Attempt the reproduction while drafting the spec, before asking the maintainer to review it, and
record it in `manual-verification-evidence.md`. State the outcome as exactly one of:

- **Reproduced**: the wrong outcome itself was observed.
- **Trigger only**: the faulty code path was reached, but the wrong outcome was not observed. This
  is not a reproduction. Say what prevented observing the outcome and what would observe it.
- **Infeasible**: record the attempted commands, blocking constraint, and strongest substitute
  evidence.

When the wrong outcome is internal (a published event, cached value, or internal state) and no
public surface (client response, metrics, logs) exposes it, a temporary test that captures the
wrong value at the nearest observation seam counts as a reproduction. Record its code verbatim in
the evidence file, revert it, and plan to turn it into the maintained regression test.

### Plan the Regression Test Explicitly

The implementation plan must contain a regression-test task whose expected output is the recorded
red run before the fix, followed by separate fix and green-plus-recheck tasks.

### Pre-Review Self-Check

Before presenting a bug spec for review, confirm:

- [ ] the background contains the plain-language explanation;
- [ ] the reproduction outcome is classified and recorded in `manual-verification-evidence.md`;
- [ ] the plan has a regression-test task that is proven red before the fix;
- [ ] the acceptance criteria match the planned tests (same cases and inputs);
- [ ] every verification row describes what was actually run, not an intended procedure; and
- [ ] the plan follows the Required Sequence order.

## Worked Example

Use the stale activity-metrics cutoff bug as a review-only worked example:

- Issue: issue #2226
- Evidence: the issue-local evidence artifacts attached to issue #2226.

For a bug whose wrong outcome is internal and invisible to clients, metrics, and logs, see
issue #2345: its evidence separates a trigger-only real-tracker run from a temporary event-bus
test that observed the wrong value.

Do not change the implementation scope of either issue when updating this workflow.

## Skill Links

Review this skill when changing:

- `.github/skills/dev/planning/create-issue/SKILL.md` — bug issue-spec requirements and evidence
  policy.
- `.github/skills/dev/testing/write-unit-test/SKILL.md` — maintained regression-test design and red
  test evidence.
- `.github/agents/implementer.agent.md` — bug workflow invocation by implementation agents.
- `docs/templates/ISSUE.md` — bug-only issue sections and manual evidence requirements.
- issue #2226 — worked example reference.
- issue #2345 — worked example reference for an internal wrong outcome; re-check the reference
  when its evidence is updated or moved to `docs/issues/closed/`.
