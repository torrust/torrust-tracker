---
semantic-links:
  related-artifacts:
    - .github/agents/copilot-suggestions-handler.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/complexity-auditor.agent.md
    - .claude/agents/pr-review-handler.md
    - .claude/agents/task-reviewer.md
    - .claude/skills/process-pr-review/SKILL.md
    - .claude/skills/review-task/SKILL.md
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

## `task-reviewer`

Replaced by the agent `task-reviewer` (task T5), pinned to the full model ID of Opus 5.5 (decision
D13), with the stop hook of decision D17. The skill `review-task` already held the checklist; T5
moves into it the rules that only the profile held (the definitions of the results, the manual
evidence, and the recording of a review), and the complexity item of decision D12.

| Rule of the profile | Destination | Notes |
| --- | --- | --- |
| It is the repository's task reviewer: it verifies that implemented work is complete before the branch is pushed and a pull request is opened. | Agent file `.claude/agents/task-reviewer.md` (identity) | |
| Follow `AGENTS.md` for repository-wide standards. | `AGENTS.md` (already held it) | A project agent loads `AGENTS.md` through the root `CLAUDE.md` (decision D11, scenario M4). |
| Use the issue specification as the source of truth for the acceptance criteria. | Skill `review-task`, workflow step 1 (already held it) | |
| Apply repository conventions consistently: tests, lint readiness, scope discipline, naming. | Skill `review-task`, the checklists (already held it) | |
| Goal 1: verify each criterion with evidence from code, tests and observable behaviour; never approve on intent alone. | Skill `review-task`, workflow steps 2 and 3 and the "Verification" checklist (already held it) | |
| Goal 2: identify pending tasks, regressions, and mismatches between the requested scope and the implementation. | Skill `review-task`, the "Scope And Criteria" checklist (already held it) | |
| Goal 3: detect convention problems that would block a clean commit. | Skill `review-task`, workflow step 3 (`linter all` at least) (already held it) | |
| Goal 4 and workflow step 5: update the specification to mark only truly verified criteria as done. | Skill `review-task`, workflow step 5 and the "Spec Hygiene" checklist (already held it) | |
| Goal 5 and workflow step 6: verify that the completion review was performed; require an issue-local `implementation-retrospective.md` for reusable lessons, material design changes or meaningful deviations, otherwise a progress-log entry saying why none was needed; require a folder-style specification first, with `ISSUE.md` and `EPIC.md` as the allowed primary files. | Skill `review-task`, workflow steps 6 and 7 and the "Spec Hygiene" checklist (already held it) | |
| Workflow step 1: identify the review inputs: the specification path, the criteria and the claimed scope. | Agent file (inputs) and skill `review-task`, the "Scope And Criteria" checklist | |
| Workflow step 2: inspect the diffs and files, and run focused checks. | Skill `review-task`, workflow step 3 (already held it) | |
| Workflow step 3: apply the `Test Design` checklist to every changed test; report each violated item with a remediation; do not pass while a fixture is a parameter bag or hides the causal state, the Act or the expected result; require the recorded prose-first Arrange-Act-Assert comparison; judge a helper by its meaning and abstraction level, not by its caller count. | Skill `review-task`, the "Test Design" checklist (already held it) | The profile's pointer to `.github/skills/dev/task-reviews/review-task/SKILL.md` is a path that now holds a redirect stub; the skill is preloaded. |
| Workflow step 4: classify each criterion as `PASS` (implemented and verified), `FAIL` (not implemented or incorrect) or `PENDING` (partial, unclear, or without evidence). | Skill `review-task`, workflow step 4 | The skill named the three words and not their meaning; T5 adds it. |
| Workflow step 7: confirm that the mandatory manual scenarios ran against the finished artifact and are recorded in `manual-verification-evidence.md` with the actual commands or interactions, output, logs and conclusions; automated or disposable-script output is not manual evidence. | Skill `review-task`, workflow step 8 and the "Verification" checklist | Only the profile held it; T5 adds it. |
| Workflow step 8: report findings with concrete remediation for every `FAIL` or `PENDING` item. | Skill `review-task`, workflow step 9 and the reply file's `findings` | |
| Workflow step 9: the overall status is `REVIEW PASSED` when every required criterion passes and no blocking issue remains, and `REVIEW FAILED` when one fails or one remains. | Skill `review-task`, "Output" | Only the profile held it; T5 adds it, changed: the profile passed the review when every required criterion passed; the skill passes it when no required criterion fails, and lets a criterion left `PENDING` for evidence from a later task not fail it (any other `PENDING` required criterion gives `REVIEW FAILED`). In a reply file it is the verdict `REVIEW_PASSED` or `REVIEW_FAILED`. |
| Persist the independent review in `agent-review-reports.md` of a folder-style specification: create it from the template when absent, otherwise read it whole and append one complete entry; keep earlier entries unchanged and in order, with a correction as a new entry; record the matrix, findings, completion-review finding, specification updates, result and remediation; update the verified checkboxes first. | Skill `review-task`, "Recording The Review": the reply file (decisions D17 and D18) for a task that names one, and `agent-review-reports.md` on a branch without the agent | From T5 a review of a folder-style specification is a reply file, not an entry (decision D18). |
| Do not commit the report, and do not ask the Committer to include it; the caller commits it with the reviewed change set or on its own, and a failed report may be committed as durable evidence. | Agent file (the caller commits the reply file) and skill `review-task`, "Recording The Review" | There is no `committer` agent (decision D14); the reply file is committed with the work it describes (decision D18). |
| When no folder-style specification is supplied (a diff or package review, a legacy standalone specification), create, migrate or modify no report, and state `Issue-local report skipped: no folder-style issue specification was supplied.` | Skill `review-task`, "No Folder-Style Specification" | The agent writes a reply file only when the task names one (decision D17). |
| Output format: the scope reviewed, the criteria matrix, the repository-convention findings, the completion-review finding, the specification updates and the overall result. | Skill `review-task`, "Output" (already held it) | The agent's final message adds the reply file path and the questions. |
| Do not review a pull request: this is pre-pull-request validation only. | Skill `review-task`, "Not In Scope" (already held it), and the agent's `description` | |
| Do not implement feature code while reviewing. | Agent file (identity) | |
| Do not mark criteria as done unless they were explicitly verified. | Skill `review-task`, workflow step 5 (already held it) | |
| Do not ask the Committer to proceed when the result is `REVIEW FAILED`. | Dropped | Decision D14: there is no `committer` agent. The caller reads the verdict. |
| Tools: `execute`, `read`, `search`, `edit`, `todo`, `agent`. | Agent file (tools) | `Bash`, `Read`, `Edit`, `Write`, `Grep`, `Glob`. `agent` is dropped because the reviewer starts no agent (decision D3); `todo` has no use in one review. `Write` is for the reply file. |
| `argument-hint`: the specification path, the criteria, the scope, and whether to update the checkboxes. | Agent file (inputs) | The checkboxes are always updated, for verified criteria only (skill `review-task`, step 5). |
| `user-invocable: true` and `disable-model-invocation: false`. | Dropped | Copilot keys with no Claude Code meaning; Claude Code starts the agent from its `description`, or when the user names it with `@agent-task-reviewer`. |

## `complexity-auditor`

Not ported as an agent (decision D12): its checks become one item of the `review-task` checklist,
checked once before the pull request, and a follow-up issue evaluates Clippy's `too_many_lines` and
`excessive_nesting` lints as the deterministic replacement. Task T5 makes the change.

| Rule of the profile | Destination | Notes |
| --- | --- | --- |
| Audit code quality: cyclomatic complexity, cognitive complexity, nesting depth and function length of the changed code. | Skill `review-task`, "Function Length And Nesting", and Clippy | Nesting and length are the checklist item. |
| Cyclomatic complexity: count the paths by hand; 1 to 10 is fine, 11 to 15 warns, 16 and more fails. | Dropped | Decision D12 keeps only length and nesting in the checklist; Clippy's `cognitive_complexity = "deny"` already fails the build on a function that is too complex. |
| Cognitive complexity: run `cargo clippy` with the `cognitive_complexity` lint; any warning fails. | `Cargo.toml` `[workspace.lints.clippy]` (already held it) | The lint is `deny` for the whole workspace, so `linter all` and the pre-commit hook already run it. |
| Nesting depth: flag a function nested more than 3 levels. | Skill `review-task`, "Function Length And Nesting" | |
| Function length: flag a function longer than 50 lines. | Skill `review-task`, "Function Length And Nesting" | |
| Focus on the diff of the current task; report pre-existing issues only when they sit next to changed code and add risk. | Skill `review-task`, "Function Length And Nesting" | |
| Do not penalise an idiomatic `match` that is the primary control flow of a function. | Skill `review-task`, "Function Length And Nesting" | |
| Run after each implementation step of the Implementer, and report `PASS`, `WARN` or `FAIL` per function with the lines `AUDIT PASSED`, `AUDIT WARNED` or `AUDIT FAILED`. | Dropped | Decision D12: the check runs once before the pull request, as part of `REVIEW PASSED` or `REVIEW FAILED`, and the checklist reports violations as findings. |
| Naming, imports, documentation and other convention checks belong to the Reviewer, not to the auditor. | Dropped | There is one reviewer for the task, and `review-task` holds every check. |
| Persist the audit in `agent-review-reports.md`, with the same append-only rules and the skipped-report statement. | Dropped | The audit is an item of the task review, which is recorded as `review-task` says. |
| Do not rewrite code, do not alter the reviewed implementation, do not invoke the Committer; keep the report to one line per function. | Dropped | No agent audits code any more; the reviewer reports findings with remediation and changes no feature code (agent file), and there is no `committer` agent (decision D14). |
| Tools: `execute`, `read`, `search`, `edit`; `user-invocable: true` and `disable-model-invocation: false`. | Dropped | There is no agent; the keys have no Claude Code meaning. |
