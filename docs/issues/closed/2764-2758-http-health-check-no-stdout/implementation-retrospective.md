---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2764-2758-http-health-check-no-stdout/ISSUE.md
    - docs/issues/closed/2764-2758-http-health-check-no-stdout/manual-verification-evidence.md
    - docs/pr-reviews/pr-2889-review/PR-REVIEW.md
    - docs/templates/ISSUE.md
---

# Implementation Retrospective — `http_health_check` follows the CLI output contract

## Purpose

Record evidence-based process improvements discovered while implementing #2764 (PR 2889). This
review is written against the merged result, after the five review rounds of PR 2889, not against
the first implementation.

## Outcome

`http_health_check` is a `no-stdout-result` program. Stdout is empty, and stderr carries one JSON
record with a `kind` (`healthy`, `unhealthy`, `unreachable` or `usage_error`). The exit code is
`0` on a 2xx answer, `1` on another status or a failed request, and `2` on a usage error. The
records never contain the probed URL. The container `HEALTHCHECK` needed no change and still turns
`unhealthy` when the probe fails (evidence V2). Eleven black-box tests in
`tests/http_health_check.rs` cover each outcome, each usage error and the redaction.

The final design differs from the first implementation in five ways. Review found all of them:

- The argument is read with `args_os`: a non-UTF-8 or malformed argument is a usage error, not a
  panic (audit F2, F11).
- A URL scheme other than `http` or `https` is a usage error (F15).
- The URL is removed from every record, including the error causes (F10, Major).
- The outcome exit code is kept when the record cannot be written (F3).
- The unreachable test holds a bound, non-listening port instead of a released one (F1, F12).

## What Went Well

1. Writing the black-box tests first against the old binary gave a red run per clause (R1), and
   each later production change got its own red test first.
2. The mutation runs (R2) showed each exit-code clause is pinned by a test, and they exposed the
   AC3 claim that a clause had a mutation when it did not (F14).
3. Running the container `HEALTHCHECK` with a port mismatch gave a real `unhealthy` result
   without stopping the tracker, which the spec's first procedure assumed was possible (F4).

## What Changed During Implementation

- The image test stages run from a nextest archive, where the compiled-in `CARGO_BIN_EXE_*` path
  does not exist. The first image build failed; the test now reads `NEXTEST_BIN_EXE_*` first
  (evidence G1).
- Review found that the diagnostics repeated the probed URL, which can carry credentials in the
  userinfo, the query or the path. The ADR redaction rule applies; the spec did not name it.
- Review found that the argument parser panicked on non-UTF-8 input and accepted any URL scheme.
- Review found the unreachable fixture could race: a released port may be taken by another
  process before the probe.
- 13 of the 24 audit findings were about the spec and evidence records, not the code (F4, F5,
  F7, F13, F14, F16 to F21, F23, F24). Stamps were later than their
  commits (F7). M2 and T3 described a run that did not happen (F4). Evidence sections showed
  output of an earlier binary without saying so (F18 to F21). A failure-path check was described
  as wider than it was (F23).

## Root Cause

- The spec stated the output shape and the exit codes, but not the inputs. Its acceptance
  criteria named no argument edge cases (non-UTF-8, wrong scheme) and did not apply the ADR
  redaction rule to the one argument the tool takes. The tests followed the acceptance criteria,
  so they checked the declared outcomes only.
- The evidence was written as a narrative of the runs, then kept as the binary changed. Nothing
  in the workflow asked for an earlier section to be marked historical when the code it records
  changed, or for a stamp to be taken at commit time.

## Improvements for Future Work

1. A spec for a CLI tool whose argument can carry a secret (a URL, a connection string) names the
   ADR redaction rule in an acceptance criterion and asks for a test that the secret is absent
   from every record.
2. A spec for a CLI tool lists the argument edge cases it treats as usage errors (count, encoding,
   format), so the tests pin them before review does.
3. When a production change lands after evidence is recorded, the same commit marks the affected
   evidence sections historical and says where the current result is.
4. Black-box tests that run a workspace binary look up `NEXTEST_BIN_EXE_*` before
   `CARGO_BIN_EXE_*`, so they also run in the image test stages.

## Avoiding Overcorrection

- No new linter or check is justified: each record defect was found by review and fixed in one
  round, and the stamp rule already exists.
- The record-level `healthy` output stays. The `HEALTHCHECK` output is kept in
  `.State.Health.Log`, not in the container logs, so a quieter mode is not justified.
- The retrospective of the review process itself (five rounds for a small binary) is not repeated
  here; the audit's Processing Log records each round.

## Evidence

- `docs/issues/closed/2764-2758-http-health-check-no-stdout/ISSUE.md`
- `docs/issues/closed/2764-2758-http-health-check-no-stdout/manual-verification-evidence.md`
  (R1, R2, G1, V1 to V3 and the sections "Round 1 of PR 2889" to "Round 5 of PR 2889")
- `docs/pr-reviews/pr-2889-review/PR-REVIEW.md` (findings F1 to F24)
- `tests/http_health_check.rs`, `src/bin/http_health_check.rs`
- PR 2889, merged into `develop` on 2026-11-16
