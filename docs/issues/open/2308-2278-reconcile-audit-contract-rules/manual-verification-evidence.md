---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2308-2278-reconcile-audit-contract-rules/ISSUE.md
last-updated-utc: "2026-09-23 09:27"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-09-23 09:27
- Artifact under test: #2308 implementation on branch `2308-2278-reconcile-audit-contract-rules`
- Operating system / environment: Linux
- Prerequisites and setup performed: Started from `torrust/develop` at `78d7e0fe` and inspected the current `process-pr-review` skill and `PR-REVIEW-TEMPLATE.md` after both documentation commits.

## Verification Processes

### V1 - Outdated Thread and Re-Raise Rules

- Goal: Confirm the skill and template agree on fixed outdated threads and distinct re-raise rows.
- Initial state: The audit contract needed explicit F60 and F73 rules in both normative documents.
- Status: `DONE`

#### Steps Performed

1. Ran `rg -n -A3 -B2 'outdated thread whose concern|own tracking row' .github/skills/dev/pr-reviews/process-pr-review/SKILL.md docs/templates/PR-REVIEW-TEMPLATE.md`.
2. Read the matched skill and template rules.

#### Observed Result

```text
The template and skill state that a fixed outdated thread is FIXED/RESOLVED. Both documents also state that every re-raise has its own tracking row and matching detail entry.
```

#### Conclusion

The observed rules satisfy F60 and F73 without changing the audit roster.

### V2 - Resolution Reference Rules

- Goal: Confirm each disposition has explicit admissible evidence and follow-up pull requests remain in their separate field.
- Initial state: F76 lacked disposition-specific resolution-reference guidance.
- Status: `DONE`

#### Steps Performed

1. Ran `rg -n -A3 -B2 'FIXED resolution|Resolution reference.*FIXED|Follow-up PR URL field' .github/skills/dev/pr-reviews/process-pr-review/SKILL.md docs/templates/PR-REVIEW-TEMPLATE.md`.
2. Compared the matched skill and template guidance.

#### Observed Result

```text
Both documents require a Conventional Commit subject for FIXED, a durable reply URL for NO_ACTION, SUPERSEDED, or FOLLOW_UP, and reserve the separate Follow-up PR URL field for a follow-up pull request.
```

#### Conclusion

The observed rules satisfy F76 without adding an audit field.

### V3 - Append-Only and Consolidated-Response Rules

- Goal: Confirm append-only recovery and conditional consolidated-response requirements are explicit.
- Initial state: F61, F62, and F79 lacked a complete shared rule.
- Status: `DONE`

#### Steps Performed

1. Ran `rg -n -A3 -B2 'rewritten in place|Append entries only|Consolidated responses that cover|consolidated PR conversation' .github/skills/dev/pr-reviews/process-pr-review/SKILL.md docs/templates/PR-REVIEW-TEMPLATE.md`.
2. Read the matched log-recovery, completion-checklist, and template completion-rule text.

#### Observed Result

```text
The template requires append-only entries and describes restoring prior entries followed by a named correction. The skill gives the same recovery. Both documents condition a consolidated response on covering multiple review rounds and require covered IDs, dispositions, resolution references, and a durable URL in each related row.
```

#### Conclusion

The observed rules satisfy F61, F62, and F79.

### V4 - Review-Body and Suppressed-Comment Handling

- Goal: Confirm source and thread-state handling for review-body findings and Copilot suppressed comments.
- Initial state: F80 and F66 had no explicit normalization rule.
- Status: `DONE`

#### Steps Performed

1. Ran `rg -n -A3 -B2 'review-body finding|Suppressed comments' .github/skills/dev/pr-reviews/process-pr-review/SKILL.md docs/templates/PR-REVIEW-TEMPLATE.md`.
2. Read the matched normalization and template guidance.

#### Observed Result

```text
The template assigns a submitted review URL and NON_RESOLVABLE state to an actionable review-body finding without an inline thread. The skill treats suppressed comments as findings only when full retrievable content has an independently actionable assertion; otherwise no row is created.
```

#### Conclusion

The observed rules satisfy F80 and F66.

## Failures and Follow-up

No manual verification scenario failed or was blocked.
