---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - "issue #2230"
    - review-finding:pr-2270-f9
    - review-finding:pr-2270-f10
    - review-finding:pr-2270-f11
    - review-finding:pr-2270-f12
    - review-finding:pr-2270-f13
    - review-finding:pr-2270-f14
    - review-finding:pr-2270-f16
    - review-finding:pr-2270-f18
    - review-finding:pr-2270-f19
---

# Implementation Retrospective — Add a `fix-bug` Skill and Bug-Spec Guardrails

## Purpose

Record evidence-based process improvements discovered while implementing issue #2230. This is a
blameless review of the implementation approach and does not replace acceptance-criteria
verification.

## Outcome

The implementation established the repository-owned `fix-bug` workflow, semantic bug detection,
bug-spec guardrails, and Implementer integration. Review-driven corrections made semantic-link
frontmatter machine-readable, restored lychee-checked cross-skill links, tightened regression red
proof, preserved workflow ordering, and established the canonical PR-review audit.

Final validation includes YAML parsing of skill and agent frontmatter, `linter all`, issue-local
manual verification against the finished artifacts, independent Task Reviewer assessment, and the
PR #2270 audit at `docs/pr-reviews/pr-2270-review/PR-REVIEW.md`.

## What Went Well

1. Small signed commits kept each independent review correction attributable and independently
   validated.
2. The repository's review workflow exposed missing evidence and required the issue state to be
   reopened instead of allowing stale completion claims to survive.
3. The issue-local sample with deliberately incorrect metadata provided a stable way to verify the
   semantic bug trigger before and after agent changes.

## What Changed During Implementation

The initial implementation assumed `linter all` validated Agent Skill and agent frontmatter as YAML.
Review finding F9 disproved that assumption: malformed `write-unit-test` frontmatter passed the full
lint gate. F16 showed that Implementer's pre-existing unquoted description also prevented its new
marker from being parsed.

F12 showed that the newly merged semantic-link convention's unquoted `issue #NNNN` YAML form drops
the issue number because `#` starts a comment. The convention and this PR's new values were corrected
together.

F13 and F14 showed that thin integration references can still weaken canonical behavior. The first
version allowed a broader red-test escape hatch than `write-unit-test` and did not make Implementer
preserve the required sequence or final like-for-like recheck.

F10 and F11 showed that implementation evidence is temporal. Resolving review threads without
replies or a canonical audit removed traceability after rebases, while editing an earlier independent
review entry made it appear to cover a tree it had never assessed.

## Root Cause

The implementation relied on text presence, Markdown linting, and manual link checks where the
contract was structured YAML and ordered workflow behavior. The first review-response pass also
followed a generic thread-resolution flow instead of the repository-owned `process-pr-review`
workflow, so it skipped the required audit, replies, and append-only evidence model.

The planning assumption that no dedicated validator was needed was too weak: existing linters verify
Markdown/YAML files generally but do not parse YAML frontmatter embedded in Markdown artifacts.

## Improvements for Future Work

1. Parse every changed Agent Skill and agent frontmatter as YAML during focused validation; assert
   required metadata and semantic-link values, not only text presence.
2. Quote semantic-link YAML values containing `#`; the canonical convention now states this rule.
3. Keep cross-skill references as valid Markdown links when lychee can enforce their targets.
4. Load `process-pr-review` before acting on any PR feedback, create the audit before fixes, reply
   before resolution, and commit audit updates separately.
5. Treat independent review reports as append-only snapshots. After any material follow-up commit,
   append a fresh review rather than editing or relying on the earlier verdict.
6. Rerun manual workflow scenarios after agent instructions change; an earlier interaction does not
   prove the finished artifact's behavior.

## Avoiding Overcorrection

This evidence does not justify adding a broad new parser framework or making every documentation
relationship bidirectional. A focused frontmatter parser check for changed skills/agents and the
existing high-signal semantic-link policy are sufficient. It also does not justify duplicating the
full bug workflow in Implementer or templates; those artifacts should preserve ordering and evidence
requirements while `fix-bug` remains canonical.

## Evidence

- Issue specification: `docs/issues/open/2230-add-fix-bug-skill-and-bug-spec-guardrails/ISSUE.md`
- PR review audit: `docs/pr-reviews/pr-2270-review/PR-REVIEW.md`
- Manual verification: `manual-verification-evidence.md`, especially V3
- Pull request: https://github.com/torrust/torrust-tracker/pull/2270
- Review findings: `review-finding:pr-2270-f9` through `review-finding:pr-2270-f19`
