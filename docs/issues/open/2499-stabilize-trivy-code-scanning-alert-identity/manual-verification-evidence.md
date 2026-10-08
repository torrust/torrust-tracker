---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2499-stabilize-trivy-code-scanning-alert-identity/ISSUE.md
last-updated-utc: 2026-10-08 17:49
---

# Manual Verification Evidence

## Purpose

Record the reproduction of the Trivy alert flip and, after the fix, the like-for-like recheck.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-08 17:40-17:49
- Artifact under test: GitHub Code Scanning state of `torrust/torrust-tracker` on
  `refs/heads/develop` at `554c47f67`, produced by `.github/workflows/security-scan.yaml`
  (`aquasecurity/trivy-action@v0.36.0`, `github/codeql-action/upload-sarif@v4.38.3`).
- Operating system / environment: Linux workstation, GitHub CLI authenticated with read access.
- Prerequisites and setup performed: none.

## Verification Processes

### V1 - Reproduce the alert flip

- Goal: show that a push-triggered scan marks unchanged findings `fixed` and reopens duplicates.
- Initial state: the scheduled run 37780073778 (`285938179`, finished 12:53:52Z) and the push run
  37808635130 (`554c47f67`, finished 16:55:52Z) both uploaded to `develop`.
- Status: `DONE`

#### Steps Performed

1. Listed recent `Security Scan` runs on `develop`:

   ```bash
   gh run list --repo torrust/torrust-tracker --workflow security-scan.yaml --branch develop \
     --limit 3 --json databaseId,event,headSha,createdAt
   ```

2. Read the image reference each trigger scans in `.github/workflows/security-scan.yaml`: the
   `schedule` branch sets `image=torrust/tracker:develop`; every other trigger sets
   `image=torrust-tracker:local`.
3. Exported open and fixed Trivy alerts:

   ```bash
   gh api 'repos/torrust/torrust-tracker/code-scanning/alerts?state=open&per_page=100' --paginate
   gh api 'repos/torrust/torrust-tracker/code-scanning/alerts?state=fixed&tool_name=Trivy&per_page=100' --paginate
   ```

4. Paired the open alerts with the alerts at path `torrust/tracker` fixed at 16:55:43Z, keyed by
   rule id and the `Package:` value in the alert message.

#### Observed Result

```text
2026-10-08T16:25:14Z  push      554c47f67  37808635130
2026-10-08T12:52:57Z  schedule  285938179  37780073778
2026-10-07T12:43:44Z  schedule  24bf4746a  37623098848

open Trivy alerts at library/torrust-tracker: 31
Trivy alerts at torrust/tracker fixed at 2026-10-08T16:55:43Z: 31
same set of (CVE id, package) keys: True

alert 13 {"state":"fixed","fixed_at":"2026-10-08T16:55:43Z","path":"torrust/tracker","rule":"CVE-2026-5435"}
alert 12 {"state":"open","updated_at":"2026-10-08T16:55:43Z","path":"library/torrust-tracker","rule":"CVE-2026-27171"}
alert 24 {"state":"fixed","fixed_at":"2026-10-08T16:55:43Z","path":"torrust/tracker","rule":"CVE-2026-27171"}
```

The full pairing (scheduled-scan number, push-scan number) is recorded in the
[2026-10-08 review inventory](../../../security/analysis/github-security/reviews/2026-10-08.md).

#### Conclusion

Reproduced. The push run marked 31 findings `fixed` and reopened 31 duplicates under other
numbers. The scanned packages did not change.

### V2 - Recheck after the fix

- Goal: confirm one stable alert per Trivy finding across both trigger types.
- Initial state: fix merged to `develop`.
- Status: `TODO`

## Failures and Follow-up

None recorded.
