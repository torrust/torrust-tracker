---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: null
github-issue: 2984
spec-path: docs/issues/open/2984-tracker-client-check-udp-url-validation/ISSUE.md
branch: "2984-tracker-client-check-udp-url-validation-spec"
related-pr: null
last-updated-utc: "2026-11-21 13:15"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md
---

<!-- skill-link: create-issue -->

# Issue #2984 - `tracker_client check` panics on a UDP tracker URL without a usable address

## Goal

`tracker_client check` rejects a UDP tracker URL that has no host or no port as a usage/configuration error (exit `2`, one JSON error record), and never panics on a UDP tracker address it cannot resolve.

## Background

`tracker_client check` reads a configuration (`TORRUST_CHECKER_CONFIG` or a file) with three URL lists: `udp_trackers`, `http_trackers` and `health_checks`. Configuration validation (`TryFrom<PlainConfiguration> for Configuration`, `console/tracker-client/src/console/clients/checker/config.rs`) only checks that each entry parses as a URL. For `udp_trackers` it first prepends `udp://` when the entry does not start with it.

What happens, step by step:

1. The operator passes `{"udp_trackers":["udp://127.0.0.1"],...}`.
2. Validation accepts it: `udp://127.0.0.1` is a syntactically valid URL.
3. The UDP check calls `resolve_socket_addr` (`console/tracker-client/src/console/clients/checker/checks/udp.rs`, `url.socket_addrs(|| None).unwrap()`). `socket_addrs` returns `Err` ("No port number in the URL") because UDP has no default port.
4. The `unwrap` panics inside a spawned task. The program prints a `panic` record, then a `runtime_failure` record, and exits `1`.

The same `unwrap` panics for a URL without a host (`udp:///`: "No host name in the URL"), for an entry with another scheme such as `http://127.0.0.1` or `ftp://127.0.0.1:6969` (the `udp://` prefix is added in front of it, so the host becomes `ftp`/`http` and there is no port), and for a host that does not resolve (`udp://nonexistent.invalid:6969`: "failed to lookup address information").

Violated expectation: the [CLI output contract ADR](../../../adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md) keeps exit codes to `0`, `1` and `2`, where `2` is a usage/configuration error. A bad address in the configuration is exactly that. Other invalid URLs already do it: `"invalid URL"` and `udp://[::1` give exit `2` with an `invalid_configuration` record. The panic is also a code-quality problem on its own (an `unwrap` on user input).

Impact: no data is lost and nothing wrong is reported as success, but the operator gets a panic report and a generic `runtime_failure` (exit `1`) for a typo, instead of an error that names the offending entry. Since issue #2768 the panic is at least a JSON record on stderr, so machine consumers see a failure; the bug is the wrong kind of error and the panic.

This bug is not part of EPIC #2758 (CLI output contract migration). It was found while fixing issue #2768 (PR #2956).

## Scope

### In Scope

- Reject, during configuration validation, a `udp_trackers` entry that has no host, no port, or a non-`udp` scheme, with a new `ConfigurationError` variant that names the entry and reason; surfaces as the existing `invalid_configuration` record with exit `2`.
- Remove the `unwrap`s in `resolve_socket_addr`: a host that cannot be resolved at run time (DNS failure) becomes a failed check result for that tracker, not a panic and not a whole-run failure. The result record needs a way to represent an address that did not resolve, because `udp::Checks` carries `remote_addr: SocketAddr` (`checks/udp.rs:15`); the representation is part of Open Question 1.
- Regression tests at the configuration seam (unit) and a real-artifact recheck.

### Out of Scope

- `http_trackers` and `health_checks` URLs: HTTP(S) has default ports, so a missing port is valid. Validating their scheme or host is a separate change, only raised here for the maintainer to decide (see Open Questions).
- Any other change to the output contract or exit codes, apart from the result-shape change that Open Question 1 may require for an unresolved host.

## Open Questions (for the maintainer)

1. Is an unresolvable hostname a configuration error or a per-tracker failed check? The options:
   - **Configuration error** (exit `2`): resolve at validation time. No result-shape change, but validation then needs the network and a typo-free configuration can still fail on a transient DNS error.
   - **Failed check** (proposed: DNS is a network condition, not a syntax error, and the check's purpose is to report on tracker health): needs a representation for an address that did not resolve in `udp::Checks`, whose `remote_addr: SocketAddr` cannot hold it. Candidates: make `remote_addr` an `Option<SocketAddr>` and add the configured URL to the record, or keep `SocketAddr` and use a placeholder. This is a change to the result record, so this option widens the scope beyond what Out of Scope otherwise allows; the implementation PR must show the new record.
2. Should the same host/scheme validation apply to `http_trackers` and `health_checks` in this issue, or stay out of scope?

## Architectural Decisions

- Related ADRs: `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
- ADRs to create: None known

## Design and Ownership Review

Not applicable

## Bug-Fix Process

Follows `.claude/skills/fix-bug/SKILL.md`:

- Analysis: `resolve_socket_addr` unwraps a `Result` and an `Option` that depend on user input; validation never checks host or port for UDP URLs. Hypothesis: add the check at the validation seam, and make the run-time resolution fallible.
- Reproduction: Reproduced on `torrust/develop` (see `manual-verification-evidence.md`, V1).
- Regression-test boundary: unit tests in `config.rs` (see below).
- Red test evidence before the fix, production fix, green evidence, and a like-for-like recheck of V1 after the last production change.

## Regression Test Strategy

Unit tests at the causal seam, `TryFrom<PlainConfiguration> for Configuration` in `config.rs`, one per case: no port, no host, non-`udp` scheme. The scheme check reads the raw entry before the `udp://` prefix is added (`config.rs:65`); without that, `ftp://127.0.0.1:6969` becomes `udp://ftp://127.0.0.1:6969`, whose host is `ftp` with no port, and the no-port check would reject it. The scheme test therefore asserts the error reason (wrong scheme), not only that an error is returned. A unit test for `resolve_socket_addr` returning an error (not panicking) for an unresolvable host. The configuration cases are deterministic and need no network. The unresolvable-host case goes through the system resolver, so it only asserts that an error is returned, not which one; it does not claim to be network-free (a resolver that rewrites "name not found" answers could resolve a `.invalid` name), unless the implementation injects the resolution step, which it may do instead. Red/green output goes to `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                        | Notes / Expected Output                                           |
| --- | ------ | --------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| T1  | DONE   | Reproduce on a real artifact and classify                                   | V1 recorded: Reproduced                                           |
| T2  | TODO   | Add regression tests (config validation cases, unresolvable-host case)      | Tests fail on current code; red output recorded                   |
| T3  | TODO   | Fix: validate UDP URLs in config; make `resolve_socket_addr` fallible       | Regression tests pass; no `unwrap` on user input in `udp.rs`     |
| T4  | TODO   | Green run and like-for-like recheck of V1 after the last production change  | V1 commands give exit `2` + `invalid_configuration`, no panic     |

## Commit Points

| Task | Coherent change set                                   | Commit policy                                        |
| ---- | ----------------------------------------------------- | ---------------------------------------------------- |
| T2   | Regression tests (red)                                | Commit after focused validation and required review. |
| T3   | Validation in config and fallible resolution          | Commit after focused validation and required review. |
| T4   | Evidence update                                       | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2984-tracker-client-check-udp-url-validation/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-19 16:06 UTC - Claude Code (Claude Sonnet 5.5) - Drafted the spec; reproduced the bug on `torrust/develop` - `manual-verification-evidence.md`
- 2026-11-21 13:15 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; issue #2984 created; spec moved to open/

## Acceptance Criteria

- [ ] AC1: `udp://127.0.0.1` (no port) in `udp_trackers` gives exit `2` and one `invalid_configuration` JSON record naming the entry; no `panic` record
- [ ] AC2: `udp:///` (no host) and `http://127.0.0.1` / `ftp://127.0.0.1:6969` (wrong scheme) in `udp_trackers` give the same outcome
- [ ] AC3: `udp://nonexistent.invalid:6969` does not panic; it is reported as a failed check for that tracker
- [ ] AC4: Valid entries (`udp://127.0.0.1:6969`, `127.0.0.1:6969`, `udp://localhost:6969`) behave as before
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-client`
- Pre-push checks (when applicable)

### Manual Verification Scenarios

| ID  | Scenario                                            | Human-oriented command/steps                                                                                                                      | Expected Result                                        | Status | Evidence                                      |
| --- | --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ | ------ | --------------------------------------------- |
| M1  | UDP URL without port, host, with wrong scheme, DNS miss | `TORRUST_CHECKER_CONFIG='{"udp_trackers":["<url>"],"http_trackers":[],"health_checks":[]}' ./target/debug/tracker_client check \| cat` for each URL | Before: `panic` + `runtime_failure`, exit 1. After: see AC1-AC3 | DONE (before) / TODO (after) | `manual-verification-evidence.md` section V1 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Create the final-recheck entry only after the last production change (see `fix-bug`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |

## Risks and Trade-offs

- Rejecting a non-`udp` scheme in `udp_trackers` could break a configuration that worked by accident; no such entry can work today, because every one panics.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Found during: issue #2768 (PR #2956)
- Related ADR: `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`
