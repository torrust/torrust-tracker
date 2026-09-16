---
doc-type: issue
issue-type: enhancement
status: done
priority: p2
epic: 2003
github-issue: 2219
spec-path: docs/issues/closed/2219-2003-unify-pr-review-processing/ISSUE.md
branch: "2219-2003-unify-pr-review-processing"
related-pr: 2232
last-updated-utc: 2026-09-16 11:09
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
  related-artifacts:
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
    - .github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md
    - docs/pr-reviews/
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - docs/pr-reviews/pr-2174-review.md
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - docs/skills/semantic-skill-link-convention.md
---

<!-- skill-link: create-issue -->

# Issue #2219 - Unify and Make Deterministic the PR Review-Processing Workflow

Parent EPIC: #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make processing pull-request review feedback deterministic and auditable: one unified workflow
and audit record regardless of reviewer identity, a structured finding format reviewers are asked
to follow, and local quality gates that match CI's toolchains so recorded validation evidence
cannot be a false green. New audit records also preserve normalized categories and reviewer classes
so merged review resolutions can be mined for recurring feedback and automation opportunities.
Each new finding also has an immutable repository-controlled reference so ADRs, issue specifications,
and other artifacts can cite it without depending on GitHub identifiers.

## EPIC Alignment

This issue belongs to EPIC #2003 and directly serves its design principle 4 ("Preserve local and
CI parity: checks should have one authoritative implementation that can be invoked consistently
by developers, agents, hooks, and CI"): the stable/nightly rustfmt divergence documented below is
an observed parity failure at the existing pre-commit tier, and the review-workflow findings are
field evidence for the EPIC's guardrail-determinism thesis.

It qualifies for the EPIC's **Approved Early Implementation Candidates** exception, matching its
sibling process subissues (#2155, #2156, #2159, #2160):

- **Low-risk and additive:** the gate change swaps only the toolchain an existing formatting step
  uses (matching the pre-push hook's existing `cargo +nightly fmt --check` precedent); everything
  else is skills, templates, and documentation.
- **Independently verifiable:** each task has its own reproduction or dry-run scenario (M1-M3).
- **No architecture pre-selection:** no shared runner, cache, enforcement platform, or execution
  model is chosen; the current integration points are the pre-commit hook and the `linter`
  rustfmt step, and both remain replaceable by the EPIC's later architecture decision.

This issue is registered in the EPIC's Approved Early Implementation Candidates table with a
"Why It May Proceed" entry restating the above.

## Roles and Ownership

This issue defines exactly one tracked workflow, owned by the **PR author**. It deliberately does
not define a second, reviewer-side tracked workflow.

| Role         | Who                                                                                | Obligation                                                                                                                                                                                                                           | Repository artifact                                          |
| ------------ | ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------ |
| **Author**   | The PR owner                                                                       | Sole owner of the tracked record: the per-PR audit file, fix commits, thread replies and resolutions, and consolidated responses. Processes every review regardless of who wrote it or which format it uses.                            | `docs/pr-reviews/pr-<PR_NUMBER>-…` on the PR branch          |
| **Reviewer** | A maintainer, contributor, Copilot, or a repository review agent (Task/PR Reviewer) | Delivers findings only through GitHub review threads and review bodies, preferably in the advisory finding format. Creates no repository artifact and has no tracking obligation. When repository agents review someone else's PR, they emit findings in the advisory format. | None                                                         |

Rationale:

- Only the author can produce the artifacts: the audit file, fix commits, replies, and resolutions
  all live on the PR branch, which only the author controls. A reviewer-side record would have to
  live elsewhere and would reintroduce the fragmented discovery this issue removes.
- GitHub is already the reviewer's delivery channel. Inline threads and review bodies are durable
  and URL-addressable; they are the reviewer's artifact and the author-side workflow's input.
- The reviewer's contribution still reaches `develop` history: the author's audit records every
  review ID, finding, disposition, fix commit, and reply URL.
- Symmetric responsibility is unenforceable. External contributors and Copilot will not follow a
  reviewer-side tracking process; author ownership is the only model that works for every
  reviewer.

The advisory finding format (T5) is therefore a request that lowers the author's processing cost,
never a precondition: the unified skill must process free-form reviews with the same rigor.

## Background

Processing the review feedback on PR #2174 (four `CHANGES_REQUESTED` rounds from a maintainer
plus six Copilot threads) exposed systematic weaknesses. The full experience is recorded in
`docs/pr-reviews/pr-2174-review.md` and in the consolidated response
<https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593>.

Observed pain points, in decreasing order of cost:

1. **Toolchain divergence produced ~25 false validation records.** The local gate
   (`linter all`, invoked by `pre-commit.sh`) ran **stable** rustfmt, which only *warns* about
   the repository's unstable `imports_granularity`/`group_imports` options, while CI's
   `Run Formatting-Checks` and the reviewer's environment run **nightly** rustfmt, which
   enforces them. Every plan-document row recording `cargo fmt --all -- --check ... passed` was
   a true stable result and a false claim against CI. The reviewer escalated the same blocker
   across four rounds before the root cause was identified. The pre-push hook already uses
   `cargo +nightly fmt --check` — the gap is only in pre-commit/`linter`.
2. **Feedback arrives across three unstructured channels.** Resolvable inline threads,
   non-resolvable multi-finding review bodies, and "standing items" tables inside later review
   bodies that re-reference earlier rounds. Decomposing these into independent findings is
   manual, error-prone, and unverifiable.
3. **Re-raised findings create duplicate threads.** One formatting blocker appeared as three
   inline threads plus four validation-row threads across rounds. Nothing in the current
   workflow requires deduplication before acting, and nothing in the requested review format
   links a re-raise to the original finding.
4. **Two parallel workflows and audit directories own overlapping inputs.**
  `process-copilot-suggestions` stores records in `docs/copilot-pr-reviews/`, while
  `process-pr-review-feedback` uses `docs/pr-review-feedback/`. They differ mainly by thread
  author, yet maintainers comment on the same code Copilot commented on, and the PR #2207 audit
  had to cross-reference findings owned by the other workflow (its F1-F6). Separate top-level
  audit directories fragment discovery and force authors to choose a channel before analyzing
  the review.
5. **Commit SHAs cited from a branch that may be rebased do not survive.** PR #2174's round-5 review
   found that all eight fix-commit SHAs cited in the audit record and in twenty inline replies
   became unreachable after two same-day rebases onto `develop`; the same defect had already
   reached `develop` in PR #2177's feedback record. Audit records and replies must cite fix
   commits by their unique Conventional Commit subject (or by stable reply URL), never by branch
   SHA, until merge pins the history.
6. **Smaller frictions:** severity taxonomy (Blocker/Major/Minor/Nit/Suggestion) lives in
   free-prose bold text and is parsed by hand; superseded/outdated threads have no documented
   disposition; the consolidated-response rule ("one per review") is ambiguous when several
   rounds share the same findings; `gh pr view --head` is unsupported and REST under-reports
   inline comments, so GraphQL is mandatory but only documented in one skill; validation
   evidence rows do not name the toolchain that produced them.

## Scope

### In Scope

- Align the local formatting gate with CI: `linter`'s rustfmt step and/or the pre-commit hook
  must run the **nightly** formatter check (`cargo +nightly fmt --all -- --check`) exactly as CI
  does. If the `linter` binary lives in the external `torrust/torrust-linting` repository, open
  the change there and pin/document the required version here.
- Require validation-evidence rows (issue specs, test-refactor plans) to name the toolchain that
  produced each recorded command result, via the issue/plan templates.
- Merge `process-copilot-suggestions` and `process-pr-review-feedback` into one
  `process-pr-review` skill dispatching on thread author, with:
  - one audit file per PR in the canonical `docs/pr-reviews/` parent directory (single template
    superseding the two current ones);
  - an explicit deduplication step: map re-raised threads to the original finding ID before
    acting;
  - an explicit disposition for superseded/outdated threads (reply "superseded by …", resolve,
    record `NO_ACTION`);
  - a documented consolidated-response rule for multi-round reviews sharing findings (one
    response covering the related rounds is acceptable when each review ID and each finding
    outcome is explicitly listed);
  - a rebase-stable citation rule: audit records and thread replies cite fix commits by their
    unique Conventional Commit subject or by reply URL, never by branch SHA, while the branch can
    still be rebased; after any rebase, verify no recorded SHA remains load-bearing;
  - a claim-verification step before resolving a thread: re-derive the reply's claim against the
    current tree (PR #2174 round 5 found a thread resolved on a rename claim that was false at
    head);
  - GraphQL-first thread fetching documented as mandatory (REST under-reporting noted).
- Publish a requested reviewer finding format (advisory, for human reviewers such as maintainers):
  - one finding per inline thread;
  - a machine-readable first line: `[<Severity>][<FindingId>] <summary>` with severities
    `Blocker|Major|Minor|Nit|Suggestion`;
  - re-raises reference the original `FindingId` instead of restating the finding;
  - review bodies contain only a summary table of finding IDs and round-level verdicts.
  Document it in `docs/contributing` or the review skill and link it from the PR template, making
  clear it is a request that streamlines processing, not a gate on reviewers.
- Migrate the current `docs/copilot-pr-reviews/` and `docs/pr-review-feedback/` records below
  the canonical `docs/pr-reviews/` parent directory. Preserve files as Git renames and repair all
  links, skill metadata, and templates in one dedicated migration increment. The final layout is
  flat: `docs/pr-reviews/pr-<PR_NUMBER>-review.md`; do not retain topic subdirectories.

### Out of Scope

- Retrofitting completed audits (for example PR #2174 or #2207) to the new format.
- Changing CI workflows: CI is already correct; the local gate converges to it.
- Automating reviewer-format enforcement (bots/linting of review comments).
- The `gh pr checks` exit-code ergonomics (external tool behavior).
- Copilot agent/product configuration beyond the skill merge.
- Selecting or prototyping the EPIC #2003 shared automation architecture (runner, cache,
  policy engine, or execution model); this issue's changes stay at existing integration points
  and remain replaceable by the EPIC's design decision.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
  (workflows must be reproducible from Git-tracked documentation).
- ADRs to create: possibly one recording "local quality gates must run the same toolchain as the
  CI check they mirror" if review concludes this is a durable cross-cutting policy rather than a
  one-off fix.

## Design and Ownership Review

Not applicable: this issue changes shell/lint tooling configuration, skills, templates, and
documentation. No child processes, asynchronous I/O, or test fixtures are designed here.

## Implementation Contract

This section resolves the implementation choices material to a repeatable outcome. A task must
update this contract before proceeding when its stated command, file set, or success criterion
becomes impossible; it must not silently substitute a different workflow.

### T1 Formatter-Gate Decision

The installed `linter 0.2.0` executes `cargo fmt --check --quiet`, so it cannot express the CI
command or select a toolchain. T1 therefore adds a named `Checking nightly Rust formatting` step
to `contrib/dev-tools/git/hooks/pre-commit.sh` that runs this exact command:

```sh
cargo +nightly fmt --all -- --check
```

This is the hook's sole CI-parity formatter result. `linter all` remains an independent aggregate
linter invocation until `torrust/torrust-linting` releases a version that can omit or configure
its stable rustfmt component. Its stable rustfmt output must never be recorded or described as
formatter-parity evidence. If such a release becomes available before T1 is implemented, T1 may
instead pin that release and remove the dedicated hook step only when the linter invokes the exact
command above; record the released version and upstream change URL in the progress log.

There is no established hook test harness. T1 proves the new step with the negative M1 detached
worktree reproduction and a passing current-tree hook run. The historical fixture is parent commit
`defe8466aa5b33fed02484fdf97e997546b612e8`, immediately preceding the unique subject
`style(udp-server): fix rustfmt import grouping`. Its affected files are
`packages/udp-server/src/handlers/mod.rs`, `packages/udp-server/src/server/request_buffer.rs`, and
`packages/udp-server/src/statistics/event/handler/error.rs`. From a detached worktree at that
parent, the named nightly step must exit nonzero and identify at least one of those paths.

### Unified Audit Contract

Every new audit is `docs/pr-reviews/pr-<PR_NUMBER>-review.md`, copied from
`docs/templates/PR-REVIEW-TEMPLATE.md`. The PR author exclusively owns this tracked record;
reviewers, including repository review agents, create no repository artifact. One normalized row
represents one independent concern, including a concern stated in a multi-finding review body.

| Field | Required value |
| ----- | -------------- |
| PR number | Target pull request number. |
| Source review ID | GitHub review ID, including review-body-only findings. |
| Source URL | Stable review, thread, or comment URL. |
| Finding ID | Reviewer-provided ID, or an author-assigned `F<ordinal>` in source-review and source-order order. |
| Severity | `Blocker`, `Major`, `Minor`, `Nit`, or `Suggestion`; append `(inferred)` when derived from free prose. |
| Summary | Concise statement of one independent concern. |
| Relationship | `ORIGINAL` or `RE_RAISE_OF:<FindingId>`. |
| Disposition | `FIXED`, `NO_ACTION`, `SUPERSEDED`, or `FOLLOW_UP`; no row is closed with an undocumented value. |
| Current-tree verification | Command or file inspection and its result supporting the reply claim. |
| Resolution reference | Unique Conventional Commit subject and/or durable reply URL; never a branch SHA that can change after a rebase. |
| Reply URL | Thread reply or consolidated response URL. |
| Thread state | `RESOLVED`, `NON_RESOLVABLE`, or `SUPERSEDED`. |

Classify `github-copilot[bot]` and `copilot-pull-request-reviewer` as Copilot, a human GitHub
account as human, and every other bot or unavailable author as `unknown`; classification changes
triage context only, never the contract. Split a review body into a row for each independently
actionable assertion; its summary/verdict text has no row unless it makes an independent request.
A later thread is a duplicate/re-raise when it requests the same current-tree change as an existing
finding. It receives its own source row and `RE_RAISE_OF` relationship before any action.

Before replying or resolving an inline thread, re-derive the stated claim against the current tree.
For an outdated or superseded thread, reply exactly `Superseded by <FindingId>: <reason>.`, record
`SUPERSEDED`/`NO_ACTION` and `SUPERSEDED`, then resolve it. One consolidated PR response may cover
multiple review rounds only when it names every review ID and every finding ID with its disposition
and resolution reference. Final completion fetches threads with the GraphQL-first
`fetch-review-threads` skill and demonstrates no unresolved actionable thread; REST review-comment
responses are supplementary only.

### T9 Review-Finding Reference Decision

Every new normalized finding row has the immutable repository-controlled reference
`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`, with the finding ID lowercased in the reference.
For example, `F1` in PR #2230 is `review-finding:pr-2230-f1`. ADRs, issue specifications, and
other repository artifacts cite the reference in prose or in `semantic-links.related-artifacts`.
The reference resolves to the matching row in `docs/pr-reviews/pr-<PR_NUMBER>-review.md`.

The reference is assigned when the row is created and is never changed, including after a rebase,
thread move, provider migration, or later categorization. GitHub review, thread, comment, and URL
identifiers remain source metadata used to fetch, reply to, and resolve feedback; they are never
the canonical finding reference.

| Option | Advantages | Disadvantages | Decision |
| ------ | ---------- | ------------- | -------- |
| Deterministic `review-finding:pr-<PR_NUMBER>-<FINDING_ID>` | Requires no generator or new global index; is readable and self-locating from the canonical audit filename and row; adds no opaque identifier column; is easy to validate; and remains resolvable from Git-tracked repository history without a GitHub API. | Includes the PR number, which originated on GitHub; depends on the author never reassigning a finding ID; and would need an explicit forge namespace if audits from multiple forges with colliding PR numbers are merged. | Chosen. GitHub-originated numbers are retained as repository archive keys, not live provider identifiers. The workflow already requires source-order IDs and immutable audit history; a future multi-forge migration can add a namespace without rewriting existing references. |
| Opaque ULID `review-finding:RF-<ULID>` | Is independent of provider and PR numbering, collision-resistant across forges, and time-sortable. | Is not self-locating or readable; needs an ID generator and duplicate/format validation; adds a new audit-table column; and can be omitted or malformed when a row is created. | Rejected for now. Its extra machinery solves a multi-forge problem the repository does not yet have. |

This convention is future-only. Historical audit records retain their original fields and are not
backfilled or reinterpreted. If an older finding needs to motivate later work, cite its audit path
and local finding ID until a separately reviewed derived-data process exists.

### Migration and Compatibility Contract

T4 moves every tracked file in `docs/copilot-pr-reviews/` and `docs/pr-review-feedback/` to
`docs/pr-reviews/` with `git mv`: the two `README.md` files, `EXAMPLE-COMPLETED.md`, all
`pr-*-copilot-suggestions.md` files, and all `pr-*-review-feedback.md` files. Rename each
non-duplicate per-PR file to `pr-<PR_NUMBER>-review.md`. Where both source audits exist for one
PR, preserve the maintainer audit as `pr-<PR_NUMBER>-review.md` and preserve the Copilot audit as
`pr-<PR_NUMBER>-copilot-suggestions-legacy.md`; append a migration note to each record rather
than risking loss or reinterpretation of completed findings. The unified filename is mandatory for
new audits only. Remove both old top-level directories after migration. T4 must run a repository-wide search for
`docs/(copilot-pr-reviews|pr-review-feedback)|process-(copilot-suggestions|pr-review-feedback)` and
repair every tracked link, frontmatter reference, and skill-link marker.

T4 proceeds while PRs #2194, #2193, and #2187 remain open. After T4 merges, each author rebases
and moves its unmerged `docs/copilot-pr-reviews/pr-<PR_NUMBER>-copilot-suggestions.md` record to
`docs/pr-reviews/pr-<PR_NUMBER>-review.md`, preserving its source URLs, dispositions, replies, and
thread state. The author uses `process-pr-review` for later or re-raised feedback. Existing resolved
findings remain valid historical evidence and are not reprocessed. Restart review only if the rebase
materially changes reviewed production behavior, not because the audit record moved.

T6 retains the two old skill directories as one-release compatibility redirects. Each stub keeps
valid skill frontmatter, states that it is deprecated, links to `process-pr-review`, and contains no
workflow instructions that conflict with the unified skill. Delete both stubs in the first release
after a repository-wide search finds no `skill-link` or documented invocation of either old name.

### Reviewer-Format Contract

No GitHub-native PR template currently exists. The advisory finding guidance lives at
`docs/templates/REVIEW-FINDINGS.md` and is linked from the unified skill (T5 created it under
`.github/PULL_REQUEST_TEMPLATE/`; PR #2232 review finding `review-finding:pr-2232-f8` relocated it
to the audience-appropriate templates directory). The template's required wording states that the format is advisory and no review is
rejected for omitting it; requires one independent finding per inline thread; requires the first
line `[<Severity>][<FindingId>] <summary>`; permits only `Blocker`, `Major`, `Minor`, `Nit`, and
`Suggestion`; requires re-raises to use the original finding ID; and asks review bodies to contain
only a round verdict/summary instead of duplicated detailed findings.

### Fixed Dry-Run Fixture

T7's required acceptance evidence is a dry run of PR #2174 review `5155990517`:
<https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5155990517>. Fetch the
review body and its two source comments at
<https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969816901> and
<https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969816911>, plus review threads
through `fetch-review-threads`. It must normalize two original inline findings, author-assign
`F1` and `F2` in source order, and record one review-body summary with no independent row. It does
not contain a re-raise; the dry run additionally models a later comment repeating `F1` as
`F3 | RE_RAISE_OF:F1`. M3 processes the following literal advisory comment into the specified row:

```text
[Major][F42] Validation evidence omits the formatter toolchain.
```

Expected normalized values are `Finding ID=F42`, `Severity=Major`, `Relationship=ORIGINAL`, and
`Summary=Validation evidence omits the formatter toolchain.` GitHub data is fetched for the review
and threads; the subsequent re-raise and M3 comment are simulated fixtures, clearly labelled as
such in the evidence.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                            | Notes / Expected Output                                                                                                                                                                                                 |
| --- | ------ | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Align local formatting gate with CI             | Added `Checking nightly Rust formatting` to `contrib/dev-tools/git/hooks/pre-commit.sh`, running `cargo +nightly fmt --all -- --check`. M1 failed at that named step on the pinned fixture; the updated current-tree hook passed. |
| T2  | DONE   | Add toolchain column/note to evidence templates | Updated `docs/templates/ISSUE.md`, `create-issue`, and `write-unit-test` to require toolchain-qualified results. The template gives `cargo +nightly fmt --all -- --check` as a nightly-Rust example. Markdown, link, spell, and whitespace checks passed. |
| T3  | DONE   | Draft unified `process-pr-review` skill         | Added `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md` with GraphQL-first fetching, author classification, normalization, deduplication, current-tree verification, and resolution rules. M2 passed against the pinned PR #2174 data. |
| T4  | DONE   | Create unified audit directory and template     | Created `docs/pr-reviews/` and `docs/templates/PR-REVIEW-TEMPLATE.md`; Git-renamed all records, retaining four duplicate Copilot audits with an explicit `-legacy` suffix and migration notes. Removed both old parents and templates. Markdown, spell, link, and whitespace checks passed. |
| T5  | DONE   | Publish requested reviewer finding format       | Created `.github/PULL_REQUEST_TEMPLATE/review-findings.md` and matched its advisory contract in the unified skill. M3 parsed the pinned literal as `F42`, `Major`, `ORIGINAL`, and the expected summary. Markdown, link, spell, and whitespace checks passed. |
| T6  | DONE   | Deprecate the two old skills                    | Replaced both legacy skill bodies with one-release compatibility redirects to `process-pr-review`. Updated helper skills, the Copilot agent/prompt entry points, and orchestration diagrams to delegate to the unified skill and canonical audit. The affected contract test now prevents parallel procedures/trackers and branch-SHA citation wording. The required search now returns only redirect identities and historical issue descriptions. The focused shell test, ShellCheck, Markdown, link, spell, and whitespace checks passed. |
| T7  | DONE   | Verify and record evidence                      | M1-M3 are complete in `manual-verification-evidence.md`; M2 records fetched versus simulated inputs and the GraphQL final-thread result. AC6 ownership was verified in the unified skill and audit template. The final independent completion review passed; the final documentation commit remains. |
| T8  | DONE   | Add analysis-ready fields to new audits          | Added `Author class` and one primary `Category` to the unified template and workflow. Categories are `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`, `maintainability`, `security`, and `other`. Historical records remain unchanged. The contract test, full lint suite, and independent completion review passed. |
| T9  | DONE   | Make review findings portable concepts          | Added immutable deterministic `review-finding:pr-<PR_NUMBER>-<FINDING_ID>` references for new audit rows, separate from GitHub source metadata, and allowed ADRs and issue specifications to cite them. Historical audits remain unchanged. The contract test, V4 manual evidence, full lint suite, and independent completion review passed. |

## Commit Points

| Task | Coherent change set                                     | Commit policy                                        |
| ---- | ------------------------------------------------------- | ---------------------------------------------------- |
| T1   | Hook-level nightly formatter-parity step                | Commit after M1 fails as specified and the current tree passes. |
| T2   | Toolchain-evidence template guidance                    | Commit after focused documentation validation and required review. |
| T3   | Unified skill                                           | Commit after M2 contract dry-run validation and required review. |
| T4   | Unified audit-directory migration and template          | Commit after link validation and required review.    |
| T5   | Reviewer finding-format documentation                   | Commit after M3 and focused documentation validation. |
| T6   | Old-skill compatibility redirects and link updates      | Commit after the specified search and required review. |
| T7   | Evidence and retrospective                              | Final documentation commit after review.             |
| T8   | Future-only analysis audit schema                       | Commit after focused structural and documentation validation. |
| T9   | Portable review-finding reference convention             | Commit after focused structural and documentation validation. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-unify-pr-review-processing/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2219 created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-16 08:23 UTC - GitHub Copilot - Confirmed GitHub issue #2219 is closed after PR #2232
  merged, then archived this completed folder-style specification in `docs/issues/closed/`.
- 2026-09-14 - GitHub Copilot - Drafted this specification from the pain points recorded while
  processing PR #2174's four maintainer review rounds and six Copilot threads. Evidence:
  `docs/pr-review-feedback/pr-2174-review-feedback.md`.
- 2026-09-15 - User/maintainer - Approved the folder-style draft as an early implementation
  candidate under EPIC #2003.
- 2026-09-15 - GitHub Copilot - Created GitHub issue #2219 and moved the approved specification
  to this canonical open-issue path.
- 2026-09-15 10:15 UTC - GitHub Copilot - Hardened the implementation contract before T1: pinned
  `linter 0.2.0` as unable to select nightly rustfmt, selected the hook-level parity step, fixed
  the audit and migration layouts, and pinned M1-M3 fixtures. Created empty manual-verification
  evidence; no implementation behavior changed.
- 2026-09-15 12:35 UTC - GitHub Copilot - Completed T1. Added the named nightly formatter step
  to the pre-commit hook. In the pinned historical worktree, a test-only `linter` passthrough
  bypassed unrelated pre-existing Lychee failures so the hook reached and failed at that exact
  step; its formatter diagnostics named all three historical import-grouping files. The current
  tree's complete hook passed. Evidence: `manual-verification-evidence.md` section V1.
- 2026-09-15 12:40 UTC - GitHub Copilot - Completed T2. The issue template and its linked
  `create-issue` skill, plus `write-unit-test` test-plan guidance, now require naming a toolchain
  or runtime when it can affect a recorded command result. The issue template includes a concrete
  nightly Rust formatter example. Markdown, link, spell, and whitespace checks passed.
- 2026-09-15 12:45 UTC - GitHub Copilot - Completed T3. Added the unified
  `process-pr-review` skill and dry-ran it against the pinned PR #2174 GraphQL fixture. The two
  fetched source threads normalized to `F1` and `F2`; the deliberately simulated later duplicate
  normalizes to `F3=RE_RAISE_OF:F1`. The final GraphQL helper reported no unresolved threads.
  Evidence: `manual-verification-evidence.md` section V2.
- 2026-09-15 15:15 UTC - GitHub Copilot - Completed T4. Created the unified audit directory and
  template, Git-renamed all legacy audit records, and removed the old audit parents and templates.
  The four PRs with both source audit types retain their maintainer record under the unified name
  and their Copilot record under an explicit `-legacy` suffix, with migration notes preserving
  provenance. Existing active PR audit branches will rebase and migrate their unmerged records
  under the documented transition protocol. Markdown, CSpell, Lychee, and whitespace checks
  passed.
- 2026-09-15 15:30 UTC - GitHub Copilot - Completed T5. Added the advisory GitHub review-finding
  template and identical parsing contract to `process-pr-review`. The literal M3 input normalized
  to `F42`, `Major`, `ORIGINAL`, and the pinned summary without free-prose parsing. Markdown,
  CSpell, Lychee, and whitespace checks passed. Evidence: `manual-verification-evidence.md`
  section V3.
- 2026-09-15 15:50 UTC - GitHub Copilot - Completed T6. Replaced both legacy workflow skills
  with one-release compatibility redirects to `process-pr-review` and synchronized their helper,
  agent, prompt, and orchestration references. The required search returns only the redirect names
  themselves and historical issue descriptions; no live workflow invokes the old skills. The
  repaired `test-agent-review-report-contract.sh` now rejects a parallel Copilot procedure or
  tracker and branch-SHA citation wording, and requires the orchestration route through the unified
  skill and canonical audit. Prose-first test comparison: Arrange provides the four migrated
  skill documents and the Copilot entry-point/orchestration artifacts; Act runs the structural
  shell test; Assert requires every legacy entry point to delegate to the unified workflow, use
  the canonical audit, and avoid legacy audit locations or SHA citation wording. The test's
  focused assertions express that contract directly; its helper functions remain because they
  provide specific file-and-expected-text diagnostics. It and ShellCheck, Markdown, CSpell,
  Lychee, and whitespace checks passed.
- 2026-09-15 16:30 UTC - GitHub Copilot - Remediated the first T7 completion review. Corrected
  the unified skill's malformed `related-artifacts` metadata and V1's stale seven-step claim. The
  structural test now parses the unified skill frontmatter and requires all canonical related
  artifacts. Prose-first comparison: Arrange provides the canonical skill metadata; Act parses
  it with the already-installed PyYAML dependency; Assert requires every expected artifact as a
  distinct list member. This parsing assertion is retained because text matching cannot detect
  syntactically valid but structurally wrong YAML. The focused contract test, ShellCheck, YAML,
  Markdown, CSpell, Lychee, and whitespace checks passed.
- 2026-09-15 16:35 UTC - GitHub Copilot Task Reviewer - Re-reviewed the repaired T7 completion
  state and reported `REVIEW PASSED`: AC1-AC6 and M1-M3 are complete, the unified skill metadata
  parses as five distinct related-artifact entries, V1 accurately records eight current-tree hook
  steps, and the retrospective's metadata-coverage improvement is implemented by the parser-backed
  structural test. The full linter suite and focused contract test passed.
- 2026-09-15 16:55 UTC - GitHub Copilot - Remediated the first T8 schema review. The structural
  test now requires both Copilot account mappings, all author-class values, all nine categories
  including `correctness` in both canonical documents, exactly one primary category, the template's
  explicit every-new-row derived-author-class requirement, and the author-side normalization boundary
  by excluding `Category` from the advisory reviewer template. It also retains the historical-record
  boundary. Prose-first test comparison: Arrange provides the canonical template, unified skill,
  and advisory reviewer template; Act runs the structural shell test; Assert verifies the complete
  future-only analytics contract while retaining historical audit validity and reviewers' free-form
  advisory format. The explicit file-content assertions remain because this cross-document schema
  has no executable implementation boundary. Focused contract, ShellCheck, Markdown, CSpell,
  Lychee, YAML, and whitespace checks passed.
- 2026-09-15 17:15 UTC - GitHub Copilot Task Reviewer - Re-reviewed the completed T8 contract
  and reported `REVIEW PASSED`: the template and workflow require one derived author class and
  exactly one primary category per new audit row; both Copilot account mappings, Human, and Unknown
  are covered; all nine categories are asserted in both canonical documents; the advisory reviewer
  template remains category-free; and historical audits are unchanged and valid. The focused
  contract test, ShellCheck, and full linter suite passed.
- 2026-09-15 17:30 UTC - User/maintainer - Approved T9 as an extension to #2219. Selected the
  deterministic repository reference `review-finding:pr-<PR_NUMBER>-<FINDING_ID>` instead of an
  opaque ULID. The complete trade-off record is in the implementation contract.
- 2026-09-15 17:35 UTC - GitHub Copilot - Implemented T9's deterministic reference convention in
  the semantic-link guidance, audit template, and unified workflow without modifying historical
  audits. V4 manually demonstrated a representative `review-finding:pr-2230-f1` in an audit row,
  prose citation, and `semantic-links.related-artifacts` value. Prose-first test comparison:
  Arrange provides the convention, template, and workflow; Act runs the structural shell test;
  Assert requires the identifier format/example, row column, immutability, and GitHub-provenance
  boundary. These cross-document assertions remain because no executable production boundary
  exists. The retrospective records this reusable convention extension; focused contract,
  ShellCheck, Markdown, CSpell, Lychee, YAML, and whitespace checks passed.
- 2026-09-15 17:45 UTC - GitHub Copilot - Remediated the first T9 completion review. V4 now
  records the exact executed reference-construction command, its zero exit status, and its output
  for the audit row, prose citation, and `semantic-links.related-artifacts` citation. The structural
  test now requires the explicit never-change immutability rule, both permitted citation forms, and
  the future-only historical-audit boundary in addition to format, row field, and GitHub provenance.
  The focused contract test and ShellCheck passed.
- 2026-09-15 17:50 UTC - GitHub Copilot Task Reviewer - Re-reviewed the completed T9 convention
  and reported `REVIEW PASSED`: V4's exact fail-fast command derives and verifies the lowercase
  reference before printing audit-row, prose, and semantic-link citations; the contract test covers
  the identifier, immutability, citation forms, GitHub provenance, and future-only history; the
  deterministic-versus-ULID decision is complete; and no historical audit changed. The focused
  contract test, ShellCheck, and full linter suite passed.

## Acceptance Criteria

- [x] AC1: A rustfmt violation of the repository's unstable import-grouping options fails the
      local pre-commit gate exactly as it fails CI's formatting check, demonstrated on a
      reproduced historical violation.
- [x] AC2: The issue and test-plan templates require naming the toolchain for every recorded
      validation command, and at least one in-repo example follows the requirement.
- [x] AC3: One unified `process-pr-review` skill handles Copilot and human reviewer threads with
      documented deduplication, superseded-thread, and consolidated-response rules; the two old skills
      redirect to it.
- [x] AC4: All PR-review audit records are under the canonical `docs/pr-reviews/` parent
  directory; one unified per-PR audit template exists and the migrated documentation directs
  new PRs to it.
- [x] AC5: The requested reviewer finding format is documented, linked from the unified skill, and
      explicitly advisory.
- [x] AC6: The unified skill and audit template state that the PR author solely owns the tracked
  record and that reviewers (including repository review agents) have no repository-artifact
  obligation.
- [x] AC7: New audits record a normalized author class and exactly one primary category from a
  documented controlled vocabulary, while historical audits remain unchanged and valid.
- [x] AC8: Every new audit row has an immutable deterministic repository-controlled review-finding
  reference that other artifacts can cite, while GitHub identifiers remain source metadata and
  historical audits remain unchanged.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`
- Pre-push checks (nightly formatting parity is the subject under change)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                          | Human-oriented command/steps                                                                                                       | Expected Result                                                            | Status | Evidence                                     |
| --- | --------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | False-green reproduction          | In a detached worktree at `defe8466aa5b33fed02484fdf97e997546b612e8`, run the updated hook with `TORRUST_GIT_HOOKS_LOG_DIR=.tmp`.     | `Checking nightly Rust formatting` exits nonzero and names at least one pinned affected path. | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Unified workflow dry run          | Fetch PR #2174 review `5155990517` and its two pinned inline comments, then normalize them and the simulated `F3` re-raise through the unified skill. | Exactly two fetched original rows and simulated `F3=RE_RAISE_OF:F1`; GraphQL reports the recorded final thread state. | DONE | `manual-verification-evidence.md` section V2 |
| M3  | Reviewer-format round trip        | Normalize the literal `[Major][F42] Validation evidence omits the formatter toolchain.` comment in the Unified Audit Contract. | Produces the pinned `F42`, `Major`, `ORIGINAL`, and summary values without free-prose parsing. | DONE | `manual-verification-evidence.md` section V3 |
| M4  | Portable finding reference        | Construct a representative `review-finding:pr-2230-f1` row and cite it in prose and `semantic-links.related-artifacts`. | The reference is deterministic, provider-independent, and usable in both citation forms. | DONE | `manual-verification-evidence.md` section V4 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `manual-verification-evidence.md` section V1; hook JSON result with `failed_step=Checking nightly Rust formatting` |
| AC2   | DONE                   | `docs/templates/ISSUE.md` nightly Rust example; `linter markdown`, `linter cspell`, `linter lychee`, and `git diff --check` |
| AC3   | DONE                   | Unified skill; two one-release redirect stubs; required T6 search; strengthened `test-agent-review-report-contract.sh`; ShellCheck; Markdown, CSpell, Lychee, and `git diff --check` |
| AC4   | DONE                   | `docs/pr-reviews/`; `docs/templates/PR-REVIEW-TEMPLATE.md`; Markdown, CSpell, Lychee, and `git diff --check` |
| AC5   | DONE                   | `docs/templates/REVIEW-FINDINGS.md` (relocated per `review-finding:pr-2232-f8`); `manual-verification-evidence.md` section V3 |
| AC6   | DONE                   | `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md` ownership section; `docs/templates/PR-REVIEW-TEMPLATE.md` ownership section |
| AC7   | DONE                   | `docs/templates/PR-REVIEW-TEMPLATE.md`; `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`; `test-agent-review-report-contract.sh`; ShellCheck; `linter all`; independent completion review |
| AC8   | DONE                   | `manual-verification-evidence.md` section V4; semantic-link convention; audit template; unified workflow; `test-agent-review-report-contract.sh`; ShellCheck; `linter all`; independent completion review |

## Risks and Trade-offs

- **External dependency:** the rustfmt step may live in `torrust/torrust-linting`; the fix then
  needs a coordinated release. Mitigation: the pre-commit hook can add its own
  `cargo +nightly fmt --all -- --check` step immediately, independent of the linter release.
- **Nightly availability:** requiring nightly rustfmt locally adds a toolchain prerequisite for
  contributors. Mitigation: the pre-push hook already requires nightly, so the prerequisite is
  not new; document `rustup toolchain install nightly` in the setup skill.
- **Reviewer adoption:** the finding format is advisory; reviewers may ignore it. Mitigation:
  the unified skill must keep working with free-form reviews — the format reduces cost, it is
  not an input requirement.
- **Audit migration regression:** existing audits, skills, and links reference the two old
  top-level directories. Mitigation: preserve files as Git renames, perform a repository-wide
  link/metadata audit, and retain redirect notes only where a durable external reference requires
  one.
- **Process churn:** merging workflows mid-flight on open PRs could confuse active audits.
  Mitigation: only PRs whose audit file does not yet exist adopt the unified flow.

## Implementation Completion Review

- Retrospective: `implementation-retrospective.md` records the material implementation discoveries
  and reusable improvements.
- Final acceptance review: `REVIEW PASSED` after remediation of the unified-skill metadata and V1
  hook-step evidence.

## References

- Motivating PR: <https://github.com/torrust/torrust-tracker/pull/2174>
- Full pain-point audit: `docs/pr-reviews/pr-2174-review.md`
- Consolidated review response: <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593>
- Prior mixed-workflow audit: `docs/pr-reviews/pr-2207-review.md`
- Current skill: `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`
- Current template: `docs/templates/PR-REVIEW-TEMPLATE.md`
- Pre-push nightly parity precedent: `contrib/dev-tools/git/hooks/pre-push.sh`
- External linter repository: <https://github.com/torrust/torrust-linting>
