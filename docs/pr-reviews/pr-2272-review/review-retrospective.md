---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - "issue #2179"
    - docs/pr-reviews/pr-2272-review/PR-REVIEW.md
    - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2272 (issue #2179)

## Purpose

Record a blameless review-process retrospective for
<https://github.com/torrust/torrust-tracker/pull/2272>. The code change was a narrow, four-line CI
fix; the review process around it was disproportionately costly. This captures why, and what to
change, without relitigating individual findings already dispositioned in
[PR-REVIEW.md](PR-REVIEW.md).

## Outcome

The delivered fix (package-qualified `cargo run` invocations in `.github/workflows/testing.yaml`)
was correct from round 1 and never changed. Across four review rounds the PR accumulated 9 review
threads (2 Copilot, 7 Cameron) and required five follow-up commits, most of them touching the audit
record (`PR-REVIEW.md`) and the issue specification rather than the workflow fix itself.

## What Went Well

1. The fix itself was validated early and never disputed: package-qualified `cargo run` was
   confirmed correct in round 1 and stayed correct through round 4.
2. The audit trail is complete and traceable: every finding has a source review ID, a reply URL,
   and a resolution reference, so the full history is reconstructable after the fact.
3. GraphQL-based thread verification caught real gaps (e.g., a genuinely unresolved thread) rather
   than relying on assumptions about resolution state.

## What Changed During Implementation

- Review scope expanded from "is the fix correct" to "does the accompanying documentation exactly
  match template boilerplate, timestamps, and cross-referencing conventions." Findings F1-F3, F6,
  F7 were about the issue spec and the audit record's own prose/format, not the fix.
- The audit record became a second reviewed artifact in its own right: round 3 (F6-F7) reviewed the
  round 2 audit entries for timestamp accuracy and template-byte-parity, and round 4 (F9) reviewed
  round 3's audit entries for completeness. This produced a visible recursive loop: fix → audit the
  fix → audit the audit.
- Each finding, regardless of severity (`Nit`, `Suggestion`, `Minor`), went through the same full
  cycle: commit, push, reply (naming review ID + finding ID + resolution reference), resolve,
  re-verify via GraphQL. There was no lighter-weight path for non-blocking findings.
- Precise processing-log timestamps (`HH:MM UTC`) were required to be internally consistent with
  commit/push/reply times to the minute, which itself became a finding (F6) when an estimated stamp
  preceded the events it described.

## Root Cause

- The process has no size/risk-based scaling: a 4-line, low-risk CI fix is held to the same audit
  rigor (spec, evidence file, independent review report, canonical PR review audit, byte-for-byte
  template parity) as a large behavioral change. The overhead is fixed, not proportional to the
  change.
- The canonical audit record (`PR-REVIEW.md`) is itself a tracked, reviewable Markdown artifact with
  no automated structural check. Template drift (dropped rules, extra vocabulary values) can only be
  caught by a human re-reading it against `PR-REVIEW-TEMPLATE.md`, which is what produced F7 and,
  recursively, could produce further findings against the fix for F7. Processing-log timestamp
  precision was asserted by hand, not derived from `git log`/GraphQL data, which is what produced F6.

## Improvements for Future Work

1. Add a lightweight structural check (script or lint) that diffs `PR-REVIEW.md`'s `Status Values`
   and `Completion Rules` sections against `PR-REVIEW-TEMPLATE.md` byte-for-byte, so template drift
   is caught before a human review round, not during one.
2. Drop manual `HH:MM` precision requirements in the Processing Log for entries that only need
   relative ordering; record commit/push times by reference (`git log`/GraphQL timestamp) instead of
   hand-typed estimates, removing an entire class of findings (F6-style).
3. For findings marked `Suggestion` or `Nit` on process-only artifacts (not the product change),
   allow batching multiple such findings into one reply/resolve pass instead of one full cycle per
   finding, when they land in the same review.
4. Consider a documented lighter-weight audit path for small, low-risk fixes (e.g., a handful of
   workflow/config lines) so the audit-record overhead scales with change size and risk, similar to
   how `create-issue` already makes the spec-only PR step optional for narrow work.

## Avoiding Overcorrection

- Do not remove the canonical PR review audit requirement itself: it produced a genuinely useful,
  traceable record, and the one real gap it caught (F9, missing F6-F8 rows) was a legitimate
  completeness problem, not process noise.
- Do not relax the rule that every finding gets a reply naming its review ID, finding ID, and
  resolution reference — that rule is what made this retrospective possible to write accurately.
- Do not conclude review depth itself was the problem; every finding in [PR-REVIEW.md](PR-REVIEW.md)
  was substantively correct. The cost came from applying full-weight process to a low-risk change,
  not from the review being wrong.

## Evidence

- Issue [docs/issues/open/2179-fix-docker-e2e-package-flag/ISSUE.md](../../issues/open/2179-fix-docker-e2e-package-flag/ISSUE.md)
- Canonical audit: [PR-REVIEW.md](PR-REVIEW.md) (DOC-1, DOC-2, F1-F9)
- PR: <https://github.com/torrust/torrust-tracker/pull/2272>
