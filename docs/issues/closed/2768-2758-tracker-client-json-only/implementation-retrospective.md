---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2768-2758-tracker-client-json-only/ISSUE.md
    - docs/issues/closed/2768-2758-tracker-client-json-only/manual-verification-evidence.md
    - docs/templates/ISSUE.md
---

# Implementation Retrospective — `tracker_client`: TTY refusal and JSON-only output

## Purpose

Record evidence-based process improvements discovered while implementing #2768 (PR 2956).

## Outcome

`tracker_client` refuses a terminal (exit `2`, one `tty_refusal` record) before it parses its
arguments, writes one JSON object on stdout and nothing on failure, and writes only JSON records on
stderr (tracing, help, version, usage errors, warnings, unrecognized responses and panics). One
module (`unified/output.rs`) owns every write. `check` returns `{"results": [...]}`. Evidence:
`manual-verification-evidence.md` (R1, G1, V1 to V4) and `console/tracker-client/tests/output_contract.rs`.

## What Went Well

1. The tests ran the real binary and parsed every stderr line, so the contract was tested as a
   whole, and a deliberate red run against the unchanged tool proved they could fail.
2. The pseudo-terminal tests found a real ordering rule (refuse before parsing) and the parse-order
   test fails if someone moves it.

## What Changed During Implementation

- A first red run set `NEXTEST_BIN_EXE_*` from outside, which nextest overrides, so it proved
  nothing. It was redone by building the unchanged binary in a separate worktree.
- The pseudo-terminal tests hung for minutes because a copy of the slave end stayed open; every copy
  must be dropped before the master is read. The workspace denies `unsafe`, so `libc::openpty` was
  replaced by the safe `rustix::pty` API.
- Review found that a panic (a `check` task with `udp://127.0.0.1`) still wrote the default plain
  text. The binary now installs a panic hook that writes a `panic` record.
- Review found live skills and a README example still running the tool on a terminal, which is now
  now refused, a broken README fence, an undocumented `panic` record, and a stale PR description.

## Root Cause

- The spec listed the code to change but gave no search that finds instructions to run the tool
  elsewhere (skills, README), so the documentation sweep was not done until review. The same gap
  appeared in #2765.
- "Every stderr line is JSON" was tested for the paths the spec named; nothing asked which other
  writers exist (the default panic hook, tracing).

## Improvements for Future Work

1. A spec that changes how a command is run lists the search command that finds every instruction
   to run it (skills, READMEs, docs) and records its output; the PR updates them in the same change.
2. A spec for a "no plain text on stderr" rule lists every writer (panic hook, tracing, child
   processes) as a checklist item, each with a test.
3. When the PR description quotes counts (tests, files), regenerate them from the final head before
   asking for the ACK.

## Avoiding Overcorrection

The evidence does not justify a new lint for the panic hook or a schema version field; the `check`
shape change is recorded as a documented breaking change and a version is left to a later issue.
