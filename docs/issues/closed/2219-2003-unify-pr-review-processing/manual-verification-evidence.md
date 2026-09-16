---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2219-2003-unify-pr-review-processing/ISSUE.md
last-updated-utc: 2026-09-16 11:09
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
5. Ran the changed hook on the current tree with `--format=json`. All eight steps passed,
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
- Status: `DONE`

#### Steps Performed

1. Fetched all PR #2174 review threads with
  `get-pr-review-threads.sh --pr-number 2174 --output-file .tmp/2219-pr-2174-threads.json`.
2. Located the two pinned source comments in GraphQL thread IDs `PRRT_kwDOGp2yqc6gtUXI` and
  `PRRT_kwDOGp2yqc6gtUXL`.
3. Normalized the fetched `Major` validation-evidence comment as `F1 | ORIGINAL` and the fetched
  `Nit` fixture-naming comment as `F2 | ORIGINAL`, in source order.
4. Modelled a later comment requesting the same current-tree change as F1 as the explicitly
  simulated `F3 | RE_RAISE_OF:F1` row.
5. Ran `list-unresolved-threads.sh --threads-file .tmp/2219-pr-2174-threads.json`.

#### Observed Result

```text
The source threads were both `isResolved=true` and `isOutdated=true`; each included a durable
source-comment URL and author reply URL. The final unresolved-thread helper produced no output and
exited 0.

Normalized dry-run rows:

| Finding ID | Source review ID | Severity | Relationship | Thread state |
| ---------- | ---------------- | -------- | ------------ | ------------ |
| F1 | 5155990517 | Major (inferred) | ORIGINAL | RESOLVED |
| F2 | 5155990517 | Nit (inferred) | ORIGINAL | RESOLVED |
| F3 | simulated later comment | Major (inferred) | RE_RAISE_OF:F1 | simulated |
```

#### Conclusion

The unified skill can normalize human review threads, preserve their GraphQL state, and model a
re-raised concern without duplicating the original finding. The review body was fetched as source
context but produced no independent finding row because it summarized the two inline concerns.

### V3 - Reviewer-Format Round Trip

- Goal: Normalize the literal advisory-format comment in the issue contract.
- Initial state: `[Major][F42] Validation evidence omits the formatter toolchain.`
- Status: `DONE`

#### Steps Performed

1. Ran the following command, which restricts matching to the five contract severities and the
  `[<Severity>][<FindingId>] <summary>` first-line form:

   ```sh
   comment='[Major][F42] Validation evidence omits the formatter toolchain.'
   if [[ "$comment" =~ ^\[(Blocker|Major|Minor|Nit|Suggestion)\]\[(F[0-9]+)\]\ (.+)$ ]]; then
     severity=${BASH_REMATCH[1]}
     finding_id=${BASH_REMATCH[2]}
     summary=${BASH_REMATCH[3]}
     relationship=ORIGINAL
     printf 'Finding ID=%s\nSeverity=%s\nRelationship=%s\nSummary=%s\n' "$finding_id" "$severity" "$relationship" "$summary"
     [[ "$finding_id" == F42 && "$severity" == Major && "$relationship" == ORIGINAL && "$summary" == 'Validation evidence omits the formatter toolchain.' ]]
   else
     exit 1
   fi
   ```

2. The command exited 0 after asserting the parsed finding ID, severity, relationship, and
  summary against the pinned M3 expected values.

#### Observed Result

```text
Finding ID=F42
Severity=Major
Relationship=ORIGINAL
Summary=Validation evidence omits the formatter toolchain.
```

#### Conclusion

The literal advisory-format comment normalizes to the pinned fields without free-prose parsing.

### V4 - Portable Review-Finding Reference

- Goal: Verify that a deterministic repository reference can identify an audit row and be cited
  from prose and semantic-link metadata without a GitHub identifier.
- Initial state: Representative PR number `2230` and canonical finding ID `F1`.
- Status: `DONE`

#### Steps Performed

Ran the following command:

```sh
set -euo pipefail
pr_number=2230
finding_id=F1
reference="review-finding:pr-${pr_number}-${finding_id,,}"
row="${pr_number}|${finding_id}|${reference}"
prose="Motivated by ${reference}."
yaml="    - ${reference}"
[[ "${reference}" == 'review-finding:pr-2230-f1' ]]
[[ "${reference}" =~ ^review-finding:pr-[0-9]+-f[0-9]+$ ]]
[[ "${row}" == '2230|F1|review-finding:pr-2230-f1' ]]
printf '%s\n%s\n%s\n' "${row}" "${prose}" "${yaml}"
```

The command exited `0` after deriving the lowercase reference from `2230` and `F1`, asserting its
exact value, format, and audit-row placement, then printing both documented citation forms.
`set -euo pipefail` makes a failed assertion return nonzero instead of reaching the final `printf`.

#### Observed Result

```text
2230|F1|review-finding:pr-2230-f1
Motivated by review-finding:pr-2230-f1.
    - review-finding:pr-2230-f1
```

#### Conclusion

The deterministic reference identifies the repository audit concept independently of GitHub
review, thread, comment, and URL identifiers, while supporting both documented citation forms.

## Failures and Follow-up

Record any failed or blocked process, diagnosis, remediation, and rerun result here.
