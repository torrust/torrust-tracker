---
schema-version: 1
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
epic: 2003
github-issue: 2949
spec-path: docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/ISSUE.md
branch: "2949-keep-reviewer-ack-after-audit-only-commits-spec"
related-pr: null
last-updated-utc: "2026-11-19 18:40"
semantic-links:
  skill-links:
    - create-issue
    - merge-pull-request
    - wait-for-pr-review
    - process-pr-review
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/merge-pull-request/SKILL.md
    - .claude/skills/wait-for-pr-review/SKILL.md
    - .claude/skills/process-pr-review/SKILL.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2949 - Keep the strict ACK and document the re-ACK after an audit-only commit

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Record the decision that an `ACK <hash>` counts only on the commit it names, also when the commits after it
change only the pull request's own review audit record, and document the re-ACK that follows such a commit as a
normal step of the review process.

This issue first proposed the opposite: a rule that keeps an ACK valid after audit-only commits. The maintainer
decided against it on 2026-11-19 (see Architectural Decisions). The folder name and the branch name still carry
the first title.

## Background

The review process keeps one audit record per pull request, on the pull request's own branch:
`docs/pr-reviews/pr-<N>-review/PR-REVIEW.md` (`process-pr-review`, Workflow). Two rules require the reviewer's
ACK to name the head commit:

- `merge-pull-request`, Merge Readiness: "the other maintainer has commented `ACK <full head commit hash>` on
  the head commit".
- `wait-for-pr-review`, step 4: the same condition is part of the loop's exit condition.

When a finding arrives together with, or after, an ACK, recording the finding is a commit. The commit moves the
head. The ACK no longer names the head, and GitHub dismisses the approval. The reviewer and the maintainer ACK
again, and the maintainer approves the workflow runs again. A new finding at that point starts the same loop.

It has happened at least twice:

- PR #2947 (2026-11-18). da2ce7 approved head `e01212151` with one non-blocking Minor finding and posted the
  ACK; the maintainer posted the same ACK. The author fixed the finding by editing the pull request description
  (no commit), replied, resolved the thread and committed the audit record (`1d27a67e9`). Both ACKs became stale
  for a commit that only records a non-blocking note.
- PR #2363 (2026-09). The EPIC specification's #2347 close-out plan, which it marks as historical, has an
  unticked line to log "the stale-approval dismissal after the round-2 audit push".

The maintainer decided on 2026-11-18, in a working session, to adopt a rule that keeps the ACK valid in that
case, and to write it spec-first. da2ce7 disagreed with the rule on the specification pull request and proposed
an alternative, so the decision was reopened as D1. On 2026-11-19 the maintainer decided D1 the other way: the
strict ACK stays. The skills do not yet say what to do in this situation; this issue adds that text.

### Facts checked on `develop` (`96923f0d2`)

- The ACK rule is stated in two places only: `merge-pull-request/SKILL.md`, section Merge Readiness, item Reviewer
  ACK; and `wait-for-pr-review/SKILL.md`, section The Review Loop, step 4 (the exit condition).
- `merge-pull-request` says the ruleset requires no approvals, so a dismissed GitHub approval blocks no merge.
- The merge tool `contrib/dev-tools/git/github-merge.py` computes ACKs itself. `get_acks_from_comments` keeps a
  comment line that contains `ACK` and the first six characters of the head commit. It writes the result in the
  merge commit message ("ACKs for top commit:" or "Top commit has no ACKs.") and prints a warning when there is
  none. An ACK on an earlier commit is therefore not listed, which matches the strict ACK.
- Merge Readiness says to record an ACK waiver "as a durable comment on the pull request before merging, and in
  the review audit". Recording it in the audit is another commit on the branch, which moves the head again.

## Scope

### In Scope

- The decision and the alternatives that were considered, recorded in this specification.
- `merge-pull-request`, Merge Readiness, item Reviewer ACK: say that a commit that changes only the pull
  request's own audit record makes the ACK stale like any other commit, and that the reviewer and the maintainer
  ACK the new head. Give the command that shows what the commits after the earlier ACK changed, so the second
  review is short:
  `git log --no-renames --diff-merges=first-parent --name-only --format= X..H`, where `X` is the commit of the
  earlier ACK and `H` is the head. When every listed path starts with `docs/pr-reviews/pr-<N>-review/`, the
  commits after `X` changed only the audit record, and `git diff X..H` is the whole text to read. The command
  helps the reviewer; it decides nothing, and the new ACK still names `H`.
- `merge-pull-request`, Merge Readiness: remove "and in the review audit" from the waiver sentence. The durable
  comment on the pull request is the record, and a waiver recorded in the audit moves the head once more.
- `wait-for-pr-review`, step 4: point to that text in `merge-pull-request`, without a second wording of it.
- `process-pr-review`: one sentence saying that an audit commit pushed after an ACK makes the ACK stale, that
  the author keeps the commit and tells the maintainer the head needs new ACKs, and that an author never rewrites
  history or holds back the audit record to protect an ACK.
- A note on the #2363 line of `EPIC.md` that names this issue as the decision about the dismissal it mentions.

About the command: it lists the paths each commit changes, not only the difference between the two ends, so a
change outside the folder that a later commit reverts is still listed. `--diff-merges=first-parent` lists the
paths of a merge commit, which `git log` otherwise omits. `--no-renames` lists both paths of a file moved into
the folder. All three cases were checked with Git 2.53.0 on a throwaway repository. On PR #2947's history,
`e01212151..1d27a67e9` lists only `docs/pr-reviews/pr-2947-review/PR-REVIEW.md`.

### Out of Scope

- Any rule that lets an ACK count on a commit it does not name.
- Any change to the merge tool. It already lists only ACKs on the head.
- Moving audit records off the pull request branch.
- Changing how contributors process findings. They keep recording each finding in the audit on the pull request
  branch, as now.
- Any change to what a reviewer may do. A reviewer may still ACK in the same round as a finding.
- GitHub's dismissal of stale approvals. It is a repository setting and it blocks nothing.
- Workflow-run approval after an audit-only push. The push still needs it.

## Architectural Decisions

- Related ADRs: none for the ACK. `docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md` covers how
  the merge tool's push passes the ruleset; this issue does not change it.
- ADRs to create: none. The decision changes no tool and no architecture; this specification is its record.

### D1. Where the audit record lives, and what an ACK covers

**Decision (2026-11-19, the maintainer, josecelano, in a working session with the issue #2949 session):** the
audit record stays on the pull request branch and the ACK stays strict. An `ACK <hash>` counts only on the commit
it names. After an audit-only commit, the reviewer and the maintainer ACK the new head. The decision was announced on
the specification pull request in
[comment 7018461081](https://github.com/torrust/torrust-tracker/pull/2950#issuecomment-7018461081).

The maintainer's reasons:

- The re-ACK after an audit-only commit is cheap: the diff since the earlier ACK is the audit folder, and one
  command shows that.
- The history is worth keeping. A finding, the commit that records it, the commit that fixes it and each new ACK
  stay visible in order on the pull request.
- Every alternative adds more complexity than the pain it removes.
- The review process must stay easy for external contributors, who see one branch per pull request.

Alternatives considered and rejected:

- **(a) A rule that keeps the ACK valid.** `ACK <X>` would count on head `H` when every commit after `X` changes
  only `docs/pr-reviews/pr-<N>-review/`. Rejected: it widens what an ACK vouches for; it needs an exact folder
  match, a per-commit check, and either a merge tool that reads Git history and fails closed or a merge message
  that says "no ACKs" for an ACKed pull request. da2ce7 did not agree to it
  ([pull request comment 6987829006](https://github.com/torrust/torrust-tracker/pull/2950#issuecomment-6987829006),
  2026-11-18).
- **(b) Audit records written after merge**, in a second docs pull request. Rejected: `process-pr-review`,
  `validate-audit-record.py` and Merge Readiness assume the record is on the branch, and every pull request would
  need a second one.
- **(c) A separate metadata branch, folded in with a three-parent merge.** Proposed by da2ce7 on 2026-11-18 in
  [issue comment 6987828129](https://github.com/torrust/torrust-tracker/issues/2949#issuecomment-6987828129): the
  work branch is reviewed and ACKed, a second branch changes only the audit folder, and the merge tool makes one
  commit whose parents are `develop`, the ACKed work tip and the metadata tip. It keeps the ACK strict and needs
  no workflow-run approval after recording a finding. Rejected: the author keeps two branches per pull request;
  GitHub shows one head per pull request, so the audit is hidden from the pull request view, from CI and from
  Copilot; and the merge tool and every skill step that commits the audit would change.
- **(d) Empty commits that carry the audit text in their messages.** Raised by the maintainer on 2026-11-19.
  Rejected: an empty commit still moves the head, so the ACK goes stale just the same, and the record would
  leave the file that `validate-audit-record.py` checks.

Facts about (c), kept for a later reader:

- GitHub would mark the pull request merged.
  [About pull request merges, Indirect merges](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/incorporating-changes-from-a-pull-request/about-pull-request-merges#indirect-merges)
  says a pull request can be marked as merged when its head branch commits become reachable from the base
  branch, and the work tip is a parent of the merge commit. The same section says this happens even when branch
  protection rules were not satisfied, so under (c), as today, the merge tool and Merge Readiness enforce the
  review, not GitHub. Read on 2026-11-19; pointed out by da2ce7 on PR #2950.
- Read on `develop` at `96923f0d2`: `github-merge.py` fetches only `refs/pull/<N>/*` and the base branch from
  the upstream repository, merges one head (`git merge --no-ff <head>`), and builds the commit list and the
  symbolic-link check from `<base>..<head>`. All three would need the metadata tip.
- Not tested: whether ruleset 87085 (`docs/github-rulesets.md`) accepts the tool's push when the merge commit has
  a third parent that belongs to no pull request.

### D2 to D5

D2 (the merge tool), D3 (which folder counts) and D4 (a deterministic check) were questions about the rule in
(a). They no longer apply. D5 asked whether the rule covers the merging maintainer's own ACK; with the strict
ACK, the reviewer's ACK and the maintainer's ACK both name the head.

## Design and Ownership Review

Not applicable. The work changes only skill text and documentation; no tool or code changes.

## Bug-Fix Process

Not applicable. The current behavior follows the rules as written; this issue documents it.

## Regression Test Strategy

Not applicable. No code changes.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                           | Notes / Expected Output                                                                                     |
| --- | ------ | ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Decide D1                                      | Recorded under Architectural Decisions with its date and who decided, in the specification pull request     |
| T2  | TODO   | Document the re-ACK in `merge-pull-request`    | Merge Readiness says an audit-only commit needs new ACKs and gives the command; the waiver sentence changes |
| T3  | TODO   | Point to it from `wait-for-pr-review` step 4   | The exit condition refers to the text in `merge-pull-request`; it does not restate it                       |
| T4  | TODO   | Add the sentence to `process-pr-review`        | Authors keep the audit commit, tell the maintainer, and never rewrite history to protect an ACK             |
| T5  | TODO   | Note this issue on the #2363 line of `EPIC.md` | The line names this issue; that plan is historical, so nothing else in it changes                           |
| T6  | TODO   | Manual verification and close-out              | M1 and M2 recorded; EPIC row updated                                                                        |

## Commit Points

Each commit point completes one independently reviewable change; unrelated changes are not grouped to reduce the
commit count.

| Task | Coherent change set                                  | Commit policy                             |
| ---- | ---------------------------------------------------- | ----------------------------------------- |
| T1   | Decision recorded in this specification              | Commit in the specification pull request. |
| T2   | `merge-pull-request` re-ACK text and waiver sentence | Commit after `linter all`.                |
| T3   | `wait-for-pr-review` exit condition                  | Commit after `linter all`.                |
| T4   | `process-pr-review` sentence                         | Commit after `linter all`.                |
| T5   | `EPIC.md` note                                       | Commit with the EPIC progress-log entry.  |

T2 to T6 land in one small implementation pull request after the specification pull request merges. No task adds
tests, so the progressive test-development loop of `create-issue` does not apply.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-keep-reviewer-ack-after-audit-only-commits/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-18 15:43 UTC - AI assistant (Claude Code, Claude Fable 5.2, EPIC #2003 coordinator session) - Drafted the specification from the maintainer's decision of 2026-11-18 and the report of the #2907 session on PR #2947. Facts under Background were read on `develop` at `3a12cd095`. No decision under D1 to D5 is made yet.
- 2026-11-18 17:25 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - The maintainer approved the draft on 2026-11-18. Created issue #2949 as `josecelano-bot`, moved the specification to `docs/issues/open/2949-2003-keep-reviewer-ack-after-audit-only-commits/` on the spec branch `2949-keep-reviewer-ack-after-audit-only-commits-spec`, and checked the facts under Background again on `develop` at `96923f0d2`: none changed, so only the commit in their heading changed. Formatted the file with prettier 3.9.10 (table alignment only). D1 to D5 stay open for the specification pull request.
- 2026-11-18 18:33 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - da2ce7 does not agree to the rule as written (PR #2950 comment 6987829006) and proposed a metadata branch folded in with a three-parent merge (issue comment 6987828129). D1 now compares three options, (a) the rule, (b) audit after merge and (c) the metadata branch, with the merge-tool facts for (c) read on `develop` at `96923f0d2` and one point marked not verified. The D1 recommendation changed from "adopt the rule now" to "the maintainer chooses between (a) and (c)", because (a) cannot land without da2ce7's agreement. D5 records da2ce7's suggested limit. Nothing is decided.
- 2026-11-18 19:09 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Addressed Copilot review 5963231037 on PR #2950: the rule's command is now `git diff --no-renames --name-only X..H`, with scenario M4 for a file moved into the audit folder (rename detection otherwise lists only the new path; reproduced with Git 2.53.0), and Regression Test Strategy now names the merge-tool tests that apply when D2 is (b).
- 2026-11-18 19:43 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Addressed Copilot review 5963780219 on PR #2950: Regression Test Strategy now maps four of its five test cases to M1 to M4 one by one and says the ACK-on-head case is the baseline with no manual scenario.
- 2026-11-18 20:22 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Addressed Copilot review 5964359494 on PR #2950: the checked facts cite skill sections instead of line numbers, and the rule and D3 state the folder match as the exact prefix `docs/pr-reviews/pr-<N>-review/` with its trailing slash.
- 2026-11-19 07:40 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Addressed da2ce7's review 5970994174 on PR #2950 (Copilot review 5964972845 raised F3 and F7 in its body too): the rule now checks every commit after the ACK with `git log --no-renames --diff-merges=first-parent --name-only --format= X..H`, so a reverted change outside the folder still makes the ACK stale (AC7, M5); the folder-match examples use paths that tell the forms apart (rule, D3, M3); Background and EPIC row 24 say the maintainer's decision of 2026-11-18 to adopt the rule was reopened under D1; and Commit Points carries the test-development loop for T5 when D2 is (b).
- 2026-11-19 08:20 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Correction: the 2026-11-19 07:40 UTC entry was reworded in place at 2026-11-19 08:06 UTC (commit `docs(issues): [#2949] describe the 2026-11-18 decision one way in the spec and EPIC`) to use the wording "the maintainer's decision of 2026-11-18 to adopt the rule was reopened under D1". When first written it said the rule was proposed, which was the wording of row 24 and Background at that time. da2ce7's F19 on PR #2950 pointed out that a dated entry should not be rewritten in place.
- 2026-11-19 13:49 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Correction to the 2026-11-19 08:20 UTC entry: it says the first wording of the 07:40 entry "was the wording of row 24 and Background at that time". Only EPIC row 24 said "proposed"; Background already said the maintainer decided on 2026-11-18 and D1 reopened that decision. Pointed out by da2ce7's F22 on PR #2950.
- 2026-11-19 13:49 UTC - AI assistant (Claude Code, Claude Opus 5.5, issue #2949 session) - Addressed Copilot review 5972058838 on PR #2950: Design and Ownership Review now specifies, for D2 (b), that the merge tool's `main()` acquires the Git history and that an ACK it cannot check does not count, and the note under the manual scenarios keeps the manual runs of M1 to M5 alongside the merge-tool tests.
- 2026-11-19 18:33 UTC - AI assistant (Claude Code, Claude Fable 5.2, issue #2949 session) - The maintainer decided D1 on 2026-11-19: the strict ACK stays, and an audit-only commit needs new ACKs. He also rejected carrying the audit in empty commits. Rewrote the specification as the record of that decision: the rule, the metadata branch and the audit after merge are kept as rejected alternatives; D2 to D5 no longer apply; the merge tool does not change; and the remaining work is skill text (T2 to T6) in a small implementation pull request. For da2ce7's F27 on PR #2950, the record of alternative (c) cites GitHub's documentation on indirect merges, read on 2026-11-19, and names ruleset push acceptance as the point not tested. Their F26 no longer applies.

## Acceptance Criteria

- [ ] AC1: `merge-pull-request` Merge Readiness says that an audit-only commit makes the ACK stale and needs new ACKs, and gives the command that lists what the commits after the earlier ACK changed.
- [ ] AC2: `wait-for-pr-review` step 4 points to that text, with no second wording of it.
- [ ] AC3: The waiver sentence in Merge Readiness no longer asks for a record in the review audit.
- [ ] AC4: `process-pr-review` says that the author keeps the audit commit, tells the maintainer the head needs new ACKs, and never rewrites history or holds back the audit record to protect an ACK.
- [x] AC5: D1 is recorded with its date and who decided.
- [ ] AC6: No skill or tool text says that an ACK counts on a commit it does not name.
- [ ] AC7: The #2363 line in `EPIC.md` names this issue.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                    | Human-oriented command/steps                                                                           | Expected Result                                           | Status | Evidence                                     |
| --- | ------------------------------------------- | ------------------------------------------------------------------------------------------------------ | --------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | ACK followed by an audit-only commit        | On PR #2947's history, run the documented command with `X` = `e01212151` and `H` = `1d27a67e9`         | Only `docs/pr-reviews/pr-2947-review/PR-REVIEW.md` listed | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | ACK followed by a commit outside the folder | Run the documented command on a range of `develop` whose commits change a file outside an audit folder | The other path is listed                                  | TODO   | `manual-verification-evidence.md` section V2 |

Notes:

- Record the commits used for M2.
- Both scenarios check that the documented command reads as the skill text says. Neither decides whether an ACK
  counts: an ACK counts only on the commit it names.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                       |
| ----- | ---------------------- | -------------------------------------------------------------- |
| AC1   | TODO                   |                                                                |
| AC2   | TODO                   |                                                                |
| AC3   | TODO                   |                                                                |
| AC4   | TODO                   |                                                                |
| AC5   | DONE                   | Architectural Decisions, D1, in the specification pull request |
| AC6   | TODO                   |                                                                |
| AC7   | TODO                   |                                                                |

## Risks and Trade-offs

- **The re-ACK waits for the reviewer.** An audit-only commit holds the merge until the reviewer and the
  maintainer ACK again. Accepted by the maintainer; the command makes the second review short, and the
  maintainer may waive the reviewer's ACK as Merge Readiness already allows.
- **The loop can repeat.** A review of the new head can raise another finding, which needs another audit commit.
  Accepted; a diff that holds only the audit record rarely raises one.
- **The workflow-run approval remains.** An audit-only push still needs the maintainer's approval of the runs.
  Accepted as a cost of keeping the audit on the branch.
- **A second review is skimmed.** A reviewer may ACK the new head without reading what changed. Mitigation: the
  documented command lists every path the later commits changed, including a change that was reverted.
- **Two wordings drift apart.** Mitigation: the text is stated once, in `merge-pull-request`, and the other
  skills link to it.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #2003 (parent EPIC), #2347 (its historical close-out plan in `EPIC.md` has the #2363 line)
- Related PRs: #2947 (where the gap was reported), #2363 (earlier occurrence), #2950 (the specification pull request, where the alternatives were discussed)
- Related ADRs: none for the ACK
