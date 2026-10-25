---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/templates/ISSUE.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

# Implementation Retrospective — Migrate the REST API Test Environment (#2449)

## Purpose

Record evidence-based process improvements discovered while implementing
SI-23 (issue #2449). This is a blameless review of the implementation
approach; it does not replace acceptance-criteria verification.

## Outcome

`Environment` in `packages/axum-rest-api-server/src/testing/environment.rs`
starts through `ApiServer::start_with_cancellation` with a fresh token per
start. `Environment<Running>` owns the server task, the drain controller, and
one `DropGuard`; `stop()` keeps its signature, joins both tasks, and panics
naming each failing task. The health-check API contract test stops the REST API
through `Environment::stop()`, and the task inventory records the migration.
Evidence is in `manual-verification-evidence.md` (V0 to V4).

## What Went Well

1. The spec's "Token shape" point (one token, one `DropGuard`, because there
   is no listener) removed the only real design question before coding.
2. Running every mutation against its own test, and reading the `test result:`
   line, found two weak spots (below) before review.

## What Changed During Implementation

1. **An extra test.** Swallowing the failures inside `stop()` was not caught by
   the planned tests, which exercise `join_owned_tasks` but not `stop()`
   itself. A mutation (G) showed it, so the test
   `it_should_panic_naming_each_failing_task_when_stopped_with_failed_tasks`
   was added, using a hand-built `Running` state. The maintainer approved it at
   the design review.
2. **A generic `join_owned_tasks`.** `RuntimeError` has a private field, so the
   server-error branch could not be unit-tested with the concrete type. The
   function takes `JoinHandle<Result<Launcher, E>>` with `E: Display`. The
   maintainer approved it at the design review.
3. **Two mutation pitfalls.** Mutating the `DropGuard` to guard an unrelated
   token (D) also breaks `stop()`, so it does not isolate the drop clause; a
   plain `CancellationToken` held with a working `stop()` (H) does. A first
   version of H did not compile, because the new panic test builds a `Running`
   state, and a non-compiling mutation runs no test.

## Root Cause

The spec listed tests for what the environment does, but not for
`stop()`'s own failure reporting, which is a separate clause of D1 ("panics
naming each failing task"). The mutation method of the `write-unit-test` skill
("one mutation per clause") surfaced the gap; the plan did not list the clauses
first.

## Improvements for Future Work

1. When a spec states a rule with several clauses, such as D1, list the clauses
   and a test for each before writing tests, instead of discovering a clause
   through a mutation. The `write-unit-test` skill already says this; the
   sibling specs (SI-24) can list the clauses in their T1 test list.
2. Write the prose-first Arrange-Act-Assert paragraphs while writing the tests.
   Here they were written afterwards, after the review found the record missing
   (V4).

## Avoiding Overcorrection

The evidence does not justify a shared test-environment abstraction across the
HTTP, UDP, and REST API environments: this one owns no listener and differs in
state, so the similarity is in the pattern (guard, disarm, join, report), not in
code worth sharing.

## Evidence

- `docs/issues/closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md`
- `manual-verification-evidence.md` V3 (mutations G, D, H) and V4.
- `agent-review-reports.md` (findings F1 to F5).
