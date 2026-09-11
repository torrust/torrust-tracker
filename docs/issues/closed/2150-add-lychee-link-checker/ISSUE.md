---
doc-type: issue
issue-type: task
status: done
priority: p3
epic: null
github-issue: 2150
spec-path: docs/issues/closed/2150-add-lychee-link-checker/ISSUE.md
branch: "2150-add-lychee-link-checker"
related-pr: "https://github.com/torrust/torrust-tracker/pull/2154"
last-updated-utc: 2026-09-11
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/index.md
    - docs/issues/closed/2150-add-lychee-link-checker/link-check-report.md
---

<!-- skill-link: create-issue -->

# Issue #2150 - Add lychee local Markdown link checker

## Goal

Run [lychee](https://lychee.cli.rs/) locally against this repository's Markdown, fix every genuine
broken local link it reports, add a checked-in `lychee.toml` config that enables local anchor
checking and documents intentional exclusions, and open a follow-up GitHub issue in
[`torrust/torrust-linting`](https://github.com/torrust/torrust-linting) requesting that lychee be
added as a linter in the shared `linter` tool so link checking can later be enforced automatically
(in pre-commit and normal CI).

## Background

`torrust-linting` (the shared `linter` binary used by this repository, see
[`docs/index.md`](../../../index.md) and `AGENTS.md`) currently runs markdownlint, yamllint,
taplo, cspell, clippy, rustfmt, and shellcheck, but has no link checker. Markdown docs, ADRs, and
issue specs in this repository accumulate cross-references, and nothing currently detects when
local links break after a file move or heading rename.

lychee is a fast, async link checker written in Rust with first-class support for Markdown, HTML,
and plain text, and it is the tool the maintainer wants to standardize on. It is installed via
`cargo install lychee` (the drafting environment has `lychee 0.24.2`), but the repository has no
lychee configuration, no baseline report of existing broken links, and no CI/pre-commit
enforcement.

A quick exploratory offline run during drafting (`lychee --offline` over `docs/`, `README.md`,
`**/AGENTS.md`, `packages/*/README.md`, `.github/**/*.md`) reported roughly 170 broken
**local file** links, concentrated in `docs/issues/closed/` (historical specs whose relative
links broke when they were moved) but also present in ADRs, package READMEs, `.github/skills/`,
and `tests/AGENTS.md`. This confirms the problem is real and gives a rough size for the triage
work.

lychee distinguishes two modes that have very different cost and reliability profiles:

- **Offline** (`offline = true`): resolves local file links without making network requests.
  Combined with `include_fragments = "full"`, it also validates local Markdown heading anchors
  such as `docs/testing.md#verification-plan`. This is deterministic, sub-second, and suitable
  for pre-commit and normal CI.
- **Online** (the default): additionally requests external URLs. It is slow and subject to
  rate limits and transient network failures, so it must not block pre-push or ordinary CI.
  This issue does not run or configure online checking. A follow-up tracker issue will define
  an advisory weekly scheduled workflow, which records a real failure but is not a required PR
  status check.

Enforcement (wiring lychee into `linter all`, pre-commit, and CI) depends on `torrust-linting`
adding lychee as a linter first, matching the pattern used for the other tools in that project
(see its `src/linters/` module and the `README.md` table of expected root-level config files:
`.markdownlint.json`, `.yamllint-ci.yml`, `.taplo.toml`, `cspell.json`). That upstream change is
out of scope for this issue and is tracked instead by opening a request issue in
`torrust/torrust-linting`.

This issue therefore covers two phases:

1. Verify lychee locally, run its deterministic offline local-link and local-fragment check
   against the repository, classify every finding in an issue-local report, fix genuine broken
   links, and add a `lychee.toml` config documenting any intentional exclusions.
2. Open a GitHub issue in `torrust/torrust-linting` requesting lychee be added as a linter there,
   referencing this issue and the offline `lychee.toml` convention produced in phase 1, then open
   a tracker follow-up issue to wire the offline check into normal enforcement and add the
   advisory weekly external-link workflow.

## Scope

### In Scope

- Document the lychee version and install command used for this work in the issue-local report.
- Add a `lychee.toml` configuration file at the repository root (lychee's default config name),
  following the existing root-config convention used by other linters (`.markdownlint.json`,
  `.yamllint-ci.yml`, `.taplo.toml`, `cspell.json`). Every `exclude`/`exclude_path` entry must
  carry an inline comment with its rationale. Use `lychee.toml` as the single exclusion
  mechanism; do not add a `.lycheeignore` file.
- Enable and document `include_fragments = "full"` in `lychee.toml`. This checks the target of
  local fragment links such as `[Verification](docs/testing.md#verification-plan)`, ensuring that
  a linked Markdown heading exists as well as its file.
- Define the checked Markdown set as: `README.md`, `SECURITY.md`, `docs/**/*.md`, `**/AGENTS.md`,
  `packages/*/README.md`, `console/**/*.md`, `contrib/**/*.md`, `share/**/*.md`, and
  `.github/**/*.md` (skills and agent definitions are authoritative workflow docs). Record the
  exact command in the report so the run is reproducible.
- Run the offline local-link and local-fragment check and record it in the issue-local report.
- Classify every reported link as one of: genuinely broken (fix), false positive (add a
  documented exclusion), or historical/out of scope (document why).
- Fix all genuinely broken links found in the checked Markdown set.
- Keep an issue-local, human-readable report at `link-check-report.md` (in this spec's folder)
  summarizing the offline check, classification table, and fixes applied. Do not commit raw
  lychee output.
- Re-run the offline check after fixes to confirm zero unresolved errors (excluding documented
  exclusions).
- Open a GitHub issue in `torrust/torrust-linting` requesting lychee support, including: the
  proposed CLI subcommand (`linter lychee`, or `linter links`), the offline
  `include_fragments = "full"` `lychee.toml` convention this issue establishes, and a link back
  to this issue for context.
- After opening the `torrust-linting` issue, open a follow-up issue in this repository to:
  wire offline local file/fragment checking into pre-commit and normal CI once upstream support
  is available; and add a weekly, manually re-runnable online external-link workflow. The
  workflow must be advisory, not a required PR status check: let lychee fail normally so real
  broken-link findings are visible, but it must not block a merge. The follow-up issue must
  define `GITHUB_TOKEN` handling, bounded timeout/retries/concurrency, report-artifact retention,
  and a triage process (re-run once; fix persistently broken links or add narrow documented
  exclusions).

### Out of Scope

- Adding lychee to `torrust-linting` itself (tracked by the follow-up issue opened in that repo,
  not implemented here).
- Wiring lychee into this repository's `pre-commit.sh` or CI workflows; it is planned in the
  follow-up issue opened by T9 after `torrust-linting` provides the requested linter integration.
- Checking external URLs in pre-push or normal CI. Those checks are non-deterministic because
  third-party services and networks are outside repository control.
- Checking links inside Rust source doc comments (`///`, `//!`) or other non-Markdown files.
  Intra-doc links are already covered by rustdoc's `broken_intra_doc_links` lint; external URLs in
  doc comments can be revisited once the Markdown baseline is clean.
- Auditing/fixing links in `docs/issues/closed/` historical specs. They are excluded via
  `exclude_path` in `lychee.toml`; their relative links broke when the specs were moved and
  fixing them adds no value for actively maintained docs.

## Architectural Decisions

- Related ADRs: `None`
- ADRs to create: `None known`

This is a routine tooling-adoption task consistent with the existing multi-linter setup
(`torrust-linting`); it does not introduce a new architectural pattern.

## Design and Ownership Review

Not applicable — no child processes, asynchronous I/O, network readiness, resource cleanup, or
reusable test fixtures are introduced beyond invoking an existing CLI tool.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                     | Notes / Expected Output                                                                                                                                              |
| --- | ------ | ------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Record lychee version and install command                                | Recorded in `link-check-report.md`                                                                                                                                   |
| T2  | DONE   | Draft initial `lychee.toml`                                              | Enables offline mode and full fragment checks; excludes only `docs/issues/closed/`                                                                                   |
| T3  | DONE   | Offline baseline pass                                                    | 123 errors across 37 sources; command, unmatched `share/**/*.md` warning, summary, and source-level classifications recorded in `link-check-report.md`               |
| T4  | DONE   | Classify every finding                                                   | All findings classified and summarized in `link-check-report.md`                                                                                                     |
| T5  | DONE   | Fix genuinely broken links                                               | Repaired paths and fragments in maintained Markdown; no broad exclusions added                                                                                       |
| T6  | DONE   | Finalize `lychee.toml` exclusions                                        | All configuration entries include rationale comments                                                                                                                 |
| T7  | DONE   | Re-run offline check to confirm a clean result                           | 0 errors; result recorded in `link-check-report.md`                                                                                                                  |
| T8  | DONE   | Open GitHub issue in `torrust/torrust-linting` requesting lychee support | Created [torrust/torrust-linting#3](https://github.com/torrust/torrust-linting/issues/3)                                                                             |
| T9  | DONE   | Open tracker follow-up issue for lychee enforcement and weekly checks    | Created [#2162](https://github.com/torrust/torrust-tracker/issues/2162), blocked on [torrust/torrust-linting#3](https://github.com/torrust/torrust-linting/issues/3) |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/add-lychee-link-checker/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: no separate retrospective needed; see progress log
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-11 UTC - Repository maintenance - Archived this closed issue specification in `docs/issues/closed/2150-add-lychee-link-checker/`.
  Append one line per meaningful update.

- 2026-09-07 08:54 UTC - Copilot - Drafted initial issue specification for local lychee adoption plus torrust-linting follow-up issue - `docs/issues/drafts/add-lychee-link-checker/ISSUE.md`
- 2026-09-07 09:33 UTC - Copilot - Review pass: corrected default config filename (`lychee.toml`), aligned checked-file set with the M1 command, named the report file, and recorded an exploratory `lychee --offline` run (~170 local-link errors)
- 2026-09-07 09:43 UTC - Copilot - Revised scope: lychee will enforce only offline local Markdown file and fragment links; external URL checks are excluded from pre-push and normal CI because they are non-deterministic
- 2026-09-07 10:03 UTC - Copilot - Added a final follow-up task: open a tracker issue after the torrust-linting request to enable deterministic offline enforcement and an advisory weekly external-link workflow
- 2026-09-07 10:07 UTC - Copilot - User approved the specification; created GitHub issue #2150 - https://github.com/torrust/torrust-tracker/issues/2150
- 2026-09-07 10:33 UTC - Copilot - Created the spec-only branch `2150-add-lychee-link-checker-spec` from `torrust/develop`; spec-only PR pending
- 2026-09-07 11:39 UTC - Copilot - Spec-only PR #2154 merged into `develop`; created implementation branch `2150-add-lychee-link-checker` from merge commit `0e7fa6d2`
- 2026-09-07 12:00 UTC - Copilot - Created [torrust/torrust-linting#3](https://github.com/torrust/torrust-linting/issues/3) for lychee local Markdown link checker integration; T9 is blocked pending its own approved tracker issue specification
- 2026-09-07 14:47 UTC - Copilot - User approved the T9 specification; created [#2162](https://github.com/torrust/torrust-tracker/issues/2162) for offline enforcement and advisory scheduled external link checks
- 2026-09-07 14:59 UTC - Copilot - Validated documentation with markdownlint and cspell; an initial final lychee run omitted the required unmatched `share/**/*.md` input and is superseded by the corrected result below
- 2026-09-07 15:00 UTC - Copilot - Completed the issue implementation; no separate retrospective is needed because all findings were routine stale-link/path corrections captured in `link-check-report.md`
- 2026-09-07 15:00 UTC - Copilot - Corrected Task Reviewer findings: restored the required `share/**/*.md` input and its non-fatal warning, made all baseline errors source-auditable, and corrected the DEC-08 decision anchor; acceptance-criteria review remains pending independent verification.
- 2026-09-07 15:03 UTC - Task Reviewer - Independent completion review passed: all acceptance criteria verified, exact lychee run returned 0 errors, and the repository pre-commit quality gate passed
- 2026-09-07 15:04 UTC - Committer - Verified the issue specification progress is current before commit; skill link repairs will be committed separately from the lychee baseline

## Acceptance Criteria

- [x] AC1: `link-check-report.md` records the lychee version, install command, and the exact
      reproducible offline local-link/local-fragment command.
- [x] AC2: A `lychee.toml` config file exists at the repository root; every exclusion has an
      inline rationale comment and it enables `offline = true` and `include_fragments = "full"`.
- [x] AC3: Every finding from the offline baseline pass is classified (fix / exclude / historical) in
      `link-check-report.md`, with rationale for each non-fix row.
- [x] AC4: All genuinely broken links identified are fixed in the affected Markdown files.
- [x] AC5: Re-running the offline check after fixes reports zero errors (excluding documented
      exclusions), with the summary line recorded in the report.
- [x] AC6: A GitHub issue is opened in `torrust/torrust-linting` requesting lychee support,
      referencing this issue and the `lychee.toml` convention; the URL is recorded in `References`.
- [x] AC7: A tracker follow-up issue is opened after AC6. It describes offline pre-commit/normal
      CI enforcement and a weekly, advisory, manually re-runnable online external-link workflow;
      its URL is recorded in `References`.
- [x] `linter all` exits with code `0`
- [x] Relevant tests are not applicable: this issue changes documentation and lychee configuration only.
- [x] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated for the new lychee configuration and repaired links.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Offline lychee re-run (M2) showing zero errors

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

The checked Markdown set (`INPUTS`) is: `README.md SECURITY.md 'docs/**/*.md' '**/AGENTS.md' 'packages/*/README.md' 'console/**/*.md' 'contrib/**/*.md' 'share/**/*.md' '.github/**/*.md'`. `share/**/*.md` currently matches no tracked Markdown files, so lychee emits a non-fatal unmatched-input warning. `lychee.toml` supplies `offline = true` and `include_fragments = "full"`.

| ID  | Scenario                               | Command/Steps                                                    | Expected Result                                            | Status | Evidence                                               |
| --- | -------------------------------------- | ---------------------------------------------------------------- | ---------------------------------------------------------- | ------ | ------------------------------------------------------ |
| M1  | Offline baseline pass                  | `lychee --no-progress INPUTS`                                    | Command completes; findings captured for triage            | DONE   | `link-check-report.md`                                 |
| M2  | Post-fix offline pass                  | Same as M1 after fixes and `lychee.toml` finalization            | `0 Errors` in summary line                                 | DONE   | `link-check-report.md`                                 |
| M3  | `torrust-linting` request issue opened | `gh issue create --repo torrust/torrust-linting --body-file ...` | Issue created; URL recorded and cross-linked to this issue | DONE   | https://github.com/torrust/torrust-linting/issues/3    |
| M4  | Tracker follow-up issue opened         | `gh issue create --repo torrust/torrust-tracker --body-file ...` | Issue created after M3; URL recorded and cross-linked      | DONE   | https://github.com/torrust/torrust-tracker/issues/2162 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                               |
| ----- | ---------------------- | ------------------------------------------------------ |
| AC1   | DONE                   | `link-check-report.md`                                 |
| AC2   | DONE                   | `lychee.toml`                                          |
| AC3   | DONE                   | `link-check-report.md`                                 |
| AC4   | DONE                   | Working-tree diff; implementation PR pending           |
| AC5   | DONE                   | `link-check-report.md` (M2)                            |
| AC6   | DONE                   | https://github.com/torrust/torrust-linting/issues/3    |
| AC7   | DONE                   | https://github.com/torrust/torrust-tracker/issues/2162 |

## Risks and Trade-offs

- **Triage volume**: the exploratory offline run reported ~170 local-link errors before any
  exclusions. Mitigation: exclude `docs/issues/closed/` first (the bulk of the noise), then
  triage the remainder; most local-link fixes are mechanical path corrections.
- **Fragment checking may add work**: `include_fragments = "full"` will flag anchors that
  markdownlint does not validate. Mitigation: it is intentional: heading-anchor correctness is
  part of the goal, and broken anchors should be fixed or explicitly excluded with rationale.
- **Scope creep into non-Markdown files**: including Rust doc comments or generated files could
  significantly expand the surface area and false-positive rate. Mitigation: explicitly out of
  scope; revisit once the Markdown baseline is clean.
- **Enforcement depends on an external project**: pre-commit/CI enforcement cannot land until
  `torrust-linting` adds lychee support. Mitigation: T8 explicitly requests the upstream work
  and T9 captures tracker-side follow-up; `lychee.toml` is usable immediately by anyone running
  lychee manually, independent of the upstream timeline.
- **Scheduled external checks can fail spuriously**: third-party availability, rate limits, and
  network faults are outside repository control. Mitigation: T9 makes the workflow advisory,
  lets true failures remain visible, and requires one manual re-run before triage.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not needed` — the independent completion review found no material design
  change or reusable discovery beyond the routine path/anchor corrections recorded above.
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.

## References

- Related issues:
  - `torrust/torrust-linting` request issue: [#3](https://github.com/torrust/torrust-linting/issues/3) (T8)
  - `torrust/torrust-tracker` enforcement and scheduled-check follow-up: [#2162](https://github.com/torrust/torrust-tracker/issues/2162) (T9)
- Related PRs: #2154 (spec-only)
- Related ADRs: `None`
- lychee: <https://lychee.cli.rs/> — configuration reference:
  <https://lychee.cli.rs/guides/config/>
- `torrust-linting`: <https://github.com/torrust/torrust-linting>
