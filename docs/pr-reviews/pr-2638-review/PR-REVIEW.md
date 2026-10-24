---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2638 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2638>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Round 1 is the Copilot review `5639798194` (submitted 2026-10-24 20:18 UTC). The
reviewer IDs F1 to F4 do not collide with an existing audit ID and are kept.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2638-f1 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2638-f2 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2638-f3 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2638-f4 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Use the v3 TLS configuration key

- PR number: 2638
- Source review ID: 5639798194
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2638#discussion_r4368390482
- Concern: Manual-verification scenario M5 told the reader to configure `[http_api.tsl_config]`, but the v3 `HttpApi` type defines `tls_config` and denies unknown fields, so the tracker would not start.
- Solution: M5 now uses `[http_api.tls_config]`. The check also showed that the crate documentation in `packages/axum-rest-api-server/src/lib.rs` spells the section `tsl_config` in three places, so the specification's design-review questions now record that as a documentation correction for the `lib.rs` plan. No production file was changed.
- Current-tree verification: Read `packages/configuration/src/v3_0_0/tracker_api.rs` (the `tls_config` field and `#[serde(deny_unknown_fields)]`); `grep -n tsl_config` on the specification finds only the new note about `lib.rs`.
- Resolution reference: docs(issues): [#2637] use the v3 tls_config key in the TLS manual verification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2638#discussion_r4368404758

### F2 - Define ledger ordering within each slice

- PR number: 2638
- Source review ID: 5639798194
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2638#discussion_r4368390460
- Concern: The per-file workflow said to process files in ledger order, while the approved slice table puts rows 1 and 5 in S1 and rows 2 to 4 in S2 and forbids starting S2 before S1 merges, so both could not be followed literally.
- Solution: The Per-File Workflow now states that the slice order takes precedence and that the ledger order applies within a slice; the ledger introduction points to it.
- Current-tree verification: Read the Per-File Workflow and Source File Ledger introductions in the specification after the change.
- Resolution reference: docs(issues): [#2637] make slice order take precedence over ledger order
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2638#discussion_r4368404983

### F3 - Reconcile the refactor gate with the two-gate policy

- PR number: 2638
- Source review ID: 5639798194
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2638#discussion_r4368390474
- Concern: Step 1 of the per-file steps required a maintainer review of the R1 refactor, a third approval between the plan approval and the completed-file review, which contradicts the two-gate policy stated below the table and in the subissue guidelines.
- Solution: R1 is self-reviewed by default and the maintainer reviews at step 5; a plan may add an intermediate stop only when it records a specific risk.
- Current-tree verification: Read the step 1 row and the two-gate paragraph in the specification after the change.
- Resolution reference: docs(issues): [#2637] align the refactor gate with the two-gate policy
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2638#discussion_r4368405072

### F4 - Describe capability gating instead of route removal

- PR number: 2638
- Source review ID: 5639798194
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2638#discussion_r4368390438
- Concern: The specification and the coverage evidence said the auth-key and whitelist routes exist only for some configurations, but `v1/routes.rs` always registers them and the configuration decides only whether the service is present.
- Solution: Every statement of that kind now describes capability gating: the routes are always registered and an absent service answers `409`. The wording changed in the rank 2 responsibility row, coverage objective 2, the design paragraph, the lessons bullet and the risks bullet of `ISSUE.md`, and in row 2 and prioritized gap 2 of `coverage-evidence.md`.
- Current-tree verification: Read `packages/axum-rest-api-server/src/v1/routes.rs` and the `routes.rs` of the auth-key and whitelist contexts; `grep -n 'gating\|gated' ISSUE.md` and a read of the two `coverage-evidence.md` lines after the change.
- Resolution reference: docs(issues): [#2637] describe capability gating instead of route removal
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2638#discussion_r4368404853

## Processing Log

- 2026-10-24 20:18 UTC - Copilot review `5639798194` submitted with four inline findings. No review from the maintainer da2ce7 existed at this point.
- 2026-10-24 20:23 UTC - Started audit; fixed F1, F4, F2 and F3 in one signed commit each, rebased the branch onto `torrust/develop` and pushed it with `--force-with-lease` (the commit subjects, not ids, are cited above).
- 2026-10-24 20:24 UTC - Replied to the four threads (reply times 20:24:17 to 20:24:24 UTC) and resolved them; each resolve mutation returned `isResolved: true`.
- 2026-10-24 20:53 UTC - Correction to the 20:18 UTC entry: it called da2ce7 "the maintainer", but da2ce7 is the requested reviewer (the maintainer is the PR author, josecelano), as da2ce7's finding F9 states. The 20:18 UTC entry is left as written.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
