---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p2
epic: 2392
github-issue: null
spec-path: docs/issues/drafts/2392-figment-jail-process-environment/ISSUE.md
branch: "{issue-number}-2392-figment-jail-process-environment"
related-pr: null
last-updated-utc: "2026-11-18 11:10"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2907-2392-remove-leaked-test-temporary-files/ISSUE.md
    - docs/issues/closed/2394-2392-audit-shared-test-resources/inventory.md
    - packages/configuration/src/lib.rs
    - packages/configuration/src/v2_0_0/mod.rs
    - packages/configuration/src/v3_0_0/mod.rs
    - src/bootstrap/config.rs
---

<!-- skill-link: create-issue -->

# Keep `figment::Jail` Tests From Changing the Environment Other Tests See

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

No test of the `torrust-tracker-configuration` lib test binary sees an environment or a current
directory that another test changed. Today the tests that use `figment::Jail` change both for the
whole process while they run, and the tests that do not use `Jail` run at the same time (EPIC
rule 1).

## Background

The shared-resource audit (#2394) recorded this resource as finding G2 of its `inventory.md` and
classified it as isolated: "`Jail` serializes on its own global lock and restores both". #2907
found that the classification is wrong.

What `figment::Jail` (figment 0.10.19, `src/jail.rs`) does:

- `Jail::expect_with` takes a private global lock, creates a temporary directory and makes it the
  current directory of the process.
- `jail.clear_env()` removes every environment variable of the process, `TMPDIR` and `PATH`
  included. `jail.set_env()` sets one.
- When the jail is dropped it restores the variables and the current directory.

The lock serializes only the tests that use `Jail`. The environment and the current directory
belong to the process, and `cargo test` runs the tests of one binary on several threads. A test
of the same binary that does not use `Jail` and reads the environment or resolves a relative path
while a jail is active sees the jail's state.

Where `Jail` is used (`develop` `bc9d0590c`); no other package uses it:

| File | `Jail::expect_with` | `clear_env()` | `set_env()` |
| --- | --- | --- | --- |
| `packages/configuration/src/lib.rs` | 36 | 18 | 11 |
| `packages/configuration/src/v2_0_0/mod.rs` | 16 | 0 | 1 |
| `packages/configuration/src/v3_0_0/mod.rs` | 50 | 0 | 2 |

The same binary has about 160 tests; the ones outside these 102 closures do not take the lock.

What #2907 observed: the two configuration-file tests (`v2_0_0` and `v3_0_0`) write
`test_config_<uuid>.toml` to `std::env::temp_dir()`. Run alone with a fresh `TMPDIR`, each
leaves one file there. In a run of the whole package only one file was in the fresh `TMPDIR`;
the other went to `/tmp`, because `TMPDIR` was unset when the test read it. The #2907 fix does
not depend on this (a `TempDir` removes its folder by absolute path), so the gap stays.

A second concern: since the 2024 edition, which this workspace uses, Rust marks
`std::env::set_var` and `std::env::remove_var` as `unsafe`, because changing the environment
while another thread reads it is not safe on every platform. The jail does both while other test
threads run.

Finding G1 of the same inventory (`src/bootstrap/config.rs`, `std::env::set_var` in tests of the
`torrust-tracker` lib binary) was classified as isolated with a similar argument. T1 rechecks it
with the same question: can a test outside the serialized group read what the group changes?

## Scope

### In Scope

- Reproduce the interference with a test that does not use `Jail` and reads the environment
  while the jail tests run.
- Make the tests of the configuration lib binary isolated from the jails' environment and
  current-directory changes, with the design the maintainer chooses in T2.
- Recheck finding G1 and record the result; fix it here only if it has the same gap and the fix
  is the same.
- Correct the G2 row (and G1, if the recheck changes it) of
  `docs/issues/closed/2394-2392-audit-shared-test-resources/inventory.md` with a dated note that
  points to this issue.

### Out of Scope

- Changing how production code loads the configuration from the environment.
- Replacing `figment` or its `Jail`.
- The temporary files of #2907 and the SQLite databases of #2924 and #2925.

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none expected.
- Open, decided by the maintainer in T2. Candidate designs:

| Option | What changes | For | Against |
| --- | --- | --- | --- |
| A. Every test that reads the environment or the current directory runs inside a `Jail` | The few non-`Jail` tests that read them move into `Jail::expect_with` | Smallest change; the jail also gives them a temporary directory | Nothing stops a new test from reading the environment outside a jail |
| B. The `Jail` tests move to their own test binary | An integration-test target where every test uses `Jail` | Process boundary; the unit-test binary never sees a changed environment | The tests reach private items today, so some need public or `pub(crate)` seams; about 100 tests move |
| C. The tests stop changing the process environment | The loading code takes its environment from a provider that tests replace | No process-global change left | Changes production code, which is out of scope unless the maintainer widens it |

  The coordinator's starting recommendation is A plus a check that fails when a test of the
  package reads the environment outside a jail, if T2 finds a cheap way to write that check.

## Design and Ownership Review

- Ownership: the process owns the environment and the current directory; a test may change them
  only while every test that can observe them is excluded.
- Failure path: `Jail` restores the state when it is dropped, also when the closure fails.
- Deadline: not applicable, the change awaits no readiness operation.
- Design-review checkpoint: after T2 records the options with the reproduction's result, stop for
  the maintainer's decision before changing any test.

### Test Development Loop

Follow the progressive test-development loop of the `write-unit-test` skill
(`.claude/skills/write-unit-test/SKILL.md`): work in the smallest behavior-focused increment,
review each passing increment with the prose-first Arrange-Act-Assert comparison, record each
review in `manual-verification-evidence.md` and commit the reviewed increment before the next.

## Bug-Fix Process

The defect is in test code. Follow `.claude/skills/fix-bug/SKILL.md`:

1. Real-artifact reproduction: a probe test in the configuration lib binary that does not use
   `Jail` reads `PATH` and `TMPDIR` in a loop while the suite runs, and reports each read that
   finds them unset (V1). The probe is evidence, not a permanent test, unless T2 keeps it.
2. Regression boundary: decided with the design in T2.
3. Red before the change, green after; like-for-like recheck of step 1 (V2).

## Regression Test Strategy

Decided in T2 with the design. The reproduction probe of V1 is the starting candidate: it fails
on `develop` whenever a jail is active during one of its reads.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce and recheck G1 | Probe result recorded (V1); list of the non-`Jail` tests that read the environment or the current directory; G1 verdict recorded. |
| T2 | TODO | Decide the design | Options with the T1 result; the maintainer's decision recorded in this spec. |
| T3 | TODO | Isolate the tests | Implemented as decided; test-design review recorded. |
| T4 | TODO | Correct the audit inventory | G2 (and G1 if needed) corrected with a dated note. |
| T5 | TODO | Recheck | V2 recorded; `linter all` passes. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Evidence and decision | Documentation-only commit after maintainer review. |
| T3 | Isolated tests | Commit after focused validation and review. |
| T4-T5 | Inventory correction and recheck evidence | Documentation-only commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2392-figment-jail-process-environment/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-18 11:10 UTC - Claude Code (Claude Fable 5.2), EPIC #2392 coordinator - Drafted on the
  maintainer's instruction, from the gap the #2907 session reported and the coordinator confirmed
  in `figment` 0.10.19 and `packages/configuration/src`.

## Acceptance Criteria

- [ ] AC1: The reproduction shows a test outside `Jail` reading an unset `PATH` or `TMPDIR` on
  `develop`, or the spec records why it could not be reproduced.
- [ ] AC2: After the change, the same reproduction finds no such read in the number of runs T2
  sets.
- [ ] AC3: No test of the configuration lib binary reads the environment or the current directory
  while another test of that binary can change them.
- [ ] AC4: The audit inventory no longer classifies G2 as isolated without a pointer to this
  issue, and records the G1 recheck.
- [ ] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| V1 | Reproduce | `cargo +stable test -p torrust-tracker-configuration --lib` with the probe test, repeated as T1 records | Reads with `PATH` or `TMPDIR` unset, counted per run | TODO | `manual-verification-evidence.md` |
| V2 | Recheck | Same command and number of runs after T3 | No such read | TODO | `manual-verification-evidence.md` |

Evidence policy: every recorded command result names the toolchain that produced it, and the
evidence file is created from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | V1 |
| AC2 | TODO | V2 |
| AC3 | TODO | Code review and the list from T1 |
| AC4 | TODO | `inventory.md` diff |

## Risks and Trade-offs

- **A timing-dependent reproduction.** The probe only fails when a jail is active during a read.
  T1 records how many runs it needed; if it never fails, the code reading of `Jail` stays the
  evidence and the spec says so.
- **Option A does not prevent a relapse.** A later test that reads the environment outside a jail
  brings the gap back, which is why T2 looks for a check.
- **Small real-world effect so far.** The only observed effect is a temporary file in `/tmp`
  instead of `TMPDIR`. The cost of leaving it is a test that fails or misbehaves rarely and is
  hard to trace.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If the work invalidates an assumption of this spec, changes its design materially, or finds
  something reusable, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this folder; otherwise add a progress-log
  entry that says why none was needed.
- An independent reviewer that receives this folder-style specification records its result in
  `agent-review-reports.md` (`docs/templates/AGENT-REVIEW-REPORTS.md`).

## References

- Parent EPIC: #2392
- Found by: #2907 (its evidence file, "Failures and Follow-up")
- Corrects: #2394 finding G2
