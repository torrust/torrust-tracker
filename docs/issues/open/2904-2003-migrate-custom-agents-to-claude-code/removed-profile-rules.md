---
semantic-links:
  related-artifacts:
    - .github/agents/copilot-suggestions-handler.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/implementer.agent.md
    - .github/agents/committer.agent.md
    - .github/agents/planner.agent.md
    - .github/agents/pr-reviewer.agent.md
    - .github/agents/researcher.agent.md
    - .claude/agents/pr-review-handler.md
    - .claude/agents/task-reviewer.md
    - .claude/agents/implementer.md
    - .claude/agents/pr-reviewer.md
    - .claude/agents/researcher.md
    - .claude/skills/process-pr-review/SKILL.md
    - .claude/skills/review-task/SKILL.md
    - .claude/skills/commit-changes/SKILL.md
    - .claude/skills/implement-task/SKILL.md
    - .claude/skills/review-pr/SKILL.md
    - .claude/skills/research-external-evidence/SKILL.md
    - .claude/skills/create-issue/SKILL.md
---

# Rules of the Removed Copilot Profiles

The inventory of acceptance criterion AC6 of
[issue #2904](ISSUE.md#acceptance-criteria). It lists every rule of each Copilot profile in
`.github/agents/`, and where the rule lives once the profile is removed: a named skill; the agent
file, only for a rule about the role itself (its identity, tools, inputs or report format); or
nowhere, with the reason it is dropped. A rule whose destination already held it before this issue
says so.

Task T4 created this file with the rows of `copilot-suggestions-handler`. Each of T5 to T9 adds the
rows of the profiles its agent replaces, and T10 completes it before it removes the profiles. Task T6
adds `implementer`, `committer` and `planner`, task T7 adds `pr-reviewer`, and task T8 adds `researcher`.

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

## `implementer`

Replaced by the agent `implementer` (task T6), which uses the model of the session (decision D13) and
starts no agent (decision D3). No skill owned the sequence of an implementation, so task T6 adds the skill `implement-task`,
which holds the seven steps and links the skills that own each practice; the agent preloads it.
The handoffs to `complexity-auditor`, `task-reviewer` and `committer` are dropped: the orchestrator
starts the reviewer (task T9), and the implementer commits itself (decision D14).

| Rule of the profile | Destination | Notes |
| --- | --- | --- |
| Identity: implement tasks correctly, simply and verifiably, with test-driven development whenever practical and the simplest solution that passes. | Skill `implement-task` and the agent file (identity) | |
| Follow Beck's four rules of simple design, in priority order. | `AGENTS.md`, "Development Principles" (already held it) | The agent file points to `AGENTS.md`. |
| Follow `AGENTS.md` for repository-wide conventions. | `AGENTS.md`; agent file, "Workflow" | |
| The pre-commit command, `--format=json` first and `--format=text --verbosity=verbose` for diagnostics. | `AGENTS.md`, "Build, Test, and Lint", and skills `run-pre-commit-checks` and `commit-changes` (already held it) | |
| Skills to load when needed: `add-rust-dependency`, `fix-bug`, `write-unit-test`, `handle-errors-in-code`, `commit-changes`. | Agent file, `skills` field; skill `implement-task`, steps 1, 3, 4 and 7 | `implement-task`, `commit-changes` (decision D14) and `write-unit-test` are preloaded; the other three are read when the work needs them, so the agent does not start with their whole text. |
| Follow the verification types: durable automatic checks in Rust, manual verification against the finished artifact, a disposable script only with the rationale the template requires. | Skill `implement-task`, step 5; skill `create-issue`, "Plan The Work" | The rationale is required of the specification by `create-issue` (already held it). |
| ADR discoverability: link the ADR to the affected code and add module comments that link back. | Skill `create-adr` (already held it) | |
| Step 1: read `AGENTS.md`, the skills of the area and the whole task; state what must change and what must not. | Skill `implement-task`, step 1 | |
| Step 1: decide whether the work is substantively a bug, whatever its labels, and then follow `fix-bug` in order. | Skill `implement-task`, step 1; skill `fix-bug` (already held it) | |
| Step 1: ask rather than guess; raise a specification that is ambiguous, incomplete or out of line with the code with the Planner or the user. | Skill `implement-task`, step 1; agent file ("Inputs" and "Workflow") | There is no Planner and the implementer cannot ask the contributor: it returns the question in its reply with the verdict `QUESTION`, and the caller answers (decision D3). |
| Step 2: break each task into the smallest independent, verifiable steps, each fit for one commit. | Skill `implement-task`, step 2 | The agent follows the Commit Points of the specification. |
| Step 3: write a failing test, minimal code, refactor; run `cargo test -p <package>`; when TDD is not practical, implement defensively and add tests as the next step. | Skill `implement-task`, step 3; skill `write-unit-test` | |
| Step 3.5: check the standard library and the workspace before a new crate, start from the latest stable version, document an older choice. | `AGENTS.md`, "Engineering Policies" 1, and skill `add-rust-dependency` (already held it) | |
| Step 3.5: check base images when touching container artifacts; document the Rust API and the invariants; keep shell for orchestration. | `AGENTS.md`, "Engineering Policies" 2, 3 and 5 (already held it); skill `implement-task`, step 4 | |
| Step 4: invoke the Complexity Auditor after every step and do not go on while it reports a blocking issue. | Dropped | Decision D12: no agent audits code; the length and nesting items are checked once, by the task reviewer. |
| Step 5: compare the result with the specification; decide whether `implementation-retrospective.md` is needed; otherwise log why not; send a lesson that applies beyond the issue to the skill, agent, template or document it concerns. | Skill `implement-task`, step 6; skill `create-issue`, "Verification Requirements for Issue Specs" item 5 (already held it) | |
| Step 5: do not create retrospectives for routine work with no material discovery; keep them evidence-based and blameless, and not a substitute for verifying the criteria. | `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` (already held it) | Skill `implement-task`, step 6, keeps only the condition for creating one. |
| Step 5: a retrospective needs a folder-style specification; migrate a touched legacy single-file specification first, and `ISSUE.md` and `EPIC.md` are the allowed primary files. | Skill `review-task`, workflow step 7, and skill `create-issue`, "Migrating Legacy Specifications" (already held it) | |
| Step 5: review the collaborators, the resource ownership, the deadlines and the passive infrastructure of fixtures with child processes, asynchronous I/O or network readiness. | Skill `create-issue`, the design-review checkpoint it requires of such specifications (already held it) | |
| Step 5: perform the manual scenarios against the finished artifact before the independent verification and record the commands, output, logs and conclusions in `manual-verification-evidence.md`; for a bug, the first reproduction and the final recheck; test output is not a substitute. | Skill `implement-task`, step 5; skills `create-issue` and `fix-bug` (already held it) | |
| Step 6: invoke the Task Reviewer with the specification path, the criteria, a summary of what changed per criterion, and a request to tick the verified items; wait for its report. | Dropped from the implementer | The orchestrator starts the task reviewer (decision D3, task T9) and gives it these inputs; the implementer reports what it changed in its reply. |
| Step 6: include the reviewer's report and checkbox changes in the change set for the Committer; a failed report may be committed as durable evidence. | Skill `review-task`, "Recording The Review" (already held it) | The caller commits the reply file with the work it describes. |
| Step 6: when the reviewer reports gaps, address them and ask for review again. | Dropped from the implementer | The orchestrator loops, with the round limit of 3 (decision D18, task T9). |
| Step 7: after the reviewer approves, invoke the Committer; never commit directly. | Dropped | Decision D14: there is no `committer` agent; the implementer commits under the skill `commit-changes`. |
| Do not do more than was asked: scope creep is a defect. | Skill `implement-task`, "Boundaries" | |
| Do not suppress a warning or a Clippy lint without a documented reason. | `AGENTS.md`, "Code Style" (already held it) | |
| Do not add a dependency without running `cargo machete`. | `AGENTS.md`, "Essential Rules" 4 (already held it) | |
| Do not add a dependency without checking the latest stable version. | `AGENTS.md`, "Engineering Policies" 1 (already held it) | |
| Do not commit code that fails the pre-commit script. | `AGENTS.md`, "Essential Rules" 1; skill `implement-task`, "Boundaries" | |
| Do not skip the audit step, even for a small change. | Dropped | Decision D12. |
| Do not self-verify the acceptance criteria and do not tick them in the specification. | Agent file, "Workflow" (last paragraph); skill `implement-task`, "Boundaries" | The task reviewer verifies and ticks (skill `review-task`, step 5). |
| Do not leave meaningful behavior untested without documenting the reason. | `AGENTS.md`, "Engineering Policies" 4; skill `implement-task`, step 5 | |
| Tools: `execute`, `read`, `search`, `edit`, `todo`, `agent`. | Agent file (tools) | `Bash`, `Read`, `Edit`, `Write`, `Grep`, `Glob`. `agent` is dropped because the implementer starts no agent (decision D3); `todo` has no use in a run that the caller supervises through its reply. |
| `argument-hint` and `user-invocable: true`, `disable-model-invocation: false`; `semantic-links` to `fix-bug`. | Agent file (inputs, `description`) | The hint becomes the "Inputs" section. The visibility keys have no Claude Code meaning. The link to `fix-bug` is step 1 of the workflow. |

## `committer`

Not ported as an agent (decision D14): the implementer commits itself, with the skill `commit-changes`
preloaded. The skill already held most of the workflow; task T6 moves into it the three rules that
no other document held (the specification's progress, the branch's issue number, the dictionary
first when one change is split) and two smaller ones (accidental files, and separate commits for skill and workflow documents).

| Rule of the profile | Destination | Notes |
| --- | --- | --- |
| Identity: a commit specialist that treats every request as a review-and-verify workflow. | Dropped | Decision D14: a gate in front of every commit costs one agent run and two trace entries per commit. |
| Follow `AGENTS.md` and the skill `commit-changes`. | Skill `commit-changes` (already held it) | |
| The pre-commit command, with the JSON format first. | Skill `commit-changes`, "Automated Checks" (already held it) | |
| Create GPG-signed Conventional Commits, and on a GPG timeout stop, notify the user and never use `--no-gpg-sign`. | Skill `commit-changes`, "GPG Commit Signing" and "GPG Timeout Handling"; `AGENTS.md`, "Essential Rules" 2 (already held it) | |
| Workflow 1: check that the specification's progress is current, and that a completion commit has its manual verification evidence; stop when the specification is stale. | Skill `commit-changes`, "Checks Before The Commit" 1 | The first of the three rules moved by task T6. The rule says that only criteria a reviewer verified are ticked, because the implementer commits before the review and never ticks a criterion. |
| Workflow 2: check that the branch's issue number has an open specification; otherwise look in `closed/`, then confirm or rename with `chore/`. | Skill `commit-changes`, "Checks Before The Commit" 2; skill `run-pre-commit-checks` now points to it | The second rule. `run-pre-commit-checks` pointed to "the committer agent spec". |
| Workflow 3 to 5: read the branch, the status and the diff; summarize the scope; keep it coherent and free of unrelated changes. | Skill `commit-changes`, "Manual Checks" (self-review the diff) and "Commit Quality Guidelines" (atomic) (already held it) | |
| Workflow 6: stop on a policy violation in the diff and return it to the implementer or the reviewer. | Skill `commit-changes`, "Manual Checks"; skill `review-task` (already held it) | The implementer is the author; the task reviewer catches what it misses. |
| Workflow 7: check whether the pre-commit hook is installed before running the script by hand, so the checks do not run twice. | Skill `run-pre-commit-checks`, "Git Hook (Recommended Setup)" (already held it) | `run-pre-push-checks` and `push-changes` say the same. The skill `commit-changes` links to it. |
| Workflow 7: may fix formatting, linting, spelling and import order that come from the commit's scope; must not fix build or test failures, which go back to the implementer. | Dropped | The implementer is the author of the code, so a failure is its own to fix; there is no one to hand it back to. |
| Workflow 8 to 10: propose a precise message, commit with `git commit -S` once the scope is clear, verify the result afterwards. | Skill `commit-changes`, "Conventional Commit Format" and "One Signed Commit per Step" (already held it) | |
| Do not write code. | Dropped | The role is gone. |
| Do not bypass failing checks without telling the user what failed. | Skill `commit-changes`, "Automated Checks" (never skip the hook) and "GPG Timeout Handling" (stop and notify) (already held it) | Only the signing part tells the user what failed. For the other checks the agent file has the implementer report the failing commands in its reply and final message. |
| Do not rewrite or revert unrelated changes of the user. | Dropped | The implementer commits only the changes of its task; `AGENTS.md`, "Commit history", forbids rewriting a branch. |
| Do not create empty, vague or non-conventional messages. | Skill `commit-changes`, "Commits to Avoid" (already held it) | |
| Do not commit secrets, backup files or accidental files. | Skill `commit-changes`, "Commits to Avoid" | Moved by task T6. |
| Do not mix skill or workflow documentation changes with implementation changes: separate commits. | Skill `commit-changes`, "Commits to Avoid" | Moved by task T6. |
| Splitting commits: commit the `project-words.txt` changes first, as one `chore(cspell)` commit or folded into the first logical commit. | Skill `commit-changes`, "Checks Before The Commit" 3 | The third rule. |
| Output format: scope summary, blockers, checks, proposed message, status, post-commit verification. | Dropped | There is no agent to report; the implementer's reply and final message list the commits. |
| Tools: `execute`, `read`, `search`, `todo`; `argument-hint`; `user-invocable` and `disable-model-invocation`. | Dropped | There is no agent. |

## `planner`

Not ported as an agent: the session that plans works under the skill `create-issue`, which already held
the specification rules. Task T6 adds its section "Plan The Work" with the rules that only the
profile held, and the orchestrator (task T9) takes the delegation.

| Rule of the profile | Destination | Notes |
| --- | --- | --- |
| Identity: transform ambiguous work into clear, actionable and verifiable plans, and do not implement. | Dropped | There is no planner agent; the session drafts the specification under `create-issue`, and the implementer implements. |
| Follow `AGENTS.md`; use the issue specifications under `docs/issues/`. | Skill `create-issue` (already held it) | |
| Responsibility 1: write or refine specifications, simple issues and EPICs. | Skill `create-issue` (already held it) | |
| Responsibility 2 and workflow 4: classify the issue as `task`, `bug` or `feature`, with one sentence of reason. | Skill `create-issue`, "Plan The Work" 1 | Moved by task T6. |
| Responsibility 3 and workflow 5: choose the strategy by risk and coupling (parallel streams, progressive implementation, a spike first) and say why. | Skill `create-issue`, "Plan The Work" 2 | Moved by task T6. |
| Responsibility 4 and workflow 6 and 7: break the work into coarse tasks of about a day, each with a deliverable, and define for each its intent, output, verification and dependencies. | Skill `create-issue`, "Plan The Work" 3 | Moved by task T6. |
| Responsibility 5 and workflow 8: delegate the tasks to the implementer in an order, with a precise scope and success criteria. | Dropped | The orchestrator starts the implementer and gives it the scope (decision D3, task T9). |
| Workflow 1 and 2: clarify the objective, constraints and success criteria; inspect the repository and the existing specifications. | Skill `create-issue`, steps 1 and 2 (draft, then the user reviews) | |
| Ensure the plans are aligned with the repository's quality standards and workflow expectations. | `AGENTS.md`, "Engineering Policies" and "Collaboration Principles"; skill `create-issue` (already held it) | |
| Workflow 3: the specification holds the problem, the scope, the acceptance criteria, and the risks and assumptions. | The issue template; skill `create-issue`, "Plan The Work" 4 | Moved by task T6 for the risks and assumptions. |
| Workflow 3: a responsibility and ownership map and a design review after the first vertical slice, when child processes, asynchronous I/O, network readiness or cleanup are involved. | Skill `create-issue` (already held it) | |
| Workflow 3: manual verification scenarios with their evidence in `manual-verification-evidence.md`; a disposable script needs a rationale, a path, an owner and, for Python, a reason. | Skill `create-issue` (already held it) | |
| Use folder-style specifications for new work, `ISSUE.md` or `EPIC.md` as the primary file, and migrate a legacy single-file specification when it is materially updated. | Skill `create-issue`, step 1 (already held it) | |
| Require an evidence-based completion review, with an `implementation-retrospective.md` or the reason it is not needed. | Skill `create-issue`, "Verification Requirements for Issue Specs" item 5 (already held it) | |
| Follow the verification types. | Skill `create-issue`, "Plan The Work" 5 | Moved by task T6. |
| Output format: classification, planning summary, strategy, task breakdown, delegation plan, open questions and risks. | Dropped | The drafted specification is the output; there is no delegation plan to print. |
| Do not implement production code while planning. | Dropped | There is no planner role to restrict. |
| Do not leave acceptance criteria ambiguous, do not decompose into vague units, and do not bypass the conventions. | Skill `create-issue`, "Plan The Work"; `AGENTS.md` | Moved by task T6 for the first two. |
| Do not delegate without an explicit scope and success criteria. | Dropped | The orchestrator briefs the implementer (task T9). |
| Answer the implementer's clarifying questions promptly and update the specification before implementation resumes. | Skill `create-issue`, "Plan The Work" (last paragraph) | Moved by task T6. The orchestrator relays the question to the contributor (decision D3). |
| Tools: `execute`, `read`, `search`, `edit`, `todo`, `agent`; `argument-hint`; `user-invocable` and `disable-model-invocation`. | Dropped | There is no agent. |

## `pr-reviewer`

Replaced by the agent `pr-reviewer` (task T7), pinned to the full model ID of Opus 5.5 (decision
D13), which starts no agent (decision D3) and posts nothing on GitHub. The skill `review-pr` already
held the checklist and the findings format; T7 moves into it the rules that only the profile held
(the output and its verdicts, the merged pull request, the recording of a review).

| Rule of the profile | Destination | Notes |
| --- | --- | --- |
| Identity: review an already-open pull request and give merge-focused feedback. | Agent file (identity) and skill `review-pr` (already held it) | |
| Follow `AGENTS.md`. | `AGENTS.md`; agent file, "Workflow" | |
| Use the `review-pr` skill as the checklist source. | Agent file, `skills` field (preloaded) | |
| Review against the actual diff and the CI context, not local intent. | Skill `review-pr`, "Quick Overview Approach" step 3 | Moved by task T7. |
| Workflow 1: confirm that a pull request exists; a number or a URL is required. | Skill `review-pr`, "Quick Overview Approach" step 1; agent file, "Inputs" | A missing number or URL is returned as a question. |
| Workflow 2: gather the title, the description, the linked issue, the base branch, the merge state and time, and the checks. | Skill `review-pr`, "Quick Overview Approach" step 2 | Moved by task T7; the skill already read the title, the description and the diff. |
| Workflow 3: review the changed files and classify the findings by severity. | Skill `review-pr`, "Quick Overview Approach" step 5 and "Providing Feedback" (already held it) | The skill's finding format has five severities; the output of the profile had three, and the Output now refers to the skill's. |
| Workflow 4: verify the tests and documentation expectations of the checklist. | Skill `review-pr`, "PR Review Checklist" (already held it) | |
| Workflow 5: return a clear merge-readiness verdict. | Skill `review-pr`, "Output" | Moved by task T7. |
| A merged pull request: label the result `POST_MERGE_FINDINGS`, give no readiness verdict, implement nothing, create no branch, and direct the caller to the post-merge workflow of `process-pr-review`, which needs the maintainer's approval. | Skill `review-pr`, "A Merged Pull Request" | Moved by task T7. |
| Persist the independent review in `agent-review-reports.md` of a folder-style specification, before returning the verdict, including a `POST_MERGE_FINDINGS` result. | Skill `review-pr`, "A Reply File" | Decision D5 and D18: the record is now a JSON reply file, `pr-reviewer-pr<number>-r<round>.json`, that the validator checks. |
| Read the whole report before editing; keep earlier entries unchanged and in order; a correction is a new timestamped entry. | Skill `review-pr`, "A Branch Without The `pr-reviewer` Agent", which links the procedure of `review-task` | A reply file is one file per round and is never edited in place. |
| Record the pull-request number, the base branch, the reviewed files, the checks, the findings by severity, the checklist gaps, the verdict and the follow-up owner and action. | Skill `review-pr`, "A Reply File" (the fields) and "A Branch Without The `pr-reviewer` Agent" | |
| The record is independent of the Copilot review-thread handling and does not replace the handler's tracker. | Skill `review-pr`, "A Reply File" | Now: independent of the handling of the review threads, and of the audit record of `process-pr-review`. |
| Do not invoke the Committer or commit the report; the caller requests the Committer for the change set or a focused documentation commit. | Agent file, "Workflow" | Decision D14: there is no `committer` agent; the caller commits the reply file with the work it describes. |
| With no folder-style specification (a diff or package review, a legacy standalone specification), create no report and state `Issue-local report skipped: no folder-style issue specification was supplied.` | Skill `review-pr`, "No Folder-Style Specification" | |
| Output format: the scope, the findings by severity, the checklist gaps, the overall verdict (`APPROVE`, `REQUEST_CHANGES`, `COMMENT`, `POST_MERGE_FINDINGS`). | Skill `review-pr`, "Output"; agent file, "Final message" | The reply file maps the verdicts to the closed list of the schema. |
| Do not run the pre-pull-request task acceptance review. | Skill `review-pr`, "Recording The Review" | Moved by task T7; the skill already redirected a branch without a pull request to `review-task`. |
| Do not mark the issue specification's workflow checkpoints unless asked and evidenced. | Skill `review-pr`, "Recording The Review" | Moved by task T7. |
| Do not approve while a blocker is unresolved. | Skill `review-pr`, "Output" | Moved by task T7. |
| Tools: `execute`, `read`, `search`, `edit`, `todo`, `agent`. | Agent file (tools) | `Bash`, `Read`, `Write`, `Grep`, `Glob`. `Write` is for the reply file; `Edit` is dropped because the reviewer changes no file of the pull request; `agent` is dropped (decision D3); `todo` has no use in one review. |
| `argument-hint`; `user-invocable: true` and `disable-model-invocation: false`. | Agent file ("Inputs", `description`) | The hint becomes the "Inputs" section. The visibility keys have no Claude Code meaning. |

## `researcher`

Replaced by the agent `researcher` (task T8), which uses the model of the session (decision D13) and
starts no agent (decision D3). No skill held the research workflow ("Still happening? Ad hoc,
without the "facts, not decisions" rule", in the specification), so task T8 adds the skill
`research-external-evidence`, which holds the workflow, the report and the boundaries; the agent
preloads it, as the implementer preloads `implement-task`.

| Rule of the profile | Destination | Notes |
| --- | --- | --- |
| Identity: research external projects, source code, issue trackers and documentation to answer specific questions with concrete evidence; gather facts, make no implementation decisions and write no production code. | Agent file (identity) and skill `research-external-evidence`, introduction | |
| Follow `AGENTS.md` for repository-wide conventions. | `AGENTS.md`; agent file, "Workflow" | |
| Description: not for codebase-internal exploration, use the Explore subagent for that. | Agent file (`description`) and skill `research-external-evidence`, introduction | Kept, with the pointer to the Explore subagent worded for Claude Code (the skill is provider-neutral and says the repository's own code is searched in the repository). |
| Report findings in a format the Planner or the Implementer can directly incorporate. | Skill `research-external-evidence`, "Report"; agent file, "Reply file" | There is no Planner or Implementer agent to address: the report is for the caller, and the reply file carries it when the research belongs to an issue. |
| Clone external repositories into a temporary directory outside the workspace, such as `/tmp/tracker-research/`, or into the git-ignored `.tmp/` when `/tmp` is not available or the caller prefers it. | Skill `research-external-evidence`, "Workflow" step 3 | |
| Responsibility 1: clone external trackers (opentracker, chihaya and others) and search their source for patterns, behaviors or options. | Skill `research-external-evidence`, "Workflow" step 3 | |
| Responsibility 2: search external GitHub repositories for issues, pull requests and discussions with the search tools, or with `gh issue list` and `gh search issues` when they are not available. | Skill `research-external-evidence`, "Workflow" step 3 | |
| Responsibility 3: read external documentation (BEPs, wiki pages, READMEs) to verify claims. | Skill `research-external-evidence`, "Workflow" step 3 | |
| Responsibility 4: compare implementations across trackers and identify the de-facto standard. | Skill `research-external-evidence`, "Workflow" step 4 and "Report" item 4 | |
| Responsibility 5: return structured, evidence-backed findings with source references (paths, line numbers, issue URLs, commit hashes). | Skill `research-external-evidence`, "Report" item 3 | |
| Research domains: how other trackers handle a BEP requirement, the de-facto standard of a behavior, whether a feature exists in opentracker or chihaya, which options other trackers expose, known issues and discussions. | Skill `research-external-evidence`, introduction | The examples are kept in one sentence of the introduction. |
| Workflow 1: clarify the question, what evidence is needed and from which sources. | Skill `research-external-evidence`, "Workflow" step 1 | The agent cannot ask the contributor; it returns the question in its reply with the verdict `QUESTION`. |
| Workflow 2: plan the investigation (repositories to clone, queries, documentation). | Skill `research-external-evidence`, "Workflow" step 2 | |
| Workflow 3: gather evidence (a shallow clone with `grep`, `find` and `git log`; the search tools or `gh`; fetch and read documentation). | Skill `research-external-evidence`, "Workflow" step 3 | |
| Workflow 4: cross-reference the evidence and note the agreements and disagreements. | Skill `research-external-evidence`, "Workflow" step 4 | |
| Workflow 5 and output format: restate the question, list the sources, give the findings with paths, lines and URLs, the cross-project comparison, the conclusion with a confidence level, and the open questions. | Skill `research-external-evidence`, "Workflow" step 5 and "Report" | |
| Do not modify any file of the workspace: a read-only role. | Skill `research-external-evidence`, "Boundaries" | Now: no file may be created or changed, except the reply file and the clones in the git-ignored `.tmp/`, which the old rule contradicted by asking for clones there. |
| Do not make implementation recommendations: report facts, not decisions. | Skill `research-external-evidence`, "Boundaries" | |
| Do not clone repositories inside the workspace. | Skill `research-external-evidence`, "Workflow" step 3 | |
| Do not guess or assume behavior: every claim rests on evidence found in the session. | Skill `research-external-evidence`, "Boundaries" | |
| Do not spend time on tangents. | Skill `research-external-evidence`, "Boundaries" | |
| Clean up cloned repositories after reporting, unless the caller needs them. | Skill `research-external-evidence`, "Workflow" step 5 | The agent's report ends its run, so the clones are deleted before the report. |
| When the source code is ambiguous, say so instead of over-interpreting it. | Skill `research-external-evidence`, "Workflow" step 4 | |
| Prefer shallow clones (`--depth 1`). | Skill `research-external-evidence`, "Workflow" step 3 and "Boundaries" | |
| Tools: `execute`, `read`, `search`, `todo`. | Agent file (tools) | `Bash`, `Read`, `Write`, `Grep`, `Glob`, `WebFetch` and `WebSearch`. `Write` is for the reply file only; the web tools are what the workflow needs to read documentation; `todo` has no use in one research question. |
| `argument-hint`; `user-invocable: true` and `disable-model-invocation: false`. | Agent file ("Inputs", `description`) | The hint becomes the "Inputs" section. The visibility keys have no Claude Code meaning. |
