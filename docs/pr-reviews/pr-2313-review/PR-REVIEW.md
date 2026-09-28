---
pr-number: 2313
pr-url: https://github.com/torrust/torrust-tracker/pull/2313
last-updated-utc: "2026-09-28 17:15"
---

# PR #2313 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2313.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2313-f1` | Copilot | Major (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2313-f2` | Copilot | Major (inferred) | documentation | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F3 | `review-finding:pr-2313-f3` | Copilot | Major (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2313-f4` | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F5 | `review-finding:pr-2313-f5` | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F6 | `review-finding:pr-2313-f6` | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F7 | `review-finding:pr-2313-f7` | Human | Minor | maintainability | ORIGINAL | FOLLOW_UP | OPEN |
| F8 | `review-finding:pr-2313-f8` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2313-f9` | Human | Suggestion | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F10 | `review-finding:pr-2313-f10` | Human | Nit | metadata | ORIGINAL | NO_ACTION | RESOLVED |

Rows F4-F10 come from review 5293099957, submitted after the merge and tracked by #2347. The reviewer numbered them `loop F4`-`loop F10` after this record's F1-F3, so no ID collides. F8 is fixed in follow-up PR #2363 and stays `FOLLOW_UP`/`OPEN` until it merges. F4, F5, F6, F7, and F9 are owned by #2362 (EPIC #2278 order 11). The F10 thread is resolved in the #2347 close-out.

## Finding Details

### F1 - State the skill's fixed-outdated-thread behavior in V1

- PR number: 2313
- Source review ID: 5289588303
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4081285795
- Concern: V1 named fixed outdated threads in its goal but its observed result did not explicitly say that the skill also requires `FIXED`/`RESOLVED`.
- Solution: Updated V1 to state that both the template and skill require `FIXED`/`RESOLVED` for a fixed outdated thread.
- Current-tree verification: `rg -n 'The template and skill state' docs/issues/open/2308-2278-reconcile-audit-contract-rules/manual-verification-evidence.md` and `rg -n 'If a code or documentation change fixed|outdated thread whose concern was fixed' .github/skills/dev/pr-reviews/process-pr-review/SKILL.md docs/templates/PR-REVIEW-TEMPLATE.md` found the matching evidence and normative rules.
- Resolution reference: docs(issues): clarify #2308 verification evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4081386514

### F2 - Retain the duplicate V1 evidence request

- PR number: 2313
- Source review ID: 5289588303
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4081285850
- Concern: A second inline thread independently requested that V1 explicitly confirm the skill's fixed-outdated-thread behavior.
- Solution: The F1 evidence correction also resolves this re-raised concern.
- Current-tree verification: `rg -n 'The template and skill state' docs/issues/open/2308-2278-reconcile-audit-contract-rules/manual-verification-evidence.md` found the explicit shared behavior statement.
- Resolution reference: docs(issues): clarify #2308 verification evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4081386709

### F3 - Correct the V2 recorded search pattern

- PR number: 2313
- Source review ID: 5289588303
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4081285888
- Concern: V2's recorded `rg` command had an unmatched backtick in its quoted search pattern.
- Solution: Removed the unmatched backtick so the recorded command is reproducible.
- Current-tree verification: `rg -n 'FIXED resolution\|Resolution reference' docs/issues/open/2308-2278-reconcile-audit-contract-rules/manual-verification-evidence.md` found the corrected pattern.
- Resolution reference: docs(issues): clarify #2308 verification evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4081386861

### F4 - A duplicate thread whose concern was fixed has two mandated dispositions at head

- PR number: 2313
- Source review ID: 5293099957
- Reviewer finding ID: loop F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224040>
- Concern: For a duplicate thread whose concern a change fixed, the template mandates both `FIXED`/`RESOLVED` and `NO_ACTION`/`SUPERSEDED`, with no order between them. This record's F2 picked the first while the log calls it a duplicate.
- Solution: Follow-up in #2362, which gives the skill and the template one rule for that case. This is a skill-contract change, outside #2347's documentation-fix scope.
- Current-tree verification: on `develop` at `478516cf`, `docs/templates/PR-REVIEW-TEMPLATE.md` contains both ``An outdated thread whose concern was fixed is `FIXED`/`RESOLVED` `` and ``use `NO_ACTION`/`SUPERSEDED` only for a duplicate, superseded, or no-change concern``.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121106479>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121106479>

### F5 - The template states the `Resolution reference` rule three ways at head, two of them contradicting

- PR number: 2313
- Source review ID: 5293099957
- Reviewer finding ID: loop F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224052>
- Concern: The field placeholder and the Completion Rules bullet allow a commit subject or a reply URL for any fix, while the new rule binds `FIXED` to a commit subject. The contract check pins the placeholder.
- Solution: Follow-up in #2362, which states the rule once and updates the pinned placeholder, coordinating with #2349.
- Current-tree verification: on `develop` at `478516cf`, the template has `- Resolution reference: <UNIQUE_COMMIT_SUBJECT_OR_REPLY_URL>`, the FIXED-only rule, and `- Cite a fix by its unique Conventional Commit subject or durable reply URL`. `agent-review-report-contract/src/main.rs` pins the placeholder.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121106825>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121106825>

### F6 - F76 is named as reconciled, but at head the case F76 describes has no admissible `Resolution reference` at all

- PR number: 2313
- Source review ID: 5293099957
- Reviewer finding ID: loop F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224062>
- Concern: A `FIXED` finding whose fix is not a repository change (a PR description, label, or issue edit) has no admissible `Resolution reference`.
- Solution: Follow-up in #2362, which defines the admissible value. #2347 met this case with `review-finding:pr-2290-f7`.
- Current-tree verification: on `develop` at `478516cf`, the skill says `` `FIXED` resolution references are unique Conventional Commit subjects ``, and no rule covers a non-repository fix.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121107096>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121107096>

### F7 - A policy sentence was appended to the copied-verbatim `## Status Values` section, and this PR's own record does not copy it

- PR number: 2313
- Source review ID: 5293099957
- Reviewer finding ID: loop F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224083>
- Concern: The template's copied-verbatim `## Status Values` section ends with a policy sentence, and this record's own section omits it (the F63 anti-pattern).
- Solution: Follow-up in #2362, which moves the sentence to guidance or reclassifies the mark, coordinating with #2278 order 8's byte-diff of the copied sections. This record is historical and is not rewritten.
- Current-tree verification: on `develop` at `478516cf`, the template's `## Status Values` ends with ``A post-merge `NO_ACTION` requires maintainer approval to decline the follow-up work.``. Records from `pr-2320` onward carry it; this record does not.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121107363>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121107363>

### F8 - V1's recorded command cannot produce the Observed Result V1 asserts

- PR number: 2313
- Source review ID: 5293099957
- Reviewer finding ID: loop F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224093>
- Concern: V1's `rg` pattern matches the skill only at the re-raise rule, not at its fixed-outdated-thread rule. The earlier fix changed the assertion rather than the procedure.
- Solution: `docs(issues): [#2347] make the #2308 V1 command reproduce its result` appends a correction to V1 with a command that matches both rules in both documents, and its output.
- Current-tree verification: V1's recorded command, re-run on `develop` at `478516cf`, matches the skill once, at `68:   Every re-raise has its own tracking row and detail entry; never collapse it`. The corrected pattern also matches `If a code or documentation change fixed` in the skill.
- Resolution reference: `docs(issues): [#2347] make the #2308 V1 command reproduce its result`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4124959323>

### F9 - F79's conditional was resolved toward the condition, so the single-round consolidated response F79 named remains unruled

- PR number: 2313
- Source review ID: 5293099957
- Reviewer finding ID: loop F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224101>
- Concern: Step 8, the checklist, and the template cover only a response to multiple review rounds. A single-round response covering several findings is governed by no rule.
- Solution: Follow-up in #2362, which states whether and how one response may cover several findings of a single review.
- Current-tree verification: on `develop` at `478516cf`, the skill checklist reads `Consolidated responses that cover multiple review rounds name every covered review and`, and no rule covers a single round.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121107890>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121107890>

### F10 - This file ships at head claiming a last-updated time 54 minutes before its last edit

- PR number: 2313
- Source review ID: 5293099957
- Reviewer finding ID: loop F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224113>
- Concern: The #2308 evidence file's `last-updated-utc` predated its last edit at the PR head.
- Solution: No action, approved by the maintainer; the finding is not live.
- Current-tree verification: on `develop` at `478516cf`, the stamp reads `last-updated-utc: "2026-09-23 11:00"`, set by `chore(issues): archive closed issue #2308 spec`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121108243>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4121108243>

## Processing Log

- 2026-09-23 09:52 UTC - Started audit; fetched GraphQL review threads and GitHub PR metadata. No reviews, comments, or unresolved threads were present.
- 2026-09-23 10:22 UTC - Normalized Copilot's three inline findings. Fixed F1 and F3 in `docs(issues): clarify #2308 verification evidence`; retained the duplicate F2 request as `RE_RAISE_OF:F1`; replied to all three threads with current-tree verification and the resolution reference.
- 2026-09-23 10:41 UTC - Confirmed all three threads had replies, resolved them, and refreshed GraphQL data; no unresolved actionable thread remains.
- 2026-09-28 10:29 UTC - Posted a disposition reply on each of the seven post-merge threads of review 5293099957 (`created_at` 10:29:38Z-10:29:50Z). #2347 had posted tracking replies on 2026-09-26 from 12:05 UTC.
- 2026-09-28 10:51 UTC - Added rows F4-F10 for review 5293099957, as #2347 approved. Changed the Ownership section's `Post-merge workflow approval` from `N/A` to the #2347 approval record; that is the only in-place edit to earlier content.
- 2026-09-28 17:15 UTC - #2347 T7 close-out after PR #2363 merged into `develop` (17:00:27Z). josecelano posted the T7 replies on F8 (`created_at` 17:07:55Z), then resolved that thread and the F10 `NO_ACTION` thread. This close-out records F8 as `FIXED`/`RESOLVED`, each citing its fixing commit with its T7 reply as Reply URL, and F10 as `RESOLVED`, keeping the disposition reply. F4 stays `FOLLOW_UP`/`OPEN` (#2362); F5 stays `FOLLOW_UP`/`OPEN` (#2362); F6 stays `FOLLOW_UP`/`OPEN` (#2362); F7 stays `FOLLOW_UP`/`OPEN` (#2362); F9 stays `FOLLOW_UP`/`OPEN` (#2362).

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
