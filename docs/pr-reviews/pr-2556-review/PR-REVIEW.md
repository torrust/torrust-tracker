---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/adrs/20261016154414_keep_ci_decision_logic_in_rust_behind_thin_provider_agnostic_workflows.md
    - docs/issues/open/2525-ci-gate-required-check/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2556 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2556>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

One review was submitted. Copilot's review 5550796171 carries two inline threads, F1 and F2. Its
overview repeats them, so the body adds no rows. It read the head whose subject is
"docs(issues): [#2525] mark T1 done and link the CI-portability ADR".

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2556-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2556-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The ADR's rule prohibits necessary scheduling wiring

- PR number: 2556
- Source review ID: 5550796171
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2556#discussion_r4296963117>
- Concern: the rule that workflows must not evaluate conditions beyond what the platform needs would
  also forbid a job's `if:` or matrix reading an earlier classifier job's output, which
  `generate_coverage_pr.yaml` already does and which the planned orchestrator needs to run only the
  needed job groups. Allow consuming Rust-computed scheduling decisions while keeping classification
  and verdict policy out of YAML.
- Solution: the "Workflows only wire" bullet allows consuming a scheduling decision that a tool
  computed, and the "How to judge" test says that reading the tool's answer is wiring and producing it
  is not. Classifying changes, evaluating the results of earlier jobs, and composing a verdict stay
  out of YAML.
- Current-tree verification: `.github/workflows/generate_coverage_pr.yaml` has
  `if: ${{ fromJSON(needs.discover-package-coverage.outputs.discovery).matrix.include[0] != null }}`
  and `matrix: ${{ fromJSON(needs.discover-package-coverage.outputs.discovery).matrix }}` on the
  `package-coverage-regression` job, the pattern the ADR now allows. Both passages of the ADR say so.
- Resolution reference: docs(adrs): [#2525] allow workflows to consume Rust-computed scheduling decisions
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2556#discussion_r4315890452>

### F2 - Plain-text output conflicts with the global CLI output contract

- PR number: 2556
- Source review ID: 5550796171
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2556#discussion_r4296963188>
- Concern: the ADR said a tool "prints text or JSON", but
  `docs/adrs/20260519000000_define_global_cli_output_contract.md` requires JSON results and
  newline-delimited JSON diagnostics, with no plain text, for new commands from their introduction.
  Reference the contract and put readable explanations in JSON fields, so T2 and T3 do not receive
  contradictory output requirements.
- Solution: the ADR links the contract, says the tools' output follows it, and puts the explanation
  of a result in a JSON field. Because the contract maps a failed command to empty stdout and exit
  code 1, and a result to exit code 0, the ADR leaves to the verdict task how a failing verdict
  reaches the job status (the exit code, or a field the workflow reads) under those rules.
- Current-tree verification: the contract requires one JSON object on stdout, NDJSON records on
  stderr, and "No plain text on either channel, at any verbosity level". The ADR's output bullet,
  verdict bullet, references, and related artifacts name the contract.
- Resolution reference: docs(adrs): [#2525] align the tools' output with the global CLI output contract
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2556#discussion_r4315890698>

## Processing Log

- 2026-10-19 15:47 UTC - Fetched the review and its two threads, classified and numbered the findings,
  checked both against the current tree, and fixed F1 and F2 in separate signed commits.
- 2026-10-19 15:48 UTC - Ran the pre-push checks, pushed, and, after the push, replied on both
  threads.
- 2026-10-19 15:49 UTC - Started this record, with a reply URL in every row.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
