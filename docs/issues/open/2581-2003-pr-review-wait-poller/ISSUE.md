---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 2003
github-issue: 2581
spec-path: docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md
branch: "2581-pr-review-wait-poller"
related-pr: null
last-updated-utc: "2026-10-28 13:00"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
    - fetch-review-threads
    - merge-pull-request
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
    - .github/skills/dev/git-workflow/merge-pull-request/SKILL.md
    - contrib/dev-tools/github/github-review-threads/Cargo.toml
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/research/20260716-console-shutdown-patterns/README.md
---

<!-- skill-link: create-issue -->

# Issue #2581 - Deterministic Wait-for-Review Poller for AI Agents

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Give AI agents a deterministic, token-free way to wait for PR reviewers, so that an agent
processes each round of feedback without the contributor having to tell it to wait.

## Background

After a contributor opens a pull request, the agent must wait for the review. The current
workflow depends on the contributor telling the agent to wait, and that is repeated for
every review round. The maintainers' flow is:

1. Copilot reviews automatically when the PR is opened or updated.
2. The contributor asks a maintainer (currently `da2ce7`, Cameron Garnham) to review.
3. The agent processes findings, commits, pushes, comments, and re-requests review from
   the same reviewer. This can repeat for several rounds per reviewer.
4. The end state meets the merge-readiness checks in the `merge-pull-request` skill: the latest
   Copilot review is on the head commit with no unresolved actionable thread, and the other
   maintainer has commented `ACK <commit>` on the head commit (or the maintainer has recorded a
   waiver). The agent then prints the maintainer merge command.

Polling through an MCP server spends inference tokens on each check. Earlier agents in
other workspaces wrote ad-hoc bash pollers in `.tmp/` (for example
`torrust-tracker-agent-08/.tmp/poll-pr-reply.sh` and
`torrust-tracker-agent-04/.tmp/wait-for-review.sh`). They hardcode the repository, build
jq filters by string interpolation, and compare timestamps as strings. This issue replaces
that pattern with one maintained, tested tool that lives with the skill that uses it.

Observed in the repository (2026-10-20): Copilot's reviews appear as
`copilot-pull-request-reviewer[bot]` with state `COMMENTED` in all 53 closed PRs sampled.
Copilot never submitted an `APPROVED` review in that sample. `da2ce7` submits `APPROVED`,
`COMMENTED`, and `CHANGES_REQUESTED`.

## Scope

### In Scope

- Specify a Rust command-line poller that agents run to block until a PR event happens.
- Specify its configuration: wait timeout (default 15 minutes), poll interval, PR selector,
  and the reviewer set.
- Specify its exit contract: which events return, what it prints, and which exit codes it uses.
- Specify the review-loop contract that agents follow after the poller returns.
- Specify how the poller authenticates: it uses the `gh` CLI as the only transport.
- Decide the location of the tool inside the skill tree.
- Define verification: automated tests and a manual end-to-end run on the PR that adds this feature.
- Implement the poller crate and its tests, the `wait-for-pr-review` skill, the link from
  `process-pr-review`, and the poller's row in the CLI output contract ADR.

### Out of Scope

- Merging a PR. The agent prints the merge command for the maintainer to run.
- Replacing `fetch-review-threads` or `resolve-review-threads`.
- Polling through an MCP server.
- Using the poller to approve PRs, to post `ACK` comments, or to re-request reviews.
- Deciding whether the CLI output contract ADR covers AI-harness tools, or whether exit codes
  should be one failure code plus a `kind` rather than distinct numbers. A follow-up issue
  tracks that policy question.

## Architectural Decisions

- Related ADRs:
  - `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
    (provider-specific adapters must stay optional and portable).
  - `docs/adrs/20260519000000_define_global_cli_output_contract.md` (output channels, exit codes
    `0`/`1`/`2`, TTY refusal, NDJSON diagnostics). The poller is a `stdout-result-data` binary.
- Related research: `docs/research/20260716-console-shutdown-patterns/README.md` (SIGINT and
  SIGTERM handling).
- ADRs to create: `None known`. The ADR scope and exit-code policy question goes to a follow-up
  issue.

## Design and Ownership Review

Contract, confirmed by the maintainer on 2026-10-28:

- **Transport**: `gh api` only. The tool holds no token. It refuses to run without `gh`
  authentication.
- **Start baseline**: on start, the tool records the current head SHA and the IDs of the
  submitted reviews and the latest comment IDs. It returns only on events that happen after that
  baseline. A review keeps the ID it got as a draft, so reviews are tracked as a set of submitted
  IDs, not as a maximum, and a review submitted later with a lower ID still counts. The optional
  `--since-head-sha <sha>` moves the baseline to the commit date of that commit, so a review that
  lands between the agent's push and the poller start is not missed; the commit may be abbreviated
  or in uppercase, and the baseline uses the full lowercase SHA that GitHub returns. The optional
  `--since <utc time>` sets the baseline to a moment and cannot be combined with `--since-head-sha`.
  Without either, the baseline is the poller start.
- **Resumable cursor**: a `review` or `comment` result carries `cursor`, the GitHub time of the
  reported item. `--since <cursor>` counts only items created strictly after it, so the review loop
  can continue after an event that needed no push without the event being reported again and
  without losing what came after it.
- **Watched reviewers**: `da2ce7` and `copilot-pull-request-reviewer[bot]` by default. The
  repeatable `--reviewer <login>` flag overrides the set. Events from logins outside the set,
  including the agent's own, never count.
- **Events that return**:
  - a new review (any state) from a watched reviewer;
  - a new PR comment (issue comment or inline review comment) from a watched reviewer;
  - a change of the PR head SHA (a push by the agent or the contributor);
  - the PR is closed or merged.
- **Timeout**: after the configured wait, the poller reports a timeout result. The default is
  15 minutes. The wait is a hard bound: no request starts once the budget is spent and a `gh` call
  still running at that moment is killed. After the baseline was taken that ends the wait with a
  `timeout` result. Before it, nothing was observed, so it is a `deadline_exceeded` failure (exit
  `1`), because a `timeout` would wrongly suggest a quiet pull request.
- **Poll interval**: configurable per run. The default is 2 minutes (120 seconds).
- **Exit condition for the review loop**: all of the following hold. Copilot never submits
  `APPROVED`, so its last review is `COMMENTED`.
  - Copilot's latest review is on the current head commit.
  - The PR has no unresolved, actionable review thread anywhere, including threads opened on
    earlier commits. Count threads over the whole PR, not only those on the latest commit, so
    unresolved feedback from an earlier round cannot be overlooked. Fetch them with
    `github-review-threads`, as `process-pr-review` does.
  - A review-body finding with no thread is not counted by the thread check. The review-loop
    skill reads each new review body through the audit workflow instead.
  - The maintainer `ACK <commit>` comment on the head commit is present, or the maintainer has
    recorded a waiver, as the `merge-pull-request` skill's Merge Readiness section describes.

  The poller returns events; the review-loop skill checks this exit condition after each return.
- **Output (ADR class `stdout-result-data`)**: every normal outcome is exactly one JSON object
  on stdout followed by a newline, with a `kind` field: `review`, `comment`, `head_changed`,
  `pr_closed`, `pr_merged`, or `timeout`. A timeout is a result, like a tracker endpoint
  timeout in the ADR, so agents branch on `kind` and need no exit-code table. On failure stdout
  is empty. Every stderr record is NDJSON with a `kind` field; no plain text.
- **Exit codes**: the ADR baseline only. `0` for any result, including `timeout`. `1` for a
  runtime failure (`gh` missing or unauthenticated, API error, interruption). `2` for a usage
  error or TTY refusal.
- **Signals**: the poller handles `SIGINT` and `SIGTERM`, as the shutdown research recommends. On
  either it stops any in-flight `gh` child, writes an `interrupted` record to stderr, leaves
  stdout empty, and exits `1`.
- **Non-interactive**: the tool refuses to run when stdout is a TTY, as `github-review-threads` does
  (exit `2`, `tty_refusal` record). Its output is for agents. Manual runs must redirect stdout to
  a file or pipe it (for example `| jq .`), so the manual verification steps never attach stdout
  to a terminal.
- **Deterministic**: the tool makes no model calls and uses no MCP server.

## Implementation Plan

1. Add the crate `wait-for-pr-review` under `.github/skills/dev/pr-reviews/wait-for-pr-review/tool/`.
   Register it as a workspace member, not a default member, under `add-workspace-member`.
2. Implement event detection behind a `gh` trait so tests fake `gh` responses at the boundary.
3. Implement the CLI: flags, TTY refusal, JSON result and diagnostic output, signal handling.
4. Add the `wait-for-pr-review` skill (`SKILL.md`) with the poller usage and the review-loop
   contract, and link it from `process-pr-review`.
5. Add the poller's row to the CLI output contract ADR table.
6. Run the manual end-to-end verification on this PR and record it in
   `manual-verification-evidence.md`. Verify the Copilot re-review path and document the result.

## Commit Points

1. Spec update with the maintainer decisions.
2. Crate scaffold and workspace registration.
3. Event detection with tests.
4. CLI, output contract, and signal handling with tests.
5. Skill, `process-pr-review` link, and ADR table row.
6. Manual verification evidence and spec progress.

## Acceptance Criteria

- [x] The spec has been reviewed and approved by the maintainer before implementation starts.
- [x] The decisions recorded below are reflected in the spec.
- [x] The poller returns on each event kind and on timeout, following the exit contract above.
- [x] The poller refuses to run on a TTY and handles `SIGINT` and `SIGTERM`.
- [x] The `wait-for-pr-review` skill documents the usage and the review loop.
- [x] The ADR table lists the poller.
- [ ] The manual verification is recorded and the Copilot re-review path is documented.
- [x] `linter all` and the tool's tests pass.

## Verification Policy

Automatic checks for the implementation issue:

- `linter all`
- Unit tests for the tool's event detection, with `gh` responses faked at the boundary.
- `cargo test` for the tool's crate, plus the pre-push checks when the tool is added to the workspace.

Manual verification for the implementation issue:

- Run the poller against the PR that adds this feature. Confirm it returns on a real `da2ce7`
  review and on a Copilot review.
- Run the poller with a short timeout on a PR with no activity. Confirm the `timeout` result and exit `0`.
- Run the poller with stdout on a terminal, and send it `SIGINT` and `SIGTERM`. Confirm the
  `tty_refusal` (exit `2`) and `interrupted` (exit `1`) records.
- Verify the Copilot re-review request path on this PR and document the result in the skill.
- Record commands and output in issue-local `manual-verification-evidence.md`.

Post-implementation: review the acceptance criteria against the observed behavior before closing.
Record the implementation completion review in the progress log.

## Decisions

Decided by the maintainer (2026-10-20):

1. **Copilot approval.** Copilot never submits `APPROVED`. Copilot counts as approved when its
   latest review is on the head commit and the PR has no unresolved actionable thread (see
   Design, exit condition). Amended 2026-10-21 after review finding `WAIT-001` on PR #2584:
   the check covers the whole PR, not only the latest commit.
2. **Poll interval.** Default 2 minutes, configurable per run.
3. **Location.** The tool is a self-contained Rust crate inside the skill folder that uses it:
   `.github/skills/dev/pr-reviews/<tool-name>/`. Workspace membership is decided below.
4. **Parent EPIC.** Linked to EPIC #2003 (`Overhaul: Automation Tools and AI Agent Guardrails`).
   The folder uses the `{issue}-{epic}-` prefix (`2581-2003-`), following the `create-issue`
   convention: the GitHub issue number replaces the draft prefix and the EPIC number follows it.
5. **Issue type.** `task`.

Decided by the maintainer (2026-10-28):

- **Scope.** #2581 implements the poller. There is no separate implementation issue.
- **Layout.** A new skill `wait-for-pr-review` holds the crate in a `tool/` subfolder. The crate
   is a workspace member and not a default member, like the other developer tools.
- **ADR conformance.** The poller follows the CLI output contract ADR as written. A timeout is a
   result (`kind: timeout`, exit `0`), not a failure and not a dedicated exit code. Rationale:
   agents are the consumers, so one JSON `kind` is easier to branch on than a table of exit
   numbers, and the ADR already treats expected outcomes as results.
- **ADR table.** This PR adds the poller's row (`stdout-result-data`) to the ADR's binary table.
- **Follow-up issue.** The question of the ADR's scope (tracker only, or also AI-harness tools)
    and of its exit-code policy (one failure code plus `kind`, versus distinct numbers per
    command) goes to a separate issue.
- **Signals.** The poller handles `SIGINT` and `SIGTERM`: it stops the in-flight `gh` child,
    writes an `interrupted` record to stderr, and exits `1` (see the shutdown research).
- **Baseline.** The optional `--since-head-sha` flag sets the baseline to the commit date of that
    SHA. The default baseline is the poller start.
- **Watched reviewers.** `da2ce7` and `copilot-pull-request-reviewer[bot]` by default, with the
    repeatable `--reviewer` override. The poller ignores every other login, including the agent's.
- **Re-review.** The poller never re-requests a review. The agent does it through
    `POST repos/{owner}/{repo}/pulls/{n}/requested_reviewers`. The Copilot re-review path is not
    verified; it is verified on this PR and the result is documented in the skill.

Amended by the maintainer-approved review round on PR #2683 (2026-10-28), after Copilot's review
found four defects in the first implementation. The decisions above are unchanged; these refine
them:

- **Review IDs.** The baseline tracks the set of submitted review IDs, not a maximum, because a
  review keeps its draft ID (`review-finding:pr-2683-f1`).
- **Canonical SHA.** `--since-head-sha` accepts an abbreviated or uppercase SHA, and the poller
  compares the full lowercase SHA that GitHub returns (`review-finding:pr-2683-f2`).
- **Hard deadline.** The timeout is a hard bound on every `gh` call, and the poller starts no
  request after the budget is spent (`review-finding:pr-2683-f3`).
- **Cursor.** Results carry a `cursor` and `--since` resumes after it, so the review loop does not
  reprocess a handled event (`review-finding:pr-2683-f4`).

## Progress Tracking

### Workflow Checkpoints

- [x] Prior-art pollers located in other workspaces `.tmp/` folders
- [x] Related skills searched (`create-issue`, `process-pr-review`, `fetch-review-threads`,
  `merge-pull-request`)
- [x] Reviewer review states checked on closed PRs
- [x] Draft approved by maintainer
- [x] Draft passes `linter markdown` and `linter cspell`
- [x] GitHub issue created and spec PR opened
- [x] Maintainer decisions of 2026-10-28 recorded in the spec
- [x] Implementation complete
- [ ] Implementation PR opened

### Progress Log

- 2026-10-20: Draft created from the investigation above. The maintainer answered the open
  questions: Copilot exit condition, 2-minute default interval, skill-folder location,
  EPIC #2003 link, and task type. Entry written by the AI assistant (Copilot SDK in VS Code).
- 2026-10-21: Fixed the exit condition after Copilot finding `WAIT-001` on merged PR #2584
  (`review-finding:pr-2584-wait-001`): unresolved threads are now counted over the whole PR.
  Aligned the end state and the ACK requirement with the `merge-pull-request` skill's Merge
  Readiness section (the ACK is recommended and can be waived). Entry written by the AI assistant
  (Copilot SDK in VS Code).
- 2026-10-28: The maintainer decided to implement the poller in #2581 and answered the open
  questions: layout, baseline flag, reviewer set, signal handling, and the Copilot re-review
  check. After reading the CLI output contract ADR and the shutdown research, the maintainer chose
  to follow the ADR exit codes (`0`/`1`/`2`) with timeout as a `kind: timeout` result, to add the
  poller to the ADR table, and to track the ADR scope and exit-code policy in a follow-up issue.
  Entry written by the AI assistant (Copilot SDK in VS Code).
- 2026-10-28: Implemented the poller, the `wait-for-pr-review` skill, the link from
  `process-pr-review`, and the ADR table row. Manual verification V1 to V5 ran against the real
  GitHub API (merged PR, a real Copilot review, timeout, API failure, TTY refusal, usage error,
  `SIGTERM`, `SIGINT`); V6 (a real `da2ce7` review) and V7 (Copilot re-review path) wait for this
  feature's pull request. Entry written by the AI assistant (Copilot SDK in VS Code).
