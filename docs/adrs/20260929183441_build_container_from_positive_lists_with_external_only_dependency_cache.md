---
semantic-links:
  skill-links:
    - create-adr
    - add-workspace-member
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - .github/skills/dev/maintenance/add-workspace-member/SKILL.md
    - Containerfile
    - .dockerignore
    - Cargo.toml
    - docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md
    - docs/issues/closed/1852-1840-workflow-performance-recipe-stage-manifest-only-copy/ISSUE.md
    - docs/issues/closed/1869-1840-workflow-performance-dependency-layer-cache-reuse/ISSUE.md
---

<!-- skill-link: create-adr -->

# Build the Container from Positive Lists with an External-Only Dependency Cache

## Scope

Root ADR. It governs the repository-wide container build: what enters the Docker build context,
how Cargo Chef caches dependencies, and which workspace packages are compiled and tested inside the
image. It touches `Containerfile`, `.dockerignore`, and the root `Cargo.toml` workspace section,
so no single package owns it.

## Description

The tracker is a Cargo workspace with three kinds of crates: the product (root crate and
`packages/*`), tracker verification tools that depend on product crates (`console/tracker-client`,
`e2e-tools`, `persistence-benchmark`, `torrent-repository-benchmarking`), and AI-harness tools
under `contrib/dev-tools/` that have no tracker dependency at all. All of them are members of one
workspace, and Docker, Cargo Chef, and nextest treat the workspace as the product.

Between June and September 2026 the `Containerfile` recipe stage listed every in-repo manifest
(`COPY <pkg>/Cargo.toml`) and every declared target (`mkdir`/`touch` stubs), `.dockerignore`
carried one negation per harness manifest, and four identical `cargo nextest archive --exclude`
lists named every non-product crate. The same fact, which packages exist and which targets they
have, was copied by hand into six places in three formats. Seven commits repaired drift in those
lists (`93e194361`, `94f2441a9`, `5938a1ee2`, `b6ddf879e`, `e4d7da576`, `a45ae2d41`,
`a871c7f7b`); two of them were for product benchmark targets, the rest for harness tools. Each
omission failed only in the hosted Container workflow, after `cargo chef prepare` called
`cargo metadata` against a workspace whose manifests or target files were missing.

The manifest-only recipe stage came from issue #1852. Its goal was to stop source-only edits from
invalidating the recipe layer and, through it, the dependency cook layers. Its warm-build
measurements were never executed. The cache property that actually protects the expensive layer
came later from issue #1869: the `torrust-cargo-chef` fork adds `cargo chef prepare
--external-only`, producing `recipe-thirdparty.json` with every `path` dependency stripped, so the
third-party cook layer is keyed only on external dependency metadata and survives workspace
manifest changes. Most tracker logic lives in workspace packages that change in nearly every
commit, so this is the layer that must stay cached. Until this ADR, that model existed only in
issue #1869 and `Containerfile` comments.

Issue #2298 measured both models on a warm local BuildKit cache (`docker build --target
test_debug`, evidence in its `manual-verification-evidence.md`):

| Scenario | Manifest-only recipe stage (#1852) | Canonical `COPY . .` recipe stage |
| --- | --- | --- |
| Source-only change (one `.rs` file) | recipe `CACHED`; all cook stages `CACHED`; `build_debug` 32 s | recipe re-ran (0.5 s); all cook stages `CACHED`; `build_debug` 33 s |
| Workspace `Cargo.toml` comment change | recipe re-ran; all cook stages `CACHED` | recipe re-ran; all cook stages `CACHED` |
| Workspace `Cargo.toml` feature change | recipe re-ran; third-party cook rebuilt (47 s) and full cook rebuilt (19 s) | recipe re-ran; third-party cook rebuilt (43 s) and full cook rebuilt (21 s) |

The two models cache identically. BuildKit keys `COPY --from=recipe` on the content checksum of
the copied file, so a re-run recipe stage that emits byte-identical recipe files keeps the cook
layers cached; the hand-maintained lists bought nothing. The feature-toggle row also shows that a
workspace feature change invalidates `recipe-thirdparty.json` in both models, because toggling a
workspace feature changes which external features are enabled; that is expected and unrelated to
the recipe stage design.

The maintainer also set a security framing: for everything that enters the container, prefer a
positive list over a negative list. Forgetting to include something fails the build visibly;
including something by accident can ship unintended content into the image.

## Agreement

1. **The recipe stage is Cargo Chef's canonical planner stage.** It copies the whole allow-listed
   build context and runs `cargo chef prepare` twice (full and `--external-only`). Cargo is the
   only source of truth for packages and targets; nothing about them is listed in the
   `Containerfile`. Adding, moving, or removing a package, binary, bench, or example needs no
   container change.

2. **The three-layer cook is kept.** `dependencies_thirdparty*` cooks `recipe-thirdparty.json`
   (external dependencies only); `dependencies*` cooks the full `recipe.json` on top of it;
   `build*` compiles real source. `cargo chef cook` keeps `--workspace` so the skeleton covers
   every member the recipe enumerates. The `torrust-cargo-chef` fork stays until upstream
   `cargo-chef` merges `--external-only` (LukeMathWalker/cargo-chef#360).

3. **The Docker build context is a default-deny allow-list.** `.dockerignore` starts with `*` and
   re-includes only what a `Containerfile` stage reads: `.cargo/`, `Cargo.toml`, `Cargo.lock`,
   `src/`, `tests/`, `packages/`, `console/`, `share/`, `contrib/dev-tools/su-exec/`, and, as an
   interim measure, each AI-harness crate directory that is still a workspace member. Broad
   negations are not used. Every workspace member must be admitted whole, because `cargo metadata`
   aborts on a member whose manifest or declared target file is missing. Inside admitted
   directories, `**/AGENTS.md` and `**/docs/` are re-excluded: no build stage reads them, and doc
   edits then do not invalidate the source `COPY` layers.

4. **Container test scope is a Cargo-native positive list.** Root `Cargo.toml` declares
   `[workspace] default-members` naming the product crates. Every `cargo nextest archive` in the
   `Containerfile` runs without `--workspace` and without `--exclude`, so it builds exactly the
   default members. Tracker verification tools and AI-harness crates are members but not default
   members and therefore never enter a container test archive. This asymmetry (cook uses
   `--workspace`, archive does not) is deliberate and documented in the `Containerfile`.

5. **Every in-repo crate is listed explicitly in `[workspace].members`.** The `--external-only`
   skeleton strips `path` dependencies, so members that were only auto-discovered through path
   dependencies vanish from the skeleton, and `default-members` then fails to resolve them during
   `cargo chef cook`. Explicit membership keeps the skeleton and the real workspace identical.

6. **Accepted side effect.** With `default-members`, a bare `cargo build`, `cargo test`, or
   `cargo clippy` at the repository root acts on all product crates instead of the root crate only.
   CI workflows, git hooks, and `linter` already pass `--workspace` and are unaffected.

### Alternatives Considered

- **Validator that checks the hand lists against `cargo metadata`.** Keeps the duplication and
  needs to parse `Containerfile` syntax; rejected because the lists themselves are the problem.
- **Generating the manifest/stub block from Cargo metadata plus per-package classification
  metadata.** Adds a generation workflow and a checked-in generated artifact to remove lists that
  the measurement showed are unnecessary; rejected.
- **Generated stubs inside a manifest-only recipe stage.** Retained as the fallback if a future
  measurement shows the canonical stage losing the third-party cache: admit `**/Cargo.toml` and
  `Cargo.lock` generically, then run a repository-owned tool that touches every declared target
  path (plus `src/lib.rs` for packages with no declared target) before `cargo chef prepare`.
  Auto-detected targets are invisible without source, which is harmless because `cargo metadata`
  only aborts on declared targets whose file is missing; extra stub targets change only
  workspace-crate fingerprints, which are rebuilt from source anyway. Not implemented because the
  measured behaviour did not require it.
- **BuildKit bind mounts for metadata discovery.** `RUN --mount=type=bind` is cache-keyed on the
  mounted files' checksums like `COPY`, so it has the same cache behaviour with less portability
  to Podman/buildah; rejected.
- **A single `ARG` holding a `-p` list for `cargo nextest archive`.** Deduplicates the four lists
  but keeps the list in Docker rather than Cargo and gains nothing that `default-members` does not;
  rejected.
- **A checked-in data file classifying crates for container inclusion.** Another artifact to own
  next to Cargo metadata; rejected in favour of `default-members`.

### Consequences

- Positive: no `Containerfile` or `.dockerignore` change for product package or target changes;
  an unreachable member fails the `recipe` stage with Cargo's own diagnostic instead of a silent
  stale list; the container build context cannot grow by omission; container test scope is
  reviewable in one Cargo field.
- Positive: the container caching model is now recorded here instead of only in issue text.
- Negative: the recipe layer re-runs on every source change (well under a second).
- Negative: AI-harness crates still need one allow-list entry each while they remain workspace
  members. The residual manual surface disappears when the harness moves to its own workspace,
  proposed to EPIC #2003 by the #2298 draft sub-issue spec.
- Negative: `default-members` changes bare root `cargo` commands as described above.

## Date

2026-09-29

## References

- Issue #2298: `docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md` (root-cause
  analysis, decisions D1-D7, measurement evidence)
- Issue #1852 (manifest-only recipe stage, superseded by this ADR):
  `docs/issues/closed/1852-1840-workflow-performance-recipe-stage-manifest-only-copy/ISSUE.md`
- Issue #1869 (`--external-only` third-party recipe, preserved):
  `docs/issues/closed/1869-1840-workflow-performance-dependency-layer-cache-reuse/ISSUE.md`
- EPIC #2003 (AI-harness organization):
  `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`
- ADR: [Keep unit tests inside the container build](20260603000000_keep_unit_tests_inside_container_build.md)
- Cargo Chef README: <https://github.com/LukeMathWalker/cargo-chef>
- Upstream `--external-only` PR: <https://github.com/LukeMathWalker/cargo-chef/pull/360>
- Docker build cache invalidation (`COPY` checksums exclude `mtime`):
  <https://docs.docker.com/build/cache/invalidation/>
- Cargo `default-members`:
  <https://doc.rust-lang.org/cargo/reference/workspaces.html#the-default-members-field>
