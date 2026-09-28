---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2281-2264-frontmatter-validator-command/ISSUE.md
    - docs/issues/closed/2281-2264-frontmatter-validator-command/test-design-review.md
    - docs/issues/closed/2281-2264-frontmatter-validator-command/manual-verification-evidence.md
---

# Implementation Retrospective — Issue #2281 Frontmatter Validator Command

## Purpose

Record evidence-based process improvements discovered while implementing issue #2281. This is a
blameless review of the implementation approach. It does not replace acceptance-criteria
verification.

## Outcome

`frontmatter-validator` is a read-only command with three modes: explicit paths, `--staged`, and
`--all`. It emits the D9 NDJSON record catalog on stderr and exits `0`, `1`, or `2`. Its parts:

- The library's pure `repository` module owns location-dependent severity, `legacy-shape`,
  experimental-field warnings, and the D7 checks, over a `RepositoryFiles` snapshot.
- The binary owns discovery, git access, and rendering.
- Pre-commit runs it as a named `--staged` step.

The crate has 179 tests. Each test-producing task was proven by mutation, and manual scenarios
M1-M7 were recorded in `manual-verification-evidence.md`.

## What Went Well

1. **The vertical-slice checkpoint after T2.** It surfaced four boundary decisions (R1-R4) before
   they were built. R1, a pure policy module, made every severity and D7 rule unit-testable
   without git. That paid off in T4 and T5, where each rule got a table test with literal
   expectations.
2. **Mutation checks exposed weak tests and non-compiling mutants.** Twice the first mutant did
   not compile under `-D warnings`, and once a mutant survived: the external-file exemption test
   lacked the `doc-type: issue` it existed for. A passing suite alone would have hidden all
   three.
3. **Running the real binary against the repository before T6.** It turned the baseline into
   concrete decisions on four fix groups that the maintainer could answer quickly.

## What Changed During Implementation

- **Container tests.** The crate had never been excluded from the container test archives, and
  the new `tests/cli.rs` needs `git`, which `rust:slim-trixie` does not install. The spec planned
  only cargo-chef stubs. A `docker run` check confirmed the gap, and the crate is now excluded
  like the other dev-tool crates.
- **Masked errors.** Because the library reports one structural error per document, each T6 fix
  exposed the next one. `status: open` hid a trailing-slash artifact, two unquoted timestamps,
  and a stale reference. One envelope error hid a draft's `legacy-shape`. The baseline count was
  a lower bound, not the work list.
- **Scope beyond the spec.** Ten syntax and envelope errors sat in closed and unrelated
  documents that the spec did not anticipate. They needed maintainer decisions mid-task.
- **Stale specs.** At the initial baseline, two completed issues, #2324 and #2179, still had
  specs in `open/`, and #2324 became the accepted AC10 exception. The later rebase archived both,
  introduced two new open specs with unsupported `open` statuses, and required a fresh baseline
  before the PR could merge.
- **A pipe hid a failing gate.** One commit passed a failing `linter all` because the gate was
  piped into `tail`. It was fixed in the next commit, and the rule is now in the
  `run-pre-commit-checks` skill.
- **Test setups that proved nothing.** Two first attempts had to be redone: the first T5
  command-boundary snapshot test and the first reverse M3 scenario. Their setups could not
  distinguish the behavior they claimed to test.

## Root Cause

- The plan treated the container as a build concern, and did not check what the container test
  stage runs.
- Baseline triage assumed each finding was independent. The one-error-per-document design makes
  findings sequential.
- The gate command pattern had no exit-code discipline. A pipe inside an `&&` chain is an easy
  and silent way to lose the status.

## Improvements for Future Work

1. When a crate adds process-spawning tests (git, docker, network), check the container test
   stage's image and the `--exclude` lists in the same task. Applied: the Containerfile's
   maintenance comment now says so.
2. Treat a whole-tree baseline under a first-error validator as a lower bound. Re-run after each
   fix group until the result is stable, and say so in the spec.
3. Gate commits on a captured exit code (`cmd > log 2>&1; rc=$?`), never on a pipeline in an
   `&&` chain. Applied: added to the `run-pre-commit-checks` skill.
4. For a "not A but B" test, write down what distinguishes A from B in the setup before writing
   the assertion. The independent review caught a related slip: fixture tests carried a hidden
   `spec-path` mismatch that a structural error happened to mask.

## Avoiding Overcorrection

- No multi-error structural recovery is justified: fixing and re-running is fast, and the command
  already reports every warning and repository finding.
- No new rule that every dev-tool crate must be container-excluded: the decision depends on what
  its tests need.
- No archival of stale specs from this PR: that stays in the usual archive workflow.

## Evidence

- `ISSUE.md`: implementation plan, decisions D1-D11, R1-R4, and the progress log.
- `test-design-review.md`: prose-first reviews and mutation tables for T1-T5 and T8.
- `manual-verification-evidence.md`: B1 and V1-V7.
- Commits on `2281-frontmatter-validator-command`, cited by subject in the progress log.
