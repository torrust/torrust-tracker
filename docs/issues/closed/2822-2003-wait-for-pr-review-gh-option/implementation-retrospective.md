---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
    - docs/issues/closed/2822-2003-wait-for-pr-review-gh-option/ISSUE.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

# Implementation Retrospective — #2822 `wait-for-pr-review --gh-command`

## Purpose

Record evidence-based process improvements discovered while implementing issue #2822 (PR #2835, with the
specification in PR #2826). This is a blameless review of the implementation approach; it does not replace the
acceptance-criteria verification in `ISSUE.md`.

## Outcome

`wait-for-pr-review` takes `--gh-command <command>` (default `gh`) and runs it for every call to GitHub, so a caller
names the command of its own account and needs no `gh` stand-in on `PATH`. `gh auth status` runs only when the value
is `gh`. `wait_started` reports `gh_command`, and a `gh_identity` record reports the login the command runs as, from a
best-effort `api user` lookup bounded at `min(10 s, timeout / 4)`. Exit codes `0`, `1`, `2` and the JSON-only stderr
are unchanged. Evidence: `manual-verification-evidence.md` (red and green runs, M1 to M4b, the kept fake script) and
the tests in `tests/cli.rs` and `main.rs`.

## What Went Well

1. The merged specification fixed the design before code (D1 and D2, with pros and cons), so the implementation
   needed no design change, and the tests were written before the option; of the ten new tests of the red run, eight
   failed and two passed before the option existed (the evidence says which and why).
2. Tests pass the fake command only through the option for the custom-command cases, with a `PATH` that cannot reach
   it. The default-`gh` scenarios run a fake named `gh` from the sandbox `PATH`, as they must to exercise the default;
   no stand-in that calls `gh` again was ever put on `PATH`, which is the recursion the issue exists to prevent.
3. The identity record answers the maintainer's concern that an agent must be able to tell which account it runs as,
   with data in the output instead of prose.

## What Changed During Implementation

- **The lookup's place and bound changed twice in review of the spec** (PR #2826, five rounds): the login was first
  planned inside `wait_started`, which let a stalled `api user` call delay the record and eat the wait budget. The
  final design writes `wait_started` first, then a separate `gh_identity` record, with the lookup bounded at
  `min(10 s, timeout / 4)` and a stated baseline limit when `--since-head-sha` is not given.
- **An interrupted lookup could start the wait** (Copilot, PR #2835): the first implementation returned from the lookup
  on a signal and `wait` then spawned another command. It now ends the run with `interrupted`.
- **The red-run counts in the evidence did not recompute** (da2ce7, PR #2835): two of the ten new tests passed before the
  option existed, and six existing scenarios failed, not four. The evidence was recomputed from the committed tests.
- **A CLI scenario cannot pin the interrupted-lookup regression**, because the poller kills the spawned child at
  once; a deterministic unit test (`report_identity` against `yes` with the flag set) does.
- **AC5 and D2's test bullet were narrowed** from "the tests" to "the tests of a custom command", because the four
  scenarios that exercise the default `gh` reach the fake through the sandbox `PATH`.
- **The fake-`gh` scenarios of `tests/cli.rs` stayed flaky** (a fake script that fails to start): one failing run in
  seven after the change, not fixed here by design.

## Root Cause

- The specification placed a new child-process call before the baseline snapshot without working out its interplay
  with the single absolute deadline and with the baseline; the design-review checkpoint of `create-issue` names deadline
  coverage, but the plan treated it as "not otherwise applicable" until review asked.
- Counts and claims in the evidence and in the specification's own records were written from memory of the work, not
  recomputed from the run output; most findings of the later review rounds, on both pull requests, were about these
  statements, not about the tool.
- The close-out's first commit replaced the instruction of the merged specification ("Create an issue-local
  `implementation-retrospective.md` when there is any of these", with `create-issue` saying the same) with "No separate
  `implementation-retrospective.md` was created", although the work had deviations, validation findings and lessons.
  The rule was in front of the work; the decision not to follow it was mine, made before the reviews that showed
  it was wrong.
- The audit and log rules (append only, stamp an event after it happened, separate commits for product fixes and audit)
  were applied round by round, after a reviewer named each slip, instead of being checked before pushing.

## Improvements for Future Work

1. When a specification adds a best-effort call before a baseline, state in the design review its bound, its
   cancellation, and what an event during the call does, before the first review.
2. Write red-run and test counts into evidence only after reading the run's output per test, and name which new tests
   did not fail and why.
3. Before pushing a review round, check the audit against the append-only rule and the commit-separation rule, and
   take log stamps from the push log or the forge, not from memory.
4. The retrospective itself was a gap in this work: the close-out first recorded "no retrospective needed". The
   repository's Copilot custom agents under `.github/agents/` have no Claude Code equivalent, so behaviour they
   provided is lost when the agent is Claude Code. Migrating those definitions to the Claude Code agent format is a
   priority raised with the EPIC #2003 coordinator and is out of scope of this issue.

## Avoiding Overcorrection

The evidence does not justify new checks in the tool: the `gh_identity` record, the bound and the baseline limit cover
the identity concern, and the remaining diagnostics (`MalformedResponse`, `DeadlineExceeded`) were declined as out of
the specification's scope. It also does not justify a rule against review rounds on records; the slips were in how
the records were written, not in the rounds.

## Evidence

- `docs/issues/closed/2822-2003-wait-for-pr-review-gh-option/ISSUE.md` and `manual-verification-evidence.md`
- Specification PR #2826 and implementation PR #2835 (merge `388faf59a`), with their review audits
  `docs/pr-reviews/pr-2835-review/PR-REVIEW.md` and, for the close-out, `docs/pr-reviews/pr-2840-review/PR-REVIEW.md`
- Commit subjects `feat(wait-for-pr-review): [#2822] add the --gh-command option and report the identity` and
  `fix(wait-for-pr-review): [#2822] stop on a signal during the identity lookup and test its bound`
