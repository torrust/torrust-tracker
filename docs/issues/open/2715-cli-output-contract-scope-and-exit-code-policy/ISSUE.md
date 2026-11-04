---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: null
github-issue: 2715
spec-path: docs/issues/open/2715-cli-output-contract-scope-and-exit-code-policy/ISSUE.md
branch: "2712-wait-for-pr-review-bug-and-adr-specs-spec"
related-pr: null
last-updated-utc: "2026-11-04 13:09"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-adr/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/issues/closed/1798-global-cli-output-contract-adr/ISSUE.md
    - docs/issues/closed/2581-2003-pr-review-wait-poller/ISSUE.md
    - docs/issues/drafts/cli-output-contract-migration/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2715 - Decide the scope and the exit-code policy of the global CLI output contract

Not yet attached to an EPIC. EPIC #2003 (Overhaul: Automation Tools and AI Agent Guardrails) is a candidate,
because the tools in question are AI-agent tools, but the maintainer decides; this draft sets `epic: null`.

## Goal

Record, in an ADR, two decisions about `docs/adrs/20260519000000_define_global_cli_output_contract.md` that it
leaves open: which programs the contract covers, and how a program reports distinct outcomes through exit codes.

## Background

The global CLI output contract (ADR `20260519000000`) says every binary is assigned an output class, requires JSON
on stdout and stderr, and defines three exit codes: `0` success, `1` runtime failure, `2` usage error or TTY
refusal. Two questions came up while implementing the `wait-for-pr-review` poller (issue #2581) and are not
answered by the ADR.

**1. Scope.** The ADR was written for the tracker and its operator-facing binaries. Its own issue specification
(`docs/issues/closed/1798-global-cli-output-contract-adr/ISSUE.md`) lists "all first-party, operator-facing CLI
entrypoints" as in scope and "developer-only tooling (`contrib/dev-tools/`, benchmarks, examples, tests)" as out
of scope, and the ADR text excludes the `profiling` binary as "a developer-only diagnostic harness". Yet the ADR's
binary table now lists tools that live under `contrib/dev-tools/` or in a skill folder and serve the repository's
AI agents and CI, not tracker operators: `clippy-allow-reasons`, `frontmatter-validator`, `github-review-threads`,
`ci-gate` and `wait-for-pr-review`. The table and the scope statement disagree, and nothing says whether a new AI
or CI tool must follow the contract.

**2. Exit codes.** The ADR fixes `0`, `1` and `2`, and says tracker endpoint failures are results in the JSON
payload with exit `0`, not failures. It does not say what a program with several distinct non-success outcomes
should do. The existing tools answer differently:

- `wait-for-pr-review` reports a timeout as a result: exit `0` and `{"kind":"timeout"}` on stdout. A failure is
  exit `1` with an empty stdout, and a `kind` on stderr.
- `ci-gate` also reports a failing verdict as a result with exit `0` (its tests are named after exiting with zero
  and printing a failing verdict), and uses `1` and `2` for runtime and usage failures.
- `github-review-threads reply-status` exits `1` with a `missing_reply` diagnostic on stderr when a thread has no
  reply, which is an outcome the caller asked about, not a failure of the tool.

The Torrust Index ADR that this ADR follows (ADR-T-010) allows "command-specific non-usage exit codes" documented
by the owning command contract. The tracker ADR neither allows nor forbids them.

The trade-off is between two styles. Distinct numbers per outcome are the Unix and shell-script habit and let a
caller branch without parsing. A single failure code plus a `kind` field is easier for a program that already
parses JSON to compose, and keeps the numbers from being redefined by every command. The maintainer's first
impression (2026-10-28) was that for AI agents one failure code plus a `kind` is more composable, while a distinct
code is better for bash scripting. That impression is a starting point for the decision, not the decision.

An older draft that has no GitHub issue yet, `docs/issues/drafts/cli-output-contract-migration/ISSUE.md`, plans to migrate
the remaining `print!` usages to the contract. It also treats `contrib/dev-tools/` as out of scope, so the scope
decision here changes which of its rows apply.

The `wait-for-pr-review` spec records that it follows the ADR as written (a timeout is a result), and asked for
these two questions to be tracked separately, because deciding them was not part of building the poller.

## Scope

### In Scope

- Decide whether the contract covers AI-agent and CI tools under `contrib/dev-tools/` and in skill folders, and
  say so in the ADR, including what happens to the existing table rows.
- Decide the exit-code policy: whether a command may define its own non-usage codes, or whether outcomes beyond
  success, runtime failure and usage error are reported through a `kind` field with the existing codes.
- Record both decisions in an ADR (a new ADR that amends the existing one, or an amendment, as the `create-adr`
  skill and the maintainer decide) and update the ADR index.
- List the follow-up work the decision implies for the existing tools, as separate issues.

### Out of Scope

- Changing any binary. If the decision invalidates a tool's current behavior, that is a follow-up issue.
- The tracker server's own output contract, which the ADR already covers.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`.
- ADRs to create: one ADR (or an amendment) recording the two decisions. Root collection, `docs/adrs/`, because the
  decision is repository-wide and affects several packages and tools.

## Design and Ownership Review

Not applicable: a documentation and decision task, with no code, child process or resource change.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                  | Notes / Expected Output                                                   |
| --- | ------ | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| T1  | TODO   | Inventory how each tool in the ADR table reports outcomes and exit codes               | A short table in the issue folder: tool, outcomes, exit codes, class.     |
| T2  | TODO   | Gather the options and trade-offs for each question, with the maintainer              | Options recorded with their consequences for existing tools.              |
| T3  | TODO   | Write the ADR with the `create-adr` skill and register it in the index                 | ADR merged; the existing ADR table and scope text agree with it.          |
| T4  | TODO   | Open follow-up issues for tools whose behavior the decision changes                    | One issue per tool, or a recorded reason there is none.                   |

## Commit Points

| Task | Coherent change set                                  | Commit policy                                        |
| ---- | ---------------------------------------------------- | ---------------------------------------------------- |
| T1   | The inventory                                        | Commit after focused validation.                     |
| T3   | The ADR and the index entry                          | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2715-cli-output-contract-scope-and-exit-code-policy/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created (#2715) and issue number added to this spec
- [ ] Decisions recorded in an ADR
- [ ] Automatic verification completed (`linter all`)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-04 12:30 UTC - AI assistant (Copilot SDK in VS Code) - Drafted the specification from the open
  questions recorded while implementing #2581, and checked the ADR text, its issue specification and the exit-code
  behavior of the listed tools.
- 2026-11-04 12:12 UTC - AI assistant (Copilot SDK in VS Code) - The maintainer approved the specification; GitHub issue #2715 created, labelled `task` and `ADR`, not attached to an EPIC (the maintainer decides); the specification moved from `docs/issues/drafts/` to `docs/issues/open/` for a spec-only pull request.
- 2026-11-04 13:07 UTC - AI assistant (Copilot SDK in VS Code) - Correction: the drafting entry above (12:30) was stamped before the commands it
  describes ran. The drafting and the lint commands finished by 11:56 UTC, as the
  command output records, so the drafting stamp is wrong. The earlier entries are kept unchanged,
  and `last-updated-utc` now holds the time of this edit.
- 2026-11-04 13:09 UTC - AI assistant (Copilot SDK in VS Code) - The Evidence column of the manual scenarios M1 and M2
  now reads `TBD`: this pull request adds no `manual-verification-evidence.md` for this specification, and the file is
  written when the scenarios run.

## Acceptance Criteria

- [ ] AC1: An ADR states which programs the contract covers, and the table in the existing ADR agrees with it.
- [ ] AC2: An ADR states the exit-code policy and the rule for outcomes beyond success, runtime failure and usage
      error, with the reasons and the alternatives considered.
- [ ] AC3: Every tool in the ADR table is either consistent with the decisions or has a follow-up issue.
- [ ] `linter all` exits with code `0`
- [ ] The ADR index lists the new ADR

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo run -p frontmatter-validator --bin frontmatter-validator -- <changed files>`

### Manual Verification Scenarios

| ID | Scenario                                        | Human-oriented command/steps                                                                                          | Expected Result                                                     | Status | Evidence                                     |
| -- | ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1 | The ADR text and the table agree                | Read the scope statement and compare it with every row of the binary table                                            | No row contradicts the stated scope                                 | TODO   | TBD, written when the scenario runs      |
| M2 | The listed tools match the decision             | Run each tool in the table with no arguments and with a failing input; record the exit code and the stderr kind        | Each behaves as the ADR says, or has a follow-up issue              | TODO   | TBD, written when the scenario runs      |

Every recorded command result names the toolchain that produced it.

### Disposable Verification Scripts

None.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |

## Risks and Trade-offs

- Narrowing the scope would make the ADR table list tools it no longer governs; widening it would make every new
  tool subject to the contract. The ADR must say which.
- A single failure code plus a `kind` makes callers parse JSON to branch; distinct codes make the numbers a
  per-command convention that agents and scripts must learn. The ADR records the reasoning either way.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a concise progress-log entry explaining why.

## References

- Related issues: #2581 (closed, the poller that raised the questions), #1798 (closed, the original ADR issue)
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
