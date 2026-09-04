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

Give REST API access-token comparison a constant-time guarantee **by contract** rather than
by accident of the current libc, so that for a fixed supplied-token length the comparison
cost does not depend on token content or on which configured token matches. Apply the
confidential vulnerability-remediation process for the first time and feed what it exposed
back into that process.

## Classification

**Low-severity hardening (CWE-208 class). Not a confirmed vulnerability.** No CVE, no
security advisory. See "Maintainer triage" for the evidence behind this classification.

## Background

### The report

A security researcher reported through the coordinated-disclosure channel that
`authenticate` in `packages/axum-rest-api-server/src/v1/middlewares/auth.rs` compares the
caller-supplied token against each configured token with plain `==` and that the REST API
has no rate limiting. The reporter stated that he verified the comparison primitive by source
review only and explicitly did **not** measure a timing difference. He suggested CVSS 3.1
`AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:H/A:N` (~7.4 High) and the `subtle` crate as the fix.

### The attack, in plain words

**What a normal comparison does.** When a program checks whether two strings are equal, the
obvious implementation walks both strings from the first character and **stops at the first
character that differs**. If the strings are the same length, comparing `aaaa-aaaa` against
`baaa-aaaa` stops after one character; comparing it against `aaaa-aaab` walks all nine. The
"wrong" answer comes back a tiny bit faster when the mismatch is early and a tiny bit slower
when it is late. That difference in time is the leak.

**Why that turns brute force into something much cheaper.** Suppose the token is 32
characters from an alphabet of 62 symbols. Guessing the whole token blind means $62^{32}$
attempts — impossible. But if the attacker can _tell how many leading characters were right_
by timing the response, the game changes completely:

1. Send 62 requests, one per possible first character, keeping the rest fixed. The one whose
   response is slightly slower had the first character right.
2. Fix that character. Send 62 requests varying only the second character. Again the slowest
   one is correct.
3. Repeat for each position.

That is $62 \times 32 \approx 2\,000$ requests instead of $62^{32}$. The token is recovered
character by character, each position independently, because the comparison itself tells the
attacker how far it got before giving up. This is CWE-208 (observable timing discrepancy) and
is the same class of bug that has broken HMAC verification in web frameworks in the past.

**Where rate limiting comes in.** The timing difference per request is tiny (nanoseconds), so
a real attacker cannot trust a single measurement; they must send each candidate hundreds or
thousands of times and average out network noise. Rate limiting does not fix the leak, but it
makes the averaging slow or impractical — that is why the reporter called it
defense-in-depth. It is not required for the attack to exist; it changes how long it takes.

**Why a different `==` fixes it.** A constant-time comparison does not stop at the first
mismatch. It XORs every pair of bytes, ORs all the results together, and only looks at the
final accumulated value at the end. Whether the mismatch is at position 0 or position 31, the
same number of operations run and the same amount of time passes. The response no longer
carries any information about _how many_ characters were right — only _whether_ they all
were, which the attacker already learns from the 200/500 status. `subtle` additionally
prevents the compiler from "optimizing" the loop back into an early exit. Our implementation
also compares against **every** configured token rather than stopping at the first match, so
the attacker cannot learn which of several tokens they are closest to either.

**Why we still classify this as low severity.** The attack above assumes the time difference
is _observable_. Our measurement (`reproduction.md`) shows that on x86-64 with glibc there is
no measurable difference at all for tokens of this size: the C library compares 32 bytes with
a couple of wide SIMD instructions rather than a byte loop, so step 1 of the attack has
nothing to measure. Even if a difference existed, it would be sub-nanosecond and buried under
tens of microseconds of network jitter, tokio scheduling, and TLS. We fix it anyway because
that safety depends on the libc and CPU, not on our code, and the fix is cheap.

### Maintainer triage

**1. Code path — confirmed.** The primitive is as described:

```rust
tokens.values().any(|configured_token| configured_token.expose_secret() == token)
```

`str == str` lowers to a length check followed by `memcmp`; `Iterator::any` stops at the first
matching configured token. The same comparison exists in the latest tag `v3.0.0-rc.1`
(`src/servers/apis/v1/middlewares/auth.rs`, `tokens.values().any(|t| t == token)`) and in
`develop` at the reported commit `2d972739`. Every released version carries it.

**2. Independent reproduction — negative.** A disposable micro-benchmark (see
[`reproduction.md`](reproduction.md)) compared a 32-byte and a 128-byte secret against
candidates whose first wrong byte sat at positions 0 … N−1 and N (exact match), 2 000 000
iterations each, `--release`, x86-64, glibc.

| Comparison            | 32-byte token, all positions | 128-byte token, all positions |
| --------------------- | ---------------------------- | ----------------------------- |
| plain `==` (current)  | 1.75 – 1.90 ns/op            | 1.74 – 2.59 ns/op             |
| `subtle::ct_eq` (fix) | 31.4 – 33.7 ns/op            | 120.5 – 122.7 ns/op           |

Plain `==` showed **no monotonic relationship** between the position of the first differing
byte and the elapsed time: position 0 and position 31 differ by 0.05 ns, inside noise. On
this platform glibc's `memcmp` handles inputs of this size with wide SIMD compares, so there
is nothing position-dependent to observe even in-process — before adding tokio scheduling,
HTTP parsing, TLS, and network jitter (tens of microseconds) on top.

**3. Why fix it anyway.** The negative result is a property of _one_ compiler + libc + CPU,
not of our source. Neither the C standard nor glibc promises constant-time `memcmp`; musl
(Alpine images), aarch64, a different glibc release, or a future LLVM that inlines the
compare as a byte loop can change the behaviour with no change to our code. Relying on an
undocumented libc characteristic for a security property is not something we can write down
in an ADR without it reading as an excuse. Constant-time comparison of a secret is a textbook
baseline; declining it means re-triaging the same line every time a scanner, reviewer, or
auditor flags it. The fix costs ~30 ns on an admin endpoint that serves a handful of
requests per second.

**4. Severity — set by maintainers.** The reporter's ~7.4 High is not supported: `C:H/I:H`
assumes token recovery, which requires an observable timing signal that neither the reporter
nor we could produce. We classify the finding as **low-severity hardening**. The reporter's
identification of a non-contractual secret comparison is nevertheless correct and useful.

**5. Suggested fix vetted as untrusted input.** The reporter proposed the `subtle` crate.
Alternatives considered:

- _Do nothing_ — rejected for the reasons in point 3.
- _std-only XOR-accumulate loop with `std::hint::black_box`_ — rejected: `black_box` is
  documented as a hint with no security guarantee, hand-rolled constant-time code is exactly
  what reviewers distrust, and since `subtle` is already compiled into the binary (below) the
  "avoid a dependency" benefit is illusory.
- _`subtle` crate_ — **adopted**. Vetting results:
  - Already resolved transitively in `Cargo.lock` at the identical version and checksum
    (`2.6.1`, `13c2bdde…3292`) via `sqlx-mysql → rsa` and `digest → hmac/hkdf`. Adding a
    direct dependency introduces **zero new code** into the binary.
  - Maintained by `dalek-cryptography` (RustCrypto ecosystem), BSD-3-Clause, **zero**
    dependencies, single ~1 000-line `lib.rs`, optimisation barrier via a volatile read.
    Latest stable is `2.6.1` (`cargo search`).
  - `cargo deny check advisories` / `cargo audit`: no advisory for `subtle`. Both commands
    **do** fail on the pre-existing, unrelated `rsa 0.9.10` RUSTSEC-2023-0071 (Marvin
    attack) pulled in by `sqlx-mysql`; that is public, not caused by this change, and is
    tracked separately (see References).

**6. Disclosure path — fix-with-PR.** With no demonstrated exploit and no observable signal,
there is no exposure window to protect; publishing the PR is the disclosure. The reporter
agreed to follow the project's timeline.

### Reporter credit

Abdurazzoqov Javohir — GitHub `abdurazzoqovjavohir700-dev`. The reporter consented by email
to being named in the commit and PR. The fix commit carries a `Reported-by:` trailer with the
name and email he supplied; the PR description names him with his GitHub handle. He did not
author the patch, so commit authorship stays with the maintainer (see the credit rule in
`docs/security/vulnerability-remediation.md`).

### What the first case taught the process

The first pass at this remediation applied the reporter's suggested crate **before**
attempting reproduction or vetting the dependency. The maintainer caught it. Three steps were
added to `docs/security/vulnerability-remediation.md` step 2 as a result: mandatory
independent reproduction (negative results recorded and reclassified as hardening),
maintainer-set severity, and treating suggested fixes and named dependencies as untrusted
input. This spec is written the way the amended process now requires.

## Scope

### In Scope

- Constant-time comparison of the provided token against **every** configured token,
  without short-circuiting between tokens, using the `subtle` crate. The `fold` structure,
  rather than a timing test, verifies that all configured tokens are evaluated.
- Unit tests for the comparison behaviour (match, mismatch at various positions, length
  mismatch, empty token, match on a non-first configured token).
- The reproduction evidence artifact (`reproduction.md`) so the negative result is auditable.
- Reporter credit in the commit trailer and the PR description.
- The process amendment described above (shipped in the same PR as a separate commit).

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

| ID  | Status | Task                                                         | Notes / Expected Output                                                                                                                                                     |
| --- | ------ | ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T0  | DONE   | Independent reproduction and dependency vetting              | Negative timing result recorded in `reproduction.md`; `subtle` vetted (already in lockfile, zero deps, no advisories); std-only alternative rejected with rationale         |
| T1  | DONE   | Add direct `subtle` dependency to `axum-rest-api-server`     | Latest stable (`2.6.1`); already resolved transitively in `Cargo.lock` at the same checksum, so no new code enters the binary                                               |
| T2  | DONE   | Rewrite `authenticate` with `ConstantTimeEq`                 | Fold `Choice` over all configured tokens with bitwise OR; convert to `bool` once at the end; `expose_secret()` stays at the comparison                                      |
| T3  | DONE   | Add unit tests in `auth.rs`                                  | Match; mismatch early/late/shorter/longer; empty token; match on second configured token; no secret values in assertion messages. Structural review verifies full iteration |
| T4  | DONE   | Run quality gates and manual scenarios                       | Focused format, Clippy, tests, dependency analysis, and M1–M3 passed; full pre-commit runs at commit time                                                                   |
| T5  | DONE   | Review the remediation process against this case             | Added: first-case coupling rule, explicit disclosure-path decision, mandatory reproduction, maintainer-set severity, suggested-fix vetting. Closes T4 of the process spec   |
| T6  | TODO   | Disclose: create both issues, move specs to `open/`, open PR | PR body credits the reporter and states the hardening classification; `Closes` both issues; notify the reporter with the PR link                                            |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/rest-api-constant-time-token-comparison/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec (deferred to disclosure moment)
- [x] Implementation completed
- [x] Automatic verification completed (`cargo fmt --check`, Clippy, focused tests, `cargo machete`, documentation lint baseline)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-04 12:00 UTC - Copilot - Report triaged; scope agreed with maintainer; spec drafted
  on the local (unpushed) branch. GitHub issue deferred until the disclosure moment.
- 2026-09-04 12:00 UTC - Copilot - `linter all` passed for the documentation and draft-spec
  baseline. Implementation checks remain pending.
- 2026-09-04 12:00 UTC - Maintainer - Approved the issue scope: rate limiting, token-length
  leakage, and query-token deprecation remain separate follow-up work.
- 2026-09-04 12:00 UTC - Copilot - Implemented the `Choice` fold, added authentication unit
  tests, and performed a structural security review. `cargo fmt --check`,
  `cargo clippy -p torrust-tracker-axum-rest-api-server --all-targets -- -D warnings`, and
  `cargo test -p torrust-tracker-axum-rest-api-server` passed (4 unit tests; 58 integration
  tests). Plain `cargo machete` reported two pre-existing unrelated false positives;
  `cargo machete --with-metadata` passes in the repository pre-commit hook.
- 2026-09-04 12:00 UTC - Copilot - Manual verification passed: valid header and query tokens
  each returned `200`; a same-length invalid header token returned `500` with the expected
  rejection body. Complexity audit passed for all changed functions.
- 2026-09-04 12:00 UTC - Maintainer - **Process failure caught before commit.** The fix had
  been implemented on the reporter's suggestion without independent reproduction or
  dependency vetting. Commit withheld.
- 2026-09-04 12:00 UTC - Copilot - Reproduction attempted (`reproduction.md`): no observable
  position-dependent timing in plain `==` on x86-64/glibc; `subtle` flat at ~30–120 ns.
  `subtle` vetted: already in lockfile at identical checksum via `sqlx`/`rsa`/`digest`, zero
  deps, ~1 000 LOC, dalek-cryptography, no advisories. `cargo audit`/`cargo deny` fail only on
  pre-existing `rsa` RUSTSEC-2023-0071 (unrelated; follow-up drafted). Finding reclassified
  from vulnerability to low-severity hardening; the reporter's CVSS 7.4 is not supported.
- 2026-09-04 12:00 UTC - Maintainer - Decision: apply the hardening (contract over libc
  coincidence; zero new code; ~30 ns cost) with the reclassified severity. Process amended
  with three mandatory triage steps. Background rewritten around the evidence.

## Acceptance Criteria

- [x] AC1: For a fixed supplied-token length, `authenticate` compares the provided token with
      every configured token using `subtle::ConstantTimeEq` and does not short-circuit on token
      content or between configured tokens, verified by structural review of the `Choice` fold.
- [x] AC2: Valid tokens (any configured entry) are accepted; tokens differing at any byte or in
      length are rejected — covered by unit tests.
- [x] AC3: Existing contract tests in `tests/server/v1/contract/authentication.rs` still pass
      (bearer header, query param, precedence, empty/invalid/missing token).
- [x] AC4: No test, log, or error message exposes a token value.
- [ ] AC5: Reporter is credited with a `Reported-by:` trailer on the fix commit and by name and
      GitHub handle in the PR description.
- [x] AC6: The remediation process document has been reviewed against this case and any gap
      found is fixed on the same branch.
- [x] AC7: The public record classifies the finding as low-severity hardening, not a
      confirmed vulnerability, and includes the negative reproduction evidence and the
      dependency vetting result.
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

| ID  | Scenario                           | Command/Steps                                                                                                     | Expected Result                                                      | Status | Evidence                                        |
| --- | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ------ | ----------------------------------------------- |
| M1  | Valid token accepted (header)      | `curl -s -o /dev/null -w '%{http_code}' -H "Authorization: Bearer {REDACTED}" http://127.0.0.1:1212/api/v1/stats` | `200`                                                                | DONE   | Local tracker returned `200`                    |
| M2  | Valid token accepted (query param) | `curl -s -o /dev/null -w '%{http_code}' "http://127.0.0.1:1212/api/v1/stats?token={REDACTED}"`                    | `200`                                                                | DONE   | Local tracker returned `200`                    |
| M3  | Same-length wrong token rejected   | M1 with the last character of the token changed                                                                   | `500`, body `Unhandled rejection: Err { reason: "token not valid" }` | DONE   | Local tracker returned expected status and body |

No wall-clock timing test is included in the test suite: it would be noisy and
hardware-dependent and would not prove the property. Correctness is by construction (use of
`subtle`) plus behavioural tests. The one-off benchmark in `reproduction.md` is triage
evidence, not a regression test.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                    |
| ----- | ---------------------- | --------------------------------------------------------------------------- |
| AC1   | DONE                   | Structural review of `Choice` fold in `auth.rs`                             |
| AC2   | DONE                   | Four unit tests in `auth.rs`                                                |
| AC3   | DONE                   | `cargo test -p torrust-tracker-axum-rest-api-server` (58 integration tests) |
| AC4   | DONE                   | Review of changed production and test code                                  |
| AC5   | TODO                   |                                                                             |
| AC6   | DONE                   | Process review changes in `docs/security/vulnerability-remediation.md`      |
| AC7   | DONE                   | "Classification" and "Maintainer triage" sections; `reproduction.md`        |

## Risks and Trade-offs

- **Fix-with-PR disclosure publishes the finding before a release.** Accepted: no timing
  signal was reproducible, so there is no exposure window to protect. The PR description
  states the classification (hardening, not vulnerability) so downstream users do not
  over-react, and the standing operational advice (bind the API to a private interface or
  behind a reverse proxy).
- **Severity disagreement with the reporter.** The maintainers rate this as low-severity
  hardening; the reporter suggested CVSS 7.4. If anyone later demonstrates a practical remote
  timing recovery on any supported platform, reopen: re-evaluate severity, consider a
  CVE/advisory, and check whether the deployed fix already covers it.
- **The change was prompted by an external report we could not validate.** Mitigated by
  recording the _actual_ reason for adopting it (no constant-time contract from libc; zero
  new code; standard baseline) rather than "reporter said so", and by the process amendment
  that makes reproduction and vetting mandatory before remediation.
- **Supply-chain risk from a reporter-suggested crate.** Nil for this crate: `subtle` is
  already compiled into the binary via `sqlx`, at the identical version and checksum. The
  vetting was still performed and recorded because the _process_ must not depend on luck.
- **Compiler optimisation could in theory undo constant-time behaviour.** `subtle` uses a
  volatile-read barrier to prevent this and is the standard crate for the purpose in the Rust
  ecosystem (RustCrypto).
- **Per-request cost** from `subtle` (~30 ns for a 32-byte token) and from comparing against
  every configured token instead of stopping at the first match. Negligible on an admin
  endpoint with a handful of tokens.
- **The negative benchmark could be misread as "nothing to fix".** `reproduction.md` states
  its limitations (one platform, no musl/aarch64) and why the hardening is still applied.

## Implementation Completion Review

- Retrospective: **assessed; material discovery recorded inline.** The material finding is
  the process failure: remediation was implemented before reproduction and vetting. It is
  documented in "What the first case taught the process", the progress log, and as concrete
  amendments to `docs/security/vulnerability-remediation.md` step 2. A separate
  `implementation-retrospective.md` would duplicate that content and is not created.

## References

- Reproduction evidence: [`reproduction.md`](reproduction.md)
- Companion process spec:
  `docs/issues/drafts/define-confidential-vulnerability-remediation-process/ISSUE.md`
- Follow-up feature (to draft): `docs/issues/drafts/rest-api-rate-limiting/ISSUE.md`
- Follow-up advisory triage (to draft): `rsa 0.9.10` RUSTSEC-2023-0071 via `sqlx-mysql`,
  found during dependency vetting; public, unrelated to this change
- CWE-208: Observable Timing Discrepancy
- `subtle` crate: <https://docs.rs/subtle> — <https://github.com/dalek-cryptography/subtle>
