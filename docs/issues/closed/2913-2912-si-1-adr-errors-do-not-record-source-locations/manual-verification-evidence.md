---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2913-2912-si-1-adr-errors-do-not-record-source-locations/ISSUE.md
last-updated-utc: "2026-11-18 10:10"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-18 04:57 (first run); 2026-11-18 10:08 (re-run, recorded below)
- Artifact under test: the re-run reads `.claude/skills/handle-errors-in-code/SKILL.md` as merged
  on `develop` at `5b3c8b64c` (merge of PR #2930); `git diff --quiet torrust/develop --` on the
  skill exited 0 before the run. The first run, at 04:57 UTC, read the skill before the review
  fixes of PR #2930.
- Operating system / environment: Linux, bash, GNU grep and sed.
- Prerequisites and setup performed: none; the scenario reads documentation only.

## Verification Processes

### V1 - Read the guidance as a contributor (M1)

- Goal: a contributor who adds a new error variant and follows the `handle-errors-in-code` skill
  ends with an error that has no `location` field and finds the ADR.
- Initial state: the skill as changed by this issue, and the ADR file in `docs/adrs/`.
- Status: `DONE`

#### Steps Performed

Re-run started 10:08:11.851 UTC and ended 10:08:11.857 UTC (`date -u +%T.%3N` before and after);
the steps only read files and ran one after the other in one shell.

1. Read the skill section that replaces "The `located-error` Package":
   `sed -n '/## No Source Locations/,/^## Unwrap/p' .claude/skills/handle-errors-in-code/SKILL.md`.
2. Read the checklist a contributor follows for a new variant:
   `sed -n '/## Quick Checklist/,$p' .claude/skills/handle-errors-in-code/SKILL.md`.
3. Checked that the ADR the skill links to exists:
   `ls docs/adrs/20261118045443_errors_do_not_record_source_locations.md`.

#### Observed Result

The output of steps 1 to 3, verbatim and in order:

````text
## No Source Locations in Errors

Errors must not record or print source locations. See
[ADR 20261118045443](../../../docs/adrs/20261118045443_errors_do_not_record_source_locations.md).

- No `location` field and no `Location::caller()` in error types.
- No `torrust-located-error`: keep a source error as a plain `#[source]` (or `#[from]`) field.
- `Display` states what failed and the offending input, never a source path. A secret (an
  authentication key, a token, a password) is never echoed or kept in the error: name the parameter
  and the expected format instead (see the `handle-secrets` skill).
- Text sent to a client is the error's own message; do not format a source error from a crate the
  workspace does not control into client text.
- For developer diagnostics, emit a `tracing` event at the site; it records file and line on the
  server only.
- `#[track_caller]` is allowed only in test helpers and assertion helpers.

```rust
// ✅ Correct: names what failed and the expected format, and does not keep the secret
#[derive(Debug, thiserror::Error)]
pub enum KeyError {
    #[error("invalid authentication key: expected 32 alphanumeric characters, got {length}")]
    InvalidKeyFormat { length: usize },
}
```

```rust
// ❌ Wrong: a location that reaches clients and logs
#[derive(Debug, thiserror::Error)]
pub enum KeyError {
    #[error("invalid key format, {location}")]
    InvalidKeyFormat { location: &'static Location<'static> },
}
```

Existing `location` fields and `LocatedError` uses are being removed by EPIC #2912; do not add new
ones.

## Unwrap and Expect Policy
## Quick Checklist

- [ ] Error type uses `thiserror::Error` derive
- [ ] Error message includes specific context (names, paths, addresses, values), but no secrets
- [ ] Error message includes fix instructions where possible
- [ ] Prefer `enum` over `Box<dyn Error>` or `anyhow` in library code
- [ ] No vague messages like "invalid input" or "error occurred"
- [ ] No `.unwrap()` in production code (tests and doc examples are fine)
- [ ] No `location` field, no `Location::caller()` and no `torrust-located-error` (see the ADR above)
docs/adrs/20261118045443_errors_do_not_record_source_locations.md
````

#### Conclusion

Met. Following the checklist, a new variant has no `location` field, no `Location::caller()` and
no `LocatedError` source, keeps no secret, and the rule section links to the ADR, which exists at
the linked path (step 3).

## Failures and Follow-up

The first run, at 04:57 UTC, pasted output that review finding F9 of PR #2930 (da2ce7's F1) found
was not what the commands print once the review fixes of that PR had changed the skill, and the
first `sed` range was shortened. This re-run replaces it with the full output on the merged skill.
