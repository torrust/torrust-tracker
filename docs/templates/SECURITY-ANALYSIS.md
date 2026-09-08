---
semantic-links:
  skill-links:
    - create-markdown-template
    - catalog-security-vulnerabilities
  related-artifacts:
    - docs/security/analysis/README.md
    - .github/skills/dev/maintenance/catalog-security-vulnerabilities/SKILL.md
    - .github/skills/dev/maintenance/run-manual-docker-security-scan/SKILL.md
---

# {Finding Identifier} — {Affected Component}

> Create a concrete analysis in `docs/security/analysis/production/`,
> `docs/security/analysis/build/`, or `docs/security/analysis/affecting/` as appropriate. This
> public template is only for scanner findings and vulnerabilities approved for public disclosure.
> Do not use it for embargoed or coordinated-disclosure reports.

## Frontmatter

```yaml
---
cve-id: { CVE-YYYY-NNNN or other public identifier }
date-analyzed: YYYY-MM-DD
source: { scanner or public advisory }
status: { non-affecting|affecting|needs-investigation }
review-cadence: quarterly
requires-recheck-when: { condition that can invalidate this verdict }
semantic-links:
  related-artifacts:
    - { affected repository artifact }
---
```

Use `cve-id` for CVEs and other public identifiers to preserve the existing catalog convention.
Add scanner-specific metadata, such as an image digest, only when it is material to reproducibility.

## Vulnerability or Finding

Describe the public finding, affected package/component, severity, and a public reference.

- **Severity**: {severity}
- **Package or component**: {name and version, if known}
- **Link**: {public advisory URL}

## Context

State whether the affected component is in the production runtime, a build stage, or another
repository context. Identify the relevant image, stage, source path, or dependency boundary.

## Impact Assessment

Explain whether and how the finding is reachable or exploitable in the tracker deployment context.
Use evidence from repository code, configuration, container stages, dependency metadata, or a public
advisory. Do not make a non-affecting conclusion from the finding's severity alone.

## Verdict Rationale

### Why It Does Not Affect Us

For a `non-affecting` verdict, explain the concrete deployment or code boundary that prevents impact.

### Why It Affects Us

For an `affecting` verdict, describe impact, affected components, exploitability context, and the
required remediation path. Confirm public-disclosure status before creating this public record.

## Conditions That Would Change This Verdict

- {Code, configuration, dependency, image, or deployment change requiring re-analysis}

## Future Actions

| Action   | Cadence or Trigger   | Owner           |
| -------- | -------------------- | --------------- |
| {Action} | {When it must occur} | {Owner or team} |

## Evidence

- {Repository path, scanner command/version, public advisory, or reviewed artifact}

## References

- Security analysis process: `docs/security/analysis/README.md`
- {Related issue, PR, ADR, or public advisory}
