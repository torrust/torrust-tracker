---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 2003
github-issue: null
spec-path: docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md
branch: "{issue-number}-2003-separate-ai-harness-cargo-workspace"
related-pr: null
last-updated-utc: "2026-09-30 10:45"
semantic-links:
  skill-links:
    - create-issue
    - add-workspace-member
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/maintenance/add-workspace-member/SKILL.md
    - Cargo.toml
    - .dockerignore
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md
    - docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Separate the AI-Harness Tools into Their Own Cargo Workspace

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Move the Rust AI-harness tools out of the tracker Cargo workspace into an independent workspace in
the same repository, so the tracker build, container image, lockfile, and `--workspace` commands
describe only the tracker, and the harness can evolve, and later be extracted, without touching
tracker build artifacts.

## Background

Issue #2298 traced the recurring container-build repairs for developer tools to two causes. The
first (hand-maintained Cargo Chef lists) is fixed there. The second is structural and is proposed
here: the tracker workspace and the AI-harness workspace are mixed.

The explicit workspace members split into groups with different relationships to the tracker:

| Group | Members | Depends on tracker crates? |
| --- | --- | --- |
| AI/dev harness | `contrib/dev-tools/analysis/workspace-coupling`, `contrib/dev-tools/checks/agent-review-report-contract`, `contrib/dev-tools/checks/clippy-allow-reasons`, `contrib/dev-tools/checks/frontmatter-validator`, `contrib/dev-tools/checks/package-coverage-check`, `contrib/dev-tools/github/github-review-threads` | No. They inherit `[workspace.package]` fields and `[workspace.lints]` and share `Cargo.lock`; nothing in the tracker depends on them. |
| Tracker verification tools | `console/tracker-client`, `packages/e2e-tools`, `packages/persistence-benchmark`, `packages/torrent-repository-benchmarking` | Yes, through path dependencies. They stay in the tracker workspace. |

A Cargo workspace is one unit for the lockfile, `cargo metadata`, and `--workspace`. Docker, Cargo
Chef, nextest, `cargo deny`, `cargo machete`, Dependabot, and the linter all treat that unit as
"the product". Every harness tool therefore joins the tracker build and must be kept out again by
hand: after #2298 that residual surface is one `.dockerignore` allow-list line per harness crate,
and every harness crate must be a non-default member. Five of the seven historical container
repairs were for harness tools.

The maintainer's position (2026-09-29 review of #2298): the harness may be reused by other projects
later, so it must stay decoupled from the tracker; but while it is still changing weekly it should
live in this repository, because multiple repositories are harder to iterate on. An independent
in-repo workspace gives the decoupling now and keeps extraction (following the `torrust-linting`
precedent and the EPIC #1669 package extractions) possible later.

## Scope

### In Scope

- Create a second Cargo workspace for the harness crates and remove them from the tracker
  workspace's `members`, adding the harness root to the tracker workspace's `exclude`.
- Move the harness's shared metadata and lint policy into the new workspace root.
- Update every invocation that runs a harness binary (`cargo run -p <tool>` in git hooks and CI
  workflows) to target the new workspace (`--manifest-path` or a documented equivalent).
- Extend the quality gates so they cover both workspaces: `cargo machete`, `cargo deny`,
  `cargo test --doc`, nightly `cargo fmt --check`, Clippy, and Dependabot.
- Remove the interim harness allow-list block from `.dockerignore` (introduced by #2298) once the
  crates are no longer tracker workspace members.
- Update `add-workspace-member` and any skill that names a harness crate path or `-p` name.

### Out of Scope

- Extracting the harness to its own repository.
- Rewriting harness tools, merging them into a unified binary, or porting shell scripts to Rust
  (those are EPIC #2003 architecture decisions; this issue moves crates, it does not redesign them).
- Changing tracker verification tools (`console/tracker-client`, `e2e-tools`,
  `persistence-benchmark`, `torrent-repository-benchmarking`); they depend on tracker crates and
  remain tracker workspace members.

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md`
  (positive-list container build; its interim harness allow-list entries are removed by this
  issue).
- ADRs to create: one ADR recording the harness workspace boundary (location, what counts as
  harness, and how quality gates cover a second workspace), once EPIC #2003 settles the deferred
  decisions below.

## Decisions Deferred to EPIC #2003

These are precisely the questions EPIC #2003 exists to answer; this draft records them and takes
no position.

1. **Workspace location.** Candidates: `contrib/dev-tools/Cargo.toml` (current crate paths, least
   churn), an explicit top-level `contrib/ai-harness/` (clear naming, moves every crate), or a new
   top-level directory. The name should make "this is the harness, not the tracker" obvious to a
   reader of the repository root.
2. **What counts as harness.** Which of the current `contrib/dev-tools/` crates, shell scripts, and
   future tools belong to it; whether `workspace-coupling` (an analysis of the tracker workspace)
   is harness or tracker tooling.
3. **Binary shape.** One unified harness binary with subcommands, or one binary per tool.
4. **Scripts versus Rust.** Which shell scripts under `contrib/dev-tools/` migrate into the harness
   workspace as Rust and which stay as orchestration.
5. **Ownership and versioning.** Whether harness crates version together, and who reviews harness
   changes.

## Migration Costs to Plan For

Recorded so the EPIC can weigh them; none is a blocker.

- **Second `Cargo.lock`.** Dependabot (`.github/dependabot.yml` needs a second `cargo` entry with
  the harness directory), `cargo deny check bans` (run per workspace or configure a shared
  `deny.toml`), and `cargo machete --with-metadata` (run per workspace) must all cover it.
- **Lint policy duplication.** `[workspace.lints]` (about 39 lines) and `[workspace.package]`
  fields are copied to the harness root. A drift check between the two, or a shared include once
  Cargo supports it, is a follow-up.
- **Invocation changes.** Git hooks (`contrib/dev-tools/git/hooks/pre-commit.sh` runs
  `clippy-allow-reasons` and `frontmatter-validator` via `cargo run --package`) and CI workflows
  (`testing.yaml` runs `clippy-allow-reasons`; `generate_coverage_pr.yaml` runs
  `package-coverage-check`) switch to `--manifest-path <harness>/Cargo.toml`.
- **Repository-file embedding.** `github-review-threads` embeds
  `.github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md` with `include_str!` via a
  relative path. Moving the crate changes that path, and the file is outside the tracker container
  allow-list, which is harmless only while the crate is not a tracker default member.
- **Linter coverage.** `torrust-linting`'s `linter clippy` and `linter rustfmt` run
  `cargo clippy --workspace` / `cargo fmt --check` at the repository root only. Either
  `torrust-linting` gains a manifest-path or multi-workspace option, or the harness gets its own
  Clippy/rustfmt step in the hooks and CI.
- **CI build cache.** A second target directory for the harness workspace (`sccache` and the GHA
  cache keys in `testing.yaml`).
- **`copilot-setup-steps.yml`** runs `cargo build --workspace`; decide whether agents need the
  harness prebuilt too.
- **Docs and skills** that name `cargo run -p <harness-tool>`.

## Implementation Plan

Tasks T2 onward start only after EPIC #2003 records the deferred decisions (T1).

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | BLOCKED | Obtain EPIC #2003 decisions | Location, harness boundary, binary shape, and ownership recorded in the EPIC; this spec updated to match. Blocked on the EPIC architecture decision. |
| T2 | TODO | Create the harness workspace | New workspace root with copied `[workspace.package]` and `[workspace.lints]`; harness crates moved or re-rooted; root `members` drops them and `exclude` names the harness root; both `cargo metadata` calls succeed. |
| T3 | TODO | Migrate invocations | Hooks, CI workflows, docs, and skills call harness binaries through the new workspace; `include_str!` paths updated. |
| T4 | TODO | Cover both workspaces in quality gates | Dependabot, `cargo deny`, `cargo machete`, doc tests, formatting, and Clippy run for the harness workspace. |
| T5 | TODO | Remove interim container entries | Harness block removed from `.dockerignore`; `add-workspace-member` skill updated. |
| T6 | TODO | Verify and review | Automatic checks, manual scenarios, acceptance review, completion review. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Spec update with the EPIC decisions | Documentation-only commit. |
| T2 | Workspace creation and membership move | One commit after both workspaces resolve and build. |
| T3 | Invocation migration | One commit after hooks and a local run of each migrated invocation pass. |
| T4 | Quality-gate coverage | One commit per tool family when it improves reviewability. |
| T5 | Container allow-list and skill cleanup | One commit after `docker build --target recipe`. |
| T6 | Evidence and completion review | Documentation-only commits. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md`
- [ ] EPIC #2003 owner reviewed the draft and recorded the deferred decisions
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-29 18:40 UTC - GitHub Copilot - Drafted from issue #2298 root-cause analysis (cause 2) at
  the maintainer's request, for EPIC #2003 owner review. No GitHub issue created.
- 2026-09-30 10:45 UTC - GitHub Copilot - Added the parent marker and the template lifecycle
  sections after the PR #2385 review (`review-finding:pr-2385-f1`, `review-finding:pr-2385-f2`).

## Acceptance Criteria

- [ ] AC1: Root `Cargo.toml` `members` lists only tracker crates; `exclude` names the harness root.
- [ ] AC2: `.dockerignore` has no harness entries; the tracker container build never sees harness
  code.
- [ ] AC3: Adding a harness tool touches only the harness workspace.
- [ ] AC4: Every quality gate that covers the tracker workspace also covers the harness workspace.
- [ ] AC5: Every hook, workflow, doc, and skill invocation of a harness binary works from the
  repository root.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass in both workspaces
- [ ] Manual verification scenarios are executed and documented in issue-local
  `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --doc --workspace` in both workspaces
- `cargo metadata --no-deps` in both workspaces; the tracker workspace lists no harness package
- Pre-commit and pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Container build without harness | Remove the harness allow-list block; `docker build --target recipe --file Containerfile .` | Recipe stage passes; the build context contains no harness path. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Hooks run harness checks | Stage a Markdown file with invalid frontmatter; run `./contrib/dev-tools/git/hooks/pre-commit.sh` | The frontmatter check runs from the harness workspace and reports the error. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | CI invocations | Push to a fork PR | `testing.yaml` and `generate_coverage_pr.yaml` run the harness tools and pass. | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Dependency tooling | Run `cargo machete` and `cargo deny check bans` for both workspaces; inspect the Dependabot configuration | Both workspaces covered; no unused dependency. | TODO | `manual-verification-evidence.md` section V4 |

### Acceptance Verification

After implementation, re-review each acceptance criterion against the evidence above and record
the result in the progress log.

## Risks and Trade-offs

- Lint policy duplicated in two workspace roots can drift; a follow-up check may be needed.
- A second lockfile doubles dependency-update review for shared crates.
- Tools that inspect the tracker workspace (`workspace-coupling`) must keep targeting the tracker
  manifest explicitly once they live in another workspace.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated
assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.

## References

- Issue #2298 (root-cause analysis, cause 2; decisions D6 and D7):
  `docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md`
- ADR: `docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md`
- EPIC #2003: `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`
- Extraction precedent: [torrust/torrust-linting](https://github.com/torrust/torrust-linting)
- Cargo workspaces (`exclude`, `default-members`):
  <https://doc.rust-lang.org/cargo/reference/workspaces.html>
