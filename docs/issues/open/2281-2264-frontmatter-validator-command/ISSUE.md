---
schema-version: 1
doc-type: issue
issue-type: feature
status: planned
priority: p1
epic: 2264
github-issue: 2281
spec-path: docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md
branch: "2281-frontmatter-validator-command-spec"
related-pr: 2337
last-updated-utc: "2026-09-24 20:15"
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

Not applicable. This is new command-surface work.

## Regression Test Strategy

Not applicable. This is not substantively a bug.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Extend the diagnostic vocabulary and fix unquoted issue references | Add `Severity` and optional `field_path` to `Diagnostic`, stable kebab-case category names, and the new command/repository categories. Existing library tests keep their outcomes; field paths are populated where the library already knows the field. Also fix the latent #2266 defect found by `review-finding:pr-2337-f1`. YAML treats `#` after whitespace as a comment, so an unquoted `issue #2264` entry parses as the path `issue`. The accepted fixtures and the strict-reference unit test in `profile.rs` therefore never exercise the issue-reference form. Quote those references, and add an `invalid-reference-syntax` diagnostic, with a regression test, for a `related-artifacts` entry that the source shows as unquoted `issue #<n>`. |
| T2 | TODO | Add the command and explicit-path mode | New `frontmatter-validator` binary with `clap` (D3), explicit file paths, NDJSON rendering (D9), and exit codes. Stop for the vertical-slice checkpoint. |
| T3 | TODO | Add discovery, `--staged`, and `--all` | Directory expansion, tracked-file discovery, index-content sourcing, ownership dispatch (D5), and exclusions (D6). |
| T4 | TODO | Apply location-dependent severity and warnings | D8 severity policy, the `legacy-shape` error (D10), and the closed-spec advisory and `experimental-field` warnings. |
| T5 | TODO | Add repository-aware checks | D7 status/location, `spec-path`, artifact existence, and skill resolution through index and working-tree resolvers. |
| T6 | TODO | Run a whole-tree baseline, migrate open EPICs, and triage findings | Run `--all` on `develop`. Migrate the seven D11 EPIC records. Fix genuine errors in existing v1 draft/open specs. Record the `legacy-shape` error count and the warning counts. Do not bulk-migrate legacy issue specs (D10). |
| T7 | TODO | Integrate with pre-commit and document | Add the named `--staged` step to `pre-commit.sh`. Update the `run-pre-commit-checks` skill and `docs/git-hooks.md`, and register the binary in the CLI output ADR table. Add usage, relocation notes, and the D10 migration checklist to the crate. |
| T8 | TODO | Prove failures and portability | Command-boundary accepted/rejected fixtures and mutation cases, manual scenarios M1-M7, acceptance review, and independent Task Reviewer report. |

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
- [ ] Spec-only PR #2337 merged into `develop` before implementation
- [ ] Vertical-slice design review recorded after T2
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, pre-commit gate)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
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

## Acceptance Criteria

- [ ] AC1: The command supports explicit file or directory paths, `--staged`, and a documented
      `--all` whole-tree mode; exactly one mode is required.
- [ ] AC2: The command emits no stdout in any mode, including help and usage errors. Stderr carries
      only the D9 record catalog (`diagnostic`, `usage_error`, `runtime_error`, `help`), with
      every listed field always present, nullable fields as `null`, and a deterministic order;
      tests pin each record kind.
- [ ] AC3: Exit codes are `0` for success with or without warnings, `1` for validation errors or
      runtime failure, and `2` for invalid invocation.
- [ ] AC4: Severity follows D8 and D10: `legacy-shape` is an error for draft/open primary specs
      and `issue`/`epic` records that are not strict v1, including primary specs without
      frontmatter; closed-spec incompatibility and `experimental-field` are warnings.
- [ ] AC5: Strict v1 records are checked for status/location, `spec-path`, related-artifact file or
      directory existence, and skill resolution per D7. `--staged` resolves against the index, and
      closed specs get only status and `spec-path` warnings.
- [ ] AC6: Ownership dispatch (D5) and exclusions (D6) are applied in every mode.
- [ ] AC7: Pre-commit invokes the validator as a named, read-only `--staged` step. No CI or other
      integration tier is added.
- [ ] AC8: Accepted/rejected fixtures and mutation cases cover representative field, scalar,
      allowed-value, reference, lifecycle, and path failures within the command boundary.
- [ ] AC9: Manual portability evidence demonstrates focused, staged, whole-tree, and pre-commit use
      without network access, including no-stdout behavior.
- [ ] AC10: On the implementation branch, the seven D11 EPIC records are v1, and `--all` reports no
      errors other than `legacy-shape` for the legacy issue specs and the draft EPIC not yet
      migrated. The error and warning counts are recorded.
- [ ] AC11: The command, its temporary integration point, its relocation path under #2003, and
      the D10 migration checklist are documented, and the CLI output ADR classifies the binary.
- [ ] Focused tests, `linter all`, and the pre-commit gate exit with code `0`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

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
| M1 | Focused validation passes | Run the command on this spec and on its directory. | Exit `0`, empty stdout, no error records on stderr. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Focused validation fails | Copy an open spec to a disposable path, corrupt a field, run the command on it. | Exit `1`, empty stdout, one NDJSON error naming path, category, and field path. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Staged mode uses index content | In a disposable worktree, stage an invalid spec, then fix only the working copy and run `--staged`. | Exit `1` from the staged content, and the working copy is ignored. | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Whole-tree mode | Run `--all` on the implementation branch. | Exit `1` with only `legacy-shape` errors; error and warning counts recorded. | TODO | `manual-verification-evidence.md` section V4 |
| M5 | Pre-commit step | In a disposable worktree, run `./contrib/dev-tools/git/hooks/pre-commit.sh` with a staged invalid v1 spec, then with it fixed. Repeat with a staged edit to a legacy open spec, then with its frontmatter migrated. | The named step fails, then passes, in both cases. | TODO | `manual-verification-evidence.md` section V5 |
| M6 | Invalid invocation and help | Run with no arguments, with `--staged --all`, with a nonexistent path, with `--version`, and with `--help`. | Exit `2`, `2`, `2`, `2`, `0`; stdout empty; exactly one D9 `usage_error` or `help` record each. | TODO | `manual-verification-evidence.md` section V6 |
| M7 | Offline | Repeat M1 and M4 with `cargo run --offline` and no network. | Same outcomes; nothing is downloaded. | TODO | `manual-verification-evidence.md` section V7 |

Record the toolchain for every command result. Disposable Git worktree checkouts live under `.tmp/`
and are removed after use; the real repository index is never used for failing scenarios.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | |
| AC2 | TODO | |
| AC3 | TODO | |
| AC4 | TODO | |
| AC5 | TODO | |
| AC6 | TODO | |
| AC7 | TODO | |
| AC8 | TODO | |
| AC9 | TODO | |
| AC10 | TODO | |
| AC11 | TODO | |

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

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add a progress-log
  entry explaining why no material discovery occurred.
- The independent Task Reviewer records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2264; architecture owner: #2003
- Predecessors: #2265, #2266, #2280
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- Integration precedent: `contrib/dev-tools/checks/clippy-allow-reasons`
