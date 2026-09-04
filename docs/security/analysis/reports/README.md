---
semantic-links:
  skill-links:
    - catalog-security-vulnerabilities
  related-artifacts:
    - docs/security/vulnerability-remediation.md
    - docs/security/analysis/README.md
---

# Handled Coordinated-Disclosure Reports

One file per finding received through the channel in [`SECURITY.md`](../../../SECURITY.md)
and handled under the [confidential vulnerability-remediation process](../../vulnerability-remediation.md).

## Purpose

The CVE catalogs (`../production/`, `../build/`) record *external* vulnerabilities we have
evaluated. This catalog records *reports about our own code* we have processed, so that:

1. a second report of the same finding (from anyone) is recognised immediately and answered
   with a link instead of a new triage;
2. the maintainer who later touches the fixed code can find out **why** it looks the way it
   does (the code carries a one-line pointer here);
3. the project has an auditable, sanitized public history of what was reported, how it was
   classified, and what was done — including findings we **declined** to change.

## Rules

- **Create the record at the disclosure moment**, never before. Before disclosure the case
  lives only in the private case record and on the unpushed branch. The record is committed
  in the same PR as the fix (or, for declined findings, in its own docs PR).
- **Sanitized.** No exploit code, no raw evidence, no reporter contact details. Credit
  uses only the name/handle the reporter approved for public use.
- **Every status gets a file**, including `declined` and `non-affecting`. A declined finding
  is the one most likely to be re-reported.
- Filename: `YYYY-MM-DD_<short-slug>.md` (date = disclosure date).
- **Add a code pointer** at the fixed location when the fix is non-obvious or looks like
  something a future refactor would "simplify" away:

  ```rust
  // Constant-time by contract; see docs/security/analysis/reports/<file>.md
  ```

## Template

Copy [`docs/templates/SECURITY-REPORT.md`](../../../templates/SECURITY-REPORT.md) to
`YYYY-MM-DD_<short-slug>.md` in this directory and fill every frontmatter field.

## Index

Add a row when the handled-report record is created at disclosure time.

| Date | Report | Status | Severity |
| ---- | ------ | ------ | -------- |
