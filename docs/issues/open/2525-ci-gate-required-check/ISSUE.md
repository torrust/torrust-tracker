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
last-updated-utc: "2026-10-20 17:21"
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
| T3  | DONE        | Rust verdict                                    | Implemented as the `verdict` subcommand of the `ci-gate` crate on branch `2525-ci-gate-verdict`; decisions approved by the maintainer on 2026-10-20: one JSON object on stdin, `{"classification": <the classify output>, "results": {<group>: success, failure, cancelled, or skipped}}`; the override is `--override-actor <login>` (the Maintain or Admin role check of AC3 belongs to T5); a decided verdict, passing or failing, is a result, so the tool exits with 0 and the workflow reads `verdict`, while input it cannot decide on (unreadable, or a classification that does not list every group once) exits with 1 and prints nothing, so both fail the gate. A needed group that failed, was cancelled, was skipped, or never reported is a problem; a group that is not needed never counts; an override is recorded only when it overrides a problem. 134 unit and 10 command-line tests, 12 of 12 mutants killed. Evidence: [`manual-verification-evidence.md`](manual-verification-evidence.md) V7.                                                |
| T4  | TODO        | Decision gate: gate cost and override           | In a fork sandbox like #2507's: measure the runner time the gate adds to a documentation-only PR (tool compile included, with and without caching or a prebuilt binary), and verify that a later successful `CI gate` check run from an override-only run supersedes the earlier failing one for the required check. Maintainer approves the approach.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| T5  | TODO        | Decision gate: cross-run state for the override | A `labeled` or `unlabeled` event starts a new workflow run that cannot read the earlier run's `needs` results, and later events do not say who applied an existing label. Define, with a maintainer-approved design: (a) where the gate reads the heavy jobs' results for the head SHA (for example the checks API, persisted artifacts, or rerunning the jobs); (b) where it reads label provenance (for example the issue events API) and how it verifies the actor's Maintain or Admin role; (c) the token permissions this needs on fork pull requests, whose `GITHUB_TOKEN` is read-only, and the trust boundary of each input; (d) whether an override survives a new push (`synchronize`) or applies only to the head SHA it was granted for. Prove it with hosted runs in a sandbox for label added, label removed, and a push after an override. Keep provider queries in a thin adapter so the verdict stays provider-neutral.                                                       |
| T6  | TODO        | Orchestrating workflow                          | Convert the pull-request triggers of the heavy workflows into `workflow_call` invocations from one orchestrator; keep `push` triggers; final `CI gate` job; `labeled`/`unlabeled` handling that only re-evaluates the verdict. Follow `implement-workflow`. `CI gate` lands reporting but **not required**: no ruleset change in T6. Before T7, observe it on at least one real code PR and one real documentation-only PR (M1, M2 without the required rule) and record both runs.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| T7  | TODO        | Ruleset and documentation                       | Only after T6 is observed on real PRs, and only with explicit maintainer approval: add the required `CI gate` context (Actions app, not strict) to the rulesets. Save the ruleset JSON before and after under `evidence/`, and record the rollback command in `docs/github-rulesets.md`, so a broken gate can be removed in one step without disabling a ruleset. Update `docs/github-rulesets.md`, the `merge-pull-request` override procedure, and `implement-workflow`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| T8  | TODO        | Measure                                         | Not done in this issue, so #2525 can close without a waiting period (the same decision as #2507 T7, 2026-10-15). (a) Before T6 merges, record the "before" PR CI runner time per documentation-only and code PR in this folder, using the #2507 baseline commands, because workflow-run data expires. (b) When T7 lands, draft and create a follow-up measurement issue that compares at least two weeks after T7 with (a), and link it here.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

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
- [ ] AC6: A documentation-only pull request's CI runner time stays within the bound approved in T4.
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
