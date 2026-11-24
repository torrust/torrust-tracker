---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/3004-2758-tracker-client-mask-tracker-key/ISSUE.md
    - docs/issues/closed/3004-2758-tracker-client-mask-tracker-key/manual-verification-evidence.md
    - docs/templates/ISSUE.md
    - .claude/skills/fix-bug/SKILL.md
---

<!-- cspell:ignore SECRETKEY -->

# Implementation Retrospective — `tracker_client`: do not write the tracker key

## Purpose

Record evidence-based process improvements discovered while implementing #3004. This is a blameless
review of the implementation approach; it does not replace acceptance-criteria verification.

## Outcome

`tracker_client` writes no tracker key on stdout or stderr. One module of the library
(`packages/tracker-client/src/http/client/redaction.rs`) masks the key segment of a tracker URL as
`****`, masks the key of every tracker URL inside a free text, and describes a failed request by
its kind and the masked URL. The library error and every output path of the console client use it.
Evidence: the red, green and mutation runs and the M1 to M4 recheck in
`manual-verification-evidence.md`; the tests in `console/tracker-client/tests/key_redaction.rs`
and in the library.

## What Went Well

1. Keeping a copy of the unchanged `develop` binary before any code changed made the black-box red
   run cheap and unambiguous: `cargo test` does not set `NEXTEST_BIN_EXE_*`, so the copy was the
   binary under test, and its name in the usage error proved it.
2. Restoring each changed console file to `develop` in turn showed which test guards which fix, and
   found that the health-check path had no guarding test; one was added.
3. Running the binary against malformed input during the audit, not only reading the code, found
   two leaks the spec did not list.

## What Changed During Implementation

- The T1 audit found two more leaks (P8, P9 in the spec): `serde_json` echoes a wrong-typed
  configuration value, and `clap` echoes a misplaced argument, both of which can be a keyed URL.
  The maintainer decided to fix them here, which added a text-masking function and two tests.
- The spec's commit table had a commit of the red tests (T2). The pre-commit gate runs the tests,
  so a red test cannot be committed; the red runs are recorded in the evidence and the tests are
  committed with the fix.
- The design review split the planned single body-read test (announce and `check`) in two, one Act
  each.
- The masking function also masks the password of a URL user info, a credential in the same URL.
- The test key `SECRETKEYabc123`, used in the spec and in R0, is rejected by the spell checker in
  Rust sources; the tests use `SecretTrackerKey123`.

## Root Cause

- The spec's analysis listed the sources found by reading the code paths of a request. It did not
  ask which other inputs the tool echoes back (parse and usage errors written by third-party
  crates), so the audit found them only by trying malformed input.
- The spec template's commit table pairs every task with a commit, including a task whose output
  (red tests) cannot pass the pre-commit gate on its own.

## Improvements for Future Work

1. A spec about keeping a secret out of the output lists, in its analysis, every place where the
   tool echoes caller input through a third-party formatter (argument parser, configuration parser)
   and plans a probe with malformed input for each. Owner: the `fix-bug` skill or
   `docs/templates/ISSUE.md`; the maintainer decides whether to open an issue.
2. A bug spec's commit table commits the red tests together with the fix, and records the red run in
   the evidence; it does not plan a separate red commit. Owner: `docs/templates/ISSUE.md`.

## Avoiding Overcorrection

The evidence does not justify masking every URL in every text the tool writes: the text masker is
applied only where a third-party formatter echoes input, and the URL masker where the tool writes a
URL it holds. The query string is kept, because it carries no secret and identifies the request.
The different markers (`***` for the server side in #2999, `****` here) are not aligned in this
issue.

## Evidence

- Spec: `ISSUE.md` (Output Path Audit) and `manual-verification-evidence.md` in this directory,
  with the full red and mutation logs in `evidence/`.
- Tests: `console/tracker-client/tests/key_redaction.rs`;
  `packages/tracker-client/src/http/client/redaction.rs` and the `error_text` tests in
  `packages/tracker-client/src/http/client/mod.rs`.
