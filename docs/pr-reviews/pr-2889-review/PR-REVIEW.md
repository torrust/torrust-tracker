---
pr-number: 2889
pr-url: https://github.com/torrust/torrust-tracker/pull/2889
last-updated-utc: "2026-11-16 18:57"
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

Round 2, on the head that carries the commit `docs(pr-reviews): [#2764] audit review round 1 of PR 2889`: da2ce7 review 5925054031 (APPROVED) with findings F10 to F12 (two Nit, one Suggestion). Their IDs collide with this audit's F10 to F12, so they are audit findings F13 to F15.

Round 3, on the head that carries the commit `docs(pr-reviews): [#2764] audit review round 2 of PR 2889`: da2ce7 review 5925258553 (APPROVED) with findings F13 and F14 (one Minor, one Nit). Their IDs collide with this audit's F13 and F14, so they are audit findings F16 and F17.

Round 4, on the head that carries the commit `docs(pr-reviews): [#2764] audit review round 3 of PR 2889`: Copilot review 5925595121 (COMMENTED, a "Lite" review) with five threads and no severity tags, so each severity is inferred. Four threads carry the same text, each anchored to a different evidence section (F18 to F21); each section got its own note. The fifth is F22.

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
| F13 | review-finding:pr-2889-f13 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2889-f14 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2889-f15 | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2889-f16 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2889-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2889-f18 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2889-f19 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2889-f20 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2889-f21 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2889-f22 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

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

### F13 - Image run during a mutation kept without the stage argument

- PR number: 2889
- Source review ID: 5925054031
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589622067
- Concern: The round-1 image build ran while the binary was mutated for R2; the record kept the run with an `--entrypoint` check that rules out only the usage-code mutation, while the evidence template says to discard such a run.
- Solution: The evidence now states the stage-result argument (the stage builds its tests from the same source, each round-1 mutation fails at least two `http_health_check` tests, and the stage ran 1888 tests with none failing), says what the `--entrypoint` check does and does not rule out, and says the run is kept on that argument.
- Current-tree verification: Evidence section "Round 1 of PR 2889", Image paragraph.
- Resolution reference: `docs(issues): [#2764] address review round 2 of PR 2889 in the spec and the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589653117

### F14 - AC3 claims a mutation per exit-code clause that R2 lacks

- PR number: 2889
- Source review ID: 5925054031
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589622075
- Concern: The AC3 row said R2 has one mutation per exit-code clause, but exit 0 on a 2xx answer and exit 1 on a failed request had no row.
- Solution: R2 gains the two rows (`EXIT_HEALTHY` set to 1; the `unreachable` record returning `EXIT_HEALTHY`), each failing two tests; a first exit-0 attempt that did not compile is recorded and not counted. AC3 names the five clauses R2 covers.
- Current-tree verification: Evidence R2 table; `ISSUE.md` AC3 row.
- Resolution reference: `docs(issues): [#2764] address review round 2 of PR 2889 in the spec and the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589653468

### F15 - Non-HTTP schemes pass as URLs

- PR number: 2889
- Source review ID: 5925054031
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589622087
- Concern: `Url::parse` accepts any scheme, so `ftp://` or `mailto:` arguments failed later as `unreachable` with exit 1, unlike other unusable arguments (usage error, exit 2).
- Solution: `parse_arguments` accepts only `http` and `https`; any other scheme is a `usage_error` with exit 2. A new test covers `ftp://` and `mailto:`.
- Current-tree verification: The new test failed against the round-1 binary (exit 1, `builder error: URL scheme is not allowed`) and passes on the current tree; 11 tests pass under `cargo test` and nextest (evidence "Round 2 of PR 2889").
- Resolution reference: `fix(health-check): [#2764] report a URL scheme other than http or https as a usage error`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589653845

### F16 - Real-artifact recheck not repeated after the scheme check

- PR number: 2889
- Source review ID: 5925258553
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589777162
- Concern: The scheme check is a production change made after the round-1 recheck; `fix-bug` asks for the like-for-like real-artifact recheck after the last production change, naming the commit it ran on.
- Solution: Reran the V1 and V3 real-tracker commands, plus an `ftp://` case, on the build whose last production change is the scheme-check commit; the evidence names that commit and says why the image was not rebuilt.
- Current-tree verification: Evidence section "Round 3 of PR 2889": exits 0, 0, 1, 1, 2, 2, 2, 1; stdout empty; one JSON record per run.
- Resolution reference: `docs(issues): [#2764] address review round 3 of PR 2889 in the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589799562

### F17 - R2 timing sentence refers to shifted rows

- PR number: 2889
- Source review ID: 5925258553
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589777170
- Concern: Inserting the round-2 rows shifted the R2 table under a sentence that named rows by position ("the first and third rows").
- Solution: The sentence names the rows by clause and points to the paragraph that times the round-2 rows.
- Current-tree verification: Evidence R2, the sentence before the table.
- Resolution reference: `docs(issues): [#2764] address review round 3 of PR 2889 in the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4589799971

### F18 - Evidence G1 reads as current but shows the first implementation

- PR number: 2889
- Source review ID: 5925595121
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590037910
- Concern: Section G1 (the first green run with 5 tests, line 103) does not match the final binary (11 tests, no URL in records) and is not labelled as historical.
- Solution: The section opens with a "Historical" note: it ran on the first implementation (records with `url`, 5 tests), the final binary writes no URL and has 11 tests, and the results for it are in the sections "Round 1 of PR 2889" to "Round 3 of PR 2889". A note under "Verification Processes" explains the order. The recorded output is kept as it ran.
- Current-tree verification: Evidence G1 heading and note.
- Resolution reference: `docs(issues): [#2764] address review round 4 of PR 2889 in the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590084900

### F19 - Evidence V1, V3 shows records with the URL

- PR number: 2889
- Source review ID: 5925595121
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590037976
- Concern: Section V1, V3 (line 122) shows records with a `url` field and the URL in `message`, which the final binary no longer writes, without saying the output is historical.
- Solution: The section opens with a "Historical" note: it ran on the first implementation (records with `url`, 5 tests), the final binary writes no URL and has 11 tests, and the results for it are in the sections "Round 1 of PR 2889" to "Round 3 of PR 2889". A note under "Verification Processes" explains the order. The recorded output is kept as it ran.
- Current-tree verification: Evidence V1, V3 heading and note.
- Resolution reference: `docs(issues): [#2764] address review round 4 of PR 2889 in the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590085273

### F20 - Evidence V2 healthy record shows the URL

- PR number: 2889
- Source review ID: 5925595121
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590038015
- Concern: Section V2 (line 155) shows the `healthy` probe record with the URL, unlike the final binary, without saying the output is historical.
- Solution: The section opens with a "Historical" note: it ran on the first implementation (records with `url`, 5 tests), the final binary writes no URL and has 11 tests, and the results for it are in the sections "Round 1 of PR 2889" to "Round 3 of PR 2889". A note under "Verification Processes" explains the order. The recorded output is kept as it ran.
- Current-tree verification: Evidence V2 heading and note.
- Resolution reference: `docs(issues): [#2764] address review round 4 of PR 2889 in the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590085683

### F21 - Evidence V2 unreachable record shows the URL

- PR number: 2889
- Source review ID: 5925595121
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590038064
- Concern: Section V2 (line 161) shows the `unreachable` probe record with the URL, unlike the final binary, without saying the output is historical.
- Solution: The section opens with a "Historical" note: it ran on the first implementation (records with `url`, 5 tests), the final binary writes no URL and has 11 tests, and the results for it are in the sections "Round 1 of PR 2889" to "Round 3 of PR 2889". A note under "Verification Processes" explains the order. The recorded output is kept as it ran.
- Current-tree verification: Evidence V2 heading and note.
- Resolution reference: `docs(issues): [#2764] address review round 4 of PR 2889 in the evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590086075

### F22 - Test server treats every accept error as WouldBlock

- PR number: 2889
- Source review ID: 5925595121
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590038112
- Concern: The test server's accept loop slept and continued on any error, so a real failure (for example no file descriptors left) would make it a silent non-responder.
- Solution: The loop sleeps only on `WouldBlock`, retries on `Interrupted` and panics on any other error; `TestServer::drop` re-raises the panic with `resume_unwind` unless the test is already panicking.
- Current-tree verification: 11 tests pass under `cargo test` and nextest; with the `WouldBlock` guard temporarily changed, the four server tests failed with `the test server could not accept a connection: ...` (evidence "Round 4 of PR 2889").
- Resolution reference: `test(health-check): [#2764] fail the test when the test server cannot accept a connection`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2889#discussion_r4590086454

## Processing Log

- 2026-11-16 17:32 UTC - Replied to F1 (reply created 17:32:05Z) after committing its fix and before this record existed: a late start of the audit.
- 2026-11-16 17:40 UTC - Started audit: created this record from `docs/templates/PR-REVIEW-TEMPLATE.md` (the folder is listed as untracked by the `git status --short` run of evidence R2, at about 17:40 UTC). Normalized da2ce7 review 5924341133 (F1 to F9) and Copilot review 5924435062 (its F1, F4 and F5 as F10, F11 and F12; F11 re-raises F2, F12 re-raises F1).
- 2026-11-16 17:50 UTC - Replied to F2 to F12 after committing their fixes (replies created from 17:50:24Z to 17:50:49Z). `validate-audit-record.py --pr-number 2889 --base torrust/develop` reported 12 rows, 0 failures.
- 2026-11-16 17:51 UTC - Resolved the 12 threads (`resolveReviewThread` returned `isResolved: true` for each; the last at about 17:51:37Z). A GraphQL refresh found no unresolved thread and no thread whose last comment is not a reply from `josecelano-bot`.
- 2026-11-16 17:51 UTC - Committed this record.
- 2026-11-16 18:16 UTC - Round 2: normalized da2ce7 review 5925054031 (F13 to F15), committed the fixes, then replied (replies created from 18:16:08Z to 18:16:12Z). `validate-audit-record.py --pr-number 2889 --base torrust/develop` reported 15 rows, 0 failures.
- 2026-11-16 18:16 UTC - Resolved the three round-2 threads (`isResolved: true` for each, the last at about 18:16:50Z). A GraphQL refresh found no unresolved thread and no thread whose last comment is not a reply from `josecelano-bot`.
- 2026-11-16 18:16 UTC - Committed this record.
- 2026-11-16 18:30 UTC - Round 3: normalized da2ce7 review 5925258553 (F16, F17), committed the evidence fix, then replied (replies created at 18:30:03Z and 18:30:05Z). `validate-audit-record.py --pr-number 2889 --base torrust/develop` reported 17 rows, 0 failures.
- 2026-11-16 18:30 UTC - Resolved the two round-3 threads (`isResolved: true` for each, the last at about 18:30:38Z). A GraphQL refresh found no unresolved thread and no thread whose last comment is not a reply from `josecelano-bot`.
- 2026-11-16 18:30 UTC - Committed this record.
- 2026-11-16 18:56 UTC - Round 4: normalized Copilot review 5925595121 (F18 to F22), committed the fixes, then replied (replies created from 18:55:54Z to 18:56:03Z). `validate-audit-record.py --pr-number 2889 --base torrust/develop` reported 22 rows, 0 failures.
- 2026-11-16 18:56 UTC - Resolved the five round-4 threads (`isResolved: true` for each, the last at about 18:56:55Z). A GraphQL refresh found no unresolved thread and no thread whose last comment is not a reply from `josecelano-bot`.
- 2026-11-16 18:57 UTC - Committed this record.

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
