---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2875 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2875.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

F1, F2 and F3 are in da2ce7's review 5917183183, on the PR's first head commit, `docs(issues): [#2873] add issue specification for shortening the skill descriptions`, in that source order; the IDs are his own. It was submitted as `APPROVED`; GitHub dismissed it at 08:49:39 UTC when the fix commit was pushed, so its state is now `DISMISSED`.

F4, F5 and F6 are the three inline comments of Copilot's review 5917272531 (`COMMENTED`), submitted at 08:52:33 UTC on the same first head, in that source order. They carry no finding IDs, so they take the next free audit-local IDs. Their severity is inferred from the review overview, which marks each "Low severity". F4 and F5 ask for the changes F2 and F1 had already made on the newer head.

F7 is the one finding of da2ce7's review 5917548188 on the head commit `docs(pr-reviews): [#2873] record the first reviews of PR #2875`. It was submitted as `APPROVED`; GitHub dismissed it at 09:44:41 UTC when the fixes were pushed, so its state is now `DISMISSED`. His ID F5 collides with F5 above, so it is numbered F7, the next free audit-local ID, and his ID is kept as the reviewer finding ID.

F8 and F9 are in Copilot's review 5917643855 (`COMMENTED`) on the same head, in that source order. Its IDs F7 and F8 collide with F7 above and with each other's renumbering, so they are numbered F8 and F9 and Copilot's IDs are kept as the reviewer finding IDs. F8 is an inline thread; F9 is in the review body under "previously missed issue", with no thread, so its source URL is the review and its reply is a PR conversation comment. The fix commit bodies and the replies of this round cite the reviewers' IDs (`review-finding:pr-2875-f5`, `-f7`, `-f8`); they mean this record's F7, F8 and F9.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2875-f1 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2875-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2875-f3 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2875-f4 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F5 | review-finding:pr-2875-f5 | Copilot | Minor (inferred) | testing | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-2875-f6 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2875-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2875-f8 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2875-f9 | Copilot | Minor | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - The plan fixes the defect before its regression test exists

- PR number: 2875
- Source review ID: 5917183183
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583586667
- Concern: T2 quoted `write-markdown-docs` before T3 added the rule, so T3's red run could only show over-long descriptions, never the cut at a comment, contrary to the `fix-bug` sequence and to the spec's own Bug-Fix Process.
- Solution: The rule and its unit tests are now T2, whose red run on the tree before the fix must fail on `write-markdown-docs` as cut at a comment; the quoting fix is T3, with the green run and the recheck of V0. The Bug-Fix Process, Regression Test Strategy, Commit Points and test-design paragraph use the new numbers.
- Current-tree verification: The T2 and T3 rows of the Implementation Plan and Commit Points, and the Bug-Fix Process bullets, in `docs/issues/open/2873-2003-shorten-skill-descriptions/ISSUE.md`.
- Resolution reference: docs(issues): [#2873] put the regression test before the fix and record the reproduction
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583690745

### F2 - The reproduction is not recorded or classified before review

- PR number: 2875
- Source review ID: 5917183183
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583586675
- Concern: The issue folder held only `ISSUE.md`, so the 123-of-578 and ~50-token figures had no evidence file and no classification, as `fix-bug` asks before review.
- Solution: `manual-verification-evidence.md` section V0 records the PyYAML command and its output and the two `/context` rows that bear on the defect, with the tool versions, classified Reproduced. The Bug-Fix Process points to it.
- Current-tree verification: Section V0 of `docs/issues/open/2873-2003-shorten-skill-descriptions/manual-verification-evidence.md`; its command, rerun at 08:45:16 UTC, printed `raw 578` and `parsed 123`.
- Resolution reference: docs(issues): [#2873] put the regression test before the fix and record the reproduction
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583691092

### F3 - The governance ADR's review cadence already settles the review question

- PR number: 2875
- Source review ID: 5917183183
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583586686
- Concern: The spec made a governance ADR record conditional, but the ADR's Review cadence asks for a review whenever a tracked skill is materially changed, recorded in its `Review records` table.
- Solution: Architectural Decisions says rewriting all 53 descriptions is a material change and T6 adds a dated row to that table; the T6 row lists it.
- Current-tree verification: The Architectural Decisions bullets and the T6 row of the spec; `Review cadence` and `Review records` headings in `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
- Resolution reference: docs(issues): [#2873] put the regression test before the fix and record the reproduction
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583691443

### F4 - Add the required reproduction evidence to the issue folder

- PR number: 2875
- Source review ID: 5917272531
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583653481
- Concern: The same as F2, raised on the first head.
- Solution: No change; F2's fix already records and classifies the reproduction.
- Current-tree verification: Section V0 of `manual-verification-evidence.md`, as for F2.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583691803
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583691803

### F5 - Capture truncation regression evidence before the fix

- PR number: 2875
- Source review ID: 5917272531
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583653528
- Concern: The same as F1, raised on the first head.
- Solution: No change; F1's fix already puts the rule and its red run before the quoting fix.
- Current-tree verification: The T2 and T3 rows of the spec, as for F1.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583692105
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583692105

### F6 - Keep the new check opt-in until the descriptions comply

- PR number: 2875
- Source review ID: 5917272531
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583653560
- Concern: The pre-commit hook already runs `frontmatter-validator --staged`, so postponing "wiring into the hook" does not keep a new rule from rejecting T4's per-area commits while other descriptions are still long.
- Solution: T2 adds the rule as opt-in, its red run uses the opt-in, and T4's last commit turns it on in the existing staged and whole-tree gates; the T2 commit point says no commit fails on it before then.
- Current-tree verification: `contrib/dev-tools/git/hooks/pre-commit.sh` lists the step `cargo run --quiet --package frontmatter-validator --bin frontmatter-validator -- --staged`; the T2 and T4 rows and the T2 commit point of the spec.
- Resolution reference: docs(issues): [#2873] keep the description rule opt-in until the descriptions comply
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583692368

### F7 - The evidence file departs from its template's headings

- PR number: 2875
- Source review ID: 5917548188
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583860148
- Concern: The evidence file used `Observed Output` and `Result` for the template's `Observed Result` and `Conclusion`, lacked the template's `Failures and Follow-up` section, and the spec no longer said the file was created from the template.
- Solution: The headings and the section are the template's, and the spec's Manual Verification notes say the file was created from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` and keeps its headings.
- Current-tree verification: The headings of `docs/issues/open/2873-2003-shorten-skill-descriptions/manual-verification-evidence.md` match the template's V1 section and end with `## Failures and Follow-up`; the Notes under Manual Verification Scenarios in the spec.
- Resolution reference: docs(issues): [#2873] restore the evidence template headings
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4584096743

### F8 - The `/context` row is inference, not an observation of the cut text

- PR number: 2875
- Source review ID: 5917643855
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4583932685
- Concern: `/context` shows a token count, not the description text, so it does not show that Claude Code read the cut value; V0, T3 and M2 should say so.
- Solution: V0 is classified Reproduced for the parser, and the `/context` row is recorded as corroborating inference in V0's classification and Conclusion and in the spec's Background, Bug-Fix Process, T3, AC3 and M2.
- Current-tree verification: V0 in the evidence file; the named sections of the spec.
- Resolution reference: docs(issues): [#2873] treat the /context row as corroborating evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2875#discussion_r4584097064

### F9 - A raw-versus-parsed length check would reject valid descriptions

- PR number: 2875
- Source review ID: 5917643855
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2875#pullrequestreview-5917643855
- Concern: Quoting, escape decoding and block-scalar folding change a valid description's length, so comparing raw and parsed lengths would reject T3's quoting fix instead of detecting a comment.
- Solution: The Regression Test Strategy drops the length comparison: the rule fails only on a plain scalar containing a space followed by `#`, and its tests include passing quoted and block-scalar descriptions; AC4 asks for them.
- Current-tree verification: The last paragraph of Regression Test Strategy and AC4 in the spec.
- Resolution reference: docs(issues): [#2873] detect comment truncation without a length comparison
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2875#issuecomment-6913012946

## Processing Log

- 2026-11-16 08:44 UTC - Review 5917183183 (da2ce7, `APPROVED`) submitted on the first head: F1, F2 and F3, none blocking.
- 2026-11-16 08:49 UTC - Pushed the fix commit for F1, F2 and F3 (committed 08:46:59 UTC); the push dismissed review 5917183183 (timeline event `review_dismissed`, 08:49:39 UTC). The three fixes share one commit: splitting it after the push needed a force-push, which was not done.
- 2026-11-16 08:52 UTC - Review 5917272531 (Copilot, `COMMENTED`) submitted on the first head: F4, F5 and F6.
- 2026-11-16 08:56 UTC - Pushed the F6 fix commit (committed 08:53:23 UTC; push finished 08:56:39 UTC). Replied to the six threads (08:56:53 to 08:57:05 UTC) before this record existed; it was started late, after the replies.
- 2026-11-16 08:58 UTC - Started this record from the template, validated it (`validate-audit-record.py`: 6 rows, 0 failures), then resolved the six threads (08:58 UTC); a GraphQL refresh shows no unresolved thread.
- 2026-11-16 09:16 UTC - Review 5917548188 (da2ce7, `APPROVED`) submitted on the head commit `docs(pr-reviews): [#2873] record the first reviews of PR #2875`: F7, a Nit, not blocking.
- 2026-11-16 09:24 UTC - Review 5917643855 (Copilot, `COMMENTED`) submitted on the same head: F8 (inline) and F9 (review body).
- 2026-11-16 09:38 UTC - Committed the F7 and F8 fixes (09:37:21 and 09:38:35 UTC). A first push attempt at 09:41 UTC did not reach GitHub: its output was piped to `head`, which ended it during the pre-push hook. The replies on the F7 and F8 threads (09:41:58 and 09:42:00 UTC) were posted before the fixes were on GitHub.
- 2026-11-16 09:44 UTC - Pushed the F7 and F8 fixes (push finished 09:44:39 UTC); the push dismissed review 5917548188 (timeline event `review_dismissed`, 09:44:41 UTC).
- 2026-11-16 09:48 UTC - Pushed the F9 fix (committed 09:45:14 UTC; push finished 09:48:46 UTC) and posted the F9 response on the PR conversation (09:48:53 UTC).
- 2026-11-16 09:49 UTC - Resolved the F7 and F8 threads (09:49 UTC); a GraphQL refresh shows no unresolved thread. `validate-audit-record.py` was not run this round: it calls plain `gh`, which acts as the maintainer's account, and its use needs his approval.

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
