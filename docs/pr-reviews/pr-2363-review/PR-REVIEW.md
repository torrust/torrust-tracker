---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2363 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2363>.

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

Audit IDs `F1`-`F4` are the reviewer's IDs from review 5337742420 (round 1, at the draft head
`docs(issues): [#2347] record the rebase and the upstream F3 fix`). No earlier finding exists, so
no ID collides. The review body lists the same four findings and contains no separate request.
F2, F3, and F4 were already fixed by commits pushed after that head.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2363-f1` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2363-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2363-f3` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2363-f4` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2363-f5` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2363-f6` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2363-f7` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2363-f8` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2363-f9` | Human | Suggestion | documentation | ORIGINAL | NO_ACTION | RESOLVED |
| F10 | `review-finding:pr-2363-f10` | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F11 | `review-finding:pr-2363-f11` | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F12 | `review-finding:pr-2363-f12` | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F13 | `review-finding:pr-2363-f13` | Copilot | Nit (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F14 | `review-finding:pr-2363-f14` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

Round 2: F5-F9 are the reviewer's IDs from review 5338643536, at the head
`docs(pr-reviews): [#2347] add PR #2363 review audit`. Review 5339061836 (round 3, at the head
`docs(issues): [#2347] record the approval-timing deviation acceptance`) found nothing new and
confirmed that F5-F9 still stood. Copilot review 5338724829 (a partial review: its full run timed
out) left four unnumbered inline comments, which are audit F10-F13 in source order. Its review
overview rates F10 Medium and F11-F13 Low. Those labels are outside the severity vocabulary, so,
following `pr-2352-review` and `pr-2353-review`, F10 is `Minor (inferred)` and F11-F13 are
`Nit (inferred)`. Its review body only lists the four comments and adds no
other request. F9 was fixed by editing the PR description, outside the repository, so it has no
admissible `FIXED` resolution reference under the current contract (`review-finding:pr-2313-f6`,
owned by #2362). It is recorded as `NO_ACTION`/`RESOLVED` with its reply URL, as #2347 did for
`review-finding:pr-2290-f7`.

Round 4 (review 5340437680, at the head `docs(issues): [#2347] map the pre-PR task review finding
IDs`) approved that head and raised no new finding. It was dismissed automatically when the round-2
audit commit was pushed. Round 5 (review 5340689822, at the head
`docs(pr-reviews): [#2347] record PR #2363 review round 2`) raised F14.

## Finding Details

### F1 - The F29 in-place correction adds a line to this file, so three existing #2320 audit entries now cite wrong line numbers

- PR number: 2363
- Source review ID: 5337742420
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121490638>
- Concern: The #2320 F29 correction grew the retrospective's Timeline note from two lines to
  three, shifting every later line by one. Records F23, F24, and F26 of the #2320 audit then cited
  wrong lines, and the #2320 F28 `NO_ACTION` premise ("nothing asserts false") stopped holding.
- Solution: `docs(pr-reviews): [#2347] keep the #2320 head-id note at two lines` re-wraps the
  corrected note to two lines with the same wording, drops the F28 row's line-shift sentence, and
  logs the re-wrap in the #2320 audit. This is the reviewer's cheapest fix.
- Current-tree verification: comparing lines 66 onward of `review-retrospective.md` with `develop`
  shows no difference. `git grep -nE 'round-4 body|Branch ids: F2, F9|once, the 20:12 push'` on
  the file returns lines 161, 429, 433, and 434. `validate-audit-record.py --pr-number 2320
  --base a110200d` exits `0` (29 rows).
- Resolution reference: `docs(pr-reviews): [#2347] keep the #2320 head-id note at two lines`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121982138>

### F2 - This bullet still says all 14 approved fixes are recorded as `FOLLOW_UP`/`OPEN`

- PR number: 2363
- Source review ID: 5337742420
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121490655>
- Concern: The spec's "Approved fixes stay `FOLLOW_UP` until merge" bullet ignored amendment 2:
  #2293 F3 is `FIXED`/`RESOLVED` at the reply step, and its thread does not end with three
  replies.
- Solution: `docs(issues): [#2347] correct stamps and stale claims found by the pre-PR review`
  rewrites the bullet. It names the 13 fixes that land here and the #2293 F3 exception, and limits
  the three-reply sentence to threads fixed here. The pre-PR task review raised the same point.
- Current-tree verification: the spec reads "In this PR, the 13 approved fixes that land here are
  therefore recorded as `FOLLOW_UP`/`OPEN`" and "#2293 F3 is the exception".
- Resolution reference: `docs(issues): [#2347] correct stamps and stale claims found by the pre-PR review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121982440>

### F3 - `last-updated-utc` is older than the file's last two edits

- PR number: 2363
- Source review ID: 5337742420
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121490677>
- Concern: `triage.md` read `last-updated-utc: "2026-09-27 19:05"` after two later edits.
- Solution: The same commit, `docs(issues): [#2347] correct stamps and stale claims found by the
  pre-PR review`, refreshes the stamp. The pre-PR task review raised the same point.
- Current-tree verification: `grep -n last-updated triage.md` prints
  `3:last-updated-utc: "2026-09-28 11:48"`.
- Resolution reference: `docs(issues): [#2347] correct stamps and stale claims found by the pre-PR review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121982667>

### F4 - The #2290 F5 disposition reply cites an audit commit that this head does not contain yet

- PR number: 2363
- Source review ID: 5337742420
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121490695>
- Concern: The #2290 F5 reply names `docs(pr-reviews): [#2347] add PR #2290 post-merge review
  audit`, which stays true only if T4 commits the audit under exactly that subject.
- Solution: T4 committed the #2290 audit under that exact subject.
- Current-tree verification: `git log --oneline --fixed-strings --grep='add PR #2290 post-merge
  review audit'` returns exactly one commit, and it adds
  `docs/pr-reviews/pr-2290-review/PR-REVIEW.md`.
- Resolution reference: `docs(pr-reviews): [#2347] add PR #2290 post-merge review audit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4121982923>

<!-- cspell:ignore undercounts -->

### F5 - This sentence undercounts the #2320 citations the F29 correction moved, and misdates their fix

- PR number: 2363
- Source review ID: 5338643536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4122160567>
- Concern: The retrospective said one audit line number moved and was corrected before the pre-PR review. The F29 correction moved the citations of three #2320 entries (F23, F24, and F26), and the fix landed after that review.
- Solution: `docs(issues): [#2347] count the three #2320 citations the F29 note moved` names the three entries, says the F28 row disclosed only F24's shift, and dates the re-wrap after the pre-PR review.
- Current-tree verification: `implementation-retrospective.md` reads "moved the retrospective lines cited by three #2320 audit entries (F23, F24, and F26)" and "restored all three after the pre-PR review".
- Resolution reference: `docs(issues): [#2347] count the three #2320 citations the F29 note moved`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123638432>

### F6 - "Two findings were already dead": the triage record marks three

- PR number: 2363
- Source review ID: 5338643536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4122160577>
- Concern: `triage.md` marks three findings not live (#2293 F8, #2300 F8, #2313 F10), but the retrospective named two.
- Solution: `docs(issues): [#2347] name all three findings triage found dead` names all three, and notes that triage, not the reviewer, found #2300 F8.
- Current-tree verification: the three `triage.md` rows with Live `no` are #2293 F8, #2300 F8, and #2313 F10. The retrospective reads "Three findings were already dead (#2293 F8, #2300 F8, and #2313 F10)".
- Resolution reference: `docs(issues): [#2347] name all three findings triage found dead`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123638758>

### F7 - "four improvement candidates": the retrospective now lists five

- PR number: 2363
- Source review ID: 5338643536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4122160590>
- Concern: The spec's Implementation Completion Review said four improvement candidates, after the pre-PR correction commit added a fifth.
- Solution: `docs(issues): [#2347] count five retrospective improvement candidates`. The dated 11:00 progress-log entry's "four" stands.
- Current-tree verification: the retrospective's `## Improvements for Future Work` has five numbered items, and the spec reads "five improvement candidates".
- Resolution reference: `docs(issues): [#2347] count five retrospective improvement candidates`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123639093>

### F8 - The follow-up actions cite findings F1-F15, but the Findings list carries no ids

- PR number: 2363
- Source review ID: 5338643536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4122160595>
- Concern: The pre-PR task review entry's follow-up actions cite F1-F15, but its grouped findings carry no IDs.
- Solution: `docs(issues): [#2347] map the pre-PR task review finding IDs` appends a "Finding-ID Correction" entry that maps F1-F15 and records what fixed each. The file is append-only, and the 11:41 entry is unchanged.
- Current-tree verification: `agent-review-reports.md` has a `### 2026-09-28 14:37 UTC - GitHub Copilot Finding-ID Correction` entry listing F1 through F15.
- Resolution reference: `docs(issues): [#2347] map the pre-PR task review finding IDs`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123639421>

### F9 - The PR body's Audits and validator lines do not mention this record

- PR number: 2363
- Source review ID: 5338643536
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4122160599>
- Concern: The PR description's audit and validator lines listed five audits and omitted this record.
- Solution: The PR description was edited at 14:54 UTC. The audit line now names `docs/pr-reviews/pr-2363-review/`, and the validator line adds this audit. No repository change was needed; see the Findings intro for why the row is `NO_ACTION`.
- Current-tree verification: `gh pr view 2363 --json body` contains `pr-2363-review` twice.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123639705>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123639705>

### F10 - Quote the #2361 evidence file's `last-updated-utc`

- PR number: 2363
- Source review ID: 5338724829
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4122225231>
- Concern: The unquoted `last-updated-utc` might be read as a datetime, and differs from the quoted stamps elsewhere in the PR.
- Solution: No change. The value follows the evidence template (`last-updated-utc: YYYY-MM-DD HH:MM`, unquoted) and loads as a string. The double-quoted rule applies only to strict issue and EPIC records. Round 3 (review 5339061836) reached the same conclusion.
- Current-tree verification: `yaml.safe_load` returns a `str` for `last-updated-utc: 2026-09-28 10:00`, and `frontmatter-validator` exits `0` for the file.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123640055>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123640055>

### F11 - Quote the #2222 evidence file's `last-updated-utc`

- PR number: 2363
- Source review ID: 5338724829
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4122225308>
- Concern: The same as F10, for the closed #2222 evidence file.
- Solution: No change. The stamp was unquoted on `develop` before this PR, and only its value changed here; the rest is as for F10.
- Current-tree verification: `git show 478516cf:` of the file shows `last-updated-utc: 2026-09-22 16:40`. `yaml.safe_load` returns a `str`, and `frontmatter-validator` exits `0`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123640365>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123640365>

### F12 - Quote the closed #2261 spec's `last-updated-utc`

- PR number: 2363
- Source review ID: 5338724829
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4122225360>
- Concern: The same as F10, for the closed #2261 spec.
- Solution: No change. The spec has no `schema-version`, so the strict v1 profile does not apply. Its stamp was unquoted on `develop` before this PR, and only its value changed here.
- Current-tree verification: `grep -c '^schema-version'` prints `0`. `git show torrust/develop:` of the file showed `last-updated-utc: 2026-09-22 10:18` before this PR. `frontmatter-validator` exits `0`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123640634>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123640634>

### F13 - "Nothing record F24 asserts is false" reads as missing a word

- PR number: 2363
- Source review ID: 5338724829
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4122225399>
- Concern: Copilot read the #2320 F28 solution sentence as ungrammatical.
- Solution: No change. The sentence is a relative clause with the relative pronoun omitted ("Nothing [that] record F24 asserts is false"). Round 3 reached the same conclusion.
- Current-tree verification: the #2320 audit's F28 row reads "Nothing record F24 asserts is false".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123640914>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123640914>

### F14 - "Copilot gave no severity": the Copilot review overview labels each comment, one Medium and three Low

- PR number: 2363
- Source review ID: 5340689822
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4123802356>
- Concern: The Round 2 paragraph said Copilot gave no severity and recorded all four Copilot rows as `Nit (inferred)`. Copilot's review overview rates F10 Medium and F11-F13 Low.
- Solution: `docs(pr-reviews): [#2347] record Copilot's severity labels in the PR #2363 audit` states the labels and records F10 as `Minor (inferred)` and F11-F13 as `Nit (inferred)`, as `pr-2352-review` and `pr-2353-review` do. The cause was stripping the review body's HTML before reading it, which removed the `<picture>` severity badges.
- Current-tree verification: the raw body of review 5338724829 contains `alt="Medium severity"` and `alt="Low severity"`. The F10 row reads `Minor (inferred)`, and the Round 2 paragraph reads "Its review overview rates F10 Medium and F11-F13 Low".
- Resolution reference: `docs(pr-reviews): [#2347] record Copilot's severity labels in the PR #2363 audit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2363#discussion_r4124584947>

## Processing Log

- 2026-09-28 12:03 UTC - Started processing round 1: review 5337742420 (da2ce7, `CHANGES_REQUESTED`, submitted 11:16 UTC at the draft head) with four inline findings. At the current head, F2, F3, and F4 were already fixed by commits pushed after that head; F1 was live (`git grep` returned 162, 430, 434, and 435).
- 2026-09-28 12:10 UTC - Committed the F1 fix, `docs(pr-reviews): [#2347] keep the #2320 head-id note at two lines` (authored 12:05:02Z), and pushed it; the pre-push hook passed and the push finished at 12:10.
- 2026-09-28 12:12 UTC - Posted a reply on all four threads (`created_at` 12:11:54Z-12:12:00Z), each with the fixing commit subject and the current-tree result.
- 2026-09-28 12:17 UTC - `github-review-threads reply-status` confirmed a reply on all four threads, and I resolved them with `resolve-all-unresolved-threads.sh`. A GraphQL refetch shows zero unresolved threads on PR #2363.
- 2026-09-28 13:53 UTC - Started processing round 2. The inputs were review 5338643536 (da2ce7, `CHANGES_REQUESTED`, submitted 12:32 UTC) with F5-F9; Copilot review 5338724829 (12:39 UTC) with four unnumbered comments; and review 5339061836 (round 3, 13:07 UTC), which reconfirmed F5-F9, added nothing new, and judged the four Copilot items not to be defects.
- 2026-09-28 14:40 UTC - Committed the F5-F8 fixes, authored 14:35:16Z-14:38:02Z after one GPG timeout was retried with the maintainer at the terminal, and pushed them; the pre-push hook passed.
- 2026-09-28 14:54 UTC - Edited the PR description for F9.
- 2026-09-28 14:55 UTC - Posted a reply on all nine threads (`created_at` 14:55:25Z-14:55:39Z). The F10-F13 replies use the `Superseded by <FindingId>:` form. At 14:55:57Z the F13 reply was edited to remove a false sentence, which had claimed the #2320 F28 disposition reply used the same wording; it reads "Nothing the record asserts is false".
- 2026-09-28 14:56 UTC - `reply-status` confirmed a reply on all nine threads, and I resolved them with `resolve-all-unresolved-threads.sh`. A GraphQL refetch shows zero unresolved threads on PR #2363.
- 2026-09-28 15:01 UTC - GitHub dismissed round 4 (review 5340437680, da2ce7 `APPROVED` at 14:51 UTC, with PR comment `ACK 068a0189b…`) as stale when the round-2 audit commit was pushed. Round 4 raised no finding.
- 2026-09-28 16:25 UTC - Round 5 (review 5340689822, da2ce7, `CHANGES_REQUESTED` at 15:10 UTC) raised F14. Committed the F14 fix `docs(pr-reviews): [#2347] record Copilot's severity labels in the PR #2363 audit` (authored 16:20:03Z) and pushed it; the pre-push hook passed.
- 2026-09-28 16:27 UTC - Replied on the F14 thread (`created_at` 16:26:09Z), confirmed the reply with `reply-status`, and resolved it. A GraphQL refetch shows zero unresolved threads on PR #2363.

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
