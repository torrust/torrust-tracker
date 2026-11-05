---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p3
epic: null
github-issue: 2733
spec-path: docs/issues/closed/2733-health-check-failing-status/ISSUE.md
branch: "2733-health-check-failing-status"
related-pr: 2740
last-updated-utc: "2026-11-05 21:55"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-health-check-api-server/src/resources.rs
    - docs/issues/closed/2726-rest-api-https-health-check/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2733 - The REST API health check reports `Ok` when the health endpoint answers a failing HTTP status

## Goal

Make the health checks that the REST API and the HTTP tracker register report an error when the health endpoint answers
with a failing HTTP status (for example `500`), instead of reporting `Ok`.

## Background

Noticed while fixing #2726 and left out of that fix on purpose (see its "Observation, a separate question" section).

`check_fn_with_client` in `packages/axum-rest-api-server/src/server.rs` turns every received response into a success:

```rust
match client.get(url).send().await {
    Ok(response) => Ok(response.status().to_string()),
    Err(err) => Err(err.to_string()),
}
```

Only a transport failure (connection refused, TLS error, timeout) becomes `Err`. A REST API whose health endpoint answers
`500 Internal Server Error` is reported as healthy. The doc comment of `check_fn` says the function fails "if there
request returns an error code", which the code does not do. The HTTP tracker's `check_fn_with_client` in
`packages/axum-http-server/src/server.rs` has the identical `match`, and its doc comment makes the same claim.

### The problem in one paragraph

The check is meant to answer "is the service working?" but it only answers "did the service answer?". A service that
reports itself as unhealthy (a `500` or `503` from its health endpoint) is still reported as healthy, because the check
never looks at what it said. This can hide a real outage from anyone reading the health report. It is latent today: both
health endpoints are static handlers that always answer `200`, so nothing can fail yet. It starts to matter as soon as a
health endpoint reflects real state, such as a dependency that is down.

### Evidence

Reproduced with the regression tests only (a real `500` run is not possible, the real endpoint always answers `200`); see `manual-verification-evidence.md`. Original plan (T1): run the check against a server whose health
endpoint answers `500`, and record that the result is `Ok("500 Internal Server Error")`.

### Impact (to confirm)

The health check API treats `result.is_err()` as a failed check (`CheckReport::fail`), so a `500` currently never produces
an `Error` report. Whether the container `HEALTHCHECK` is affected is not known; #2726 found it only checks the HTTP
status of the health check API itself, so it is probably not.

## Scope

### In Scope

- Report a non-success HTTP status from the REST API health check as `Err`.
- The same change for the HTTP tracker health check, which has the same defect, so both stay consistent.
- Regression tests for a success status and a failing status in each check.
- Correct the doc comments if any claim stays untrue.

### Out of Scope

- The HTTPS probing fix (#2726, done).
- How the health check API aggregates or reports results, or the report schema.
- Health checks of other services (the UDP tracker is checked differently).

## Decision (maintainer, 2026-11-05)

Any status that is not 2xx is a failure (`response.status().is_success()`). A health endpoint says it is fine with a 2xx;
anything else means something is wrong or the check probes the wrong thing (a `404` or `401` is a misconfiguration worth
reporting). Redirects fail too, which changes nothing today because both endpoints never redirect and `reqwest` follows
redirects to the target's status. The `Err` text keeps the status, for example `Err("500 Internal Server Error")`, so the
report shows what the service said.

## Architectural Decisions

No new decision expected beyond the status rule above.

## Design and Ownership Review

Each server owns its own check, so the fix is local to the two `server.rs` files. A shared helper for the status rule would
need `torrust-server-lib` (an extracted repository); do not change it in this issue without the maintainer.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. Analysis: every response is mapped to `Ok`.
2. Reproduction with real artifacts: see the manual scenario below.
3. Regression-test boundary: see the next section.
4. Red evidence: the failing-status tests failing before the change.
5. Fix: return `Err` for a non-success status.
6. Green evidence and a like-for-like recheck of the reproduction.

## Regression Test Strategy

In each `server.rs`, using `check_fn_with_client` against a local test server (no new dependency expected; use what
the package already has):

- A health endpoint answering `200` gives `Ok`. Guards against over-correcting.
- A health endpoint answering `500` gives `Err`. Fails now.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                          | Notes / Expected Output                                           |
| --- | ------ | --------------------------------------------- | ----------------------------------------------------------------- |
| T1  | DONE   | Reproduce and record the evidence             | Reproduced by the red regression tests (no real-run reproduction). |
| T2  | DONE   | Write the failing regression tests            | `500` gives `Err`, `200` gives `Ok`, in both packages.            |
| T3  | DONE   | Return `Err` for a non-success status         | Both checks; doc comments match the behavior.                     |
| T4  | DONE   | Verify green and recheck                      | Tests green; real run with `200` shows no regression.     |

## Commit Points

Signed Conventional Commits with the issue number: one for the REST API check with its tests and one for the HTTP tracker
check with its tests (`fix(axum-rest-api-server): [#2733] ...`, `fix(axum-http-server): [#2733] ...`). A red test cannot be
committed alone because the pre-commit script runs the tests; red output goes in the evidence file.

## Progress Tracking

### Workflow Checkpoints

- [x] Draft written.
- [x] GitHub issue created (#2733).
- [x] Status rule decided by the maintainer.
- [x] Draft reviewed and approved by the maintainer (2026-11-05).
- [x] Spec merged.
- [x] Implementation merged (PR 2740).
- [x] Spec archived to `docs/issues/closed/`.

### Progress Log

- 2026-11-05 16:12 UTC - Claude Sonnet 5.5 - Draft written. The issue was created first by mistake, before this draft existed
  and was reviewed; the maintainer chose to keep it and follow the process from here.
- 2026-11-05 16:32 UTC - Claude Sonnet 5.5 - Draft approved by the maintainer. Moved to `docs/issues/open/`.
- 2026-11-05 17:37 UTC - Claude Sonnet 5.5 - Implemented on branch `2733-health-check-failing-status`: both checks return `Err(status)` for a non-2xx answer; regression tests (red then green) and the manual run are recorded in `manual-verification-evidence.md`. No dependency added. The `500` case in a real run could not be produced (see evidence); the tests cover it.
- 2026-11-05 21:55 UTC - Claude Sonnet 5.5 - PR 2740 merged by the maintainer and #2733 closed. Spec archived to `docs/issues/closed/`; no retrospective needed.

## Acceptance Criteria

- [x] AC1: A REST API health endpoint answering a failing status is reported as an error by the registered check.
- [x] AC2: The same for the HTTP tracker check.
- [x] AC3: A healthy endpoint (`200`) is still reported `Ok` in both.
- [x] AC4: Regression tests failed before the change and pass after it; the output is recorded.

## Verification Plan

### Automatic Checks

`cargo test -p torrust-tracker-axum-rest-api-server`, `cargo test -p torrust-tracker-axum-http-server`, `linter all`,
`./contrib/dev-tools/git/hooks/pre-commit.sh` and `./contrib/dev-tools/git/hooks/pre-push.sh`.

### Manual Verification Scenarios

1. Run the tracker (skill `run-tracker-locally`) with the REST API and the health check API.
2. Make the REST API health endpoint answer `500` (for example with a throwaway local server bound to the configured API
   address, since the real endpoint always answers `200`).
3. Fetch the health check API report and read the REST API entry: on `develop` it is `Ok`; after the fix it is an error.
4. Repeat with the real endpoint (`200`) and confirm no regression.

Record the commands and output in `manual-verification-evidence.md` next to this spec.

## Risks and Trade-offs

- A stricter check can make a report show `Error` where it showed `Ok` before. That is the intended change, but anyone
  parsing the report body should be told in the PR.
- Treating redirects as failures is a policy choice (see the decision above).

## References

- Noticed while fixing: #2726, `docs/issues/closed/2726-rest-api-https-health-check/ISSUE.md`
- Code: `check_fn_with_client` in `packages/axum-rest-api-server/src/server.rs` and `packages/axum-http-server/src/server.rs`;
  `CheckReport::fail` in `packages/axum-health-check-api-server/src/resources.rs`
