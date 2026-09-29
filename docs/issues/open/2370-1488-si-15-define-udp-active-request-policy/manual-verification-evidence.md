---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2370-1488-si-15-define-udp-active-request-policy/ISSUE.md
last-updated-utc: "2026-09-29 13:15"
---

# Manual Verification Evidence

## Purpose

Record the orphaned-processor bug reproduction and final like-for-like recheck,
then the executable-boundary scenarios from the issue specification.

## Environment and Prerequisites

- Date and time (UTC): To be recorded with each executed scenario.
- Artifact under test: To be recorded by Conventional Commit subject.
- Operating system / environment: To be recorded with each executed scenario.
- Prerequisites and setup performed: To be recorded with each executed scenario.

## Bug Evidence

### V1 - Orphaned-Processor Reproduction

- Goal: Observe the current full-buffer path losing a live processor handle.
- Initial state: Current ring-only processor ownership before the T5 fix.
- Status: `TODO`

#### Steps Performed

1. Attempt a direct tracker run that reaches the full-buffer ordering.
2. Record `Reproduced`, `Trigger only`, or `Infeasible`, including exact command, toolchain, output, and logs.
3. When the public artifact cannot expose the lost internal handle, record the temporary nearest-seam observation test verbatim, revert it, and use its maintained successor for the red regression proof.

#### Observed Result

```text
Not yet executed.
```

#### Conclusion

Not yet assessed.

### V2 - Red and Green Regression Evidence

- Goal: Prove the receive-loop collaboration test fails with ring-only ownership and passes after the T5 owner and drain wiring.
- Initial state: To be recorded when T3 adds the maintained test.
- Status: `TODO`

#### Steps Performed

1. Record the focused red test command and output before the production fix.
2. Record the focused green test command and output after the fix.
3. Mutate the fix without staging it, confirm the test fails, and restore it.

#### Observed Result

```text
Not yet executed.
```

#### Conclusion

Not yet assessed.

## Shutdown Verification

Record M1-M4 after implementation, including direct binary PID, command output,
relevant logs, bounded exit status, and listener rebind evidence.

## Failures and Follow-up

Record failed or blocked scenarios, diagnosis, remediation, and rerun status.
