---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2845-1488-si-26-consolidate-shutdown-supervisor/ISSUE.md
    - docs/issues/closed/2845-1488-si-26-consolidate-shutdown-supervisor/manual-verification-evidence.md
    - docs/pr-reviews/pr-2870-review/PR-REVIEW.md
---

# Implementation Retrospective — Consolidate the Shutdown Supervisors (SI-26, #2845)

## Purpose

Record evidence-based process improvements discovered while implementing #2845 (SI-26 of
EPIC #1488). This is a blameless review of the implementation approach.

## Outcome

PR #2870 merged on 2026-11-16. One shared supervisor (`src/bootstrap/jobs/token_aware_server.rs`)
serves the HTTP tracker, REST API and health-check API with the health-check join order;
`udp_ban_cleanup` is a reserved component that stops before the health check; the legacy
registration path and the manager's root token are gone. Manual scenarios M1 to M4 are recorded in
`manual-verification-evidence.md`: M1 is a search of the tree, and M2 to M4 ran on the binary.

## What Went Well

1. Recording the owner-type decision (T1) with the maintainer before coding kept the change small:
   `OwnedReceiveLoop` stayed package-local and no shared crate was needed.
2. Reviewers checked the tests by restoring the old behaviour. That is how the weak regression test
   was found and replaced by one that fails on regression.

## What Changed During Implementation

- The first regression test for "the controller is joined after a server-join error" only proved the
  controller reached its send, so it passed with the old early return. The replacement gates the
  controller on a paused clock and asserts the supervisor stays pending
  (`it_should_wait_for_the_drain_controller_when_the_server_join_fails`). The spec's design review
  after each of T2, T3 and T4 (a recorded prose-first Arrange-Act-Assert review of the tests) was not
  done; it is the step that would most likely have caught the weak test before review.
- M4 assumed a "short" ban reset interval, but the configuration rejects anything below 3600 s, so
  one run takes over an hour. It was first left TODO and run before the close-out.
- Two scenarios were first marked DONE or BLOCKED without a binary run, and the first M2 evidence was
  a summary, not the commands and output. Review sent them back.
- Removing the legacy path left stale text in places `git grep` for the symbol names does not reach:
  a broken doc sentence, the task-inventory tree and formula, and a "root token" mention.
- The PR description used `Closes #2845` inside a negated sentence, which still closes the issue.

## Root Cause

- The spec described M4's steps without checking them against the configuration validation.
- The T5 check (`git grep` for the removed symbols) catches names, not descriptions of the removed
  behaviour.
- Evidence was summarized from memory of a run instead of being produced by a recorded script.

## Improvements for Future Work

1. When a manual scenario depends on a configuration value, check the value against the
   configuration's validation while writing the spec, and state the run time.
2. When removing a mechanism, search the docs for the concepts it introduced (here "root token",
   "compatibility registry"), not only for its symbol names.
3. Produce manual evidence with a script whose text and raw output both go into the evidence file.

## Avoiding Overcorrection

No new rule for regression tests: the existing review habit of restoring the old code caught the weak
test. No change to the closing-keyword guidance either: the `open-pull-request` skill already covers
negated mentions.

## Evidence

- Spec: `docs/issues/closed/2845-1488-si-26-consolidate-shutdown-supervisor/ISSUE.md`
- Evidence: `manual-verification-evidence.md` in this folder
- Review audit: `docs/pr-reviews/pr-2870-review/PR-REVIEW.md`
- PR: https://github.com/torrust/torrust-tracker/pull/2870
