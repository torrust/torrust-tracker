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
`state`, one `DropGuard`, a joined `stop()` that names every failing task, a
`bind_address()` accessor, and a `stop()` that removes the environment's own
registration from the `registar`, so a stopped environment starts again also on
a fixed port. The evidence is in `manual-verification-evidence.md` (V0 to V7).
One limitation remains: a bare drop cannot await the registry lock, so only
`stop()` removes the registration.

## What Went Well

1. The #2449 sibling was a complete model, so the design review of the new
   environment needed no change.
2. Every new test was mutation-proven before review.
3. Two independent reviewers (Copilot and da2ce7) found the same defect, which
   no earlier check had covered.

## What Changed During Implementation

Copilot's review of #2652 found that restarting on a fixed port panics with
`DuplicateBinding` (`review-finding:pr-2652-f1`). The legacy
`server::start` never registered the service, while the token-aware
`start_with_cancellation` does, so the migration added a registration that
`stop()` could not remove: `torrust-server-lib` 0.3.0's `Registar` had no remove
operation. The restart test used `:0`, which hides the problem.

The first response was a documented limitation plus a pinning test, with a
registry change left for a follow-up. da2ce7 re-raised the finding
(`review-finding:pr-2652-f2`) and asked for the fix, because AC4 promised a
restart without a port condition. A second look found that the registry change was
small, so the fix went into this PR: `Registar::deregister` in
`torrust-server-lib` 0.4.0, then `stop()` deregisters its own binding. The first
decision was reversed, and the pinning test was replaced by a green fixed-port
restart test. See V7.

## Root Cause

The spec compared the start and stop paths but not the side effects of the
token-aware start. D4 asked for `RuntimeServiceMetadata` and treated the
registration as a given, so no task asked what a restart does to the registry.
The restart test used the one configuration, an ephemeral port, that cannot
collide. The first fix choice then weighed the registry change as large before
its size was measured: counting the `register` call sites and reading
`Registar` showed an additive method was enough.

## Improvements for Future Work

1. When a spec migrates a lifecycle that keeps a `stop()` then `start()` path,
   add a task that lists every side effect of `start` (registrations, files,
   channels) and says how `stop` undoes each.
2. For a restart test, run it on a fixed port as well as on `:0`.
3. Before choosing to document a limitation in place of a fix, measure the cost
   of the fix (call sites, API surface) and write it down; an AC that states no
   condition is a promise the limitation breaks.

## Avoiding Overcorrection

The evidence does not justify the receipt-returning `register` (a breaking
change to about eight production call sites for no present need), shutdown-time
deregistration for real services (the maintainer decided shutdown stays decoupled
from registration for now), or a rule that every test environment must support a
restart after a bare drop.

## Evidence

- `manual-verification-evidence.md` V7 (red test, both decisions, mutation proofs)
- `ISSUE.md` progress log and D6
- `torrust/torrust-server-lib#2` (the `Registar::deregister` change)
- Commit subjects
  `fix(axum-health-check-api-server): [#2450] deregister the health-check test environment on stop`
  and `chore(deps): [#2450] bump torrust-server-lib to 0.4.0`
