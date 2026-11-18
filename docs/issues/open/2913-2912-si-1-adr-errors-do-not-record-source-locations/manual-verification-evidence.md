---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2913-2912-si-1-adr-errors-do-not-record-source-locations/ISSUE.md
last-updated-utc: "2026-11-18 04:58"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-18 04:57
- Artifact under test: the branch `2913-adr-errors-do-not-record-source-locations` with the
  commit "docs(adrs): [#2913] add ADR: errors do not record source locations" and the working-tree
  change to `.claude/skills/handle-errors-in-code/SKILL.md` that is committed as
  "docs(skills): [#2913] replace located-error guidance with the no-source-location rule".
- Operating system / environment: Linux, bash, GNU grep and sed.
- Prerequisites and setup performed: none; the scenario reads documentation only.

## Verification Processes

### V1 - Read the guidance as a contributor (M1)

- Goal: a contributor who adds a new error variant and follows the `handle-errors-in-code` skill
  ends with an error that has no `location` field and finds the ADR.
- Initial state: the skill as changed by this issue, and the ADR file in `docs/adrs/`.
- Status: `DONE`

#### Steps Performed

Started 04:57:59 UTC, ended 04:57:59 UTC; the steps only read files, in this order.

1. Read the skill section that replaces "The `located-error` Package":
   `sed -n '/## No Source Locations/,/^## Unwrap/p' .claude/skills/handle-errors-in-code/SKILL.md`.
2. Read the checklist a contributor follows for a new variant:
   `sed -n '/## Quick Checklist/,$p' .claude/skills/handle-errors-in-code/SKILL.md`.
3. Checked that the ADR the skill links to exists:
   `ls docs/adrs/20261118045443_errors_do_not_record_source_locations.md`.

#### Observed Result

```text
## No Source Locations in Errors

Errors do not record or print source locations. See
[ADR 20261118045443](../../../docs/adrs/20261118045443_errors_do_not_record_source_locations.md).

- No `location` field and no `Location::caller()` in error types.
- No `torrust-located-error`: keep a source error as a plain `#[source]` (or `#[from]`) field.
- `Display` states what failed and the offending input, never a source path.
- Text sent to a client is the error's own message; do not format a source error from a crate the
  workspace does not control into client text.
- For developer diagnostics, emit a `tracing` event at the site; it records file and line on the
  server only.

## Quick Checklist

- [ ] Error type uses `thiserror::Error` derive
- [ ] Error message includes specific context (names, paths, addresses, values)
- [ ] Error message includes fix instructions where possible
- [ ] Prefer `enum` over `Box<dyn Error>` or `anyhow` in library code
- [ ] No vague messages like "invalid input" or "error occurred"
- [ ] No `.unwrap()` in production code (tests and doc examples are fine)
- [ ] No `location` field, no `Location::caller()` and no `torrust-located-error` (see the ADR above)

docs/adrs/20261118045443_errors_do_not_record_source_locations.md
```

#### Conclusion

Met. Following the checklist, a new variant has no `location` field, no `Location::caller()`
and no `LocatedError` source, and the rule section links to the ADR, which exists at the linked
path (step 3).

## Failures and Follow-up

None.
