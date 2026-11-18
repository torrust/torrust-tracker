---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2765-2758-remove-deprecated-tracker-clients/ISSUE.md
    - docs/issues/closed/2765-2758-remove-deprecated-tracker-clients/manual-verification-evidence.md
    - docs/templates/ISSUE.md
---

# Implementation Retrospective — Remove the deprecated tracker client binaries

## Purpose

Record evidence-based process improvements discovered while implementing #2765 (PR 2901).

## Outcome

The three deprecated binaries are gone, `tracker_client` is the only binary of the client crate, and
the e2e runner helper builds and runs `tracker_client check` with captured output and deadlines.
Evidence: `manual-verification-evidence.md` (V1 to V3) and the unit tests of the helper.

## What Went Well

1. The checker integration tests already ran the unified binary, so removing the binaries lost no
   coverage and the removal was a deletion plus documentation.
2. The helper's tests use plain `sh` children, so the capture, the pipe-buffer case, the kill and
   the descendant case run in milliseconds without a tracker.

## What Changed During Implementation

- Review found that the first tests did not guard the capture itself: they stayed green with
  inherited streams. The helper now returns the captured output and the tests assert it.
- Review found that the deadline also covered a cold `cargo run` build, and that a descendant
  holding a pipe could keep the drain past the deadline. The build became its own step and the
  drain got a grace period.
- Review found stale instructions in an unrelated open spec (#2594) and in migrated examples that
  kept legacy flags (`--format pretty`, `--peer-addr`).
- The first records cited a run that was a throwaway harness without recording it, and the design
  checkpoint, the prose-first comparison and the completion review were missing.

## Root Cause

- The spec named the files this issue may touch but no search that finds instructions elsewhere
  (other specs, examples in doc comments), so the sweep for stale references was done once, by
  hand, and without a recorded command.
- The spec asked for a design-review checkpoint and the test skill for a prose-first comparison,
  but nothing in the task list made them a step, so they were skipped until review.

## Improvements for Future Work

1. A spec that removes a command lists the search command that proves no instruction to run it
   remains, written so that it does not match its own text, and records its output.
2. A throwaway verification harness is recorded verbatim in the evidence when a result relies on
   it, with the exact command.
3. When a spec asks for a design-review checkpoint or the skill asks for a comparison, add each as
   a task row so it is done and recorded in the commit that introduces the code.
