---
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2808-workspace-package-release-process/ISSUE.md
    - docs/release-process/workspace-packages.md
---

# Implementation Retrospective — Workspace-package release process (#2808)

## Purpose

Record evidence-based process improvements discovered while implementing issue #2808. This is a
blameless review of the implementation approach.

## Outcome

The release document is split into an index, `tracker.md` and `workspace-packages.md`; the package
process, the standalone-crate variant, the `release-workspace-package` skill and the
`release-new-version` fixes are in PR 2825, with manual verification in
`manual-verification-evidence.md`.

## What Went Well

1. Keeping the split a pure move in its own commit let the reviewer check it byte for byte.
2. Dry runs of a real workspace package and of `torrust-server-lib` showed the documented path works.

## What Changed During Implementation

The review found that step 5 of the package process, kept from the old document, updated only the
root `Cargo.toml` requirement after publishing, while 14 manifests declare a versioned path
requirement on the same crate (audit F1 of PR 2825). The step was replaced by a precondition that
raises the requirement in every dependent manifest in the version-bump PR. The review also found
that removing the skill's "all `Cargo.toml` files" wording dropped the `e2e-tools` requirement on
`torrust-tracker` (F2).

## Root Cause

The spec treated the old package section as correct apart from the defects it listed, and the
implementation moved its steps without checking each one against the manifests. The spec and the
manual verification checked that the steps could be followed, not that each step is true for the
current tree.

## Improvements for Future Work

1. When a process document is rewritten from an existing one, check every retained step against the
   tree (here: `grep` the manifests for the requirement the step edits) before marking the task done.
2. When removing a broad instruction ("edit all `Cargo.toml` files"), search for the cases it
   covered and keep or name them.

## Avoiding Overcorrection

The evidence does not justify a new automated check for dependency requirements: the process document
now states the rule, and Cargo itself rejects an incompatible bump at build time.

## Evidence

- Issue specification: `docs/issues/open/2808-workspace-package-release-process/ISSUE.md`
- Review audit: `docs/pr-reviews/pr-2825-review/PR-REVIEW.md` (F1, F2)
