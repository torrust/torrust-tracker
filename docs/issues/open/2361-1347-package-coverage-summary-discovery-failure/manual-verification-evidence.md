---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2361-1347-package-coverage-summary-discovery-failure/ISSUE.md
last-updated-utc: 2026-09-28 10:00
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-28 09:55
- Artifact under test: `package-coverage-check` on `develop` at `478516cf`, the tool that the
  `Package Coverage Regression` job of `generate_coverage_pr.yaml` runs.
- Operating system / environment: Linux local development workspace, stable Rust toolchain
  (`rustc 1.98.1`).
- Prerequisites and setup performed: none beyond a workspace build. The workflow's summary step
  writes the discovery job's output with `printf '%s' "$DISCOVERY"`. When the discovery job fails
  or is skipped, that output is the empty string, so the reproduction writes an empty discovery
  file the same way.

## Verification Processes

### V0 - Initial Reproduction (Before the Fix)

- Goal: Observe the summary command failing when discovery produced no output.
- Initial state: an empty discovery file and a missing results directory. A discovery failure
  also means no comparison job ran, so no results directory is downloaded.
- Status: `DONE`
- Outcome: **Reproduced**. The wrong outcome is that the report-only summary exits non-zero with
  a JSON parse error, and that was observed at the tool level. The red hosted check follows from
  the workflow, whose summary job declares no `continue-on-error`. It was not observed on a
  hosted runner, because doing so needs a failing discovery job in a pull request.

#### Steps Performed

1. `mkdir -p .tmp/fu-b && printf '%s' "" > .tmp/fu-b/package-coverage-discovery.json`
2. `cargo run -q -p package-coverage-check -- summary .tmp/fu-b/package-coverage-discovery.json .tmp/fu-b/package-coverage-results; echo "exit=$?"`
3. Control: `printf '%s' '{"matrix":{"include":[]},"unavailable":[]}' > .tmp/fu-b/ok.json`, then
   the same command with `.tmp/fu-b/ok.json`.

#### Observed Result

```text
package-coverage-check: .tmp/fu-b/package-coverage-discovery.json: invalid JSON: EOF while parsing a value at line 1 column 0
exit=1
```

Control:

```text
## Package Coverage Regression

This report is informational and does not block merging.

No directly changed workspace package was selected.

exit=0
```

#### Conclusion

The summary fails on empty discovery input with a parse error that does not name the cause, while
a valid empty discovery renders an informational report. The bug is reproduced at the tool level.

## Failures and Follow-up

None yet.
