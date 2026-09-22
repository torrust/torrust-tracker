---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2295-2278-single-source-audit-roster/ISSUE.md
last-updated-utc: "2026-09-22"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-09-22 13:13-13:16
- Artifact under test: #2295 implementation on branch `2295-2278-single-source-audit-roster`
- Operating system / environment: Linux; stable Rust toolchain
- Prerequisites and setup performed: Started from `torrust/develop` at merge commit `a3e9c9e1`.

## Verification Processes

### V1 - Roster Equals Skeleton

- Goal: Confirm the canonical skill roster and template skeleton contain the same 19 fields in the same order.
- Initial state: The skill exposed the 19-field grouped roster and the template exposed its tracking row, detail heading, and detail lines.
- Status: `DONE`

#### Steps Performed

1. Extracted list-item fields between the three roster headings in `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md` with `awk`, stopping at the next `##` heading.
2. Extracted the template tracking-row columns, `Summary` heading, and detail-line labels with `awk`, then ran `diff -u .tmp/2295-skill-fields.txt .tmp/2295-template-fields.txt`.

#### Observed Result

```text
M1 fields: 19; sequences identical
Finding ID
Review finding reference
Author class
Severity
Category
Relationship
Disposition
Thread state
Summary
PR number
Source review ID
Reviewer finding ID
Source URL
Concern
Solution
Current-tree verification
Resolution reference
Follow-up PR URL
Reply URL
```

#### Conclusion

The roster and skeleton have identical 19-field sequences.

### V2 - Contract Pins Are Live

- Goal: Confirm the moved contract-check pins identify the canonical roster and the normalized compact-row rule.
- Initial state: T1 rewrote the skill's old prose roster; T2 moved each affected `REQUIRED_TEXT` or `WRAPPED_TEXT` pin.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo run --package agent-review-report-contract` with the stable Rust toolchain.
2. Ran `grep -nF '### Tracking Row Fields'`, `grep -nF '### Detail-Entry Heading Field'`, and `grep -nF 'tracking row plus one matching detail entry carrying the remaining narrative and'` against the skill. The full wrapped pin is verified by the contract checker, which normalizes whitespace.

#### Observed Result

```text
Agent review report contract check passed.
137:### Tracking Row Fields
148:### Detail-Entry Heading Field
167:tracking row plus one matching detail entry carrying the remaining narrative and
```

#### Conclusion

The contract check succeeds and every replacement pin identifies its current skill location.

### V3 - Template Sections Match Real Audits

- Goal: Confirm the template has one classification marker for each section and its section headings match recent records.
- Initial state: The template marks `Ownership`, `Status Values`, and `Completion Rules` as copied verbatim, and marks instantiated `Findings`, `Finding Details`, and `Processing Log` content as guidance omitted from records.
- Status: `DONE`

#### Steps Performed

1. Compared `grep '^## ' docs/templates/PR-REVIEW-TEMPLATE.md` with the same command for `docs/pr-reviews/pr-2288-review/PR-REVIEW.md` using `diff -u`.
2. Repeated the comparison for `docs/pr-reviews/pr-2296-review/PR-REVIEW.md`.
3. Ran `grep -c '^<!-- Guidance omitted' docs/templates/PR-REVIEW-TEMPLATE.md`.

#### Observed Result

```text
Both diff commands produced no output.
M3 guidance markers: 3
```

#### Conclusion

Both recent audits use the template's six section headings. The template locally distinguishes three copied sections from three guidance-instantiated sections.

### V4 - Historical Records Untouched

- Goal: Confirm this implementation does not edit historical PR-review audit records.
- Initial state: #2295 changes only the review-processing skill, its template, and the contract checker.
- Status: `DONE`

#### Steps Performed

1. Ran `git diff --stat torrust/develop...HEAD -- docs/pr-reviews/`.

#### Observed Result

```text
No output
```

#### Conclusion

No historical audit record is changed by the implementation.

## Failures and Follow-up

The first M1 extraction command did not stop at the next `##` heading and therefore included unrelated later lists. It was a verification-command defect, not an artifact defect. The corrected command stopped at the next `##` heading and produced the passing 19-field comparison recorded above.
