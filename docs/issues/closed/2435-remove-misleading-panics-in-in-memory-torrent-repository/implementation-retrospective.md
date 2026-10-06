---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md
    - docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md
    - .github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md
    - packages/swarm-coordination-registry/src/swarm/registry.rs
---

# Implementation Retrospective — Remove Misleading Panics From the In-Memory Torrent Repository

## Purpose

Record evidence-based process improvements discovered while implementing issue #2435. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

The ten swarm registry methods that could never fail now return plain values, and the registry
`Error` type is gone. `InMemoryTorrentRepository` passes calls straight through, with no `expect`
and no `# Panics` sections. The seven read-only registry queries are `#[must_use]`. The net code
diff against `develop` is five files, mostly deletions; one of them, `torrent/services.rs`, only
loses three false `# Panics` doc sections (PR #2445 review).

The policy is recorded in
[ADR 20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md):
return `Result` only when an operation can fail today, crosses an I/O boundary, or is a trait or
port with an existing or planned backend that can fail. EPIC #1669 gained a pre-publish checklist to audit
public error enums for `#[non_exhaustive]`.

Validation: `cargo clippy --workspace --all-targets --all-features` is clean, the full stable test
suite passes (2975 tests), `linter all` passes, and an independent task review returned PASS WITH
FINDINGS. All findings are addressed or decided.

## What Went Well

1. **Small, coherent commits made the reversal cheap.** Each option C step (ADR, error type, REST
   path, propagation) was its own commit, so undoing it took two `git revert` commits instead of
   hand edits. The history keeps both the attempt and the correction.
2. **Claims were verified before they were recorded.** A two-crate scratch build confirmed how
   `#[non_exhaustive]` behaves across crates; the cited references were checked against the live
   pages; the new REST test was mutation-proven.
3. **Independent review caught a real regression.** Removing `Result` also removed the unused-value
   warning on the query methods. The task review flagged it, and `#[must_use]` restored it.
4. **The spec kept the reasoning.** The superseded T2 decision, the T7 revision, and the progress
   log show why the design changed, not just that it did.

## What Changed During Implementation

The decision changed from option C (keep `Result` with an uninhabited `#[non_exhaustive]` error and
propagate it to every delivery layer) to option B (return plain values).

- Option C was fully implemented in three commits ("replace the Infallible error alias with a
  non-exhaustive empty enum", "return a 500 when the tracker stats cannot be collected", and
  "propagate swarm registry errors instead of expecting them"): `SwarmRegistry`
  variants in `AnnounceError` and `ScrapeError`, a REST `StatsError` with a `500` path, error
  handling in the cleanup job, and 37 new `.unwrap()` calls in tests.
- The ADR was then refined after an external AI review ("sharpen the non-exhaustive error ADR
  around abstraction semantics"). The review sharpened the wording but did not question the
  premise.
- The premise broke on one concrete question from the maintainer: can counting the swarms
  (`Registry::len`) ever fail? Nobody could name a case, and the case for the counting methods that
  returned `Result` was just as weak. The maintainer reversed the decision (T7). The option C
  commits were reverted with two `revert(...)` commits, and the registry was changed to return
  plain values ("return plain values from infallible registry methods").

## Root Cause

The T2 decision was framed as "how should the code keep `Result` for future errors?" (which error
type, how far to propagate) before testing the premise that each operation could plausibly fail.
No step asked for a concrete, plausible failure scenario per method.

Three things let the premise through:

- **The general argument was persuasive in the abstract.** "Implementations change; public crates
  should be ready" is true in general, so it was accepted without being applied to a single method.
- **The cost was not put next to each option.** The propagation inventory (every production caller
  and delivery boundary) was gathered before implementation but not set against option B's cost.
  The plumbing cost only became concrete once it was on the branch.
- **The reviews checked how the decision was argued, not whether it held.** The external ADR review
  improved precision without testing the decision against a real method.

## Improvements for Future Work

1. **Before keeping `Result` for future failures, name one plausible failure per operation.** If
   none can be named (the `len()` test), the operation returns a plain value. The ADR's three
   conditions now encode this; reviewers should ask the question explicitly when a public signature
   returns `Result` without a visible failure source.
2. **Put each option's concrete cost next to it when asking for a decision.** List the signatures
   that change, the new error variants, the untestable paths, and the test churn per option, so the
   trade-off is visible before implementation rather than after.
3. **Check an abstract design argument against one concrete case before agreeing.** When a
   maintainer or reviewer offers a general rationale, try it on a specific method first; a
   one-question check would have saved the option C implementation and its reverts.

## Avoiding Overcorrection

- Do not ban `Result` on infallible implementations of ports or traits. A port with an existing or
  planned backend that can fail (for example the database driver traits) correctly returns `Result`
  even when one implementation cannot fail. Being swappable alone does not qualify (ADR condition 3).
- Do not require a prototype or a retrospective for every design decision. This one was warranted
  because the decision was fully implemented, then reversed, and produced a repository-wide ADR.
- Do not treat breaking changes as free. They are acceptable because they are semver-signalled and
  paid only when a failure becomes real, and existing public error enums still get
  `#[non_exhaustive]` before their first publish (EPIC #1669 checklist).

## Evidence

- [Issue specification](ISSUE.md): Decision (T2), Decision Revision (T7), and the progress log
- [ADR 20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md)
- The PR for issue #2435: the option C commits, the ADR refinement, the decision switch, the two
  `revert(...)` commits, the option B commit, and the `must_use` review fix, cited above by their
  Conventional Commit subjects
