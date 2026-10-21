---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2003
github-issue: 2581
spec-path: docs/issues/open/2581-2003-pr-review-wait-poller/ISSUE.md
branch: "2581-pr-review-wait-poller"
related-pr: null
last-updated-utc: "2026-10-21 07:48"
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

### Out of Scope

- Writing the Rust poller, its tests, or any skill edits. These belong to the implementation issue.
- Merging a PR. The agent prints the merge command for the maintainer to run.
- Replacing `fetch-review-threads` or `resolve-review-threads`.
- Polling through an MCP server.
- Using the poller to approve PRs or to post `ACK` comments.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
  (provider-specific adapters must stay optional and portable).
- ADRs to create: `None known`. Revisit if the placement decision below becomes an
  architectural choice.

## Design and Ownership Review

Proposed contract (to be confirmed before implementation):

- **Transport**: `gh api` only. The tool holds no token. It refuses to run without `gh`
  authentication.
- **Start baseline**: on start, the tool records the current head SHA and the latest
  review and comment IDs of each watched reviewer. It returns only on events that happen
  after that baseline.
- **Events that return**:
  - a new review (any state) from a watched reviewer;
  - a new PR comment (issue comment or inline review comment) from a watched reviewer;
  - a change of the PR head SHA (a push by the agent or the contributor);
  - the PR is closed or merged.
- **Timeout**: after the configured wait, exit with a timeout status. The default is 15 minutes.
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
- **Output**: one JSON object on stdout describing the event. Diagnostic text goes to stderr.
- **Exit codes**: distinct codes for event, timeout, and error. Exact values are set during implementation.
- **Non-interactive**: the tool refuses to run when stdout is a TTY, as `github-review-threads` does.
  Its output is for agents. Manual runs must redirect stdout to a file or pipe it (for example
  `| jq .`), so the manual verification steps never attach stdout to a terminal.
- **Deterministic**: the tool makes no model calls and uses no MCP server.

## Implementation Plan

Not started. This issue is spec-only. The implementation issue will hold the plan.

## Commit Points

Not applicable to this spec PR. The implementation issue will define the commit points.

## Acceptance Criteria

- [ ] The spec has been reviewed and approved by the maintainer before implementation starts.
- [ ] The decisions recorded below are reflected in the spec.
- [ ] The implementation issue specifies the exit contract, the configuration, and the reviewer set.

## Verification Policy

Automatic checks for the implementation issue:

- `linter all`
- Unit tests for the tool's event detection, with `gh` responses faked at the boundary.
- `cargo test` for the tool's crate, plus the pre-push checks when the tool is added to the workspace.

Manual verification for the implementation issue:

- Run the poller against the PR that adds this feature. Confirm it returns on a real `da2ce7`
  review and on a Copilot review.
- Run the poller with a short timeout on a PR with no activity. Confirm the timeout exit code.
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
   `.github/skills/dev/pr-reviews/<tool-name>/`. The implementation issue must decide its
   workspace membership under `add-workspace-member`.
4. **Parent EPIC.** Linked to EPIC #2003 (`Overhaul: Automation Tools and AI Agent Guardrails`).
   The folder uses the `{issue}-{epic}-` prefix (`2581-2003-`), following the `create-issue`
   convention: the GitHub issue number replaces the draft prefix and the EPIC number follows it.
5. **Issue type.** `task`.

Defaults to confirm during implementation (not blocking this spec):

- **Watched reviewers.** `da2ce7` and `copilot-pull-request-reviewer[bot]` by default, with a
  per-run override.
- **Re-request mechanism.** The re-request step uses
  `POST repos/{owner}/{repo}/pulls/{n}/requested_reviewers`. The Copilot re-review path is not
  verified and must be checked before implementation.

## Progress Tracking

### Workflow Checkpoints

- [x] Prior-art pollers located in other workspaces `.tmp/` folders
- [x] Related skills searched (`create-issue`, `process-pr-review`, `fetch-review-threads`,
  `merge-pull-request`)
- [x] Reviewer review states checked on closed PRs
- [x] Draft approved by maintainer
- [x] Draft passes `linter markdown` and `linter cspell`
- [ ] GitHub issue created and spec PR opened

### Progress Log

- 2026-10-20: Draft created from the investigation above. The maintainer answered the open
  questions: Copilot exit condition, 2-minute default interval, skill-folder location,
  EPIC #2003 link, and task type. Entry written by the AI assistant (Copilot SDK in VS Code).
- 2026-10-21: Fixed the exit condition after Copilot finding `WAIT-001` on merged PR #2584
  (`review-finding:pr-2584-wait-001`): unresolved threads are now counted over the whole PR.
  Aligned the end state and the ACK requirement with the `merge-pull-request` skill's Merge
  Readiness section (the ACK is recommended and can be waived). Entry written by the AI assistant
  (Copilot SDK in VS Code).
