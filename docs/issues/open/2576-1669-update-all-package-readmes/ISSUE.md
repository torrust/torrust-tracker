---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1669
github-issue: 2576
spec-path: docs/issues/open/2576-1669-update-all-package-readmes/ISSUE.md
branch: "2576-1669-update-all-package-readmes-spec"
related-pr: null
last-updated-utc: "2026-10-20 16:21"
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - "issue #1669"
    - "issue #2482"
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/readme-audit.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - docs/packages.md
    - Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #2576 - Update all package READMEs and crates.io metadata before the first publish

Parent EPIC: #1669 (Overhaul: Packages).

## Goal

Before any workspace crate is first published, give every publishable crate a `README.md` and
`Cargo.toml` metadata that describe that crate accurately on crates.io and docs.rs: its own title,
purpose, place in the workspace, documentation link, license, description, keywords and
categories.

## Background

The [README audit](../../open/1669-overhaul-packages/readme-audit.md) (first written for SI-01,
refreshed on 2026-10-06 for #2446) rates the 25 tracker packages: 4 good, 12 minimal, 9 stub.
Three titles are wrong, and 14 READMEs link the root crate's documentation
(`docs.rs/crate/torrust-tracker/latest`) instead of their own.

A check of the current workspace (`cargo metadata --no-deps`, 2026-10-20) adds these facts:

- **Package set.** 31 workspace members. 22 can be published; 4 of them are already on
  crates.io at 3.0.0 (`torrust-tracker`, `torrust-tracker-configuration`,
  `torrust-tracker-primitives`, `torrust-tracker-test-helpers`) and 18 have never been
  published. Three tracker packages set `publish = false` (`e2e-tools`,
  `persistence-benchmark`, `torrent-repository-benchmarking`), as do the six
  `contrib/dev-tools/` members.
- **`documentation`.** Every tracker crate inherits `documentation.workspace = true`, which is
  `https://docs.rs/crate/torrust-tracker/`, so crates.io links each crate to the root crate's
  page. The crates already extracted from this workspace set their own URL, for example
  `torrust-clock` uses `https://docs.rs/torrust-clock/`. The Cargo reference says crates.io
  links a crate to its own docs.rs page when the field is absent.
- **`keywords`.** crates.io accepts at most 5 keywords per crate. Seven crates declare 6 or 7:
  `axum-health-check-api-server` (7), `axum-http-server`, `axum-rest-api-server`,
  `rest-api-application`, `rest-api-protocol`, `rest-api-runtime-adapter` and `udp-server`
  (6 each). Several crates use filler keywords such as `library`, `rust` or `api`.
- **`categories`.** Only `swarm-coordination-registry` and `torrent-repository-benchmarking`
  set any.
- **License links.** `rest-api-client`, `tracker-client` (`torrust-tracker-client-lib`) and
  `console/tracker-client` are `license = "LGPL-3.0"`, but their READMEs link
  `../../LICENSE`, which is the repository's AGPL-3.0 text. `"LGPL-3.0"` is the deprecated SPDX
  identifier for `LGPL-3.0-only`, which those READMEs name.
- **Stale content.** The `torrent-repository-benchmarking` README runs
  `cargo bench -p torrust-tracker-torrent-repository`, a package that no longer exists. Four
  READMEs (`http-core`, `tracker-core`, `udp-core`, `tracker-client`) and the console client carry
  a "not ready for production" disclaimer that needs a decision before publishing.

### Ordering

- **Renames: done.** The draft waited for the rename issues. All are closed: #1816, #1819, #1821,
  #1823 (2026-05-21/22), #1829 (2026-05-27) and #1910 (2026-06-19). The extractions it was meant
  to precede (clock, metrics, located-error, net-primitives, server-lib) are done too; only the
  client extraction remains, and it comes after the publish.
- **Before the first publish.** The maintainer decided on 2026-10-20 that the unpublished crates
  are published at the end of EPIC #1669, one crate at a time, through a separate publish
  tracking issue. crates.io shows the README and metadata of each published version and they
  cannot be changed without a new release, so this issue must be merged before the first crate
  of that tracking issue is published.
- **After #2482.** Sub-EPIC #2482 (reorganize shared test support) can change the public API of
  `axum-http-server`, `axum-rest-api-server`, `udp-server`, `axum-health-check-api-server`
  (public `testing`/`environment` modules), `test-helpers` and `tracker-core` (`Mock*` types,
  crate-private test helpers), and may add support crates. The maintainer decided on 2026-10-20
  that implementation starts only after #2482 is closed, so READMEs describe the final API and
  any crates #2482 adds are covered too.

## Scope

Package tiers used below:

| Tier | Meaning                                                    | Packages                                                                                                                                                                                                                                                                                                                                |
| ---- | ---------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| U    | Publishable, never published (18)                          | `axum-health-check-api-server`, `axum-http-server`, `axum-rest-api-server`, `axum-server`, `events`, `http-core`, `http-protocol`, `rest-api-application`, `rest-api-client`, `rest-api-protocol`, `rest-api-runtime-adapter`, `swarm-coordination-registry`, `tracker-client`, `tracker-core`, `udp-core`, `udp-protocol`, `udp-server`, `console/tracker-client` |
| P    | Already published at 3.0.0 (4)                             | root `torrust-tracker`, `configuration`, `primitives`, `test-helpers`                                                                                                                                                                                                                                                                   |
| N    | `publish = false` tracker packages (3)                     | `e2e-tools`, `persistence-benchmark`, `torrent-repository-benchmarking`                                                                                                                                                                                                                                                                 |

The maintainer's answers of 2026-10-20 set this scope:

| Question                    | Decision                                                                                                       |
| --------------------------- | -------------------------------------------------------------------------------------------------------------- |
| Pull requests               | One implementation PR with one commit per crate (README and metadata together).                                |
| `documentation` field       | Remove it, so crates.io links each crate to its own docs.rs page.                                              |
| `keywords` and `categories` | In scope.                                                                                                      |
| Published crates (tier P)   | `configuration`, `primitives` and `test-helpers`: README and metadata. Root `torrust-tracker`: metadata only. |
| `console/tracker-client`    | Metadata and the license-link fix here; its README is otherwise already good.                                  |
| README code examples        | Allowed only when compile-checked by a README doctest hook.                                                    |
| LGPL crates                 | Fix the license links and use `LGPL-3.0-only`.                                                                 |
| Start                       | After #2482 is closed.                                                                                         |
| Template                    | Add a reusable `docs/templates/PACKAGE-README.md`.                                                             |

### In Scope

- **Package README template.** Add `docs/templates/PACKAGE-README.md` that encodes the
  [README quality bar](#readme-quality-bar), and list it in `docs/templates/README.md`.
- **README content.** Every tier U README except `console/tracker-client`, and the
  `configuration`, `primitives` and `test-helpers` READMEs, follow the template and meet the
  quality bar.
- **`Cargo.toml` metadata, tiers U and P (root crate included).** `description`, `keywords` and
  `categories` meet the [metadata rules](#cargotoml-metadata-rules).
- **`documentation` removal.** Remove `documentation.workspace = true` from every member that
  inherits it (tiers U, P and N) and remove `documentation` from `[workspace.package]` in the
  root `Cargo.toml`. A member that still inherits a removed workspace key fails to build, so tier
  N is included in this change.
- **README doctest hook.** In each crate whose README has a Rust code example, add a
  `#[cfg(doctest)]` item documented with `include_str!("../README.md")` to the crate root, so
  `cargo test --doc` compiles and runs the README examples. Non-Rust fences in those READMEs are
  tagged (`sh`, `toml`, `text`) so rustdoc does not test them.
- **Tier N READMEs.** Fix wrong titles, stale commands and broken links only. No crates.io
  metadata work, because these crates are never published.
- **License sections.** Each README's license section matches the crate's `license` field and
  links a text of that license. The repository has no LGPL text, so the three LGPL READMEs link
  `https://www.gnu.org/licenses/lgpl-3.0.html`. Replace the deprecated `LGPL-3.0` SPDX
  identifier with the equivalent `LGPL-3.0-only` in `rest-api-client`, `tracker-client` and
  `console/tracker-client`.
- **Readme audit.** Record a post-implementation re-audit in issue-local evidence and refresh
  `readme-audit.md` (point-in-time document, new date) to match.

### Out of Scope

- The root `README.md` content (already rated good; covers the application, not a library).
- `AGENTS.md`, `docs/packages.md` and other repository docs. READMEs link `docs/packages.md`
  instead of duplicating it.
- Crate-level rustdoc (`//!` docs) and API reference docs.
- Code, test or public API changes, except the README doctest hook.
- Changing any crate's license, adding license files, renaming crates or bumping versions.
- Publishing any crate (publish tracking issue) and extracting the console client.
- The six `contrib/dev-tools/` members.

### README Quality Bar

For each README in scope:

1. **Title.** `# <Title>` derived from the crate name, for example `torrust-tracker-udp-core` is
   "Torrust Tracker UDP Core", followed by the crate name in code in the first paragraph.
2. **Purpose.** One paragraph: what the crate does, what it does not do, and who should depend
   on it. Crates that exist only to build the tracker say so and point to the tracker.
3. **Where it fits.** Its layer (`*-protocol`, `*-core`, server, `axum-*`, client, domain) and
   its main workspace dependencies and dependents in one or two sentences, with a link to
   `docs/packages.md` for the full picture.
4. **Main API.** The few public types, traits or modules a user starts from, or a link to
   docs.rs when the list would only repeat it.
5. **Usage.** A short example for crates meant for reuse outside the tracker (for example the
   protocol crates, `primitives`, `rest-api-client`, `tracker-client`). Rust examples are
   compile-checked through the README doctest hook; a crate without the hook has no Rust code
   blocks in its README.
6. **Stability.** An accurate statement for a `0.x` or `3.x` crate, replacing the current
   disclaimers.
7. **Documentation.** A link to the crate's own docs.rs page (`https://docs.rs/<crate>`), plus
   links to package docs that ship with it (ADRs, benchmarking notes).
8. **License.** Matches the `license` field and links the right text.
9. **Links work on GitHub and crates.io.** Links to files outside the package directory use
   absolute `https://github.com/torrust/torrust-tracker/...` URLs, because only the package
   directory is packaged.

### Cargo.toml Metadata Rules

| Field           | Rule                                                                                                                       |
| --------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `description`   | One accurate plain-text sentence about the crate (crates.io requires it).                                                 |
| `keywords`      | At most 5; ASCII, at most 20 characters; no filler (`library`, `rust`, `api`); no repeat of words already in the name. |
| `categories`    | 1 to 5 valid crates.io slugs (`https://crates.io/category_slugs`).                                                         |
| `documentation` | Absent, in members and in `[workspace.package]`; crates.io then links the crate's own docs.rs page.                      |
| `readme`        | `README.md` (already set everywhere; keep).                                                                                |

## Architectural Decisions

- Related ADRs:
  [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
  says the crate README and `Cargo.toml` metadata are each workspace crate's documentation
  surface on crates.io.
- ADRs to create: none known. The `documentation` field choice and the README quality bar are
  recorded in this specification and in the new `docs/templates/PACKAGE-README.md` template.

## Design and Ownership Review

Not applicable: documentation and manifest metadata only.

## Bug-Fix Process

Not applicable: the stale titles, links and keyword counts are documentation defects fixed by
rewriting the documents; no runtime behavior is wrong.

## Regression Test Strategy

Not applicable. The metadata rules are checked by the commands in the verification plan.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                                                               | Notes / Expected Output                                                                                                                                      |
| --- | ------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE    | Confirm the rename issues are closed                               | #1816, #1819, #1821, #1823, #1829 and #1910 are closed (checked 2026-10-20).                                                                                 |
| T2  | BLOCKED | Wait for #2482 to close, then re-baseline the package set          | Blocked on #2482. Record the `cargo metadata` tiers (including any crate #2482 adds) and the README baseline in `manual-verification-evidence.md`.           |
| T3  | TODO    | Add the package README template                                    | `docs/templates/PACKAGE-README.md` encodes the quality bar (use the `create-markdown-template` skill); listed in `docs/templates/README.md`.                  |
| T4  | TODO    | Remove the inherited `documentation` field                         | No member sets or inherits `documentation`; `[workspace.package]` has no `documentation` key; the workspace builds.                                         |
| T5  | TODO    | Update each tier U and P crate, stubs first, then minimal, then good | Per crate: README from the template, metadata rules, license section, doctest hook when the README has Rust examples. Root crate: metadata only. Console client: metadata and license fix only. |
| T6  | TODO    | Fix the tier N READMEs                                             | Correct titles; the `cargo bench` command names an existing package; links work.                                                                             |
| T7  | TODO    | Re-audit and refresh `readme-audit.md`                             | No tier U or P package rated stub or minimal; audit dated and summarized.                                                                                    |
| T8  | TODO    | Run verification and review acceptance criteria                    | Automatic checks pass; manual scenarios recorded in `manual-verification-evidence.md`.                                                                      |

## Commit Points

| Task | Coherent change set                                       | Commit policy                                                                                        |
| ---- | --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| T2   | Baseline in `manual-verification-evidence.md`             | One `docs(issues)` commit.                                                                           |
| T3   | Package README template and template index                | One `docs(templates)` commit.                                                                        |
| T4   | `documentation` removal in the root and every member      | One `chore(workspace)` commit: removing the workspace key breaks any member that still inherits it. |
| T5   | One crate's README, metadata, license and doctest hook    | One `docs(<package>)` commit per crate.                                                              |
| T6   | One tier N README                                         | One `docs(<package>)` commit per crate.                                                              |
| T7   | Re-audit evidence and `readme-audit.md`                   | One `docs(issues)` commit.                                                                           |

All commits go into one implementation pull request. Every commit is a signed Conventional
Commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-update-all-package-readmes/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] #2482 closed before implementation starts
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, metadata check, `cargo package --list`, pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-18 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669; uses
  readme-audit.md baseline from SI-01. Ordered after renaming (SI-07-SI-10) and before
  extraction (SI-16+).
- 2026-10-20 16:21 UTC - GitHub Copilot (Claude Opus 5.5) - Refreshed the stale draft: added
  strict frontmatter (`schema-version`, `epic: 1669`); marked the rename prerequisite done
  (#1816, #1819, #1821, #1823, #1829, #1910 closed); based the scope on the 2026-10-06 README audit and
  `cargo metadata --no-deps` (22 publishable crates, 4 published); added the ordering before the
  first publish and after #2482; added the keyword-limit, `documentation`, category and LGPL
  license-link findings; replaced the nonexistent audit tool with concrete checks.
- 2026-10-20 16:21 UTC - GitHub Copilot (Claude Opus 5.5) - Applied the maintainer's answers (one PR with
  one commit per crate; remove `documentation`; keywords and categories in scope; tier P and
  console client scope; compile-checked README examples; LGPL fix; start after #2482; add a
  package README template). The maintainer approved the draft. Created #2576, linked it as a
  subissue of #1669, and moved the spec to `docs/issues/open/` for a spec-only PR on branch
  `2576-1669-update-all-package-readmes-spec`.

## Acceptance Criteria

- [ ] AC1: Every tier U and P README in scope meets the README quality bar, and every tier N
      README has a correct title, current commands and working links.
- [ ] AC2: Every tier U and P crate has a `description`, at most 5 valid `keywords` and 1 to 5
      valid `categories`; no workspace member sets or inherits `documentation`.
- [ ] AC3: Every README's license section matches its crate's `license` field and links that
      license's text; no crate uses the deprecated `LGPL-3.0` identifier.
- [ ] AC4: READMEs of crates whose public API #2482 changed describe the post-#2482 API.
- [ ] AC5: `readme-audit.md` is refreshed and rates no tier U or P package as stub or minimal.
- [ ] AC6: The issue is merged before the first publish of any tier U crate.
- [ ] AC7: `docs/templates/PACKAGE-README.md` exists, encodes the quality bar and is listed in
      `docs/templates/README.md`.
- [ ] AC8: Every README with Rust code examples is compile-checked by the README doctest hook,
      and `cargo test --doc --workspace` passes.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass (`cargo build --workspace` and `cargo test --doc --workspace`)
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all` (markdownlint, lychee local links, cspell, taplo).
- Metadata check, run from the repository root and recorded in the evidence:

  ```sh
  cargo metadata --no-deps --format-version 1 \
    | jq -r '.packages[] | select(.publish == null)
        | [.name, (.keywords | length), (.categories | length), .documentation, (.description // "MISSING")]
        | @tsv'
  ```

  Every row has at most 5 keywords, 1 to 5 categories, an empty `documentation` and a
  description. `grep -rn '^documentation' Cargo.toml packages/*/Cargo.toml console/*/Cargo.toml`
  prints nothing. Category slugs are compared with `https://crates.io/api/v1/category_slugs`.
- `cargo package -p <crate> --list` for every tier U and P crate lists `README.md`.
- `cargo build --workspace` (the `documentation` removal) and `cargo test --doc --workspace`
  (README doctests).
- `./contrib/dev-tools/git/hooks/pre-push.sh` before opening the implementation PR.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Human-oriented command/steps                                                                                    | Expected Result                                                              | Status | Evidence                                     |
| --- | ------------------------------------- | --------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Read each README as a new user        | Open each changed README on the pushed branch on GitHub; follow every link                                     | Quality bar met; every link resolves; title matches the crate               | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Read each README as crates.io shows it | For each tier U crate, run `cargo package -p <crate> --list` and read `README.md` with only the packaged files in mind | No link points to a file that is not packaged unless it is an absolute URL | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Re-audit                              | Re-rate every package with the audit's quality scale                                                            | No tier U or P package rated stub or minimal                                 | TODO   | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated checks pass.
- Record the toolchain for each command (for example `stable Rust toolchain` for `cargo package`).
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The metadata check is a one-line `cargo metadata | jq` command recorded in the
evidence.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |

## Risks and Trade-offs

- **Drift before publish.** READMEs written now can go stale before the first publish, mainly
  because of #2482. Mitigation: start after #2482, or run T6, and re-check at publish time from
  the publish tracking issue.
- **Duplicating `docs/packages.md`.** Dependency context in every README can drift from the
  canonical package doc. Mitigation: one or two sentences plus a link.
- **Unchecked examples.** Code blocks in READMEs are not compiled unless a doctest hook includes
  them. Mitigation: Rust examples are allowed only with the README doctest hook.
- **Large diff.** About 25 READMEs and 25 manifests in one pull request. Mitigation: one commit
  per crate, so each crate can be reviewed on its own.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- Independent reviewers record results in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1669, [`EPIC.md`](../../open/1669-overhaul-packages/EPIC.md)
- Input: [`readme-audit.md`](../../open/1669-overhaul-packages/readme-audit.md)
- Ordering: #2482 ([`EPIC.md`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md)), the
  publish tracking issue (not yet created)
- Rename issues: #1816, #1819, #1821, #1823, #1829, #1910
- Cargo reference: [The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
