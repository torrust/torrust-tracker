---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2751-1488-si-18-deprecate-legacy-shutdown-api/ISSUE.md
last-updated-utc: "2026-11-09 10:55"
---

# Manual Verification Evidence: Deprecate Legacy Shutdown API

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 10:50 to 10:52 (V1 rerun for the review of PR 2752)
- Artifact under test: the specification branch of PR 2752 (the specification only; no code changes yet)
- Operating system / environment: Linux 7.0, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
- Prerequisites and setup performed: a checkout of the branch; network access to crates.io

## Verification Processes

### V1 - Eligibility gate

- Goal: show that every eligibility-gate item of the issue holds.
- Initial state: branch of PR 2752 (the code is that of `torrust/develop` at `da8bd15d1`, which the
  branch was cut from).
- Status: `DONE`

#### Steps Performed

1. `grep -n "start_with_cancellation" src/bootstrap/jobs/*.rs`: the application components.
2. `grep -n '#\[cfg(test)\]' <file>` for the files holding legacy `start`/`stop`/`Halted::Normal` calls, and
   `grep -n "\.stop()\.await\|started.stop\|Halted::Normal"` over the same files: where those calls sit.
3. `git grep -q "HaltSignal\|NestedServerTask" v3.0.0; echo $?`: the root types against the published tag.
4. `curl -s https://crates.io/api/v1/crates/torrust-server-lib`, then listing the first two versions.
5. Reading the cited documents (links below).

#### Observed Result

```text
$ grep -n "start_with_cancellation" src/bootstrap/jobs/*.rs
src/bootstrap/jobs/http_tracker.rs:124:    .start_with_cancellation(http_tracker_container, form, metadata, cancellation_token.clone())
src/bootstrap/jobs/health_check_api.rs:52:    let server = server::start_with_cancellation(
src/bootstrap/jobs/tracker_apis.rs:132:        .start_with_cancellation(http_api_container, form, metadata, access_tokens, cancellation_token.clone())
src/bootstrap/jobs/udp_tracker.rs:72:        .start_with_cancellation(

$ first '#[cfg(test)]' line per file (the legacy calls found by step 2 sit after it)
packages/axum-http-server/src/server.rs: 528     legacy .stop() call at line 832
packages/axum-rest-api-server/src/server.rs: 521 legacy .start( at 599, .stop() at 607
packages/udp-server/src/server/mod.rs: 68        legacy .start( at 127, 176, 230; .stop() at 138, 189
src/bootstrap/jobs/manager.rs: 1073              HaltSignal and NestedServerTask users in the test module
(the remaining `Halted::Normal` hits, at server.rs lines 209, 357 and 474, are the legacy adapter
implementations themselves, not callers)

$ git grep -q "HaltSignal\|NestedServerTask" v3.0.0; echo "git grep exit=$?"
git grep exit=1

$ crates.io torrust-server-lib, first two versions
[('0.5.0', '2026-11-06'), ('0.4.0', '2026-10-26')]
```

#### Conclusion

Every gate item holds:

- Application components: the four bootstrap jobs call `start_with_cancellation` (step 1).
- Remaining legacy callers are test modules or the legacy adapter implementations (step 2).
- The root `HaltSignal` and `NestedServerTask` are not in tag `v3.0.0` (step 3, exit 1 means no match; they
  first appear in commit `3ee28b723`).
- `torrust-server-lib` 0.5.0 is published (step 4); its `CHANGELOG.md` gives the migration steps and the
  support window (items stay for all 0.5.x, removed in 0.6.0).
- Linked records for the migrations and Q5, from the repository root:
  - Standalone environments and examples: `docs/issues/closed/2412-1488-si-16-migrate-standalone-http-environment/ISSUE.md`
    and `docs/issues/closed/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md`.
  - Test environments: `docs/issues/closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md` and
    `docs/issues/closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md`.
  - Task inventory: `docs/features/shutdown-process/task-inventory.md`, finding 4.
  - Q5: `docs/features/shutdown-process/questions.md` (Resolved) and `docs/containers.md#stopping-the-tracker`.
- External: torrust-server-lib issue 3 and PR 4 (merge commit `0ad8ca0`) at
  `https://github.com/torrust/torrust-server-lib/pull/4`.

## Failures and Follow-up

None. V2 to V4 (see the issue's Manual Verification Scenarios) run after implementation.
