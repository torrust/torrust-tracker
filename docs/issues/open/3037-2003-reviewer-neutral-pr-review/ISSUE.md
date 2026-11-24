---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p1
epic: 2003
github-issue: 3037
spec-path: docs/issues/open/3037-2003-reviewer-neutral-pr-review/ISSUE.md
branch: "3037-reviewer-neutral-pr-review"
related-pr: null
last-updated-utc: "2026-11-24 18:27"
semantic-links:
  skill-links:
    - create-issue
    - wait-for-pr-review
    - process-pr-review
    - merge-pull-request
    - watch-pull-request-checks
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/wait-for-pr-review/SKILL.md
    - .claude/skills/wait-for-pr-review/tool/src/main.rs
    - .claude/skills/wait-for-pr-review/tool/src/event.rs
    - .claude/skills/process-pr-review/SKILL.md
    - .claude/skills/merge-pull-request/SKILL.md
    - .claude/skills/watch-pull-request-checks/SKILL.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - contrib/dev-tools/checks/agent-review-report-contract/src/main.rs
    - "issue #2003"
---

<!-- skill-link: create-issue -->

# Issue #3037 - PR review without a mandatory Copilot review

Parent EPIC: #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make the pull-request review process reviewer-neutral. No skill, tool default or document makes a
review from a named reviewer (today: Copilot) a condition for a pull request to be ready. Findings
from every reviewer go through the same `process-pr-review` workflow. A review that a maintainer
requests by hand, Copilot included, is still described.

## Background

On 2026-11-24 the maintainer stopped requesting Copilot pull-request reviews by default. The reasons
are cost (about 500 € per month at the current pace) and the quality of the affordable level.
CodeRabbit and Greptile are enabled on the repository on trial for about a week; Cameron (`da2ce7`)
reviews with his own agents, but not on a fixed schedule, and waiting for those reviews made pull
requests stack up. Support for Copilot reviews stays: a maintainer may still request one by hand.

The repository's skills still assume a Copilot review on every head commit. Every agent session that
reads them today gets instructions that no longer hold, so this issue goes before #2977 (maintainer,
2026-11-24). Checked on `develop` at `231daa225`:

- `wait-for-pr-review`: the description ("Wait for Copilot or maintainer reviews"); the exit
  condition "Copilot's latest review is on the current head commit"; Watching the Checks step 4
  ("request the review"); the whole "Re-Requesting a Review" section, which tells the agent to
  request Copilot and `da2ce7` after each round; the note on Copilot logins; the `--reviewer`
  default in the options table.
- The poller (`.claude/skills/wait-for-pr-review/tool/src/main.rs`) watches only `da2ce7` and
  `copilot-pull-request-reviewer[bot]` by default, so it never wakes for another reviewer unless a
  session passes it by hand. Tests in `tool/src/event.rs` use the Copilot login.
- `merge-pull-request`, Merge Readiness: a "Copilot review" check.
- `process-pr-review` step 2 and `docs/templates/PR-REVIEW-TEMPLATE.md`: the author class is
  `Copilot`, `Human` or `Unknown`, so every other bot is `Unknown`. The
  `agent-review-report-contract` check pins these sentences.
- `AGENTS.md` ("After opening a PR … and re-request the review"), `open-pull-request` "After
  Opening", `.claude/agents/pr-review-handler.md`, `.claude/skills/README.md` and the "Copilot
  Suggestions Handler" nodes of `docs/agents/orchestration.md` repeat the Copilot-centred wording.

What the reviewers do on this repository's pull requests (read with the GitHub API on 2026-11-24):

- **CodeRabbit** (`coderabbitai[bot]`) reviews fork pull requests of `josecelano-bot` by itself and
  again after later pushes (PR #3020: reviews on three different head commits). When it finds
  nothing it submits no review and only edits its summary issue comment ("No actionable comments
  were generated"; PRs #3023, #3026, #3030). When it answers a thread reply it submits a
  `COMMENTED` review with an empty body (four on PR #3020). It reports a check run named
  `CodeRabbit` from its own GitHub App, not from a workflow (`success` or `skipped`).
- **Greptile** (`greptile-apps[bot]`) reviewed PRs #3001 to #3015 (a review and an issue comment)
  and posted nothing on any pull request from #3018 to #3032. The maintainer has since activated a
  free plan for `josecelano-bot`, which may explain it. This issue does not depend on it.
- **Copilot** (`copilot-pull-request-reviewer[bot]`) reviewed every pull request up to #3018, and
  none after.

## Decisions

The maintainer answered these questions on 2026-11-24, in the session that drafted this
specification.

1. **Split with #2977: this issue removes Copilot as a condition; #2977 keeps the deadline and the
   hand-over report.** This issue rewrites "Re-Requesting a Review" into a section on reviews a
   maintainer requests, and says only a maintainer requests a review. #2977 then adds the 60-minute
   deadline and the fixed hand-over report on top of it. This issue goes first.
2. **Rules for every contributor:** no review from a named reviewer is required; a pull request is
   ready when the checks pass, every review received so far is addressed, and the Merge Readiness
   ACK rule holds (unchanged). The 5-round limit counts rounds from every reviewer. A review that
   arrives after the pull request looked ready reopens the loop. **Rule for AI agents only:** an
   agent never triggers a review (no reviewer request, no bot command such as `@coderabbitai
   review`, no `@`-mention); human contributors may still ask a reviewer. A reviewer that reviews by
   itself after a push (CodeRabbit today) is not triggered by the agent: the agent processes its new
   findings like any other review.
3. **Poller default: every account except the pull request's author.** No reviewer login is
   hard-coded. `--reviewer` still narrows the set. Accepted cost: the poller also wakes on a
   reviewer's empty thread-reply reviews, which a session already sees today when it passes the bots
   by hand.
4. **The audit record names the reviewer by login.** New audit records replace the `Author class`
   column with a `Reviewer` column holding the GitHub login (for example `coderabbitai[bot]`,
   `da2ce7`), or `unknown` when GitHub shows no author. Existing records keep `Author class` and stay
   valid; nothing is migrated.
5. **Review-bot check runs do not count for readiness "for now".** Readiness counts the check runs
   of the repository's workflows. A check run that a review service posts from its own GitHub App
   (for example `CodeRabbit`) neither blocks nor is waited for. The maintainer does not trust the
   external reviewers enough to let them gate a pull request. Proposed mechanism, to confirm during
   implementation: `gh pr checks --json name,bucket,workflow` reports an empty `workflow` for such a
   check run (verified on PRs #3014 and #3032), so no reviewer name is needed.
6. **`.github/agents/` and the `process-copilot-suggestions` redirect are not edited here.** The
   #2904 migration owns `.github/agents/`. The deprecated redirect stays as it is. The reviewer
   wording of `docs/agents/orchestration.md` is updated here, after checking with the #2904 session.

## Relationship to Other Issues

- **#2977** (`wait-for-pr-review`: reviewer deadline, who re-requests a review, hand-over report):
  same skill. This issue takes "who re-requests" (decision 1) and goes first; #2977 keeps the
  60-minute deadline and the hand-over report, and its specification is written against the text
  this issue leaves. Its own description should be narrowed when it starts.
- **#2976** (`wait-for-pr-review`: report workflow runs waiting for approval): same poller, different
  events. No overlap; whichever lands second rebases.
- **#2981** (maintainer tools: queue, request reviews, approve runs): the request-reviews helper is
  the maintainer's side of decision 2. No overlap.
- **#2904** (migrate custom agents to Claude Code): owns `.github/agents/`, including
  `copilot-suggestions-handler.agent.md` and `pr-reviewer.agent.md` (decision 6). Its session said
  on 2026-11-24 that nothing is in flight in `.github/agents/`, `docs/agents/` or the contract
  check. Its later task T9 rewrites `docs/agents/orchestration.md`, and T10 removes `.github/agents/`
  together with the contract-check rules that name those profiles. This issue lands first and #2904
  rebases on it, so T7 keeps its `orchestration.md` change to the reviewer wording, and the
  implementation PR tells the #2904 session which pinned sentences changed.
- **#2949** (an ACK names one commit): the Merge Readiness ACK rule is not changed here.

## Scope

### In Scope

- The poller's default reviewer set and its tests (decision 3).
- `wait-for-pr-review`: description, options table, exit condition, Watching the Checks, the
  re-request section, the login note (decisions 1, 2, 3, 5).
- `watch-pull-request-checks`: how readiness ignores check runs that do not come from a workflow
  (decision 5).
- `merge-pull-request` Merge Readiness: replace the "Copilot review" check (decisions 2, 5).
- `process-pr-review`, `docs/templates/PR-REVIEW-TEMPLATE.md`, `validate-audit-record.py` if it reads
  the column, and the `agent-review-report-contract` check: the `Reviewer` column, with the legacy
  `Author class` column still accepted (decision 4).
- Reviewer-neutral wording in `AGENTS.md`, `open-pull-request` "After Opening",
  `.claude/agents/pr-review-handler.md`, `.claude/skills/README.md` and
  `docs/agents/orchestration.md`.
- An ADR recording decisions 2 to 5 (see Architectural Decisions).

### Out of Scope

- The 60-minute reviewer deadline and the fixed hand-over report (#2977).
- `.github/agents/` and the `process-copilot-suggestions` redirect (decision 6).
- Copilot as a coding agent: `.github/workflows/copilot-setup-steps.yml`, `docs/adrs/*copilot*`,
  the model-provenance examples in `docs/templates/ISSUE.md` and `EPIC.md`.
- Rewriting existing audit records under `docs/pr-reviews/`.
- Choosing which reviewers stay after the trial; configuring CodeRabbit or Greptile.
- The Merge Readiness ACK rule (#2949).

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
  (provider-specific tools are optional adapters: a named reviewer must not be required).
- ADRs to create: one root ADR in `docs/adrs/`, "Review pull requests without a mandatory
  reviewer" (created in T1: `docs/adrs/20261124182709_review_pull_requests_without_a_mandatory_reviewer.md`). It records decisions 2 to 5 and the alternatives that were rejected (require Copilot;
  require the new bots on the head; hard-code or configure the reviewer list; per-bot or generic
  author classes; count review-bot check runs).

## Design and Ownership Review

Not applicable: no child processes, readiness waits or fixtures change. The poller keeps its
existing wait loop; only which logins count changes.

## Bug-Fix Process

Not applicable. The skills describe the process the maintainer used until 2026-11-24 correctly;
this issue changes the process, so there is no defect to reproduce.

## Regression Test Strategy

Not applicable (not a bug). The poller's new default is covered by unit tests in T2.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | DONE   | Write the ADR | Root ADR recording decisions 2 to 5 and the rejected alternatives. |
| T2  | DONE   | Poller default reviewers | No default logins in `main.rs`. Without `--reviewer`, every review and comment counts except those by the pull request's author (read from the pull request, compared without regard to case). With `--reviewer`, only the listed logins count, as today. Unit tests: a review from a login in no list is reported by default; the author's own review and comment are not; `--reviewer` narrows. Replace the Copilot-specific test names with reviewer-neutral ones (the "never approves" case stays, with any bot login). |
| T3  | DONE   | `wait-for-pr-review` skill text | Description and options table match T2. Exit condition: drop the Copilot bullet; ready = checks pass (T4), no unresolved actionable thread, every review received so far processed, ACK rule unchanged. Watching the Checks step 4: run the poller, request nothing. Replace "Re-Requesting a Review" with "A Review a Maintainer Requested": only a maintainer requests a review; an agent never triggers one; a reviewer that reviews by itself after a push is processed like any other; the Copilot timing facts stay as facts about a manual request. Generalize the login note (a reviewer's review and its inline comments may come from different logins). Round limit counts rounds from every reviewer. Do not add the deadline or the report (#2977). |
| T4  | DONE   | Review-bot check runs | `watch-pull-request-checks` and the exit condition count only check runs from a workflow (non-empty `workflow` in `gh pr checks --json`). Verify how `gh pr checks --watch` behaves with a pending non-workflow check and document the command that waits only for workflow checks. |
| T5  | DONE   | `merge-pull-request` Merge Readiness | Replace "Copilot review" with a reviewer-neutral "Reviews" check: every review received so far is addressed and no actionable thread is unresolved; no named reviewer's review is required; checks as in T4. |
| T6  | TODO   | Audit `Reviewer` column | `process-pr-review` step 2 records the login; `PR-REVIEW-TEMPLATE.md` Status Values and Findings table use `Reviewer`; `validate-audit-record.py` (if it reads the column) and the `agent-review-report-contract` pins accept `Reviewer` for new records and `Author class` for existing ones. Tests of the contract check cover both. |
| T7  | TODO   | Reviewer-neutral wording elsewhere | `AGENTS.md` (After opening a PR: no "re-request the review"), `open-pull-request` After Opening, `pr-review-handler.md`, `.claude/skills/README.md`, `docs/agents/orchestration.md` (reviewer wording only; #2904 T9 rewrites the file later). Contract pins on these files updated in the same commit. |
| T8  | TODO   | Search for leftovers | `git grep -i copilot` outside the out-of-scope paths returns only the manual-request description, the coding-agent mentions and historical records. Record the command and result in the evidence. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | The ADR | Commit after `linter markdown` and `linter cspell`. |
| T2 | Poller default and its tests, plus the options table row | Commit after the poller's tests pass and the test-design review is recorded. |
| T3, T4 | `wait-for-pr-review` and `watch-pull-request-checks` text | One commit per skill after `linter all`. |
| T5 | `merge-pull-request` Merge Readiness | Commit after `linter all`. |
| T6 | Audit column: skill, template, validator, contract check and its tests | One commit: the contract pins and the texts they pin must change together. |
| T7 | Wording in the remaining files and their pins | Commit after `linter all` and the contract check pass. |
| T8 | Evidence only | Commit with the evidence file. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, use the `write-unit-test` skill and complete an explicit design review after
each passing test increment, before maintainer review and commit. Confirm that the test exposes the
one causal initial-state difference; its fixture owns only incidental mechanics; and the production
Act plus independently specified expected result remain visible. The review must use the mandatory
prose-first Arrange-Act-Assert comparison: write temporary prose for each section, refactor until
the code expresses it, remove redundant prose, and retain only irreducible context. Record this
review in task evidence or a file-local test plan. Assess helper boundaries by meaningful named
actions and abstraction-level alignment, not caller count. Use a Conventional Commit message with
the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-reviewer-neutral-pr-review/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
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

- 2026-11-24 17:30 UTC - Claude Code (Claude Opus 5.5) - Draft created from the maintainer's six
  answers of 2026-11-24 and the reviewer data read from PRs #3001 to #3032.
- 2026-11-24 17:36 UTC - Claude Code (Claude Opus 5.5) - The maintainer approved the draft. Created
  issue #3037 and moved the specification to `docs/issues/open/` on the spec-only branch
  `3037-reviewer-neutral-pr-review-spec`.
- 2026-11-24 18:27 UTC - Claude Code (Claude Opus 5.5) - Spec PR #3039 merged (maintainer ACK on
  `389c2d287`). Started the implementation on branch `3037-reviewer-neutral-pr-review`. T1 done:
  `docs/adrs/20261124182709_review_pull_requests_without_a_mandatory_reviewer.md`.

## Acceptance Criteria

- [ ] AC1: No skill, tool default or document outside the Out of Scope paths makes a review from a
  named reviewer a condition for a pull request to be ready or mergeable.
- [ ] AC2: Run without `--reviewer`, the poller reports a review or comment from any account except
  the pull request's author; with `--reviewer`, only the listed logins.
- [ ] AC3: The skills say that an AI agent never triggers a review, that only a maintainer requests
  one, and what the agent does when a requested review (Copilot included) or a self-started review
  arrives: process it with `process-pr-review`.
- [ ] AC4: Readiness in `wait-for-pr-review` and `merge-pull-request` ignores check runs that do not
  come from a workflow.
- [ ] AC5: New audit records carry a `Reviewer` column with the login; the contract check and
  `validate-audit-record.py` accept it, and still accept existing records with `Author class`.
- [ ] AC6: The ADR records decisions 2 to 5 and the rejected alternatives.
- [ ] AC7: The text this issue leaves contains neither the 60-minute deadline nor the hand-over
  report, which stay with #2977.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass (`wait-for-pr-review` tool, `agent-review-report-contract`)
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p wait-for-pr-review` and the tests of `agent-review-report-contract`
- The `agent-review-report-contract` check on the whole tree
- `validate-audit-record.py` on one existing audit record (legacy column) and on the new record of
  the implementation PR (new column)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | The poller wakes on a reviewer it was not told about | Run the poller without `--reviewer` on a pull request with a CodeRabbit review, with `--since` before that review | A `review` event with `coderabbitai[bot]`; the author's own replies produce no event | TODO | `manual-verification-evidence.md` section V1 |
| M2 | `--reviewer` still narrows | Same pull request with `--reviewer da2ce7` and a short timeout | No event for `coderabbitai[bot]`; `timeout` | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Readiness ignores a review-bot check run | Run the documented checks command on a pull request with a `CodeRabbit` check run | Only workflow check runs are counted | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Old and new audit records validate | Run the contract check and `validate-audit-record.py` on an existing record and on the implementation PR's record | Both pass | TODO | `manual-verification-evidence.md` section V4 |
| M5 | A fresh agent follows the new skills | Give a fresh agent session the updated `wait-for-pr-review` and a pull request with a CodeRabbit review and no Copilot review; ask whether the pull request is ready and what it would do next | It processes the CodeRabbit findings, requests no review, and does not wait for Copilot | TODO | `manual-verification-evidence.md` section V5 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain of every command result that depends on one.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | T8 search |
| AC2 | TODO | T2 tests, M1, M2 |
| AC3 | TODO | T3 diff, M5 |
| AC4 | TODO | T4 diff, M3 |
| AC5 | TODO | T6 tests, M4 |
| AC6 | TODO | T1 ADR |
| AC7 | TODO | T3 diff |

## Risks and Trade-offs

- **A pull request can reach the maintainer with no AI review at all.** Accepted by the maintainer;
  he reviews closely himself.
- **More wake-ups.** Watching every account except the author also wakes the poller on empty
  thread-reply reviews and on comments from any account. Each costs one look; the agent records
  nothing when there is nothing to process.
- **A late review reopens a pull request that looked ready.** Expected with reviewers that answer on
  their own schedule; the skill keeps watching until merge.
- **The `workflow` field is the only signal for a review-bot check run.** If GitHub stops reporting
  it, T4's command counts the bot check again; M3 records the observed output.
- **#2977 starts from different text than its description assumes.** Mitigated by this issue going
  first and its own description being narrowed.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2003, #2977, #2976, #2981, #2904, #2949
- Reviewer data: PRs #3001 to #3032
