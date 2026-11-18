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
last-updated-utc: "2026-11-18 17:25"
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
    - .claude/skills/process-pr-review/scripts/validate-audit-record.py
    - contrib/dev-tools/git/github-merge.py
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2949 - Keep a reviewer's ACK valid when only the pull request's audit record changed after it

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

A reviewer's `ACK <X>` must still count on the current head when the commits after `X` change only the pull
request's own review audit record. Recording a finding must not cost a new ACK.

## Background

The review process keeps one audit record per pull request, on the pull request's own branch:
`docs/pr-reviews/pr-<N>-review/PR-REVIEW.md` (`process-pr-review`, Workflow). Two rules require the reviewer's
ACK to name the head commit:

- `merge-pull-request`, Merge Readiness: "the other maintainer has commented `ACK <full head commit hash>` on
  the head commit".
- `wait-for-pr-review`, step 4: the same condition is part of the loop's exit condition.

The two rules work against each other when a finding arrives together with, or after, an ACK. Recording the
finding is a commit. The commit moves the head. The ACK no longer names the head, and GitHub dismisses the
approval. The reviewer must review and ACK again, and the maintainer must approve the workflow runs again. A new
finding at that point starts the same loop.

It has happened at least twice:

- PR #2947 (2026-11-18). da2ce7 approved head `e01212151` with one non-blocking Minor finding and posted the
  ACK; the maintainer posted the same ACK. The author fixed the finding by editing the pull request description
  (no commit), replied, resolved the thread and committed the audit record (`1d27a67e9`). Both ACKs became stale
  for a commit that only records a non-blocking note.
- PR #2363 (2026-09). The EPIC specification's #2347 close-out plan, which it marks as historical, has an
  unticked line to log "the stale-approval dismissal after the round-2 audit push".

The maintainer decided on 2026-11-18 to adopt the rule in this specification and to write it spec-first. Until
it lands, the audit commit is kept and the maintainer ACKs the new head again (applied to PR #2947). The
decision was made in a working session, so this specification and its pull request are its record in the
repository.

### Facts checked on `develop` (`96923f0d2`)

- The ACK rule is stated in two places only: `merge-pull-request/SKILL.md` line 63 and
  `wait-for-pr-review/SKILL.md` line 178.
- `merge-pull-request` says the ruleset requires no approvals, so a dismissed GitHub approval blocks no merge.
- The merge tool `contrib/dev-tools/git/github-merge.py` computes ACKs itself. `get_acks_from_comments` keeps a
  comment line that contains `ACK` and the first six characters of the head commit. It writes the result in the
  merge commit message ("ACKs for top commit:" or "Top commit has no ACKs.") and prints a warning when there is
  none. An ACK on an earlier commit is therefore not listed today.
- Merge Readiness says to record an ACK waiver "as a durable comment on the pull request before merging, and in
  the review audit". Recording it in the audit is another commit on the branch, which moves the head again.

## Scope

### In Scope

- The rule, stated once and referenced from the other places:
  a reviewer's `ACK <X>` counts on head `H` when `X` is `H`, or when `X` is an ancestor of `H` and
  `git diff --name-only X..H` lists only files under `docs/pr-reviews/pr-<N>-review/`, where `<N>` is this pull
  request's number. Any other changed file makes the ACK stale, as today.
- `merge-pull-request`, Merge Readiness: apply the rule to the Reviewer ACK check, with the exact command an
  agent or a person runs to check it.
- `merge-pull-request`, Merge Readiness: remove "and in the review audit" from the waiver sentence. The durable
  comment on the pull request is the record.
- `wait-for-pr-review`, step 4: apply the same rule to the exit condition. Step 6 keeps restarting the loop on a
  `head_changed` event; the rule only changes how the ACK is judged when the condition is checked again.
- The merge tool: decide how it treats an ACK that counts under the rule (see Decisions to make), and implement
  the decision.
- `process-pr-review`: one sentence saying that an audit-only commit does not make a reviewer's ACK stale, with
  a link to the rule, and that an author never rewrites history to protect an ACK.
- da2ce7's agreement to the rule, recorded as a durable comment on the specification pull request, because the
  rule changes what their ACK covers.
- A comparison with the alternative of writing audit records after merge (see Decisions to make).
- A note on the #2363 line of `EPIC.md` that names this issue as the fix for the cause.

### Out of Scope

- Moving audit records off the pull request branch. This specification compares the alternative and records the
  reason for the choice; it does not implement it.
- Changing how contributors process findings. They keep recording each finding in the audit on the pull request
  branch, as now.
- Any change to what a reviewer may do. A reviewer may still ACK in the same round as a finding.
- GitHub's dismissal of stale approvals. It is a repository setting and it blocks nothing.
- The poller `wait-for-pr-review` (the tool). It returns events and does not decide the exit condition.
- Workflow-run approval after an audit-only push. The push still needs it.

## Architectural Decisions

- Related ADRs: none known. The implementer must check `docs/adrs/` for an ADR about the merge process or the
  vendored merge tool before changing `github-merge.py`.
- ADRs to create: none expected if the merge tool is left unchanged. If the tool's ACK matching changes, decide
  in the specification pull request whether that change needs an ADR.

### Decisions to make in the specification pull request

These are open. Each has a recommendation; the maintainer decides.

- **D1. Audit on the branch with this rule, or audit after merge.** Writing every audit record in a docs change
  after merge removes the problem at its root, and also removes the extra review round and workflow-run approval
  that each audit push causes. It is a larger change: `process-pr-review` and `validate-audit-record.py` assume
  the record is on the branch, Merge Readiness requires the record to pass the validator before merge, and every
  pull request would need a second docs pull request. The maintainer's backlog items on roles and on a more
  deterministic review process (outside this repository) bear on it. Recommendation: adopt the rule now, and
  record here what would make the after-merge design worth doing later.
- **D2. The merge tool.** Options: (a) leave `github-merge.py` unchanged and document that it prints "Top commit
  has no ACKs" when the ACK counts only under the rule; (b) change `get_acks_from_comments` to also accept an
  ACK on an ancestor that satisfies the rule, and say in the merge message which commit was ACKed.
  Recommendation: (b), because the merge commit message is the permanent record of who ACKed what, and (a)
  makes it say "no ACKs" for a pull request that was ACKed. The tool is repository-owned
  (`merge-pull-request`: "the repository-local wrapper and `contrib/dev-tools/git/github-merge.py` are
  authoritative").
- **D3. Which folder counts.** Only this pull request's own folder, `docs/pr-reviews/pr-<N>-review/`, matched as
  a whole path segment (`pr-29-review` must not match pull request 2947 or 294). Audit records of other pull
  requests changed in the same branch make the ACK stale. Recommendation: as stated.
- **D4. A deterministic check.** Whether the rule ships as text with a command, or as a small tool that prints
  whether a given ACK counts. Recommendation: text with the command in this issue; a tool only if D2 is (b), in
  which case the same function serves both.
- **D5. The maintainer's own ACK.** Whether the rule also applies to the ACK the merging maintainer posts.
  Recommendation: yes, the rule is about what an ACK covers, not about who wrote it.

## Design and Ownership Review

Not applicable. The work changes skill text and, if D2 is (b), one pure function of the merge tool. No child
process, asynchronous I/O or shared fixture is involved.

## Bug-Fix Process

Not applicable. The current behaviour follows the rules as written; this issue changes the rules.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                           | Notes / Expected Output                                                                                        |
| --- | ------ | ---------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Settle D1 to D5 and get da2ce7's agreement     | Each decision recorded under Architectural Decisions with its date and who decided; da2ce7's comment linked    |
| T2  | TODO   | State the rule in `merge-pull-request`         | Merge Readiness carries the rule and the exact check command; the waiver sentence no longer mentions the audit |
| T3  | TODO   | Apply the rule in `wait-for-pr-review` step 4  | The exit condition refers to the rule in `merge-pull-request`; it does not restate it                          |
| T4  | TODO   | Add the sentence to `process-pr-review`        | Authors know an audit-only commit keeps the ACK, and that history is never rewritten to protect one            |
| T5  | TODO   | Merge tool, as D2 decides                      | (a) the expected message is documented; or (b) the tool lists the ACK and names the ACKed commit, with tests   |
| T6  | TODO   | Note this issue on the #2363 line of `EPIC.md` | The line names this issue as the fix for the cause; that plan is historical, so nothing else in it changes     |
| T7  | TODO   | Manual verification and close-out              | M1 to M3 recorded; EPIC row updated                                                                            |

## Commit Points

| Task | Coherent change set                                          | Commit policy                             |
| ---- | ------------------------------------------------------------ | ----------------------------------------- |
| T1   | Decisions recorded in this specification                     | Commit in the specification pull request. |
| T2   | `merge-pull-request` rule and waiver sentence                | Commit after `linter all`.                |
| T3   | `wait-for-pr-review` exit condition                          | Commit after `linter all`.                |
| T4   | `process-pr-review` sentence                                 | Commit after `linter all`.                |
| T5   | Merge tool change and its tests, or the documentation of (a) | Commit after the tool's tests pass.       |
| T6   | `EPIC.md` link                                               | Commit with the EPIC progress-log entry.  |

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

## Acceptance Criteria

- [ ] AC1: `merge-pull-request` Merge Readiness states the rule and the exact command that checks it.
- [ ] AC2: `wait-for-pr-review` step 4 applies the same rule by reference, with no second wording of it.
- [ ] AC3: The waiver sentence in Merge Readiness no longer asks for a record in the review audit.
- [ ] AC4: `process-pr-review` says that an audit-only commit keeps the ACK and that history is never rewritten to protect an ACK.
- [ ] AC5: D1 to D5 are recorded with date and decider, and da2ce7's agreement is linked.
- [ ] AC6: The merge tool behaves as D2 decides, and its behaviour for an ACK that counts under the rule is documented in `merge-pull-request`.
- [ ] AC7: A commit after the ACK that changes any file outside `docs/pr-reviews/pr-<N>-review/` makes the ACK stale under every changed text and tool.
- [ ] AC8: The #2363 line in `EPIC.md` names this issue.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- The merge tool's tests, if D2 is (b)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                    | Human-oriented command/steps                                                                           | Expected Result                                                    | Status | Evidence                                     |
| --- | ------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------ | ------ | -------------------------------------------- |
| M1  | ACK followed by an audit-only commit        | On PR #2947's history, run the documented command with `X` = `e01212151` and `H` = `1d27a67e9`         | Only files under `docs/pr-reviews/pr-2947-review/`; the ACK counts | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | ACK followed by a commit outside the folder | Run the documented command on a pull request whose commits after the ACK change another file           | Another path is listed; the ACK is stale                           | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Another pull request's audit folder         | Run the documented command where the commits after the ACK change `docs/pr-reviews/pr-<other>-review/` | The ACK is stale                                                   | TODO   | `manual-verification-evidence.md` section V3 |

Notes:

- M2 and M3 may use a local throwaway branch; record the commits used.
- If D2 is (b), repeat M1 to M3 through the merge tool's own tests instead of by hand where a test covers them.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |

## Risks and Trade-offs

- **Unreviewed text enters `develop`.** The audit record changed after the ACK is merged without the reviewer
  reading it. Mitigation: the rule covers one folder that holds only the record of that pull request's review;
  the diff after the ACK is one command away; Copilot still reviews the new head.
- **The folder is used to carry other content.** A file placed under `docs/pr-reviews/pr-<N>-review/` that is
  not an audit record would pass the rule. Mitigation: decide under D3 whether the rule names the folder or only
  `PR-REVIEW.md`; `validate-audit-record.py` already checks the record.
- **The ACK names a commit that is not the merged head.** The merge commit message must say so, or the record
  misleads. Mitigation: D2 (b).
- **Two wordings of the rule drift apart.** Mitigation: the rule is stated once, in `merge-pull-request`, and
  the other skills link to it.
- **The workflow-run approval remains.** An audit-only push still needs the maintainer's approval of the runs.
  Accepted; D1 records it as a cost of keeping the audit on the branch.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #2003 (parent EPIC), #2347 (its historical close-out plan in `EPIC.md` has the #2363 line)
- Related PRs: #2947 (where the gap was reported), #2363 (earlier occurrence)
- Related ADRs: none known
