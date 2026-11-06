---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/issues/open/2715-cli-output-contract-scope-and-exit-code-policy/ISSUE.md
---

<!-- skill-link: create-adr -->

# Scope the CLI Output Contract and Keep Exit Codes to 0, 1 and 2

## Scope

Repository-level decision: it governs several packages and tools (the tracker, the operator binaries, the CI
tools under `contrib/dev-tools/` and the AI-agent tools in skill folders), so it lives in the root collection,
`docs/adrs/`. It amends [Define the Global CLI Output Contract](20260519000000_define_global_cli_output_contract.md).

## Description

The global CLI output contract ADR leaves two questions open.

**Scope.** Its issue specification (#1798) puts developer tooling under `contrib/dev-tools/` out of scope, and the
ADR excludes `profiling` as a developer-only harness. Yet its binary table lists `clippy-allow-reasons`,
`frontmatter-validator`, `shipped-features-check`, `github-review-threads`, `ci-gate` and `wait-for-pr-review`,
which live under `contrib/dev-tools/` or in a skill folder and serve CI and AI agents, not tracker operators. The
table and the scope statement disagree, and nothing says whether a new CI or AI tool must follow the contract.

**Exit codes.** The ADR fixes `0`, `1` and `2` but not what a program with several non-success outcomes does. The
existing tools answer differently: `wait-for-pr-review` reports a timeout as a result (exit `0`), `ci-gate` reports a
failing verdict as a result (exit `0`), and `github-review-threads reply-status` exits `1` with a `missing_reply`
diagnostic for an outcome the caller asked about. The Torrust Index ADR-T-010, which the contract follows, allows
command-specific non-usage exit codes; the tracker ADR neither allows nor forbids them.

## Agreement

### 1. The contract covers every first-party program that machines call

The contract applies to:

- the tracker's operator binaries, and
- the AI-agent and CI tools, wherever they live: `contrib/dev-tools/`, a skill folder, or a workspace package.

It does not apply to developer-only diagnostics: `profiling`, benchmarks, examples and tests. A new program is
covered if a script, a CI job or an AI agent consumes its output or its exit code; it must be added to the binary
table when it is introduced.

The binary table in the global ADR gains one row, for `package-coverage-check`, which the coverage workflow
(`.github/workflows/generate_coverage_pr.yaml`) calls and whose output it consumes. It is a `stdout-result-data`
tool: `matrix` and `compare` write a result for the workflow, and `summary` writes a report for a pull-request
comment. It is not compliant today: its errors are plain text on stderr, and every failure, including a usage
error, exits `1`. Its scope sentence, which excludes `contrib/dev-tools/`, is corrected by the pointer note added to
that ADR.

### 2. Exit codes stay `0`, `1` and `2`; a distinct outcome is a `kind`

No command defines its own non-usage exit code. Commands use only:

| Code | Meaning                                                                    |
| ---- | -------------------------------------------------------------------------- |
| 0    | The command did its job. The answer, whatever it is, is in the result.     |
| 1    | The command could not do its job (runtime or internal failure).            |
| 2    | Usage error: invalid arguments or configuration, or TTY refusal.           |

A distinct outcome is reported with a `kind` field, in one of two places, chosen by one rule:

- **The outcome is an answer to what the caller asked.** A timeout while waiting, a failing merge verdict, a thread
  without a reply: the command worked and found that answer. For a `stdout-result-data` command it is a result on
  stdout, with a `kind` (or equivalent field) and exit `0`.
- **The command could not produce an answer.** A failed `gh` call, an interrupted run, an exhausted budget with no
  answer to give: stdout is empty, stderr carries a diagnostic with a `kind`, and the exit code is `1` (or `2` for
  usage).

A `no-stdout-result` command has no result channel, so every failure that is not a usage error is exit `1`, with the
findings as NDJSON diagnostics on stderr. Usage errors keep exit `2`, as in the table above. This covers checks whose purpose is to fail on a finding
(`clippy-allow-reasons`, `frontmatter-validator`, `shipped-features-check`) and the CI orchestrators.

A caller branches on the exit code to learn whether the command worked, and on `kind` to learn which outcome it
got. Bash callers that need the distinction pipe stdout to `jq -e` or read the `kind`.

### 3. Existing tools

- `wait-for-pr-review`, `ci-gate`: consistent. A timeout and a failing verdict are results with exit `0`.
- `github-review-threads reply-status`: not consistent. The missing reply is an answer, so it should be a result with
  exit `0`; follow-up issue.
- `package-coverage-check`: not consistent. Errors must be JSON, usage errors must exit `2`, and TTY refusal
  applies; follow-up issue (task T4 of #2715). Whether `summary`, which renders a report for humans, must emit JSON
  is part of that follow-up.
- `clippy-allow-reasons`, `frontmatter-validator`, `shipped-features-check`: consistent as `no-stdout-result`
  checks.

Changing any tool is out of scope for this ADR. The changes are listed as task T4 of
[#2715](https://github.com/torrust/torrust-tracker/issues/2715) until their follow-up issues exist; this ADR will not
name those issues.

## Alternatives Considered

**Narrow the scope to operator-facing binaries.** Rejected: the AI-agent and CI tools are exactly the machine-driven
programs the contract exists for, and leaving them out would give agents an unregulated surface. It would also
remove table rows that are accurate today.

**Two tiers: the full contract for operators, a lighter profile for dev tools.** Rejected as premature. Every
requirement of the contract except TTY refusal is cheap, a second profile doubles what contributors must learn, and
no tool has yet asked for the exemption. It can be added later by amending this ADR.

**Command-specific exit codes (as in ADR-T-010).** Rejected. Distinct numbers let a bash script branch without
parsing, but the callers here are mostly programs that already parse JSON, and each command would redefine the
numbers (is `3` a timeout or a failing verdict?). One failure code plus a `kind` composes better and keeps `0`, `1`
and `2` meaning the same everywhere.

**A reserved extra code for a negative verdict.** Rejected. It is the same per-command redefinition on a smaller
scale, and the "answer in the result with exit `0`" rule already covers a verdict.

**Edit the original ADR in place instead of adding a new one.** Rejected. An ADR records a decision at a date, and
these decisions came later. A new ADR keeps the original reasoning and shows when and why the scope and the exit-code
rule were settled; the original gets a short pointer so that it does not mislead.

## Consequences

### Positive

- The binary table and the scope statement no longer disagree, and a new CI or AI tool knows it must comply.
- Callers learn one exit-code meaning and one place to look for the outcome.

### Negative

- A bash script that wants to react to a verdict must read JSON; it cannot test `$?` alone.
- Two existing tools must change (`github-review-threads reply-status` and `package-coverage-check`); until their
  follow-ups land, the non-compliance is documented here and in the binary table of the amended ADR.

## Date

2026-11-06

## References

- Issue: [#2715](https://github.com/torrust/torrust-tracker/issues/2715), spec
  `docs/issues/open/2715-cli-output-contract-scope-and-exit-code-policy/ISSUE.md`
- Amended ADR: [Define the Global CLI Output Contract](20260519000000_define_global_cli_output_contract.md)
- Original issue: #1798; poller that raised the questions: #2581
- Torrust Index ADR-T-010:
  <https://github.com/torrust/torrust-index/blob/develop/adr/010-global-command-line-output-contract.md>
