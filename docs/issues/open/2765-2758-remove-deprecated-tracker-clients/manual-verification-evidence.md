---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2765-2758-remove-deprecated-tracker-clients/ISSUE.md
last-updated-utc: "2026-11-17 09:07"
---

# Manual Verification Evidence

## Purpose

Real verification of issue #2765 (remove the deprecated tracker client binaries). Both runs below
were executed on the implementation branch after the removal and the e2e helper migration.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-17 09:07
- Artifact under test: the PR patches "remove the deprecated tracker client binaries" and
  "run `tracker_client check` from the e2e runner and capture its output".
- Operating system / environment: Linux, rustc 1.99.1 (2026-10-13).
- Setup: the tracker built from the branch, started from a scratch directory with
  `TORRUST_TRACKER_CONFIG_TOML_PATH=share/default/config/tracker.development.sqlite3.toml`
  (UDP 6969, HTTP 7070, health check API on 1212).

## Verification Processes

### V1 - Unified checker (M1)

- Goal: `tracker_client check` works against a local tracker as the README documents.
- Status: `DONE`

Command (stdin from `/dev/null`, stdout and stderr redirected to files; ran 09:07:35.123 to 09:07:35.136):

```text
TORRUST_CHECKER_CONFIG='{"udp_trackers":["127.0.0.1:6969"],"http_trackers":["http://127.0.0.1:7070"],"health_checks":["http://127.0.0.1:1212/api/health_check"]}' target/debug/tracker_client check
```

Result: exit code `0`, stderr empty, stdout:

```json
[{"Udp":{"Ok":{"remote_addr":"127.0.0.1:6969","results":[["Setup",{"Ok":null}],["Connect",{"Ok":null}],["Announce",{"Ok":null}],["Scrape",{"Ok":null}]]}}},{"Health":{"Ok":{"url":"http://127.0.0.1:1212/api/health_check","result":{"Ok":"200 OK"}}}},{"Http":{"Ok":{"url":"http://127.0.0.1:7070/","results":[["Announce",{"Ok":null}],["Scrape",{"Ok":null}]]}}}]
```

### V2 - E2E checker launch (M2, smoke test)

- Goal: the helper `tracker_checker::run` launches `tracker_client check` with
  `TORRUST_CHECKER_CONFIG`, captures the child's stdout and stderr, and succeeds.
- Status: `DONE`
- Scope: this called the helper directly from a throwaway integration test (not committed, deleted
  after the run) against the tracker of V1; the full e2e runner (which needs Docker) was not run.
  The interactive-terminal case is verified in issue #2768 once the TTY refusal exists.

The run (09:07:38.800 to 09:07:40.959) passed. The helper logged this line, which names the
unified binary and the environment variable:

```text
Running Tracker Checker: TORRUST_CHECKER_CONFIG=[config] cargo run -p torrust-tracker-client --bin tracker_client -- check
```

and then the captured stdout (the same JSON array as V1) and the captured stderr
(cargo's "Finished" and "Running `target/debug/tracker_client check`" lines).
