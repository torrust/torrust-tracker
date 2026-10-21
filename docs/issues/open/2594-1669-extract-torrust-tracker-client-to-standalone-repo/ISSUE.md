---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1669
github-issue: 2594
spec-path: docs/issues/open/2594-1669-extract-torrust-tracker-client-to-standalone-repo/ISSUE.md
branch: "2594-1669-extract-torrust-tracker-client-to-standalone-repo-spec"
related-pr: null
last-updated-utc: "2026-10-21 07:44"
semantic-links:
  skill-links:
    - create-issue
    - add-workspace-member
    - use-tracker-client
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/maintenance/add-workspace-member/SKILL.md
    - .github/skills/usage/use-tracker-client/SKILL.md
    - "issue #1669"
    - "issue #2583"
    - "issue #2576"
    - "issue #269"
    - console/tracker-client/Cargo.toml
    - packages/tracker-client/Cargo.toml
    - src/console/ci/e2e/tracker_checker.rs
    - Cargo.toml
    - .dockerignore
    - AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md
    - docs/issues/open/269-review-dependency-licenses/initial-review.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
---

<!-- skill-link: create-issue -->

# Issue #2594 - Extract the `torrust-tracker-client` console client to a standalone repository

Parent EPIC: #1669 (Overhaul: Packages).

## Goal

Move the console client crate `torrust-tracker-client` (`console/tracker-client`) out of the
tracker workspace into its own repository, publish it to crates.io from there, and leave the
tracker repository working without it. This is the last item of EPIC #1669: it starts only after
issue #2583 has published the three workspace crates the client depends on.

## Background

`console/tracker-client` is the crate `torrust-tracker-client` 0.1.0: a set of console clients for
BitTorrent trackers (`tracker_client`, `tracker_checker`, and the deprecated `udp_tracker_client`
and `http_tracker_client`), licensed `LGPL-3.0`, unlike the tracker's AGPL-3.0-only workspace. It
has its own documentation (CLI I/O contract, feature docs, local ADRs) because extraction was
always planned; its README says the code is being extracted from the tracker.

Moving it out lets the client follow its own release cadence and license, and makes it installable
with `cargo install` without the tracker source tree.

### Facts checked on 2026-10-21

- **No workspace crate depends on it.** `cargo metadata --no-deps` lists no dependent of
  `torrust-tracker-client`.
- **Its workspace dependencies are not on crates.io.** It depends by path on three workspace
  crates, and `https://crates.io/api/v1/crates/<name>` returns `404` for each:

  | Dependency                      | Folder                    | #2583 checklist row |
  | ------------------------------- | ------------------------- | ------------------- |
  | `torrust-tracker-udp-protocol`  | `packages/udp-protocol`   | 4                   |
  | `torrust-tracker-http-protocol` | `packages/http-protocol`  | 3                   |
  | `torrust-tracker-client-lib`    | `packages/tracker-client` | 8                   |

  The client imports `torrust-tracker-client-lib` under the renamed key `torrust-tracker-client`
  (library name `torrust_tracker_client`). Every other dependency is already on crates.io,
  including `bencode2json`, `torrust-peer-id` and `torrust-info-hash` (pinned `=0.2.0`). It no
  longer depends on `torrust-tracker-configuration`.
- **Names are free.** The crate name `torrust-tracker-client` returns `404` on crates.io, and the
  repository `torrust/torrust-tracker-client` does not exist.
- **The tracker repository uses the client.**
  - The E2E runner (`e2e_tests_runner` in `packages/e2e-tools`, run by `testing.yaml` and
    `container.yaml`) starts the checker with
    `cargo run -p torrust-tracker-client --bin tracker_checker`
    (`src/console/ci/e2e/tracker_checker.rs`). Deleting the crate breaks this CI step.
  - Six skills run `cargo run -p torrust-tracker-client --bin tracker_client`:
    `use-tracker-client`, `use-rest-api`, `run-tracker-locally`, `public-trackers-for-testing`,
    `manual-udp-download-completion-e2e`, `manual-torrent-cleanup-e2e`; and
    `release-new-version` lists `console/tracker-client/Cargo.toml` among the files to bump.
  - Root `Cargo.toml` lists `console/tracker-client` as a member; `.dockerignore` admits
    `/console/` only for it; `AGENTS.md` has a "Console tools" table for it.
  - Root ADRs `20260512102000` (peer ID convention) and `20260519000000` (global CLI output
    contract, which supersedes the client's local CLI ADR) name `console/tracker-client/` as a
    related artifact.
- **`torrust-tracker-client-lib` is also used by tracker crates**: `udp-server` (normal
  dependency; `udp::client::check` in its launcher and its `testing` environment), `test-helpers`
  (normal) and `axum-http-server` (dev). #2583 publishes it from this repository (row 8).
- **Precedent.** The earlier extractions (`torrust-clock`, `torrust-located-error`,
  `torrust-metrics`, `torrust-net-primitives`, `torrust-server-lib`) created
  `torrust/<crate>` repositories with a single `testing.yaml` (or `ci.yaml`) workflow; their
  repositories have only a handful of commits, so the source was copied rather than moved with
  its history.

### Blocked by #2583

The maintainer decided on 2026-10-20 that this issue is opened now, labelled `Blocked`, and is
blocked by #2583 (first publish of the unpublished workspace crates). #2583 publishes the crates
one at a time at the end of EPIC #1669, so this extraction is the EPIC's last item. It can start
when #2583's checklist rows 3, 4 and 8 are `PUBLISHED` (#2583's T6 hands over to this issue).

In #2583 the console client itself is row 13, `DEFERRED`: it is not published from the tracker
repository, and its first publish comes from its own repository in this issue.

## Maintainer Decisions

The maintainer answered these questions on 2026-10-21, when he approved this specification. The
IDs are referenced in the rest of the document.

| ID  | Question                         | Decision                                                                                                                                                                                                                                                  |
| --- | -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Q1  | Target repository                | `torrust/torrust-tracker-client`, created by an organisation admin.                                                                                                                                                                                      |
| Q2  | History                          | Plain copy, like the earlier extractions. The first commit in the new repository names the tracker commit it was copied from; the tracker history keeps the full record.                                                                              |
| Q3  | License                          | `LGPL-3.0-only` in `Cargo.toml`; the repository carries the LGPL-3.0 text and the GPL-3.0 text it amends, plus the MIT-0 legacy exception file; README links point to them.                                                                            |
| Q4  | CI in the new repository         | One `testing.yaml` like the earlier extractions (format, `linter all`, clippy, tests), plus a publish workflow for crates.io. No binary release artefacts.                                                                                               |
| Q5  | First publish                    | `0.1.0` from the new repository, owners `josecelano` and `da2ce7` (as #2583).                                                                                                                                                                             |
| Q6  | `torrust-tracker-client-lib`     | Stays in the tracker workspace: `udp-server`, `test-helpers` and `axum-http-server` use it and #2583 publishes it from here. Whether those servers should depend on a client library at all is a question for the coupling-report review, not this issue. |
| Q7  | Tracker E2E runner               | CI installs the published client (`cargo install torrust-tracker-client --version <v> --locked`) and the E2E runner starts `tracker_checker` from `PATH`.                                                                                                 |
| Q8  | Skills that run the client       | Stay in this repository; they replace `cargo run -p torrust-tracker-client --bin tracker_client` with the installed `tracker_client` after `cargo install torrust-tracker-client`.                                                                       |
| Q9  | Client ADRs and docs             | Its local ADRs and docs move with it. The root ADRs stay as records of this repository's decisions; the new repository records which CLI output contract it follows.                                                                                    |
| Q10 | #2583 spec                       | It already agrees (row 13 `DEFERRED`). The spec-only pull request of this issue updates its References link to this specification and names this issue in its T6 hand-over.                                                                            |
| Q11 | Other links to the draft path    | The same pull request repairs the link in the open #269 artifact `initial-review.md`. The drafts README keeps the folder name as a plain-text naming example.                                                                                            |

## Scope

### In Scope

- Create the target repository (Q1).
- Move `console/tracker-client/` (source, tests, docs, local ADRs, license files) there (Q2).
- Make its `Cargo.toml` self-contained: no `workspace = true` fields or lints, and crates.io
  version requirements for `torrust-tracker-udp-protocol`, `torrust-tracker-http-protocol` and
  `torrust-tracker-client-lib` at the versions #2583 published.
- License files and README license links in the new repository (Q3).
- CI in the new repository (Q4), and its first crates.io publish (Q5).
- In the tracker repository:
  - replace the E2E runner's use of the in-workspace checker (Q7);
  - remove `console/tracker-client` from the root `Cargo.toml` members, delete the directory, and
    drop the `/console/` entry from `.dockerignore` (`add-workspace-member` skill);
  - update the skills that run the client (Q8) and the `release-new-version` file list;
  - update `AGENTS.md` (move the crate to the extracted-packages table, remove the console tools
    table and the `console/` directory entry), `packages/AGENTS.md` and `docs/packages.md`;
  - update EPIC #1669 (package inventory, desired state, "Packages moving to standalone
    repositories", checklist).

### Out of Scope

- Changes to the client's features or behaviour.
- Publishing `torrust-tracker-udp-protocol`, `torrust-tracker-http-protocol` and
  `torrust-tracker-client-lib` (#2583).
- Moving `torrust-tracker-client-lib` out of the tracker workspace (Q6).
- Renaming the crate.
- The README and license-link refresh of the crate before extraction (#2576, which merges before
  #2583's first publish).

## Architectural Decisions

- Related ADRs:
  [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md),
  [`20260519000000_define_global_cli_output_contract.md`](../../../adrs/20260519000000_define_global_cli_output_contract.md)
  and
  [`20260512102000_define_tracker_client_peer_id_convention.md`](../../../adrs/20260512102000_define_tracker_client_peer_id_convention.md).
- ADRs to create: none known.

## Design and Ownership Review

Not applicable: no new runtime code. The E2E runner keeps its current process handling for the
external `tracker_checker` process (Q7); only the command changes.

## Bug-Fix Process

Not applicable: this is not a bug.

## Regression Test Strategy

Not applicable: no behaviour changes. The tracker E2E runner and the client's own tests cover the
move.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                                       | Notes / Expected Output                                                                                                                       |
| --- | ------- | ------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | BLOCKED | Wait for #2583 and check again             | #2583 rows 3, 4 and 8 `PUBLISHED`. Re-run the "Facts checked" commands and update this spec.                                                  |
| T2  | BLOCKED | Create the repository                      | Q1; README and license files committed.                                                                                                       |
| T3  | BLOCKED | Move the source                            | Q2; the new repository holds the crate at its root.                                                                                           |
| T4  | BLOCKED | Make the manifest self-contained           | No `workspace = true`; crates.io dependencies; `cargo build` and `cargo test` pass in a clean clone.                                          |
| T5  | BLOCKED | License files and README links             | Q3.                                                                                                                                           |
| T6  | BLOCKED | CI in the new repository                   | Q4; green on the default branch.                                                                                                              |
| T7  | BLOCKED | First publish                              | Q5; crates.io page, docs.rs build and owners checked.                                                                                         |
| T8  | BLOCKED | Tracker E2E runner without the local crate | Q7; `e2e_tests_runner` passes locally and in CI.                                                                                              |
| T9  | BLOCKED | Remove the crate from the tracker          | Member, directory, `.dockerignore` entry and `Cargo.lock` entries gone; `cargo build --workspace` passes.                                     |
| T10 | BLOCKED | Update docs and skills                     | Q8; `AGENTS.md`, `packages/AGENTS.md`, `docs/packages.md`, the seven skills above; no live reference to `console/tracker-client` remains.           |
| T11 | BLOCKED | Update EPIC #1669                          | Inventory, desired state, standalone-repository table, checklist and progress log.                                                            |
| T12 | BLOCKED | Verify and review acceptance criteria      | Automatic checks and manual scenarios recorded; acceptance criteria re-reviewed.                                                              |

## Commit Points

Tracker repository commits (the new repository's commits follow its own history):

| Task | Coherent change set                                       | Commit policy                         |
| ---- | --------------------------------------------------------- | ------------------------------------- |
| T1   | Re-checked facts in this spec                             | One `docs(issues)` commit.            |
| T8   | E2E runner and workflows use the installed client         | One `ci(e2e)` commit.                 |
| T9   | Remove the member, directory and `.dockerignore` entry    | One `chore(workspace)!` commit.       |
| T10  | Catalogs, `docs/packages.md` and skills                   | One `docs` commit per area.           |
| T11  | EPIC update                                               | One `docs(issues)` commit.            |
| T12  | Evidence and acceptance review                            | One `docs(issues)` commit.            |

Every commit is a signed Conventional Commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-extract-torrust-tracker-client-to-standalone-repo/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] #2583 published `torrust-tracker-udp-protocol`, `torrust-tracker-http-protocol` and `torrust-tracker-client-lib`
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-15 12:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669
- 2026-10-21 07:34 UTC - GitHub Copilot (Claude Opus 5.5) - Refreshed the draft against `develop`
  and crates.io: renamed crates (`torrust-tracker-udp-protocol`, `torrust-tracker-client-lib`),
  added the third blocker `torrust-tracker-http-protocol`, recorded the maintainer's 2026-10-20
  decision (opened now, `Blocked` by #2583, last item of the EPIC), and found the tracker E2E
  runner and six skills that run the client. Questions Q1 to Q10 await the maintainer.
- 2026-10-21 07:44 UTC - GitHub Copilot (Claude Opus 5.5) - The maintainer approved the spec and answered Q1 to
  Q11 (see Maintainer Decisions; Q2 is a plain copy, not `git filter-repo`). Created #2594 (labels
  `task`, `packages`, `Blocked`), linked it as a subissue of #1669 and marked it blocked by #2583
  on GitHub. Moved the spec to `docs/issues/open/` for a spec-only PR on branch
  `2594-1669-extract-torrust-tracker-client-to-standalone-repo-spec`; repaired the links to the
  draft in the #1669, #2583 and #269 documents.

## Acceptance Criteria

- [ ] AC1: The repository from Q1 holds the client, builds and tests from a clean clone with
      crates.io dependencies only, and its CI is green.
- [ ] AC2: `torrust-tracker-client` is published on crates.io from the new repository, with the
      owners from Q5, a successful docs.rs build, and the license from Q3.
- [ ] AC3: `console/tracker-client/` no longer exists in the tracker repository, the root
      `Cargo.toml` does not list it, and no tracker manifest depends on `torrust-tracker-client`.
- [ ] AC4: The tracker E2E runner passes without the local crate (Q7), locally and in CI.
- [ ] AC5: No live document, skill or catalog in the tracker repository tells the reader to run
      `cargo run -p torrust-tracker-client`; `AGENTS.md` lists the crate as extracted.
- [ ] AC6: EPIC #1669 records the extraction in its inventory, desired state and checklist.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- Tracker repository: `linter all`, `cargo build --workspace`, `cargo test --doc --workspace`,
  `cargo test --tests --benches --examples --workspace --all-targets --all-features`,
  `cargo machete --with-metadata`, `cargo deny check bans` (stable Rust toolchain), and the
  pre-push checks.
- New repository: its CI (Q4) on the default branch, and `cargo package` before the first publish.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                 | Human-oriented command/steps                                                                                                                                       | Expected Result                                       | Status  | Evidence                                     |
| --- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------- | ------- | -------------------------------------------- |
| M1  | A user installs and uses the client      | `cargo install torrust-tracker-client`; start a local tracker (`run-tracker-locally` skill); `tracker_client udp announce` and `tracker_client http scrape` against it | Valid JSON responses from both protocols              | BLOCKED | `manual-verification-evidence.md` section V1 |
| M2  | The checker reports a healthy tracker    | `TORRUST_CHECKER_CONFIG='{...}' tracker_checker` with the installed binary, against the local tracker                                                               | Every check `Ok`                                      | BLOCKED | `manual-verification-evidence.md` section V2 |
| M3  | The tracker E2E runner without the crate | `cargo run -p torrust-tracker-e2e-tools --bin e2e_tests_runner` on the T9 branch                                                                                   | Runner passes                                         | BLOCKED | `manual-verification-evidence.md` section V3 |
| M4  | No stale references                      | `rg -n 'console/tracker-client\|-p torrust-tracker-client\b' --glob '!docs/issues/closed/**' --glob '!docs/pr-reviews/**'`                                         | Only historical records match, each one justified     | BLOCKED | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even when automated checks pass.
- Record the toolchain for each command (for example `stable Rust toolchain`).
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- **Tracker CI loses its checker.** Mitigation: T8 replaces it before T9 deletes the crate, and M3
  checks it.
- **Protocol drift.** After extraction the client builds against published protocol crates, so a
  protocol change in the tracker reaches the client only after a new publish. Mitigation: that is
  the intended independent versioning (ADR `20260629000000`).
- **Lost history.** A plain copy (Q2) loses blame and log in the new repository. Mitigation: its
  first commit names the source commit, and the tracker history keeps the full record.
- **Stale facts.** #2583 can take months. Mitigation: T1 re-checks every fact before work starts.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- Independent reviewers record results in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1669, [`EPIC.md`](../../open/1669-overhaul-packages/EPIC.md)
- Blocked by: #2583
  ([`ISSUE.md`](../../open/2583-1669-first-publish-of-workspace-crates/ISSUE.md#per-crate-checklist))
- README and license links before extraction: #2576
  ([`ISSUE.md`](../../open/2576-1669-update-all-package-readmes/ISSUE.md))
- Earlier extraction used as a model: #1909
  ([`ISSUE.md`](../../closed/1909-1669-si-28-extract-server-lib-to-standalone-repo/ISSUE.md))
