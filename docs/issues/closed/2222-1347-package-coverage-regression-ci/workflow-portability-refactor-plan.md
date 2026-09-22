---
doc-type: refactor-plan
status: in-progress
related-issue: 2222
spec-path: docs/issues/closed/2222-1347-package-coverage-regression-ci/workflow-portability-refactor-plan.md
last-updated-utc: 2026-09-22 16:40
semantic-links:
  related-artifacts:
    - .github/workflows/generate_coverage_pr.yaml
   - contrib/dev-tools/checks/package-coverage-check/
    - AGENTS.md
      - .github/workflows/AGENTS.md
      - .github/skills/dev/ci/implement-workflow/SKILL.md
---

# Workflow Portability Refactor Plan

## Goal

Make the package-coverage workflow a thin GitHub Actions adapter and move its
non-trivial behavior into a repository-owned Rust tool. Rename the tool so its
location under `contrib/dev-tools/checks/` and its CI-check role are clear.

Related artifact: [Issue #2222 specification](ISSUE.md)

## Boundary

GitHub Actions remains responsible only for platform-specific orchestration:

- pull-request triggering, permissions, runners, and immutable revision checkouts;
- dynamic-matrix scheduling;
- artifact upload/download; and
- writing a command's Markdown output to `$GITHUB_STEP_SUMMARY`.

The Rust tool owns portable behavior:

- changed-package discovery and base/head package pairing;
- Codecov report parsing and exact coverage comparison;
- validation of comparison-result artifacts;
- rate and percentage-point delta formatting; and
- report-only Markdown summary rendering.

The command interface must use ordinary command-line arguments, JSON files or
standard input, and standard output. It must not require GitHub environment
variables or GitHub Actions output files. A CI provider other than GitHub must
be able to call the same commands and consume their JSON or Markdown output.

## Items

### 1. [x] Rename the CI tool to `package-coverage-check` [High impact / Low effort]

**Problem**: `package-coverage-regression` sounds like an application workspace
package. Its actual location is `contrib/dev-tools/checks/`, where it is
repository-owned CI/developer tooling. The current name obscures that boundary.

**Files**:

- `Cargo.toml`
- `Cargo.lock`
- `contrib/dev-tools/checks/package-coverage-check/`
- `.github/workflows/generate_coverage_pr.yaml`
- `docs/testing.md`
- `docs/issues/closed/2222-1347-package-coverage-regression-ci/`

**Change**:

1. Move the directory to `contrib/dev-tools/checks/package-coverage-check/`.
2. Rename the Cargo package and crate imports to `package-coverage-check` and
   `package_coverage_check`.
3. Update every workspace member, workflow invocation, test command,
   documentation reference, and issue evidence.
4. Keep the GitHub job identifier `package-coverage-regression`: it names the
   reported condition, not the tool implementing the check.

**Validation**:

- `cargo test -p package-coverage-check`
- `cargo clippy -p package-coverage-check --all-targets --all-features -- -D warnings`
- `cargo machete --with-metadata`
- `git diff --check`

**Commit checkpoint**: Commit this rename after the focused checks pass, before changing the
command contract or workflow behavior.

---

### 2. [x] Define portable discovery and summary command contracts [High impact / Medium effort]

**Problem**: the workflow currently parses discovery JSON with `jq` and renders
its report using `jq`, `awk`, Bash loops, and process substitution. Those
operations are behavior and formatting logic coupled to GitHub-hosted shell
semantics, rather than an adapter concern.

**Files**:

- `contrib/dev-tools/checks/package-coverage-check/src/lib.rs`
- `contrib/dev-tools/checks/package-coverage-check/src/main.rs`
- `.github/workflows/generate_coverage_pr.yaml`

**Change**:

1. Define a generic discovery command that emits compact JSON for either the
   dynamic matrix or unavailable outcomes. It receives only base/head workspace
   roots and immutable Git revisions.
2. Define a generic summary command that receives discovery JSON and a
   directory containing comparison-result JSON files, then writes Markdown to
   standard output.
3. Make result-file validation explicit: absent, malformed, duplicate, and
   unexpected package results must render an informative unavailable/error row
   without failing this report-only summary job.
4. Keep GitHub-specific output assignment to simple command substitution and
   `echo` statements. Do not add GitHub environment handling to Rust.

**Target workflow shape**:

```yaml
- id: discover
  run: |
    echo "matrix=$(cargo run -p package-coverage-check -- matrix ...)" >> "$GITHUB_OUTPUT"
    echo "unavailable=$(cargo run -p package-coverage-check -- unavailable ...)" >> "$GITHUB_OUTPUT"

- name: Write Report-Only Summary
  run: |
    cargo run -p package-coverage-check -- summary \
      "$MATRIX_JSON" "$UNAVAILABLE_JSON" package-coverage-results \
      >> "$GITHUB_STEP_SUMMARY"
```

The final spelling may use temporary JSON files rather than JSON command-line
arguments if that produces clearer quoting and error handling. The Rust command
contract remains provider-neutral in either form.

**Validation**:

- Unit tests for summary rendering with comparable, unavailable, and no-package
  discovery data.
- Unit or command-level tests for absent, malformed, duplicate, and unexpected
  comparison-result files.
- A local command invocation that renders the same Markdown written in CI.

**Commit checkpoint**: Commit the portable command contract and its tests after they pass, before
switching the workflow to consume it.

---

### 3. [x] Simplify the GitHub Actions adapter [Medium impact / Low effort]

**Problem**: [`.github/workflows/generate_coverage_pr.yaml`](../../../../.github/workflows/generate_coverage_pr.yaml)
contains report decisions and numeric presentation details that the Rust tool
can own and test.

**Files**:

- `.github/workflows/generate_coverage_pr.yaml`

**Change**:

1. Remove the summary Bash loops and all `jq`/`awk` parsing and formatting.
2. Retain only checkout, tool installation, coverage commands, artifact
   transport, workflow output assignment, and calls to the Rust tool.
3. Preserve the unprivileged `pull_request` boundary, immutable base/head
   checkouts, dynamic matrix, report-only behavior, and existing workspace-wide
   Codecov job.
4. Preserve an always-present summary job even when the matrix is empty or all
   comparison artifacts are unavailable.

**Validation**:

- `linter yaml`
- `git diff --check`
- A reviewed hosted pull-request run covering an empty matrix, one comparable
  package, and an unavailable package outcome.

**Commit checkpoint**: Commit the workflow simplification after focused local validation. Record
the hosted run separately when its evidence is available.

---

### 4. [x] Establish reusable CI workflow implementation guidance [High impact / Medium effort]

**Problem**: the current shell-versus-Rust policy in `AGENTS.md` says when Rust
is preferred, but it does not make the CI portability, testability, and
provider-boundary requirements discoverable without making the root instruction
file larger.

**Files**:

- `AGENTS.md`
- `.github/workflows/AGENTS.md`
- `.github/skills/dev/ci/implement-workflow/SKILL.md`

**Change**:

1. Create `.github/skills/dev/ci/implement-workflow/SKILL.md` as the canonical,
   repository-owned procedure for adding or materially changing CI workflows.
   It must require thin provider adapters, portable command-line contracts,
   automated tests for non-trivial tooling, Rust by default, and documented
   alternatives for unavoidable provider-specific behavior.
2. Add a concise pointer in `AGENTS.md` to the workflow-implementation skill.
   Retain the existing shell-versus-Rust threshold rather than duplicating the
   detailed policy in the root file.
3. Create `.github/workflows/AGENTS.md` for local workflow constraints only:
   pinned actions, least-privilege permissions, immutable revision checkout,
   supported validation commands, and the requirement to follow the canonical
   skill. Do not duplicate the skill's full portability procedure.
4. Treat a custom agent as an optional execution adapter. It may point to the
   skill, but it must not be the sole source of the policy.

The skill must state this repository-wide rule:

> Keep CI workflow definitions as thin platform adapters. Put non-trivial
> decisions, data transformation, validation, and report rendering in
> repository-owned tools with a documented command-line interface and automated
> tests. Prefer Rust for that logic unless another implementation language is
> justified. Workflow files may retain only provider-specific triggering,
> permissions, runner setup, checkout, scheduling, artifact transport, and the
> small adapter calls needed to invoke the tool. Document unavoidable
> provider-specific behavior and the practical portable alternative.

The existing portability ADR supplies the broader rationale. No new ADR is
needed unless a later implementation changes a CI trust or permission boundary.

**Validation**:

- Review the skill, root pointer, and scoped instructions against the existing
   shell-versus-Rust and portability policies to avoid duplication or
   contradiction.
- `linter markdown`
- Confirm the workflow refactor follows the documented rule.

**Commit checkpoint**: Commit the skill, concise root pointer, and scoped
workflow instructions together as documentation after the maintainer approves
them and the Markdown checks pass.

---

### 5. [ ] Reconcile rollout evidence after the refactor [Medium impact / Medium effort]

**Problem**: the current evidence proves a local double build but does not yet
prove hosted artifact transport, the final rendered summary, or fork behavior.

**Files**:

- `docs/issues/closed/2222-1347-package-coverage-regression-ci/ISSUE.md`
- `docs/issues/closed/2222-1347-package-coverage-regression-ci/manual-verification-evidence.md`

**Change**:

Run and record a hosted pull request after items 1-3. Record elapsed job times,
rendered summary output, comparable and unavailable outcomes, artifact behavior,
and a fork pull request result. Do not claim hosted verification from local
commands.

**Validation**:

- Complete M2-M5 in the issue specification with direct evidence.
- Reconcile acceptance criteria and implementation completion review.

**Commit checkpoint**: Commit each material hosted-verification record as documentation once the
observed evidence has been reviewed; do not combine it with implementation changes.

## Order of Execution

| Order | Status | Item | Impact | Effort |
| --- | --- | --- | --- | --- |
| 1 | [x] | Rename the CI tool | High | Low |
| 2 | [x] | Define portable command contracts and tests | High | Medium |
| 3 | [x] | Simplify the GitHub Actions adapter | Medium | Low |
| 4 | [x] | Establish reusable CI workflow implementation guidance | High | Medium |
| 5 | [ ] | Reconcile hosted rollout evidence | Medium | Medium |

## Commit Boundaries

1. `refactor(ci): rename package coverage check tool` for item 1.
2. `refactor(ci): move coverage summary rendering into Rust` for item 2.
3. `refactor(ci): simplify package coverage workflow adapter` for item 3.
4. `docs(ci): add portable workflow implementation guidance` for item 4.
5. `docs(issues): record package coverage workflow rollout` for hosted item-5 evidence.
