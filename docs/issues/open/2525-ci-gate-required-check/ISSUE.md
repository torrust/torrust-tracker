---
schema-version: 1
doc-type: issue
issue-type: feature
status: planned
priority: p2
epic: null
github-issue: 2525
spec-path: docs/issues/open/2525-ci-gate-required-check/ISSUE.md
branch: "2525-ci-gate-required-check"
related-pr: null
last-updated-utc: "2026-10-22 07:07"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
    - implement-workflow
    - add-workspace-member
  related-artifacts:
    - docs/github-rulesets.md
    - docs/issues/open/2525-ci-gate-required-check/manual-verification-evidence.md
    - docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md
    - .github/workflows/testing.yaml
    - .github/workflows/os-compatibility.yaml
    - .github/workflows/container.yaml
    - .github/workflows/generate_coverage_pr.yaml
    - .github/workflows/docs-lint.yaml
    - contrib/dev-tools/checks/package-coverage-check/Cargo.toml
    - "issue #2507"
    - "issue #1743"
---

<!-- skill-link: create-issue -->

# Issue #2525 - Require one always-reporting CI gate decided by a Rust tool

## Goal

Make GitHub enforce that a pull request's CI passed before it can merge, with one required check,
`CI gate`, that reports on every pull request (code and documentation-only alike), can be
overridden per pull request by a maintainer without disabling any ruleset, and whose decisions live
in a Rust tool behind thin workflows that do not lock the project into GitHub Actions.

## Background

Issue #2507 found that the only required-checks ruleset (94237) had been effectively disabled since
2026-05-05 and deleted it. It could not be satisfied: job renames had removed six of its 16 required
check names, and since the documentation-only fast path (#1743) a Markdown-only pull request reports
only `Docs Lint`, so the other required checks stayed "expected" forever. The maintainer disabled it
around every merge, and it stayed disabled, taking the force-push ban with it.

A fork sandbox in #2507 (evidence section T5 of that issue) established the constraints:

- GitHub enforces required status checks on the maintainer merge tool's direct merge push, reading
  them from the pull-request head commit. A GitHub-required check therefore protects every merge,
  whichever tool performs it.
- A workflow skipped by `paths-ignore` reports nothing, and a missing required check blocks the merge
  forever. Every required check must report on every pull request.
- A later commit status overrides an earlier commit status with the same context, but not a failing
  GitHub Actions check run with the same name. An override of an Actions-produced check needs a
  different mechanism.

The maintainer decided (2026-10-14) on one required, always-reporting `CI gate`, overridable by a
maintainer-applied `ci-override` label, with the decision logic in Rust: "put as little code as
possible in GitHub workflows". Keeping CI decisions in a Rust tool makes them testable, lets them
grow smarter about what to run for a given change (performance), and decouples the project from
GitHub so that moving to another CI provider or self-hosted CI stays feasible. Rising GitHub
infrastructure cost and lower reliability since AI agents multiplied pull requests make that
portability more valuable.

## Scope

### In Scope

- A Rust workspace tool (working name `ci-gate`) with provider-neutral inputs and outputs:
  - **change detection:** given the base and head revisions, classify the changed paths and print
    which job groups the change needs (for example documentation, Rust build and tests, container,
    databases, security scan), replacing workflow-level `paths-ignore` and `paths` filters;
  - **verdict:** given the needed job groups, each group's result, and an optional override with
    its actor, decide pass or fail and print a human-readable explanation of every failing or
    missing group.
- One orchestrating pull-request workflow that runs change detection, invokes the existing
  workflows as reusable workflows only for needed groups, and ends with a `CI gate` job
  (`if: always()`) that runs the verdict. Workflow YAML only passes data to and from the tool.
- The `ci-override` label: the orchestrator re-evaluates the gate when the label is added or
  removed, without rerunning the heavy jobs; the verdict passes with an explanation naming the
  actor; only actors with the Maintain or Admin role count.
- A ruleset requiring the `CI gate` context from GitHub Actions with
  `strict_required_status_checks_policy: false`, applied after the gate is proven, and
  `docs/github-rulesets.md` updated.
- A root ADR recording the direction: CI decision logic in Rust behind thin, provider-agnostic
  workflows.
- Skill and documentation updates: `merge-pull-request` (override procedure), `implement-workflow`
  (thin-workflow rule), and `docs/github-rulesets.md`.

### Out of Scope

- Migrating away from GitHub Actions or to self-hosted CI.
- Changing what each test job runs, or the self-hosted container runner.
- Rewriting `github-merge.py` in Rust.
- Post-merge `push` workflows on `develop` and `main`, except where a reusable-workflow conversion
  requires keeping their triggers working unchanged.

## Architectural Decisions

- Related ADRs:
  [`20261013161419_stop_requiring_pull_request_rebases.md`](../../../adrs/20261013161419_stop_requiring_pull_request_rebases.md)
  (required checks must not require up-to-date branches),
  [`20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`](../../../adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md).
- ADRs to create: root ADR "Keep CI decision logic in Rust behind thin, provider-agnostic
  workflows" (T1), written as
  [`20261016154414_keep_ci_decision_logic_in_rust_behind_thin_provider_agnostic_workflows.md`](../../../adrs/20261016154414_keep_ci_decision_logic_in_rust_behind_thin_provider_agnostic_workflows.md).

## Design and Ownership Review

Not applicable to tracker runtime code. The Rust tool reads Git and JSON inputs and writes text; it
owns no child processes, network readiness, or long-lived resources.

## Bug-Fix Process

Not applicable. This is a feature.

## Regression Test Strategy

Not applicable. The Rust tool gets unit tests for classification and verdict rules (T2, T3).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status      | Task                                            | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| --- | ----------- | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE        | Write the CI-portability ADR                    | Root ADR under `docs/adrs/`: decision logic in Rust tools with provider-neutral inputs (paths, revisions, JSON results); workflows only schedule jobs and pass data; how to judge whether new logic belongs in YAML.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| T2  | DONE        | Rust change detection                           | Implemented as the `ci-gate` crate (`contrib/dev-tools/checks/ci-gate`, subcommand `classify`) on branch `2525-ci-gate-change-detection`, approved by the maintainer on 2026-10-20 and committed as `feat(ci-gate): [#2525] add the change-detection tool`. Decisions of 2026-10-20: one crate for T2 and T3; one group per workflow (`docs-lint`, `testing`, `os-compatibility`, `coverage`, `container`, `db-compatibility`, `db-benchmarking`, `security-scan`); changed paths on stdin plus `--base-branch`, so the tool runs no child process and reads no git state; the rules are a typed Rust table that reproduces the filters of the eight pull-request workflows; T3 follows in its own pull request. Output follows the global CLI output contract (one JSON object on stdout, JSON diagnostics on stderr, TTY refusal). 104 unit and 6 command-line tests; 16 of 16 mutants killed; 60 of 60 recent merged pull requests classified exactly as GitHub ran them. Evidence: [`manual-verification-evidence.md`](manual-verification-evidence.md) V1 to V6. |
| T3  | DONE        | Rust verdict                                    | Implemented as the `verdict` subcommand of the `ci-gate` crate on branch `2525-ci-gate-verdict`; decisions approved by the maintainer on 2026-10-20: one JSON object on stdin, `{"classification": <the classify output>, "results": {<group>: success, failure, cancelled, or skipped}}`; the override is `--override-actor <login>` (the Maintain or Admin role check of AC3 belongs to T5); a decided verdict, passing or failing, is a result, so the tool exits with 0 and the workflow reads `verdict`, while input it cannot decide on (unreadable, or a classification that does not list every group once) exits with 1 and prints nothing, so both fail the gate. A needed group that failed, was cancelled, was skipped, or never reported is a problem; a group that is not needed never counts; an override is recorded only when it overrides a problem. 135 unit and 10 command-line tests, 13 of 13 mutants killed. Evidence: [`manual-verification-evidence.md`](manual-verification-evidence.md) V7.                                                |
| T4  | DONE        | Decision gate: gate cost and override           | In a fork sandbox like #2507's: measure the runner time the gate adds to a documentation-only PR (tool compile included, with and without caching or a prebuilt binary), and verify that a later successful `CI gate` check run from an override-only run supersedes the earlier failing one for the required check. Sandbox runs DONE on 2026-10-21 ([`manual-verification-evidence.md`](manual-verification-evidence.md) V8 and V9): a cold build of the tool costs a median 0.40 runner minutes per job and a prebuilt binary about 0.12; a later check run supersedes an earlier one in both directions for the same job of a workflow, and two workflows with a job named `CI gate` do not combine into a pass. Approved by the maintainer on 2026-10-21: both gate jobs build the tool cold, and a prebuilt binary is adopted only if a later measurement exceeds the AC6 bound, which is at most 1.0 runner minute added at the median to a documentation-only pushed head (two tool runs). |
| T5  | DONE        | Decision gate: cross-run state for the override | A `labeled` or `unlabeled` event starts a new workflow run that cannot read the earlier run's `needs` results, and later events do not say who applied an existing label. Define, with a maintainer-approved design: (a) where the gate reads the heavy jobs' results for the head SHA (for example the checks API, persisted artifacts, or rerunning the jobs); (b) where it reads label provenance (for example the issue events API) and how it verifies the actor's Maintain or Admin role; (c) the token permissions this needs on fork pull requests, whose `GITHUB_TOKEN` is read-only, and the trust boundary of each input; (d) whether an override survives a new push (`synchronize`) or applies only to the head SHA it was granted for. Prove it with hosted runs in a sandbox for label added, label removed, and a push after an override. Keep provider queries in a thin adapter so the verdict stays provider-neutral. Sandbox runs DONE on 2026-10-21 ([`manual-verification-evidence.md`](manual-verification-evidence.md) V10); the design is in "T5 Design Proposal: the Override Across Runs", approved by the maintainer on 2026-10-21 as proposed. |
| T6  | IN_PROGRESS | Orchestrating workflow                          | Convert the pull-request triggers of the heavy workflows into `workflow_call` invocations from one orchestrator; keep `push` triggers; final `CI gate` job; `labeled`/`unlabeled` handling that only re-evaluates the verdict. Follow `implement-workflow`. `CI gate` lands reporting but **not required**: no ruleset change in T6. Before T7, observe it on at least one real code PR and one real documentation-only PR (M1, M2 without the required rule) and record both runs. Split on 2026-10-21 into three steps, each its own pull request: T6a, the Rust adapter that fetches the raw GitHub API responses and turns them into the verdict input, with the contract for the group job names, the stale-override check, and the role check, and no workflow change; T6b, one workflow converted behind the orchestrator, tried in the fork sandbox first; T6c, the other workflows, one commit each. T6a is implemented as the `github-verdict` subcommand of the `ci-gate` crate (`contrib/dev-tools/checks/ci-gate/src/github.rs` and `gh.rs`), on branch `2525-ci-gate-github-adapter`: [`manual-verification-evidence.md`](manual-verification-evidence.md) V11. |
| T7  | TODO        | Ruleset and documentation                       | Only after T6 is observed on real PRs, and only with explicit maintainer approval: add the required `CI gate` context (Actions app, not strict) to the rulesets. Save the ruleset JSON before and after under `evidence/`, and record the rollback command in `docs/github-rulesets.md`, so a broken gate can be removed in one step without disabling a ruleset. Update `docs/github-rulesets.md`, the `merge-pull-request` override procedure, and `implement-workflow`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| T8  | IN_PROGRESS | Measure                                         | Not done in this issue, so #2525 can close without a waiting period (the same decision as #2507 T7, 2026-10-15). (a) DONE on 2026-10-21: [`pr-ci-cost-baseline.md`](pr-ci-cost-baseline.md) records the "before" runner time per pushed head and per pull request for documentation-only and code pull requests, from every run of the 98 pull requests merged since 7 October, with the raw per-run data under `evidence/`. (b) When T7 lands, draft and create a follow-up measurement issue that compares at least two weeks after T7 with (a), and link it here.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

## Delivery Sequence

Agreed with the maintainer on 2026-10-15. Each step is safe to merge on its own. No step changes
which checks GitHub requires until step 5.

1. **Rust only (T1 to T3).** ADR, change detection, and verdict, with unit tests. No workflow or
   ruleset change, so CI behavior stays the same. These may land in one or more pull requests.
2. **Sandbox decision gates (T4, T5).** Gate cost, check-run superseding, and the override's
   cross-run state, proven in a fork sandbox. The maintainer approves each decision before T6
   starts.
3. **Before-baseline (T8 a).** Record the current PR CI runner time before T6 merges.
4. **Orchestrator, not required (T6).** `CI gate` reports on every pull request but no ruleset
   requires it. Observe it on real code and documentation-only pull requests.
5. **Make it required (T7).** Only with explicit maintainer approval. Save the ruleset JSON and
   document the rollback.
6. **Close.** Create the follow-up measurement issue (T8 b), finish the verification and review
   checkpoints, close #2525, and archive this specification. Closing does not wait for the
   measurement window.

### Sandbox Plan for T4 and T5

Approved by the maintainer on 2026-10-21, as a throwaway sandbox in the fork `josecelano/torrust-tracker`
instead of experiments on upstream pull requests, like the #2507 sandbox. T4 runs first. Its numbers go to
the maintainer, who approves the AC6 bound, before T5 starts. Every experiment records its commands, its
observed results with UTC times, and its cleanup in
[`manual-verification-evidence.md`](manual-verification-evidence.md).

#### Setup and cleanup

- Branches `exp-2525-base` and one head branch per sandbox pull request, all prefixed `exp-2525-`. A
  documentation-only sandbox pull request adds one signed commit with a Markdown file under `exp-2525/`,
  so the fork's heavy workflows skip it.
- A sandbox ruleset scoped to `refs/heads/exp-2525-base` that requires the status check `CI gate`
  (Actions app, not strict), with the other rules of the upstream rulesets 87085 and 94133 that matter to a
  merge (pull request required, signatures required).
- Sandbox workflows exist only on the `exp-2525-*` branches and never reach upstream. Merges are built the
  way `github-merge.py` builds them and pushed directly, as in #2507.
- Cleanup at the end of each task: delete the sandbox ruleset, every `exp-2525-*` branch, and close every
  sandbox pull request. Record the time.

#### T4: gate cost and override

- **E1, runner time the gate adds.** A minimal gate job runs on documentation-only sandbox pull requests, at
  least 10 runs per variant, measured as job duration like the baseline
  ([`pr-ci-cost-baseline.md`](pr-ci-cost-baseline.md)). Variants: (a) a cold `cargo build -p ci-gate` with
  no cache; (b) the same with the Rust cache; (c) a prebuilt binary, either cached by a hash of the crate
  or built by a build job and downloaded. The comparison is the `Docs Lint` baseline: median 0.85 minutes,
  with a run of about 4.3 minutes one time in sixteen on documentation-only pull requests. The result is
  a proposed bound for AC6, which the maintainer approves.
- **E2, a later check run superseding an earlier one.** With `CI gate` required in the sandbox ruleset:
  (a) a failing `CI gate` followed, for the same head commit, by a passing one from a different workflow
  run, started by a `labeled` event; (b) the reverse order, a passing one followed by a failing one, started
  by an `unlabeled` event; (c) two workflows that each define a job named `CI gate`. Each case records the
  pull request's merge state and the result of a real merge push, because what the ruleset does is what
  matters.

#### T5: cross-run state for the override

- **(a) Where the gate reads the earlier results.** Candidates: the check-runs API for the head commit,
  artifacts of the earlier run, or rerunning the jobs. The first to prove is the check-runs API, behind a
  thin adapter that produces the `results` JSON the verdict reads, so the verdict stays provider-neutral.
- **(b) Where it reads who applied the label.** Candidates: `sender` in the `labeled` event payload, and
  the issue events API for events that do not carry it. The role is checked with the collaborator
  permission API; record which token can call it.
- **(c) Token permissions.** `pull_request` against `pull_request_target`, where the re-evaluation runs
  only trusted code from the base branch and never checks out the pull request's code. A pull request
  inside the fork gets a write token, so the sandbox cannot reproduce the read-only token of a fork pull
  request. By maintainer decision, this part relies on GitHub's documented behavior and is verified on real
  upstream pull requests when T6 lands, where `CI gate` is reporting but not required.
- **(d) Whether an override survives a push.** The recommended answer is that it does not, as the
  repository already dismisses stale reviews on push. Two ways are tested: removing the label on
  `synchronize`, and tying the override to the head commit it was granted for.
- **Hosted runs** in the sandbox for label added, label removed, and a push after an override. The
  maintainer approves the design before T6 starts.

### T5 Design Proposal: the Override Across Runs

Proposed on 2026-10-21 from the sandbox runs of
[`manual-verification-evidence.md`](manual-verification-evidence.md) V8 to V10, and approved by the
maintainer on 2026-10-21 as proposed. It is the design of T6. The sandbox used `jq` and `gh` in workflow steps only to rehearse; the
production gate keeps every decision in Rust, so the workflow only fetches raw API responses and passes
them to the tool, as the ADR requires.

#### Decisions proposed

- **(a) Where the gate reads the results.** From the checks API, for the head commit: the latest run of
  each group's job that is not skipped, by the Actions app, with a name that is a group name; a job that
  has not completed is missing. One code path serves every event, push or label, and `needs` only orders
  the jobs. Not chosen: artifacts of the earlier run (needs more permission and more plumbing, and was not
  needed), and rerunning the jobs (costs the runner time that the override is meant to save). Contract:
  the name of each group's job is the group's name.
- **(b) Who applied the override.** The latest `labeled` event for `ci-override` in the issue events
  API, while the label is still on the pull request. Its actor is checked with the `role_name` of the collaborator
  permission API (its `permission` field is the legacy base role and reports Maintain as `write`), and only `admin`
  or `maintain` counts, a custom role never; any other result, or an error reading it, means no override.
- **(c) Token and trust.** The gate runs on `pull_request`, never `pull_request_target`, with a token of
  read scopes only (`contents`, `checks`, `issues`, `pull-requests`) and no secrets, so it works the same
  for a pull request from a fork. It builds the tool from a checkout of the base branch, so a pull request
  cannot change the tool. Trust of each input: the check runs and the events are created by GitHub; the
  label can be applied by anyone with Triage access, so the role check on the actor is what restricts it; the
  classification is computed from the pull request's file list, which is the pull request's own content, as
  it is for the workflow filters today. Residual risk, the same as today: a pull request that edits workflow
  files changes what its own checks report, and the maintainer who merges reviews those changes.
- **(d) A push revokes the override.** The override counts only if the label was applied after the head
  commit's first check suite was created, that is, after that push. The label stays on the pull request but
  is ignored, and the explanation says it is stale and must be removed and applied again. Not chosen:
  removing the label on a push, which needs a write token that a pull request from a fork does not have.

#### What T6 must still verify, on real pull requests

- The three API calls, and the label events, with the token of a pull request from a fork, and whether a
  first-time contributor's run waits for approval, which would hold `CI gate` pending.
- That a label applied by a user without Maintain or Admin access is ignored, which the sandbox could not
  test with one account.
- The names of the group jobs of the real workflows against the contract of (a), and the check runs that
  skipped jobs leave.
- What happens to a pull request that is open when the orchestrator changes: its label runs keep the old
  workflow until its head changes.

## Commit Points

| Task | Coherent change set                                      | Commit policy                                                           |
| ---- | -------------------------------------------------------- | ----------------------------------------------------------------------- |
| T1   | ADR and index row                                        | Commit after maintainer review of the ADR text.                         |
| T2   | Change-detection module and its tests                    | Commit after focused tests pass and the test design review is recorded. |
| T3   | Verdict module and its tests                             | Commit after focused tests pass and the test design review is recorded. |
| T4   | Sandbox evidence                                         | Commit after the maintainer approves the approach.                      |
| T5   | Sandbox evidence and the approved cross-run design       | Commit after the maintainer approves the design.                        |
| T6   | One commit per converted workflow, then the orchestrator | Commit after `linter all` and a sandbox or PR run.                      |
| T7   | Documentation and skills; ruleset evidence               | Commit after the maintainer applies the ruleset.                        |
| T8   | Before-baseline evidence; follow-up issue specification  | Commit (a) before T6 merges; commit (b) after maintainer review.        |

For test-producing work, use the `write-unit-test` skill and record the prose-first
Arrange-Act-Assert review after each passing increment, before maintainer review and commit. Use
Conventional Commits with narrow scopes and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/ci-gate-required-check/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, the tool's tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-14 10:08 UTC - AI assistant - Drafted as task T8 of issue #2507 from its sandbox evidence
  and the maintainer's decisions of 2026-10-14.
- 2026-10-14 10:10 UTC - AI assistant - Maintainer approved the draft; created issue #2525 and moved
  the spec to `docs/issues/open/`. It ships with the #2507 implementation pull request; the
  implementation branch is `2525-ci-gate-required-check`.
- 2026-10-14 10:38 UTC - AI assistant - Added decision gate T5 for the override's cross-run state
  (Copilot review of PR #2526, finding F3) and renumbered the later tasks; added scenarios M6 and M7.
- 2026-10-15 17:20 UTC - AI assistant - Recorded the maintainer-agreed Delivery Sequence. T6 lands
  `CI gate` reporting but not required, and T7 makes it required only after real-PR observation and
  maintainer approval, with a documented rollback. T8 now records a before-baseline before T6 and
  moves the measurement to a follow-up issue created at T7, so #2525 can close without a waiting
  period, as decided for #2507 T7. The specification merged with PR #2526, so no separate spec-only
  PR is needed.
- 2026-10-16 16:03 UTC - GitHub Copilot - T1 done: the maintainer reviewed and approved the draft of the
  CI-portability ADR. It records that CI decision logic lives in Rust tools with provider-neutral
  inputs and outputs and that workflows stay thin, with one always-reporting `CI gate`, and leaves the
  override mechanism, the gate cost, and the tool layout to T2 to T5. Next: T2 and T3 (Rust only, no
  workflow or ruleset change).
- 2026-10-20 12:25 UTC - GitHub Copilot - T2 implemented, awaiting maintainer review (no commit yet).
  The maintainer decided the layout (one `ci-gate` crate for T2 and T3, in
  `contrib/dev-tools/checks/ci-gate`), the groups (one per workflow), the input (changed paths on
  stdin, later changed to NUL-separated `git diff --name-only -z` records after the review of PR #2574, plus `--base-branch`), the rules (a typed Rust table), and the scope (T2 only). Evidence:
  86 unit and 3 command-line tests, `cargo clippy` and `cargo +nightly fmt --check` clean, 10 of 10
  mutants killed, 60 of 60 recent pull requests classified as GitHub ran them, the recipe stage of the
  container build passes, and the TTY refusal shown in a pseudo-terminal -
  [`manual-verification-evidence.md`](manual-verification-evidence.md).
  Test-design review (prose-first Arrange-Act-Assert, recorded after the tests passed):
  - **Tables of rules** (documentation rule, branch rule, database and security rules): Arrange is the
    changed paths and base branch, written inline in each `#[case]`, whose name states the one
    difference that matters (for example "package whose name starts with the same text"). Act is the
    visible `classify(...)` call. Assert compares `Classification::needs(group)` with a literal
    expectation, and its message carries the group, the paths and the branch. Nothing is derived from
    the production table.
  - **Decision text** (which path or branch is named): Arrange is one change in which a single path or
    branch decides; Assert checks the explanation contains it, with the actual text in the failure
    message.
  - **Command-line tests**: `run_ci_gate` hides only the process mechanics (spawning, piping, waiting);
    the Act, the arguments and the changed paths, stays in the test. Asserts check the whole contract
    for that run: exit status, stdout, and stderr.
  - **Smells checked**: no complex Arrange (no fixture beyond `run_ci_gate`); no hidden Act; no
    production-derived expectation; the CLI success test has several assertions that together specify
    one result (the contract of a successful run) and is kept as one test.
  - **Change made**: the test that read the private `RULES` table to check one rule per group was
    removed, because it was structure-sensitive and the output-order test already covers the same
    behavior (one decision per group, in order).
  - **Untested on purpose**: the TTY refusal is one early return in `main` that the harness cannot
    trigger (stdout is always piped); manual scenario M8 covers it. The base-branch filters were not
    exercised by a real pull request in M9, because the sample is all into `develop`; unit tests cover
    them.
- 2026-10-20 12:28 UTC - GitHub Copilot - The maintainer reviewed T2 and approved it as it stood, with no
  requested changes. Committed in three signed commits: `feat(ci-gate): [#2525] add the
  change-detection tool`, `docs(adrs): [#2525] list ci-gate in the CLI output contract table`, and this
  specification and evidence. T2 and AC4 are done. Next: T3 (the verdict), in its own pull request.
- 2026-10-20 13:06 UTC - GitHub Copilot - Processed Copilot's review of PR #2574 (two findings, one root
  cause): newline-separated text input could not represent file names faithfully, because `trim()`
  changed names with spaces and `git diff --name-only` C-quotes non-ASCII names. The input is now the
  output of `git diff --name-only -z` (NUL-separated, kept as is), fixed in `fix(ci-gate): [#2525] read
  the changed paths as NUL-separated records`, with unit and end-to-end regression tests (99 unit and 5
  command-line tests, and 3 more mutants killed). The comparison with GitHub's real runs was re-run with
  the new input: 60 of 60 - [`manual-verification-evidence.md`](manual-verification-evidence.md) V2, V3,
  V6.
- 2026-10-20 14:59 UTC - GitHub Copilot - Processed the second Copilot review of PR #2574 (three findings, all
  confirmed). The documented claim that replacing invalid UTF-8 cannot change a match is true (ASCII bytes
  decode to themselves and U+FFFD is not ASCII) but nothing argued or pinned it, so the documentation now
  gives the argument and five tests pin it (`fix(ci-gate): [#2525] state why invalid UTF-8 cannot change
  a match`). The documentation counted six workflows where the rules cover eight, here and in the T2 row
  (`docs(ci-gate): [#2525] correct the workflow count in the rules documentation`). Every success-path
  command-line test now asserts exit status 0 and an empty stderr (`test(ci-gate): [#2525] assert the
  success contract in every success-path CLI test`). Now 104 unit and 5 command-line tests, and 2 more
  mutants killed - [`manual-verification-evidence.md`](manual-verification-evidence.md) V6.
- 2026-10-20 15:35 UTC - GitHub Copilot - Processed the third Copilot review of PR #2574. Its two findings are in
  the review body and have no inline thread. Both were confirmed. `env::args` panics on a non-UTF-8
  argument, so the binary exited with 101 and a plain-text message instead of the JSON usage diagnostic
  and exit code 2 that the CLI output contract requires; reproduced with a real binary, fixed in
  `fix(ci-gate): [#2525] report a non-UTF-8 argument as a usage error`, with a Unix-only regression test
  that was seen to fail first. The usage-error test wrote stdin to a binary that exits before reading
  it, which can fail with a broken pipe; it now writes nothing (`test(ci-gate): [#2525] send no stdin to
  the command that rejects its arguments`). Now 104 unit and 6 command-line tests, and one more mutant
  killed - [`manual-verification-evidence.md`](manual-verification-evidence.md) V6.
- 2026-10-20 16:50 UTC - GitHub Copilot - T3 implemented on branch `2525-ci-gate-verdict`, after the maintainer approved
  the defaults on 2026-10-20 (JSON on stdin, `--override-actor`). The `verdict` subcommand decides from the
  classification, each group's result, and the optional override; see the T3 row for the rules and the
  exit-code decision, which the ADR now records. 132 unit and 10 command-line tests, 10 of 10 mutants
  killed, and a manual run of `git diff -z` into `classify` into `verdict` -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) V7. Next: T8(a), the "before"
  baseline, then the T4 and T5 decision gates.
  Test-design review (prose-first Arrange-Act-Assert, recorded after the tests passed):
  - **Decision tests**: Arrange is the set of needed groups and the results, written in the test through
    `classification_needing(&[...])` and `results(&[...])`; the helpers hide only the explanation
    placeholder that every group decision carries, and the causal facts (which groups are needed, what
    each reported, who overrode) stay visible. Act is the visible `decide(...)` call. Assert compares the
    whole `VerdictResult` with a literal, so the verdict, the problems, the actor, and the explanation are
    one specification. Nothing is derived from the production code.
  - **Result and problem tables**: one `#[case]` per way a needed group fails to count (failed, cancelled,
    skipped, no result), and one per way a group that is not needed must be ignored.
  - **Input shape**: each rejection case changes one thing in an input that is otherwise valid, so it
    can only be rejected for the reason it names. A first version used `null` for the classification, so
    every case would have been rejected for that reason alone; it was caught in review and rewritten.
  - **Command-line tests**: the Act, the arguments and stdin, stays in the test; helpers hide spawning
    and piping. The exit-code decision is pinned by a test that a failing verdict exits with 0, and a
    second test pins that unreadable input exits with 1 and prints nothing. One test pipes the real
    output of `classify` into `verdict`, so the two tools' formats cannot drift apart unseen.
  - **Smells checked**: no complex Arrange; no hidden Act; the whole-result assertions are one
    specification each; the explanation text is pinned because it is the output a maintainer reads.
  - **Untested on purpose**: the wording of the runtime-error message for unreadable input (the test
    pins `kind` and the exit code); the Maintain or Admin check of the override actor, which belongs to T5.
- 2026-10-20 17:21 UTC - GitHub Copilot - Processed Copilot's review of PR #2580 (one finding, confirmed):
  `deny_unknown_fields` on the verdict input is not recursive, so an extra field inside the classification
  or inside a group decision was ignored and the input still passed, although the documentation said
  unknown fields are errors. Reproduced with the real binary, fixed in `fix(ci-gate): [#2525] reject
  unknown fields at every level of the verdict input` with one test case per level, each seen to fail
  first and killed by its own mutant. Now 134 unit and 10 command-line tests -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) V7.
- 2026-10-20 17:52 UTC - GitHub Copilot - Processed the second Copilot review of PR #2580 (five findings, all
  confirmed; the two about the audit's severity value are one concern). The documentation said the problems
  follow the order of `JobGroup::ALL`, but the verdict kept the order of the classification; it now derives
  the needed groups once in group order, with a test on a reversed classification that failed first
  (`fix(ci-gate): [#2525] list the verdict's groups in group order`). The V7 evidence omitted the backticks
  the tool prints around the actor's login, and is now verbatim (`docs(issues): [#2525] quote the override
  explanation verbatim in the V7 evidence`). I had dropped the JSON diagnostics note from the ADR's contract
  row to fit its column, and restored it (`docs(adrs): [#2525] keep the JSON diagnostics note in the ci-gate
  contract row`). The audit now uses a defined severity value (`docs(pr-reviews): use a defined severity
  value in the PR #2580 audit`). Now 135 unit and 10 command-line tests -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) V7.
- 2026-10-21 07:17 UTC - GitHub Copilot - T8(a) done on branch `2525-before-baseline`: the "before" PR CI runner time
  is recorded in [`pr-ci-cost-baseline.md`](pr-ci-cost-baseline.md), from every `pull_request` run of the
  98 pull requests merged into `develop` since 7 October (1,189 attributed runs), with the per-run data in
  `evidence/` because GitHub deletes old runs. A documentation-only pushed head costs a mean 1.1 runner
  minutes (median 0.85; one run in sixteen takes about 4.3), a code pushed head 87.8 (90.8 since #2526).
  The first collection silently dropped 245 runs because a listing is capped at 1000 results; comparing
  with `total_count` showed it, and per-day queries fixed it. T8(b) remains for when T7 lands. Next: the
  T4 and T5 decision gates, which need a fork sandbox and the maintainer's approval.
- 2026-10-21 07:44 UTC - GitHub Copilot - Processed Copilot's review of PR #2590 (three findings, all confirmed). The
  saved data did not say which pull requests the per-pull-request table excludes, so each now has its
  creation time and a `complete_history` flag, and the table was recomputed from the committed files alone
  and matched (`docs(issues): [#2525] save the creation time and cohort of each baseline pull request`).
  I had counted 394 `Docs Lint` runs under 1.5 minutes and 2 in between, where the data has 392 and 4
  (`docs(issues): [#2525] correct the Docs Lint duration bucket counts in the baseline`). The one-in-nine
  slow-run rate pooled both classes; for documentation-only pull requests it is 17 of 270 runs, about one
  in sixteen (`docs(issues): [#2525] give the Docs Lint slow-run rate per class of pull request`).
- 2026-10-21 09:07 UTC - GitHub Copilot - The maintainer approved the sandbox plan for T4 and T5 (2026-10-21): a
  throwaway sandbox in the fork, T4 first with its numbers approved before T5, and T5(c) relying on
  GitHub's documented fork-pull-request token behavior, to be verified on real upstream pull requests when
  T6 lands. The plan is recorded in "Sandbox Plan for T4 and T5". The fork was checked first: no leftover
  `exp-` branches, no open pull requests, Actions enabled, and the workflows run on `ubuntu-latest`.
- 2026-10-21 09:47 UTC - GitHub Copilot - T4 sandbox runs done in the fork, then cleaned up (rulesets, branches, release,
  labels; verified). E1: one commit rerun 12 times, five jobs per attempt, 60 jobs, all succeeded. A
  cold `cargo build --release -p ci-gate` job costs a median 0.40 runner minutes, the Rust cache does not help
  (0.37), and a prebuilt binary, cached or downloaded, costs about 0.12; the gate needs two tool runs, so
  about 0.8 minutes cold or 0.25 with a prebuilt binary, against a `Docs Lint` baseline median of 0.85. E2,
  with a ruleset requiring `CI gate` and real merge pushes: a later passing run overrides an earlier failing
  one and the reverse, for the same job of one workflow; with two workflows that each define `CI gate`, one
  failing blocks the merge although the other passes, so the gate must be one workflow with one job. The
  fork's default workflow permission is `write` and upstream's is `read`, so the sandbox says nothing about
  upstream's default token. Awaiting the maintainer's approval of the approach and of a bound for AC6
  before T5 starts - [`manual-verification-evidence.md`](manual-verification-evidence.md) V8 and V9.
- 2026-10-21 10:31 UTC - GitHub Copilot - The maintainer approved T4 on 2026-10-21, after the evidence of PR #2600: both
  gate jobs build the tool cold, and a prebuilt binary is adopted only if a later measurement exceeds the AC6
  bound, which is at most 1.0 runner minute added at the median to a documentation-only pushed head, counting
  the two tool runs (the sandbox measured about 0.8 minutes cold and 0.25 with a prebuilt binary). T4 is
  done. Next: T5, the cross-run state of the override, in a new sandbox, with its design approved by the
  maintainer before T6 starts.
- 2026-10-21 10:53 UTC - GitHub Copilot - T5 sandbox runs done in the fork, then cleaned up (ruleset, branches, label;
  verified). One orchestrator rehearsal with a read-only token and the real `ci-gate verdict` on a sandbox pull
  request, in seven runs: a label run reads the earlier results of the groups from the checks API, the actor
  from the issue events API, and the role from the collaborator permission API, and passes with the override
  named; a new push makes the override stale although the label stays on the pull request; a label applied
  after the push counts, and the merge push was accepted. A script error of mine (an empty actor) made two
  runs fail closed with a usage error instead of a verdict, and another finding is that a workflow change on
  the base branch does not reach the label runs of an open pull request until its head changes. The design
  proposed from this is in "T5 Design Proposal: the Override Across Runs", for the maintainer's approval
  before T6 starts.
- 2026-10-21 11:07 UTC - GitHub Copilot - The maintainer approved the T5 design as proposed, on 2026-10-21 (decisions (a)
  to (d) of "T5 Design Proposal: the Override Across Runs"). T5 is done. T6 is split into three steps, as
  recommended: T6a, the Rust adapter that turns raw API responses into the verdict input (no workflow
  change); T6b, one workflow converted behind the orchestrator, reporting but not required, tried in the
  fork sandbox first; T6c, the other workflows, one commit each.
- 2026-10-21 11:23 UTC - GitHub Copilot - T6a implemented on branch `2525-ci-gate-github-adapter`, with the design of T5
  and the defaults named here. `ci-gate github-verdict --repo <owner/name> --pull-request <number>
  --head-sha <sha> --classification <file>` reads the check runs, labels, events, check suites, and the
  actor's permission through `gh api --paginate`, and prints the verdict JSON with one more field,
  `override_ignored`, which says why a label that was asked for did not count. Defaults chosen: the API is
  reached through the `gh` program, behind a `GithubApi` trait, so the rules are tested with fixtures and
  there is no new dependency; the classification comes from a file, which `classify` writes from
  `git diff -z`; the "latest" run of a group is the one with the highest check-run id, not the latest start
  time, because a queued rerun has no start time; the override is looked up only when the verdict fails without
  it; an error reading the check runs is a runtime error (exit 1, nothing on stdout), and an error reading
  anything the override needs means no override with the reason in `override_ignored`; a failing verdict
  still exits with 0; the repository, commit id, and login are validated because they become part of request
  paths. 198 unit and 18 command-line tests, 16 of 16 mutants killed, and a manual run on real GitHub data,
  including a replay of runs 5 and 7 of the T5 sandbox from the captured responses that gave the same results -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) V11. The current upstream workflows
  have no check runs named after the groups, so on an upstream pull request every needed group is missing; that
  is the contract for T6b.
  Test-design review (prose-first Arrange-Act-Assert, recorded after the tests passed):
  - **Rule tables** (the result of a check run, which runs are ignored, which run of a group is the latest,
    whether an override counts, which repositories are accepted): Arrange is one fixture value written in the
    `#[case]`, named for the one difference that matters; Act is the visible call of the function under test;
    Assert compares a literal. Nothing is derived from the production code.
  - **Scenarios** through `decide_for_pull_request`: a `FakeApi` answers each path with a fixture and records the
    requests. The causal values, the actor, the time of the label, the time of the first check suite, and the
    permission, are arguments in the test and not hidden in the fake. The paths are written out in the tests,
    because they are the contract with GitHub, and one test asserts that only the check runs are requested when
    nothing failed.
  - **Command-line tests**: `FakeGh` hides only how a stand-in `gh` program is written and removed; the
    fixtures and the arguments stay in the test, and an end-to-end test exists for a passing verdict, a failing
    one, an override, a stale override, an unreadable GitHub, a missing file, and a usage error.
  - **Smells checked**: the first version of the end-to-end override helper hard-coded the actor that the
    assertion then named, a hidden behavioral value; the actor is now an argument. The whole-result assertions
    are one specification each.
  - **Untested on purpose**: the real `gh` program talking to GitHub is exercised only manually (V11),
    because a test needs GitHub and a token. The process spawning and the parse of its output, including
    several pages one after the other, are tested with a stand-in program.
- 2026-10-21 14:43 UTC - GitHub Copilot - Processed Copilot's review of PR #2610 (four findings, all confirmed). The
  documentation of `override_ignored` said it is set only when the label is on the pull request, but it is also
  set when the labels, the events, or the permission could not be read (`docs(ci-gate): [#2525] document every
  case of the override_ignored field`). A label on the pull request with no event of it being applied was
  treated as an override that nobody asked for, and nothing was said; it is now reported as ignored, with the
  reason (`fix(ci-gate): [#2525] explain a label that is on the pull request with no event of it being
  applied`). The end-to-end override helper still built the permission path from one login: when I made the
  actor a parameter, my replacement of that line had silently not applied, and the tests passed because every
  test used that login, so the helper is fixed and the passing-override test uses another login, which failed
  with the old path (`test(ci-gate): [#2525] build the permission fixture path from the actor in the override
  helper`). The error of a failed request named `gh api <path>` although the program runs `gh api --paginate
  <path>` (`fix(ci-gate): [#2525] name the real gh invocation in the error of a failed request`). Now 200 unit
  and 18 command-line tests, and one more mutant killed, from 199 unit tests and 17 mutants after the test that the
  earliest check suite is taken over all pages (`test(ci-gate): [#2525] test that the earliest check suite is taken
  over all pages`) -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) V11.
- 2026-10-21 15:24 UTC - GitHub Copilot - Processed Copilot's second review of PR #2610 (two findings, both confirmed). The
  adapter read the events of the pull request whenever the verdict failed, so a failing events request set
  `override_ignored` on a pull request with no override label at all; it now returns as soon as the labels show
  that the label is absent and reads the events only after that, and the documentation of the field says when it
  is `None` (`fix(ci-gate): [#2525] read the events only when the override label is on the pull request`). The
  behavior was in the first version of the adapter: the first review's finding about it offered to fix the
  implementation or to broaden the documentation, I broadened the documentation, and this finding asked for the
  implementation fix. No test covered a label that is absent while the events request fails. The directory of a fake `gh` in the
  end-to-end tests was named after the process and the test name, so two fixtures with the same name would share
  files; a counter is added (`test(ci-gate): [#2525] give each fake gh a directory of its own`). Both new tests
  were seen to fail first. Now 201 unit and 19 command-line tests, and two more mutants killed -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) V11.
- 2026-10-21 15:52 UTC - GitHub Copilot - Processed Copilot's third review of PR #2610 (three threads, one concern: the
  end-to-end tests of `github-verdict` exist only on Unix, because the stand-in `gh` is a shell script). I checked
  what is lost: the pull-request workflows run tests on Linux only, because the Windows and macOS jobs of
  `os-compatibility.yaml` only run `cargo build`, and the sibling tool `frontmatter-validator` gates its command-line
  tests the same way, so a cross-platform stand-in would cost a helper binary built with every `cargo build -p
  ci-gate`, which the cold-build budget of AC6 counts, for no coverage that CI runs today. What the finding does
  show is that the parsing of the paginated output of `gh` and the failure message were covered only by the
  Unix tests, so both are now pure functions with unit tests that run everywhere (`refactor(ci-gate): [#2525] test
  the parsing of the output of gh on every platform`), and the reason for the gating is written next to the
  stand-in (`docs(ci-gate): [#2525] say why the github-verdict command-line tests are unix-only`). Now 209 unit and
  19 command-line tests, and four more mutants killed -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) V11.
- 2026-10-21 16:46 UTC - GitHub Copilot - Processed Copilot's fourth review of PR #2610 (two threads, one concern, both
  confirmed). The name of a fake `gh` directory contained the test name unchanged, and the directory is removed
  recursively when the fixture is dropped, so a name with a path separator or two dots could choose where it is
  created and removed. Every character that is not a letter, a digit, a hyphen, or an underscore is now replaced
  (`test(ci-gate): [#2525] keep a fake gh directory inside the temporary directory`). The new test failed first
  and left a stray directory in the temporary directory, which was removed by hand; a mutant that replaces only
  the dots is killed. My first attempt at that mutant asserted that the text to replace was there and stopped
  because the formatter had wrapped it, so nothing was silently skipped. Now 209 unit and 20 command-line tests,
  and one more mutant killed - [`manual-verification-evidence.md`](manual-verification-evidence.md) V11.
- 2026-10-21 18:50 UTC - GitHub Copilot - Processed Copilot's fifth review of PR #2610 (one thread, confirmed). The fixture
  of the fake `gh` dropped the result of removing its directory, so a failed cleanup left a stray directory with
  no trace. The removal is now a function that returns a message for any failure except a directory that is already
  gone, and the fixture writes the message on stderr; it does not panic, because a second panic while one unwinds
  aborts the test process (`test(ci-gate): [#2525] report a failed cleanup of a fake gh directory`). Three new
  tests, and two mutants killed. The first commit was made without gating on the formatter and clippy, which
  rejected it (`eprintln` is denied by `clippy::print_stderr`), and a second commit fixed it
  (`test(ci-gate): [#2525] write the cleanup failure through writeln and format it`). The call of the
  function from the fixture's `Drop` is not covered by a test: a mutant that ignores the message there would
  survive, and it is a one-line call whose effect is a line on stderr. Now 209 unit and 23 command-line tests -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) V11.
- 2026-10-21 19:17 UTC - GitHub Copilot - Processed the maintainer-side review of PR #2610 (review 5604832273, nine threads,
  F13 to F21, `CHANGES_REQUESTED`). F13 was real and Major: the permission endpoint's `permission` field is the
  legacy base role and reports the Maintain role as `write`, so an override by a Maintain-role actor was always
  refused, although the module, the T5 design and AC3 say it counts. V10 had observed only `admin` and `read`.
  The adapter now reads `role_name`, a custom role never counts, and the fixtures have the shape of the real
  response (`fix(ci-gate): [#2525] read the role of the actor from role_name, not permission`). F16: the
  check-runs request now sets `filter=all`, which the group results rely on (`fix(ci-gate): [#2525] ask for every
  check run, not only the latest of each name`). F17: tests for `is_login` and for the unreadable labels and
  check suites. F20: the cleanup test now asserts the cause. F15, F14, F18, and F19 are fixes to documentation and
  records, each in its own commit. F21 asks to fold the lint-fix commit into the one it fixes; it is not done,
  because the repository keeps a mistake and its correction on purpose. Now 226 unit and 23 command-line tests, and
  nine more mutants killed - [`manual-verification-evidence.md`](manual-verification-evidence.md) V11.
- 2026-10-21 19:47 UTC - GitHub Copilot - Processed Copilot's sixth review of PR #2610 (three threads, F22 to F24), which had
  arrived at 19:20 UTC, before I pushed the previous round. I found it only after a bulk resolve had closed its threads
  without my reading them; I then read each, checked it, and replied. F22 asked to avoid `Option::is_none_or` for the
  MSRV: it is stable since Rust 1.82.0, the workspace `rust-version` is 1.88, and the crate and its tests compile with
  the 1.88.0 toolchain, so no change. F23 asked to simplify the `Some` built inside a `map`; done with a `let ... else`
  (`refactor(ci-gate): [#2525] return the override request without a nested option in a map`), with two mutants
  killed. F24 asked for `let _ =` instead of `drop(...)`, which the workspace lint `let_underscore_drop` denies; the
  pattern stays and a comment says why (`test(ci-gate): [#2525] say why the fixture ignores the write with drop`).
  Still 226 unit and 23 command-line tests - [`manual-verification-evidence.md`](manual-verification-evidence.md) V11.
- 2026-10-21 19:53 UTC - GitHub Copilot - Processed the second review of the reviewer `da2ce7` on PR #2610 (review 5605208219, state
  `DISMISSED` by the later push, three threads; the reviewer labels them F22 to F24, recorded here as F25 to F27 because
  the record had used those labels). All of round 2 was found repaired, and F21 was withdrawn on the rule against
  squashing. F25: the record missed the review bot's sixth review; it was recorded and answered in the previous
  commit of this branch. F26: three records cited commit ids of the branch; I had not run the check of the
  `open-pull-request` skill before pushing, and the audit, V11, and one of my replies now cite subjects
  (`docs(pr-reviews): cite commit subjects, not ids, in the PR #2610 audit and V11`). F27: the 14:43 entry now shows
  the step from 199 tests and 17 mutants (`docs(issues): [#2525] show the step from 199 tests and 17 mutants in the
  14:43 progress log entry`). No code changed.
- 2026-10-22 07:05 UTC - GitHub Copilot - The reviewer `da2ce7` approved PR #2610 at 2026-10-21 20:10 UTC (review 5605593995) and
  posted an ACK of the head at that time, with one open thread: a nit, F28, that a log entry of the audit placed an
  event after a review that already reported it. It was right and is fixed in the audit
  (`docs(pr-reviews): place the bulk resolve before the review that reported it in the PR #2610 audit`). No code changed,
  so the ACK no longer names the head of the branch.
- 2026-10-22 07:07 UTC - GitHub Copilot - Processed Copilot's seventh review of PR #2610 (three threads, F29 to F31), which had arrived
  at 2026-10-21 20:14 UTC, after the approval above. F29 asked to change the saved sandbox workflow, which still reads
  `.permission`; no change, because that file is the record of what ran in T5 and V10 now carries the correction.
  F30: the usage text said `--head-sha <sha>` for a value that must have 40 hexadecimal digits; it says
  `<40-hex-commit-id>` now (`fix(ci-gate): [#2525] show the form of the head commit id in the usage text`). F31:
  `override_request` checked the labels that its caller had already checked; it takes only the events now and returns
  a request, not an option of one (`refactor(ci-gate): [#2525] take only the events to find the override request`), and
  a mutant that removes the caller's check is killed by two tests of the caller. Now 226 unit and 23 command-line tests,
  and two more mutants killed - [`manual-verification-evidence.md`](manual-verification-evidence.md) V11.

## Acceptance Criteria

- [ ] AC1: Every pull request, code or documentation-only, reports exactly one `CI gate` check.
- [ ] AC2: GitHub rejects the maintainer merge push while `CI gate` is failing, pending, or missing,
      and accepts it when `CI gate` passes, with no ruleset disabled at any point.
- [ ] AC3: A maintainer-applied `ci-override` label makes `CI gate` pass without rerunning the heavy
      jobs, the explanation names the actor and the overridden failures, and a label from an actor
      without the Maintain or Admin role does not count. The gate reads the earlier results and the
      label provenance from the sources approved in T5, and behaves as T5 decided after a new push.
- [x] AC4: Change detection selects the same job groups as today's workflow path filters for every
      tested path rule.
- [ ] AC5: Workflow YAML contains no change classification or verdict logic; both live in the Rust
      tool with unit tests.
- [ ] AC6: A documentation-only pull request's CI runner time stays within the bound approved in T4: the
      gate adds at most 1.0 runner minute at the median to a pushed head, with its two tool runs, compared
      with the baseline in [`pr-ci-cost-baseline.md`](pr-ci-cost-baseline.md).
- [ ] AC7: The CI-portability ADR is merged and `docs/github-rulesets.md` lists the `CI gate` rule.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p <ci-gate package>` (stable Rust toolchain)
- `./contrib/dev-tools/git/hooks/pre-push.sh`
- The `implement-workflow` skill's checks for the changed workflows

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                          | Human-oriented command/steps                                                            | Expected Result                                                           | Status | Evidence                                     |
| --- | --------------------------------- | --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Documentation-only PR             | Open a Markdown-only PR and read its checks.                                            | Heavy jobs skipped; `CI gate` passes; merge tool push accepted.           | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Code PR                           | Open a PR changing Rust code.                                                           | Needed groups run; `CI gate` passes after them; push accepted.            | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Failing PR is blocked             | Merge-tool push for a PR with a failing needed job.                                     | `GH013` naming the failing `CI gate`; gate output names the failed group. | TODO   | `manual-verification-evidence.md` section V3 |
| M4  | Maintainer override               | Add `ci-override` to the M3 PR as a maintainer, then push the merge.                    | `CI gate` passes naming the actor; heavy jobs not rerun; push accepted.   | TODO   | `manual-verification-evidence.md` section V4 |
| M5  | Override by an unauthorized actor | In the sandbox, have an account with the Triage role add the label.                     | `CI gate` stays failing.                                                  | TODO   | `manual-verification-evidence.md` section V5 |
| M6  | Override removed                  | Remove `ci-override` from the M4 PR.                                                    | `CI gate` fails again, naming the failed group.                           | TODO   | `manual-verification-evidence.md` section V6 |
| M7  | Push after an override            | Push a new commit to the M4 PR while the label is present.                              | `CI gate` follows the T5 decision for a new head SHA.                     | TODO   | `manual-verification-evidence.md` section V7 |
| M8  | TTY refusal                       | Run `ci-gate classify --base-branch develop` with stdout on a terminal (`script -qec`). | `tty_refusal` record on stderr, exit 2, nothing on stdout.                | DONE   | `manual-verification-evidence.md` section V1 |
| M9  | Differential check vs real runs   | Compare the tool's groups with the workflows GitHub ran for the 60 latest merged PRs.   | Every pull request matches.                                               | DONE   | `manual-verification-evidence.md` section V3 |

Notes:

- M3 to M5 run in a fork sandbox like #2507's unless a real PR provides the situation.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Disposable Verification Scripts

One, recorded 2026-10-20 for T2 and not retained: a Python script that joined the GitHub API's changed
files and workflow runs for the 60 latest merged pull requests with the `ci-gate classify` output
(scenario M9, [`manual-verification-evidence.md`](manual-verification-evidence.md) section V3).

- Rationale: it is a single comparison against GitHub's real behavior, not a durable test. The durable
  tests are the Rust unit and command-line tests of the `ci-gate` crate.
- Why not Rust: the script only glues `gh api` JSON to the binary's output for one run; a Rust test
  would need network access and GitHub credentials, which a maintained test must not.
- Owner and removal: not retained. The commands it ran are recorded verbatim in the evidence, and the
  per-pull-request results are in `evidence/ci-gate-differential-check.txt`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                            |
| ----- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | TODO                   |                                                                                                                                     |
| AC2   | TODO                   |                                                                                                                                     |
| AC3   | TODO                   |                                                                                                                                     |
| AC4   | DONE                   | Unit tests pin each rule, and 60 of 60 recent merged pull requests match what GitHub ran (evidence V3). |
| AC5   | TODO                   |                                                                                                                                     |
| AC6   | TODO                   |                                                                                                                                     |
| AC7   | TODO                   |                                                                                                                                     |

## Risks and Trade-offs

- **Gate cost on documentation-only PRs.** Compiling a Rust tool in every PR can cost minutes and
  erase the #1743 fast path. Mitigation: T4 measures it and chooses caching, a minimal-dependency
  crate, or a prebuilt binary before anything is required.
- **Override semantics.** The sandbox showed statuses cannot override failing check runs; whether a
  later passing check run with the same name supersedes an earlier failing one is unverified. T4
  verifies it before the design depends on it.
- **A broken gate blocks every PR.** Mitigation: introduce the required context only after T6 runs
  on real PRs (T7), keep the verdict unit-tested, and document how to revert the ruleset change.
- **Reusable-workflow conversion breaks `push` CI.** Mitigation: one commit per workflow, keeping the
  `push` triggers and checking a `develop` run after merge.
- **Label permissions.** Labels need only the Triage role. Mitigation: the verdict counts the label
  only when the labeling actor has the Maintain or Admin role, verified through the source approved
  in T5.
- **Cross-run state.** The override run cannot see the earlier run's results or who applied the
  label. Mitigation: T5 defines and proves those sources before the orchestrator is built.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2507 (origin, sandbox evidence), #1743 (documentation-only fast path)
- Related documents: [`docs/github-rulesets.md`](../../../github-rulesets.md)
