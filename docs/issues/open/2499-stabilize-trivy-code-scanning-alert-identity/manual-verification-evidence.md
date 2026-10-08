---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2499-stabilize-trivy-code-scanning-alert-identity/ISSUE.md
last-updated-utc: 2026-10-08 18:58
---

<!-- cspell:ignore tojson tostring -->

# Manual Verification Evidence

## Purpose

Record the reproduction of the Trivy alert flip and, after the fix, the like-for-like recheck.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-08 18:57:14-18:58:10 for the commands and output recorded in V1;
  step 1 ran last, at 18:58:10, with its output joined by spaces instead of tabs.
  The first reproduction, at 17:40-17:49, used ad-hoc commands that were not recorded exactly; this
  rerun records them. The Code Scanning state was unchanged between the two runs.
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

1. Listed the latest `Security Scan` runs on `develop`:

   ```bash
   gh run list --repo torrust/torrust-tracker --workflow security-scan.yaml --branch develop --limit 3 \
     --json databaseId,event,headSha,createdAt \
     --jq '.[] | [.createdAt, .event, .headSha[0:9], (.databaseId | tostring)] | join("  ")'
   ```

2. Read the image reference each trigger scans in `.github/workflows/security-scan.yaml`: the
   `schedule` branch sets `image=torrust/tracker:develop`; every other trigger sets
   `image=torrust-tracker:local`.
3. Exported the open and the fixed Trivy alerts:

   ```bash
   gh api --paginate -H 'X-GitHub-Api-Version: 2022-11-28' \
     'repos/torrust/torrust-tracker/code-scanning/alerts?state=open&tool_name=Trivy&per_page=100' > .tmp/trivy-open.json
   gh api --paginate -H 'X-GitHub-Api-Version: 2022-11-28' \
     'repos/torrust/torrust-tracker/code-scanning/alerts?state=fixed&tool_name=Trivy&per_page=100' > .tmp/trivy-fixed.json
   ```

4. Paired the open alerts with the alerts at path `torrust/tracker` fixed at 16:55:43Z, keyed by
   rule id and the `Package:` value in the alert message, with `python3 .tmp/pair-trivy.py`. The
   one-off script, verbatim:

   ```python
   import json, re

   def key(alert):
       package = re.search(r'Package: (\S+)', alert['most_recent_instance']['message']['text']).group(1)
       return alert['rule']['id'], package

   open_alerts = json.load(open('.tmp/trivy-open.json'))
   fixed = [a for a in json.load(open('.tmp/trivy-fixed.json'))
            if a['most_recent_instance']['location']['path'] == 'torrust/tracker'
            and a['fixed_at'] == '2026-10-08T16:55:43Z']
   open_keys, fixed_keys = {key(a) for a in open_alerts}, {key(a) for a in fixed}
   print('open Trivy alerts by path:', sorted({a['most_recent_instance']['location']['path'] for a in open_alerts}), len(open_alerts))
   print('Trivy alerts at torrust/tracker fixed at 2026-10-08T16:55:43Z:', len(fixed))
   print('distinct (CVE id, package) keys, open / fixed:', len(open_keys), '/', len(fixed_keys))
   print('same set of (CVE id, package) keys:', open_keys == fixed_keys)
   ```

5. Read one alert of each series, and the zlib alert of each series:

   ```bash
   for n in 13 12 24; do
     gh api -H 'X-GitHub-Api-Version: 2022-11-28' "repos/torrust/torrust-tracker/code-scanning/alerts/$n" \
       --jq '"alert \(.number) " + ({state, fixed_at, path: .most_recent_instance.location.path, rule: .rule.id} | tojson)'
   done
   ```

#### Observed Result

```text
2026-10-08T16:25:14Z  push  554c47f67  37808635130
2026-10-08T12:52:57Z  schedule  285938179  37780073778
2026-10-07T12:43:44Z  schedule  24bf4746a  37623098848
open Trivy alerts by path: ['library/torrust-tracker'] 31
Trivy alerts at torrust/tracker fixed at 2026-10-08T16:55:43Z: 31
distinct (CVE id, package) keys, open / fixed: 31 / 31
same set of (CVE id, package) keys: True
alert 13 {"fixed_at":"2026-10-08T16:55:43Z","path":"torrust/tracker","rule":"CVE-2026-5435","state":"fixed"}
alert 12 {"fixed_at":null,"path":"library/torrust-tracker","rule":"CVE-2026-27171","state":"open"}
alert 24 {"fixed_at":"2026-10-08T16:55:43Z","path":"torrust/tracker","rule":"CVE-2026-27171","state":"fixed"}
```

The full pairing (scheduled-scan number, push-scan number) is recorded in the
[2026-10-08 review inventory](../../../security/analysis/github-security/reviews/2026-10-08.md).

#### Conclusion

Reproduced. The push run marked 31 findings `fixed` and reopened 31 duplicates under other
numbers. Each series holds 31 distinct CVE-and-package keys, and the two sets are equal, so the
pairing is one-to-one. The scanned packages did not change.

### V2 - Recheck after the fix

- Goal: confirm one stable alert per Trivy finding across both trigger types.
- Initial state: fix merged to `develop`.
- Status: `TODO`

## Failures and Follow-up

None recorded.
