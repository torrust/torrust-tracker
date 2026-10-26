---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/templates/ISSUE.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

# Implementation Retrospective — Migrate the Health-Check API Test Environment (#2450)

## Purpose

Record evidence-based process improvements discovered while implementing
SI-24 (issue #2450). This is a blameless review of the implementation
approach; it does not replace acceptance-criteria verification.

## Outcome

`Environment` runs the health-check API on the token lifecycle, with private
`state`, one `DropGuard`, a joined `stop()` that names every failing task, and a
`bind_address()` accessor. The evidence is in `manual-verification-evidence.md`
(V0 to V7). One limitation remains: a stop-then-start on a fixed port panics,
because the environment's registration stays in the `registar`.

## What Went Well

1. The #2449 sibling was a complete model, so the design review of the new
   environment needed no change.
2. Every new test was mutation-proven before review.

## What Changed During Implementation

Copilot's review of #2652 found that restarting on a fixed port panics with
`DuplicateBinding` (`review-finding:pr-2652-f1`). The legacy
`server::start` never registered the service, while the token-aware
`start_with_cancellation` does, so the migration added a registration that
`stop()` cannot remove: `torrust-server-lib` 0.3.0's `Registar` has no remove
operation. The restart test used `:0`, which hides the problem. The maintainer
chose to document and pin the limitation and file a follow-up; see V7.

## Root Cause

The spec compared the start and stop paths but not the side effects of the
token-aware start. D4 asked for `RuntimeServiceMetadata` and treated the
registration as a given, so no task asked what a restart does to the registry.
The restart test used the one configuration, an ephemeral port, that cannot
collide.

## Improvements for Future Work

1. When a spec migrates a lifecycle that keeps a `stop()` then `start()` path,
   add a task that lists every side effect of `start` (registrations, files,
   channels) and says how `stop` undoes each.
2. For a restart test, run it on a fixed port as well as on `:0`.

## Avoiding Overcorrection

The evidence does not justify changing the `Registar` inside this PR, or a rule
that every test environment must support a fixed-port restart: no caller restarts
this environment on a fixed port.

## Evidence

- `manual-verification-evidence.md` V7 (red test, decision, mutation proof)
- `ISSUE.md` progress log
- Commit subject `test(axum-health-check-api-server): [#2450] document and pin the fixed-port restart limitation`
