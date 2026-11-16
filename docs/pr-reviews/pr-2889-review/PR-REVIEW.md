---
pr-number: 2889
pr-url: https://github.com/torrust/torrust-tracker/pull/2889
last-updated-utc: "2026-11-16 17:51"
---

<!-- skill-link: process-pr-review -->

# PR #2889 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2889.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1, on the head that carries the commit `docs(issues): [#2764] record the evidence and the spec progress`: da2ce7 review 5924341133 (APPROVED) with findings F1 to F9 (five Minor, three Nit, one Suggestion), and Copilot review 5924435062 (COMMENTED) with three findings. Copilot's own IDs F1, F4 and F5 collide with da2ce7's, so they are audit findings F10, F11 and F12; F11 and F12 re-raise F2 and F1.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2889-f1 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2889-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2889-f3 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2889-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2889-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2889-f6 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2889-f7 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2889-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2889-f9 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2889-f10 | Copilot | Major | security | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2889-f11 | Copilot | Minor | correctness | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F12 | review-finding:pr-2889-f12 | Copilot | Minor | testing | RE_RAISE_OF:F1 | FIXED | RESOLVED |

## Finding Details

### F1 - Released port probed and expected dead

- PR number: 2889
- Source review ID: 5924341133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589083422
- Concern: The unreachable test bound port 0, dropped the listener and expected nothing to answer on the released port; a concurrent port-0 bind could take the port and answer.
- Solution: The test holds a `tokio::net::TcpSocket` bound to `127.0.0.1:0` without listening for the whole probe and drops it after the child exits, as `packages/axum-rest-api-server/src/server.rs` does. The spec's Design and Ownership Review and Regression Test Strategy now describe that form.
- Current-tree verification: `tests/http_health_check.rs` builds the port with `hold_a_port_that_refuses_connections()`; `cargo test --test http_health_check` and `cargo nextest run --test http_health_check` report 10 passed.
- Resolution reference: `test(health-check): [#2764] hold the unreachable port bound without listening`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589182576

### F2 - Non-UTF-8 argument panics; malformed URL is not a usage error

- PR number: 2889
- Source review ID: 5924341133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589083437
- Concern: `env::args()` panics on a non-UTF-8 argument (plain text, exit 101); the reviewer also asked whether an unparsable URL is a usage error.
- Solution: The binary reads `env::args_os()`; a non-UTF-8 argument and an argument that `Url::parse` rejects are `usage_error` records with exit 2. New tests cover both.
- Current-tree verification: Against the previous binary the two new tests failed with exit 101 (panic text on stderr) and exit 1 (`unreachable`); on the current tree they pass. Manual run: `http_health_check not-a-url` exits 2 with a `usage_error` record (evidence V1).
- Resolution reference: `fix(health-check): [#2764] address review round 1 of PR 2889 in the binary and its tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589376725

### F3 - Write failure escalated a healthy exit code

- PR number: 2889
- Source review ID: 5924341133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589083446
- Concern: When the stderr write failed, `exit_code.max(EXIT_FAILURE)` turned a healthy probe into exit 1, contrary to its comment and the reference tools.
- Solution: The write result is dropped and the outcome's exit code is kept, as `ci-gate` and `shipped-features-check` do; the comment says the exit code alone carries the outcome (a broken pipe), and no longer says "closed".
- Current-tree verification: `src/bin/http_health_check.rs` `main` returns `ExitCode::from(exit_code)` unconditionally after the write.
- Resolution reference: `fix(health-check): [#2764] address review round 1 of PR 2889 in the binary and its tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589377257

### F4 - M2 and T3 describe a procedure that was not run

- PR number: 2889
- Source review ID: 5924341133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589083457
- Concern: M2 and T3 said the health API was stopped; evidence V2 ran a second container whose probe port does not match.
- Solution: M2 and T3 now describe the run that happened: a healthy container, then a second container with `HEALTH_CHECK_API_PORT=1314`, and why the health API cannot be stopped on its own.
- Current-tree verification: `ISSUE.md` rows T3 and M2 read against evidence V2.
- Resolution reference: `docs(issues): [#2764] address review round 1 of PR 2889 in the spec and the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589377616

### F5 - Red and mutation records thinner than their claims

- PR number: 2889
- Source review ID: 5924341133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589083465
- Concern: R2 had no command for the clean-tree claim and mutated only the usage exit code while AC3 cites it for every exit code; R1 did not show the 503 case.
- Solution: R1 was re-run against the `develop` binary with the current tests, and its output for every case (the 503 case included) is recorded. R2 now has one mutation per clause (non-2xx exit 1, usage exit 2, JSON on stderr) with the failing tests and the `diff -q` / `git status --short` that showed the tree restored.
- Current-tree verification: Evidence R1 and R2 in `manual-verification-evidence.md`.
- Resolution reference: `docs(issues): [#2764] address review round 1 of PR 2889 in the spec and the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589377996

### F6 - Exit-code assertions carry no message

- PR number: 2889
- Source review ID: 5924341133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589083475
- Concern: A failing exit-code assertion printed only `left`/`right`, not the stderr record that explains it.
- Solution: Every exit-code check goes through `assert_exit_code`, which passes the child's stderr as the assertion message.
- Current-tree verification: Red runs recorded in R1 show messages such as `unexpected exit code; stderr was: {"kind":"unreachable",...}`.
- Resolution reference: `fix(health-check): [#2764] address review round 1 of PR 2889 in the binary and its tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589378431

### F7 - Stamps later than their carrying commit

- PR number: 2889
- Source review ID: 5924341133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589083483
- Concern: `last-updated-utc` and the progress-log entry were stamped 16:59 UTC; the commit carrying them was made at 16:58:26Z.
- Solution: The log entry is left as written and a correction entry is appended naming it; `last-updated-utc` is set from this round's commit.
- Current-tree verification: `ISSUE.md` progress log: the correction entry follows the 16:59 entry.
- Resolution reference: `docs(issues): [#2764] address review round 1 of PR 2889 in the spec and the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589378806

### F8 - tests/AGENTS.md does not mention root src/bin tool tests

- PR number: 2889
- Source review ID: 5924341133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589083496
- Concern: The test directory guide scopes it to tracker-application tests; this file is the first binary-boundary test of a root `src/bin` tool there.
- Solution: `tests/AGENTS.md` gains a short exception paragraph (the `CARGO_BIN_EXE_*` reason) and the file in its structure listing.
- Current-tree verification: `tests/AGENTS.md` sections "What Does NOT Belong Here" and "Current Test Structure".
- Resolution reference: `docs(tests): [#2764] say where the black-box tests of root src/bin tools live`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589379279

### F9 - Proxy variables can divert the probes

- PR number: 2889
- Source review ID: 5924341133
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589083506
- Concern: The child inherits `HTTP_PROXY`/`ALL_PROXY`, which `reqwest` honours, so on a host with a proxy the probes may not reach the test server.
- Solution: The test removes the six proxy variables (upper and lower case `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`) from the child's environment; the binary is unchanged.
- Current-tree verification: `tests/http_health_check.rs` `PROXY_VARIABLES` and `run_health_check`.
- Resolution reference: `fix(health-check): [#2764] address review round 1 of PR 2889 in the binary and its tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589379682

### F10 - Credential-bearing URLs in diagnostics

- PR number: 2889
- Source review ID: 5924435062
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589155409
- Concern: The `healthy`, `unhealthy` and `unreachable` records copied the URL into `message` and `url`, and the request error formats the URL too, exposing credentials in userinfo, query or path (contract ADR, redaction policy).
- Solution: No record carries the URL any more (`url` field removed, messages say "the health endpoint"); request errors pass through `reqwest::Error::without_url` before their cause chain is formatted; a usage error never repeats the argument. Three regression tests probe a URL with credentials in userinfo, path and query for the healthy, unhealthy and unreachable outcomes, and the malformed-URL test checks its argument is not echoed.
- Current-tree verification: Against the previous binary the three credential tests failed (`stderr exposes probe-user`); on the current tree they pass. Manual run with a credential URL against a real tracker shows no credential in the record (evidence V1).
- Resolution reference: `fix(health-check): [#2764] address review round 1 of PR 2889 in the binary and its tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589380097

### F11 - Non-UTF-8 argument panics

- PR number: 2889
- Source review ID: 5924435062
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589155516
- Concern: Same concern as F2: `env::args()` panics on a non-UTF-8 argument; asks for `args_os` and a Unix regression test.
- Solution: Fixed with F2; the Unix test `it_should_exit_2_with_a_usage_error_record_when_the_argument_is_not_valid_utf8` passes an invalid byte.
- Current-tree verification: Same as F2.
- Resolution reference: `fix(health-check): [#2764] address review round 1 of PR 2889 in the binary and its tests`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589380587

### F12 - Released port probed and expected dead

- PR number: 2889
- Source review ID: 5924435062
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589155600
- Concern: Same concern as F1: keep a bound, non-listening socket alive through the probe.
- Solution: Fixed with F1.
- Current-tree verification: Same as F1.
- Resolution reference: `test(health-check): [#2764] hold the unreachable port bound without listening`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589381011

## Processing Log

- 2026-11-16 17:32 UTC - Replied to F1 (reply created 17:32:05Z) after committing its fix and before this record existed: a late start of the audit.
- 2026-11-16 17:40 UTC - Started audit: created this record from `docs/templates/PR-REVIEW-TEMPLATE.md` (the folder is listed as untracked by the `git status --short` run of evidence R2, at about 17:40 UTC). Normalized da2ce7 review 5924341133 (F1 to F9) and Copilot review 5924435062 (its F1, F4 and F5 as F10, F11 and F12; F11 re-raises F2, F12 re-raises F1).
- 2026-11-16 17:50 UTC - Replied to F2 to F12 after committing their fixes (replies created from 17:50:24Z to 17:50:49Z). `validate-audit-record.py --pr-number 2889 --base torrust/develop` reported 12 rows, 0 failures.
- 2026-11-16 17:51 UTC - Resolved the 12 threads (`resolveReviewThread` returned `isResolved: true` for each; the last at about 17:51:37Z). A GraphQL refresh found no unresolved thread and no thread whose last comment is not a reply from `josecelano-bot`.
- 2026-11-16 17:51 UTC - Committed this record.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
