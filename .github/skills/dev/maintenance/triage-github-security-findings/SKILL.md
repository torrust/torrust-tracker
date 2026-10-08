---
name: triage-github-security-findings
description: "Review and cluster public GitHub Security tab findings from Code Scanning and Code Quality, classify affecting and non-affecting results, maintain the finding inventory, and create focused issue specs by root cause. Use when reviewing security alerts, CodeQL alerts, GitHub code scanning, code quality findings, workflow permission findings, or recurring scanner triage."
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/security/public-scanner-findings.md
      - docs/security/analysis/README.md
      - docs/security/README.md
      - .github/skills/dev/maintenance/catalog-security-vulnerabilities/SKILL.md
---

# Triage GitHub Security Findings

Use this skill for already-public findings in the repository's GitHub **Code Scanning** and
**Code Quality** views. The authoritative policy and classification rules are in
[`docs/security/public-scanner-findings.md`](../../../../../docs/security/public-scanner-findings.md).

Do not use this workflow for confidential or embargoed reports. Follow
[`docs/security/vulnerability-remediation.md`](../../../../../docs/security/vulnerability-remediation.md)
instead.

## Workflow

1. **Capture the current findings.**
   - Record the review date, source view, rule identifier, severity, affected path, and alert URL
     or stable alert identifier.
   - Preserve enough source metadata to compare the next review without copying sensitive data.
   - Identify a Trivy finding by CVE id and package, not by alert number. Until issue #2499 is
     fixed, each Trivy finding has a scheduled-scan and a push-scan alert number, and only one is
     open at a time; record both.
2. **Check existing records.**
   - Search the public scanner inventory, security analysis catalog, handled-report catalog, and
     open or closed issue specs for the rule, weakness class, path, and root cause.
   - Reuse an existing disposition when its recheck conditions still hold.
3. **Cluster before classifying.**
   - Group findings by root cause, rule, affected subsystem, and shared remediation.
   - Do not assume that identical rule identifiers imply one implementation issue.
   - Split a cluster when fixes have different owners, verification strategies, or independently
     substantial effort.
4. **Classify every cluster.**
   - `affecting`: confirmed problem requiring remediation.
   - `hardening`: valid defensive improvement without a confirmed exploitable path.
   - `non-affecting`: false positive or inapplicable in the current project context.
   - `needs-investigation`: insufficient evidence for a final disposition.
   Record evidence and recheck conditions for every non-affecting or accepted result.
5. **Assign the security surface and priority.**
   - Record `runtime`, `release-supply-chain`, `ci-build`, or `development-test`.
   - Treat scanner severity as evidence, not repository priority.
   - Put shipped runtime code, runtime dependencies, production images, and release publishing in
     the highest trust tier.
   - Escalate CI or development findings when they can reach release artifacts, publishing
     credentials, maintainer credentials, protected branches, or privileged shared runners.
   - Follow the priority matrix in `docs/security/README.md`.
6. **Create focused issue specifications.**
   - Use the `create-issue` skill and the normal spec-first workflow.
   - Create one issue per root cause or independently deliverable remediation unit, not one issue
     per alert and not one umbrella issue for the whole Security tab.
   - Apply `Security` and the appropriate issue-type and priority labels. Security findings are
     not automatically `p1`; priority follows demonstrated impact and urgency.
7. **Update the inventory.**
   - Link every source alert to its cluster and disposition.
   - Link affecting or hardening clusters to their issue specs.
   - Record the review date, reviewer, evidence, and explicit recheck trigger.
8. **Verify closure.**
   - After fixes merge, rerun or wait for the owning scanner and confirm the relevant alerts close.
   - Do not close the issue solely because code changed.
9. **Schedule the next review.**
   - Follow the event and release-cycle cadence in the authoritative process document.

## Issue Granularity Decision

Keep findings together only when all of these are true:

- they have the same root cause;
- one coherent change can remediate them;
- one verification strategy proves all occurrences are fixed; and
- the resulting issue remains small enough to complete and review promptly.

Otherwise split by subsystem, owner, remediation strategy, or independently deliverable batch.

## Priority Guardrails

- Do not assign `p1` merely because GitHub lists a finding in the Security tab.
- Runtime and release/CD findings can be `p1` because they can reach downstream production users.
- CI-only findings normally start at `p2`; escalate when a credible pivot reaches release
  publishing, secrets, or privileged shared infrastructure.
- Local development and test findings normally receive `p2` or `p3`; escalate when they can steal
  maintainer credentials or poison committed or published output.
- Non-affecting findings receive no remediation priority; retain their evidence and recheck
  triggers in the inventory.

## Security Workflow Routing

| Finding source | Workflow |
| --- | --- |
| GitHub Code Scanning (any tool, including Trivy) or Code Quality: inventory and clustering | This skill |
| CVE reachability analysis, including for a Trivy alert in Code Scanning | `catalog-security-vulnerabilities` |
| Container, dependency, or CVE warning from another source | `catalog-security-vulnerabilities` |
| Manual tracker runtime image scan | `run-manual-docker-security-scan` |
| Confidential or embargoed report | `docs/security/vulnerability-remediation.md` |

There is intentionally no single catch-all security-remediation skill. These workflows have
different confidentiality boundaries, evidence requirements, and publication rules.

## Skill Links

Review this skill whenever any of these artifacts changes:

- [`docs/security/public-scanner-findings.md`](../../../../../docs/security/public-scanner-findings.md)
- [`docs/security/analysis/README.md`](../../../../../docs/security/analysis/README.md)
- [`docs/security/README.md`](../../../../../docs/security/README.md)
- [`catalog-security-vulnerabilities`](../catalog-security-vulnerabilities/SKILL.md)
