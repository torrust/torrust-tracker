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
related-pr: null
last-updated-utc: "2026-09-24 16:01"
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
    - write-unit-test
    - run-pre-commit-checks
  related-artifacts:
    - issue #2264
    - issue #2266
    - issue #2280
    - issue #2003
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
- Fixing genuine errors the first `--all` run reports in current draft/open v1 specs, in separate
  commits.

### Out of Scope

- The final #2003 automation architecture, binary or package selection, orchestration, shared
  runners, caches, or policy composition.
- JSON Schema generation, its tracked artifact, regeneration, and drift verification (#2280).
- CI, pre-push, nightly, release, or agent-policy integration.
- New frontmatter fields, profiles, or semantic-link forms. Extending strict profiles to ADRs,
  skills, agents, and evidence records is EPIC #2264 row 3.
- Existence or skill-resolution checks for legacy, unknown, or externally governed documents.
- Migrating or rewriting legacy documents to remove warnings. Historical records are never rewritten
  solely to silence advisory diagnostics.
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
  precedent: call `try_parse` and never let clap write to stdout. Every clap output becomes one
  NDJSON record on stderr. Help and version exit `0`; usage errors exit `2`.
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
  own the fixtures.
- **D7 - Repository-aware checks.** Apply these only to strict v1 issue and EPIC records:
  - `status` matches the spec's location: `draft` in `docs/issues/drafts/`, neither `draft` nor
    `done` in `docs/issues/open/`, `done` in `docs/issues/closed/`;
  - `spec-path` equals the file's repository-relative path;
  - each repository-path entry in `related-artifacts` exists (`issue #` and `review-finding:`
    entries are syntax-only);
  - each `skill-links` name resolves to a tracked `.github/skills/**/<name>/SKILL.md`.

  `--staged` resolves existence against the index; the other modes resolve against the working
  tree. Legacy, unknown, and external documents receive no existence checks.
- **D8 - Severity and exit status.**
  - Any present frontmatter: an unclosed delimiter, malformed YAML, a non-mapping root, or an
    invalid universal envelope is an `error` everywhere.
  - Strict v1 records under `docs/issues/drafts/` or `docs/issues/open/`: profile, scalar,
    allowed-value, reference-syntax, and D7 failures are `error`s.
  - Strict v1 records under `docs/issues/closed/`: the same failures are `warning`s (advisory).
  - Strict v1 records elsewhere: structural failures are `error`s; the location check in D7 does
    not apply.
  - Warning kinds from the contract: `legacy-shape` for a draft/open document whose `doc-type` is
    `issue` or `epic` but lacks `schema-version: 1`; advisory closed-spec incompatibility; and
    `experimental-field` for each `x-` field in a strict profile.
  - Exit `0` when there are no errors, even if warnings were emitted; `1` for any validation error
    or runtime failure (git, I/O); `2` for invalid invocation.
- **D9 - Diagnostic record.** Add `severity` and an optional `field_path` to the library
  `Diagnostic`. The binary renders each diagnostic as one NDJSON line on stderr with `kind`,
  `path`, `severity`, kebab-case `category`, optional `field_path`, and `message`. Stdout stays
  empty in every mode, including help, usage errors, and runtime failures.
- Related ADRs: [`docs/adrs/20260519000000_define_global_cli_output_contract.md`](../../../adrs/20260519000000_define_global_cli_output_contract.md).
  Register `frontmatter-validator` as `no-stdout-result` in that ADR's binary classification table.
- ADRs to create: none expected. The temporary placement is already approved early work under
  #2003. Create an ADR only if implementation needs a durable, repository-wide choice beyond D1-D9.

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
`show :<path>`, `cat-file -e`) and waits for each one to finish. It needs no network,
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
| T1 | TODO | Extend the diagnostic vocabulary | Add `Severity` and optional `field_path` to `Diagnostic`, stable kebab-case category names, and the new command/repository categories. Existing library tests keep their outcomes; field paths are populated where the library already knows the field. |
| T2 | TODO | Add the command and explicit-path mode | New `frontmatter-validator` binary with `clap` (D3), explicit file paths, NDJSON rendering (D9), and exit codes. Stop for the vertical-slice checkpoint. |
| T3 | TODO | Add discovery, `--staged`, and `--all` | Directory expansion, tracked-file discovery, index-content sourcing, ownership dispatch (D5), and exclusions (D6). |
| T4 | TODO | Apply location-dependent severity and warnings | D8 severity policy, `legacy-shape`, closed-spec advisory, and `experimental-field` warnings. |
| T5 | TODO | Add repository-aware checks | D7 status/location, `spec-path`, artifact existence, and skill resolution through index and working-tree resolvers. |
| T6 | TODO | Run a whole-tree baseline and triage findings | Run `--all` on `develop`. Fix genuine draft/open v1 errors in separate `docs(issues)` commits. Record the warning counts, and do not rewrite legacy records. |
| T7 | TODO | Integrate with pre-commit and document | Add the named `--staged` step to `pre-commit.sh`. Update the `run-pre-commit-checks` skill and `docs/git-hooks.md`, register the binary in the CLI output ADR table, and add usage and relocation notes to the crate. |
| T8 | TODO | Prove failures and portability | Command-boundary accepted/rejected fixtures and mutation cases, manual scenarios M1-M7, acceptance review, and independent Task Reviewer report. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Diagnostic severity, field path, and category vocabulary | Commit after focused library tests and prose-first test-design review. |
| T2 | Binary, argument parsing, explicit-path mode, NDJSON rendering | Commit after focused command tests and the vertical-slice review. |
| T3 | Discovery, `--staged`, `--all`, ownership dispatch, exclusions | Commit after TempDir git-repository tests. |
| T4 | Severity policy and warning kinds | Commit after focused accepted/rejected cases per location. |
| T5 | Repository-aware checks and resolvers | Commit after index and working-tree resolver tests. |
| T6 | Each genuine current-tree spec fix | One `docs(issues)` commit per coherent fix; no commit if the baseline is clean. |
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
- [ ] Specification reviewed and approved by user/maintainer
- [ ] Spec-only PR merged into `develop` before implementation
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

## Acceptance Criteria

- [ ] AC1: The command supports explicit file or directory paths, `--staged`, and a documented
      `--all` whole-tree mode; exactly one mode is required.
- [ ] AC2: The command emits no stdout in any mode, including help and usage errors. Each
      diagnostic is one stable NDJSON record on stderr with `kind`, source `path`, `severity`,
      `category`, `field_path` when applicable, and an actionable `message`.
- [ ] AC3: Exit codes are `0` for success with or without warnings, `1` for validation errors or
      runtime failure, and `2` for invalid invocation.
- [ ] AC4: Severity follows D8, including the `legacy-shape`, closed-spec advisory, and
      `experimental-field` warnings.
- [ ] AC5: Strict v1 records are checked for status/location, `spec-path`, related-artifact path
      existence, and skill resolution per D7. `--staged` resolves against the index.
- [ ] AC6: Ownership dispatch (D5) and exclusions (D6) are applied in every mode.
- [ ] AC7: Pre-commit invokes the validator as a named, read-only `--staged` step. No CI or other
      integration tier is added.
- [ ] AC8: Accepted/rejected fixtures and mutation cases cover representative field, scalar,
      allowed-value, reference, lifecycle, and path failures within the command boundary.
- [ ] AC9: Manual portability evidence demonstrates focused, staged, whole-tree, and pre-commit use
      without network access, including no-stdout behavior.
- [ ] AC10: `--all` exits `0` on the implementation branch, with its warning counts recorded.
- [ ] AC11: The command, its temporary integration point, and its relocation path under #2003 are
      documented, and the CLI output ADR classifies the binary.
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
| M1 | Focused validation passes | Run the command on this spec and on an open-spec directory. | Exit `0`, empty stdout, only expected warnings on stderr. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Focused validation fails | Copy an open spec to a disposable path, corrupt a field, run the command on it. | Exit `1`, empty stdout, one NDJSON error naming path, category, and field path. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Staged mode uses index content | In a disposable worktree, stage an invalid spec, then fix only the working copy and run `--staged`. | Exit `1` from the staged content, and the working copy is ignored. | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Whole-tree mode | Run `--all` on the implementation branch. | Exit `0`; warning counts recorded. | TODO | `manual-verification-evidence.md` section V4 |
| M5 | Pre-commit step | Run `./contrib/dev-tools/git/hooks/pre-commit.sh` with a staged invalid spec in a disposable worktree, then with it fixed. | The named step fails, then passes. | TODO | `manual-verification-evidence.md` section V5 |
| M6 | Invalid invocation | Run with no arguments, with `--staged --all`, with a nonexistent path, and with `--help`. | Exit `2`, `2`, `2`, `0`; stdout empty; one NDJSON record each. | TODO | `manual-verification-evidence.md` section V6 |
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
- **Warning noise.** Hundreds of legacy specs may emit `legacy-shape` warnings under `--all`.
  Mitigation: warnings never fail the command, and `--staged` only reports on files being changed.
  If the noise proves harmful, revisit it in the EPIC rather than rewriting historical records.
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
