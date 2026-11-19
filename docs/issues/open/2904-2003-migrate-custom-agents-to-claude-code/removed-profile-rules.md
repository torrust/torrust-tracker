---
semantic-links:
  related-artifacts:
    - .github/agents/copilot-suggestions-handler.agent.md
    - .claude/agents/pr-review-handler.md
    - .claude/skills/process-pr-review/SKILL.md
---

# Rules of the Removed Copilot Profiles

The inventory of acceptance criterion AC6 of
[issue #2904](ISSUE.md#acceptance-criteria). It lists every rule of each Copilot profile in
`.github/agents/`, and where the rule lives once the profile is removed: a named skill; the agent
file, only for a rule about the role itself (its identity, tools, inputs or report format); or
nowhere, with the reason it is dropped. A rule whose destination already held it before this issue
says so.

Task T4 created this file with the rows of `copilot-suggestions-handler`. Each of T5 to T9 adds the
rows of the profiles its agent replaces, and T10 completes it before it removes the profiles.

## `copilot-suggestions-handler`

Replaced by the agent `pr-review-handler` (task T4), which handles every reviewer, not only Copilot
(proposal P1 of the specification).

| Rule of the profile | Destination | Notes |
| --- | --- | --- |
| It is the entry point that processes Copilot-authored review findings. | Agent file `.claude/agents/pr-review-handler.md` (identity) | Widened to every reviewer: Copilot, a person or another bot. |
| Process the findings through `process-pr-review`, and create or update `docs/pr-reviews/pr-<number>-review/PR-REVIEW.md` as it requires. | Skill `process-pr-review` (already held it) | The agent preloads the skill with its `skills` field. |
| The skill alone defines normalization, dispositions, current-tree verification, commit-subject citation, replies, resolution order and completion checks; keep no parallel Copilot-only procedure. | Skill `process-pr-review` (already held it: "Process all authors through this workflow") | The agent file repeats only that the skill is the single source. |
| Follow `AGENTS.md`. | `AGENTS.md` (already held it) | `pr-review-handler`, a project agent, loads `AGENTS.md` through the root `CLAUDE.md` (decision D11), as scenario M4 showed for a started agent. |
| Use the **Committer** agent for all GPG-signed commits. | Dropped | Decision D14: there is no `committer` agent. The handler commits itself under `AGENTS.md` and the preloaded `commit-changes` skill. |
| On a merged pull request, do only the read-only post-merge triage, then stop for the maintainer's approval; a pre-merge request does not carry across the merge. | Skill `process-pr-review`, section "Reviews Submitted After Merge" (already held it) | The agent file adds that the handler returns the triage's proposal, because it cannot ask the contributor. |
| On a pull request closed without merging and replaced by another, fix and audit on the replacement, and reply on the closed one. | Skill `process-pr-review`, section "Reviews on a Closed, Superseded Pull Request" (already held it) | |
| Tools: `execute`, `read`, `search`, `edit`, `todo`, `agent`. | Agent file (tools) | `Bash`, `Read`, `Edit`, `Write`, `Grep`, `Glob`. `agent` is dropped because the handler starts no agent (decision D3); `todo` has no use in one round. |
| Input: the pull-request number, and optionally an existing audit record. | Agent file (inputs) | The inputs are the pull request, the poller's event and the commands that act as the contributor; the audit record has one fixed path, so it is not an input. |
| `user-invocable: true` and `disable-model-invocation: false`. | Dropped | Copilot keys with no Claude Code meaning. Claude Code starts the agent from its `description`, or when the user names it with `@agent-pr-review-handler`. |
