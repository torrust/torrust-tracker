---
doc-type: epic
issue-type: task
status: planned
github-issue: 2003
spec-path: docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
epic-owner: josecelano
last-updated-utc: 2026-08-17
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/
    - .github/agents/
    - .github/workflows/
    - .github/workflows/testing.yaml
    - .githooks/
    - contrib/dev-tools/git/hooks/
    - contrib/dev-tools/git/install-git-hooks.sh
    - contrib/dev-tools/analysis/workspace-coupling/
    - deny.toml
    - project-words.txt
    - AGENTS.md
    - docs/templates/EPIC.md
    - docs/issues/open/1843-migrate-git-hooks-scripts-from-bash-to-rust.md
    - docs/issues/open/1774-automate-cleanup-completed-issues-skill-script.md
    - docs/issues/open/1768-refactor-update-dependencies-skill-automation.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/initial-inventory.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/previous-single-runner-proposal.md
---

<!-- skill-link: create-issue -->

# EPIC #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Research, design, implement, and progressively adopt repository automation and AI-agent
guardrails that make repetitive tasks deterministic where practical and give humans and agents
deterministic, timely feedback about whether work satisfies repository rules.

The EPIC starts with discovery, research, and comparison of alternatives. It does not select a
tool shape, implementation language, crate layout, or single execution model in advance. After
maintainers record the design decision, the EPIC continues through implementation, progressive
migration, and removal of superseded paths.

## Previous Design Discussion

An earlier discussion explored consolidating repository guardrails into one extensible Rust
runner. That proposal introduced independently implemented guardrails, policy-based execution,
dependency resolution, shared repository context, standardized results, and identical local and
CI entry points.

The discussion is preserved in
[`previous-single-runner-proposal.md`](previous-single-runner-proposal.md) as design input. It is
not the selected architecture. Its assumptions and trade-offs must be evaluated alongside
distributed and incremental alternatives during this EPIC.

## Why This Is Needed

Repository checks and procedures have grown across several independently maintained surfaces:

- `pre-commit.sh` and `pre-push.sh` contain duplicated step-runner, logging, argument-parsing,
  and output-format logic around different check lists.
- `.github/workflows/testing.yaml` is an existing composite guardrail. It repeats some local
  checks and also enforces broader guarantees through formatting, linters, documentation tests,
  workspace tests across targets and features, Cargo layer-boundary bans, container image
  builds, tracker E2E tests, and qBittorrent E2E tests against SQLite, MySQL, and PostgreSQL.
- Skills and agent instructions describe repeatable workflows and objective rules, but rules
  expressed only as instructions still depend on an agent interpreting and following them.
- Some architecture rules are already deterministic through `cargo deny check bans`, while
  other possible repository-policy checks remain manual or have not been evaluated.
- Existing automation proposals choose different script locations, interfaces, and
  implementation approaches without a shared repository-wide decision framework.

This distribution is not inherently wrong. The problem is that the repository lacks a current
inventory showing ownership, overlap, execution cost, feedback behavior, and which rules should
remain guidance versus become deterministic applications or tests. Without that evidence, a
large consolidation could replace working checks with a more complex system without proving a
benefit.

## Design Principles

The research and options analysis must apply these principles:

1. **Maximize determinism**: when a workflow step or rule has objective inputs and pass/fail
   semantics, prefer an executable application, test, or linter over instructions asking an
   agent to reproduce the procedure. Keep skills and agent instructions for orchestration,
   judgment, and context that cannot be encoded reliably.
2. **Minimize inference-token and execution waste**: reduce instructions that only restate
   deterministic behavior, and avoid repeating expensive checks when an equivalent successful
   result can be reused safely. Any cache must key results by the exact relevant inputs,
   configuration, tool versions, and check version. Pre-commit checks may need staged-tree
   identity, while pre-push and CI checks may use commit or tree identity; branch name or commit
   identity alone is not always sufficient.
3. **Design for AI agents and humans**: automation must be non-interactive, composable,
   idempotent where practical, and explicit about side effects. Commands must provide stable
   exit codes, actionable diagnostics, and streaming machine-readable events using JSON Lines
   (JSONL/NDJSON) in accordance with the repository CLI output contract. Human-readable
   presentation may be layered over the same event model.
4. **Preserve local and CI parity**: checks should have one authoritative implementation that
   can be invoked consistently by developers, agents, hooks, and CI, even when those entry
   points select different check profiles.
5. **Fail safely and explain recovery**: cached results, skipped checks, partial failures, and
   destructive operations must be visible and auditable. Automation must state why a result was
   reused or invalidated and what action is required after failure.

## Tooling Taxonomy

Automation actions and guardrail checks are related but are not equivalent. The EPIC must model
their different safety and result contracts while assessing which infrastructure they can share.

| Type       | Purpose                                                                                             | Side effects                                                       | Typical result                              |
| ---------- | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ------------------------------------------- |
| **Action** | Perform repository work, such as updating dependencies or moving issue specs                        | Expected; dry-run/apply and idempotency safeguards may be required | Changed, unchanged, skipped, failed         |
| **Check**  | Evaluate whether repository work satisfies an objective rule                                        | Read-only by default                                               | Passed, warning, skipped, failed            |
| **Policy** | Select and order actions/checks for a context such as pre-commit, pre-push, CI, nightly, or release | Inherits the selected operations' effects                          | Aggregate execution result and event stream |

Actions and checks may share configuration loading, repository discovery, Git/Cargo metadata,
dependency planning, caching infrastructure, JSONL/NDJSON events, diagnostics, and progress
reporting. They must not share a contract that hides whether an operation mutates state.

Workflows and hooks can themselves be **composite guardrails** when they orchestrate multiple
checks into one merge or lifecycle gate. In particular, `.github/workflows/testing.yaml` is a
current composite CI guardrail even though its implementation also performs setup and container
build actions needed by its checks.

## Scope

### In Scope

- Catalog current automation and guardrails across local hooks, CI workflows, skills, custom
  agents, repository instructions, linters, dependency checks, and reusable analysis tools.
- Validate and maintain the initial baseline in
  [`initial-inventory.md`](initial-inventory.md), including explicit unknowns rather than
  treating the first pass as complete evidence.
- Trace each check or workflow by purpose, owner/source of truth, invocation sites, inputs,
  outputs, runtime tier, environment requirements, duplication, and failure feedback.
- Distinguish repetitive task automation from verification guardrails; both are relevant, but
  they require different side-effect, result, retry, and cache contracts even if they share an
  execution framework.
- Treat hooks and CI workflows as composite guardrails where they aggregate checks, and inventory
  their setup/action steps separately from the guarantees they enforce.
- Identify objective skill and instruction rules that could be enforced mechanically, while
  retaining human judgment where a deterministic rule would be brittle or incomplete.
- Assess the likely context and inference-token effect of replacing selected instructions with
  executable checks, using a documented measurement or estimation method rather than assuming
  savings.
- Inventory repeated checks and design safe result reuse based on exact input identity so hooks
  and agents do not rerun unchanged work unnecessarily.
- Research multiple implementation and execution models. Options must include retaining
  distributed tools with clearer contracts as well as one or more consolidation approaches.
- Compare options using explicit criteria: correctness, feedback latency, local/CI parity,
  testability, maintainability, portability, incremental adoption, failure modes, developer and
  agent usability, runtime cost, and migration risk.
- Evaluate check placement across pre-commit, pre-push, CI, and any future repository-policy or
  architecture-check category without assuming that every check belongs in one runner.
- Explore future architecture-check candidates, including dictionary ordering and dependency
  policy. Document existing coverage from Cargo and `deny.toml`, remaining gaps, false-positive
  risk, and whether a new category is justified; do not select a framework prematurely.
- Add a required deterministic check that verifies `project-words.txt` uses one documented
  ordering rule and contains no duplicate entries. Decide its package and execution tier through
  the EPIC design rather than coupling it to the tracker library.
- Permit the narrowly scoped interim formatter described by
  [`2019-automatically-format-project-dictionary/ISSUE.md`](../../closed/2019-automatically-format-project-dictionary/ISSUE.md).
  It supplies immediate developer feedback but does not select the EPIC's long-term architecture,
  execution tier, or check/action contract, and may be replaced or refactored after the design
  decision.
- Re-evaluate #1843, #1774, and #1768 against the resulting evidence and recommend whether each
  should proceed unchanged, be re-scoped, be split, or be superseded.
- Present the evidence and options for maintainer review before selecting a full design.
- Define implementation subissues from the approved design, including ownership, dependency
  order, migration boundaries, compatibility periods, and independent verification.
- Implement the approved action, check, policy, output, and result-reuse capabilities through
  those subissues, including the dictionary-integrity check.
- Migrate local, agent, and CI consumers progressively; remove superseded implementations and
  instructions only after parity and rollback evidence is recorded.

### Out of Scope

- Implementing, migrating, or consolidating automation tools or checks before the design decision
  and implementation subissues are approved, except for the explicitly approved interim project
  dictionary formatter linked in Scope.
- Prescribing a `workspace-tools` crate, a single Rust binary, Bash scripts, a task runner, or
  any other tool shape before alternatives are compared and reviewed.
- Prototyping architecture tests before the research identifies a question that requires a
  bounded proof of concept and maintainers approve that follow-up.
- Changing the CI/CD provider, release process, or the external `torrust-linting` project.
- Treating every agent instruction as suitable for deterministic enforcement.
- Shutdown and runtime task-ownership work. Issue #1586 belongs with shutdown EPIC #1488 and is
  unrelated to repository automation or agent guardrails.

## Known Existing Issues

These issues are paused dependencies of the EPIC. Their current implementation choices are
proposals to re-evaluate, not constraints on the EPIC design. Implementation must not resume
until the architecture decision records whether each issue proceeds, is re-scoped or split, or
is superseded.

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue                                                 | Local Spec                                                                | Status  | Relationship                                                                                                            |
| ----- | ----------------------------------------------------- | ------------------------------------------------------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------- |
| 1     | #1843 - Migrate git hooks scripts from Bash to Rust   | `docs/issues/open/1843-migrate-git-hooks-scripts-from-bash-to-rust.md`    | BLOCKED | Pause implementation; runner shape, contracts, check ownership, and migration depend on the design decision             |
| 2     | #1774 - Automate cleanup of completed issue specs     | `docs/issues/open/1774-automate-cleanup-completed-issues-skill-script.md` | BLOCKED | Pause implementation; action placement, dry-run/apply, GitHub access, and output contract depend on the design decision |
| 3     | #1768 - Refactor update-dependencies skill automation | `docs/issues/open/1768-refactor-update-dependencies-skill-automation.md`  | BLOCKED | Pause implementation; action decomposition, shared infrastructure, and validation policy depend on the design decision  |

## Proposed Research and Design Subissues

These are proposed planning subissues. Titles and boundaries may be adjusted during maintainer
review; no GitHub issues should be created from this draft without approval.

| Order | Proposed Issue                                           | Intent                                                                                                         | Expected Output                                                                                                                                                     | Verification                                                                                                                                           | Dependencies                                    |
| ----- | -------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------- |
| 1     | Inventory repository automation and guardrails           | Validate and complete the initial current-system evidence baseline                                             | Reviewed revision of `initial-inventory.md` and invocation/overlap map covering local, CI, skill, agent, and architecture-policy surfaces                           | Sample entries traced end to end; catalog cross-checked against repository entry points and reviewed for omissions                                     | Initial inventory in this EPIC                  |
| 2     | Assess deterministic automation and guardrail candidates | Separate objective rules that are candidates for automation from judgment-based guidance and estimate benefits | Candidate matrix with determinism, current failure mode, proposed enforcement point, expected benefit, token-impact method, cost, and false-positive risk           | Representative skill and instruction rules reviewed by maintainers; rejected candidates retain rationale                                               | Subissue 1                                      |
| 3     | Enforce project dictionary integrity                     | Add the known required guardrail without coupling it to the tracker library                                    | Deterministic test or check proving `project-words.txt` is sorted by a documented rule and has no duplicates; selected execution tier and actionable failure output | Positive test plus mutations for out-of-order and duplicate entries; invocation verified through the selected local and CI profiles                    | Subissue 2 for placement and interface decision |
| 4     | Research safe check-result reuse                         | Avoid rerunning equivalent successful pre-commit, pre-push, and agent checks                                   | Cache-key model, invalidation rules, audit record, threat/failure analysis, and measured savings for representative workflows                                       | Mutations to staged content, commit/tree, configuration, tool version, and check version invalidate stale results; exact matches reuse results visibly | Subissues 1 and 2                               |
| 5     | Define agent-friendly automation contracts               | Standardize non-interactive execution and machine-readable feedback                                            | Contract for JSONL/NDJSON events, stable exit codes, diagnostics, progress/heartbeat, side-effect reporting, and idempotent retries                                 | Fixture or contract tests cover success, failure, progress, cache hit/miss, and malformed invocation                                                   | Subissues 1 and 2                               |
| 6     | Research and compare architecture options                | Evaluate viable organization, execution, feedback, and migration models without preselecting a tool            | Options paper with diagrams, decision criteria, trade-offs, migration paths, and bounded proof-of-concept recommendations where evidence is insufficient            | Every criterion and design principle applied consistently to each viable option; claims linked to inventory evidence or experiments                    | Subissues 1, 2, 4, and 5                        |
| 7     | Record maintainer decision and implementation roadmap    | Convert the reviewed options into an explicit decision or documented request for more evidence                 | Decision record, disposition of #1843/#1774/#1768, ordered implementation scope, migration plan, and implementation-ready specs                                     | Maintainer review recorded; roadmap items trace to selected option and include independent verification criteria                                       | Subissues 3 and 6                               |
| 8     | Implement approved automation foundation                 | Build only the shared contracts and infrastructure justified by the decision                                   | Tested implementation of the approved operation model, event and exit-code contracts, configuration, and any selected planning or result-reuse infrastructure       | Contract, unit, integration, failure, and invalidation tests pass; implementation maps to the decision record without speculative framework features   | Subissues 4, 5, and 7                           |
| 9     | Implement and migrate approved operations                | Deliver the approved actions and checks, then move consumers without losing current guarantees                 | Dictionary-integrity guardrail plus approved #1843/#1774/#1768 scopes; local, agent, and CI migration; superseded-path removal                                      | Old/new parity or intentional-difference evidence, rollback exercise, consumer migration audit, and selected local/CI policies pass                    | Subissue 8                                      |
| 10    | Validate rollout and close the EPIC                      | Prove the resulting system is usable, maintainable, and no longer depends on superseded paths                  | Runtime and token-impact results, final ownership map, operating documentation, residual-risk record, and closure dispositions                                      | Representative human and agent workflows pass; required CI guarantees remain enforced; stale references and temporary compatibility paths are removed  | Subissue 9                                      |

## Delivery Strategy

Use an evidence-first, progressive delivery strategy because the problem crosses repository
workflows, developer tooling, and agent behavior, while the desired architecture is
intentionally unsettled. Discovery and candidate analysis can gather evidence independently,
but architecture selection and implementation must wait until both are complete.

Research artifacts should be committed as durable documentation under the EPIC or an approved
canonical docs location. Implementation subissues begin only after maintainers review the
alternatives and record a decision. The decision may keep the current distributed model,
approve only targeted improvements, select consolidation, or request a bounded experiment
before committing to the remaining implementation.

For each completed subissue in this EPIC, the default completion policy is:

1. Run applicable automatic checks (`linter markdown`, `linter cspell`, and any tests for
   research utilities or prototypes explicitly approved later).
2. Run the defined manual review scenarios and record evidence.
3. Re-review the subissue and EPIC acceptance criteria against the produced evidence.

### Phase 1: Discovery

- Outcome: a validated inventory and overlap map of the current automation and guardrail system.
- Exit criteria: maintainers can trace what runs, where it runs, what it enforces, and where
  duplication, redundant execution, feedback gaps, or manual-only rules exist. The initial
  inventory is reviewed, corrected, and accepted as the baseline for later comparisons.

### Phase 2: Candidate and Options Analysis

- Outcome: a ranked candidate matrix and comparison of multiple viable architectures.
- Exit criteria: alternatives use common evaluation criteria, identify unresolved evidence,
  and avoid assuming a single binary, crate, language, or check category.

### Phase 3: Maintainer Decision and Implementation Planning

- Outcome: maintainers select an option, choose targeted changes, request further research, or
  explicitly retain the current structure.
- Exit criteria: the decision and rationale are recorded; existing issues have dispositions;
  only approved implementation work has implementation-ready specs and ordering.

### Phase 4: Foundation Implementation

- Outcome: the minimal shared contracts and infrastructure selected by the decision are
  implemented and tested without migrating all consumers at once.
- Exit criteria: operation contracts, machine-readable events, failure behavior, and any
  approved cache or planning behavior pass focused tests; rollback remains possible.

### Phase 5: Operation Implementation and Progressive Migration

- Outcome: approved actions and checks are delivered, including dictionary integrity, and local,
  agent, and CI consumers move to the selected interfaces in reviewable increments.
- Exit criteria: each migration preserves or intentionally revises documented guarantees;
  superseded paths are removed only after parity, failure, and rollback evidence is accepted.

### Phase 6: Rollout Validation and Closure

- Outcome: the delivered system has final ownership, usage, performance, and maintenance
  evidence, with paused issues closed, re-scoped, or completed according to the decision.
- Exit criteria: representative human and agent workflows pass; required CI guarantees remain;
  stale references and temporary compatibility paths are removed; residual risks are recorded.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [ ] Research/design subissues approved, created, and linked in this spec
- [ ] Initial inventory reviewed and accepted as the Phase 1 baseline
- [ ] Phase 1 discovery evidence reviewed
- [ ] Phase 2 candidate and options analysis reviewed
- [ ] Phase 3 maintainer decision recorded
- [ ] Phase 4 foundation implementation completed and verified
- [ ] Phase 5 operation implementation and progressive migration completed
- [ ] Phase 6 rollout validation completed
- [ ] Existing issue dispositions recorded for #1843, #1774, and #1768
- [ ] Subissue statuses kept up to date in the relevant tables
- [ ] For each completed subissue: automatic checks completed and recorded
- [ ] For each completed subissue: manual verification completed and recorded
- [ ] For each completed subissue: acceptance criteria reviewed post-completion
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-20 00:00 UTC - Copilot - Initial epic draft
- 2026-07-20 00:00 UTC - Planner - Refined draft to make discovery and options analysis the
  immediate scope; removed unrelated and unsupported references; separated existing issues from
  proposed research and design work - draft updated
- 2026-07-20 00:00 UTC - Copilot - Converted the draft to a folder-type EPIC and recorded an
  earlier single-runner design discussion as a non-binding supporting artifact
- 2026-07-20 00:00 UTC - Copilot - Distinguished mutating automation actions from read-only
  guardrail checks and documented the testing workflow as an existing composite CI guardrail
- 2026-07-20 00:00 UTC - Copilot - Added the initial repository inventory, paused existing
  implementation issues pending the design decision, and extended the EPIC through implementation,
  progressive migration, rollout validation, and closure
- 2026-07-20 00:00 UTC - josecelano - Approved the draft EPIC and its supporting artifacts
- 2026-07-20 00:00 UTC - GitHub Operator - Created EPIC #2003 and moved the approved local
  specification to `docs/issues/open/2003-overhaul-guardrails-and-automation/`
- 2026-07-22 00:00 UTC - josecelano - Approved a narrowly scoped interim project dictionary
  formatter; it may be replaced or refactored after the EPIC design decision

## Acceptance Criteria

- [ ] AC1: A reviewed catalog identifies current automation and guardrails, their invocation
      sites, ownership, inputs/outputs, runtime tier, environment needs, and feedback behavior.
- [ ] AC2: An overlap and gap analysis shows which checks are duplicated, unique, manual-only,
      or already deterministic, including the guarantees enforced by `.github/workflows/testing.yaml`
      and existing `deny.toml` dependency enforcement.
- [ ] AC3: A candidate matrix distinguishes repetitive task automation from verification
      guardrails, records side effects explicitly, and separates mechanically enforceable rules
      from judgment-based guidance.
- [ ] AC4: Token/context impact claims use a documented measurement or estimation method and
      state limitations; the EPIC does not assume that deterministic checks are free.
- [ ] AC5: Multiple viable architecture options are compared against the same explicit criteria,
      including at least one incremental/distributed option and one consolidation option.
- [ ] AC6: Architecture-check research documents current enforcement, candidate gaps, and risks
      without requiring a framework, crate, binary, or prototype as an EPIC outcome.
- [ ] AC7: #1843, #1774, and #1768 each receive a documented disposition based on the analysis;
      #1586 is excluded as unrelated shutdown work.
- [ ] AC8: Maintainer review is recorded before a full design is selected or implementation
      subissues begin; unresolved evidence results in explicit research actions.
- [ ] AC9: Approved implementation subissues have ordered, independently verifiable specs;
      unapproved implementation ideas remain options rather than commitments.
- [ ] AC10: Each completed research/design subissue records automatic checks, manual review
      evidence, and a post-completion acceptance-criteria review.
- [ ] AC11: A required deterministic check verifies that `project-words.txt` follows its
      documented ordering rule and contains no duplicates, with mutation evidence for both
      failure modes.
- [ ] AC12: The selected automation contract is non-interactive and defines streaming
      JSONL/NDJSON events, stable exit codes, actionable diagnostics, progress reporting, and
      explicit side-effect and cache-result reporting.
- [ ] AC13: Reusable check results are keyed and invalidated by all relevant inputs,
      configuration, tool versions, and check version; cache hits are visible and cannot silently
      reuse stale results after representative mutations.
- [ ] AC14: The selected design defines distinct contracts for mutating actions, read-only
      checks, and orchestration policies while identifying the infrastructure they may safely
      share.
- [ ] AC15: The approved foundation and operations are implemented through independently
      verifiable subissues, including focused contract, failure, and invalidation tests.
- [ ] AC16: Local hooks, agent workflows, and CI consumers migrate progressively with documented
      parity or intentional differences, rollback evidence, and no premature removal of the old path.
- [ ] AC17: Rollout evidence records runtime and context/token effects, final ownership, residual
      risks, and removal of stale references and temporary compatibility paths.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                          |
| ----- | ---------------------- | ----------------------------------------------------------------- |
| AC1   | TODO                   | Inventory artifact and maintainer review                          |
| AC2   | TODO                   | Overlap/gap map                                                   |
| AC3   | TODO                   | Candidate matrix                                                  |
| AC4   | TODO                   | Token/context measurement method and results                      |
| AC5   | TODO                   | Options paper and comparison matrix                               |
| AC6   | TODO                   | Architecture-check feasibility section                            |
| AC7   | TODO                   | Existing-issue disposition record                                 |
| AC8   | TODO                   | Maintainer review record or decision log                          |
| AC9   | TODO                   | Approved follow-up specs and dependency order                     |
| AC10  | TODO                   | Subissue verification records                                     |
| AC11  | TODO                   | Dictionary-integrity check and mutation-test evidence             |
| AC12  | TODO                   | Automation interface contract and contract-test evidence          |
| AC13  | TODO                   | Cache-key design, invalidation tests, and measured reuse evidence |
| AC14  | TODO                   | Action/check/policy contracts and side-effect review              |
| AC15  | TODO                   | Implementation subissue tests and verification records            |
| AC16  | TODO                   | Consumer migration, parity, and rollback evidence                 |
| AC17  | TODO                   | Rollout measurements, ownership map, and stale-path audit         |

## Verification Plan

### Automatic Checks

- `linter markdown`
- `linter cspell`
- Validate referenced repository paths while producing and reviewing each research artifact.
- Run focused tests only for a bounded research utility or proof of concept approved later.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                             | Steps                                                                                                                         | Expected Result                                                                                                                | Status | Evidence |
| --- | ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ | ------ | -------- |
| M1  | Inventory traceability sample        | Select representative local hook, CI, skill, and architecture-policy entries; trace each from source to invocation and output | Catalog entries match repository behavior, distinguish setup/actions from checks, and identify source of truth and duplication | TODO   |          |
| M2  | Objective-rule classification review | Review representative accepted and rejected automation candidates with maintainers                                            | Mechanical rules have testable pass/fail semantics; judgment-based rules remain guidance with rationale                        | TODO   |          |
| M3  | Options comparison review            | Walk maintainers through each option using the same decision criteria and evidence links                                      | Trade-offs and unknowns are visible; no option receives unearned preference from the document structure                        | TODO   |          |
| M4  | Existing-issue disposition review    | Compare #1843, #1774, and #1768 with the reviewed decision                                                                    | Each issue is retained, re-scoped, split, or superseded with rationale; no unrelated issue is included                         | TODO   |          |
| M5  | Dictionary guardrail mutation        | Introduce one out-of-order entry and one duplicate in isolated fixtures or temporary copies                                   | The check rejects each mutation with the offending entries and recovery guidance, then passes the unchanged dictionary         | TODO   |          |
| M6  | Check-result reuse invalidation      | Repeat an unchanged check, then mutate each cache-key input in turn                                                           | Exact inputs produce a visible cache hit; every relevant mutation forces execution and cannot reuse a stale pass               | TODO   |          |
| M7  | Agent interface exercise             | Invoke representative success, failure, long-running, and cache-hit paths without a TTY                                       | The process never prompts, streams valid JSONL/NDJSON events, exits predictably, and gives actionable failure data             | TODO   |          |

## Risks and Assumptions

- Risk: inventory work becomes an unbounded catalog. Mitigation: record only artifacts that
  execute tasks, enforce rules, or materially instruct agent execution, and define completion by
  traced entry points rather than raw file count.
- Risk: consolidation is treated as inherently simpler. Mitigation: require a distributed,
  incremental baseline option and compare total ownership and migration cost.
- Risk: deterministic checks encode incomplete policy and create false confidence. Mitigation:
  document rule semantics, false-positive/false-negative risks, and keep judgment-based review.
- Risk: token savings are overstated or moved into tool execution cost. Mitigation: report the
  measurement boundary, assumptions, and both context and execution costs.
- Risk: result caching hides failures after relevant inputs change. Mitigation: use
  content-addressed keys over declared inputs and versions, expose cache decisions, and test
  invalidation with representative mutations.
- Risk: machine-readable output is technically valid but difficult for humans or agents to act
  on. Mitigation: define semantic event contracts and actionable fields, not only JSON syntax,
  and validate representative consumers.
- Risk: existing issue scopes conflict with the selected design. Mitigation: do not implement
  them through this EPIC until their dispositions are reviewed and recorded.
- Assumption: maintainers prefer evidence and reversible incremental adoption over a mandatory
  repository-wide migration. Maintainer review may replace this assumption with an explicit
  constraint.

## References

- Existing candidate issues: #1843, #1774, #1768
- Unrelated shutdown issue excluded from this EPIC: #1586
- Current local checks: `contrib/dev-tools/git/hooks/pre-commit.sh`,
  `contrib/dev-tools/git/hooks/pre-push.sh`
- Current CI checks: `.github/workflows/testing.yaml`
- Current dependency-policy enforcement: `deny.toml`, `docs/packages.md`
- Existing workspace analysis tool: `contrib/dev-tools/analysis/workspace-coupling/`
- Initial inventory: `docs/issues/open/2003-overhaul-guardrails-and-automation/initial-inventory.md`
- Previous candidate architecture:
  `docs/issues/open/2003-overhaul-guardrails-and-automation/previous-single-runner-proposal.md`
