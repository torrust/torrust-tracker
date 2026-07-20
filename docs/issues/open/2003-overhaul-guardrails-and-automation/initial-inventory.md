# Initial Repository Automation and Guardrail Inventory

## Status and Purpose

This is the EPIC's initial evidence baseline, created before the dedicated inventory subissue.
It records observed repository entry points and known drift so the subissue starts from a
reviewable artifact rather than an empty catalog. It is intentionally incomplete: runtime
measurements, owners, exact trigger coverage, output samples, and end-to-end traces still require
validation.

This document describes the current system. It does not select the future architecture, assign
operations to a unified runner, or approve implementation from the paused issues.

## Classification

| Classification         | Meaning                                                                              |
| ---------------------- | ------------------------------------------------------------------------------------ |
| Action                 | Intentionally changes repository, Git, external service, or published artifact state |
| Check                  | Evaluates an objective condition and should be read-only except for caches and logs  |
| Policy                 | Selects and orders operations for an execution context                               |
| Composite guardrail    | Lifecycle or merge gate composed from multiple checks and required setup/actions     |
| Guidance/orchestration | Human or agent instructions that select tools, add judgment, or define handoffs      |
| Setup/infrastructure   | Prepares an environment or artifact needed by another action or check                |

## Runtime Tiers

These tiers are qualitative until Phase 1 records measurements on representative warm and cold
environments.

| Tier | Current interpretation                                                    |
| ---- | ------------------------------------------------------------------------- |
| T0   | Seconds; metadata, file, or focused documentation checks                  |
| T1   | Roughly one minute; local lint, dependency, and documentation-test gates  |
| T2   | Several minutes; full builds, tests, compatibility matrices, or coverage  |
| T3   | Tens of minutes; container builds, E2E suites, publication, or benchmarks |

## Local Git Entry Points

| Artifact / command                           | Class                        | Invocation and current behavior                                                                                                       | Output / side effects                                                                                              | Tier | Source of truth / notes                                                                    |
| -------------------------------------------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ | ---- | ------------------------------------------------------------------------------------------ |
| `.githooks/pre-commit`                       | Policy / composite guardrail | Installed Git hook; selects text for TTY stdout and JSON otherwise, then delegates to the pre-commit script                           | Inherits script logs and exit code; dispatcher itself is read-only                                                 | T1   | Dispatcher policy is here; operation list is in the script                                 |
| `contrib/dev-tools/git/hooks/pre-commit.sh`  | Policy / composite guardrail | Runs `cargo machete --with-metadata`, `cargo deny check bans`, `linter all`, and workspace doc tests; fail-fast                       | Text or one JSON document; creates per-step logs in `TORRUST_GIT_HOOKS_LOG_DIR`; JSON is buffered until completion | T1   | Authoritative current local step list; duplicated runner/reporting framework with pre-push |
| `.githooks/pre-push`                         | Policy / composite guardrail | Installed Git hook; selects text for TTY stdout and JSON otherwise, then delegates to the pre-push script                             | Inherits script logs and exit code                                                                                 | T2   | Dispatcher policy is here; operation list is in the script                                 |
| `contrib/dev-tools/git/hooks/pre-push.sh`    | Policy / composite guardrail | Runs nightly format/check/doc and full stable workspace tests; intentionally excludes pre-commit and E2E checks                       | Text or one JSON document; creates per-step logs; fail-fast                                                        | T2   | Authoritative current local step list; assumes pre-commit ran for every pushed commit      |
| `contrib/dev-tools/git/install-git-hooks.sh` | Action                       | Manually or during Copilot setup; copies every `.githooks/*` file into the active Git hooks directory and sets executable permissions | Mutates `.git/hooks`; plain text; no dry-run                                                                       | T0   | Installation behavior lives in script; copied hooks can become stale until reinstalled     |
| `contrib/dev-tools/git/check-git-hooks.sh`   | Check                        | Agent skills use it before manual validation to avoid running an installed hook suite twice                                           | Reports installation state; expected read-only                                                                     | T0   | Needs output and exit-code contract validation during Phase 1                              |

## Primitive Checks and Analysis Tools

| Artifact / command                                  | Class         | Guarantee or purpose                                                                               | Invocation points                                                    | Output / side effects                                                | Tier  | Source of truth / gaps                                                                                                 |
| --------------------------------------------------- | ------------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | -------------------------------------------------------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------- |
| `linter all` and focused `linter <name>`            | Check adapter | Delegates to Clippy, rustfmt, markdownlint, cspell, yamllint, Taplo, and ShellCheck                | Pre-commit, Testing CI, Docs Lint CI, skills, agents, and direct use | Tool-dependent text; tools may create caches or install dependencies | T0-T2 | External `torrust-linting` binary plus repository tool configs; no repository-owned shared event contract              |
| `cargo machete --with-metadata`                     | Check         | Finds unused Cargo dependencies                                                                    | Pre-commit; described in several skills and agent policies           | Cargo tool output; metadata/cache effects only                       | T1    | Not observed in Testing CI; local gate currently owns it                                                               |
| `cargo deny check bans` with `deny.toml`            | Check         | Enforces configured dependency/layer bans                                                          | Pre-commit and Testing CI `layer-bans` job                           | Cargo tool output; read-only except caches                           | T0-T1 | Deterministic architecture policy; local and CI invocations duplicate the same primitive                               |
| Cargo format, check, test, doc, build, and coverage | Check family  | Compiler, formatting, test, documentation, successful build, and coverage guarantees               | Hooks, CI workflows, skills, agents, and direct package validation   | Cargo/tool text and build artifacts under `target/`                  | T1-T3 | Flags and toolchains differ by policy; exact equivalence must not be assumed                                           |
| E2E runner binaries                                 | Check family  | Tracker behavior and qBittorrent interoperability, including SQLite, MySQL, and PostgreSQL paths   | Testing and Container workflows                                      | JSON-compatible repository CLI output plus containers and logs       | T3    | Require built image, container engine, ports, and database services; overlap is conditionally suppressed in Testing CI |
| `contrib/dev-tools/analysis/workspace-coupling/`    | Analysis tool | Scans workspace package dependencies and imported paths to produce coupling evidence               | Manual architecture analysis; generated reports under issue folders  | Produces reports; reads Cargo/source metadata                        | T1    | A reusable Rust tool, but not currently a mandatory guardrail; known text-scan limitations are documented in reports   |
| `project-words.txt` ordering and uniqueness         | Manual rule   | Dictionary entries are expected to be alphabetized; duplicate behavior is not mechanically guarded | Human/agent instructions and review                                  | No current deterministic result                                      | T0    | Required future check is a separate EPIC subissue; ordering semantics must be documented before implementation         |

## GitHub Workflow Inventory

Each workflow is a policy or composite entry point. Setup steps are not themselves evidence that
the guarded property passed.

| Workflow                    | Class                        | Trigger / guarantee summary                                                                                                             | Side effects and outputs                                                           | Tier  | Overlap / initial observations                                                                                                  |
| --------------------------- | ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------------------- |
| `testing.yaml`              | Composite guardrail          | Non-doc pushes/PRs; stable/nightly linters and tests, nightly formatting, doc tests, layer bans, conditional container and database E2E | Builds artifacts/images, starts containers, emits GitHub logs/statuses             | T2-T3 | Repeats local lint/doc/bans and full-test primitives; condition avoids selected overlap with `container.yaml`                   |
| `docs-lint.yaml`            | Composite guardrail          | Every push/PR; focused Markdown and spelling checks; provides the required signal for docs-only changes                                 | Installs linter and emits statuses                                                 | T0-T1 | Deliberately overlaps `linter all`; path-policy comments must remain synchronized across workflows                              |
| `container.yaml`            | Composite guardrail / action | Relevant pushes/PRs test a built image and qBittorrent database matrix; protected-branch paths also publish development/release images  | Builds, loads, logs into registry, and may publish container images                | T3    | E2E overlap with Testing is managed by event conditions; combines mutating publication actions with checks                      |
| `coverage.yaml`             | Check / reporting policy     | Branch coverage run using nightly LLVM tooling                                                                                          | Generates and uploads coverage artifacts/reports                                   | T2-T3 | Related logic also exists in PR coverage generation and upload workflows                                                        |
| `generate_coverage_pr.yaml` | Check / reporting policy     | Pull-request coverage generation                                                                                                        | Produces coverage and metadata artifacts                                           | T2-T3 | Paired with `upload_coverage_pr.yaml`; split trust/permission boundary needs tracing                                            |
| `upload_coverage_pr.yaml`   | Action                       | Consumes completed PR coverage workflow output                                                                                          | Writes coverage report content and PR/issue-facing state with elevated permissions | T0-T1 | Mutating second half of PR coverage flow; must remain distinct from the coverage check                                          |
| `db-compatibility.yaml`     | Composite guardrail          | Persistence-relevant changes; tracker-core tests against MySQL 8.0/8.4 and PostgreSQL 14-17                                             | Starts test containers/services and emits statuses                                 | T2    | Broader than the E2E database-driver matrix in version coverage; narrower in package/path scope                                 |
| `db-benchmarking.yaml`      | Benchmark policy             | Persistence-relevant changes run small SQLite, MySQL, and PostgreSQL benchmark scenarios                                                | Starts services and produces benchmark output                                      | T2-T3 | Performance signal semantics and whether regressions block are not yet cataloged                                                |
| `os-compatibility.yaml`     | Composite guardrail          | Non-doc pushes/PRs build stable and nightly on Linux, macOS, and Windows                                                                | Build artifacts/caches and GitHub statuses                                         | T2    | Unique cross-OS guarantee; overlaps Linux builds elsewhere                                                                      |
| `security-scan.yaml`        | Reporting guardrail          | Container changes, protected branches, daily schedule, and manual runs scan an image with Trivy                                         | Pulls/builds image; uploads SARIF; Trivy steps explicitly use exit code 0          | T2-T3 | Visibility and GitHub Security reporting, not a direct vulnerability-failing job; enforcement ownership is external to the step |
| `deployment.yaml`           | Composite release policy     | Tracker release branches run full workspace tests before publication                                                                    | Publishes tracker release artifacts/state                                          | T2-T3 | Repeats full tests as a release prerequisite                                                                                    |
| `deployment-packages.yaml`  | Composite release policy     | Package release paths identify, test, and publish a selected crate                                                                      | Publishes package artifacts and external registry state                            | T2    | Package-scoped test guarantee; parsing and publication are mutating actions                                                     |
| `copilot-setup-steps.yml`   | Setup / smoke-check policy   | Changes to setup/hook files and manual dispatch build workspace, install tools/hooks, and smoke-check all linters                       | Installs tools and mutates checkout `.git/hooks`; emits status                     | T2    | Validates Copilot environment setup, not product behavior; references only a subset of files whose changes can affect hooks     |
| `labels.yaml`               | Action                       | Manual or label-config changes export and synchronize GitHub labels                                                                     | Mutates repository files or GitHub labels, depending on job                        | T0    | External-service automation; outside code guardrails but relevant to the shared action contract                                 |

## Skills, Agents, and Repository Guidance

| Surface                                                  | Class                  | Current role                                                                                                  | Deterministic dependency / observed gap                                                                                                       |
| -------------------------------------------------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `AGENTS.md`                                              | Policy guidance        | Defines mandatory quality gates, Git workflow, engineering policy, and skill entry points                     | Must be interpreted by agents; currently summarizes local gates and should link rather than duplicate changing procedures                     |
| `run-pre-commit-checks` and `run-pre-push-checks` skills | Orchestration guidance | Explain installation, duplicate-run avoidance, commands, tiers, output modes, and troubleshooting             | Depend on hook scripts and `check-git-hooks.sh`; pre-commit skill omits the script's current `cargo deny` step and uses older machete wording |
| `run-linters` and `install-linter` skills                | Orchestration / setup  | Explain focused and aggregate linter use and external tool installation                                       | Depend on external linter behavior; duplicate tool/config lists also summarized in `AGENTS.md`                                                |
| `setup-dev-environment` skill                            | Setup policy           | Builds workspace, creates storage, installs tools/hooks, runs smoke tests, and verifies tests                 | Mutates machine/working tree state; manual multi-command procedure overlaps Copilot setup workflow                                            |
| `update-dependencies` skill                              | Action guidance        | Prescribes branch-first dependency updates, classification, validation, and commit preparation                | Mostly manual; #1768 proposes scripts but is paused pending shared design                                                                     |
| `cleanup-completed-issues` skill                         | Action guidance        | Prescribes issue-state validation and moving completed specs                                                  | Manual GitHub/repository mutation; #1774 proposes a non-interactive script but is paused pending shared design                                |
| Planning, testing, review, and maintenance skills        | Guidance/policies      | Encode document creation, tests, reviews, security triage, dependency changes, and other repeatable workflows | Mix objective commands with judgment; candidate analysis must avoid converting subjective review into brittle checks                          |
| Implementer agent                                        | Agent policy           | Requires focused tests, complexity audit after steps, task review, then commit delegation                     | Coordinates Complexity Auditor, Task Reviewer, and Committer; repeats hook command/output details                                             |
| Committer agent                                          | Agent policy           | Checks hook installation, runs or relies on pre-commit, reviews staged scope, and creates signed commits      | Relies on script/skill correctness; duplicate-run avoidance is procedural                                                                     |
| Complexity Auditor and Task Reviewer agents              | Review policies        | Evaluate changed-function complexity and acceptance-criteria completion                                       | Judgment-heavy outputs; not equivalent to deterministic repository checks                                                                     |
| Other specialized agents                                 | Role policies          | Clippy repair, PR review, research, planning, and GitHub operations                                           | Select tools and make judgments; inventory subissue must trace only rules that materially execute or gate work                                |

## Current Invocation and Ownership Map

```text
git commit -> installed .githooks/pre-commit -> pre-commit.sh
  -> machete + deny bans + linter all + doc tests

git push -> installed .githooks/pre-push -> pre-push.sh
  -> nightly fmt/check/doc + stable full tests

push / pull request -> GitHub workflow trigger policies
  -> docs-only signal OR broader testing/compatibility/container policies
  -> primitive Cargo/linter checks and repository E2E runners

skills / agents -> choose direct commands, hooks, workflows, and manual review
  -> repeated procedure text can drift from executable operation lists
```

Current source-of-truth boundaries are fragmented but identifiable:

- Executable operation semantics live in Cargo tests/binaries, external linters, hook scripts,
  workflow commands, and tool configuration.
- Context-specific selection lives in hook step arrays, workflow jobs/triggers, skills, agents,
  and `AGENTS.md`.
- Human and agent recovery procedures live primarily in skills and agent definitions.
- GitHub branch protection and required-check configuration are outside this repository and have
  not yet been inventoried.

## Initial Overlap and Drift Findings

1. Pre-commit and pre-push duplicate a substantial Bash framework for arguments, execution,
   timing, logging, JSON escaping, and summaries while selecting different operations.
2. Local and CI policies invoke several identical primitives, but toolchain, flags, changed-file
   scope, and environment differ; they are overlapping guarantees, not automatically reusable
   results.
3. `linter all` provides one command but not one repository-owned result/event contract; its
   delegated tools keep separate configuration and ignore rules.
4. The pre-commit script currently runs four operations, including `cargo deny check bans`, while
   the pre-commit skill and some agent-facing summaries still describe the older three-step gate.
5. Hook JSON mode emits one document after execution, so non-interactive consumers receive no
   structured progress during long steps. Concise mode writes detailed logs outside the event
   payload.
6. The installed hooks are copies, creating a stale-installation risk after `.githooks/` changes.
7. The docs-only path policy is copied across several workflows and depends on comments and path
   filters remaining synchronized.
8. Container E2E duplication is controlled through event conditions in Testing and Container;
   this is an existing example of policy-level redundant-execution avoidance.
9. Security scanning reports findings through SARIF but deliberately does not fail on Trivy's
   vulnerability exit status; “security scan passed” must not be interpreted as “no high or
   critical vulnerabilities.”
10. Skills and agents contain both judgment and objective procedures. Deterministic candidates
    must be extracted selectively, leaving review and decision responsibilities explicit.

## Known Gaps for the Inventory Subissue

- Record measured warm/cold runtime and feedback latency for representative local and CI paths.
- Capture exact stdout, stderr, exit-code, log, artifact, and JSON schemas for each executable
  entry point.
- Trace every workflow trigger, path filter, required status, permission boundary, and external
  service dependency, including branch-protection settings not stored in the repository.
- Confirm owners and maintenance boundaries for each operation, policy, configuration, and
  external binary.
- Enumerate all skill-local scripts and `contrib/dev-tools/` tools that mutate or validate state;
  the initial pass emphasizes the surfaces already implicated by the EPIC.
- Separate cache writes needed for execution from repository mutations and identify undeclared
  network, container, credential, and tool-installation requirements.
- Build a machine-readable operation-to-policy matrix after identifiers and equivalence semantics
  are designed; this Markdown inventory is not that future configuration.
- Validate documentation drift findings against current maintainers' intended policy before
  treating either executable code or prose as normatively correct.
- Determine which current checks are merge-required in GitHub settings and which only produce
  informational statuses.

## Validation Plan for Phase 1

1. Select at least one local hook, one primitive check, one CI composite guardrail, one mutating
   action, one skill, and one agent policy and trace each from trigger through result.
2. Cross-check repository files by entry-point class rather than assuming this first-pass list is
   exhaustive.
3. Run representative commands only where doing so is safe and useful; record environment,
   runtime, output channels, exit codes, artifacts, logs, and side effects.
4. Review overlap claims using exact command, configuration, toolchain, inputs, and environment;
   label near-matches rather than claiming equivalence without evidence.
5. Obtain maintainer review of ownership, intentional duplication, external settings, and known
   omissions, then update this document as the accepted Phase 1 baseline.

## References

- [`EPIC.md`](EPIC.md)
- [`previous-single-runner-proposal.md`](previous-single-runner-proposal.md)
- `AGENTS.md`
- `.github/workflows/`
- `.github/skills/`
- `.github/agents/`
- `.githooks/`
- `contrib/dev-tools/git/`
- `contrib/dev-tools/analysis/workspace-coupling/`
- `deny.toml`
- `project-words.txt`
