---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2930 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2930.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2930-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2930-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2930-f3 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2930-f4 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2930-f5 | Copilot | Suggestion (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-2930-f6 | Copilot | Suggestion (inferred) | metadata | RE_RAISE_OF:F5 | NO_ACTION | SUPERSEDED |
| F7 | review-finding:pr-2930-f7 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Skill example repeats a variant name

- PR number: 2930
- Source review ID: 5952710132
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4610720594
- Concern: The skill's error example used the variant name `InvalidKeyFormat` twice in one block with different fields, which is not valid Rust.
- Solution: Split the correct and the wrong example into two blocks, each a complete `KeyError` enum with one variant. While there, the rule sentence now says "must not", like the ADR after F2.
- Current-tree verification: Read `.claude/skills/handle-errors-in-code/SKILL.md` section "No Source Locations in Errors": two `rust` blocks, each declaring `enum KeyError` once with one `InvalidKeyFormat` variant. `linter markdown` passed.
- Resolution reference: docs(skills): [#2913] make the error example of handle-errors-in-code valid Rust
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611359135

### F2 - ADR Agreement reads as the current state

- PR number: 2930
- Source review ID: 5952710132
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4610720648
- Concern: The Agreement says the workspace "does not depend on `torrust-located-error`", while the Consequences say the dependency and the location fields remain until SI-2 to SI-6 land.
- Solution: The Agreement now opens with "must not record or print", says that new and changed code follows the rules now and that EPIC #2912 brings the older code in line, and points 1 and 2 say "must not"; point 2 adds that the EPIC removes the existing dependency.
- Current-tree verification: Read the Agreement of `docs/adrs/20261118045443_errors_do_not_record_source_locations.md`: points 1 and 2 use "must not" and the opening paragraph names EPIC #2912. `linter markdown` and `linter cspell` passed.
- Resolution reference: docs(adrs): [#2913] state the no-source-location rules as rules, not as the current state
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611359588

### F3 - Same Agreement concern on the Consequences line

- PR number: 2930
- Source review ID: 5952710132
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4610720696
- Concern: The same comment as F2, anchored on the Consequences paragraph that says the code still has location fields and `LocatedError`.
- Solution: Same change as F2; the Consequences paragraph is the transitional note the reviewer suggested keeping, so it is unchanged.
- Current-tree verification: Same inspection as F2.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611360007
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611360007

### F4 - ADR index row reads as already done

- PR number: 2930
- Source review ID: 5952710132
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4610720763
- Concern: The index row said "the workspace drops `torrust-located-error`", which is not true until EPIC #2912 completes.
- Solution: The row now says that error types must not have a `location` field and that the workspace must not use `torrust-located-error`, with existing uses removed by EPIC #2912.
- Current-tree verification: `grep -n 20261118045443 docs/adrs/index.md` shows the new wording. `linter markdown` passed.
- Resolution reference: docs(adrs): [#2913] state the no-source-location rules as rules, not as the current state
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611360405

### F5 - Spec status in-progress while tasks are done

- PR number: 2930
- Source review ID: 5952710132
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4610720817
- Concern: The front matter says `status: in-progress` although T1-T3, the implementation and the acceptance criteria are checked.
- Solution: No change. The status tracks the issue, which stays open until the PR is merged and the close-out moves the specification to `docs/issues/closed/`; the checkpoints "Reviewer validated acceptance criteria" and "Issue closed" are still open. Closed specifications use `done`, and the close-out sets it.
- Current-tree verification: Read the Workflow Checkpoints of the SI-1 spec: "Reviewer validated acceptance criteria" and "Issue closed and spec moved" are unchecked. `grep -h '^status:' docs/issues/closed/*/ISSUE.md` shows `done` as the usual closed value.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611360821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611360821

### F6 - Same status concern on the task table

- PR number: 2930
- Source review ID: 5952710132
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4610720864
- Concern: The same comment as F5, anchored on the implementation-plan table.
- Solution: Same answer as F5.
- Current-tree verification: Same inspection as F5.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611361230
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611361230

### F7 - "Relevant tests pass" checked without a test

- PR number: 2930
- Source review ID: 5952710132
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4610720906
- Concern: The criterion "Relevant tests pass" is checked although no test ran, because no code changed.
- Solution: The criterion now reads "Relevant tests pass (none apply: this issue changes no code, so no test was run)".
- Current-tree verification: `grep -n 'Relevant tests pass' docs/issues/open/2913-2912-si-1-adr-errors-do-not-record-source-locations/ISSUE.md` shows the new text.
- Resolution reference: docs(issues): [#2913] say that no test applies to the SI-1 change
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2930#discussion_r4611361740

## Processing Log

- 2026-11-18 08:04 UTC - Started audit for review 5952710132 by Copilot (submitted 2026-11-18 07:58 UTC on the head commit "docs(skills): [#2913] replace located-error guidance with the no-source-location rule"). Seven inline comments, recorded as F1-F7. The review body also warns that Copilot could not run its full agentic review because no runner started before the timeout; that is not a finding about the change.
- 2026-11-18 08:04 UTC - (Exact time not recorded; after the audit start.) The first push of the fixes for F1, F2, F4 and F7 (three commits) was refused by the pre-push hook: `cargo +nightly check` failed in `torrust-tracker-udp-protocol` with `uncommon_codepoints` errors from `zerocopy-derive` on nightly 2026-11-18, unrelated to this PR. The maintainer chose (through the EPIC coordinator) to wait for the fix on `develop` instead of bypassing the hook.
- 2026-11-18 08:57 UTC - The fix (#2932, issue #2931) was on `develop` (seen by a fetch at 08:57:29Z); rebased the branch onto it and pushed the three fix commits with `--force-with-lease`.
- 2026-11-18 09:00 UTC - Replied to F1 to F7 (replies created from 09:00:40Z to 09:00:55Z). Threads not yet resolved.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
