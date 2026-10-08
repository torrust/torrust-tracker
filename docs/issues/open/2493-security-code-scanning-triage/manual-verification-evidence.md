---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2493-security-code-scanning-triage/ISSUE.md
last-updated-utc: 2026-10-08 18:54
---

<!-- cspell:ignore findall setdefault splitlines startswith -->

# Manual Verification Evidence

## Purpose

Record the verification of the GitHub Security triage process and its first review: workflow
routing (M1), coverage of every open finding (M2), and issue granularity (M3).

## Environment and Prerequisites

- Date and time (UTC): 2026-10-08 17:58-18:00 for the exports and the first reconciliation run;
  18:01:43-18:02:56 for the second reconciliation run (see V2). The first export,
  used to draft the review inventory, ran earlier on 2026-10-08, before 16:55:43Z; its raw output
  was not retained (see [Failures and Follow-up](#failures-and-follow-up)).
- Artifact under test: the `docs(security): define scanner finding triage`,
  `docs(security): catalog GitHub findings`, and `docs(security): record both Trivy alert series`
  changes on the PR #2498 branch, plus the live GitHub Security state of `torrust/torrust-tracker`.
- Operating system / environment: Linux workstation, `gh version 2.76.2`, Python 3.
- Prerequisites and setup performed: GitHub CLI authenticated with read access to the repository's
  security alerts.

## Verification Processes

### V1 - Security Workflow Routing

- Goal: confirm that each security finding source has one owning workflow.
- Initial state: `triage-github-security-findings` and the updated
  `catalog-security-vulnerabilities` skill on the PR branch.
- Status: `DONE`

#### Steps Performed

1. Read the routing table in `.github/skills/dev/maintenance/triage-github-security-findings/SKILL.md`.
2. Read the scope and skill-architecture sections of `docs/security/public-scanner-findings.md`
   and the routing paragraph of `catalog-security-vulnerabilities/SKILL.md`.
3. Compared the three statements for each source.

#### Observed Result

| Source | Workflow |
| --- | --- |
| GitHub Code Scanning (including Trivy) and Code Quality: inventory and clustering | `triage-github-security-findings` |
| CVE reachability analysis, including for Trivy alerts | `catalog-security-vulnerabilities` |
| Manual production image scan | `run-manual-docker-security-scan` |
| Confidential or embargoed report | `docs/security/vulnerability-remediation.md` |

#### Conclusion

Met. Each step has one owner. A Trivy alert is inventoried and clustered by the triage skill, and
its CVE analysis is recorded by the CVE catalog skill. No confidential workflow creates public
issues or catalog entries.

### V2 - Initial Finding Coverage

- Goal: confirm that every open finding maps to exactly one cluster in the review inventory.
- Initial state: review inventory `docs/security/analysis/github-security/reviews/2026-10-08.md`.
- Status: `DONE`

#### Steps Performed

1. Exported the open Code Scanning alerts and summarized them by tool, rule, and Trivy path:

   ```bash
   gh api --paginate -H 'Accept: application/vnd.github+json' \
     -H 'X-GitHub-Api-Version: 2022-11-28' \
     '/repos/torrust/torrust-tracker/code-scanning/alerts?state=open&per_page=100' > .tmp/v2-cs.json
   jq -r '"open Code Scanning alerts: \(length)",
     (group_by(.tool.name)[] | "  \(.[0].tool.name): \(length)"),
     (map(select(.tool.name=="CodeQL")) | group_by(.rule.id)[] | "    \(.[0].rule.id): \(length)"),
     (map(select(.tool.name=="Trivy")) | group_by(.most_recent_instance.location.path)[]
       | "    Trivy path \(.[0].most_recent_instance.location.path): \(length)")' .tmp/v2-cs.json
   ```

2. Exported the open Code Quality findings:

   ```bash
   gh api --paginate -H 'Accept: application/vnd.github+json' \
     -H 'X-GitHub-Api-Version: 2026-03-10' \
     '/repos/torrust/torrust-tracker/code-quality/findings?state=open&per_page=100' > .tmp/v2-cq.json
   jq -r '"open Code Quality findings: \(length)",
     (.[] | "  \(.number // .id) \(.rule.id // .rule) \(.location.path // .most_recent_instance.location.path)")' \
     .tmp/v2-cq.json
   ```

3. Expanded every alert number in the inventory's Coverage Reconciliation table, using the push-scan
   column for Trivy rows because the push-scan series was open, and compared the result with the
   two exports with `python3 .tmp/reconcile.py push`. The script ran twice against the same two
   exports:

   - **Run 1, 17:58-18:00 UTC**, against the coverage table before it gained the severity and path
     columns. The numbers column was then the fifth `|`-separated field, so the script read
     `row.split('|')[4]`.
   - **Run 2, 18:01:43-18:02:56 UTC**, after `docs(security): record scanner severity and paths`
     widened the table and before that change was committed. The script was changed to read
     `row.split('|')[-3]`, the numbers column counted from the end of the row, which works for
     both table layouts.

   The script as used in run 2, verbatim (run 1 differed only in that index):

   ```python
   import json, re, sys
   review = open('docs/security/analysis/github-security/reviews/2026-10-08.md').read()
   table = review.split('## Coverage Reconciliation')[1].split('\n## ')[0]
   def expand(cell):
       nums = []
       for part in re.findall(r'\d+(?:–\d+)?', cell):
           a, _, b = part.partition('–')
           nums += range(int(a), int(b or a) + 1)
       return nums
   listed = {}
   for row in (l for l in table.splitlines() if l.startswith('| GSF-')):
       cluster, source, numbers = row.split('|')[1].strip(), row.split('|')[2].strip(), row.split('|')[-3]
       if source == 'Trivy':
           scheduled, push = numbers.split(';')
           numbers = push if sys.argv[1] == 'push' else scheduled
       for n in expand(numbers):
           listed.setdefault(n, []).append(cluster)
   cs = json.load(open('.tmp/v2-cs.json')); cq = json.load(open('.tmp/v2-cq.json'))
   source = sorted([a['number'] for a in cs] + [f['number'] for f in cq])
   print('source findings:', len(source))
   print('inventory numbers:', len(listed))
   print('missing from inventory:', sorted(set(source) - set(listed)))
   print('not in source:', sorted(set(listed) - set(source)))
   print('in more than one cluster:', sorted(n for n, c in listed.items() if len(c) > 1))
   ```

4. Counted the disposition totals by hand from the catalog table.

#### Observed Result

The export summaries, then the reconciliation output. Runs 1 and 2 printed the same five lines.

```text
open Code Scanning alerts: 66
  CodeQL: 35
  Trivy: 31
    actions/missing-workflow-permissions: 27
    rust/cleartext-logging: 4
    rust/hard-coded-cryptographic-value: 4
    Trivy path library/torrust-tracker: 31
open Code Quality findings: 2
  316 py/file-not-closed contrib/dev-tools/git/github-merge.py
  314 py/file-not-closed docs/issues/closed/2022-vendor-and-document-maintainer-merge-workflow/github-merge.py

source findings: 68
inventory numbers: 68
missing from inventory: []
not in source: []
in more than one cluster: []
```

| Disposition | Count |
| --- | ---: |
| Hardening | 27 |
| Needs investigation | 2 |
| Non-affecting | 39 |
| Confirmed affecting | 0 |
| **Total** | **68** |

#### Conclusion

Met. All 68 open findings map to exactly one of the 11 clusters. The scheduled-scan Trivy series,
which the first export returned, reconciles the same way; its pairing with the push-scan series is
recorded in the review inventory.

### V3 - Issue Granularity

- Goal: confirm that actionable findings produced focused, independently deliverable issues.
- Initial state: the catalog's actionable clusters GSF-001 and GSF-006.
- Status: `DONE`

#### Steps Performed

1. Read the follow-up issues on GitHub:

   ```bash
   for n in 2495 2496 2497 2499; do
     gh issue view $n --repo torrust/torrust-tracker --json number,title,state,labels \
       --jq '"#\(.number) \(.state) [\([.labels[].name]|join(", "))] \(.title)"'
   done
   ```

2. Compared each issue's alert list with the inventory's workflow-permission split and GSF-006.

#### Observed Result

```text
#2495 OPEN [Continuous Integration, Security, task] Set explicit permissions for CI workflows
#2496 OPEN [Continuous Integration, Security, task] Set explicit permissions for release publishing workflows
#2497 OPEN [Security, Needs Research, task] Investigate native zlib runtime linkage and CVE reachability
#2499 OPEN [Bug, Continuous Integration, Security] Stabilize Trivy Code Scanning alert identity
```

- #2497 covers alerts 24 and 59 (push-scan series: 12 and 60).
- #2496 covers the seven release-publishing alerts.
- #2495 covers the twenty CI/build alerts.
- #2499 came from PR #2498 review, not from a cluster: it fixes the alert-identity defect behind
  the two Trivy series.

#### Conclusion

Met. The two workflow issues are split because release publishing and CI validation have different
trust boundaries, owners, verification paths, and consequences. The zlib investigation needs
runtime linkage and symbol evidence rather than workflow changes. The 39 non-affecting findings
remain in the catalog with recheck triggers and create no remediation issues.

## Failures and Follow-up

The first export's raw output was not recorded, and its Trivy numbers became stale when the push
run for `554c47f67` reopened the other alert series at 16:55:43Z. PR #2498 review finding
`review-finding:pr-2498-f10` found the mismatch. The review inventory now records both series, V2
was rerun with recorded output, and issue #2499 tracks the workflow defect.
