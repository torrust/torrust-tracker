---
report-id: <YYYY-MM-DD_slug>
date-received: YYYY-MM-DD
date-disclosed: YYYY-MM-DD
status: <fixed | hardened | declined | non-affecting>
severity: <maintainer-assigned: critical | high | medium | low | hardening>
weakness: <CWE id or short class>
component: <package/path>
fix-commit: <sha or n/a>
fix-pr: <#number or n/a>
issue-spec: <docs/issues/... path>
reported-by: <approved public name / handle, or "anonymous">
review-cadence: <none | quarterly | on-recheck-condition>
requires-recheck-when: <condition that would reopen this>
---

<!-- Template for docs/security/analysis/reports/YYYY-MM-DD_<slug>.md. -->
<!-- Rules and index: docs/security/analysis/reports/README.md -->
<!-- Process: docs/security/vulnerability-remediation.md -->

# {Title}

## Finding

What was reported, in one paragraph. State whether the reporter demonstrated it or
identified it by source review. No exploit code, no raw evidence, no reporter contact
details.

## Maintainer assessment

Was reproduction attempted, and what was the result? Which severity was assigned and why
(state where and why it differs from the reporter's)? Link to the issue spec for the full
evidence.

## Action taken

What changed, or why nothing changed. Link the commit and PR.

## Recheck conditions

What would make this verdict stale (mirrors `requires-recheck-when`).

## Credit

Approved public credit, or "anonymous".
