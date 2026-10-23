---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2618-2482-decide-test-support-rules/ISSUE.md
    - docs/issues/closed/2618-2482-decide-test-support-rules/manual-verification-evidence.md
    - docs/issues/closed/2618-2482-decide-test-support-rules/agent-review-reports.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
---

# Implementation Retrospective - Decide how packages share test support code

> Create this concrete artifact only as lowercase
> `implementation-retrospective.md` inside a folder-style issue specification.

## Purpose

Record evidence-based process improvements discovered while implementing issue #2618 (subissue 1
of #2482). This is a blameless review of the approach; it does not replace the acceptance-criteria
verification.

## Outcome

An ADR with one rule per pattern, DEC-18 amending DEC-13, a verified Cargo evidence file (V1 to
V4c plus a packaging baseline), and eight migration drafts (rows 2 to 9) in the fixed sequence 1,
2, 8, 9, 3, 4, 5, 6, 7 (rows 4 to 7 also wait for their #1488 gates). The maintainer chose O1 with a path-only self
dev-dependency for the four server environments and the shared mock, O4 and O3 for `test-helpers`,
O5 for crate-private helpers and O3 for per-package folders. The change set is documentation only.

## What Went Well

1. Verifying the Cargo behaviour in scratch workspaces before writing the ADR. The results settled
   which O1 mechanism is viable, and showed two traps the EPIC did not list: `cargo test
   --workspace` hides a missing per-package feature, and `--all-targets` turns dev-dependency
   features on for binaries.
2. Putting evidence and a recommendation before the maintainer before writing the decision. He
   approved it in one round, and later redirected the container question with a clearer principle
   than the recommendation had.
3. An independent pre-PR review of the finished documentation. It found a real gap (the container
   build) that the author's own checks did not.

## What Changed During Implementation

1. **The inventory was wrong about pattern D.** It said only `tracker-core`'s own source uses its
   mocks. `MockTorrentMetricsStore` is imported by four files in `http-core` and `udp-core`, so the
   ADR needs rule D and row 3 gains two consumers. Found by grepping the mock type names across the
   workspace while preparing the ADR; corrected in the inventory in the commit "docs(issues):
   [#2618] correct the inventory: http-core and udp-core use a tracker-core mock".
2. **The container image ships the test-built binary.** `Containerfile` builds the release binary
   with `cargo nextest archive --tests --all-features --release`. With the ADR's rules the image
   would carry every `testing` feature. Found by the first Task Reviewer entry in
   `agent-review-reports.md`; the maintainer decided on 2026-10-22 to build shipped artifacts on
   their own with explicit features and a CI check, which added row 9 and rule A.5.
3. **Cargo 1.88.0 cannot run the workspace packaging check**, which the EPIC did not know; stable
   can, and CI and pre-push use stable.

## Root Cause

1. The inventory's method listed consumers of the modules it had classified (patterns A to C) but
   not of the generated types of pattern D, and the "only the crate's own source" statement was
   written without a workspace-wide search for those types.
2. The EPIC reasoned about "production builds" as `cargo build` of a package. It did not list the
   commands that produce shipped artifacts, so the container's combined test-and-release build was
   outside its model until a reviewer read the `Containerfile`.

## Improvements for Future Work

1. When an inventory says a shared item is used only by its own crate, back the claim with a
   workspace-wide search for the item's exported names, and record the command.
2. A spec or ADR that makes a claim about "production builds" should list the commands that produce
   each shipped artifact (binary, container image, published crate) and check each against the
   claim.
3. Run the Task Reviewer on decision documents before the PR, as done here, because the author's own
   consistency checks missed the shipped-artifact gap and several stale counts after a plan change.
4. After a plan change that adds a row or a count, search the spec, EPIC, inventory and ADR for the
   old count (`grep` for "rows 2 to 8", "8 subissues") in the same commit.

## Avoiding Overcorrection

No new template section or skill is justified by one occurrence. The improvements above are
practices for the author and the reviewer, not new gates; issue #2503 already plans an author
self-audit gate for statements in records.

## Evidence

- [Issue specification](ISSUE.md) and its progress log
- [Manual verification evidence](manual-verification-evidence.md), V1 to V4c
- [Agent review reports](agent-review-reports.md)
- The ADR and DEC-18
