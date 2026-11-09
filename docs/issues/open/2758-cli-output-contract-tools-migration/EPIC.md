---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2758
spec-path: docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-11-09 10:23"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md
    - docs/issues/drafts/cli-output-contract-migration/ISSUE.md
    - docs/issues/closed/2715-cli-output-contract-scope-and-exit-code-policy/ISSUE.md
---

<!-- skill-link: create-issue -->

# EPIC #2758 - Migrate the CLI Tools to the Global Output Contract

**Parent EPIC:** none. Related: #2003 (the AI and CI tools came from there), #2715 (the decision), #1798 (the contract).

## Goal

Bring every first-party program that a script, a CI job or an AI agent calls into line with the global CLI output
contract, as scoped by ADR `20261106102632`: JSON on stdout and stderr, no plain text, exit codes `0`/`1`/`2`
only, a distinct outcome reported as a `kind`, and TTY refusal for `stdout-result-data` commands.

## Why This Is Needed

The two ADRs decide what the contract covers and how exit codes work, but change no code. An audit of the tree
(2026-11-06, base `develop` plus PR #2748) found that many programs still break the contract, including some the
ADR table already calls compliant or does not list at all. The older migration draft
(`docs/issues/drafts/cli-output-contract-migration/ISSUE.md`, 2026-05-19, no GitHub issue) is out of date: the
workspace already denies `clippy::print_stdout` and `clippy::print_stderr`, most of its 46 `print!` sites are gone,
and it treats `contrib/dev-tools/` as out of scope, which the new ADR reverses. This EPIC supersedes it; the draft
is left in place as history (it is not edited, to keep older references valid); this EPIC replaces it.

### Audit result

Checked against `develop` at `35ec81000` plus the PR #2748 head.

| Program | Class (ADR table) | Finding |
| --- | --- | --- |
| `github-review-threads reply-status` | `stdout-result-data` | A thread without a reply is exit `1` plus a `missing_reply` diagnostic; it is an answer and must be a result with exit `0`. The skills `resolve-review-threads` and `fetch-review-threads` describe the exit `1`. |
| `package-coverage-check` | `stdout-result-data` | Plain-text errors, every failure exits `1` (usage included), no TTY refusal, `summary` prints Markdown. |
| `http_health_check` | `stdout-result-data` | Plain text on stdout (`Health check ...`, `STATUS: ...`), plain `eprintln!` usage, usage error exits `1`. It is the container `HEALTHCHECK`, so it needs exit `1` when unhealthy: reclassify as `no-stdout-result`. |
| `tracker_client` | `stdout-result-data` | No TTY refusal; `tracing` writes plain text to stdout; `check --format text` prints plain text; direct prints in the console layer. |
| `http_tracker_client`, `udp_tracker_client`, `tracker_checker` | not listed | Deprecated; the ADR says remove them. They still exist and print a plain-text warning. |
| `torrust-tracker` (daemon) | `no-stdout-result` | `tracing` writes plain text (ANSI, `full` style) to stdout; startup failure is a plain `eprintln!`. |
| `e2e_tests_runner`, `qbittorrent_e2e_runner` | `no-stdout-result` | Plain `tracing` output on stdout, failures via `anyhow` as plain text. |
| `agent-review-report-contract` | not listed | Plain-text success line on stdout and plain-text failures on stderr; not in the table. |
| `frontmatter-schema` | not listed | Plain-text diagnostics on stderr; not in the table. |
| `workspace-coupling` | not listed | NDJSON on stderr already; not in the table; needs a class and a check of where its report goes. |
| `clippy-allow-reasons`, `frontmatter-validator`, `shipped-features-check`, `ci-gate`, `wait-for-pr-review` | listed | Consistent. |
| Shell and Python scripts under `contrib/dev-tools/` and `.github/skills/` | not listed | Several are called by CI or agents and write plain text (for example `reply-to-thread.sh`). Not audited yet. |

## Maintainer Decisions (2026-11-06)

1. **No parent EPIC.** The work is half operator binaries, which do not fit #2003's goal. #2003 and #2715 are
   cross-referenced.
2. **`http_health_check` becomes `no-stdout-result`.** Pass/fail by exit code with NDJSON diagnostics on stderr,
   because the container `HEALTHCHECK` needs a non-zero exit when unhealthy. The ADR table is amended.
3. **The daemon's logging is migrated in this EPIC** (JSON on stderr by default), not deferred. It is a visible
   change for operators and is handled as its own issue with release notes.
4. **Scripts: audit only.** One issue audits and classifies them; this EPIC does not rewrite them.

## Scope

### In Scope

- Migrating the programs in the audit table above to the contract, one issue per tool or tool group.
- Aligning the ADR binary table with the audit (new rows, reclassification).
- The skills, workflows and docs that describe or consume a changed tool, in the same issue as the tool.
- An audit of the shell and Python scripts called by CI or agents.

### Out of Scope

- Developer-only diagnostics: `profiling`, benchmarks, examples, tests (ADR 20261106102632).
- Rewriting scripts (only an audit issue).
- A shared CLI infrastructure crate (the contract ADR defers it).
- Changing the exit-code policy or the scope; those are settled by the ADRs.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`. File sets are disjoint inside a wave, so a wave can run in
parallel; the maintainer sets how many sessions run at once.

| Order | Issue | Local Spec | Status | Files it touches / notes |
| ----- | ----- | ---------- | ------ | ------------------------ |
| 1 | #2760 - Align the ADR binary table with the audit | `docs/issues/open/2760-2758-adr-binary-table/ISSUE.md` | TODO | Wave 0, docs only: `docs/adrs/20260519000000_define_global_cli_output_contract.md`. Must land first; no other issue edits it. |
| 2 | #2761 - `github-review-threads reply-status`: a missing reply is a result | `docs/issues/closed/2761-2758-reply-status-result/ISSUE.md` | DONE | Implemented in PR #2775. Wave 1: `contrib/dev-tools/github/github-review-threads/**`, `.github/skills/dev/pr-reviews/resolve-review-threads/`, `.github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md` |
| 3 | #2762 - `package-coverage-check`: NDJSON errors, usage exit 2, TTY refusal, JSON `summary` | `docs/issues/open/2762-2758-package-coverage-check-contract/ISSUE.md` | TODO | Wave 1: `contrib/dev-tools/checks/package-coverage-check/**`, `.github/workflows/generate_coverage_pr.yaml` |
| 4 | #2763 - Make `agent-review-report-contract` and `frontmatter-schema` emit NDJSON; classify `workspace-coupling` | `docs/issues/open/2763-2758-dev-tool-checks-ndjson/ISSUE.md` | TODO | Wave 1: `contrib/dev-tools/checks/agent-review-report-contract/**`, `contrib/dev-tools/checks/frontmatter-validator/src/bin/frontmatter-schema.rs`, `contrib/dev-tools/analysis/workspace-coupling/**` |
| 5 | #2764 - `http_health_check`: NDJSON diagnostics, usage exit 2 | `docs/issues/open/2764-2758-http-health-check-no-stdout/ISSUE.md` | TODO | Wave 1: `src/bin/http_health_check.rs` and its tests; verify `Containerfile` `HEALTHCHECK` |
| 6 | #2765 - Remove the deprecated tracker client binaries | `docs/issues/open/2765-2758-remove-deprecated-tracker-clients/ISSUE.md` | TODO | Wave 1: `console/tracker-client/src/bin/{http_tracker_client,udp_tracker_client,tracker_checker}.rs`, `console/tracker-client/Cargo.toml`, README, tests, docs that name them |
| 7 | #2766 - `torrust-tracker` daemon: JSON logs on stderr and JSON startup failure | `docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md` | TODO | Wave 1: `packages/configuration/src/{logging.rs,v3_0_0/logging.rs}`, `src/main.rs`, `src/console/ci/e2e/logs_parser.rs`, default configs and docs. Release note: operator-visible change. |
| 8 | #2767 - Audit and classify the scripts called by CI and agents | `docs/issues/open/2767-2758-scripts-audit/ISSUE.md` | TODO | Wave 1: a new audit document only; no script changes |
| 9 | #2768 - `tracker_client`: TTY refusal, JSON-only output | `docs/issues/open/2768-2758-tracker-client-json-only/ISSUE.md` | TODO | Wave 2, after 6: `console/tracker-client/**` |
| 10 | #2769 - `e2e_tests_runner` and `qbittorrent_e2e_runner`: JSON diagnostics | `docs/issues/open/2769-2758-e2e-runners-json/ISSUE.md` | TODO | Wave 2, after 7: `src/console/ci/**`, `packages/e2e-tools/src/bin/` |
| 11 | #2770 - Close the migration: update the ADR notes and the EPIC | `docs/issues/open/2770-2758-close-out-adr-and-epic/ISSUE.md` | TODO | Last: ADR "currently non-compliant" notes, `docs/issues/` bookkeeping. The only close-out PR for the EPIC. |

## Delivery Strategy

Wave 0 first (the table the others cite). Wave 1 issues touch disjoint files and may run in parallel. Wave 2 waits
for its predecessor because it shares files with it (`console/tracker-client/**` with issue 6; the e2e log parser
with issue 7). Issue 11 runs last. Issues 2 to 5 and 9 are independent of the operator-visible changes.

For each subissue, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an issue-local retrospective for reusable
   lessons, material design changes, or meaningful deviations from the plan; otherwise record why one was
   unnecessary in the issue progress log.

Every issue that changes a tool also changes, in the same PR, each skill, workflow, script or doc that describes or
consumes the old behavior (named in its spec). No issue edits the ADR table except issue 1 and issue 11.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [x] Subissues created and linked in this spec (GitHub sub-issue links pending the maintainer: the bot cannot call the sub-issues API)
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 16:45 UTC - Claude Code (Claude Sonnet 5.5) - Drafted the EPIC after auditing `develop` plus PR #2748; the maintainer chose no parent, the `no-stdout-result` reclassification of `http_health_check`, a daemon logging migration in this EPIC, and an audit-only issue for scripts.
- 2026-11-06 17:25 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the EPIC and the 11 child specs; created EPIC #2758 and issues #2760 to #2770 as josecelano-bot. Linking them as sub-issues returned 404 for the bot, so the maintainer links them.

## Acceptance Criteria

- [ ] AC1: Every program in the audit table is compliant with the contract, or its remaining non-compliance is recorded in the ADR with a linked issue.
- [ ] AC2: The ADR binary table lists every in-scope program with the right class and no "currently non-compliant" note remains for a migrated tool.
- [ ] AC3: Every skill, workflow and doc that consumed a changed tool's old output or exit code is updated.
- [ ] AC4: The scripts audit exists and each script it flags has a decision (migrate, exempt, or follow-up issue).
- [ ] AC5: All required subissues are created, linked, and ordered with dependencies documented.
- [ ] AC6: Every completed subissue includes automated and manual verification evidence and an implementation completion review.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | |
| AC2 | TODO | |
| AC3 | TODO | |
| AC4 | TODO | |
| AC5 | TODO | |
| AC6 | TODO | |

## Risks and Trade-offs

- **The daemon's log format is operator-visible.** Existing deployments parse the plain text. Mitigation: a release
  note, the `trace_style` option keeps working, and the issue decides the migration detail with the maintainer before code.
- **Health check semantics.** The container `HEALTHCHECK` must keep failing on an unhealthy service. Mitigation: the
  `no-stdout-result` reclassification, and a manual verification that `docker inspect` reports `unhealthy`.
- **Review load.** Eleven issues is a lot for one reviewer. Mitigation: the maintainer sets the parallelism; waves are
  a limit, not a target.
- **ADR #2748 is not merged yet.** Specs cite its path. Mitigation: the spec PR is cut after it merges, or rebased onto it.

## References

- Related issues: #2003, #2715, #1798, #2581
- Related PRs: #2748 (the ADR)
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`,
  `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
- Superseded draft: `docs/issues/drafts/cli-output-contract-migration/ISSUE.md`
