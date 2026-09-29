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
last-updated-utc: "2026-09-29 18:40"
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
- **Linter coverage.** `torrust-linting`'s `linter clippy` and `linter rustfmt` run
  `cargo clippy --workspace` / `cargo fmt --check` at the repository root only. Either
  `torrust-linting` gains a manifest-path or multi-workspace option, or the harness gets its own
  Clippy/rustfmt step in the hooks and CI.
- **CI build cache.** A second target directory for the harness workspace (`sccache` and the GHA
  cache keys in `testing.yaml`).
- **`copilot-setup-steps.yml`** runs `cargo build --workspace`; decide whether agents need the
  harness prebuilt too.
- **Docs and skills** that name `cargo run -p <harness-tool>`.

## Expected Outcome

- Root `Cargo.toml` `members` lists only tracker crates; `exclude` names the harness root.
- `.dockerignore` has no harness entries; the tracker container build never sees harness code.
- Adding a harness tool touches only the harness workspace.
- All quality gates run for both workspaces and pass.

## Verification Plan

- `cargo metadata --no-deps` in both workspaces succeeds; the tracker workspace lists no harness
  package.
- `docker build --target recipe --file Containerfile .` passes with the harness allow-list block
  removed.
- `./contrib/dev-tools/git/hooks/pre-commit.sh` passes and still runs the harness checks.
- The `testing.yaml` and `generate_coverage_pr.yaml` workflows pass on a fork PR.
- `cargo machete`, `cargo deny check bans`, and Dependabot cover the harness `Cargo.lock`.

## References

- Issue #2298 (root-cause analysis, cause 2; decisions D6 and D7):
  `docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md`
- ADR: `docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md`
- EPIC #2003: `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`
- Extraction precedent: [torrust/torrust-linting](https://github.com/torrust/torrust-linting)
- Cargo workspaces (`exclude`, `default-members`):
  <https://doc.rust-lang.org/cargo/reference/workspaces.html>
