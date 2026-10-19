---
semantic-links:
  skill-links:
    - create-adr
    - implement-workflow
  related-artifacts:
    - docs/issues/open/2525-ci-gate-required-check/ISSUE.md
    - docs/github-rulesets.md
    - docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md
    - .github/workflows/docs-lint.yaml
    - .github/workflows/testing.yaml
    - contrib/dev-tools/checks/package-coverage-check/Cargo.toml
    - "issue #2525"
---

<!-- skill-link: create-adr -->

# Keep CI Decision Logic in Rust Behind Thin, Provider-Agnostic Workflows

## Scope

Root ADR. It governs where the decisions that CI makes about a pull request live: which job groups
a change needs, and whether the pull request may merge. Those decisions apply to every package and
every workflow in the repository, and no package owns them.

## Description

The project's CI decisions are written as GitHub Actions YAML. Which workflows run for a change is
decided by `paths` and `paths-ignore` filters, repeated in `testing.yaml`, `os-compatibility.yaml`,
`container.yaml`, `generate_coverage_pr.yaml`, `db-compatibility.yaml`, and `db-benchmarking.yaml`,
and mirrored by hand in the header of `docs-lint.yaml`. Whether a pull request may merge is decided
by GitHub's required status checks, which compare check names. Both kinds of decision are hard to
test, hard to read in one place, and tied to one provider.

Issue #2507 showed the cost. The only required-checks ruleset could not be satisfied: six of its 16
required check names had disappeared when jobs were renamed, and a documentation-only pull request
reports only `Docs Lint`, because the other workflows are skipped by `paths-ignore`. A skipped
workflow reports nothing, and a required check that never reports blocks the merge forever. The
maintainer disabled the ruleset around every merge, and it stayed disabled. The sandbox in #2507
also established that GitHub enforces required checks on the maintainer merge tool's direct push.
A single check that reports on every pull request would therefore protect every merge, but nothing
in YAML makes such a check easy to build or to keep correct.

Two further pressures make the placement of this logic worth a decision:

- **Performance.** Rebasing and agent-driven pull requests multiplied CI runs (see the ADR to stop
  requiring pull-request rebases). A decision that can inspect the changed paths can also run less,
  but a path filter in YAML can only match a glob list and cannot be tested.
- **Portability.** GitHub's infrastructure cost and reliability have worsened while pull requests
  multiplied, so moving to another CI provider or to self-hosted CI should stay feasible. Decision
  logic embedded in one provider's YAML must be rewritten for the new provider, and its behavior
  re-derived.

The repository already treats this kind of choice as a policy: use shell for simple orchestration and
Rust once logic is non-trivial, stateful, safety-critical, or worth testing independently
(`AGENTS.md`, "Shell vs Rust threshold"). It also already has a Rust developer tool of this shape,
`package-coverage-check`, run from a workflow.

## Agreement

Decision logic that CI applies to a pull request lives in Rust tools, with inputs and outputs that do
not depend on a CI provider. Workflows stay thin: they schedule jobs, pass data to and from those
tools, and report the result.

Concretely:

- **Inputs and outputs are provider-neutral.** A tool takes revisions or paths, structured job
  results (JSON), and an optional override with its actor. Its output follows the
  [global CLI output contract](20260519000000_define_global_cli_output_contract.md): one JSON result
  object on stdout, NDJSON records on stderr, and the contract's exit codes, with no plain text on
  either channel. The human-readable explanation of a result goes in a field of the JSON object. It
  does not read GitHub-specific environment variables, and it does not call the GitHub API. Anything
  that needs a provider query (the checks API, label events, role lookups) goes through a thin
  adapter, so the rule that uses the answer stays testable without the provider.
- **Change detection is a tool.** Which job groups a change needs (documentation, Rust build and
  tests, container, databases, security scan) is decided by one Rust tool that replaces the
  workflow-level `paths` and `paths-ignore` filters. It first reproduces today's filters exactly,
  with a table-driven test per rule, and may then become smarter about what a change needs.
- **The merge verdict is a tool.** Whether a pull request may merge is decided by a Rust tool from
  the needed job groups, each group's result, and an optional maintainer override. It passes only
  when every needed group succeeded or the override applies, and otherwise names each failing,
  cancelled, or missing group in the explanation field of its JSON result. The contract treats a
  failed command as empty stdout and exit code 1, and a result as a JSON object with exit code 0, so
  how a failing verdict reaches the job's status (the exit code, or a field the workflow reads) is a
  choice for the task that writes the verdict tool, made under that contract.
- **Workflows only wire.** A workflow may run a tool, pass it job results, set a job's status from its
  exit code, and consume a scheduling decision that a tool computed: a job's `if:` or a matrix that
  reads an earlier tool job's output, as `generate_coverage_pr.yaml` does with the
  `package-coverage-check` tool. It must not decide which job groups a change needs, evaluate the
  results of earlier jobs, or compose a verdict itself.
- **One always-reporting required check.** GitHub is asked to require one check, `CI gate`, which
  reports on every pull request, whether it changes code or only documentation. The tool behind it
  decides its outcome. The issue that implements the gate (#2525) defines the override and the other
  open design questions, after sandbox experiments.

### How to judge whether new logic belongs in YAML

Keep a rule in a workflow only when all of these hold:

- The platform needs it to schedule the job (an event trigger, a matrix, a runner label, a
  permission, an artifact hand-off), or it only reads a scheduling decision that a tool already
  computed (for example, running a job only when the classifier tool's output lists its group).
- It has no branches of its own on the content of the change or on earlier results.
- A reader needs no test to trust it.

If a rule itself depends on which paths changed, on the results of other jobs, or on who did
something, it is decision logic and belongs in a tool, where a unit test can cover it. Reading the
tool's answer is wiring; producing the answer is not.

### What this ADR does not decide

- How a maintainer overrides the gate without disabling a ruleset, where the gate reads earlier job
  results and label provenance, and whether an override survives a new push. These depend on how
  GitHub behaves, and issue #2525 settles them with sandbox experiments before the design relies on
  them.
- The package layout and name of the tools, and their dependencies.
- Moving away from GitHub Actions. This ADR keeps that feasible; it does not start it.

### Alternatives Considered

- **Keep the logic in YAML.** The usual pattern is a `CI gate` job that lists its dependencies with
  `needs:` and runs `if: always()`, with path filters in each workflow. It needs no new code, but
  the rules cannot be unit tested, they live in six workflow files, and they would have to be
  rewritten on another provider. A path filter on the gate's own dependencies also reproduces the
  skipped-workflow problem that disabled the ruleset.
- **Use a third-party action for path filtering or for the gate.** It removes the code but adds a
  dependency that sits in the trust boundary of a required check, binds the logic to GitHub, and
  still cannot be tested here. The repository restricts allowed actions for the same reason.
- **Write the logic as shell scripts.** Shell is portable between providers, but this logic is
  stateful and safety-critical: a wrong verdict either blocks every merge or lets a broken one
  through. That is the case the shell-versus-Rust threshold assigns to Rust.
- **Leave required checks off and rely on the maintainer.** This has been the effective state since
  2026-05-05. It depends on a person checking CI before every merge, which does not scale with the
  number of pull requests.

### Consequences

Positive:

- Classification and verdict rules have unit tests, and one place to read them.
- A change can later be classified more precisely, so a pull request runs only the workflows it
  needs.
- Replacing the CI provider means rewriting thin workflows, not the decisions.
- A required check that always reports ends the cycle of disabling a ruleset to merge.

Negative, with the mitigation:

- **More code to maintain.** The tools are small and live next to the existing developer tools.
- **Extra time on every pull request.** The tool has to be built or fetched before it runs, which
  matters most for documentation-only pull requests. Issue #2525 measures this cost in a sandbox and
  gets maintainer approval before the gate is introduced.
- **A broken gate blocks every pull request.** The gate is introduced as a reporting check first,
  observed on real pull requests, and required only afterwards, with a documented one-step rollback.
  The verdict logic is unit tested.

## Affected Code

- [`.github/workflows/docs-lint.yaml`](../../.github/workflows/docs-lint.yaml) and the `paths` and
  `paths-ignore` filters of the other pull-request workflows, which the change-detection tool
  replaces.
- [`contrib/dev-tools/checks/package-coverage-check/`](../../contrib/dev-tools/checks/package-coverage-check/Cargo.toml),
  an existing Rust tool called from a workflow, next to which the new tools are expected.
- [`docs/github-rulesets.md`](../github-rulesets.md), which lists the active rulesets and gains the
  `CI gate` rule when it is required.

## Date

2026-10-16

## References

- Issue #2525: [specification](../issues/open/2525-ci-gate-required-check/ISSUE.md)
- Issue #2507: the sandbox evidence that required checks are enforced on the maintainer merge push
  and that a skipped workflow blocks a required check forever
- Issue #1743: the documentation-only CI fast path
- [`20261013161419_stop_requiring_pull_request_rebases.md`](20261013161419_stop_requiring_pull_request_rebases.md):
  required checks must not require up-to-date branches
- [`20260519000000_define_global_cli_output_contract.md`](20260519000000_define_global_cli_output_contract.md):
  the output contract the tools follow
- [`20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`](20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md)
- `AGENTS.md`, "Shell vs Rust threshold"
