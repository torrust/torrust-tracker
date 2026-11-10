---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2802 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2802.

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

Three review rounds so far: da2ce7 round 1 (review 5842591976, F1-F6), Copilot (review 5842646453,
F7-F9) and da2ce7 round 2 (review 5842971275, F10-F16). da2ce7's round-2 IDs F2-F5 and F7-F9 collide
with round 1, so they are reassigned here and the original ID is kept in each detail entry; F11, F12,
F14 and F15 re-raise a residual of an earlier finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2802-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2802-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2802-f3 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2802-f4 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2802-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2802-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2802-f7 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2802-f8 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2802-f9 | Copilot | Minor (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2802-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2802-f11 | Human | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F12 | review-finding:pr-2802-f12 | Human | Minor | metadata | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F13 | review-finding:pr-2802-f13 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2802-f14 | Human | Nit | documentation | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F15 | review-finding:pr-2802-f15 | Human | Nit | documentation | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F16 | review-finding:pr-2802-f16 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The new constant took the `Cli` doc comment, so `--help` lost the program description

- PR number: 2802
- Source review ID: 5842591976
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525394567
- Concern: `MAX_TIMEOUT_SECONDS` sat between the `Cli` doc comment and its derive, so clap used the constant's text and `--help` lost the description.
- Solution: Moved the constant above the doc comment and added a help-test assertion for the description and the maximum.
- Current-tree verification: `target/debug/wait-for-pr-review --help` starts with "Block until a watched reviewer"; the help test asserts it; 27 integration tests pass.
- Resolution reference: fix(wait-for-pr-review): keep the --help description and tighten the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525434964

### F2 - V2 names its build vaguely and records no green run

- PR number: 2802
- Source review ID: 5842591976
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525394576
- Concern: V2 named "the implementation commit" instead of a subject, and had a one-sentence red run and no green output.
- Solution: Named the build by subject, added a run time, the red failures with their messages, and the green counts.
- Current-tree verification: Read V2 in `manual-verification-evidence.md`; `cargo test -p wait-for-pr-review` gave 62 and 27 passed.
- Resolution reference: fix(wait-for-pr-review): keep the --help description and tighten the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525435427

### F3 - The spec's bookkeeping disagreed with itself after the change

- PR number: 2802
- Source review ID: 5842591976
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525394584
- Concern: Frontmatter, the unstamped log entry and the unticked checkboxes disagreed with the DONE task and acceptance rows.
- Solution: Updated `related-pr` and `last-updated-utc`, ticked the supported checkboxes, left the completion review and the move to `closed/` for the close-out. The reply claimed a stamp that did not exist yet; that was corrected in round 2 (F12).
- Current-tree verification: Read the frontmatter and checkpoints in `ISSUE.md`.
- Resolution reference: fix(wait-for-pr-review): keep the --help description and tighten the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525435752

### F4 - `gh_calls` counts only the pull-request endpoint

- PR number: 2802
- Source review ID: 5842591976
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525394596
- Concern: The helper counted only `pulls/7` calls, so its name and AC1's wording claimed more than it checked.
- Solution: Renamed the helper to `pull_calls` and documented that it does not count `gh auth status`.
- Current-tree verification: `grep -n pull_calls tests/cli.rs`.
- Resolution reference: fix(wait-for-pr-review): keep the --help description and tighten the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525436188

### F5 - The decision record names no source a reader can follow

- PR number: 2802
- Source review ID: 5842591976
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525394608
- Concern: The spec did not say where the one-day maximum was chosen, and the PR description overstated the spec.
- Solution: The decision paragraph now records that the maintainer answered in the implementation session and that no GitHub comment exists.
- Current-tree verification: Read the `Decision` paragraph in `ISSUE.md`.
- Resolution reference: fix(wait-for-pr-review): keep the --help description and tighten the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525436540

### F6 - The PR title carries no `[#2713]`

- PR number: 2802
- Source review ID: 5842591976
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525394611
- Concern: The open-pull-request skill asks for `[#<issue>]` in the title of a pull request that closes an issue.
- Solution: Edited the PR title; the commit subject keeps its suffix because history is not rewritten. A title edit is outside the tree, so the reply is the durable reference.
- Current-tree verification: `gh pr view 2802 --json title` starts with `[#2713]`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525436986
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525436986

### F7 - The constant's comment claims `Instant + Duration` cannot overflow on any platform

- PR number: 2802
- Source review ID: 5842646453
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525434872
- Concern: Rust guarantees no minimum `Instant` range, so "cannot overflow" is too strong; suggested `Instant::checked_add`.
- Solution: Reworded the comment to say one day is far inside the range of every supported platform and that Rust gives no minimum. `checked_add` was not added: the spec chose a parse-time bound over a run-time check, and the bound is what the tests pin.
- Current-tree verification: Read the doc comment on `MAX_TIMEOUT_SECONDS` in `main.rs`.
- Resolution reference: docs(wait-for-pr-review): address Copilot review round on the timeout bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525472157

### F8 - `pull_calls` panics on an unparsable `pull-calls` file

- PR number: 2802
- Source review ID: 5842646453
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525434938
- Concern: `trim().parse().unwrap()` fails without a clear message when the counter file is empty.
- Solution: Replaced `unwrap` with `expect` carrying a descriptive message.
- Current-tree verification: `grep -n expect tests/cli.rs`; 27 integration tests pass.
- Resolution reference: docs(wait-for-pr-review): address Copilot review round on the timeout bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525472572

### F9 - An inline code span in the evidence file contains a newline

- PR number: 2802
- Source review ID: 5842646453
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525435008
- Concern: The recorded V2 stderr was an inline code span broken over two lines.
- Solution: Put the record in a fenced block on one line.
- Current-tree verification: Read V2 in `manual-verification-evidence.md`.
- Resolution reference: docs(wait-for-pr-review): address Copilot review round on the timeout bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525473014

### F10 - No review audit record exists at this head

- PR number: 2802
- Source review ID: 5842971275
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525681677
- Concern: The process-pr-review skill makes the author own `docs/pr-reviews/pr-2802-review/PR-REVIEW.md`; none existed.
- Solution: Added this record and ran `validate-audit-record.py`.
- Current-tree verification: `validate-audit-record.py --pr-number 2802` exits 0.
- Resolution reference: docs(pr-reviews): add the review audit record for #2802
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525738322

### F11 - V2's build is still not a commit on the branch

- PR number: 2802
- Source review ID: 5842971275
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525681682
- Concern: V2 named a subject plus uncommitted fixes; the evidence should name the commit whose build the recheck ran on.
- Solution: V2 now names the subject of the commit with the production change and says why the later doc-comment commit needs no rerun.
- Current-tree verification: Read the `Build` line of V2.
- Resolution reference: docs(wait-for-pr-review): name the commit V2 ran on in the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525738769

### F12 - `status`, `branch` and one log stamp are still stale

- PR number: 2802
- Source review ID: 5842971275
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525681689
- Concern: `status: planned`, the spec branch in `branch`, and an unstamped 2026-11-10 entry remained.
- Solution: Set `status: in-progress`, `branch` to the PR branch, and stamped the entry 12:59 UTC from `git log --format=%cI`.
- Current-tree verification: Read the frontmatter and the log in `ISSUE.md`.
- Resolution reference: docs(wait-for-pr-review): correct the spec status, branch and log stamps
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525739187

### F13 - Three stamps disagree with `git log`

- PR number: 2802
- Source review ID: 5842971275
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525681695
- Concern: Two 13:32 stamps and the evidence file's 13:28 stamp did not match the commit times.
- Solution: Set them to 13:31 from `git log --format=%cI`.
- Current-tree verification: `git log --format=%cI` gave 13:31:52 for the commit carrying them.
- Resolution reference: docs(wait-for-pr-review): correct the spec status, branch and log stamps
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525739551

### F14 - AC1's evidence and the PR description still say "no `gh` call"

- PR number: 2802
- Source review ID: 5842971275
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525681701
- Concern: The test counts only pull-request calls, so "no `gh` call" claims more.
- Solution: Reworded the scope line, AC1 and its evidence in the spec; the PR description was edited too (outside the tree).
- Current-tree verification: `grep -n 'gh. call' ISSUE.md` finds nothing.
- Resolution reference: docs(wait-for-pr-review): say no pull-request call where the test counts only that
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525740001

### F15 - The PR description overstates the spec's option 1 values

- PR number: 2802
- Source review ID: 5842971275
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525681713
- Concern: The description called one day a choice among the spec's values, while the spec names only one year.
- Solution: The description now says "the maintainer's choice". A description edit is outside the tree, so the reply is the durable reference.
- Current-tree verification: `gh pr view 2802 --json body` no longer has the phrase.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525740439
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525740439

### F16 - Commits bundle several fixes and one is typed `docs` with a test change

- PR number: 2802
- Source review ID: 5842971275
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525681722
- Concern: `223de5ae2` mixes types and each new commit bundles independent fixes.
- Solution: Nothing to repair in the bytes; from this round on each fix is its own commit, with a matching type.
- Current-tree verification: Round-2 fixes are three separate commits; see the replies.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525740818
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2802#discussion_r4525740818

## Processing Log

- 2026-11-10 13:28 UTC - Round 1 fixes for F1-F6 committed (commit time of "fix(wait-for-pr-review): keep the --help description and tighten the evidence"); replies and resolution followed.
- 2026-11-10 13:31 UTC - Copilot fixes for F7-F9 committed (commit time of "docs(wait-for-pr-review): address Copilot review round on the timeout bound"); replies and resolution followed.
- 2026-11-10 13:56 UTC - Round 2 fixes for F11-F14 committed as three commits; the PR description was edited for F14 and F15.

Append entries only. If an entry was rewritten in place, restore the prior entries, then append a
correction that names the rewritten content; do not rewrite the record again.

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
