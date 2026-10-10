---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2003
spec-path: docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
epic-owner: da2ce7
last-updated-utc: "2026-10-09 10:25"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
  related-artifacts:
    - .github/skills
    - .github/agents
    - .github/workflows
    - .github/workflows/testing.yaml
    - .githooks
    - contrib/dev-tools/git/hooks
    - contrib/dev-tools/git/install-git-hooks.sh
    - contrib/dev-tools/analysis/workspace-coupling
    - deny.toml
    - project-words.txt
    - AGENTS.md
    - docs/templates/EPIC.md
    - docs/issues/open/1843-migrate-git-hooks-scripts-from-bash-to-rust/ISSUE.md
    - docs/issues/open/1774-automate-cleanup-completed-issues-skill-script/ISSUE.md
    - docs/issues/open/1768-refactor-update-dependencies-skill-automation/ISSUE.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/initial-inventory.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/previous-single-runner-proposal.md
    - docs/issues/closed/2233-2003-tune-unified-pr-review-process/ISSUE.md
    - docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md
    - docs/schemas/frontmatter-v1.schema.json
    - "issue #2264"
    - "issue #2278"
    - "issue #2347"
---

<!-- skill-link: create-issue -->

# EPIC #2003 - Overhaul: Automation Tools and AI Agent Guardrails

**EPIC owner:** da2ce7, since 2026-09-28 ([hand-off](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5873655520), [acknowledgement](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5874065152)); josecelano owned it from its creation until then.

## Goal

Research, design, implement, and progressively adopt repository automation and AI-agent guardrails that make repetitive tasks deterministic where practical and give humans and agents deterministic, timely feedback about whether work satisfies repository rules.

The EPIC starts with discovery, research, and comparison of alternatives. It does not select a tool shape, implementation language, crate layout, or single execution model in advance. After maintainers record the design decision, the EPIC continues through implementation, progressive migration, and removal of superseded paths.

This specification is the EPIC's record. Decisions, findings and plans first posted as comments on the issue are carried here; the comment thread keeps only the friction register index and the current ownership record visible.

## Previous Design Discussion

An earlier discussion explored consolidating repository guardrails into one extensible Rust runner. That proposal introduced independently implemented guardrails, policy-based execution, dependency resolution, shared repository context, standardized results, and identical local and CI entry points.

The discussion is preserved in [`previous-single-runner-proposal.md`](previous-single-runner-proposal.md) as design input. It is not the selected architecture. Its assumptions and trade-offs must be evaluated alongside distributed and incremental alternatives during this EPIC.

## Why This Is Needed

Repository checks and procedures have grown across several independently maintained surfaces:

- `pre-commit.sh` and `pre-push.sh` contain duplicated step-runner, logging, argument-parsing, and output-format logic around different check lists.
- `.github/workflows/testing.yaml` is an existing composite guardrail. It repeats some local checks and also enforces broader guarantees through formatting, linters, documentation tests, workspace tests across targets and features, Cargo layer-boundary bans, container image builds, tracker E2E tests, and qBittorrent E2E tests against SQLite, MySQL, and PostgreSQL.
- Skills and agent instructions describe repeatable workflows and objective rules, but rules expressed only as instructions still depend on an agent interpreting and following them.
- Some architecture rules are already deterministic through `cargo deny check bans`, while other possible repository-policy checks remain manual or have not been evaluated.
- Existing automation proposals choose different script locations, interfaces, and implementation approaches without a shared repository-wide decision framework.

This distribution is not inherently wrong. The problem is that the repository lacks a current inventory showing ownership, overlap, execution cost, feedback behavior, and which rules should remain guidance versus become deterministic applications or tests. Without that evidence, a large consolidation could replace working checks with a more complex system without proving a benefit.

The friction register below adds a second kind of evidence: recorded frictions, places where following a skill, guide or template literally and doing the work accurately diverged, most of them still open, plus two CI findings filed beside them; part 3 of the register's index carries their live counts.

## Design Principles

The research and options analysis must apply these principles:

1. **Maximize determinism**: when a workflow step or rule has objective inputs and pass/fail semantics, prefer an executable application, test, or linter over instructions asking an agent to reproduce the procedure. Keep skills and agent instructions for orchestration, judgment, and context that cannot be encoded reliably.
2. **Minimize inference-token and execution waste**: reduce instructions that only restate deterministic behavior, and avoid repeating expensive checks when an equivalent successful result can be reused safely. Any cache must key results by the exact relevant inputs, configuration, tool versions, and check version. Pre-commit checks may need staged-tree identity, while pre-push and CI checks may use commit or tree identity; branch name or commit identity alone is not always sufficient.
3. **Design for AI agents and humans**: automation must be non-interactive, composable, idempotent where practical, and explicit about side effects. Commands must provide stable exit codes, actionable diagnostics, and streaming machine-readable events using JSON Lines (JSONL/NDJSON) in accordance with the repository CLI output contract. Human-readable presentation may be layered over the same event model.
4. **Preserve local and CI parity**: checks should have one authoritative implementation that can be invoked consistently by developers, agents, hooks, and CI, even when those entry points select different check profiles.
5. **Fail safely and explain recovery**: cached results, skipped checks, partial failures, and destructive operations must be visible and auditable. Automation must state why a result was reused or invalidated and what action is required after failure.

## Tooling Taxonomy

Automation actions and guardrail checks are related but are not equivalent. The EPIC must model their different safety and result contracts while assessing which infrastructure they can share.

| Type | Purpose | Side effects | Typical result |
| ---- | ------- | ------------ | -------------- |
| **Action** | Perform repository work, such as updating dependencies or moving issue specs | Expected; dry-run/apply and idempotency safeguards may be required | Changed, unchanged, skipped, failed |
| **Check** | Evaluate whether repository work satisfies an objective rule | Read-only by default | Passed, warning, skipped, failed |
| **Policy** | Select and order actions/checks for a context such as pre-commit, pre-push, CI, nightly, or release | Inherits the selected operations' effects | Aggregate execution result and event stream |

Actions and checks may share configuration loading, repository discovery, Git/Cargo metadata, dependency planning, caching infrastructure, JSONL/NDJSON events, diagnostics, and progress reporting. They must not share a contract that hides whether an operation mutates state.

Workflows and hooks can themselves be **composite guardrails** when they orchestrate multiple checks into one merge or lifecycle gate. In particular, `.github/workflows/testing.yaml` is a current composite CI guardrail even though its implementation also performs setup and container build actions needed by its checks.

## Scope

### In Scope

- Catalog current automation and guardrails across local hooks, CI workflows, skills, custom agents, repository instructions, linters, dependency checks, and reusable analysis tools.
- Validate and maintain the initial baseline in [`initial-inventory.md`](initial-inventory.md), including explicit unknowns rather than treating the first pass as complete evidence.
- Trace each check or workflow by purpose, owner/source of truth, invocation sites, inputs, outputs, runtime tier, environment requirements, duplication, and failure feedback.
- Distinguish repetitive task automation from verification guardrails; both are relevant, but they require different side-effect, result, retry, and cache contracts even if they share an execution framework.
- Treat hooks and CI workflows as composite guardrails where they aggregate checks, and inventory their setup/action steps separately from the guarantees they enforce.
- Identify objective skill and instruction rules that could be enforced mechanically, while retaining human judgment where a deterministic rule would be brittle or incomplete.
- Assess the likely context and inference-token effect of replacing selected instructions with executable checks, using a documented measurement or estimation method rather than assuming savings.
- Inventory repeated checks and design safe result reuse based on exact input identity so hooks and agents do not rerun unchanged work unnecessarily.
- Research multiple implementation and execution models. Options must include retaining distributed tools with clearer contracts as well as one or more consolidation approaches.
- Compare options using explicit criteria: correctness, feedback latency, local/CI parity, testability, maintainability, portability, incremental adoption, failure modes, developer and agent usability, runtime cost, and migration risk.
- Evaluate check placement across pre-commit, pre-push, CI, and any future repository-policy or architecture-check category without assuming that every check belongs in one runner.
- Explore future architecture-check candidates, including dictionary ordering and dependency policy. Document existing coverage from Cargo and `deny.toml`, remaining gaps, false-positive risk, and whether a new category is justified; do not select a framework prematurely.
- Add a required deterministic check that verifies `project-words.txt` uses one documented ordering rule and contains no duplicate entries. Decide its package and execution tier through the EPIC design rather than coupling it to the tracker library.
- Permit the narrowly scoped interim formatter described by [`2019-automatically-format-project-dictionary/ISSUE.md`](../../closed/2019-automatically-format-project-dictionary/ISSUE.md). It supplies immediate developer feedback but does not select the EPIC's long-term architecture, execution tier, or check/action contract, and may be replaced or refactored after the design decision.
- Permit low-risk, additive, independently verifiable documentation, skill, profile, template, and focused validation subissues to proceed before the EPIC architecture decision when they do not select or depend on a shared runner, cache, enforcement platform, external workflow tool, or broad consumer migration. These exceptions must document their current integration point and remain replaceable by the later design. The approved candidates are the subissues in the Subissues table below; each row states why it may proceed.
- Coordinate the child EPIC to refactor semantic-link and frontmatter conventions. That child owns document metadata and reference semantics, typed profiles, accepted and rejected examples, compatibility policy, and validation behavior. This EPIC retains ownership of shared automation architecture, final binary and package placement, command and event contracts, policy composition, caching, and local/CI/agent integration.
- Permit the child EPIC's small, read-only frontmatter validator to proceed before the architecture decision with pre-commit as its single integration tier, direct path inputs for focused use, and a whole-tree mode for manual validation. It must remain independently testable and replaceable and must not introduce CI integration, a shared runner, cache, policy engine, or orchestration framework before this EPIC selects them.
- Keep the friction register (see Friction Register) as discovery evidence for Proposed Subissues 1 and 2 and as input for the owners named in Decisions Recorded on This EPIC.
- Re-evaluate #1843, #1774, and #1768 against the resulting evidence and recommend whether each should proceed unchanged, be re-scoped, be split, or be superseded.
- Present the evidence and options for maintainer review before selecting a full design.
- Define implementation subissues from the approved design, including ownership, dependency order, migration boundaries, compatibility periods, and independent verification.
- Implement the approved action, check, policy, output, and result-reuse capabilities through those subissues, including the dictionary-integrity check.
- Migrate local, agent, and CI consumers progressively; remove superseded implementations and instructions only after parity and rollback evidence is recorded.

### Out of Scope

- Implementing, migrating, or consolidating automation tools or checks before the design decision and implementation subissues are approved, except for the explicitly approved interim project dictionary formatter linked in Scope.
- Prescribing a `workspace-tools` crate, a single Rust binary, Bash scripts, a task runner, or any other tool shape before alternatives are compared and reviewed.
- Prototyping architecture tests before the research identifies a question that requires a bounded proof of concept and maintainers approve that follow-up.
- Changing the CI/CD provider, release process, or the external `torrust-linting` project.
- Treating every agent instruction as suitable for deterministic enforcement.
- Shutdown and runtime task-ownership work. Issue #1586 belongs with shutdown EPIC #1488 and is unrelated to repository automation or agent guardrails.
- Selecting, requiring, or routing work to a named model or vendor for any review or implementation role.

## Known Existing Issues

These issues are paused dependencies of the EPIC. Their current implementation choices are proposals to re-evaluate, not constraints on the EPIC design. Implementation must not resume until the architecture decision records whether each issue proceeds, is re-scoped or split, or is superseded.

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue | Local Spec | Status | Relationship |
| ----- | ----- | ---------- | ------ | ------------ |
| 1 | #1843 - Migrate git hooks scripts from Bash to Rust | `docs/issues/open/1843-migrate-git-hooks-scripts-from-bash-to-rust/ISSUE.md` | BLOCKED | Pause implementation; runner shape, contracts, check ownership, and migration depend on the design decision |
| 2 | #1774 - Automate cleanup of completed issue specs | `docs/issues/open/1774-automate-cleanup-completed-issues-skill-script/ISSUE.md` | BLOCKED | Pause implementation; action placement, dry-run/apply, GitHub access, and output contract depend on the design decision |
| 3 | #1768 - Refactor update-dependencies skill automation | `docs/issues/open/1768-refactor-update-dependencies-skill-automation/ISSUE.md` | BLOCKED | Pause implementation; action decomposition, shared infrastructure, and validation policy depend on the design decision |

The hold ends only through the Phase 3 decision (AC7), and the three paused specifications do not yet point back to this EPIC; the register tracks that gap as `epic-2003-known-issue-hold-no-exit` (OPEN).

### Findings Without an Owning Subissue

These findings were recorded on this EPIC and have no owning subissue yet. The CI findings are rows in the register's CI group; the others are open questions for Proposed Subissue 1.

- **CI: intermittent `clippy-allow-reasons` CLI test failure** ([comment 5680002696](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5680002696)). One test in `contrib/dev-tools/checks/clippy-allow-reasons/tests/cli.rs` failed once with `Git command failed` on a fork mirror of `develop`, then passed unchanged elsewhere. The test's `git` helper does not report which command failed or its stderr. Proposed fixes so the failure explains itself: include the command and its stderr in the assertion message, and fail loudly when the fixture's temporary root already exists. Register label `clippy-allow-reasons-cli-test-flake` (OPEN).
- **CI: the clippy-allows base-ref fallback breaks on fork push events** ([comment 5681397737](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5681397737)). On a push event, `BASE_REF: ${{ github.base_ref || 'develop' }}` in `.github/workflows/testing.yaml` resolves to the fork's own `develop`. A stale fork therefore reports allowances the pull request never touched. This is the same family as #2179: a merge-base against the upstream remote, or skipping the job on non-`develop` pushes, would fix both. Register label `testing-yaml-base-ref-fork-push-fallback` (OPEN).
- **Hook-gate descriptions disagree** ([comment 5813999181](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5813999181), at `fa698c43`). Seven texts give four different pre-commit step counts: `contrib/dev-tools/git/hooks/pre-commit.sh`, the `run-pre-commit-checks` skill, root `AGENTS.md`, `docs/testing.md`, this EPIC's `initial-inventory.md` and the #1843 body. Also, `AGENTS.md` and the `run-pre-push-checks` skill say pre-push repeats no pre-commit step, yet both hooks run a nightly format check. Open questions: which description is the reference, and whether the repeated nightly format check is intended (Proposed Subissue 4 targets such repeats).
- **Folder-style leftovers after #2159** ([comment 5814000222](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5814000222)). Three texts are out of step:
  - `docs/issues/open/AGENTS.md` keeps a legacy-standalone paragraph although no standalone specification remains.
  - `docs/copilot-pr-reviews/pr-2241-copilot-suggestions.md` is the only flat review record outside `docs/pr-reviews/`.
  - `docs/adrs/README.md` does not name `docs/templates/ADR.md`, although the `create-adr` skill does.

  Open questions: move the flat record or keep it as a historical exception; drop the legacy paragraph; point the ADR README at the template.

## Subissues

These are the approved early-implementation subissues. Each meets the Scope exception for low-risk, additive, independently verifiable work and selects no long-term automation or orchestration architecture. Final GitHub issue creation for a new row remains subject to maintainer approval of its draft specification.

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue | Local Spec | Status | Notes |
| ----- | ----- | ---------- | ------ | ----- |
| 1 | #2155 - Document AI agent orchestration | `docs/issues/closed/2155-2003-document-ai-agent-orchestration/ISSUE.md` | DONE | Documents current profiles and creates evidence for future enforcement; selects no enforcement tool. No dependencies. |
| 2 | #2156 - Create Markdown template skill | `docs/issues/closed/2156-2003-create-markdown-template-skill/ISSUE.md` | DONE | Documentation convention and skill only; does not alter shared execution architecture. No dependencies. |
| 3 | #2157 - Require documented Clippy allows | `docs/issues/closed/2157-2003-require-documented-clippy-allows/ISSUE.md` | DONE | Focused policy and validator at an existing validation tier; prospective baseline remains replaceable. Depends on existing lint entry points. |
| 4 | #2158 - Inventory existing Clippy allows | `docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md` | DONE | Evidence and incremental remediation; does not redesign the linter runner. Follows #2157's policy conventions. |
| 5 | #2159 - Adopt folder-style issue specifications | `docs/issues/closed/2159-2003-adopt-folder-style-issue-specs/ISSUE.md` | DONE | Documentation, templates, and root ADR only; prospective and reversible for new work. Leftovers are listed under Findings Without an Owning Subissue. |
| 6 | #2160 - Persist independent agent review reports | `docs/issues/closed/2160-2003-persist-independent-agent-review-reports/ISSUE.md` | DONE | Profile/template documentation and explicit records; does not enforce transitions technically. Follows #2155's documentation conventions. |
| 7 | #2185 - Triage advisory external-link check findings | `docs/issues/closed/2185-2003-triage-advisory-external-link-check-findings/ISSUE.md` | DONE | Evidence-driven refinement of an existing advisory workflow; does not redesign the linter or workflow runner. Builds on the #2162 external-link workflow. |
| 8 | #2219 - Unify PR review-processing workflow | `docs/issues/closed/2219-2003-unify-pr-review-processing/ISSUE.md` | DONE | Low-risk, additive, independently verifiable local/CI formatting parity plus skill, template, and review-audit process improvements; selects no shared automation architecture. Depends on existing pre-commit, pre-push, and PR-review entry points. |
| 9 | #2233 - Tune unified PR-review process | `docs/issues/closed/2233-2003-tune-unified-pr-review-process/ISSUE.md` | DONE | Evidence-driven follow-up to #2219; refines skills, templates, audit process, and design notes without selecting shared automation architecture. |
| 10 | #2264 - Refactor semantic-link and frontmatter conventions (child EPIC) | `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md` | IN_PROGRESS | Defines document metadata and reference semantics. Its read-only frontmatter validator may use pre-commit as its single integration tier but must remain replaceable by the architecture selected here. #2266, #2280 and #2281 (the validator command and its pre-commit rollout) are closed. |
| 11 | #2278 - Strengthen PR review author self-audit and evidence generation (child EPIC) | `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md` | IN_PROGRESS | Converted from a task by #2288 after its improvement matrix showed several independently reviewable artifacts; split into one-PR subissues. It owns the author-side register items (see Decisions Recorded on This EPIC) and, from #2347, its order 8 and #2362 (order 11). It remains read-only at the helper boundary and selects no model, shared runner, cache, policy engine, or CI integration. Evidence: PR #2270, #2271 and #2272 retrospectives, #2219 and #2233, and #2266 for check-crate placement. |
| 12 | #2347 - Triage post-merge review findings on PRs #2290, #2293, #2300, #2313, and #2320 | `docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md` | IN_PROGRESS | Processes the 32 findings through the existing `process-pr-review` post-merge workflow: audits, approved dispositions and documentation fixes only; workflow, CI and Rust changes become their own issues. Every finding has an approved disposition, a disposition reply and an audit row; the implementation is PR #2363. The close-out is pending; see Order 12 Close-Out. |
| 13 | #2360 - Resolve the crate-level scope of the UDP protocol `empty_enums` allowance | `docs/issues/open/2360-2003-resolve-udp-protocol-empty-enums-allowance-scope/ISSUE.md` | TODO | Follow-up of #2347 (`review-finding:pr-2290-f4`); approved 2026-09-28 and landed with PR #2363. |
| 14 | #2375 - Define unambiguous issue specification directory names | `docs/issues/open/2375-2003-unambiguous-issue-spec-names/ISSUE.md` | TODO | Documentation convention, a name-consistency check at the existing `frontmatter-validator` tier, and a mechanical rename of `drafts/` and `open/` specs. Selects no automation architecture; coordinates validator placement and path references with #2264 (frontmatter and semantic-link conventions). Scheduling: implementation starts only after a fresh review of the spec against the current tree, the rule the spec itself introduces. That review settles two open points from the #2377 review: whether a slug may begin with a reserved `i<digits>-` or `e<digits>-` prefix, and whether the guarantee covers `closed/` names. The name check's placement follows #2264. |
| 15 | #2473 - Run design discussions as asynchronous, attributed rounds | `docs/issues/closed/2473-2003-asynchronous-discussion-rounds/ISSUE.md` | DONE | Documentation process and a Markdown template only: rewrites the `docs/discussions/` lifecycle so participants contribute through attributed rounds and only the decision needs agreement. Selects no automation architecture. No dependencies; proposed in PR #2467. |

Also linked as GitHub sub-issues, outside the order:

- #2019: the interim project dictionary formatter that Scope permits (DONE, `docs/issues/closed/2019-automatically-format-project-dictionary/ISSUE.md`).
- #2022: the vendored maintainer merge workflow (DONE, `docs/issues/closed/2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md`).
- #2261: the review of the UDP protocol Clippy baseline (DONE, `docs/issues/closed/2261-2003-review-udp-protocol-clippy-baseline/ISSUE.md`).

### Order 12 Close-Out

Issue #2347 closes through one close-out PR. PR #2363, which carries the #2347 implementation, merged on 2026-09-28 at 17:00 UTC as `bc90cde1b` after seven review rounds; the last approval (review 5341908562) and its ACK stand on the merged head, whose commit records rounds 4 and 5 in the PR's own audit.

- [ ] Change the 13 rows fixed in PR #2363 from `FOLLOW_UP`/`OPEN` to `FIXED`, citing their commit subjects.
- [ ] Post the final replies, and resolve those 13 threads and the 9 `NO_ACTION` threads.
- [ ] Keep the 9 `FOLLOW_UP` threads open until their owners' fixes merge.
- [ ] Log PR #2363's later review rounds in its PR-review record. That covers the round-4 approval, the ACK, and the stale-approval dismissal after the round-2 audit push.
- [ ] Record in the PR-review record the findings and rounds it does not yet carry: F14 (review 5340689822, at the head of `docs(pr-reviews): [#2347] record PR #2363 review round 2`: the audit said Copilot gave no severity while review 5338724829's overview rates one item Medium and three Low; fixed in `docs(pr-reviews): [#2347] record Copilot's severity labels in the PR #2363 audit`) and the processing-log entries for rounds 4 to 6 (review 5341713976, F15).
- [ ] Run M2, close AC4, then close #2347 and archive its specification.

### Follow-Up Owners from #2347

| Owner | Home | Finding | Scope |
| ----- | ---- | ------- | ----- |
| #2360 | This EPIC, order 13 | `review-finding:pr-2290-f4` | The crate-level scope of UDP protocol allowance A159 |
| #2361 (bug) | #1347, order 9 | `review-finding:pr-2293-f1` | The report-only coverage summary fails when discovery fails |
| #2301 | #1347, existing coverage rollout review (`docs/issues/open/2301-1347-review-package-coverage-rollout/ISSUE.md`) | `review-finding:pr-2293-f5` | Its specification records hosted run 36305549957, which settles the source-prefix risk |
| #2278 order 8 | #2278 | `review-finding:pr-2300-f10` | Pin the whole audit roster on both sides |
| #2362 | #2278, order 11 | `review-finding:pr-2313-f4`, `-f5`, `-f6`, `-f7`, `-f9` | The remaining audit-contract rules; coordinate its placeholder pin change with #2349 (`docs/issues/open/2349-2278-contract-checker-evidence-boundary/ISSUE.md`) |

### Drafts Awaiting Issues

The Subissues table lists only created issues. These draft specifications on `develop` are placed in the plan; each becomes a row when its issue is created.

- **`docs/issues/drafts/2003-mine-ai-agent-memories/ISSUE.md`** (PR #2388) mines the 110 GitHub Copilot repository memories and 50 local Copilot Chat memories for missing, hard-to-find or contradictory guidance. It falls under the Scope exception: it is evidence and documentation (snapshots, a classification ledger, a report, small approved guidance changes and memory cleanup), it implements no new check, and it selects no architecture. It takes the next order when its issue is created. Its report is input for Proposed Subissue 1 (what agents had to remember instead of finding) and Proposed Subissue 2 (the deterministic checks it proposes become candidates there). Its frontmatter and issue-metadata findings go to #2264. It overlaps the draft `docs/issues/drafts/mine-pr-review-audit-records/ISSUE.md` (PR #2371, parent not yet decided), because Copilot code review writes the memories while reviewing the PRs those audit records cover; shared findings are cross-referenced, not proposed twice.
- **`docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md`** (PR #2385, from #2298) moves the six AI-harness crates under `contrib/dev-tools/` into their own Cargo workspace. It is not a Scope exception: it changes hook and CI invocations, and its first task waits for five decisions this EPIC owns (workspace location, what counts as harness, binary shape, scripts versus Rust, ownership and versioning). Those decisions are inputs to Proposed Subissues 6 and 7, and they are recorded under Decisions Recorded on This EPIC before the issue is created. Until the separation lands, each harness crate needs one line in the interim allow-list block of `.dockerignore` and stays out of `default-members` in `Cargo.toml`, as `docs/adrs/20260929183441_build_container_from_positive_lists_with_external_only_dependency_cache.md` records.
- **`docs/issues/drafts/2003-adopt-prose-tests-as-executable-specification/ISSUE.md`** (PR #2437) drafts the ADR that records prose-style tests as the executable specification of behavior, this EPIC's first design principle (maximize determinism) applied, and it settles where a product decision lives (Specifications and rationale, under Decisions Recorded on This EPIC); the EPIC owner accepted its placement under this EPIC in that pull request. It falls under the Scope exception: it is documentation and a convention (a root ADR and updates to the `write-unit-test` and `create-adr` skills and `docs/testing.md`), it implements no new check, and it selects no automation architecture. It becomes a subissue row when its issue is created.

## Undecided Improvement Candidates

These five candidates come from the #2347 implementation retrospective's "Improvements for Future Work" section, which PR #2363 adds. None is decided. Each either moves into an owner's specification, with a progress-log line here, or is declined with its reason.

1. **A read-only check that lists unresolved review threads on recently merged PRs.** The 32 findings of #2347 were found by chance. Suggested home: this EPIC (a candidate for Proposed Subissue 2) or #2278.
2. **An `AGENT-REVIEW-REPORTS.md` rule to cite commits by subject, never by branch SHA.** Three of the 32 findings were this defect. The hand-off suggests no home; the register's `agent-review-reports-vs-branch-commit-id-rule` covers the same template.
3. **In post-merge triage, search for an existing owner under the affected folder's parent EPIC.** #2347 T2 missed #2301 this way. The candidate touches `process-pr-review`; the hand-off suggests #2362 or "order 5".
4. **Take log and comment stamps from `date -u` or the event's `created_at`, never an estimate, and stamp a batch at its last event.** The hand-off suggests no home.
5. **When disposition replies must precede audit rows, commit the approval line to each existing audit first.** The deviation #2347 made is accepted in [#2347 comment 5869786394](https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5869786394). The candidate touches `process-pr-review`; the hand-off suggests #2362 or "order 5".

## Proposed Research and Design Subissues

These are proposed planning subissues; none has been created. Titles and boundaries may be adjusted during maintainer review; no GitHub issues should be created from this draft without approval. Proposed Subissues 4 and 5 take [comment 5444339176](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5444339176) as design input. That comment proposes a streaming NDJSON event contract, sketched as a tagged event enum with an input digest, a cache-hit flag, a duration and a cached-skip verdict. It also proposes a content-addressed cache key over rule version, tool version, target content and configuration bytes.

| Order | Proposed Issue | Intent | Expected Output | Verification | Dependencies |
| ----- | -------------- | ------ | --------------- | ------------ | ------------ |
| 1 | Inventory repository automation and guardrails | Validate and complete the initial current-system evidence baseline | Reviewed revision of `initial-inventory.md` and invocation/overlap map covering local, CI, skill, agent, and architecture-policy surfaces | Sample entries traced end to end; catalog cross-checked against repository entry points and reviewed for omissions | Initial inventory in this EPIC; the open questions under Findings Without an Owning Subissue |
| 2 | Assess deterministic automation and guardrail candidates | Separate objective rules that are candidates for automation from judgment-based guidance and estimate benefits | Candidate matrix with determinism, current failure mode, proposed enforcement point, expected benefit, token-impact method, cost, and false-positive risk | Representative skill and instruction rules reviewed by maintainers; rejected candidates retain rationale | Subissue 1; the friction register |
| 3 | Enforce project dictionary integrity | Add the known required guardrail without coupling it to the tracker library | Deterministic test or check proving `project-words.txt` is sorted by a documented rule and has no duplicates; selected execution tier and actionable failure output | Positive test plus mutations for out-of-order and duplicate entries; invocation verified through the selected local and CI profiles | Subissue 2 for placement and interface decision |
| 4 | Research safe check-result reuse | Avoid rerunning equivalent successful pre-commit, pre-push, and agent checks | Cache-key model, invalidation rules, audit record, threat/failure analysis, and measured savings for representative workflows | Mutations to staged content, commit/tree, configuration, tool version, and check version invalidate stale results; exact matches reuse results visibly | Subissues 1 and 2 |
| 5 | Define agent-friendly automation contracts | Standardize non-interactive execution and machine-readable feedback | Contract for JSONL/NDJSON events, stable exit codes, diagnostics, progress/heartbeat, side-effect reporting, and idempotent retries | Fixture or contract tests cover success, failure, progress, cache hit/miss, and malformed invocation | Subissues 1 and 2 |
| 6 | Research and compare architecture options | Evaluate viable organization, execution, feedback, and migration models without preselecting a tool | Options paper with diagrams, decision criteria, trade-offs, migration paths, and bounded proof-of-concept recommendations where evidence is insufficient | Every criterion and design principle applied consistently to each viable option; claims linked to inventory evidence or experiments | Subissues 1, 2, 4, and 5 |
| 7 | Record maintainer decision and implementation roadmap | Convert the reviewed options into an explicit decision or documented request for more evidence | Decision record, disposition of #1843/#1774/#1768, ordered implementation scope, migration plan, and implementation-ready specs | Maintainer review recorded; roadmap items trace to selected option and include independent verification criteria | Subissues 3 and 6 |
| 8 | Implement approved automation foundation | Build only the shared contracts and infrastructure justified by the decision | Tested implementation of the approved operation model, event and exit-code contracts, configuration, and any selected planning or result-reuse infrastructure | Contract, unit, integration, failure, and invalidation tests pass; implementation maps to the decision record without speculative framework features | Subissues 4, 5, and 7 |
| 9 | Implement and migrate approved operations | Deliver the approved actions and checks, then move consumers without losing current guarantees | Dictionary-integrity guardrail plus approved #1843/#1774/#1768 scopes; local, agent, and CI migration; superseded-path removal | Old/new parity or intentional-difference evidence, rollback exercise, consumer migration audit, and selected local/CI policies pass | Subissue 8 |
| 10 | Validate rollout and close the EPIC | Prove the resulting system is usable, maintainable, and no longer depends on superseded paths | Runtime and token-impact results, final ownership map, operating documentation, residual-risk record, and closure dispositions | Representative human and agent workflows pass; required CI guarantees remain enforced; stale references and temporary compatibility paths are removed | Subissue 9 |

## Decisions Recorded on This EPIC

- **Interim dictionary formatter (2026-07-22).** A narrowly scoped interim project dictionary formatter (#2019) may proceed. It may be replaced or refactored after the EPIC design decision.
- **Author-side register items go to #2278 (2026-09-22, [comment 5772662609](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5772662609)).** The `retrospective-improvement-matrix.md` of #2278, which #2288 converted into a child EPIC, dispositions every author-side register item together with the three PR retrospectives:
  - **Already specified, now named in tasks:** F64, F65, F74, F75, and the false-evidence part of F56.
  - **Skill/template contradictions, adopted as one contract reconciliation with no new fields:** F58, F60, F61, F62, F63, F66, F73, F76, F77, F79 and F80. These were verified against `develop` `ffa3528c` before adoption.
  - **Helper-skill alignment, adopted:** F17, F32 and F18.
  - **Tooling, adopted:** F7 (a Rust port of `validate-audit-record.py` with parity fixtures, following #2266's check-crate placement) and F57.
  - **Deferred:** F55. Pre-commit has no pull-request context, so only a self-audit step is in scope; a PR-time check is a CI decision for this EPIC.
  - **Left to other owners:** the reviewer-side `review-pr` items, planning-template items, #2264 items (including the F51 sweep), linter and CI items, and CI invocation of the checkers.
  - **Boundary:** the audit validator validates the audit body only. Frontmatter and `review-finding:` target existence stay with #2264.

  The register's index records where each of these numbers stands now.
- **Frictions are filed under semantic labels (2026-09-26).** The F series is closed at F102, and the index is the register; see Friction Register.
- **Ownership (2026-09-28).** This EPIC and #2347 pass to da2ce7, per the [hand-off](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5873655520) and the [acknowledgement](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5874065152); the assignees changed at 16:17 UTC. The #2347 close-out, the follow-up owners and the five improvement candidates are taken as listed and placed from this EPIC.
- **Draft placement (2026-10-01).** The memories draft (`2003-mine-ai-agent-memories`) proceeds under the Scope exception and takes the next order when its issue is created; its report feeds Proposed Subissues 1 and 2. The harness-workspace draft (`2003-separate-ai-harness-cargo-workspace`) is an implementation subissue gated on the five decisions it lists, which are pending and are recorded here before its issue is created. Order 14 (#2375) is scheduled behind a fresh review of its spec against the current tree; see Drafts Awaiting Issues and the Subissues table.
- **Four-aspect frame (2026-10-03, [review 5400754664](https://github.com/torrust/torrust-tracker/pull/2428#pullrequestreview-5400754664), [Outcome](../../../discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md#outcome)).** The four-aspect frame of the goals-and-boundaries discussion is this EPIC's structure for sorting proposals.
- **Gates (2026-10-03, [review 5400754664](https://github.com/torrust/torrust-tracker/pull/2428#pullrequestreview-5400754664), [Outcome](../../../discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md#outcome)).** The gates are not a fifth aspect: they are the shared enforcement layer through which every aspect's contract is expressed, owned by this EPIC's architecture decision. That decision settles where each gate runs, its output contract and how a gate is added, together with the five decisions in `docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md`.
- **Contract or technique (2026-10-03, [review 5400754664](https://github.com/torrust/torrust-tracker/pull/2428#pullrequestreview-5400754664), [Outcome](../../../discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md#outcome)).** A requirement is contract when a gate can check it from tracked bytes or GitHub state without knowing which tool produced the work, and technique otherwise:
  - contract: a recorded independent review of the acceptance criteria, acceptance-criteria evidence in the specification, the pre-commit gate and linters, Conventional Commit subjects, and the PR-review audit record;
  - technique: which named profile performs a step, pair review and model routing; a complexity audit after every step has an objective result and belongs in a gate;
  - follow-up work: reword the issue template's Committer checkpoint, which names a role where the checkable fact is that the specification's progress matches the commit, and the orchestration guide.
- **Coordination claims (2026-10-03, [review 5400754664](https://github.com/torrust/torrust-tracker/pull/2428#pullrequestreview-5400754664), [Outcome](../../../discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md#outcome)).** A claim on work lives in the issue assignee, the only claim GitHub keeps as structured state. Assign the issue before starting, then open a draft pull request that references it as the second, observable signal; the proposed overlap check flags open pull requests that reference the same issue or touch the same files.
- **Copies of GitHub state (2026-10-03, [review 5400754664](https://github.com/torrust/torrust-tracker/pull/2428#pullrequestreview-5400754664), [Outcome](../../../discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md#outcome)).** Identifiers (`github-issue`, `related-pr`) stay, because they do not go stale; a copied state drifts unless it is stamped with its capture time or checked:
  - specification `status` and folder: kept, and checked at archive time;
  - the Status column of EPIC subissue tables: removed or checked;
  - the friction register's stamped state line: the honest form of a copy;
  - the audit record's copied states and `epic-owner`: recorded with the audit-record contract on #2278.
- **Discussions convention (2026-10-03, [review 5400754664](https://github.com/torrust/torrust-tracker/pull/2428#pullrequestreview-5400754664), [Outcome](../../../discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md#outcome)).** The `docs/discussions/` convention holds, with two additions for its guide: once a discussion's Outcome is recorded and links its canonical record, the discussion is edited only to repair links, so that it does not become a second copy of the decision; and the guide says whether discussions are kept permanently or pruned like closed specifications. The semantic-link parts of these outcomes are recorded on #2264 and the audit-record parts on #2278.
- **Specifications and rationale (2026-10-03, [review 5402713128](https://github.com/torrust/torrust-tracker/pull/2428#pullrequestreview-5402713128); revised 2026-10-04 in [review 5407197495](https://github.com/torrust/torrust-tracker/pull/2428#pullrequestreview-5407197495); [Outcome](../../../discussions/2003-overhaul-guardrails-and-automation/20261003-specifications-and-rationale/README.md#outcome)).**
  - The long-term specification of behavior is the tests that drive a public interface: package integration tests under `packages/*/tests/`, application-level tests under `tests/`, and behavior-named handler tests; unit tests of private seams do not count. Its naming contract is the `write-unit-test` skill's `it_should_{expected_behavior}_when_{condition}` rule, with module names carrying the context, and because the tree does not follow it uniformly yet, a naming check comes before any outline is relied on. Marking and indexing are engineering for the implementing specification; a test as a link target needs #2264's `rust-item` kind and its resolver.
  - `AGENTS.md` keeps its subject grouping: rules are looked up by subject, a tier is a property of a rule rather than a place, and a rule whose tier is unclear can say "always", "ask first" or "never".
  - Where a product decision lives is settled by the prose-tests ADR that [`docs/issues/drafts/2003-adopt-prose-tests-as-executable-specification/ISSUE.md`](../../drafts/2003-adopt-prose-tests-as-executable-specification/ISSUE.md) drafts. Until that ADR is accepted, both homes are in use: Rustdoc beside the tests that specify the rule, and an ADR. The decision log and impact maps stay deferred until then, and a capability's goal lives in its EPIC's Goal section.
  - One issue template, with optional and conditional sections marked and no short form, because a second form is another copy to keep in step. The work is this EPIC's planning-template follow-up, not #2278's; the register tracks it as `issue-template-checkpoint-optionality-unmarked` and `create-issue-init-list-omits-conditional-sections`.

## Friction Register

A friction is a place where following a repository skill, guide or template literally and doing the work accurately diverged. Each entry records what the text says, what the work met, and the smallest change that would close the gap. The register is kept in four comments on this issue, edited in place; at index v31 (2026-10-08 13:31 UTC) they hold 12 artifact groups and 238 labels, 236 frictions and two CI findings:

- [Index part 1](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5847229228) holds the first two artifact groups: the reviewer side and the author workflow.
- [Index part 2](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5871784196) holds the third: the planning skills and spec templates.
- [Index part 3](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-6001593887) holds the next four (the audit template, the audit tooling, the hooks, linters and maintenance skills, and the CI findings), then the former-number map, the counts and the method; it carries the live counts.
- [Index part 4](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-6040926242) holds the last five: the link and frontmatter convention, the repository rules, the discussions, the review-thread helper skills and the external linter.

- **Labels.** Each friction has a kebab-case label that starts with the owning artifact and names the defect in about six words, at most eight parts. A label is unique, derivable from the gist, and stable when rows are reordered or merged. Labels replace running numbers because a number carries no meaning, collides with reviewers' finding IDs and with re-raises, and turns deduplication into a separate lookup.
- **The F series is closed at F102.** Each former number maps to exactly one label in the former-number map in part 3. Numbers that re-raised one defect share a label, and F5, F9–F13 and F30 were never filed here. New frictions receive a label only.
- **Dispositions.** Each disposition is recomputed at a stated `develop` head, not copied from an earlier summary, and the precedence is **ADOPTED** > **SUPERSEDED** > **SPECIFIED** > **OPEN**:
  - **ADOPTED:** the fix is on `develop`, cited with file:line and landing commit.
  - **SUPERSEDED:** a later change removed the ground.
  - **SPECIFIED:** an open spec on `develop` names the fix as a task.
  - **OPEN:** none of the above.
- **Filing.** Every filing edits the index in place: a new row linking its filing comment, or a status note on an existing label. A draft that duplicates a label becomes a status note on that label, not a new row. Each filing comment carries the full text of its entries. Once the index carries a filing comment's rows, that comment is minimized; it stays reachable through the index's links.

Counts at the index's revision of 2026-09-28 (the ninth labelled batch), kept as a snapshot: groups and labels have been added and moved between parts since, and part 3 carries the live counts.

| Artifact group | Index part | Labels | ADOPTED | SUPERSEDED | SPECIFIED | OPEN |
| -------------- | ---------- | ------ | ------- | ---------- | --------- | ---- |
| Reviewer side: `review-pr/SKILL.md` and `REVIEW-FINDINGS.md` | 1 | 20 | 1 | 1 | 0 | 18 |
| Author workflow: `process-pr-review/SKILL.md` | 1 | 24 | 5 | 2 | 4 | 13 |
| Audit template: `PR-REVIEW-TEMPLATE.md` | 1 | 11 | 6 | 0 | 1 | 4 |
| Audit tooling: `validate-audit-record.py`, `agent-review-report-contract` | 1 | 6 | 0 | 0 | 3 | 3 |
| Review-thread helper skills: `fetch-review-threads`, `resolve-review-threads` | 1 | 4 | 4 | 0 | 0 | 0 |
| Planning skills and spec templates | 2 | 32 | 1 | 0 | 1 | 30 |
| Link and frontmatter convention | 2 | 8 | 0 | 0 | 2 | 6 |
| Repository rules and git-workflow skills | 2 | 8 | 2 | 0 | 0 | 6 |
| Hooks, linters and maintenance skills | 2 | 11 | 0 | 0 | 1 | 10 |
| External: `torrust-linting` | 2 | 3 | 0 | 0 | 0 | 3 |
| CI findings (filed as not frictions) | 2 | 2 | 0 | 0 | 0 | 2 |
| **Total** | | **129** | **19** | **3** | **12** | **95** |

At that revision the largest open clusters were the planning skills and spec templates (30 open) and the reviewer side (18 open). The #2278 matrix names the reviewer-side items as input for a separate reviewer-side issue, which does not yet exist.

**PR #2484 review retrospective (2026-10-08).** Its six proposals are dispositioned in #2278's improvement matrix. Items 1 to 3 go to #2278: the manual pre-push sweep and anchoring verifications to headings or quoted phrases are adopted for its order 5, the sweep's tool is deferred to its order 8, and deferring non-blocking findings after an approval to its order 10. The parts this EPIC owns enter the register with the next batch: item 4 (one stamp per event in spec progress logs) on `issue-template-log-correction-rule-unstated`, item 5 (a step for restructuring a spec under review) on `create-issue-issue-to-epic-conversion-unstated`, item 6 (one session per pull-request branch) as a new label against `docs/agents/orchestration.md`, and item 1's tree-wide path sweep and line-width check on `linter-path-citations-unchecked` and `write-markdown-docs-line-wrap-rule-unstated`. Item 4 overlaps Undecided Improvement Candidate 4, which stays listed until it is placed.

## Delivery Strategy

Use an evidence-first, progressive delivery strategy because the problem crosses repository workflows, developer tooling, and agent behavior, while the desired architecture is intentionally unsettled. Discovery and candidate analysis can gather evidence independently, but architecture selection and implementation must wait until both are complete.

Research artifacts should be committed as durable documentation under the EPIC or an approved canonical docs location. Architecture-dependent implementation subissues begin only after maintainers review the alternatives and record a decision. The subissues in the Subissues table may proceed now because they are additive, independently verifiable, and do not preselect the shared architecture. The later decision may keep the current distributed model, approve only targeted improvements, select consolidation, or request a bounded experiment before committing to the remaining implementation.

The EPIC is in Phase 1 (Discovery). None of the ten proposed research subissues exists, AC1–AC17 are `TODO`, and the early-implementation subissues proceed under the Scope exception.

For each completed subissue in this EPIC, the default completion policy is:

1. Run applicable automatic checks (`linter markdown`, `linter cspell`, and any tests for research utilities or prototypes explicitly approved later; `linter all` and relevant tests for implementation subissues).
2. Run the defined manual review scenarios and record evidence.
3. Re-review the subissue and EPIC acceptance criteria against the produced evidence.
4. Complete an evidence-based implementation review. Create or update an issue-local retrospective for reusable lessons, material design changes, or meaningful deviations from the plan; otherwise record why one was unnecessary in the issue progress log.

### Phase 1: Discovery

- Outcome: a validated inventory and overlap map of the current automation and guardrail system.
- Exit criteria: maintainers can trace what runs, where it runs, what it enforces, and where duplication, redundant execution, feedback gaps, or manual-only rules exist. The initial inventory is reviewed, corrected, and accepted as the baseline for later comparisons.

### Phase 2: Candidate and Options Analysis

- Outcome: a ranked candidate matrix and comparison of multiple viable architectures.
- Exit criteria: alternatives use common evaluation criteria, identify unresolved evidence, and avoid assuming a single binary, crate, language, or check category.

### Phase 3: Maintainer Decision and Implementation Planning

- Outcome: maintainers select an option, choose targeted changes, request further research, or explicitly retain the current structure.
- Exit criteria: the decision and rationale are recorded; existing issues have dispositions; only approved implementation work has implementation-ready specs and ordering.

### Phase 4: Foundation Implementation

- Outcome: the minimal shared contracts and infrastructure selected by the decision are implemented and tested without migrating all consumers at once.
- Exit criteria: operation contracts, machine-readable events, failure behavior, and any approved cache or planning behavior pass focused tests; rollback remains possible.

### Phase 5: Operation Implementation and Progressive Migration

- Outcome: approved actions and checks are delivered, including dictionary integrity, and local, agent, and CI consumers move to the selected interfaces in reviewable increments.
- Exit criteria: each migration preserves or intentionally revises documented guarantees; superseded paths are removed only after parity, failure, and rollback evidence is accepted.

### Phase 6: Rollout Validation and Closure

- Outcome: the delivered system has final ownership, usage, performance, and maintenance evidence, with paused issues closed, re-scoped, or completed according to the decision.
- Exit criteria: representative human and agent workflows pass; required CI guarantees remain; stale references and temporary compatibility paths are removed; residual risks are recorded.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [x] Early-implementation subissues created and listed in the Subissues table
- [ ] Research/design subissues approved, created, and linked in this spec
- [ ] Initial inventory reviewed and accepted as the Phase 1 baseline
- [ ] Phase 1 discovery evidence reviewed
- [ ] Phase 2 candidate and options analysis reviewed
- [ ] Phase 3 maintainer decision recorded
- [ ] Phase 4 foundation implementation completed and verified
- [ ] Phase 5 operation implementation and progressive migration completed
- [ ] Phase 6 rollout validation completed
- [ ] Existing issue dispositions recorded for #1843, #1774, and #1768
- [ ] Order 12 (#2347) close-out completed and its specification archived
- [ ] Undecided improvement candidates placed with an owner or declined
- [ ] Subissue statuses kept up to date in the relevant tables
- [ ] For each completed subissue: automatic checks completed and recorded
- [ ] For each completed subissue: manual verification completed and recorded
- [ ] For each completed subissue: acceptance criteria reviewed post-completion
- [ ] For each completed subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-20 00:00 UTC - Copilot - Initial epic draft
- 2026-07-20 00:00 UTC - Planner - Refined draft to make discovery and options analysis the immediate scope; removed unrelated and unsupported references; separated existing issues from proposed research and design work - draft updated
- 2026-07-20 00:00 UTC - Copilot - Converted the draft to a folder-type EPIC and recorded an earlier single-runner design discussion as a non-binding supporting artifact
- 2026-07-20 00:00 UTC - Copilot - Distinguished mutating automation actions from read-only guardrail checks and documented the testing workflow as an existing composite CI guardrail
- 2026-07-20 00:00 UTC - Copilot - Added the initial repository inventory, paused existing implementation issues pending the design decision, and extended the EPIC through implementation, progressive migration, rollout validation, and closure
- 2026-07-20 00:00 UTC - josecelano - Approved the draft EPIC and its supporting artifacts
- 2026-07-20 00:00 UTC - GitHub Operator - Created EPIC #2003 and moved the approved local specification to `docs/issues/open/2003-overhaul-guardrails-and-automation/`
- 2026-07-22 00:00 UTC - josecelano - Approved a narrowly scoped interim project dictionary formatter; it may be replaced or refactored after the EPIC design decision
- 2026-09-10 09:51 UTC - da2ce7 - Filed the first frictions on this EPIC, found while aligning EPIC #2190 with the planning guides; the numbered series reached F102 on 2026-09-26 - [comment 5616633026](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5616633026)
- 2026-09-21 20:45 UTC - da2ce7 - General review of the thread: the register F1–F81 summarized by theme - [comment 5767266486](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5767266486)
- 2026-09-22 07:20 UTC - josecelano - Author-side register items dispositioned through #2278's improvement matrix (see Decisions Recorded on This EPIC) - [comment 5772662609](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5772662609)
- 2026-09-24 12:23 UTC - da2ce7 - Inventory input for Proposed Subissue 1: hook-gate descriptions, folder-style leftovers, and skill-link and merge-tool measurements - [comment 5813999181](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5813999181), [comment 5814000222](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5814000222), [comment 5814006328](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5814006328)
- 2026-09-26 12:20 UTC - GitHub Copilot - Maintainer approved subissue #2347 (order 12) to triage 32 review findings posted after PRs #2290, #2293, #2300, #2313, and #2320 merged; created and linked it under this EPIC
- 2026-09-26 14:53 UTC - da2ce7 - Overhauled the friction register: semantic labels replace running numbers, the F series is closed at F102, and every disposition is recomputed at `develop` `4fd1876cc`; the index is edited in place on every later filing - [index](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5847229228), [overhaul](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5847232340)
- 2026-09-28 09:42 UTC - GitHub Copilot - Maintainer approved subissue #2360 (order 13), follow-up FU-A of #2347 for `review-finding:pr-2290-f4`; created and linked it under this EPIC
- 2026-09-28 14:16 UTC - da2ce7 - Split the register index into two comments to keep each under the comment size limit - [index part 2](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5871784196)
- 2026-09-28 15:54 UTC - josecelano - Handed this EPIC and #2347 over: #2347 status, its close-out list, the follow-up owners, and five undecided improvement candidates - [hand-off](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5873655520)
- 2026-09-28 16:18 UTC - da2ce7 - Acknowledged the hand-off; da2ce7 holds #2003 and #2347 (assignees changed 16:17 UTC) - [acknowledgement](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5874065152)
- 2026-09-28 17:03 UTC - da2ce7 - Overhauled this specification to carry what the thread established: the Subissues table through order 13, the order 12 close-out, the follow-up owners, the decisions, the undecided candidates, the friction register's rules and counts, and the findings without an owner; the comments it carries are minimized
- 2026-09-28 17:10 UTC - da2ce7 - Corrected two statements found in the PR #2366 review: #2281 is closed, and the register holds 127 frictions plus two CI findings
- 2026-09-29 16:29 UTC - GitHub Copilot - Maintainer approved subissue #2375 (order 14), unambiguous issue specification directory names; created and linked it under this EPIC
- 2026-10-01 08:26 UTC - da2ce7 - Rebased over the maintainer's #2375 row and owner metadata; placed the two draft specifications from PRs #2388 and #2385; recorded the #2375 scheduling position
- 2026-10-04 19:18 UTC - da2ce7 - Recorded the outcomes of the goals-and-boundaries and specifications-and-rationale discussions (PR #2428, reviews 5400754664, 5402713128 and 5407197495) under Decisions Recorded on This EPIC; the semantic-linking outcome and the per-EPIC parts are recorded on #2264 and #2278
- 2026-10-05 12:40 UTC - da2ce7 - Followed the prose-tests draft's placement under this EPIC (PR #2437): the specifications-and-rationale decision links the draft at its new path, and Drafts Awaiting Issues lists it
- 2026-10-07 16:15 UTC - AI assistant (Copilot SDK in VS Code) - Maintainer approved subissue #2473 (order 15), asynchronous attributed discussion rounds proposed in PR #2467; created and linked it under this EPIC
- 2026-10-08 09:12 UTC - AI assistant (Copilot SDK in VS Code) - Subissue #2473 (order 15) closed by PR #2481; archived its specification to `docs/issues/closed/`
- 2026-10-08 16:50 UTC - da2ce7 - Triaged the six proposals of the PR #2484 review retrospective: items 1 to 3 go to #2278's improvement matrix, and items 4 to 6 with item 1's tree-wide checks to the friction register; see Friction Register
- 2026-10-09 10:25 UTC - da2ce7 - Answered review 5468071770 on PR #2366: the Friction Register describes the index's four comments at v31 and keeps the 2026-09-28 counts as a snapshot, and the order range is dropped from the checkpoint and the Delivery Strategy

## Acceptance Criteria

- [ ] AC1: A reviewed catalog identifies current automation and guardrails, their invocation sites, ownership, inputs/outputs, runtime tier, environment needs, and feedback behavior.
- [ ] AC2: An overlap and gap analysis shows which checks are duplicated, unique, manual-only, or already deterministic, including the guarantees enforced by `.github/workflows/testing.yaml` and existing `deny.toml` dependency enforcement.
- [ ] AC3: A candidate matrix distinguishes repetitive task automation from verification guardrails, records side effects explicitly, and separates mechanically enforceable rules from judgment-based guidance.
- [ ] AC4: Token/context impact claims use a documented measurement or estimation method and state limitations; the EPIC does not assume that deterministic checks are free.
- [ ] AC5: Multiple viable architecture options are compared against the same explicit criteria, including at least one incremental/distributed option and one consolidation option.
- [ ] AC6: Architecture-check research documents current enforcement, candidate gaps, and risks without requiring a framework, crate, binary, or prototype as an EPIC outcome.
- [ ] AC7: #1843, #1774, and #1768 each receive a documented disposition based on the analysis; #1586 is excluded as unrelated shutdown work.
- [ ] AC8: Maintainer review is recorded before a full design is selected or implementation subissues begin; unresolved evidence results in explicit research actions.
- [ ] AC9: Approved implementation subissues have ordered, independently verifiable specs; unapproved implementation ideas remain options rather than commitments.
- [ ] AC10: Each completed research/design subissue records automatic checks, manual review evidence, and a post-completion acceptance-criteria review.
- [ ] AC11: A required deterministic check verifies that `project-words.txt` follows its documented ordering rule and contains no duplicates, with mutation evidence for both failure modes.
- [ ] AC12: The selected automation contract is non-interactive and defines streaming JSONL/NDJSON events, stable exit codes, actionable diagnostics, progress reporting, and explicit side-effect and cache-result reporting.
- [ ] AC13: Reusable check results are keyed and invalidated by all relevant inputs, configuration, tool versions, and check version; cache hits are visible and cannot silently reuse stale results after representative mutations.
- [ ] AC14: The selected design defines distinct contracts for mutating actions, read-only checks, and orchestration policies while identifying the infrastructure they may safely share.
- [ ] AC15: The approved foundation and operations are implemented through independently verifiable subissues, including focused contract, failure, and invalidation tests.
- [ ] AC16: Local hooks, agent workflows, and CI consumers migrate progressively with documented parity or intentional differences, rollback evidence, and no premature removal of the old path.
- [ ] AC17: Rollout evidence records runtime and context/token effects, final ownership, residual risks, and removal of stale references and temporary compatibility paths.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Inventory artifact and maintainer review |
| AC2 | TODO | Overlap/gap map |
| AC3 | TODO | Candidate matrix |
| AC4 | TODO | Token/context measurement method and results |
| AC5 | TODO | Options paper and comparison matrix |
| AC6 | TODO | Architecture-check feasibility section |
| AC7 | TODO | Existing-issue disposition record |
| AC8 | TODO | Maintainer review record or decision log |
| AC9 | TODO | Approved follow-up specs and dependency order |
| AC10 | TODO | Subissue verification records |
| AC11 | TODO | Dictionary-integrity check and mutation-test evidence |
| AC12 | TODO | Automation interface contract and contract-test evidence |
| AC13 | TODO | Cache-key design, invalidation tests, and measured reuse evidence |
| AC14 | TODO | Action/check/policy contracts and side-effect review |
| AC15 | TODO | Implementation subissue tests and verification records |
| AC16 | TODO | Consumer migration, parity, and rollback evidence |
| AC17 | TODO | Rollout measurements, ownership map, and stale-path audit |

## Verification Plan

### Automatic Checks

- `linter markdown`
- `linter cspell`
- Validate referenced repository paths while producing and reviewing each research artifact.
- Run focused tests only for a bounded research utility or proof of concept approved later.

### Issue Body Mirror

The GitHub issue body mirrors this file and is re-derived on every change to it. To derive it, drop the frontmatter, the skill-link marker and the title line, and keep the rest byte for byte. A body edited by hand drifts: until this revision, the body mirrored an older spec that lacked six subissue rows, the child-EPIC scope and the #2347 log line.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Steps | Expected Result | Status | Evidence |
| --- | -------- | ----- | --------------- | ------ | -------- |
| M1 | Inventory traceability sample | Select representative local hook, CI, skill, and architecture-policy entries; trace each from source to invocation and output | Catalog entries match repository behavior, distinguish setup/actions from checks, and identify source of truth and duplication | TODO | |
| M2 | Objective-rule classification review | Review representative accepted and rejected automation candidates with maintainers | Mechanical rules have testable pass/fail semantics; judgment-based rules remain guidance with rationale | TODO | |
| M3 | Options comparison review | Walk maintainers through each option using the same decision criteria and evidence links | Trade-offs and unknowns are visible; no option receives unearned preference from the document structure | TODO | |
| M4 | Existing-issue disposition review | Compare #1843, #1774, and #1768 with the reviewed decision | Each issue is retained, re-scoped, split, or superseded with rationale; no unrelated issue is included | TODO | |
| M5 | Dictionary guardrail mutation | Introduce one out-of-order entry and one duplicate in isolated fixtures or temporary copies | The check rejects each mutation with the offending entries and recovery guidance, then passes the unchanged dictionary | TODO | |
| M6 | Check-result reuse invalidation | Repeat an unchanged check, then mutate each cache-key input in turn | Exact inputs produce a visible cache hit; every relevant mutation forces execution and cannot reuse a stale pass | TODO | |
| M7 | Agent interface exercise | Invoke representative success, failure, long-running, and cache-hit paths without a TTY | The process never prompts, streams valid JSONL/NDJSON events, exits predictably, and gives actionable failure data | TODO | |

## Risks and Assumptions

- Risk: inventory work becomes an unbounded catalog. Mitigation: record only artifacts that execute tasks, enforce rules, or materially instruct agent execution, and define completion by traced entry points rather than raw file count.
- Risk: consolidation is treated as inherently simpler. Mitigation: require a distributed, incremental baseline option and compare total ownership and migration cost.
- Risk: deterministic checks encode incomplete policy and create false confidence. Mitigation: document rule semantics, false-positive/false-negative risks, and keep judgment-based review.
- Risk: token savings are overstated or moved into tool execution cost. Mitigation: report the measurement boundary, assumptions, and both context and execution costs.
- Risk: result caching hides failures after relevant inputs change. Mitigation: use content-addressed keys over declared inputs and versions, expose cache decisions, and test invalidation with representative mutations.
- Risk: machine-readable output is technically valid but difficult for humans or agents to act on. Mitigation: define semantic event contracts and actionable fields, not only JSON syntax, and validate representative consumers.
- Risk: existing issue scopes conflict with the selected design. Mitigation: do not implement them through this EPIC until their dispositions are reviewed and recorded.
- Risk: a register disposition graded against text that predates the friction overstates adoption. The 2026-09-21 summary graded F4, F6 and F45 that way ([comment 5866514218](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5866514218)). Mitigation: recompute every disposition at a stated head with the landing commit found by `git log -S`, as the index does; the index records F4 as OPEN.
- Risk: late review findings go unnoticed; #2347's 32 findings were posted after merge and found by chance. Mitigation: undecided improvement candidate 1.
- Risk: the issue body drifts from this specification. Mitigation: the derivation rule under Issue Body Mirror.
- Risk: the thread outgrows its readers (46 comments, about 300,000 characters on 2026-09-28). Mitigation: this specification carries what the thread established; only the register index and the current ownership record stay visible, and minimized comments remain reachable by link.
- Assumption: maintainers prefer evidence and reversible incremental adoption over a mandatory repository-wide migration. Maintainer review may replace this assumption with an explicit constraint.

## References

- Existing candidate issues: #1843, #1774, #1768
- Unrelated shutdown issue excluded from this EPIC: #1586
- Child EPICs: #2264, #2278; sibling EPICs owning #2347 follow-ups: #1347 (#2301, #2361)
- Current local checks: `contrib/dev-tools/git/hooks/pre-commit.sh`, `contrib/dev-tools/git/hooks/pre-push.sh`
- Current CI checks: `.github/workflows/testing.yaml`
- Current dependency-policy enforcement: `deny.toml`, `docs/packages.md`
- Existing workspace analysis tool: `contrib/dev-tools/analysis/workspace-coupling/`
- Initial inventory: `docs/issues/open/2003-overhaul-guardrails-and-automation/initial-inventory.md`
- Previous candidate architecture: `docs/issues/open/2003-overhaul-guardrails-and-automation/previous-single-runner-proposal.md`
- Strict frontmatter profile: `docs/schemas/frontmatter-v1.schema.json`
- Friction register index: [part 1](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5847229228), [part 2](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5871784196)
- Ownership record: [hand-off](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5873655520), [acknowledgement](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5874065152)
- Design input for AC12–AC14: [comment 5444339176](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5444339176)
- Inventory measurements at `fa698c43` ([comment 5814006328](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5814006328)). No change was requested.
  - Skill links: 30 of 46 skills had no inbound link from another skill; they are reached through root `AGENTS.md`, the agent profiles or their descriptions.
  - Merge tool: 183 of 185 first-parent merges since the merge tool was vendored carry its subject form.
  - Open question for Proposed Subissue 1: does the inventory record how each skill is reached?
- Earlier digests of the thread, superseded by this specification and the index: [general review](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5767266486) (2026-09-21) and [register overhaul](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5847232340) (2026-09-26)
- Filing comments of the register: linked from the index rows
