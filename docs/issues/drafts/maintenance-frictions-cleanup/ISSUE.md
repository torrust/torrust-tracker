---
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/maintenance-frictions-cleanup/ISSUE.md
branch: "{issue-number}-maintenance-frictions-cleanup-spec"
related-pr: null
last-updated-utc: 2026-09-09 14:11
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - AGENTS.md
    - .github/workflows/container.yaml
    - contrib/dev-tools/checks/tests/test-format-project-words.sh
    - contrib/dev-tools/analysis/workspace-coupling/src/main.rs
    - docs/issues/open/README.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Repository maintenance frictions clean-up

## Goal

Clear a verified batch of small, independent maintenance frictions in one review pass, so that each one stops costing attention on every unrelated change, and record the two items that are not code changes so a maintainer can act on them deliberately.

## Background

Routine work across this repository keeps running into the same set of small defects: a developer-tool test suite that has been red on `develop` since it was moved, three such suites that no orchestrator runs at all, sixteen issue specifications for closed issues still sitting in the open backlog, a workflow trigger that starts a job on branches it immediately refuses, and documentation that still points at binaries which have since moved into a package. None of these is large enough to justify its own issue and none of them blocks a feature, so each has survived several release cycles. Individually they are noise; collectively they are a standing tax on every contributor and agent who reads a stale path, re-diagnoses a red test, or scrolls past a failed workflow run that was never going to succeed.

Each entry below was re-verified against `develop` at revision `f6b73e29` on 2026-09-09; the evidence column records what was observed rather than what was reported. Two candidates that had been proposed for this batch did not reproduce and were dropped; they are recorded in [Dropped Candidates](#dropped-candidates) so the same ground is not re-covered.

The frictions are deliberately grouped rather than filed separately. They share a review context — small, low-risk, mechanical — and splitting them into a dozen issues would cost more maintainer attention than the defects themselves. They do not share an implementation: each row is independently revertible, so a single reviewer can accept or reject them one at a time within one pull request.

## Scope

### In Scope

- The eight code, configuration, and documentation frictions listed in [Friction Inventory](#friction-inventory), each fixed or explicitly deferred with a linked issue.
- Recording the two maintainer-owned items in [Maintainer Actions](#maintainer-actions) so they are visible and actionable, without attempting to perform them from a pull request.
- Recording the one behavioural defect in [Related but Separate](#related-but-separate) with a recommendation that it gets its own issue.

### Out of Scope

- Automating the issue-specification archival flow. That is #1774; this issue performs the current archival by hand and does not constrain the script's design.
- Triaging external-link check findings. That is #2185, whose specification is under review in #2186.
- Repairing the `Docker E2E` job in `.github/workflows/testing.yaml`. That is #2179; this issue only records which stale pull requests are blocked behind it.
- Migrating legacy single-file specifications to the folder-style layout. That is #2159.
- Designing the long-term check harness and sensor architecture. That is EPIC #2003; F1 below adds the missing invocation of suites that already exist and does not prejudge that design.
- Any behavioural change to the tracker itself, other than the explicitly out-of-scope item recorded in [Related but Separate](#related-but-separate).

## Friction Inventory

Size is the expected reviewable weight of the change: `XS` is a one-line edit, `S` is a handful of lines in one file, `M` spans several files or needs a judgement call.

| ID  | Area                | Evidence (verified at `f6b73e29`, 2026-09-09)                                                                                                                                                                                                                                                                          | Proposed fix                                                                                                                                                                                                                        | Size |
| --- | ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| F1  | Developer tooling   | Three developer-tool test suites exist and no orchestrator runs any of them: `contrib/dev-tools/git/tests/test-merge-pull-request.sh`, `contrib/dev-tools/checks/tests/test-format-project-words.sh`, `contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh`. `grep -rn "contrib/dev-tools" .github/workflows/` returns only the container persistence test and the hook installer; the suites appear in no `STEPS` entry of `contrib/dev-tools/git/hooks/pre-commit.sh` (lines 51-58) or `pre-push.sh`. Two scripts state the gap themselves: `contrib/dev-tools/checks/tests/test-format-project-words.sh:6` and `contrib/dev-tools/checks/format-project-words.sh:6`. | Add one lightweight CI step that runs the three suites, and delete the now-untrue "not automatically run" notes. Keep the step outside the expensive test matrix; the three suites together complete in roughly one second on a warm host. | S    |
| F2  | Issue lifecycle     | Sixteen specification directories under `docs/issues/open/` belong to issues GitHub reports as `CLOSED`: 1586, 1588, 2121, 2122, 2130, 2132, 2134, 2136, 2138, 2140, 2150, 2151, 2155, 2156, 2160, 2162. Verified by `gh issue view <n> --json state,closedAt` for every directory in `docs/issues/open/`. The archival rule is `docs/issues/closed/README.md`.                                                                                                       | Move the sixteen directories to `docs/issues/closed/` per the `cleanup-completed-issues` skill, repairing any live references the move breaks. Automation of this flow stays with #1774.                                              | M    |
| F3  | Workflow triggers   | `.github/workflows/container.yaml:16` triggers pushes on `releases/**/*`, while `.github/workflows/deployment.yaml:15` was narrowed to `releases/v*`. Package release branches match `releases/pkg/**` (`.github/workflows/deployment-packages.yaml:35`), so every package release push also starts the container workflow, which then extracts `pkg/<crate>/v<semver>` as its version, fails the semver test at `container.yaml:165`, prints `Not a valid release branch semver. Will Not Continue`, and exits 0. | Narrow `container.yaml:16` to `releases/v*`, matching `deployment.yaml`. The semver guard stays as the second line of defence.                                                                                                        | XS   |
| F4  | Documentation drift | `src/bin/` now contains only `http_health_check.rs`; `e2e_tests_runner`, `profiling`, and `qbittorrent_e2e_runner` moved to `packages/e2e-tools/src/bin/` in commit `c47173f53`. Five live references were never updated: `AGENTS.md:39`, `.github/skills/dev/testing/manual-http-download-completion-e2e/SKILL.md:21` and `:307`, `.github/skills/dev/testing/manual-udp-download-completion-e2e/SKILL.md:207`, `docs/profiling.md:10`. `docs/adrs/20260519000000_define_global_cli_output_contract.md:8` lists `src/bin/` as a related artifact and now under-describes the binary landscape. | Correct the five live references to `packages/e2e-tools/src/bin/`. Extend the ADR's related-artifacts list rather than rewriting the decision. Leave `docs/issues/closed/` untouched: those are immutable historical records. #2179 names this drift out of its own scope and worth its own issue. | S    |
| F5  | Test health         | `contrib/dev-tools/checks/tests/test-format-project-words.sh` exits 1 on `develop` for two independent reasons. First, `PROJECT_ROOT` at line 13 climbs three levels from `contrib/dev-tools/checks/tests/` and lands on `<repo>/contrib`, so every fixture copy reads `<repo>/contrib/contrib/dev-tools/...` and fails; the sibling suite at `contrib/dev-tools/git/tests/test-merge-pull-request.sh:6` climbs four from an equally deep directory and is correct. Commit `ffa2aa2c5` introduced the wrong depth when it moved the suite beside its sensor. Second, with the depth corrected the suite still fails: `create_fixture` (lines 21-28) never provisions `contrib/dev-tools/checks/lint-containerfile.sh`, so pre-commit step 5 of 6 reports `No such file or directory`, `commands.log` holds 3 entries where line 205 asserts 4, and the success banner asserted at line 206 is never printed. | Correct the `PROJECT_ROOT` depth to four levels, and provision the Containerfile lint sensor in the fixture with a stub that keeps the suite hermetic and free of any container runtime. Re-check the `commands.log` count assertion against whichever stub is chosen. | S    |
| F6  | Code readability    | `contrib/dev-tools/analysis/workspace-coupling/src/main.rs:191` writes an explicit generic argument naming Rust's character type — the five-character token spelled angle bracket, c-h-a-r, angle bracket. It is the only occurrence in the tracked tree. Chat and review transport layers substitute that token with a role or account name, so the line cannot be quoted in a review discussion without arriving as text that would not compile, which makes a reviewer reasonably distrust the quoted artifact. | Take the character by value — `fn is_rust_identifier_char(ch: char) -> bool` — and move the optionality to the two call sites at line 187, which already hold values from `chars().next_back()` and `chars().next()` and can use `is_some_and`. Behaviour is unchanged: an absent adjacent character still reads as "not an identifier character". Where a token-free signature is genuinely impossible, a local type alias or inference through `collect()` serves the same purpose. | XS   |
| F7  | Dead comment        | `contrib/dev-tools/checks/lint-containerfile.sh:4` reads `Tests: (no automated tests yet — EPIC #2003)`. Once F5 and F1 land, the fixture for that sensor exists and is exercised, so the note becomes untrue in the same pull request that makes it so.                                                                                                                                                       | Update the note to point at the fixture that covers the sensor, or delete it. Handle it in the same commit as F1 so the tree is never internally inconsistent.                                                                        | XS   |
| F8  | Documentation drift | `docs/issues/open/2150-add-lychee-link-checker/ISSUE.md:93` records the decision not to add a `.lycheeignore` file, and `project-words.txt:306` still carries `lycheeignore` as a dictionary entry. The file does not exist and by that decision never will.                                                                                                                                                   | Confirm no remaining prose needs the word, then drop the dictionary entry. Keep it if the closed specification's own text still requires it — the dictionary serves the documents, not the other way round.                            | XS   |

## Maintainer Actions

These two items cannot be performed from a pull request. They are recorded here so they are visible and so the pull request does not silently leave them undone.

| ID  | Item                                             | Evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Action owner and change                                                                                                                                                                                                                                                                                        |
| --- | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A1  | Allowed-actions allowlist pins an exact patch    | `gh api repos/torrust/torrust-tracker/actions/permissions/selected-actions` returns `taiki-e/install-action@v2.87.2` among `patterns_allowed`. Three workflows use that action (`.github/workflows/testing.yaml:80`, `coverage.yaml:51`, `generate_coverage_pr.yaml:46`), so no Dependabot bump of it can run. On PR #2180 (`2.87.2` to `2.87.6`, head `a90c3bb1`), `gh api "…/actions/runs?head_sha=a90c3bb1…"` shows `Testing` (push), `Testing` (pull_request), and `Generate Coverage Report (PR)` all `startup_failure`, while `gh pr checks 2180` lists 18 rows that are all `pass` or `skipping` — the failures are invisible in the ordinary check view. The repository's own skill already prescribes the fix at `.github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md:49`. | Repository administrator: change the allowlist entry to `taiki-e/install-action@v2.*`, as that skill directs. No file in this repository changes. Note that the permissions endpoint is readable with ordinary access, so the current value can be confirmed before and after.                                       |
| A2  | Two Dependabot pull requests stalled since August | #2055 (`base64` 0.22.1 to 0.23.1, opened 2026-08-07) and #2106 (`syn` 2.0.119 to 3.0.4, opened 2026-08-27). `gh api repos/torrust/torrust-tracker/compare/<head>...develop` puts `develop` 263 and 137 commits ahead respectively; both report `mergeStateStatus: UNSTABLE`, and on each the only failing check is `Docker E2E` — the symptom #2179 diagnoses and fixes.                                                                                                                                                                                            | Maintainer: after #2179 merges, rebase both and re-run, or close them if the major-version bumps are unwanted on their own merits. This issue does not touch either branch and does not duplicate #2179's fix.                                                                                                     |

## Related but Separate

`Configuration::load` runs the strict Figment extract at `packages/configuration/src/v3_0_0/mod.rs:392` before checking `metadata.schema_version` at line 395. Because every configuration section carries `#[serde(deny_unknown_fields)]`, a version-2 configuration file fails on the first section name the version-3 schema does not know — reported as an unknown-field error naming `tracker` — instead of the `Error::UnsupportedVersion` that exists precisely to explain this case. The user sees a spelling complaint where the real problem is that the file is a schema version behind.

This is a behavioural change to a user-facing error path, not a maintenance clean-up: it needs its own acceptance criteria, its own regression test, and a decision about whether the version probe reads the metadata section separately or whether the extract is relaxed. It is recorded here because it was found while verifying this batch, and it is recommended for its own issue rather than folded into this one.

## Dropped Candidates

Two items proposed for this batch did not reproduce and are recorded so the ground is not re-covered.

| Candidate                                                              | Finding                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Broken or redirected local links                                       | `linter lychee` exits 0 on `develop` with `All local Markdown links passed checking!` in 0.137 s. Local link health is already enforced by `.github/workflows/docs-lint.yaml:60`. External links are wholly owned by #2185, whose specification is under review in #2186. Nothing is left for this issue. The `.lycheeignore` file named in the original report does not exist and, per #2150, will not: the configuration lives in `lychee.toml` and `.github/lychee-online.toml`. |
| A second never-run merge-tool suite, `test-github-merge-symlinks.py`       | No such file exists on `develop`. `contrib/dev-tools/git/tests/` contains only `test-merge-pull-request.sh`. The Python suite belongs to the unmerged symlink-exceptions work for #2175 and will arrive with it; F1 covers the suites that exist today.                                                                                                                          |

## Architectural Decisions

No architectural decision is expected. Every item is a mechanical correction to an existing decision's implementation, or the recording of an action outside this repository.

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (F4 touches only its related-artifacts list, not the decision)
- ADRs to create: `None known`

If F1's CI placement turns into a real choice about where developer-tool tests belong rather than a one-step addition, stop and raise it against EPIC #2003 rather than settling it inside a clean-up pull request.

## Design and Ownership Review

`Not applicable`. No item involves child processes, asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures beyond the existing shell fixture repaired in F5, which creates and removes its own temporary directory under an `EXIT` trap that already works.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                        | Notes / Expected Output                                                                                                                                                              |
| --- | ------ | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Repair the dictionary-formatter test suite (F5)             | `bash contrib/dev-tools/checks/tests/test-format-project-words.sh` exits 0 from a clean checkout; both defects fixed; no container runtime required.                                  |
| T2  | TODO   | Run the three developer-tool suites in CI (F1, F7)          | One CI step invokes all three suites and fails the workflow when any fails; the "not automatically run" notes and the dead `Tests:` note are removed or corrected in the same change. |
| T3  | TODO   | Narrow the container workflow trigger (F3)                  | `container.yaml:16` reads `releases/v*`; a push to a `releases/pkg/**` branch no longer starts the workflow.                                                                          |
| T4  | TODO   | Correct the stale binary-path references (F4)               | The five live references name `packages/e2e-tools/src/bin/`; the ADR's related-artifacts list is extended; `docs/issues/closed/` is untouched.                                        |
| T5  | TODO   | Remove the unquotable explicit generic (F6)                 | The tracked tree contains no occurrence of the token; `cargo clippy` and the tool's own behaviour are unchanged.                                                                      |
| T6  | TODO   | Archive the sixteen closed issue specifications (F2)        | The sixteen directories are under `docs/issues/closed/`; `linter lychee` still exits 0 after the move.                                                                                |
| T7  | TODO   | Retire the stale dictionary entry (F8)                      | `linter cspell` exits 0 with the entry removed, or the entry is kept with a one-line justification in the progress log.                                                               |
| T8  | TODO   | Record the maintainer actions (A1, A2)                      | A1 and A2 are stated in the issue body so a maintainer can act; neither is attempted from the pull request.                                                                           |
| T9  | TODO   | Final verification and acceptance review                    | `linter all` exits 0, the pre-commit checks pass, manual scenarios are recorded, and every acceptance criterion is re-reviewed against observed behaviour.                            |

## Commit Points

| Task | Coherent change set                                                                              | Commit policy                                                                                                                    |
| ---- | ------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- |
| T1   | The two defects in the dictionary-formatter suite, with the suite green.                         | Commit once the suite exits 0 from a clean checkout. This lands first: T2 must not wire a red suite into CI.                       |
| T2   | The CI step plus the two now-untrue script notes.                                                 | Commit after the step is validated against the real workflow file. Depends on T1.                                                  |
| T3   | The one-line trigger narrowing.                                                                   | Commit on its own after workflow validation.                                                                                       |
| T4   | The five documentation and skill references plus the ADR artifact list.                           | Commit on its own; documentation-only and independently revertible.                                                                |
| T5   | The signature change and its two call sites.                                                      | Commit on its own after focused validation of the tool's output.                                                                   |
| T6   | The sixteen directory moves and any reference repairs they force.                                 | Commit on its own. It is a large diff of pure moves; mixing it with anything else would bury the other changes.                     |
| T7   | The dictionary entry.                                                                             | Fold into T4 if it turns out to be a one-word edit; commit separately if the word survives and the decision needs its own message. |
| T8   | No repository change.                                                                             | Record in the progress log as a justified no-change decision; do not create an empty commit.                                       |
| T9   | Completion evidence.                                                                              | Keep separate from the fixes so the verification record is reviewable on its own.                                                  |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/maintenance-frictions-cleanup/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
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

- 2026-09-09 14:11 UTC - Specification author - Drafted from a re-verification of every candidate friction against `develop` at `f6b73e29`; eight in-scope items, two maintainer actions, one related-but-separate behavioural defect, two dropped candidates - evidence recorded inline in the Friction Inventory, Maintainer Actions, and Dropped Candidates tables

## Acceptance Criteria

- [ ] AC1: Each of F1 through F8 is either fixed in this issue's pull request, or explicitly deferred with a linked issue recorded in the progress log. No item is left silently undone.
- [ ] AC2: `bash contrib/dev-tools/checks/tests/test-format-project-words.sh` exits 0 from a clean checkout, on a host with no container runtime available.
- [ ] AC3: All three developer-tool suites run in CI, and a deliberately broken suite fails the workflow. No script in `contrib/dev-tools/` still claims its tests are not automatically run when they are.
- [ ] AC4: A push to a branch matching `releases/pkg/**` starts no run of the container workflow.
- [ ] AC5: No live document, skill, or workflow places `e2e_tests_runner`, `profiling`, or `qbittorrent_e2e_runner` under `src/bin/`. Records under `docs/issues/closed/` are unchanged.
- [ ] AC6: `docs/issues/open/` contains a specification directory only for issues GitHub reports as `OPEN`, verified issue by issue at merge time.
- [ ] AC7: The tracked tree contains no occurrence of the explicit character-type generic described in F6, and `contrib/dev-tools/analysis/workspace-coupling` produces the same output as before the change.
- [ ] AC8: The issue body states maintainer actions A1 and A2 clearly enough for a maintainer to act without re-deriving the evidence.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `bash contrib/dev-tools/git/hooks/pre-commit.sh`
- The three developer-tool suites, individually and through the new CI step
- `linter lychee`, specifically after the F2 archival moves, to catch references the moves break

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                    | Human-oriented command/steps                                                                                                                                          | Expected Result                                                                                                     | Status | Evidence                                     |
| --- | ------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Repaired suite passes from a clean checkout | Clone the branch into a fresh directory on a host with no container runtime, then run `bash contrib/dev-tools/checks/tests/test-format-project-words.sh` and echo `$?` | Exit code 0 and the line `All formatter and pre-commit hook tests passed.`                                             | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | The new CI step actually catches a failure  | On a scratch branch, break one assertion in one suite, push, and read the workflow run                                                                                 | The workflow fails and names the broken suite. Revert the scratch branch afterwards                                    | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Package release branches stay quiet         | Push a throwaway branch named `releases/pkg/scratch/v0.0.1`, then list workflow runs for its head with `gh api "…/actions/runs?head_sha=<sha>"`, then delete the branch | No `Container` run appears. Before the fix, one appears and exits 0 at the semver guard                                 | TODO   | `manual-verification-evidence.md` section V3 |
| M4  | Corrected paths point at real files         | For each reference changed in F4, read the file at the path it now names                                                                                               | Every path exists. `docs/profiling.md` still describes a runnable profiling flow                                       | TODO   | `manual-verification-evidence.md` section V4 |
| M5  | The workspace-coupling tool is unchanged    | Run the tool on `develop` and on the branch, and compare the two reports                                                                                               | The reports are identical apart from any embedded timestamp                                                            | TODO   | `manual-verification-evidence.md` section V5 |
| M6  | Archived specifications are all closed      | For every directory left in `docs/issues/open/`, run `gh issue view <n> --json state`                                                                                  | Every remaining directory maps to an `OPEN` issue, and every moved one to a `CLOSED` issue                              | TODO   | `manual-verification-evidence.md` section V6 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- M3 requires pushing and deleting a branch in a repository whose workflows react to `releases/**`. Run it on a fork unless a maintainer agrees otherwise, and record which repository was used.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None is planned. Every scenario above is a direct human-oriented invocation of tooling that already exists, and F5's repair makes the one relevant fixture a maintained suite rather than a throwaway. If implementation nonetheless calls for a temporary script, record its issue-local path, why a maintained Rust automatic test is unsuitable for that specific scenario, its removal or retention owner, and — if written in Python — why Rust does not suit that script, before creating it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {PR link}          |
| AC2   | TODO                   | M1                 |
| AC3   | TODO                   | M2                 |
| AC4   | TODO                   | M3                 |
| AC5   | TODO                   | M4                 |
| AC6   | TODO                   | M6                 |
| AC7   | TODO                   | M5                 |
| AC8   | TODO                   | {issue body link}  |

## Risks and Trade-offs

- Batching unrelated fixes can hide a bad one inside a large diff. Mitigation: one commit per friction, in the order set by [Commit Points](#commit-points), so a reviewer can reject a single item without unpicking the rest. F2's sixteen directory moves stay in their own commit for the same reason.
- The F2 archival is a snapshot. Issues close while the pull request is open, so the list can be stale by the time it merges. Mitigation: AC6 re-verifies every remaining directory against GitHub at merge time rather than trusting the list drafted here.
- Wiring the three suites into CI makes a previously invisible failure blocking. That is the point of F1, but it means F5 must land first, and it means a future breakage in those suites will stop a merge. Mitigation: the suites are fast and hermetic, and the CI step is a separate lightweight job so a failure is easy to read and does not consume the test matrix.
- Fixing F6 by changing a function signature touches behaviour-adjacent code in a clean-up pull request. Mitigation: M5 compares the tool's output before and after; if the comparison is anything but identical, defer the item to its own issue.
- F1's placement could turn out to be a design question about the check harness rather than a one-step addition. Mitigation: the stop condition is written into [Architectural Decisions](#architectural-decisions) — raise it against EPIC #2003 rather than deciding it here.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if any friction turns out to be larger than its recorded size, if F1's CI placement becomes a design decision, or if the F6 output comparison is not identical.
- If none of those occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #1774 (archival automation), #2003 (guardrails and automation EPIC), #2159 (folder-style spec adoption), #2179 (Docker E2E package flag), #2185 (external-link triage)
- Related PRs: #2055, #2106, #2180, #2186
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
