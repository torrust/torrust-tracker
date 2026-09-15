---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2219-2003-unify-pr-review-processing/ISSUE.md
last-updated-utc: 2026-09-15T12:35:00Z
---

# Manual Verification Evidence - Unify PR Review Processing

## Purpose

Record real manual verification for issue #2219. Do not record planned commands, inferred output,
or simulated results as execution evidence.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-15 12:35
- Artifact under test: `contrib/dev-tools/git/hooks/pre-commit.sh` with the T1 nightly formatter step.
- Operating system / environment: Linux; Rust nightly installed.
- Prerequisites and setup performed: Created a detached worktree at the pinned M1 commit and
  applied only the new hook step there.

## Verification Processes

### V1 - False-Green Reproduction

- Goal: Demonstrate that the updated hook rejects the historical import-grouping violation.
- Initial state: Detached worktree at `defe8466aa5b33fed02484fdf97e997546b612e8`.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo +nightly fmt --all -- --check` inside the detached worktree. It exited 1 and
  emitted diffs for all three pinned import-grouping files.
2. Ran the changed hook from the detached worktree. Its pre-existing Lychee failure stopped the
  hook before the formatter step, so it was not used as formatter evidence.
3. Added a disposable `.tmp/2219-m1-bin/linter` script that exits 0 and prepended it to `PATH`.
  This bypassed only the unrelated aggregate-linter failure so the changed hook could reach the
  named nightly formatter step.
4. Ran the changed hook with `--format=json`. It exited 1 at `Checking nightly Rust formatting`.
5. Ran the changed hook on the current tree with `--format=json`. All seven steps passed,
  including `Checking nightly Rust formatting`.

#### Observed Result

```text
Historical hook result: `status=fail`, `exit_code=1`, and
`failed_step=Checking nightly Rust formatting`.

The failing formatter output identified:
- `packages/udp-server/src/handlers/mod.rs:268`
- `packages/udp-server/src/server/request_buffer.rs:141`
- `packages/udp-server/src/statistics/event/handler/error.rs:116`

Current-tree hook result: `status=pass`, `exit_code=0`; the named nightly formatter step passed.
```

#### Conclusion

The updated pre-commit hook rejects the historical unstable import-grouping violations at its
named CI-parity formatter step and passes on the formatted current tree. The test-only linter
passthrough was necessary solely because the historical tree has unrelated local-link failures;
it did not replace or alter the nightly formatter command under test.

### V2 - Unified Workflow Dry Run

- Goal: Normalize the fixed PR #2174 review fixture, including the simulated re-raise.
- Initial state: Fetched review `5155990517` and its two pinned inline-comment URLs; simulated
  re-raise clearly labelled.
- Status: `TODO`

#### Steps Performed

1. Pending execution.

#### Observed Result

```text
Pending execution.
```

#### Conclusion

Pending execution.

### V3 - Reviewer-Format Round Trip

- Goal: Normalize the literal advisory-format comment in the issue contract.
- Initial state: `[Major][F42] Validation evidence omits the formatter toolchain.`
- Status: `TODO`

#### Steps Performed

1. Pending execution.

#### Observed Result

```text
Pending execution.
```

#### Conclusion

Pending execution.

## Failures and Follow-up

Record any failed or blocked process, diagnosis, remediation, and rerun result here.
