---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/1348-1347-add-tests-udp-core/ISSUE.md
    - docs/issues/open/1348-1347-add-tests-udp-core/lessons.md
    - docs/issues/open/1348-1347-add-tests-udp-core/mutation-evidence.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

# Implementation Retrospective — Issue #1348, Add Tests to the `udp-core` Package

## Purpose

Record evidence-based process improvements discovered while implementing Issue #1348 (EPIC #1347).
This is a blameless review; it does not replace the acceptance review in [ISSUE.md](ISSUE.md).

## Outcome

Every `udp-core` source file went through its own reviewed file test plan, with one signed commit
per file. Unit-only line coverage rose from 72.49% to 98.04%, part of it from #2458, #2505, #2517,
and #2521.
Mutation proofs are recorded for every test-producing plan, a bounded `cargo-mutants` sample on
the connection cookie left no survivors, and M1-M4 passed against a running tracker. The only
production changes are documentation fixes in the files under test.

## What Went Well

1. **One file, one plan, one approval, one commit.** Each file's decisions, findings, and
   evidence stayed reviewable, and pausing for other issues (#2458, #2505) cost no rework.
2. **"What does this module decide?" before writing tests.** It exposed tests that pinned the
   wrong thing: the statistics service test passed with both inputs ignored, the closed-receiver
   listener test asserted a counter instead of the outcome, and nothing pinned the exported metric
   name or the container's configuration wiring. Each was proven with a hand-written mutation.
3. **Handing findings off instead of fixing them in place.** Production bugs and API clean-ups
   became separate issues or drafts with hand-off notes (#2458, #2505, two EPIC #1669 drafts), so
   #1348 stayed a test-only change.

## What Changed During Implementation

- **Two issues interrupted the per-file order.** The `crypto/keys.rs` review found production
  could encrypt cookies with the public test key (#2458, which removed two files from the ledger).
  The banning review found the banned-IPs metric counted non-banned IPs (#2505). `container.rs`
  waited for #2505 and the branch was rebased onto both.
- **`cargo-mutants` misreported the most important mutants.** With the `unused` lint denied in
  `[workspace.lints.rust]`, 7 of 8 "unviable" cookie mutants were real behavior changes (for
  example, `check` accepting any ID); the recorded run uses `RUSTFLAGS="--cap-lints=warn"`
  ([mutation-evidence.md](mutation-evidence.md)).
- **Findings outgrew the issue.** Several findings turned out to be workspace-wide, such as the
  unused SI-30 traits and the public `stats` lock in five packages. They became EPIC #1669 drafts,
  were implemented by other agents as #2521 and #2517, and merged before this PR; the final rebase
  had to drop three #1348 tests whose code those issues removed (V1 and V2 with
  `statistics/services.rs`, and S4 with its trait method), and one duplicate test.
- **The manual-verification client was needed a second time** (after #2505's shell probe); the
  maintainer chose a Rust client and the reusable form is a follow-up.
- **The spec carried two garbled lines** from its merge; repaired during T8.

## Root Cause

- The T1 inventory judged files by coverage and ownership, which cannot show that a covered
  mechanism protects the wrong value (the test-key bug) or that a metric's meaning differs from
  its name (the banned-IPs bug). Those surfaced only in the per-file "what does it decide?" review.
- The T6 guidance said "bounded sample" but not how to read unviable mutants; the workspace lint
  policy interacts with mutation tools in a way the spec did not anticipate.
- Spec-only review checked content, not rendering of every line, so duplicated fragments passed.

## Improvements for Future Work

1. **Mutation samples:** in the #1347 subissue guidelines (or the testing skill), require
   `RUSTFLAGS="--cap-lints=warn"` for `cargo-mutants` and a stated reason for every unviable
   mutant. Recheck #2283's two unviable mutants under this rule.
2. **Reusable verification client:** add `--connection-id`, `--bind`, and `--count` options to
   `tracker_client` (follow-up in [lessons.md](lessons.md)), so cookie and ban checks need no
   disposable code.
3. **Hand-off pattern:** the `.tmp/<topic>/ISSUE.md` + `HANDOFF.md` pair worked for four hand-offs
   in one issue; consider documenting it in the `create-issue` skill.

## Avoiding Overcorrection

- Do not make the T1 inventory deeper to catch semantic bugs early; the per-file review is the
  right place, and it worked.
- Do not require `rstest` or property tests by default; they were kept only where they improved
  readability or caught a mutation the examples missed.
- Do not promote every disposable verification script to a maintained tool; this one qualifies
  only because it was needed twice.

## Evidence

- [ISSUE.md](ISSUE.md): ledger, progress log, acceptance review.
- [coverage-evidence.md](coverage-evidence.md), [mutation-evidence.md](mutation-evidence.md),
  [manual-verification-evidence.md](manual-verification-evidence.md).
- [test-refactor-plans/](test-refactor-plans/README.md): one plan per file with mutation proofs.
- [lessons.md](lessons.md): follow-up candidates and testing lessons.
