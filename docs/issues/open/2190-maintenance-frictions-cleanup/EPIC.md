---
doc-type: epic
issue-type: task
status: planned
priority: p2
github-issue: 2190
spec-path: docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
epic-owner: null
branch: "2190-maintenance-frictions-cleanup-spec"
related-pr: null
last-updated-utc: 2026-09-10 09:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/templates/EPIC.md
    - docs/issues/open/README.md
    - docs/issues/drafts/README.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - AGENTS.md
    - .github/workflows/container.yaml
    - contrib/dev-tools/checks/tests/test-format-project-words.sh
    - contrib/dev-tools/analysis/workspace-coupling/src/main.rs
---

<!-- skill-link: create-issue -->

# EPIC #2190 - Repository maintenance frictions clean-up

## Goal

Turn a verified inventory of small repository maintenance frictions into a set of domain-clustered, independently reviewable subissues, so each friction is fixed in its own small pull request and under the EPIC that already owns its domain. This EPIC is planning-only: it changes no code, configuration, or documentation outside its own specification and the subissue specifications it produces.

## Why This Is Needed

Routine work across this repository keeps running into the same set of small defects: a developer-tool test suite that has been red on `develop` since it was moved, three such suites that no orchestrator runs at all, sixteen issue specifications for closed issues still sitting in the open backlog, a workflow trigger that starts a job on branches it immediately refuses, and documentation that still points at binaries which have since moved into a package. None of these is large enough to have justified its own issue when it was found, and none of them blocks a feature, so each has survived several release cycles. Individually they are noise; collectively they are a standing tax on every contributor and agent who reads a stale path, re-diagnoses a red test, or scrolls past a failed workflow run that was never going to succeed.

The first version of this specification proposed fixing all of them in one pull request. That shape has two costs. A single pull request touching workflows, shell fixtures, Rust source, documentation, and sixteen directory moves is hard to review and hard to revert item by item, and the reviewing effort grows faster than the size of the diff. It also discards domain context: several of the frictions sit squarely inside the scope of an EPIC that already exists, and fixing them elsewhere hides them from the EPIC that will later have to reason about the same surface. Clustering by domain keeps each fix next to the work that shares its context, which is easier for a human reviewer to hold in mind and easier for an agent to work through with the parent EPIC as its context.

The inventory itself is still worth keeping. Each entry was verified against the tree rather than reported, and that evidence is what makes the resulting subissues cheap to write and cheap to accept. This EPIC preserves that evidence and spends it on subissue specifications instead of on one large change.

## Scope

### In Scope

- Preserve the verified [Friction Inventory](#friction-inventory), [Maintainer Actions](#maintainer-actions), [Related but Separate](#related-but-separate), and [Dropped Candidates](#dropped-candidates) evidence as the durable record behind every subissue this EPIC produces.
- Cluster every inventory item by domain and record, for each cluster, the EPIC that owns it and the reason that EPIC owns it.
- Produce one draft subissue specification per cluster item under `docs/issues/drafts/`, each sized for one focused pull request and each naming its parent EPIC.
- Create the GitHub subissues from the approved drafts, link them under their parent EPIC, and move each specification into `docs/issues/open/` under the naming convention.
- Hand the two clusters that belong to EPIC #2003 to that EPIC, including the row each needs in its subissue table.
- Track completion of the subissues this EPIC owns until every inventory item is delivered or explicitly closed as won't-fix.

### Out of Scope

- Fixing any friction in this EPIC's own pull request. Every code, configuration, and documentation change belongs to a subissue and is reviewed there.
- Re-verifying the inventory. It was verified against `develop` at revision `f6b73e29` on 2026-09-09; a subissue re-checks its own item at implementation time, and a stale entry is corrected in that subissue rather than here.
- Automating the issue-specification archival flow. That is #1774; the archival subissue performs the current archival by hand and does not constrain the script's design.
- Triaging external-link check findings. That is #2185, whose specification is under review in #2186.
- Repairing the `Docker E2E` job in `.github/workflows/testing.yaml`. That is #2179; this EPIC only records which stale pull requests are blocked behind it.
- Migrating legacy single-file specifications to the folder-style layout. That is #2159.
- Designing the long-term check harness and sensor architecture. That is EPIC #2003; the two clusters handed to it add and repair invocations of checks that already exist and do not prejudge that design.
- Any behavioural change to the tracker itself. The one behavioural defect found while verifying the inventory is recorded in [Related but Separate](#related-but-separate) and becomes a standalone issue with its own acceptance criteria.

## Friction Inventory

This is the verified evidence behind the subissues. Each entry was re-verified against `develop` at revision `f6b73e29` on 2026-09-09; the evidence column records what was observed rather than what was reported. The proposed fix is the starting point for the subissue that owns the item, not a decision binding on it.

Size is the expected reviewable weight of the change: `XS` is a one-line edit, `S` is a handful of lines in one file, `M` spans several files or needs a judgement call. The `Cluster` column links each item to its parent in [Clusters](#clusters).

| ID  | Cluster | Area                | Evidence (verified at `f6b73e29`, 2026-09-09)                                                                                                                                                                                                                                                                          | Proposed fix                                                                                                                                                                                                                        | Size |
| --- | ------- | ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| F1  | C2      | Developer tooling   | Three developer-tool test suites exist and no orchestrator runs any of them: `contrib/dev-tools/git/tests/test-merge-pull-request.sh`, `contrib/dev-tools/checks/tests/test-format-project-words.sh`, `contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh`. `grep -rn "contrib/dev-tools" .github/workflows/` returns only the container persistence test and the hook installer; the suites appear in no `STEPS` entry of `contrib/dev-tools/git/hooks/pre-commit.sh` (lines 51-58) or `pre-push.sh`. Two scripts state the gap themselves: `contrib/dev-tools/checks/tests/test-format-project-words.sh:6` and `contrib/dev-tools/checks/format-project-words.sh:6`. | Add one lightweight CI step that runs the three suites, and delete the now-untrue "not automatically run" notes. Keep the step outside the expensive test matrix; the three suites together complete in roughly one second on a warm host. | S    |
| F2  | C5      | Issue lifecycle     | Sixteen specification directories under `docs/issues/open/` belong to issues GitHub reports as `CLOSED`: 1586, 1588, 2121, 2122, 2130, 2132, 2134, 2136, 2138, 2140, 2150, 2151, 2155, 2156, 2160, 2162. Verified by `gh issue view <n> --json state,closedAt` for every directory in `docs/issues/open/`. The archival rule is `docs/issues/closed/README.md`.                                                                                                       | Move the sixteen directories to `docs/issues/closed/` per the `cleanup-completed-issues` skill, repairing any live references the move breaks. Automation of this flow stays with #1774.                                              | M    |
| F3  | C3      | Workflow triggers   | `.github/workflows/container.yaml:16` triggers pushes on `releases/**/*`, while `.github/workflows/deployment.yaml:15` was narrowed to `releases/v*`. Package release branches match `releases/pkg/**` (`.github/workflows/deployment-packages.yaml:35`), so every package release push also starts the container workflow, which then extracts `pkg/<crate>/v<semver>` as its version, fails the semver test at `container.yaml:165`, prints `Not a valid release branch semver. Will Not Continue`, and exits 0. | Narrow `container.yaml:16` to `releases/v*`, matching `deployment.yaml`. The semver guard stays as the second line of defence.                                                                                                        | XS   |
| F4  | C4      | Documentation drift | `src/bin/` now contains only `http_health_check.rs`; `e2e_tests_runner`, `profiling`, and `qbittorrent_e2e_runner` moved to `packages/e2e-tools/src/bin/` in commit `c47173f53`. Five live references were never updated: `AGENTS.md:39`, `.github/skills/dev/testing/manual-http-download-completion-e2e/SKILL.md:21` and `:307`, `.github/skills/dev/testing/manual-udp-download-completion-e2e/SKILL.md:207`, `docs/profiling.md:10`. `docs/adrs/20260519000000_define_global_cli_output_contract.md:8` lists `src/bin/` as a related artifact and now under-describes the binary landscape. | Correct the five live references to `packages/e2e-tools/src/bin/`. Extend the ADR's related-artifacts list rather than rewriting the decision. Leave `docs/issues/closed/` untouched: those are immutable historical records. #2179 names this drift out of its own scope and worth its own issue. | S    |
| F5  | C1      | Test health         | `contrib/dev-tools/checks/tests/test-format-project-words.sh` exits 1 on `develop` for two independent reasons. First, `PROJECT_ROOT` at line 13 climbs three levels from `contrib/dev-tools/checks/tests/` and lands on `<repo>/contrib`, so every fixture copy reads `<repo>/contrib/contrib/dev-tools/...` and fails; the sibling suite at `contrib/dev-tools/git/tests/test-merge-pull-request.sh:6` climbs four from an equally deep directory and is correct. Commit `ffa2aa2c5` introduced the wrong depth when it moved the suite beside its sensor. Second, with the depth corrected the suite still fails: `create_fixture` (lines 21-28) never provisions `contrib/dev-tools/checks/lint-containerfile.sh`, so pre-commit step 5 of 6 reports `No such file or directory`, `commands.log` holds 3 entries where line 205 asserts 4, and the success banner asserted at line 206 is never printed. | Correct the `PROJECT_ROOT` depth to four levels, and provision the Containerfile lint sensor in the fixture with a stub that keeps the suite hermetic and free of any container runtime. Re-check the `commands.log` count assertion against whichever stub is chosen. | S    |
| F6  | C6      | Code readability    | `contrib/dev-tools/analysis/workspace-coupling/src/main.rs:191` writes an explicit generic argument naming Rust's character type — the five-character token spelled angle bracket, c-h-a-r, angle bracket. It is the only occurrence in the tracked tree. Chat and review transport layers substitute that token with a role or account name, so the line cannot be quoted in a review discussion without arriving as text that would not compile, which makes a reviewer reasonably distrust the quoted artifact. | Take the character by value — `fn is_rust_identifier_char(ch: char) -> bool` — and move the optionality to the two call sites at line 187, which already hold values from `chars().next_back()` and `chars().next()` and can use `is_some_and`. Behaviour is unchanged: an absent adjacent character still reads as "not an identifier character". Where a token-free signature is genuinely impossible, a local type alias or inference through `collect()` serves the same purpose. | XS   |
| F7  | C2      | Dead comment        | `contrib/dev-tools/checks/lint-containerfile.sh:4` reads `Tests: (no automated tests yet — EPIC #2003)`. Once F5 and F1 land, the fixture for that sensor exists and is exercised, so the note becomes untrue in the same pull request that makes it so.                                                                                                                                                       | Update the note to point at the fixture that covers the sensor, or delete it. Handle it in the same commit as F1 so the tree is never internally inconsistent.                                                                        | XS   |
| F8  | C4      | Documentation drift | `docs/issues/open/2150-add-lychee-link-checker/ISSUE.md:93` records the decision not to add a `.lycheeignore` file, and `project-words.txt:306` still carries `lycheeignore` as a dictionary entry. The file does not exist and by that decision never will.                                                                                                                                                   | Confirm no remaining prose needs the word, then drop the dictionary entry. Keep it if the closed specification's own text still requires it — the dictionary serves the documents, not the other way round.                            | XS   |

## Maintainer Actions

These two items cannot be performed from a pull request; both are cluster C7 and each becomes a subissue that changes no file in this repository. The evidence is kept here so the subissue specifications do not have to re-derive it.

| ID  | Item                                             | Evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Action owner and change                                                                                                                                                                                                                                                                                        |
| --- | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A1  | Allowed-actions allowlist pins an exact patch    | `gh api repos/torrust/torrust-tracker/actions/permissions/selected-actions` returns `taiki-e/install-action@v2.87.2` among `patterns_allowed`. Three workflows use that action (`.github/workflows/testing.yaml:80`, `coverage.yaml:51`, `generate_coverage_pr.yaml:46`), so no Dependabot bump of it can run. On PR #2180 (`2.87.2` to `2.87.6`, head `a90c3bb1`), `gh api "…/actions/runs?head_sha=a90c3bb1…"` shows `Testing` (push), `Testing` (pull_request), and `Generate Coverage Report (PR)` all `startup_failure`, while `gh pr checks 2180` lists 18 rows that are all `pass` or `skipping` — the failures are invisible in the ordinary check view. The repository's own skill already prescribes the fix at `.github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md:49`. | Repository administrator: change the allowlist entry to `taiki-e/install-action@v2.*`, as that skill directs. No file in this repository changes. Note that the permissions endpoint is readable with ordinary access, so the current value can be confirmed before and after.                                       |
| A2  | Two Dependabot pull requests stalled since August | #2055 (`base64` 0.22.1 to 0.23.1, opened 2026-08-07) and #2106 (`syn` 2.0.119 to 3.0.4, opened 2026-08-27). `gh api repos/torrust/torrust-tracker/compare/<head>...develop` puts `develop` 263 and 137 commits ahead respectively; both report `mergeStateStatus: UNSTABLE`, and on each the only failing check is `Docker E2E` — the symptom #2179 diagnoses and fixes.                                                                                                                                                                                            | Maintainer: after #2179 merges, rebase both and re-run, or close them if the major-version bumps are unwanted on their own merits. This issue does not touch either branch and does not duplicate #2179's fix.                                                                                                     |

## Related but Separate

`Configuration::load` runs the strict Figment extract at `packages/configuration/src/v3_0_0/mod.rs:392` before checking `metadata.schema_version` at line 395. Because every configuration section carries `#[serde(deny_unknown_fields)]`, a version-2 configuration file fails on the first section name the version-3 schema does not know — reported as an unknown-field error naming `tracker` — instead of the `Error::UnsupportedVersion` that exists precisely to explain this case. The user sees a spelling complaint where the real problem is that the file is a schema version behind.

This is a behavioural change to a user-facing error path, not a maintenance clean-up: it needs its own acceptance criteria, its own regression test, and a decision about whether the version probe reads the metadata section separately or whether the extract is relaxed. It is recorded here because it was found while verifying this inventory. It is cluster C8, item `R1`: a standalone issue with no parent EPIC, drafted at `docs/issues/drafts/configuration-schema-version-error-ordering/ISSUE.md`. The configuration overhaul EPIC that would have owned it, #1978, is closed.

## Dropped Candidates

Two items proposed for this inventory did not reproduce. They produce no subissue and are recorded so the same ground is not re-covered.

| Candidate                                                              | Finding                                                                                                                                                                                                                                                                                                                                                                        |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Broken or redirected local links                                       | `linter lychee` exits 0 on `develop` with `All local Markdown links passed checking!` in 0.137 s. Local link health is already enforced by `.github/workflows/docs-lint.yaml:60`. External links are wholly owned by #2185, whose specification is under review in #2186. Nothing is left for this issue. The `.lycheeignore` file named in the original report does not exist and, per #2150, will not: the configuration lives in `lychee.toml` and `.github/lychee-online.toml`. |
| A second never-run merge-tool suite, `test-github-merge-symlinks.py`       | No such file exists on `develop`. `contrib/dev-tools/git/tests/` contains only `test-merge-pull-request.sh`. The Python suite belongs to the unmerged symlink-exceptions work for #2175 and will arrive with it; F1 covers the suites that exist today.                                                                                                                          |

## Clusters

Each inventory item belongs to exactly one cluster, and each cluster to exactly one parent. A cluster is a domain — one reviewer context, one surface, one body of prior decisions — not a size bucket. Two clusters were adopted by an EPIC that already exists; the rest stay under this EPIC because no existing EPIC's stated scope covers them.

| Cluster | Items      | Domain                                                    | Parent                                       | Draft specification                                                                   |
| ------- | ---------- | --------------------------------------------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------- |
| C1      | F5         | Developer-tool checks: repairing an existing check        | EPIC #2003                                   | `docs/issues/drafts/2003-repair-project-dictionary-formatter-test-suite/ISSUE.md`      |
| C2      | F1, F7     | Developer-tool checks: invoking existing checks from CI   | EPIC #2003                                   | `docs/issues/drafts/2003-run-developer-tool-test-suites-in-ci/ISSUE.md`                |
| C3      | F3         | Workflow trigger correctness                              | EPIC #2190                                   | `docs/issues/drafts/2190-narrow-container-workflow-release-branch-trigger/ISSUE.md`    |
| C4      | F4, F8     | Stale references left behind by completed moves and decisions | EPIC #2190                               | `docs/issues/drafts/2190-correct-stale-documentation-references/ISSUE.md`              |
| C5      | F2         | Issue-backlog archival                                    | EPIC #2190                                   | `docs/issues/drafts/2190-archive-closed-issue-specifications/ISSUE.md`                 |
| C6      | F6         | Source hygiene for quotable artifacts                     | EPIC #2190                                   | `docs/issues/drafts/2190-remove-unquotable-character-type-generic/ISSUE.md`            |
| C7      | A1, A2     | Repository administration; no repository change           | EPIC #2190                                   | `docs/issues/drafts/2190-widen-workflow-actions-allowlist-entry/ISSUE.md`, `docs/issues/drafts/2190-resolve-stalled-dependency-update-pull-requests/ISSUE.md` |
| C8      | R1         | Configuration error reporting                             | None; standalone issue                       | `docs/issues/drafts/configuration-schema-version-error-ordering/ISSUE.md`              |

### Why Each Parent

**C1 and C2 belong to EPIC #2003 - Overhaul: Automation Tools and AI Agent Guardrails.** Its In Scope list covers exactly this surface: it undertakes to "Evaluate check placement across pre-commit, pre-push, CI, and any future repository-policy or architecture-check category", and it explicitly permits "the narrowly scoped interim formatter described by [`2019-automatically-format-project-dictionary`](../../closed/2019-automatically-format-project-dictionary/ISSUE.md)". F5 repairs that permitted formatter's own test suite, and C2 wires that suite and its two siblings into CI. Both also meet #2003's stated exception for work that may proceed before its architecture decision: "low-risk, additive, independently verifiable documentation, skill, profile, template, and focused validation subissues ... when they do not select or depend on a shared runner, cache, enforcement platform, external workflow tool, or broad consumer migration". Neither cluster selects a runner shape; each adds one invocation of a check that already exists, and both remain replaceable by the later design. F7 rides with C2 rather than standing alone because the dead note it removes — `contrib/dev-tools/checks/lint-containerfile.sh:4`, which reads that the sensor has no automated tests yet and names EPIC #2003 — only becomes untrue once C2 lands; splitting them would leave the tree self-contradicting between two pull requests.

**C3 stays under this EPIC.** The only EPIC that owns `.github/workflows/container.yaml` is #1840 - Improve PR Workflow Performance, and its goal is stated in time: "Reduce the execution time of the critical PR validation workflows ... so maintainers and contributors can get faster feedback". F3 changes no workflow's runtime and no pull request's wait time. The spurious run it prevents is started by pushes to package release branches, which are not on the pull request critical path at all, and #1840's Out of Scope explicitly excludes "Optimizing unrelated workflows unless they directly affect these two critical paths". F3 is a trigger-predicate defect, not a performance item.

**C4 stays under this EPIC.** F4's stale paths were created by a package move, which invites EPIC #1669 - Overhaul: Packages, but that EPIC's documentation duty is specific: its audit covers `docs/packages.md`, `packages/AGENTS.md`, the extracted-packages tables, and the dependency diagram at `docs/media/packages/dependencies-workspace-packages.md`. None of F4's five live references is one of those; they are in the testing section of `AGENTS.md`, two testing skills, and `docs/profiling.md`. F8's stale dictionary entry has no package dimension at all. What F4 and F8 share is the shape of the defect rather than the file: both name a referent that no longer exists or never will, one because the binaries moved and one because #2150 decided the file would not be created. That is one reviewer context and one small pull request.

**C5 stays under this EPIC.** The archival flow's automation is #1774, which is a paused dependency of EPIC #2003 and blocked on that EPIC's architecture decision. Performing the current archival by hand is not automation work and must not inherit that block: it implements no tool, selects no interface, and constrains nothing in #1774's design. Filing it under #2003 would either park it behind a decision it does not need or force an exception into that EPIC's scope for a one-off manual action.

**C6 stays under this EPIC.** The file it touches, `contrib/dev-tools/analysis/workspace-coupling/`, appears in EPIC #2003's related-artifacts list, but appearing as an artifact of an EPIC is not the same as falling inside its scope, and #2003's scope is the architecture of automation and guardrails rather than the source hygiene of any one tool. F6 changes one function signature so the line can survive being quoted in a review discussion; that concern is about how repository artifacts are reviewed, which is this EPIC's own domain.

**C7 stays under this EPIC as two subissues with no repository change.** A1 and A2 are administrative actions on the GitHub repository and its pull requests. They are subissues rather than notes so that each has an owner, an acceptance criterion, and a visible state, which a table row inside another issue's body does not give them.

**C8 has no parent.** The configuration error-ordering defect is a behavioural change to a user-facing error path in `packages/configuration`. The configuration overhaul EPIC that would have owned it, #1978, is closed. It needs its own acceptance criteria and its own regression test, and attaching it to a maintenance EPIC would misrepresent it as clean-up.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

These are the subissues this EPIC owns. Issue numbers are assigned when the drafts are approved and the GitHub issues are created; until then the draft path is the specification.

| Order | Issue                                                             | Local Spec                                                                                 | Status | Notes                                                                                                      |
| ----- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------ | ------ | ---------------------------------------------------------------------------------------------------------- |
| 1     | #[To be assigned] - Narrow the container workflow release trigger | `docs/issues/drafts/2190-narrow-container-workflow-release-branch-trigger/ISSUE.md`         | TODO   | C3. One-line trigger change; independent of every other subissue.                                          |
| 2     | #[To be assigned] - Correct stale documentation references        | `docs/issues/drafts/2190-correct-stale-documentation-references/ISSUE.md`                   | TODO   | C4. Documentation and dictionary only; independently revertible.                                           |
| 3     | #[To be assigned] - Archive closed issue specifications           | `docs/issues/drafts/2190-archive-closed-issue-specifications/ISSUE.md`                      | TODO   | C5. Large diff of pure moves; re-verify issue state at merge time. Does not constrain #1774.               |
| 4     | #[To be assigned] - Remove the unquotable character-type generic  | `docs/issues/drafts/2190-remove-unquotable-character-type-generic/ISSUE.md`                 | TODO   | C6. Signature change with a before/after output comparison.                                                |
| 5     | #[To be assigned] - Widen the workflow actions allowlist entry    | `docs/issues/drafts/2190-widen-workflow-actions-allowlist-entry/ISSUE.md`                   | TODO   | C7, A1. Repository administrator action; no file in this repository changes.                               |
| 6     | #[To be assigned] - Resolve the stalled dependency update PRs     | `docs/issues/drafts/2190-resolve-stalled-dependency-update-pull-requests/ISSUE.md`          | TODO   | C7, A2. Blocked until #2179 merges; no file in this repository changes.                                    |

## Issues Handed to Other Owners

These items leave this EPIC. Each draft names its own parent, and the parent EPIC's subissue table gains its row when the GitHub issue is created — not in this EPIC's pull request.

| Items  | Draft specification                                                                | New owner              | Ordering                                                                 |
| ------ | ---------------------------------------------------------------------------------- | ---------------------- | ------------------------------------------------------------------------ |
| F5     | `docs/issues/drafts/2003-repair-project-dictionary-formatter-test-suite/ISSUE.md`   | EPIC #2003             | First. The suite must be green before anything runs it in CI.            |
| F1, F7 | `docs/issues/drafts/2003-run-developer-tool-test-suites-in-ci/ISSUE.md`             | EPIC #2003             | After F5.                                                                |
| R1     | `docs/issues/drafts/configuration-schema-version-error-ordering/ISSUE.md`           | Standalone; no parent  | Independent.                                                             |

## Architectural Decisions

No architectural decision is expected from this EPIC. It produces specifications; every item is a mechanical correction to an existing decision's implementation, the recording of an action outside this repository, or — for C8 — a behavioural defect whose own issue carries its design question.

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md` (C4 touches only its related-artifacts list, not the decision)
- ADRs to create: `None known`

If C2's CI placement turns out to be a real choice about where developer-tool tests belong rather than a one-step addition, it is settled inside EPIC #2003's design rather than in a clean-up pull request. That is one of the reasons C1 and C2 move there.

## Delivery Strategy

The EPIC delivers specifications, not fixes. The order below reflects dependency and reviewer cost, not priority.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an issue-local retrospective for reusable lessons, material design changes, or meaningful deviations from the plan; otherwise record why one was unnecessary in the issue progress log.

### Phase 1: Clustering

- Outcome: every inventory item is assigned to a cluster and a parent, with the assignment argued from the parent's own stated scope.
- Exit criteria: the [Clusters](#clusters) tables are complete, a draft specification exists for every cluster item, and a maintainer has reviewed the clustering.

### Phase 2: Issue Creation

- Outcome: the approved drafts become GitHub issues, linked as subissues of their parent EPIC, with each specification moved from `docs/issues/drafts/` to `docs/issues/open/` under its assigned number.
- Exit criteria: every row in [Subissues](#subissues) and [Issues Handed to Other Owners](#issues-handed-to-other-owners) carries a real issue number, and EPIC #2003's subissue table carries the two rows it adopted.

### Phase 3: Delivery

- Outcome: each subissue is implemented and merged in its own pull request, in the ordering its cluster records.
- Exit criteria: every inventory item is delivered or explicitly closed as won't-fix with the reason recorded in this EPIC's progress log.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted and moved to `docs/issues/open/2190-maintenance-frictions-cleanup/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue [#2190](https://github.com/torrust/torrust-tracker/issues/2190) created and issue number added to this spec
- [x] Specification converted from a single implementation issue into a planning-only EPIC
- [ ] Clustering reviewed and approved by a maintainer
- [ ] Spec-only PR merged into `develop`
- [ ] GitHub issue #2190 converted to an EPIC issue: title, labels, and body updated to match this specification
- [ ] Subissues created from the approved drafts and linked under their parent EPIC
- [ ] Subissue specifications moved from `docs/issues/drafts/` to `docs/issues/open/`
- [ ] EPIC #2003's subissue table updated with the two adopted clusters
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-09 14:11 UTC - Specification author - Drafted from a re-verification of every candidate friction against `develop` at `f6b73e29`; eight in-scope items, two maintainer actions, one related-but-separate behavioural defect, two dropped candidates - evidence recorded inline in the Friction Inventory, Maintainer Actions, and Dropped Candidates tables
- 2026-09-09 15:39 UTC - Specification author - GitHub issue #2190 created from the reviewed draft; specification moved to `docs/issues/open/2190-maintenance-frictions-cleanup/ISSUE.md` - https://github.com/torrust/torrust-tracker/issues/2190
- 2026-09-10 09:30 UTC - Specification author - Restructured after the review on PR #2193 requesting an EPIC with domain-clustered subissues instead of one implementation issue: specification renamed to `EPIC.md` and made planning-only, the eight frictions and two maintainer actions clustered into eight clusters, two clusters handed to EPIC #2003, and nine draft subissue specifications added under `docs/issues/drafts/` - https://github.com/torrust/torrust-tracker/pull/2193#pullrequestreview-5164674289

## Acceptance Criteria

- [ ] AC1: Every item in [Friction Inventory](#friction-inventory) and [Maintainer Actions](#maintainer-actions), and the item in [Related but Separate](#related-but-separate), appears in exactly one row of [Clusters](#clusters) with a named parent.
- [ ] AC2: Each cluster's parent is justified from that parent's own stated scope, and the drafts adopted by another EPIC set `epic:` to that EPIC and name it below their title.
- [ ] AC3: Each draft subissue specification is sized for one focused pull request and carries its own acceptance criteria and verification plan.
- [ ] AC4: This EPIC's pull request changes no file outside `docs/issues/` and `docs/copilot-pr-reviews/`.
- [ ] AC5: Every row in [Subissues](#subissues) and [Issues Handed to Other Owners](#issues-handed-to-other-owners) carries a created GitHub issue number, linked as a subissue of its parent EPIC.
- [ ] AC6: Every inventory item is delivered by a merged subissue pull request, or closed as won't-fix with the reason recorded in the progress log.
- [ ] AC7: The [Dropped Candidates](#dropped-candidates) record survives the restructuring, so the same ground is not re-covered.
- [ ] `linter all` exits with code `0`
- [ ] Documentation and governance updates are included when required.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                   |
| ----- | ---------------------- | ---------------------------------------------------------- |
| AC1   | TODO                   | [Clusters](#clusters)                                      |
| AC2   | TODO                   | [Why Each Parent](#why-each-parent) and the draft frontmatter |
| AC3   | TODO                   | The draft specifications listed in [Clusters](#clusters)   |
| AC4   | TODO                   | {PR link}                                                  |
| AC5   | TODO                   | {issue links}                                              |
| AC6   | TODO                   | {PR links}                                                 |
| AC7   | TODO                   | [Dropped Candidates](#dropped-candidates)                  |

## Risks and Trade-offs

- Splitting eight frictions into six subissues plus two adoptions costs more issue-management overhead than one pull request would have. Mitigation: the split is by domain rather than by item, so the count stays close to the number of reviewer contexts involved rather than to the number of defects, and two clusters are absorbed by an EPIC that already has the context.
- Small subissues can stall before they are filed, leaving the inventory as documentation rather than work. Mitigation: AC5 and AC6 make the created issues and their delivery an acceptance condition of this EPIC, and the EPIC stays open until every item is delivered or closed as won't-fix.
- The inventory is a snapshot taken at `f6b73e29`. An item can be fixed or changed by unrelated work before its subissue is implemented. Mitigation: each subissue re-checks its own evidence at implementation time and records a no-change outcome rather than forcing a fix.
- The C5 archival list is the most perishable entry: issues close while a pull request is open. Mitigation: its subissue re-verifies every remaining directory against GitHub at merge time rather than trusting the list drafted here.
- Handing C1 and C2 to EPIC #2003 places them behind that EPIC's review attention, which is directed at an unfinished architecture decision. Mitigation: both clusters are filed under #2003's own exception for additive, independently verifiable work that may proceed before the decision, so neither waits on it.
- Two subissues change no file in this repository, which is an unusual shape for an issue here. Mitigation: their specifications state the no-change outcome explicitly and record a justified no-change decision in the progress log rather than producing an empty commit.

## References

- Related issues: #1774 (archival automation), #2003 (guardrails and automation EPIC), #2150 (lychee link checker), #2159 (folder-style spec adoption), #2179 (Docker E2E package flag), #2185 (external-link triage)
- Related PRs: #2055, #2106, #2180, #2186, #2193
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
