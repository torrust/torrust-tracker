---
doc-type: issue
issue-type: enhancement
status: open
priority: p2
epic: 2003
github-issue: 2219
spec-path: docs/issues/open/2219-2003-unify-pr-review-processing/ISSUE.md
branch: "2219-2003-unify-pr-review-processing"
related-pr: null
last-updated-utc: 2026-09-15T12:45:00Z
semantic-links:
  skill-links:
    - create-issue
    - process-copilot-suggestions
    - process-pr-review-feedback
  related-artifacts:
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md
    - .github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md
    - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
    - .github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md
    - docs/pr-reviews/
    - docs/pr-review-feedback/
    - docs/copilot-pr-reviews/
    - docs/templates/PR-REVIEW-FEEDBACK-TEMPLATE.md
    - docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md
    - docs/pr-review-feedback/pr-2174-review-feedback.md
    - contrib/dev-tools/git/hooks/pre-commit.sh
---

<!-- skill-link: create-issue -->

# Issue #2219 - Unify and Make Deterministic the PR Review-Processing Workflow

Parent EPIC: #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make processing pull-request review feedback deterministic and auditable: one unified workflow
and audit record regardless of reviewer identity, a structured finding format reviewers are asked
to follow, and local quality gates that match CI's toolchains so recorded validation evidence
cannot be a false green.

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
`docs/pr-review-feedback/pr-2174-review-feedback.md` and in the consolidated response
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

### Migration and Compatibility Contract

T4 moves every tracked file in `docs/copilot-pr-reviews/` and `docs/pr-review-feedback/` to
`docs/pr-reviews/` with `git mv`: the two `README.md` files, `EXAMPLE-COMPLETED.md`, all
`pr-*-copilot-suggestions.md` files, and all `pr-*-review-feedback.md` files. Rename each per-PR
file to `pr-<PR_NUMBER>-review.md`; where both sources exist for one PR, merge their histories and
content into that one audit as a documented migration exception. Remove both old top-level
directories after migration. T4 must run a repository-wide search for
`docs/(copilot-pr-reviews|pr-review-feedback)|process-(copilot-suggestions|pr-review-feedback)` and
repair every tracked link, frontmatter reference, and skill-link marker.

T6 retains the two old skill directories as one-release compatibility redirects. Each stub keeps
valid skill frontmatter, states that it is deprecated, links to `process-pr-review`, and contains no
workflow instructions that conflict with the unified skill. Delete both stubs in the first release
after a repository-wide search finds no `skill-link` or documented invocation of either old name.

### Reviewer-Format Contract

No GitHub-native PR template currently exists. T5 creates
`.github/PULL_REQUEST_TEMPLATE/review-findings.md` and links the advisory finding guidance in the
unified skill. The template's required wording states that the format is advisory and no review is
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
| T4  | TODO   | Create unified audit directory and template     | Create `docs/pr-reviews/` and `docs/templates/PR-REVIEW-TEMPLATE.md`, then perform the exact migration in the Migration and Compatibility Contract. Validate with the specified repository-wide search, Markdown/link/spell checks, and `git diff --check`. |
| T5  | TODO   | Publish requested reviewer finding format       | Create `.github/PULL_REQUEST_TEMPLATE/review-findings.md` and add the identical advisory guidance to the unified skill. Validate M3's literal comment and Markdown/link/spell checks. |
| T6  | TODO   | Deprecate the two old skills                    | Replace only the bodies of `process-copilot-suggestions` and `process-pr-review-feedback` with the compatibility redirects in the contract. Validate every result from the specified repository-wide search and skill-link synchronization. |
| T7  | TODO   | Verify and record evidence                      | Complete M1-M3 in `manual-verification-evidence.md`; M2's PR #2174 dry run is mandatory and a next-real-PR run is optional. Record fetched versus simulated inputs and the GraphQL final-thread result. |

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

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-unify-pr-review-processing/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2219 created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

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

## Acceptance Criteria

- [x] AC1: A rustfmt violation of the repository's unstable import-grouping options fails the
      local pre-commit gate exactly as it fails CI's formatting check, demonstrated on a
      reproduced historical violation.
- [x] AC2: The issue and test-plan templates require naming the toolchain for every recorded
      validation command, and at least one in-repo example follows the requirement.
- [ ] AC3: One unified `process-pr-review` skill handles Copilot and human reviewer threads with
      documented deduplication, superseded-thread, and consolidated-response rules; the two old skills
      redirect to it.
- [ ] AC4: All PR-review audit records are under the canonical `docs/pr-reviews/` parent
  directory; one unified per-PR audit template exists and the migrated documentation directs
  new PRs to it.
- [ ] AC5: The requested reviewer finding format is documented, linked from the PR template, and
      explicitly advisory.
- [ ] AC6: The unified skill and audit template state that the PR author solely owns the tracked
      record and that reviewers (including repository review agents) have no repository-artifact
      obligation.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

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
| M3  | Reviewer-format round trip        | Normalize the literal `[Major][F42] Validation evidence omits the formatter toolchain.` comment in the Unified Audit Contract. | Produces the pinned `F42`, `Major`, `ORIGINAL`, and summary values without free-prose parsing. | TODO | `manual-verification-evidence.md` section V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `manual-verification-evidence.md` section V1; hook JSON result with `failed_step=Checking nightly Rust formatting` |
| AC2   | DONE                   | `docs/templates/ISSUE.md` nightly Rust example; `linter markdown`, `linter cspell`, `linter lychee`, and `git diff --check` |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

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

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.

## References

- Motivating PR: <https://github.com/torrust/torrust-tracker/pull/2174>
- Full pain-point audit: `docs/pr-review-feedback/pr-2174-review-feedback.md` (migrated by T4)
- Consolidated review response: <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593>
- Prior mixed-workflow audit: `docs/pr-review-feedback/pr-2207-review-feedback.md`
- Current skills: `.github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md`,
  `.github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md`
- Current templates: `docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md`,
  `docs/templates/PR-REVIEW-FEEDBACK-TEMPLATE.md`
- Pre-push nightly parity precedent: `contrib/dev-tools/git/hooks/pre-push.sh`
- External linter repository: <https://github.com/torrust/torrust-linting>
