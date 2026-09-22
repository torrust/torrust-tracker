---
schema-version: 1
doc-type: issue
issue-type: bug
status: open
priority: p2
epic: null
github-issue: 2298
spec-path: docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md
branch: "2298-rust-dev-tool-container-integration-spec"
related-pr: 2293
last-updated-utc: 2026-09-22 13:17
semantic-links:
  skill-links:
    - create-issue
    - add-workspace-member
  related-artifacts:
    - Cargo.toml
    - Containerfile
    - .dockerignore
    - .github/skills/dev/maintenance/add-workspace-member/SKILL.md
    - docs/issues/open/2222-1347-package-coverage-regression-ci/ISSUE.md
---

# Issue #2298 - Eliminate Manual Container Integration for Rust Developer Tools

## Goal

Make adding a Rust developer tool to the Cargo workspace reliable without repeatedly repairing the
Docker build recipe by hand. The eventual solution must keep developer-only tools out of the
production tracker image and container test archives while ensuring Cargo Chef can resolve the
whole workspace.

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
- Decide and implement one maintainable mechanism that prevents or detects omissions before hosted
  Container CI fails.
- Preserve Cargo Chef dependency-layer caching where practical.
- Preserve exclusion of developer-only checks, analysis tools, benchmarks, CLI tools, and host-only
  E2E tools from production tracker test archives unless an individual tool has a documented reason
  to be included.
- Add automated validation for the selected mechanism and update the workspace-member workflow
  documentation.

### Out of Scope

- Adding a new developer tool.
- Changing the tracker runtime image contents or production deployment behavior.
- Removing Cargo Chef, Docker, nextest archives, or `.dockerignore` without an independently
  approved container-build redesign.
- Deciding that every workspace member belongs in container test archives.
- Retrofitting unrelated path dependencies unless analysis proves they share the same fault.

## Architectural Decisions

- Related guidance: `.github/skills/dev/maintenance/add-workspace-member/SKILL.md` already
  documents the manual checklist.
- Related implementation evidence: PR #2293 and its Container workflow repair.
- ADRs to create: `None known`. Create one if the chosen solution establishes a new generated-file
  authority, changes Docker build-context policy, or changes the container trust/performance model.

## Design and Ownership Review

This work owns build metadata and validation only; it does not introduce runtime child-process or
network-lifetime behavior.

- Cargo metadata remains the authority for workspace packages and declared targets.
- The selected mechanism must have one documented source of truth and a clear owner for generated
  or checked artifacts.
- The solution must distinguish recipe reachability from archive inclusion: every workspace member
  may need metadata visibility, while only production-relevant members belong in container archives.
- The first vertical slice must add one disposable or representative developer tool and demonstrate
  both a passing container build and a deterministic failure when the mechanism is intentionally
  bypassed, where practical.

## Bug-Fix Process

1. Reproduce the current failure using a workspace member omitted from the recipe build context or
   from required target stubs.
2. Capture the Cargo Chef/Cargo metadata diagnostic in issue-local manual evidence.
3. Add a smallest deterministic automatic check at the selected ownership boundary.
4. Demonstrate that the check fails for the omission before relying on hosted CI.
5. Implement the selected source-of-truth or validation mechanism.
6. Re-run the same container target and hosted Container workflow.

## Regression Test Strategy

The preferred regression boundary is a repository-owned Rust or shell-free validation tool that
compares Cargo metadata with the effective recipe/build-context policy. It must fail deterministically
when a developer-only explicit workspace member lacks required recipe visibility, required target
stubs, or a synchronized archive decision.

A direct Container build remains mandatory integration evidence because Cargo Chef, Docker ignore
semantics, and nextest archives interact across tools. The selected solution should additionally
make the common omission fail faster than a full hosted Docker build.

## Proposed Solutions

No option is selected in this draft. Evaluate implementation complexity, cache behavior, portability,
reviewability, failure quality, and migration cost before choosing one.

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

The selected option may combine an immediate validator with later generation, but the issue must
record why each retained manual surface remains necessary.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce and inventory drift | Record the Cargo Chef failure and derive the full current manual-edit matrix from Cargo metadata, `.dockerignore`, and Containerfile. |
| T2 | TODO | Evaluate solution options | Compare the proposed options against cache behavior, Docker/Podman portability, test scope, and maintenance cost; obtain maintainer decision. |
| T3 | TODO | Design selected mechanism | Specify the source of truth, classification model, ownership boundaries, migration path, and failure diagnostics. Create an ADR if warranted. |
| T4 | TODO | Implement prevention or detection | Add the selected tool/generator/validation and migrate current developer-only members. |
| T5 | TODO | Add automated tests | Cover a representative developer tool, target variants, archive policy, and an intentional mismatch. Perform the required test-design review. |
| T6 | TODO | Update contributor workflow | Update `add-workspace-member` guidance and canonical container documentation without duplicating procedural detail. |
| T7 | TODO | Record container verification | Record local recipe/test targets and hosted Container workflow evidence in issue-local manual evidence. |
| T8 | TODO | Complete review | Reconcile acceptance criteria, observe cache/runtime effects, and record the implementation completion review. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Reproduction and inventory evidence | Documentation-only commit after evidence review. |
| T3 | Approved design or ADR | Documentation-only commit before implementation when it changes repository policy. |
| T4 | Prevention/detection mechanism and migration | Commit after focused validation. |
| T5 | Automated regression coverage | Commit each reviewed test increment separately when it improves reviewability. |
| T6 | Workflow and documentation updates | Documentation commit after Markdown validation. |
| T7-T8 | Hosted evidence and completion reconciliation | Documentation-only commits after direct observation. |

## Progress Tracking

### Workflow Checkpoints

- [x] Temporary draft created in `.tmp/rust-dev-tool-container-integration/`.
- [x] Draft moved to `docs/issues/drafts/` and reviewed by the maintainer.
- [x] GitHub issue #2298 created and issue number added to this specification.
- [ ] Implementation approach selected after draft review.
- [ ] Automatic verification completed.
- [ ] Manual container verification scenarios recorded in issue-local `manual-verification-evidence.md`.
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

## Acceptance Criteria

- [ ] AC1: Adding or removing a developer-only explicit Rust workspace member cannot silently
  leave Cargo Chef recipe metadata, Docker build context, target stubs, or archive policy stale.
- [ ] AC2: The selected mechanism provides a fast, actionable local diagnostic before hosted
  Container CI when its required metadata is missing or inconsistent.
- [ ] AC3: Cargo Chef dependency-layer caching remains materially equivalent or any regression is
  measured and explicitly accepted.
- [ ] AC4: Developer-only tools remain absent from the final production tracker image and from
  container nextest archives unless an explicit documented classification includes them.
- [ ] AC5: The mechanism handles library-only, binary-only, and mixed-target developer tools.
- [ ] AC6: The add-workspace-member workflow directs contributors to the selected mechanism and
  no longer relies on an undocumented repeated manual repair.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant automated tests pass.
- [ ] Manual verification scenarios are executed and recorded after the draft becomes an open issue.

## Verification Plan

### Automatic Checks

- Focused tests for the selected validator or generator.
- A deterministic negative test for an omitted manifest, target stub, or archive decision.
- `linter all`.
- `cargo test --doc --workspace`.
- Pre-push checks when implementation changes are ready to publish.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Reproduce current failure | Use a disposable representative developer-tool member while omitting one required current-manual integration entry. | The new local check or existing recipe fails with a diagnostic that identifies the missing surface. | TODO | `manual-verification-evidence.md#M1` |
| M2 | Developer-only tool integration | Add a representative library-and-binary developer tool through the selected mechanism. | Recipe and `test_debug` targets pass; the tool is not in the production test archive. | TODO | `manual-verification-evidence.md#M2` |
| M3 | Production-relevant member control | Classify a representative production-relevant member. | It remains available to required container build and test stages. | TODO | `manual-verification-evidence.md#M3` |
| M4 | Hosted container workflow | Push the final implementation to a fork PR. | The Container workflow passes; logs confirm the selected mechanism runs with actionable output. | TODO | `manual-verification-evidence.md#M4` |

## Risks and Trade-offs

- Generating Dockerfile fragments can obscure the final image recipe unless generated output is
  reviewable and drift is checked.
- Parsing a Containerfile is brittle unless the file contains deliberately constrained markers or
  structured generated blocks.
- Broader Docker build context can reduce cache efficiency or portability and must be benchmarked.
- Treating every workspace member as developer-only would risk excluding needed production tests;
  classification must be explicit and reviewable.
- Separating tools from the workspace may move rather than solve dependency-lock and linting costs.

## Implementation Completion Review

After implementation, compare the selected mechanism against this draft. Record invalidated
assumptions, observed cache/runtime effects, migration deviations, and reusable lessons. Create an
issue-local retrospective if the selected design or migration has material consequences; otherwise
add a concise progress-log entry explaining why one is unnecessary.

## References

- PR #2293: https://github.com/torrust/torrust-tracker/pull/2293
- Container repair commit: `a9b4723677d7f6edbe34680176f2fd9cda2b4c0f`
- Issue evidence record: `docs/issues/open/2222-1347-package-coverage-regression-ci/ISSUE.md`
- Existing manual workflow: `.github/skills/dev/maintenance/add-workspace-member/SKILL.md`
- Cargo Chef recipe and archive policy: `Containerfile`
- Docker build-context policy: `.dockerignore`
