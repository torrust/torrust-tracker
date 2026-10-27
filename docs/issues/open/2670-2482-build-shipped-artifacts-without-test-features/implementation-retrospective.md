---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md
    - docs/issues/open/2670-2482-build-shipped-artifacts-without-test-features/manual-verification-evidence.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
---

# Implementation Retrospective - Build the shipped container binary without test features

## Purpose

Record evidence-based process improvements discovered while implementing issue #2670 (subissue 9
of #2482). This is a blameless review of the implementation approach; it does not replace the
acceptance-criteria verification.

## Outcome

The `Containerfile` builds `torrust-tracker` and `http_health_check` with `cargo build` invocations
of their own, without `--tests` or `--all-features`, and ships those binaries; the nextest archive
still builds and runs the tests. The native-tracker tests run against the shipped binary through
`TORRUST_TRACKER_BINARY`. A new Rust tool, `shipped-features-check`, run by the `Shipped Features`
job of `testing.yaml`, fails when a workspace `testing` feature is enabled in the shipped package's
tree. Evidence: [`manual-verification-evidence.md`](manual-verification-evidence.md).

## What Went Well

1. The spec asked for a scratch-workspace red case because no real crate has a `testing` feature
   yet, so the check was proved able to fail before it had anything real to fail on.
2. Every test was run red before green, and the negative tests that passed against the first stub
   were checked with two mutations, so they are known to fail when the code is wrong.
3. The measured baseline (1531 tests, their list, and the build times) made the later claims
   checkable: 0 baseline tests missing, and the image binaries byte-identical to the build stage.

## What Changed During Implementation

1. **`cargo metadata` could not be the check's input.** The maintainer chose "a Rust check over
   `cargo metadata`" before any experiment existed. A scratch workspace showed that `cargo metadata`
   unifies features across the workspace and counts a dev-dependency's request, so it reports
   `testing` where `cargo build -p` does not enable it. The design moved to `cargo tree
   -e normal,build` and the maintainer confirmed the change at spec approval. The experiment cost
   a few minutes and avoided a check that would fail correct builds.
2. **The check could pass silently.** The independent pre-PR review found that if `cargo tree` ever
   printed paths differently, no line would parse as a workspace package and the check would print
   `checked` and exit 0. That is the worst failure for a guard, and no test of mine covered it
   because every test used output the parser already understood. The check now fails unless the
   shipped package itself is listed.
3. **The same review found a path that contains the field separator**, and an empty
   `TORRUST_TRACKER_BINARY` that spawned an empty path.
4. **Process losses.** A `/tmp` cleanup removed the first baseline build's worktree and logs
   mid-build, so the baseline was redone; an interrupted warm build made its rerun a full cache
   hit (2 s), which would have shown a meaningless "warm" time had I not compared it with the
   build's own log. A GPG timeout on one commit needed a retry with fresh stamps.

## Root Cause

1. The input of the check was chosen from the tool's name and its documentation, not from its
   behaviour on the one case that matters (a feature requested only by a dev-dependency).
2. Tests of a parser describe the output the author has seen. A guard has a second duty: to refuse
   output it has not seen, and nothing in the spec or the test plan named that duty.
3. Scratch files for a long build went under `/tmp`, which this host cleans.

## Improvements for Future Work

1. Before choosing the input of a check, run each candidate on the one input where the candidates
   disagree (here: a feature only a dev-dependency enables) and record the outputs in the spec.
2. For a check that passes when it finds nothing, add a test where the input is unreadable and
   require the check to fail, and make that a line of the spec's test plan. A pass must mean "I
   looked and found nothing", never "I could not look".
3. Keep scratch checkouts, logs and measurements for a long build under a directory the host does
   not clean (here `~/.cache/<issue>/`), and take a baseline before touching the tree.
4. When a measurement looks too good (a "warm" build in 2 s), read the build log for cache hits
   before recording it.

## Avoiding Overcorrection

A lint that compares the `Containerfile`'s shipped `cargo build` flags with the `testing.yaml` job
(review finding 6) is not justified yet: the shipped build names no feature today, two comments
name each other, and `docs/containers.md` says to keep them in step. Revisit it when a shipped
build first names a feature.

## Evidence

- [`ISSUE.md`](ISSUE.md) and [`manual-verification-evidence.md`](manual-verification-evidence.md)
- Commits `ci(container): [#2670] build the shipped binaries without test features`,
  `feat(shipped-features-check): [#2670] fail when a testing feature reaches a shipped package`,
  `fix(shipped-features-check): [#2670] fail instead of passing on tree output it cannot read` and
  `fix(tracker): [#2670] ignore an empty TORRUST_TRACKER_BINARY`
