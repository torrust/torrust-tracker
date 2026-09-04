---
doc-type: issue
issue-type: bug
status: draft
priority: p1
epic: null
github-issue: null
spec-path: docs/issues/drafts/rest-api-constant-time-token-comparison/ISSUE.md
branch: "security/private-remediation-20260904"
related-pr: null
last-updated-utc: 2026-09-04 12:00
semantic-links:
  skill-links:
    - create-issue
    - handle-secrets
    - add-rust-dependency
  related-artifacts:
    - docs/security/vulnerability-remediation.md
    - packages/axum-rest-api-server/src/v1/middlewares/auth.rs
    - packages/axum-rest-api-server/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Use constant-time comparison for REST API access tokens

## Goal

Make REST API access-token validation independent of the token content so that response
timing cannot be used to recover a configured token, and apply the confidential
vulnerability-remediation process for the first time.

## Background

A security researcher reported through the coordinated-disclosure channel that
`authenticate` in `packages/axum-rest-api-server/src/v1/middlewares/auth.rs` compares the
caller-supplied token against each configured token with plain `==` (CWE-208, observable
timing discrepancy) and that the REST API has no rate limiting. The reporter verified the
comparison primitive by source review and explicitly did **not** demonstrate a live timing
attack; the suggested CVSS 3.1 vector is `AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:H/A:N`.

Maintainer triage:

- The primitive is as described. `str == str` short-circuits on length and on the first
  differing byte, and `Iterator::any` stops on the first matching configured token.
- **Affected versions:** every released version. The same plain comparison exists in the
  latest tag `v3.0.0-rc.1` (`src/servers/apis/v1/middlewares/auth.rs`,
  `tokens.values().any(|t| t == token)`) and in `develop` at the reported commit
  `2d972739`. There is no configuration-level workaround other than network isolation.
- Practical exploitability over a network is low: the differing-byte timing is in the
  nanosecond range and is dominated by network jitter, tokio scheduling, and the HashMap
  iteration. The REST API is an admin surface that operators are already advised to bind to
  a private interface or put behind TLS. The maintainers therefore rate the finding lower
  than the reporter's suggested CVSS (~7.4 High): remote timing recovery of a token through
  this path has not been demonstrated and is considered impractical.
- It is still a genuine hardening gap with a well-known, cheap, standard fix. Treated as a
  **low-severity hardening bug**, remediated with the **fix-with-PR** disclosure path.

Reporter credit: Abdurazzoqov Javohir — GitHub `abdurazzoqovjavohir700-dev`. The reporter
consented by email to being named in the commit and PR. The fix commit carries a
`Reported-by:` trailer with the name and email he supplied; the PR description names him with
his GitHub handle. He did not author the patch, so commit authorship stays with the
maintainer (see the credit rule in `docs/security/vulnerability-remediation.md`).

## Scope

### In Scope

- Constant-time comparison of the provided token against **every** configured token,
  without short-circuiting between tokens, using the `subtle` crate.
- Unit tests for the comparison behaviour (match, mismatch at various positions, length
  mismatch, match on a non-first configured token).
- Reporter credit in the commit trailer and the PR description.

### Out of Scope

- **Rate limiting / request throttling on the REST API.** Valid defense-in-depth, but a
  feature with its own design questions (client identity behind reverse proxies, IPv6
  prefix keying, bounded memory, operator configuration, impact on legitimate dashboards).
  To be proposed as a separate feature issue; the reporter's suggestion (`governor` /
  `tower_governor`, /64 or /56 IPv6 keying, bounded LRU eviction) is recorded there.
- Token-length leakage. `subtle`'s slice comparison still short-circuits on length; hiding
  the length would require hashing both sides first. Tokens are operator-chosen; length is
  not considered a meaningful secret for this surface.
- Removing or deprecating the `?token=` query-string authentication path (separate
  credential-in-URL/logging concern).

## Architectural Decisions

- Related ADRs: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`
  (`expose_secret()` is allowed at the immediate comparison boundary).
- ADRs to create: `None known`.

## Design and Ownership Review

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                         | Notes / Expected Output                                                                                                                |
| --- | ------ | ------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Add direct `subtle` dependency to `axum-rest-api-server`     | Latest stable (`2.6.1` at drafting time); already resolved transitively in `Cargo.lock`, so no new crate is pulled in                  |
| T2  | TODO   | Rewrite `authenticate` with `ConstantTimeEq`                 | Fold `Choice` over all configured tokens with bitwise OR; convert to `bool` once at the end; `expose_secret()` stays at the comparison |
| T3  | TODO   | Add unit tests in `auth.rs`                                  | Match; mismatch early/late/shorter/longer; empty token; match on second configured token; no secret values in assertion messages       |
| T4  | TODO   | Run quality gates and manual scenarios                       | `cargo test -p torrust-tracker-axum-rest-api-server`, `cargo machete`, pre-commit hook; M1–M3 recorded with evidence                   |
| T5  | TODO   | Draft the follow-up rate-limiting feature spec               | `docs/issues/drafts/rest-api-rate-limiting/ISSUE.md` with the reporter's design notes                                                  |
| T6  | TODO   | Review the remediation process against this case             | Feed gaps back into `docs/security/vulnerability-remediation.md`; closes T4 of the companion process spec                              |
| T7  | TODO   | Disclose: create both issues, move specs to `open/`, open PR | PR body credits the reporter; `Closes` both issues; notify the reporter with the PR link                                               |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/rest-api-constant-time-token-comparison/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec (deferred to disclosure moment)
- [ ] Implementation completed
- [x] Automatic verification completed (`linter all`, documentation baseline only)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-04 12:00 UTC - Copilot - Report triaged; scope agreed with maintainer; spec drafted
  on the local (unpushed) branch. GitHub issue deferred until the disclosure moment.
- 2026-09-04 12:00 UTC - Copilot - `linter all` passed for the documentation and draft-spec
  baseline. Implementation checks remain pending.

## Acceptance Criteria

- [ ] AC1: `authenticate` compares the provided token with every configured token using
      `subtle::ConstantTimeEq` and does not short-circuit between configured tokens.
- [ ] AC2: Valid tokens (any configured entry) are accepted; tokens differing at any byte or in
      length are rejected — covered by unit tests.
- [ ] AC3: Existing contract tests in `tests/server/v1/contract/authentication.rs` still pass
      (bearer header, query param, precedence, empty/invalid/missing token).
- [ ] AC4: No test, log, or error message exposes a token value.
- [ ] AC5: Reporter is credited with a `Reported-by:` trailer on the fix commit and by name and
      GitHub handle in the PR description.
- [ ] AC6: The remediation process document has been reviewed against this case and any gap
      found is fixed on the same branch.
- [ ] `linter all` exits with code `0`
- [ ] `cargo machete` reports no unused dependencies

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-axum-rest-api-server`
- `cargo machete`
- `./contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

Run the tracker with the development configuration
(`share/default/config/tracker.development.sqlite3.toml`, API on `0.0.0.0:1212`, token label
`admin`). The development token is a public fixture, not a secret, but it is still redacted here
so the spec does not double as a copy-paste credential.

| ID  | Scenario                           | Command/Steps                                                                                                     | Expected Result                                                      | Status | Evidence |
| --- | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ------ | -------- |
| M1  | Valid token accepted (header)      | `curl -s -o /dev/null -w '%{http_code}' -H "Authorization: Bearer {REDACTED}" http://127.0.0.1:1212/api/v1/stats` | `200`                                                                | TODO   |          |
| M2  | Valid token accepted (query param) | `curl -s -o /dev/null -w '%{http_code}' "http://127.0.0.1:1212/api/v1/stats?token={REDACTED}"`                    | `200`                                                                | TODO   |          |
| M3  | Same-length wrong token rejected   | M1 with the last character of the token changed                                                                   | `500`, body `Unhandled rejection: Err { reason: "token not valid" }` | TODO   |          |

No wall-clock timing test is included: it would be noisy and hardware-dependent and would not
prove the property. Correctness is by construction (use of `subtle`) plus behavioural tests.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- **Fix-with-PR disclosure exposes the finding before a release.** Accepted because the
  finding is a hardening gap with no demonstrated exploit and low practical exploitability;
  the reporter agreed to follow the project's timeline. All released versions remain
  affected until the next release; the PR description must state the interim mitigation
  (bind the API to a private interface / restrict by firewall).
- **Severity disagreement with the reporter.** The maintainers rate this lower than the
  suggested CVSS 7.4. If the reporter or a later reviewer demonstrates a practical remote
  timing recovery, the disclosure path must be re-evaluated before the PR is opened, and a
  CVE/advisory considered.
- **Compiler optimisation could in theory undo constant-time behaviour.** `subtle` uses
  black-box barriers to prevent this; it is the standard crate for this purpose in the Rust
  ecosystem (used by RustCrypto).
- **Marginal per-request cost** from comparing against every token instead of stopping at the
  first match. Negligible: operators configure a handful of tokens.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Companion process spec:
  `docs/issues/drafts/define-confidential-vulnerability-remediation-process/ISSUE.md`
- Follow-up feature (to draft): `docs/issues/drafts/rest-api-rate-limiting/ISSUE.md`
- CWE-208: Observable Timing Discrepancy
- `subtle` crate: <https://docs.rs/subtle>
