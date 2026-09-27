---
schema-version: 1
doc-type: issue
issue-type: feature
status: in-progress
priority: p1
epic: 2264
github-issue: 2281
spec-path: docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md
branch: "2281-frontmatter-validator-command"
related-pr: 2337
last-updated-utc: "2026-09-26 21:57"
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
    - write-unit-test
    - run-pre-commit-checks
  related-artifacts:
    - "issue #2264"
    - "issue #2266"
    - "issue #2280"
    - "issue #2003"
    - contrib/dev-tools/checks/frontmatter-validator
    - contrib/dev-tools/checks/clippy-allow-reasons
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-v1-contract.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
---

<!-- skill-link: create-issue -->

# Issue #2281 - Add Frontmatter Validator Command and Pre-Commit Rollout

**Parent EPIC:** #2264 - Refactor Semantic Link and Frontmatter Conventions

**Predecessors:**
[#2266 - Implement the Rust Frontmatter Model and Initial Validator](../../closed/2266-2264-implement-rust-frontmatter-model-and-validator/ISSUE.md)
and [#2280 - Generate V1 Schema and Verify Drift](../../closed/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md)

## Goal

Give humans, AI agents, and the pre-commit hook a read-only `frontmatter-validator` command that
applies the approved v1 frontmatter contract to explicit paths, staged files, or the whole tracked
tree, and reports stable NDJSON diagnostics on stderr with contract-defined exit codes.

## Background

Issue #2266 delivered the canonical Rust v1 model in the non-published
`contrib/dev-tools/checks/frontmatter-validator` crate: frontmatter extraction, the universal
`semantic-links` envelope, strict issue and EPIC profiles, and the frozen provisional reference
syntax. At its mandatory split checkpoint it moved T7-T8 here: command modes, diagnostic rendering,
location/lifecycle and `x-` warning rules, pre-commit integration, and fixture, mutation, and manual
portability proof. Issue #2280 then added the generated schema and its `frontmatter-schema` binary.

The library is therefore complete for structural validation but has no command a person, agent, or
hook can run. It also returns a single `Diagnostic` without severity or field path, and it cannot
see repository location. The [v1 contract](../../closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-v1-contract.md)
makes severity location-dependent (strict errors in drafts/open, advisory warnings in closed) and
assigns lifecycle/location consistency, path existence, and skill discovery to a repository-aware
layer that this issue introduces.

The integration precedent is `clippy-allow-reasons`: a non-published crate invoked by
`pre-commit.sh` through `cargo run --quiet --package ... -- --staged`, classified
`no-stdout-result` under the CLI output contract ADR.

A measurement on `develop` at 2026-09-24 found 822 tracked Markdown files, 680 with frontmatter,
and 20 with `schema-version: 1`: two templates, six crate fixtures, six closed specs, one draft, and
five open specs. Most repository specs are therefore legacy records.

## Scope

### In Scope

- A new read-only `frontmatter-validator` binary in the existing crate, beside
  `frontmatter-schema`.
- Three mutually exclusive invocation modes: explicit file or directory paths, `--staged`, and
  `--all` (manual whole-tree validation).
- Candidate discovery over tracked Markdown, path-based ownership dispatch for Agent Skills and
  agent profiles, and a built-in exclusion list.
- Diagnostic severity and field path in the library `Diagnostic`, rendered as NDJSON on stderr.
- Location-dependent severity, the contract's three warning kinds, and repository-aware checks for
  strict v1 issue and EPIC records.
- A named, read-only pre-commit step running `--staged`.
- Fixture, mutation, and manual portability evidence for the command boundary.
- Documentation of the command, its temporary integration point, and how it can move into the
  architecture later selected by #2003.
- Fixing genuine errors the first `--all` run reports in the existing draft/open v1 specs, in
  separate commits.
- Migrating the seven legacy open EPIC records to v1 (D11), and any other legacy draft/open spec
  that the implementation PR itself modifies.
- A short v1 migration checklist that the `legacy-shape` message points to.

### Out of Scope

- The final #2003 automation architecture, binary or package selection, orchestration, shared
  runners, caches, or policy composition.
- JSON Schema generation, its tracked artifact, regeneration, and drift verification (#2280).
- CI, pre-push, nightly, release, or agent-policy integration.
- New frontmatter fields, profiles, or semantic-link forms. Extending strict profiles to ADRs,
  skills, agents, and evidence records is EPIC #2264 row 3.
- Existence or skill-resolution checks for closed, legacy, unknown, or externally governed
  documents.
- Validating `docs/templates/` against the v1 contract. Their placeholders cannot pass strict
  validation, so template-drift protection is deferred to EPIC #2264 row 3.
- Bulk migration of legacy draft/open specs. Under D10 each one migrates when its next change is
  committed.
- Rewriting closed or other historical records to remove warnings. Historical records are never
  rewritten solely to silence advisory diagnostics.
- Auto-fixing or formatting frontmatter.
- Validating ordinary Markdown links, which remain Lychee's responsibility.

## Architectural Decisions

Maintainer decisions recorded on 2026-09-24, before implementation:

- **D1 - Workflow.** Spec-first. This specification merges through a spec-only PR from
  `2281-frontmatter-validator-command-spec`. Implementation starts only after that merge, on
  `2281-frontmatter-validator-command`.
- **D2 - Binary shape.** Add a separate `frontmatter-validator` binary in the existing crate
  (`src/bin/frontmatter-validator.rs`). Do not merge it with `frontmatter-schema`. The schema
  command writes a tracked artifact; the validator is strictly read-only, and #2003 requires
  checks and repository-mutating actions to stay distinguishable. Keeping them apart also avoids
  reopening the command #2280 just hardened. Both binaries share the library.
- **D3 - Argument parsing.** Use `clap` with derive, already used by the workspace and the
  `github-review-threads` dev tool. Select the current compatible 4.x release under the dependency
  freshness policy. Exactly one mode is required: positional paths, `--staged`, or `--all`. No
  arguments, or a combination of modes, is a usage error. Follow the `github-review-threads`
  precedent: call `try_parse` and never let clap write to stdout. Every clap outcome becomes one
  D9 record on stderr: `--help` is a `help` record with exit `0`, and every parse failure is a
  `usage_error` record with exit `2`. The command has no `--version` flag, so `--version` is an
  ordinary usage error.
- **D4 - Discovery.**
  - Candidates are tracked `*.md` files (`git ls-files`).
  - A directory argument expands to the tracked Markdown files under it.
  - An explicit file argument is read from disk even when untracked. A nonexistent explicit path
    is a usage error (exit `2`).
  - `--staged` selects added, copied, modified, and renamed staged Markdown files and validates
    their index content, not the working copy.
  - `--all` selects every tracked Markdown file.
- **D5 - Ownership dispatch.** Files named `SKILL.md` or ending in `.agent.md` use
  `DocumentOwnership::External`. Every other file uses `DocumentOwnership::Repository`.
- **D6 - Exclusions.** A built-in exclusion list skips `docs/templates/` and
  `contrib/dev-tools/checks/frontmatter-validator/fixtures/` silently in every mode. These files
  intentionally carry placeholder or invalid `schema-version: 1` records; the crate's own tests
  own the fixtures. Consequence: nothing in this issue detects template drift from the v1
  contract; EPIC #2264 row 3 owns that follow-up.
- **D7 - Repository-aware checks.** Apply these only to strict v1 issue and EPIC records:
  - `status` matches the spec's location: `draft` in `docs/issues/drafts/`, neither `draft` nor
    `done` in `docs/issues/open/`, `done` in `docs/issues/closed/`;
  - `spec-path` equals the file's repository-relative path;
  - in drafts/open only, each repository-path entry in `related-artifacts` exists as a tracked
    file or as a directory containing tracked files (`issue #` and `review-finding:` entries are
    syntax-only);
  - in drafts/open only, each `skill-links` name resolves to a tracked
    `.github/skills/**/<name>/SKILL.md`.

  `--staged` resolves existence against the index file list, read once with `git ls-files`; a
  directory exists when some index entry lies under it. The other modes resolve against the
  working tree. Closed specs are historical, and their paths go stale by design, so they get only
  the status and `spec-path` checks, as warnings. Legacy, unknown, and external documents receive
  no repository-aware checks.
- **D8 - Severity and exit status.**
  - Any present frontmatter: an unclosed delimiter, malformed YAML, a non-mapping root, or an
    invalid universal envelope is an `error` everywhere.
  - Strict v1 records under `docs/issues/drafts/` or `docs/issues/open/`: profile, scalar,
    allowed-value, reference-syntax, and D7 failures are `error`s.
  - Strict v1 records under `docs/issues/closed/`: profile, scalar, allowed-value, and
    reference-syntax failures, plus the closed-location D7 checks, are `warning`s (advisory).
  - Strict v1 records elsewhere: structural failures are `error`s; the location check in D7 does
    not apply.
  - `legacy-shape` is an `error` for a draft/open document that is not a strict v1 record and
    either is a primary spec (`ISSUE.md` or `EPIC.md`, with or without frontmatter) or declares
    `doc-type` `issue` or `epic` (D10). Supporting files such as evidence records and plans stay
    permissive.
  - Warning kinds: advisory closed-spec incompatibility, and `experimental-field` for each `x-`
    field in a strict profile.
  - Exit `0` when there are no errors, even if warnings were emitted; `1` for any validation error
    or runtime failure (git, I/O); `2` for invalid invocation.
- **D9 - NDJSON record catalog.** Add `severity` and an optional `field_path` to the library
  `Diagnostic`. Stdout stays empty in every mode. Stderr carries only the records below, each one
  JSON object per line. Every record has a snake_case `kind` and a `message`, like the
  `usage_error` and `runtime_error` records of `clippy-allow-reasons`. Every listed field is always
  present; a field marked nullable is `null` when not applicable. No other fields are emitted.

  | `kind` | Fields in order | Exit status it implies |
  | ------ | --------------- | ---------------------- |
  | `diagnostic` | `kind`, `path`, `severity` (`error` or `warning`), `category` (kebab-case), `field_path` (nullable), `message` | `1` if any `severity` is `error`, otherwise `0` |
  | `usage_error` | `kind`, `message`, `exit_code` (`2`) | `2` |
  | `runtime_error` | `kind`, `path` (nullable), `message`, `exit_code` (`1`) | `1` |
  | `help` | `kind`, `message` (the rendered clap help text) | `0` |

  - `path` is the repository-relative path with `/` separators; `field_path` is a dotted YAML
    path such as `semantic-links.related-artifacts`.
  - Diagnostic categories are the D8/T1 category names, for example `wrong-scalar-type`,
    `legacy-shape`, `experimental-field`, and `missing-artifact`.
  - A `usage_error` or `help` record is the only record of its run. A `runtime_error` ends the
    run, after any diagnostics already emitted.
  - A successful run without warnings emits nothing.
  - Records appear in a deterministic order: files sorted by `path`, then, within a file,
    structural findings, warnings, and repository-aware findings.
  - If writing to stderr fails, the command exits `1` without a record.

  Examples:

  ```json
  {"kind":"diagnostic","path":"docs/issues/open/1-example/ISSUE.md","severity":"error","category":"invalid-allowed-value","field_path":"status","message":"`status` must be one of: draft, planned, in-progress, blocked, in-review, done."}
  {"kind":"diagnostic","path":"docs/issues/open/2-example/ISSUE.md","severity":"error","category":"legacy-shape","field_path":null,"message":"Draft/open issue specs must use the v1 frontmatter; see the migration checklist."}
  {"kind":"usage_error","message":"the argument '--staged' cannot be used with '--all'","exit_code":2}
  {"kind":"runtime_error","path":null,"message":"`git ls-files` failed: not a git repository","exit_code":1}
  {"kind":"help","message":"Validate Markdown frontmatter..."}
  ```

  The field names, `kind` values, and nullability are the contract; the message texts in these
  examples are illustrative. T2 pins the catalog with tests.
- **D10 - Progressive migration.** New issues are strict from the start, because the templates
  already produce `schema-version: 1` records. This issue does not migrate the existing legacy
  draft/open specs (50 primary `ISSUE.md`/`EPIC.md` files on 2026-09-24). Instead, `legacy-shape`
  is an error, so the pre-commit `--staged` step rejects a commit that changes a legacy draft/open
  spec until its author migrates the frontmatter to v1. After this issue merges, contributors
  whose branches edit such a spec will see this error on their next commit after rebasing onto
  `develop`; the maintainer accepts that. Commits that do not stage a legacy spec are unaffected.
  Archiving a spec into `docs/issues/closed/` stages it at the closed location, where only advisory
  warnings apply, so archival never requires migration.

  Migrating a legacy spec also brings in the D7 checks, so a small edit may require fixing stale
  `related-artifacts` or `skill-links` in the same commit. The `legacy-shape` message names the
  file and points to a short migration checklist in the crate documentation (T7). The checklist
  covers:
  - copying the frontmatter shape from `docs/templates/ISSUE.md` or `docs/templates/EPIC.md`;
  - prefixing or dropping fields outside the profile;
  - quoting `last-updated-utc` and every `issue #<n>` reference;
  - repairing stale references.

  The primary-spec rule closes a bypass: without it, a new `ISSUE.md` or `EPIC.md` written with no
  frontmatter or no `doc-type` would escape strict validation. All 50 current legacy primary specs
  declare a `doc-type`, so the rule adds no new errors on 2026-09-24.

  This deliberately tightens the approved
  [v1 contract](../../closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-v1-contract.md),
  which classifies the legacy shape as a warning. The closed contract stays unchanged as the
  historical record; this decision supersedes that one table row. The convention-split subissue
  (EPIC #2264 row 4) carries the rule into the owned convention documents.
- **D11 - Migrate legacy open EPICs in this issue.** Almost every subissue workflow edits its
  parent EPIC, so under D10 the first contributor to touch a legacy EPIC would have to migrate a
  shared document they do not own. This issue therefore migrates the seven legacy open EPIC
  records:
  - `docs/issues/open/1347-overhaul-packages-testing/EPIC.md`;
  - `docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md`, which declares `doc-type: epic`
    and is not renamed here;
  - `docs/issues/open/1669-overhaul-packages/EPIC.md`;
  - `docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md`;
  - `docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`;
  - `docs/issues/open/2243-review-numeric-conversion-boundaries/EPIC.md`;
  - `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md`.

  The unassigned draft EPIC `docs/issues/drafts/generalize-error-events/EPIC.md` and the 42 legacy
  draft/open issue specs remain under D10's progressive rule.
- Related ADRs: [`docs/adrs/20260519000000_define_global_cli_output_contract.md`](../../../adrs/20260519000000_define_global_cli_output_contract.md).
  Register `frontmatter-validator` as `no-stdout-result` in that ADR's binary classification table.
- ADRs to create: none expected. The temporary placement is already approved early work under
  #2003. Create an ADR only if implementation needs a durable, repository-wide choice beyond D1-D11.

## Design and Ownership Review

- **Library (`frontmatter-validator` crate):** owns extraction, envelope, strict profiles,
  reference syntax, and the `Diagnostic` vocabulary, now including severity and field path. It
  stays free of filesystem and git I/O.
- **Repository-aware layer:** owns location classification, severity policy (D8), the
  status/location and `spec-path` checks, and existence and skill resolution through a
  narrow resolver. That resolver has two implementations: the index for `--staged` and the
  working tree otherwise.
- **Command adapter (binary):** owns argument parsing, discovery, exclusions, ownership dispatch,
  content sourcing (disk or index), NDJSON rendering, and exit status. It is the only component
  that writes to stderr or spawns `git`.

Child processes: the binary runs short, synchronous `git` commands (`ls-files`, `diff --cached`,
and `show :<path>`) and waits for each one to finish. It needs no network,
asynchronous readiness, or persistent resources. A failed or non-zero `git` invocation is a
runtime failure (exit `1`) with an NDJSON record. Tests that need a repository create a disposable
`git init` repository inside a `TempDir`. The `TempDir` owns it, it is removed on drop, and it
never touches the real repository's index.

The library returns the first structural failure per document. The command reports that failure
plus all warnings and repository-aware findings it can still evaluate for the document; it does not
attempt multi-error structural recovery.

**Vertical-slice checkpoint.** After T2 (explicit-path mode with NDJSON rendering, end to end),
stop and review these boundaries before building discovery, severity policy, and repository checks.
Record the result in the progress log.

## Bug-Fix Process

The issue is a feature, but T1b fixes a bug in the merged #2266 library found by
`review-finding:pr-2337-f1`: an unquoted `issue #<n>` related artifact parses as `issue` and is
accepted as a path. It follows [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

1. the hypothesis;
2. the reproduction with an independent YAML parse of the accepted fixtures;
3. a red unit regression test;
4. the fix;
5. the green run and a like-for-like recheck.

These are recorded in `manual-verification-evidence.md` section B1.

## Regression Test Strategy

The unit test `it_should_reject_an_unquoted_issue_reference_that_yaml_truncates_to_issue` sits at
the causal seam, the `RelatedArtifact` value type exercised through strict-profile validation. The
accepted-fixture tests are a second guard: they fail if a fixture reverts to the unquoted form.
No higher boundary is needed.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Extend the diagnostic vocabulary and fix unquoted issue references | Add `Severity` and optional `field_path` to `Diagnostic`, stable kebab-case category names, and the new command/repository categories. Existing library tests keep their outcomes; field paths are populated where the library already knows the field. Also fix the latent #2266 defect found by `review-finding:pr-2337-f1`. YAML treats `#` after whitespace as a comment, so an unquoted `issue #2264` entry parses as the path `issue`. The accepted fixtures and the strict-reference unit test in `profile.rs` therefore never exercise the issue-reference form. Quote those references, and add an `invalid-reference-syntax` diagnostic, with a regression test, for a `related-artifacts` entry that the source shows as unquoted `issue #<n>`. |
| T2 | DONE | Add the command and explicit-path mode | New `frontmatter-validator` binary with `clap` (D3), explicit file paths, NDJSON rendering (D9), and exit codes. Stop for the vertical-slice checkpoint. |
| T3 | DONE | Add discovery, `--staged`, and `--all` | Directory expansion, tracked-file discovery, index-content sourcing, ownership dispatch (D5), and exclusions (D6). |
| T4 | DONE | Apply location-dependent severity and warnings | D8 severity policy, the `legacy-shape` error (D10), and the closed-spec advisory and `experimental-field` warnings. |
| T5 | DONE | Add repository-aware checks | D7 status/location, `spec-path`, artifact existence, and skill resolution through index and working-tree resolvers. |
| T6 | DONE | Run a whole-tree baseline, migrate open EPICs, and triage findings | Run `--all` on `develop`. Migrate the seven D11 EPIC records. Fix genuine errors in existing v1 draft/open specs. Record the `legacy-shape` error count and the warning counts. Do not bulk-migrate legacy issue specs (D10). |
| T7 | DONE | Integrate with pre-commit and document | Add the named `--staged` step to `pre-commit.sh`. Update the `run-pre-commit-checks` skill and `docs/git-hooks.md`, and register the binary in the CLI output ADR table. Add usage, relocation notes, and the D10 migration checklist to the crate. |
| T8 | IN_PROGRESS | Prove failures and portability | Command-boundary accepted/rejected fixtures and mutation cases, manual scenarios M1-M7, acceptance review, and independent Task Reviewer report. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Diagnostic severity, field path, and category vocabulary; separately, the unquoted issue-reference fix | Two commits, each after focused library tests and prose-first test-design review. The reference fix follows the `fix-bug` red/green regression proof. |
| T2 | Binary, argument parsing, explicit-path mode, NDJSON rendering | Commit after focused command tests and the vertical-slice review. |
| T3 | Discovery, `--staged`, `--all`, ownership dispatch, exclusions | Commit after TempDir git-repository tests. |
| T4 | Severity policy and warning kinds | Commit after focused accepted/rejected cases per location. |
| T5 | Repository-aware checks and resolvers | Commit after index and working-tree resolver tests. |
| T6 | Each D11 EPIC migration, each genuine current-tree spec fix, and each other legacy spec migration this PR requires | One `docs(issues)` commit per EPIC migration or coherent fix; no commit if none is needed. |
| T7 | Pre-commit step and documentation | Commit after the full pre-commit gate passes with the new step. |
| T8 | Mutation and manual evidence, review records | Commit after full quality gate and review. |

For every test-producing increment, use the `write-unit-test` skill and record the mandatory
prose-first Arrange-Act-Assert design review before commit. Confirm that each test exposes the one
causal initial-state difference, such as the location, the mode, or the single offending field.
Stop for maintainer review after the final increment before final verification, commit, or pull
request. Use signed Conventional Commits with the `frontmatter` scope and the `[#2281]` reference.

## Progress Tracking

### Workflow Checkpoints

- [x] GitHub issue #2281 created as the command/integration follow-up from #2266
- [x] Local folder-style specification created
- [x] Specification reviewed and approved by user/maintainer
- [x] Spec-only PR #2337 merged into `develop` before implementation
- [x] Vertical-slice design review recorded after T2
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, pre-commit gate)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md`
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-24 16:01 UTC - GitHub Copilot - Confirmed #2281 as the only open native sub-issue of
  #2264 and created this local specification from the GitHub issue body. Recorded maintainer
  decisions D1-D9 and added the missing #2281 row to the parent EPIC - This specification
- 2026-09-24 16:20 UTC - Jose Celano - Chose progressive migration: new issues are strict, legacy
  draft/open specs are not bulk-migrated, and `legacy-shape` becomes an error. Contributors
  therefore migrate a legacy spec when they next commit a change to it, including after rebasing
  onto `develop`. Recorded as D10, which tightens the #2265 contract's legacy-shape warning -
  User conversation
- 2026-09-24 16:45 UTC - GitHub Copilot - Applied maintainer-approved review changes:
  `legacy-shape` also covers primary specs without v1 frontmatter, closing a new-spec bypass; D11
  migrates the seven legacy open EPIC records here; closed specs skip existence and
  skill-resolution checks; index existence uses the `git ls-files` list so directories resolve; a
  migration checklist is required; template-drift protection is deferred to EPIC row 3 -
  User conversation
- 2026-09-24 17:20 UTC - GitHub Copilot - Maintainer approved the specification; opened spec-only
  PR #2337 and recorded it in frontmatter - https://github.com/torrust/torrust-tracker/pull/2337
- 2026-09-24 20:05 UTC - GitHub Copilot - Addressed `review-finding:pr-2337-f1`: quoted this
  spec's `issue #<n>` references, which YAML parsed as `issue`. The same latent defect in the
  #2266 accepted fixtures and strict-reference unit test is now part of T1. No other v1 draft/open
  spec uses the unquoted form - `docs/pr-reviews/pr-2337-review/PR-REVIEW.md`
- 2026-09-24 20:15 UTC - GitHub Copilot - Addressed `review-finding:pr-2337-fm-001`: D9 now
  defines the full NDJSON record catalog (`diagnostic`, `usage_error`, `runtime_error`, `help`)
  with field order, nullability, ordering, and examples. D3 drops `--version`, so it is a usage
  error - `docs/pr-reviews/pr-2337-review/PR-REVIEW.md`
- 2026-09-25 07:49 UTC - GitHub Copilot - Spec-only PR #2337 merged into `develop`. Started
  implementation on `2281-frontmatter-validator-command`; `related-pr` stays 2337 until the
  implementation PR opens - https://github.com/torrust/torrust-tracker/pull/2337
- 2026-09-25 08:10 UTC - GitHub Copilot - Completed T1a:
  - `Diagnostic` now carries `severity` (always `error` from the library) and `field_path`.
  - `DiagnosticCategory` and `Severity` serialize to stable kebab-case and lowercase names.
  - Every single-field library failure names its field. Serde scalar-type failures get their
    path from `serde_path_to_error` 0.1.20, the latest release, already in the lockfile through
    axum. `serde_yaml::from_value` drops paths, and without it the most common failure,
    `wrong-scalar-type`, would have no `field_path`.

  Deviation: the new command and repository categories are added with their behavior in T4/T5
  rather than as unused variants now. A mutation proved the field-path assertion. 47 library
  tests, Clippy, the nightly Rust formatting check, `cargo machete`, and the schema drift check
  pass - `test-design-review.md`
- 2026-09-25 08:40 UTC - GitHub Copilot - Completed T1b under the fix-bug workflow:
  - reproduced that both accepted fixtures parse their issue reference as `issue`;
  - a new regression test failed against the unfixed library;
  - the related-artifact predicate and its generated schema pattern now reject the bare `issue`
    value with a quoting hint;
  - the fixtures and the accept-all-forms test now quote their references.

  The regenerated schema differs by that one pattern line. 48 library tests pass - B1 in
  `manual-verification-evidence.md`, `test-design-review.md`
- 2026-09-26 09:30 UTC - GitHub Copilot - Completed T2 (vertical slice): a new
  `frontmatter-validator` binary with `clap` 4.6.7, the latest release and already in the
  lockfile, with color and suggestion features disabled. It has explicit file paths, D5 ownership
  dispatch, the four D9 record kinds, and the D8 exit codes. 14 binary test cases pass, and four
  mutations were each caught. Two notes:
  - The Containerfile's maintenance note asks for cargo-chef stubs for
    `src/bin/frontmatter-schema.rs` (missed in #2280) and the new binary. Nothing breaks without
    them, because cargo only discovers bin files that exist; T7 adds both with a recipe-stage
    build check.
  - `rstest` 0.27.0 was added as a dev-dependency in a separate test refactor commit.

  Evidence: `test-design-review.md`.
- 2026-09-26 09:30 UTC - GitHub Copilot - Vertical-slice design review, awaiting maintainer
  review. The boundaries held: the library only validates text, and the binary owns arguments,
  file reading, ownership dispatch, rendering, and exit status. Findings for T3-T5:
  - **R1 - Policy placement.** D7/D8 severity and repository checks are pure decisions over a
    repository-relative path, the parsed frontmatter, and a resolver. Proposal: a library
    `repository` module with a `Resolver` trait. It holds no I/O, so the rules are unit-tested
    without git. The binary supplies the index and working-tree resolvers.
  - **R2 - Multiple findings per document.** Warnings and repository findings coexist with at
    most one structural error, so the per-document result becomes a `Vec<Diagnostic>`.
    Structural failures still short-circuit the profile.
  - **R3 - Binary layout.** Discovery, git access, and resolvers would make one file too large.
    Proposal: move to `src/bin/frontmatter-validator/main.rs` with sibling modules in T3, and
    update the T7 Containerfile stub to that path.
  - **R4 - Paths outside the repository.** D9 renders repository-relative paths, but D4 did not
    say what happens to an explicit path outside the repository. Proposal: a `usage_error`
    (exit `2`), because such a file has no repository location for D8 or D7 to classify.

  This specification
- 2026-09-26 10:40 UTC - Jose Celano - Approved R1-R4 as proposed. The binary now lives at
  `src/bin/frontmatter-validator/main.rs`, and the record catalog is in `record.rs` (behavior
  unchanged). Git-dependent behavior is tested through the built binary in a disposable
  repository, following `clippy-allow-reasons/tests/cli.rs`. Those tests clear inherited
  `GIT_*` variables so a run inside a git hook cannot touch the real repository - User
  conversation
- 2026-09-26 11:01 UTC - GitHub Copilot - Completed T3:
  - Exactly one mode is required: paths, `--staged`, or `--all`.
  - Explicit paths resolve against the working directory. They must exist and be inside the
    repository (R4), and are rendered repository-relative with `/`.
  - Directories expand to tracked Markdown. `--all` covers tracked Markdown still in the working
    tree. `--staged` validates the index content of added, copied, modified, and renamed files.
  - D6 exclusions apply in every mode.
  - Git runs read-only with `--no-optional-locks`, and a missing repository is a `runtime_error`
    with a `null` path.

  Five mutations were each caught. 112 crate tests pass - `test-design-review.md`
- 2026-09-26 11:18 UTC - GitHub Copilot - Completed T4 in a new library module `repository`
  (R1):
  - Location classification, ownership dispatch (moved from the binary), and per-document
    validation return every diagnostic in order (R2).
  - Closed-spec profile findings become warnings, while syntax stays an error.
  - `legacy-shape` is an error for draft and open primary specs and for `issue`/`epic` records
    without `schema-version: 1` (D10). Externally governed files are exempt.
  - Each `x-` field in a strict profile gets an `experimental-field` warning.
  - Adds the `legacy-shape` and `experimental-field` categories.

  One mutation first survived because of a weak test; the test was strengthened and all five
  are now caught. 142 crate tests pass - `test-design-review.md`
- 2026-09-26 12:22 UTC - GitHub Copilot - Completed T5:
  - D7 lookups use a pure `RepositoryFiles` snapshot. The binary builds it from index entries
    for `--staged`, or from tracked files present in the working tree.
  - Drafts and open specs get the status, `spec-path`, related-artifact, and skill checks as
    errors. Closed specs get only status and `spec-path`, as warnings.
  - Adds four categories: `lifecycle-location-mismatch`, `spec-path-mismatch`,
    `missing-artifact`, and `unknown-skill`.
  - Clarification: D8 said only that the location check does not apply outside the lifecycle
    folders. The implementation applies no D7 check there, because no such spec has a lifecycle
    to check against.

  Six mutations were each caught. The whole-repository smoke run reports 50 `legacy-shape` errors
  (D10/D11), 14 structural errors in non-v1 documents for T6, 3 closed-spec warnings, and no D7
  findings in the open v1 specs - `test-design-review.md`
- 2026-09-26 12:40 UTC - Jose Celano - T6 baseline decisions:
  - Leave #2324 for the usual archive PR. It is completed but its spec is still in `open/`
    with `status: open`, so it is the one accepted non-`legacy-shape` error under AC10 until
    archived.
  - Fix the malformed YAML in four closed records, whitespace only.
  - Move the unsupported `related-issues`/`issue-spec` keys into `related-artifacts`.
  - Set #2283, #2298, and #2301 from `open` to `planned`.

  User conversation
- 2026-09-26 13:00 UTC - GitHub Copilot - Completed T6 in eleven signed `docs(issues)` commits:
  - the four approved fix groups;
  - the next errors each fix exposed: a trailing-slash artifact, two unquoted timestamps, and
    one stale #2222 reference;
  - seven D11 EPIC migrations, one commit each. They drop issue-only fields, map
    `open` to `in-progress` where subissues are completed, quote timestamps and issue
    references, and update three stale references.

  `--all` now reports:
  - 44 `legacy-shape` errors: 42 legacy issue specs, the unassigned draft EPIC, and one draft
    whose envelope error had masked its legacy shape;
  - the #2324 exception;
  - 3 closed-spec warnings (a pre-#2281 unquoted issue reference in #2280, and URL-valued
    `related-pr` in #2295 and #2308), left as advisory history.

  Evidence: `manual-verification-evidence.md` section V4 in T8.
- 2026-09-26 13:20 UTC - GitHub Copilot - Completed T7:
  - Pre-commit runs the named, read-only step `Checking staged Markdown frontmatter`, which
    takes about 1 s. The hook passed with the step validating the staged T7 files.
  - The crate README documents usage, the D10 migration checklist named in the `legacy-shape`
    message, and the #2003 relocation path.
  - The CLI output ADR classifies the binary as `no-stdout-result`.
  - The `run-pre-commit-checks` skill lists the step, and its step list was corrected: a wrong
    formatter path, and missing nightly-fmt and hadolint steps. `docs/git-hooks.md` lists no
    steps, so it was left unchanged rather than duplicate the skill.
  - Containerfile: cargo-chef stubs for both frontmatter binaries, verified with `docker build
    --target recipe`.
  - Finding: the crate was never in the container test-archive exclusions. The new
    `tests/cli.rs` needs `git`, which `rust:slim-trixie` lacks (checked with `docker run`), so
    the crate is now excluded like the other dev-tool crates.

  Implementation is complete; awaiting maintainer review before T8 - This specification
- 2026-09-26 13:25 UTC - Jose Celano - Acknowledged the T7 report and approved continuing
  with T8 - User conversation
- 2026-09-26 13:25 UTC - GitHub Copilot - T8 verification, pending the independent Task
  Reviewer:
  - Manual scenarios M1-M7 are recorded in `manual-verification-evidence.md` V1-V7. Failing
    cases ran in a disposable `.tmp/` worktree, and the offline runs used `unshare -rn`.
  - Two first attempts proved nothing and were redone: the reverse M3 case had nothing staged,
    and the first M5b edit tripped markdownlint MD012.
  - A command-boundary failure-family table was added for AC8, bringing the crate to 176
    tests.
  - All acceptance criteria were reviewed against evidence. AC10 holds with the accepted #2324
    exception.
  - Another completed issue, #2179, still has its spec in `open/` and is an archive candidate.
  - `implementation-retrospective.md` records the material discoveries.

  Evidence: `manual-verification-evidence.md`, `test-design-review.md`,
  `implementation-retrospective.md`.
- 2026-09-26 15:10 UTC - Task Reviewer - First pre-PR review: REVIEW FAILED. All AC1-AC11 are met
  and all gates pass, but two command-boundary tests carried a hidden `spec-path` mismatch
  masked by the structural error, and T8 lacked a prose-first comparison and mutation evidence.
  The review also raised minor and nit findings - `agent-review-reports.md`
- 2026-09-26 15:29 UTC - GitHub Copilot - Remediated the review findings:
  - **Finding 1:** the fixture tests now use the fixtures' own `spec-path`, and a
    `relocated_fixture` helper moves the closed case. The T8 prose-first comparison and four
    caught mutations are recorded.
  - **Finding 2, a correction of earlier entries:** the 44 `legacy-shape` errors are 42 primary
    `ISSUE.md` specs, the masked draft included, plus the draft EPIC and the #2230 supporting
    sample spec that declares `doc-type: issue`. The T5 and T6 entries above miscounted this. V4
    and the T5 test note are corrected in place, while those log entries stay as history. The
    #2230 sample also has `status: draft` in `open/`, so migrating it will need a lifecycle fix.
  - **Finding 3:** the README severity summary is complete.
  - **Finding 4:** retrospective improvements 1 and 3 are applied to the Containerfile
    maintenance comment and the `run-pre-commit-checks` skill, and the agent-memory claim is
    removed.
  - **Finding 5:** test isolation clears `GIT_CONFIG_PARAMETERS`/`GIT_CONFIG_COUNT`, and fixture
    commits disable hooks.
  - **Finding 6:** directory arguments use `:(literal)` path patterns and symlinked files keep their
    own path, both with regression tests.
  - **Finding 7:** "four approved fix groups" above means three fix commits plus the decision to
    leave #2324.
  - **Finding 8:** it predates this branch and is out of scope.

  178 tests pass - `test-design-review.md`
- 2026-09-26 21:57 UTC - GitHub Copilot - Follow-up review (2026-09-26 21:47 UTC): REVIEW FAILED
  on one new Major finding. Findings 1-5 and 7 were verified as fixed. Remediated:
  - **N1:** the finding-6 `:(literal)` prefix matched nothing under `GIT_LITERAL_PATHSPECS=1`,
    which some git front ends export to hooks, so a directory argument silently passed. Git now
    runs with its global `--literal-pathspecs` option. A regression test sets the variable, test
    isolation clears the git path-pattern-mode variables, and the full CLI suite also passes with the
    variable exported. Removing the option, and reintroducing the prefix, each failed the new
    test.
  - **N2:** `relocated_fixture` asserts that the fixture still declares what it replaces.
  - **N3:** the README says outside records get no repository-aware checks.
  - **N4, a correction:** the entry above stamped the first review at 15:10 UTC; its report is
    stamped 14:23 UTC.
  - **N5:** the README documents symlink handling.

  179 tests pass - `agent-review-reports.md`, `test-design-review.md`

## Acceptance Criteria

- [x] AC1: The command supports explicit file or directory paths, `--staged`, and a documented
      `--all` whole-tree mode; exactly one mode is required.
- [x] AC2: The command emits no stdout in any mode, including help and usage errors. Stderr carries
      only the D9 record catalog (`diagnostic`, `usage_error`, `runtime_error`, `help`), with
      every listed field always present, nullable fields as `null`, and a deterministic order;
      tests pin each record kind.
- [x] AC3: Exit codes are `0` for success with or without warnings, `1` for validation errors or
      runtime failure, and `2` for invalid invocation.
- [x] AC4: Severity follows D8 and D10: `legacy-shape` is an error for draft/open primary specs
      and `issue`/`epic` records that are not strict v1, including primary specs without
      frontmatter; closed-spec incompatibility and `experimental-field` are warnings.
- [x] AC5: Strict v1 records are checked for status/location, `spec-path`, related-artifact file or
      directory existence, and skill resolution per D7. `--staged` resolves against the index, and
      closed specs get only status and `spec-path` warnings.
- [x] AC6: Ownership dispatch (D5) and exclusions (D6) are applied in every mode.
- [x] AC7: Pre-commit invokes the validator as a named, read-only `--staged` step. No CI or other
      integration tier is added.
- [x] AC8: Accepted/rejected fixtures and mutation cases cover representative field, scalar,
      allowed-value, reference, lifecycle, and path failures within the command boundary.
- [x] AC9: Manual portability evidence demonstrates focused, staged, whole-tree, and pre-commit use
      without network access, including no-stdout behavior.
- [x] AC10: On the implementation branch, the seven D11 EPIC records are v1, and `--all` reports no
      errors other than `legacy-shape` for the legacy issue specs and the draft EPIC not yet
      migrated. The error and warning counts are recorded. Accepted exception: the completed
      #2324 spec's `status: open` until the usual archive PR moves it to `closed/`.
- [x] AC11: The command, its temporary integration point, its relocation path under #2003, and
      the D10 migration checklist are documented, and the CLI output ADR classifies the binary.
- [x] Focused tests, `linter all`, and the pre-commit gate exit with code `0`.
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `cargo test --package frontmatter-validator` (stable Rust toolchain).
- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain).
- `cargo clippy --package frontmatter-validator --all-targets -- -D warnings` (stable Rust
  toolchain).
- `linter all` and `./contrib/dev-tools/git/hooks/pre-commit.sh` with the new step.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Focused validation passes | Run the command on this spec and on its directory. | Exit `0`, empty stdout, no error records on stderr. | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Focused validation fails | Copy an open spec to a disposable path, corrupt a field, run the command on it. | Exit `1`, empty stdout, one NDJSON error naming path, category, and field path. | DONE | `manual-verification-evidence.md` section V2 |
| M3 | Staged mode uses index content | In a disposable worktree, stage an invalid spec, then fix only the working copy and run `--staged`. | Exit `1` from the staged content, and the working copy is ignored. | DONE | `manual-verification-evidence.md` section V3 |
| M4 | Whole-tree mode | Run `--all` on the implementation branch. | Exit `1` with only `legacy-shape` errors, plus the accepted #2324 exception; error and warning counts recorded. | DONE | `manual-verification-evidence.md` section V4 |
| M5 | Pre-commit step | In a disposable worktree, run `./contrib/dev-tools/git/hooks/pre-commit.sh` with a staged invalid v1 spec, then with it fixed. Repeat with a staged edit to a legacy open spec, then with its frontmatter migrated. | The named step fails, then passes, in both cases. | DONE | `manual-verification-evidence.md` section V5 |
| M6 | Invalid invocation and help | Run with no arguments, with `--staged --all`, with a nonexistent path, with `--version`, and with `--help`. | Exit `2`, `2`, `2`, `2`, `0`; stdout empty; exactly one D9 `usage_error` or `help` record each. | DONE | `manual-verification-evidence.md` section V6 |
| M7 | Offline | Repeat M1 and M4 with `cargo run --offline` and no network. | Same outcomes; nothing is downloaded. | DONE | `manual-verification-evidence.md` section V7 |

Record the toolchain for every command result. Disposable Git worktree checkouts live under `.tmp/`
and are removed after use; the real repository index is never used for failing scenarios.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Unit tests for the mode group (`main.rs`); `tests/cli.rs` explicit-file, directory, `--staged`, and `--all` tests; README usage; V1, V3, V4 |
| AC2 | DONE | `record.rs` catalog; key-order and `null` assertions in `tests/cli.rs` and `main.rs` for all four kinds; `Outcome` asserts empty stdout on every CLI run; ordering test; V6 |
| AC3 | DONE | `it_should_exit_one_only_when_a_record_is_an_error`; usage-error tests; V2, V6 |
| AC4 | DONE | `repository::tests` legacy-shape, severity-by-location, and experimental-field tables with T4 mutations; V5b |
| AC5 | DONE | `repository::tests` D7 tables with T5 mutations; `it_should_resolve_related_artifacts_against_the_index_only_with_staged` |
| AC6 | DONE | Ownership and exclusion tests in every mode (`repository::tests`, `tests/cli.rs`); T2/T3 mutations |
| AC7 | DONE | `pre-commit.sh` step `Checking staged Markdown frontmatter`; no CI or pre-push change; V5 |
| AC8 | DONE | Library fixture and mutation tables (T1-T5); `it_should_report_each_failure_family_as_one_error_record` at the command boundary, free of hidden state after the review remediation |
| AC9 | DONE | V1-V7, including `unshare -rn` offline runs (V7) and no-stdout checks |
| AC10 | DONE | V4: 44 `legacy-shape` errors, the accepted #2324 exception, 3 closed-spec warnings; seven EPIC migration commits |
| AC11 | DONE | Crate `README.md` (usage, migration checklist, relocation); CLI output ADR row; `run-pre-commit-checks` skill step |

## Risks and Trade-offs

- **Pre-commit latency.** `cargo run` compiles the binary on the first run. Mitigation: the crate
  is small and already built by workspace checks; `--staged` validates only staged Markdown.
- **Legacy specs block commits after rebase.** Contributors editing one of the 43 remaining legacy
  draft/open specs get a `legacy-shape` error on their next commit after this issue merges. The
  maintainer accepts this as the migration trigger (D10), and D11 removes the shared EPICs from
  that path. Mitigation: the message points to the migration checklist.
- **Migration cascades.** Migrating brings in the D7 checks, so a one-line edit may also require
  repairing stale references. Mitigation: the checklist names this step, and the diagnostics name
  each missing path or skill.
- **`--all` is not a clean pass/fail signal yet.** It exits `1` until every legacy draft/open spec
  is migrated. Mitigation: `--all` is manual only, and the pre-commit gate uses `--staged`.
- **Warning noise.** Closed specs may emit advisory warnings under `--all`, limited to profile,
  syntax, status, and `spec-path` findings. Mitigation: warnings never fail the command, and
  `--staged` only reports on files being changed. If the noise proves harmful, revisit it in the
  EPIC rather than rewriting historical records.
- **Template drift is undetected.** D6 excludes the templates. Mitigation: deferred to EPIC #2264
  row 3.
- **Existence checks can block unrelated commits.** Renaming a file referenced by a staged v1 spec
  could fail the hook. Mitigation: the checks apply only to strict v1 records in drafts/open, and
  the diagnostic names the missing path.
- **One structural error per file.** The library short-circuits. Mitigation: fixing and rerunning
  is fast, and multi-error recovery is out of scope.
- **Hard-coded exclusions and ownership rules.** They are simple but not configurable. Mitigation:
  they are named constants beside the dispatch code and can move into the #2003 policy layer later.

## Implementation Completion Review

- Retrospective: created as `implementation-retrospective.md`. The material discoveries were the
  container test-stage gap, masked errors under a first-error validator, out-of-spec baseline
  findings, and a pipe-hidden gate failure.
- The independent Task Reviewer records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2264; architecture owner: #2003
- Predecessors: #2265, #2266, #2280
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- Integration precedent: `contrib/dev-tools/checks/clippy-allow-reasons`
