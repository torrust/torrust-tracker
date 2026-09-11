---
semantic-links:
  related-artifacts:
    - docs/issues/open/2185-2003-triage-advisory-external-link-check-findings/ISSUE.md
    - docs/issues/open/2185-2003-triage-advisory-external-link-check-findings/external-link-baseline.md
    - .github/lychee-online.toml
    - .github/workflows/external-link-check.yaml
---

# Agent Review Reports - Issue #2185

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-09-11 08:32 UTC - Task Reviewer

- Invocation scope: Read-only review of the hosted C1/C9 verification evidence in `ISSUE.md` and `external-link-baseline.md`.
- Inputs: GitHub Actions runs [34577938048](https://github.com/torrust/torrust-tracker/actions/runs/34577938048) and [34578523069](https://github.com/torrust/torrust-tracker/actions/runs/34578523069), the downloaded replacement artifact, `.github/lychee-online.toml`, and `.github/workflows/external-link-check.yaml`.
- Evidence: Replacement run 34578523069 completed with a visible Lychee failure on `427b0c93b01f1264f7ef824097ac5669d899486c`; its `Upload Lychee Report` step succeeded. The unexpired 1,743-byte `lychee-external-link-report` artifact expires on 2026-09-25 and records 1,816 total checks, 1,288 successes, 26 redirects, 476 exclusions, 52 errors, and no timeouts. It contains no exact `#discussion_r` pull-request review-comment URLs, while unrelated `404`, `403`, loopback, GitHub issue-comment/pull-request-review fragments, other missing fragments, and a `429` remain visible.
- Findings:
  - Resolved: The exact online-only C1/C9 exclusion is sufficiently narrow and its hosted boundary behavior is verified.
  - Correction required: Cancelled run 34577938048 retained an empty, unusable report artifact; it did not provide usable Lychee output but should not be described as having no artifact.
  - Pending: Keep issue #2185 open. T5, AC2, AC6, remaining quality/completion checkpoints, and C2-C8 triage remain incomplete.
- Verdict: REVIEW PASSED after the cancelled-run artifact wording is corrected.
- Follow-up actions:
  - Record the cancelled-run correction in `ISSUE.md`.
  - Commit and review the hosted-verification evidence without closing issue #2185.
  - Continue with the next independently reviewable category after the evidence is merged.

### 2026-09-11 09:42 UTC - Task Reviewer

- Invocation scope: Independent review of the uncommitted C2 loopback-exclusion slice for issue #2185 in `.github/lychee-online.toml`, `ISSUE.md`, and `external-link-baseline.md`.
- Inputs: Uncommitted diff, `.github/workflows/external-link-check.yaml`, `lychee.toml`, installed Lychee `0.24.2` implementation, and an explicit three-link online-config boundary check.
- Evidence: `.github/lychee-online.toml` alone adds `exclude_loopback = true`; the unchanged `lychee.toml` remains offline and has no loopback exclusion. The online workflow continues to use the online config. Lychee `0.24.2` maps this option exclusively to loopback filtering: IPv4 `127.0.0.0/8`, IPv6 `::1/128`, and the `localhost` hostname; private and link-local filtering are separate disabled options. The explicit check of `http://127.0.0.1:9/`, `http://localhost:9/`, and `https://www.rust-lang.org/` reported 3 total, 2 excluded, 1 successful, 1 redirect, and 0 errors. The issue and baseline record the same test and correctly state that hosted verification is pending.
- Findings:
  - Resolved: C2 is online-only, narrowly limited to loopback addresses and `localhost`, and does not enable broad private, link-local, or public-address exclusion.
  - Pending: The required hosted rerun has not yet verified C2 removal in the workflow report while retaining unrelated failures and its uploaded artifact. The prior hosted evidence only validates C1/C9.
  - Correction applied: AC3 and AC5 were marked complete despite the new C2 slice awaiting its required hosted evidence; both checkboxes and the acceptance-verification table now correctly show pending status.
  - Scope: The three-file diff contains no unrelated workflow, local-policy, timeout, retry, concurrency, path, or URL-pattern policy changes. `git diff --check` passed.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Manually dispatch the hosted External Link Check with the C2 change, retain the report artifact, and record its URL, revision, counts, C2 absence, remaining unrelated failures, and upload outcome.
  - After that evidence is independently reviewed, restore AC3 and AC5 only if the hosted boundary behavior passes.
