---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: null
github-issue: 2298
spec-path: docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md
branch: "2298-rust-dev-tool-container-integration"
related-pr: 2293
last-updated-utc: "2026-09-29 18:50"
semantic-links:
  skill-links:
    - create-issue
    - add-workspace-member
  related-artifacts:
    - Cargo.toml
    - Containerfile
    - .dockerignore
    - .github/skills/dev/maintenance/add-workspace-member/SKILL.md
    - docs/issues/closed/2222-1347-package-coverage-regression-ci/ISSUE.md
    - docs/issues/closed/1852-1840-workflow-performance-recipe-stage-manifest-only-copy/ISSUE.md
    - docs/issues/closed/1869-1840-workflow-performance-dependency-layer-cache-reuse/ISSUE.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md
    - docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md
---

# Issue #2298 - Eliminate Manual Container Integration for Rust Developer Tools

## Goal

Make adding a Rust developer tool to the Cargo workspace reliable without repeatedly repairing the
Docker build recipe by hand. The eventual solution must keep developer-only tools out of the
production tracker image and container test archives while ensuring Cargo Chef can resolve the
whole workspace.

Selected direction (2026-09-29 maintainer review, see [Root Cause Analysis](#root-cause-analysis)
and [Selected Design](#selected-design)): remove the duplicated hand-maintained lists instead of
validating or generating them. The container build derives package facts from Cargo, the Docker
build context becomes an explicit allow-list, container test archives use a Cargo-native positive
list, and the AI-harness tools are proposed for a separate workspace under EPIC #2003.

## Background

The tracker is a Cargo workspace whose root `Cargo.toml` explicitly registers several developer
and CI tools. The production Containerfile uses a manifest-only Cargo Chef recipe stage to preserve
dependency-layer caching. Cargo Chef runs `cargo metadata` and therefore needs every workspace
member's manifest plus a stub for every declared or auto-detected target, including developer-only
members that are not built into the final tracker image.

The Docker build context is filtered by `.dockerignore`, which excludes most developer tooling.
Consequently, adding a developer-only Rust binary can succeed locally with Cargo but fail in the
Container workflow when the new member is absent from the recipe stage or its manifest is excluded
from the context. The same member must then be excluded from every `cargo nextest archive`
invocation so it does not add non-production work to container test archives.

PR #2293 reproduced the problem with `package-coverage-check`. The hosted Container workflow
failed before the repair because the root workspace referenced the new member while the recipe
could not see its manifest. After the manual repair, a local
`docker build --target test --file Containerfile .` completed all 68 stages, including Cargo Chef
recipe generation, nextest archive creation and extraction, and release test execution. The hosted
workflow still needs final observation after that repair is pushed.

This issue is a recurring-maintenance problem, not a request to change the current production-image
scope for developer tools.

## Problem Model

### Ownership and Constraints

| Surface | Current responsibility | Required invariant |
| --- | --- | --- |
| Root `Cargo.toml` | Declares explicit developer-tool workspace members. | Cargo recognizes the tool as a workspace member. |
| `.dockerignore` | Limits Docker build context. | Every manifest copied during the recipe stage remains reachable. |
| `Containerfile` recipe manifest block | Copies manifests before source to preserve dependency caching. | Cargo Chef can load every workspace manifest. |
| `Containerfile` recipe stubs | Creates directories and target files before `cargo chef prepare`. | `cargo metadata` resolves every target without real tool source. |
| `cargo chef cook` | Warms dependency layers for the full workspace. | May see developer-tool manifests; it has no `--exclude` option. |
| `cargo nextest archive` | Produces artifacts run in the container test stages. | Developer-only tools remain excluded from every archive invocation. |

### Failure Sequence

1. A new Rust developer tool is added to root `[workspace].members`.
2. Cargo commands in the source checkout work because the tool manifest and source exist there.
3. Docker's recipe stage copies only its hand-maintained manifest list and may not receive the new
   manifest because `.dockerignore` excludes `contrib/dev-tools/`.
4. `cargo chef prepare` calls Cargo metadata against a workspace that names a missing member or
   target, so the Container workflow fails before it reaches production tracker tests.
5. A manual repair is required in several distant, duplicated lists.

### Why This Recurs

The workspace-member list, Docker build-context exceptions, Cargo Chef manifest copies, target
stubs, and nextest exclusions encode overlapping facts in different formats. No generated artifact
or validation currently proves they remain synchronized when a developer tool is added, removed,
renamed, or gains another binary target.

## Current Manual Solution

Use this as observed evidence, not as the desired long-term workflow.

When adding a developer-only Rust binary or crate such as
`contrib/dev-tools/checks/package-coverage-check`, maintainers currently make all of these changes:

1. Add the member path to root `Cargo.toml` under `[workspace].members` and update `Cargo.lock`.
2. In `.dockerignore`:
   - add a comment explaining why the manifest is needed by Cargo Chef;
   - add `!/contrib/dev-tools/checks/package-coverage-check/Cargo.toml` after the
     `/contrib/dev-tools/` exclusion.
3. In the Containerfile recipe stage:
   - add `package-coverage-check` to the comment listing members excluded from container test
     archives, with its developer-only reason;
   - add `COPY contrib/dev-tools/checks/package-coverage-check/Cargo.toml
     contrib/dev-tools/checks/package-coverage-check/` to the manifest-only copy list;
   - add `contrib/dev-tools/checks/package-coverage-check/src` to the `mkdir -p` list;
   - add `contrib/dev-tools/checks/package-coverage-check/src/lib.rs` and
     `contrib/dev-tools/checks/package-coverage-check/src/main.rs` to the `touch` list, matching
     its actual targets;
   - add `package-coverage-check` to the explanatory `cargo chef cook` comment; and
   - add `--exclude package-coverage-check` to every `cargo nextest archive` command: debug cook
     warm-up, release cook warm-up, debug archive build, and release archive build.
4. Validate the recipe and test archive stages, then run normal repository quality gates.

The target-stub entries are not optional. A manifest with implicit `src/lib.rs`, `src/main.rs`, or
`src/bin/*.rs` targets must have corresponding stub files because `cargo metadata` validates the
manifest target layout during Cargo Chef preparation.

## Scope

### In Scope

- Document and reproduce the mismatch between explicit Cargo workspace membership and Cargo Chef's
  manifest-only Docker recipe.
- Inventory every duplicated source of truth that changes for developer-only Rust tools.
- Remove the duplicated hand-maintained lists from the container build (D1/D2, D4) and measure the
  cache effect against the current baseline (M5, M6).
- Preserve the #1869 third-party dependency cache layer; any regression triggers the D2 fallback.
- Convert the Docker build context to a default-deny allow-list (D3).
- Keep developer-only checks, analysis tools, benchmarks, CLI tools, and host-only E2E tools out of
  container test archives through the `default-members` positive list (D4).
- Record the container caching and positive-list model in an ADR (D5).
- Draft the separate AI-harness workspace sub-issue spec for EPIC #2003 (D6).
- Update the workspace-member workflow documentation (AC6).

### Out of Scope

- Adding a new developer tool.
- Changing the tracker runtime image contents or production deployment behavior.
- Removing Cargo Chef, Docker, or nextest archives.
- Implementing the AI-harness workspace separation (owned by EPIC #2003).
- Deciding that every workspace member belongs in container test archives.
- Changing which tracker verification tools are tested inside the container.

## Architectural Decisions

- Related guidance: `.github/skills/dev/maintenance/add-workspace-member/SKILL.md` documents the
  manual checklist that this issue retires.
- Related implementation evidence: PR #2293 and its Container workflow repair.
- Related prior decisions without an ADR: #1852 (manifest-only recipe stage) and #1869
  (`--external-only` third-party recipe). This issue supersedes #1852 and preserves #1869.
- ADR to create (D5): container caching and positive-list model. It changes Docker build-context
  policy and the container performance model, which the original draft named as ADR triggers.

## Design and Ownership Review

This work owns build metadata only; it does not introduce runtime child-process or
network-lifetime behavior.

- Cargo is the single authority for workspace packages and declared targets; the container build
  reads them through `cargo chef prepare`, never through a copied list.
- Recipe reachability and archive inclusion are distinct: every workspace member must be visible to
  `cargo metadata` (build context), while only `default-members` are compiled and tested inside the
  image.
- Positive lists are preferred over negative lists wherever content enters the container.
- The first vertical slice adds one disposable package and demonstrates a passing container build
  with no `Containerfile` change (M2) and exclusion from the archive when it is not a default
  member (M3).

## Bug-Fix Process

1. Reproduce the current failure using a workspace member omitted from the recipe build context or
   from required target stubs (M1).
2. Capture the Cargo Chef/Cargo metadata diagnostic in issue-local manual evidence.
3. Measure the baseline cache behaviour (M5, M6) on the current `Containerfile`.
4. Implement D1 and repeat the measurement; fall back to D2 only if the third-party cook layer is
   no longer cached.
5. Implement D3 and D4.
6. Re-run the same container targets and the hosted Container workflow.

## Regression Test Strategy

The regression boundary is the container build itself: with the canonical recipe stage, an
unreachable manifest or target fails `cargo chef prepare` with Cargo's own diagnostic, and there is
no hand-maintained list left to drift. If D2 is implemented, the stub generator gets focused unit
tests covering library-only, binary-only, mixed, and explicit-path targets plus a deterministic
negative case.

## Root Cause Analysis

Recorded from the 2026-09-29 maintainer review. The original draft framed the problem as keeping
several lists synchronized; the review concluded that the lists themselves are the problem.

### Type of coupling

One fact, "which packages exist, where they live, and which targets they have", is owned by Cargo
and copied by hand into six places in three formats:

1. root `Cargo.toml` `[workspace].members`;
2. `.dockerignore` negation entries;
3. `Containerfile` recipe-stage `COPY <pkg>/Cargo.toml` lines;
4. `Containerfile` recipe-stage `mkdir`/`touch` target stubs;
5. four identical `cargo nextest archive --exclude` lists; and
6. explanatory comments listing the same packages.

This is content coupling through duplicated knowledge: no artifact derives from another, and no
check ties them together, so every package addition, move, or target change must be replicated by
hand. Git history confirms it is not limited to developer tools: `93e194361` and `a871c7f7b`
repaired UDP benchmark stubs for tracker packages, while `94f2441a9`, `5938a1ee2`, `b6ddf879e`,
`e4d7da576`, and `a45ae2d41` repaired AI-harness tools.

### Cause 1: the hand-maintained recipe stage (issue #1852)

Issue #1852 replaced Cargo Chef's canonical `COPY . .` planner stage with a manifest-only copy plus
target stubs so that source-only changes would not invalidate the recipe layer. Its own warm-build
measurements (T4, M3, M4) were never executed; the change assumed that Docker "can't tell" whether a
regenerated `recipe.json` is identical. Cargo Chef's README describes hand-maintained manifest lists
as the fragile approach it was created to replace.

The cache property that actually matters was delivered later by issue #1869: the
`torrust-cargo-chef` `--external-only` recipe (`recipe-thirdparty.json`) strips all `path`
dependencies, so the expensive third-party cook layer is keyed only on external dependency metadata
and survives workspace `Cargo.toml` changes. Most tracker logic lives in workspace packages that
change in nearly every commit, so this layer is the one that must remain cached. No ADR records this
model; the rationale exists only in #1869 and `Containerfile` comments.

Whether `COPY . .` can return to the recipe stage without regressing #1869 depends on one fact:
BuildKit keys `COPY --from=recipe` cache entries on the content checksum of the copied file, not on
whether the producing stage re-ran (Docker's cache-invalidation documentation states that `COPY`
checksums exclude `mtime`). If that holds, a re-run recipe stage that emits an identical
`recipe-thirdparty.json` keeps the third-party cook cached. This must be measured before adoption,
and the design carries a fallback that does not depend on it.

### Cause 2: the tracker workspace and the AI-harness workspace are mixed

The explicit members split into two groups with different relationships to the tracker:

| Group | Members | Depends on tracker crates? |
| --- | --- | --- |
| AI/dev harness | `workspace-coupling`, `agent-review-report-contract`, `clippy-allow-reasons`, `frontmatter-validator`, `package-coverage-check`, `github-review-threads` | No. They share only `[workspace.package]` fields, `[workspace.lints]`, and `Cargo.lock`; nothing in the tracker depends on them. |
| Tracker verification tools | `console/tracker-client`, `e2e-tools`, `persistence-benchmark`, `torrent-repository-benchmarking` | Yes, through path dependencies; they belong in the tracker workspace. |

A Cargo workspace is one unit for the lockfile, `cargo metadata`, and `--workspace`. Docker, Cargo
Chef, and nextest treat that unit as the product, so every harness tool joins the product build and
must then be kept out again by hand. The harness is expected to grow and to be reused by other
projects later; keeping it inside the tracker workspace couples it to the tracker while it is still
changing weekly. EPIC #2003 owns how the harness is organized (location, what counts as harness,
unified binary versus multiple binaries, scripts versus Rust), so the separation is proposed there
rather than decided here.

### Security framing

The maintainer prefers a positive list over a negative list for everything that enters the
container: forgetting to include something breaks the build visibly, while accidentally including
something can expose the tracker to attack. This applies to the Docker build context
(`.dockerignore`) and to the container test archives (which packages are compiled and tested inside
the image).

## Selected Design

### Decision summary

| # | Decision | Owner | Status |
| --- | --- | --- | --- |
| D1 | Restore Cargo Chef's canonical `COPY . .` recipe stage and delete the manifest-copy and stub lists, provided measurement M5/M6 shows the third-party cook layer stays cached across source-only and workspace-manifest-only changes. | #2298 | Implemented; gate passed (V2) |
| D2 | If D1 measurement fails, implement the generated-stubs fallback (below) instead of restoring hand lists. | #2298 | Not needed (kept as documented fallback in the ADR) |
| D3 | Convert `.dockerignore` to a default-deny allow-list of what the container build needs. | #2298 | Implemented |
| D4 | Replace the four `--exclude` lists with a Cargo-native positive list: `[workspace] default-members` names the tracker packages, and `cargo nextest archive` runs without `--workspace`. | #2298 | Implemented |
| D5 | Write an ADR recording the container caching model: three-layer cook, `--external-only` recipe, canonical recipe stage, allow-list context, positive archive list. | #2298 | Written |
| D6 | Propose a separate AI-harness Cargo workspace as a draft sub-issue spec of EPIC #2003 (owner: Cameron). #2298 does not implement it. | EPIC #2003 | Draft spec written |
| D7 | Until D6 lands, harness crates remain reachable to `cargo metadata` through explicit allow-list entries, removed when the harness leaves the tracker workspace. | #2298 | Implemented (interim block in `.dockerignore`) |
| D8 | List every in-repo crate explicitly in `[workspace].members`, not only the ones Cargo cannot auto-discover. Found during V3: the `--external-only` skeleton strips path dependencies, so auto-discovered members vanish from it and `default-members` fails to resolve during `cargo chef cook`. | #2298 | Implemented |

Options 1 (validator), 2 (generated Containerfile fragment), 4 (BuildKit bind mounts), 5
(reclassify tools outside the workspace, now D6), and 6 (data file) from the original draft are
retired: they add machinery on top of the duplicated lists instead of removing them. Option 3 is
retained only as the D2 fallback.

### D1: canonical recipe stage

```dockerfile
FROM chef AS recipe
WORKDIR /build/src
COPY . /build/src
RUN cargo chef prepare --recipe-path /build/recipe.json
RUN cargo chef prepare --external-only --recipe-path /build/recipe-thirdparty.json
```

The recipe stage layer is rebuilt on every source change (`cargo chef prepare` costs about 0.1 s per
the #1841 baseline). Downstream `COPY --from=recipe` steps must hit the cache when the recipe files
are byte-identical. New tracker packages and targets are picked up automatically; no `Containerfile`
edit is needed when a package, binary, bench, or example is added.

Measurement gate (see M5, M6): with a warm cache, a source-only change must leave
`dependencies_thirdparty*` and `dependencies*` cook stages `CACHED`; a workspace `Cargo.toml`-only
change must leave `dependencies_thirdparty*` `CACHED` and rebuild only the full cook stubs. Both
scenarios were specified by #1852 (M3, M4) and never executed; this issue runs them for the previous
and the new `Containerfile` and records the numbers.

Measured result (2026-09-29, `manual-verification-evidence.md` V1/V2): identical cache behaviour
in both `Containerfile`s. Source-only and manifest-comment changes keep every cook stage `CACHED`
with the canonical stage (`COPY --from=recipe` hit the cache although the recipe stage re-ran,
confirming content-checksum keying); a workspace feature toggle rebuilds both cooks in both
designs because it changes external feature resolution. Recipe stage re-run cost: about 0.5 s.
The gate passed and D2 was not implemented.

### D2: generated-stubs fallback

Used only if D1 loses the third-party cache. The recipe stage keeps a manifest-only context, but
nothing is hand-listed:

1. Manifests reach the recipe stage generically: either a `.dockerignore` allow-list rule admitting
   `**/Cargo.toml` and `Cargo.lock`, or `COPY --parents ./**/Cargo.toml ./` (Dockerfile labs
   syntax; check Podman/buildah support before choosing it).
2. A repository-owned Rust tool runs inside the recipe stage before `cargo chef prepare`. It parses
   each manifest and touches every declared target path (`[lib]`, `[[bin]]`, `[[bench]]`,
   `[[example]]`, `[[test]]` with explicit `path`) plus `src/lib.rs` when a package declares no
   target, so `cargo metadata` can resolve every package.
3. Auto-detected targets (`src/bin/*.rs`, `benches/*.rs`, `examples/*.rs`) are invisible without the
   source tree. That is harmless: `cargo metadata` only aborts on declared targets whose file is
   missing or on packages with no targets at all. Extra or missing stub targets change only
   workspace-crate fingerprints, which are rebuilt from real source in the build stage anyway;
   `recipe-thirdparty.json` depends only on external entries.

Cost: one more repository-owned tool executed inside the container build. That is why it is the
fallback rather than the default.

### D3: `.dockerignore` allow-list

Default-deny (`*`) followed by explicit negations for what the `Containerfile` reads:
`Cargo.toml`, `Cargo.lock`, `.cargo/`, `packages/`, `console/`, `src/`, `tests/`, `share/`,
`contrib/dev-tools/su-exec/`, and, per D7, the harness crate directories that are still workspace
members. Everything else (`.github/`, `docs/`, compose files, linter configs, `.tmp/`, `storage/`,
`target/`) is excluded by default and needs no individual entry. The `Containerfile` header keeps
its `related-artifacts` link so the two files are reviewed together.

### D4: positive list for container test archives

`[workspace] default-members` lists the tracker packages (root crate plus every `packages/*` crate
that is part of the product or its tests). Every `cargo nextest archive` invocation drops
`--workspace` and the `--exclude` flags, so it builds exactly the default members. `cargo chef cook`
keeps `--workspace` because it must warm the full recipe.

Accepted side effect: at the repository root, `cargo build`, `cargo test`, and `cargo clippy`
without `--workspace` currently act on the root crate only (Cargo's default when the root manifest
is a package); with `default-members` they act on all listed tracker packages. CI workflows, git
hooks, and `linter` already pass `--workspace`, so they are unaffected. A single `ARG` holding a
`-p` list was considered as an alternative that avoids the side effect but keeps the list in Docker
rather than Cargo; rejected because the only goal of the `ARG` was deduplication, which
`default-members` achieves natively.

The tracker verification tools (`console/tracker-client`, `e2e-tools`, `persistence-benchmark`,
`torrent-repository-benchmarking`) remain outside `default-members`: excluding them from container
archives is a deliberate test-scope decision, not a build-integrity problem, and it is unchanged by
this issue.

Implementation finding (D8): `default-members` entries must also be explicit `members`. Cargo
normally auto-discovers `packages/*` crates through the root crate's path dependencies, but the
`--external-only` skeleton that `cargo chef cook` builds has those path dependencies stripped, so
the auto-discovered crates are absent from the skeleton workspace and Cargo rejects the
`default-members` entry. The first `test_debug` build on D4 failed this way; listing every in-repo
crate in `members` fixed it and makes membership explicit and reviewable.

### D5: ADR

`docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md`
covers: the three-layer cook, the `--external-only` recipe and the `torrust-cargo-chef` fork, the
canonical recipe stage and its measured cache behaviour, the allow-list build context, the
`default-members` archive policy, explicit membership (D8), and the D2 fallback with its trigger
condition. Indexed in `docs/adrs/index.md`.

### D6 and D7: AI-harness workspace

Draft spec: `docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md`
(`status: draft`, `epic: 2003`). It proposes a separate Cargo workspace for the harness tools,
owned by Cameron through EPIC #2003. It records the goal (decoupling while the harness
matures in-repo, with extraction to its own repository possible later, following the
`torrust-linting` precedent) and lists as EPIC decisions: workspace location (for example
`contrib/ai-harness/`), what counts as harness, unified binary versus multiple binaries, scripts
versus Rust. It lists the migration costs: second `Cargo.lock` for Dependabot, `cargo deny`, and
`cargo machete`; duplicated lint policy; `cargo run -p …` calls in hooks and CI becoming
`--manifest-path` calls; `linter clippy`/`rustfmt` in `torrust-linting` running only the root
workspace; a second CI build cache. It also records that root `[workspace].exclude` removes the
harness from `cargo metadata`, which deletes the D7 interim entries. No GitHub issue is created by
this work.

## Retired Options

Kept for traceability; none is selected. See the decision summary for why.

1. **Repository-owned validator as an immediate guardrail**
   - Add a Rust check under `contrib/dev-tools/checks/` that reads Cargo metadata and validates the
     expected Containerfile and `.dockerignore` entries for explicit developer-tool members.
   - Keep the Containerfile lists manual but make drift fail locally and in CI with a targeted
     diagnostic.
   - Advantage: smallest adoption risk and preserves the current Dockerfile structure.
   - Drawback: duplicated lists remain; the validator must parse or constrain Containerfile syntax.

2. **Generate a checked-in Containerfile fragment from Cargo metadata and classification metadata**
   - Add per-package metadata such as `package.metadata.torrust.container-role = "developer-tool"`
     and generate the manifest-copy, stub, and archive-exclude blocks from Cargo metadata.
   - Commit generated output or generate it during validation and reject drift.
   - Advantage: replaces several duplicated lists with a structured authority.
   - Drawback: introduces generation workflow and requires a deliberate format/stability policy.

3. **Replace manual recipe lists with a generated Cargo Chef preparation input**
   - Create a repository-owned Rust tool that generates a minimal recipe workspace or manifest/stub
     tree from Cargo metadata, then invoke Cargo Chef from that result.
   - Retain Containerfile orchestration while moving target enumeration out of Dockerfile syntax.
   - Advantage: target stubs derive directly from Cargo metadata.
   - Drawback: needs careful Docker-context design; generated input must remain available before
     the expensive dependency layers run.

4. **Use BuildKit bind mounts or a broader context only for metadata discovery**
   - Make source metadata available to the recipe stage without maintaining allow-list exceptions.
   - Advantage: fewer `.dockerignore` exceptions.
   - Drawback: may weaken cache isolation, depend on BuildKit features, and reduce Podman/buildah
     portability; document a portable alternative if chosen.

5. **Reclassify or isolate developer tools outside the explicit workspace**
   - Run some tools as independent manifests so the tracker workspace recipe does not enumerate
     them.
   - Advantage: removes the integration point for tools that do not need workspace membership.
   - Drawback: may lose shared lockfile, linting, and workspace dependency benefits; evaluate only
     if tool isolation is otherwise justified.

6. **Container build redesign using a data file**
   - Define a structured, checked-in manifest of container-test inclusion classes and generate both
     Containerfile recipe blocks and archive exclusions from it.
   - Advantage: explicit reviewable policy for production, test, and developer-only crates.
   - Drawback: adds another artifact and requires clear ownership relative to Cargo metadata.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce and inventory drift | Root-cause analysis and manual-edit matrix recorded in this spec (2026-09-29). |
| T2 | DONE | Evaluate solution options | Maintainer review retired options 1, 2, 4, 5, 6; selected D1-D7 with option 3 as fallback. |
| T3 | DONE | Measure baseline cache behaviour | V1 in `manual-verification-evidence.md`: M5 all cooks `CACHED`; M6a all cooks `CACHED`; M6b (feature toggle) both cooks rebuilt. |
| T4 | DONE | Implement D1 canonical recipe stage | `COPY . /build/src`; both `cargo chef prepare` invocations kept; manifest-copy and stub lists deleted; stage comments rewritten. |
| T5 | DONE | Measure D1 cache behaviour | V2: identical to baseline in M5, M6a, M6b. Gate passed. |
| T5b | DONE | Generated-stubs fallback (only if T5 fails) | Not needed. Design kept in the ADR's alternatives as the documented fallback. |
| T6 | DONE | Implement D3 allow-list `.dockerignore` | Default-deny with explicit inclusions; interim harness block (D7) marked for removal by the EPIC #2003 sub-issue; `AGENTS.md` files re-excluded inside admitted directories. |
| T7 | DONE | Implement D4 `default-members` positive list | `default-members` added; all in-repo crates listed explicitly in `members` (D8); `--workspace` and all `--exclude` flags removed from the four `cargo nextest archive` commands; comments updated. |
| T8 | IN_PROGRESS | Verify container targets | `recipe` and `test_debug` pass (V3: 1121 tests / 38 binaries, identical to baseline). `test` (release) was interrupted by a host restart and `runtime` is pending; both rerun by the maintainer outside the IDE. |
| T9 | DONE | Write D5 ADR | `docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md`, indexed. |
| T10 | DONE | Draft D6 EPIC #2003 sub-issue spec | `docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md`. |
| T11 | DONE | Update contributor workflow | `add-workspace-member` skill v2.0: no `Containerfile` edits; `default-members` decision; allow-list rule; explicit membership. `docs/containers.md` does not describe the recipe stage, so no change. |
| T12 | IN_PROGRESS | Record container verification | V1-V3 recorded; V4 (release/runtime), M1-M3, M7, and M4 (hosted workflow) pending. |
| T13 | TODO | Complete review | Reconcile acceptance criteria; record the implementation completion review. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Spec update with root-cause analysis and decisions | Documentation-only commit before implementation. |
| T3 | Baseline measurement evidence | Documentation-only commit. |
| T4-T5 | Canonical recipe stage plus its measurement | One commit; if T5 fails, the fallback lands as separate commits without rewriting this one. |
| T6 | Allow-list `.dockerignore` | Own commit after a `--target recipe` build. |
| T7 | `default-members` and archive commands | Own commit after `--target test_debug`. |
| T9 | ADR | Documentation-only commit. |
| T10 | EPIC #2003 draft sub-issue spec | Documentation-only commit. |
| T11 | Skill and docs updates | Documentation commit after Markdown validation. |
| T12-T13 | Evidence and completion reconciliation | Documentation-only commits after direct observation. |

## Progress Tracking

### Workflow Checkpoints

- [x] Temporary draft created in `.tmp/rust-dev-tool-container-integration/`.
- [x] Draft moved to `docs/issues/drafts/` and reviewed by the maintainer.
- [x] GitHub issue #2298 created and issue number added to this specification.
- [x] Implementation approach selected after draft review.
- [x] Baseline and post-change cache measurements recorded.
- [ ] Automatic verification completed.
- [ ] Manual container verification scenarios recorded in issue-local `manual-verification-evidence.md`.
- [x] ADR written and indexed.
- [x] EPIC #2003 draft sub-issue spec written.
- [ ] Acceptance criteria reviewed after implementation.
- [ ] Implementation completion review recorded.

### Progress Log

- 2026-09-22 10:07 UTC - GitHub Copilot - Drafted from the recurring developer-tool Container
  integration problem observed in PR #2293. The current manual repair and successful local
  68-stage container build are documented as evidence; no long-term solution has been selected.
- 2026-09-22 13:11 UTC - GitHub Copilot - Moved the reviewed draft to the canonical issue-drafts
  folder and prepared it for GitHub issue creation.
- 2026-09-22 13:17 UTC - GitHub Copilot - Created GitHub issue #2298 and moved this specification
  to the canonical open-issues folder.
- 2026-09-29 17:07 UTC - GitHub Copilot - Maintainer review of root causes. Recorded that the
  duplicated lists, not their synchronization, are the problem; that the cache property to protect
  is the #1869 `--external-only` third-party layer, not the #1852 manifest-only recipe stage (whose
  measurements were never run); and that the AI-harness tools do not belong in the tracker
  workspace. Selected D1-D7 (canonical recipe stage gated on measurement, generated-stubs fallback,
  allow-list `.dockerignore`, `default-members` positive archive list, ADR, EPIC #2003 draft
  sub-issue). Retired the validator, generated-fragment, bind-mount, and data-file options.
  Branch `2298-rust-dev-tool-container-integration` created.
- 2026-09-29 18:50 UTC - GitHub Copilot - Measured M5/M6 on the baseline and on D1 (V1, V2):
  identical cache behaviour; the D1 gate passed and D2 was not implemented. Implemented D1, D3,
  D4; `test_debug` passes with identical scope (1121 tests / 38 binaries). Found D8: every in-repo
  crate must be an explicit member because the `--external-only` skeleton strips path
  dependencies. Wrote the ADR, the `add-workspace-member` skill v2.0, the EPIC #2003 draft
  sub-issue spec, and the evidence record. Release `test` build was interrupted by a host restart
  (Docker saturated the workstation while the IDE ran); the maintainer reruns `test` and `runtime`
  outside the IDE.

## Acceptance Criteria

- [ ] AC1: Adding, moving, or removing a tracker package or one of its targets requires no
  `Containerfile` change; adding a developer-only explicit workspace member requires no
  `Containerfile` change and no `.dockerignore` change beyond the interim D7 entries.
- [ ] AC2: Omissions fail fast and visibly: a package or target unreachable to `cargo metadata`
  fails the `recipe` stage with Cargo's own diagnostic, and no silent stale-list state exists.
- [ ] AC3: Measured on a warm cache: a source-only change keeps every cook stage `CACHED`; a
  workspace `Cargo.toml`-only change keeps the third-party cook stages `CACHED`. Any regression
  versus the baseline is measured and explicitly accepted or triggers D2.
- [ ] AC4: Developer-only tools are absent from the runtime image and from container nextest
  archives; inclusion is controlled by the positive list `[workspace] default-members`.
- [ ] AC5: Library-only, binary-only, and mixed-target packages, including benches and examples,
  need no per-target maintenance.
- [ ] AC6: The `add-workspace-member` skill directs contributors to the positive lists
  (`default-members`, `.dockerignore` allow-list) and no longer documents `Containerfile` repairs.
- [ ] AC7: The Docker build context is a default-deny allow-list.
- [ ] AC8: An ADR records the container caching and positive-list model.
- [ ] AC9: A draft sub-issue spec for the separate AI-harness workspace exists for EPIC #2003.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant automated tests pass.
- [ ] Manual verification scenarios are executed and recorded.

## Verification Plan

### Automatic Checks

- `cargo metadata --no-deps` succeeds and `default-members` resolve to existing packages.
- If D2 is implemented: focused tests for the stub generator (lib-only, bin-only, mixed, explicit
  `path` targets, package with no targets) and a deterministic negative test.
- `linter all`.
- `cargo test --doc --workspace`.
- Pre-push checks when implementation changes are ready to publish.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Reproduce current failure | On the pre-change `Containerfile`, add a disposable workspace member without recipe entries; `docker build --target recipe --file Containerfile .` | `cargo chef prepare` fails with the `cargo metadata` missing-manifest diagnostic. | TODO | `manual-verification-evidence.md#V5` |
| M2 | New package needs no container edit | On the new `Containerfile`, add a disposable lib+bin package under `packages/`, register it as a member and default member; build `recipe` and `test_debug`. | Both targets pass with no `Containerfile` or `.dockerignore` edit; archive contains the package only because it is a default member. | TODO | `manual-verification-evidence.md#V5` |
| M3 | Developer-only member stays out | Add the disposable package as a member but not a default member. | `recipe` passes; `test_debug` archive does not contain its tests; runtime image unchanged. | TODO | `manual-verification-evidence.md#V5` |
| M4 | Hosted container workflow | Push the implementation to a fork PR. | Container workflow passes on the self-hosted runner. | TODO | `manual-verification-evidence.md#V6` |
| M5 | Warm cache, source-only change | Warm build; edit one `.rs` file; rebuild `--target test_debug`. Run on baseline and new `Containerfile`. | All cook stages `CACHED`; only build stages rerun. Record wall time. | DONE | `manual-verification-evidence.md#V1`, `#V2` |
| M6 | Warm cache, workspace manifest change | Warm build; toggle a feature in a `packages/*/Cargo.toml`; rebuild `--target test_debug`. Run on baseline and new `Containerfile`. | `dependencies_thirdparty*` `CACHED`; full cook stubs rebuild only. Record wall time. | DONE | `manual-verification-evidence.md#V1`, `#V2` (comment change: all cooks `CACHED`; feature toggle: both cooks rebuild in both designs, expected) |
| M7 | Build-context allow-list | `docker build --target recipe` with a `RUN find . -maxdepth 2` probe, or inspect context via `docker buildx build --progress=plain`. | Only allow-listed paths are present; `docs/`, `.github/`, `.tmp/`, `storage/` absent. | TODO | `manual-verification-evidence.md#V5` |

## Risks and Trade-offs

- D1 depends on BuildKit content-addressed caching of `COPY --from`; if it does not hold, every
  commit would rebuild external dependencies. Mitigated by the M5/M6 gate and the D2 fallback.
- `default-members` changes what bare `cargo build`/`cargo test` do at the repository root.
  Accepted; all automation passes `--workspace`.
- The allow-list must admit everything the `Containerfile` reads; a forgotten path fails the build
  visibly, which is the preferred failure mode.
- Until the EPIC #2003 sub-issue lands, harness crates still need allow-list entries (D7); this is
  the residual manual surface and is documented as such.
- `COPY --parents` (D2 option) is Dockerfile labs syntax; Podman/buildah support must be checked
  before relying on it.
- Removing `--workspace` from `cargo nextest archive` while keeping it on `cargo chef cook` is
  deliberate; the asymmetry must be explained in the `Containerfile` so it is not "fixed".

## Implementation Completion Review

After implementation, compare the selected mechanism against this draft. Record invalidated
assumptions, observed cache/runtime effects, migration deviations, and reusable lessons. Create an
issue-local retrospective if the selected design or migration has material consequences; otherwise
add a concise progress-log entry explaining why one is unnecessary.

## References

- PR #2293: https://github.com/torrust/torrust-tracker/pull/2293
- Container repair commit: `a9b4723677d7f6edbe34680176f2fd9cda2b4c0f`
- Earlier recipe repairs: `93e194361`, `94f2441a9`, `5938a1ee2`, `b6ddf879e`, `e4d7da576`,
  `a45ae2d41`, `a871c7f7b`
- Issue evidence record: `docs/issues/closed/2222-1347-package-coverage-regression-ci/ISSUE.md`
- Manifest-only recipe stage (unmeasured): `docs/issues/closed/1852-1840-workflow-performance-recipe-stage-manifest-only-copy/ISSUE.md`
- `--external-only` third-party recipe: `docs/issues/closed/1869-1840-workflow-performance-dependency-layer-cache-reuse/ISSUE.md`
- AI-harness organization: `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`
- Cargo Chef README (canonical planner stage, benefits vs manual manifest lists): https://github.com/LukeMathWalker/cargo-chef
- Docker build cache invalidation (`COPY` checksums exclude `mtime`): https://docs.docker.com/build/cache/invalidation/
- Cargo `default-members`: https://doc.rust-lang.org/cargo/reference/workspaces.html#the-default-members-field
- Existing manual workflow: `.github/skills/dev/maintenance/add-workspace-member/SKILL.md`
- Cargo Chef recipe and archive policy: `Containerfile`
- Docker build-context policy: `.dockerignore`
