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

### 2026-09-11 15:50 UTC - Task Reviewer

- Invocation scope: Read-only review of C2 hosted-verification evidence in `ISSUE.md` and `external-link-baseline.md`.
- Inputs: GitHub Actions [run 34616458439](https://github.com/torrust/torrust-tracker/actions/runs/34616458439), its downloaded report artifact, and the merged online configuration.
- Evidence: The run completed with a visible Lychee failure on `f6df96bf1ad59812db47457b84dcd5926c267c61`; `Upload Lychee Report` succeeded. The unexpired `lychee-external-link-report` artifact is 1,533 bytes and expires on 2026-09-25. Its report records 1,857 total checks, 1,318 successes, 25 redirects, 495 exclusions, 44 errors, and no timeouts. No `localhost`, `127.0.0.1`, or exact `#discussion_r` pull-request review-comment URL remained, while unrelated `404`, `403`, FSF transport, issue-comment, pull-request-review, and other missing-fragment failures remained visible.
- Findings:
  - Resolved: C2 loopback filtering is hosted-verified and remains narrow.
  - Resolved: T3, AC3, AC5, and M4 can be complete for the C1/C9+C2 exclusion slices.
  - Pending: Keep issue #2185 open for C3-C8, AC2/AC6, final quality/manual evidence, acceptance re-review, and completion review.
- Verdict: REVIEW PASSED.
- Follow-up actions:
  - Commit and review the C2 hosted-verification evidence without closing issue #2185.
  - Continue with the next independently reviewable category after the evidence is merged.

### 2026-09-11 09:42 UTC - Task Reviewer

- Invocation scope: Independent review of the uncommitted C2 loopback-exclusion slice for issue #2185 in `.github/lychee-online.toml`, `ISSUE.md`, and `external-link-baseline.md`.
- Inputs: Uncommitted diff, `.github/workflows/external-link-check.yaml`, `lychee.toml`, installed Lychee `0.24.2` implementation, and an explicit three-link online-config boundary check.
- Evidence: `.github/lychee-online.toml` alone adds `exclude_loopback = true`; the unchanged `lychee.toml` remains offline and has no loopback exclusion. The online workflow continues to use the online config. Lychee `0.24.2` maps this option exclusively to loopback filtering: IPv4 `127.0.0.0/8`, IPv6 `::1/128`, and the `localhost` hostname; private and link-local filtering are separate disabled options. The explicit three-link check excluded `http://127.0.0.1:9/` and `http://localhost:9/`; it checked `https://www.rust-lang.org/` successfully after one redirect. It therefore reported 3 total links, 2 exclusions, 1 success, 1 redirect for that successful public URL, and 0 errors. The issue and baseline record the same test and correctly state that hosted verification is pending.
- Findings:
  - Resolved: C2 is online-only, narrowly limited to loopback addresses and `localhost`, and does not enable broad private, link-local, or public-address exclusion.
  - Pending: The required hosted rerun has not yet verified C2 removal in the workflow report while retaining unrelated failures and its uploaded artifact. The prior hosted evidence only validates C1/C9.
  - Correction applied: AC3 and AC5 were marked complete despite the new C2 slice awaiting its required hosted evidence; both checkboxes and the acceptance-verification table now correctly show pending status.
  - Scope: The three-file diff contains no unrelated workflow, local-policy, timeout, retry, concurrency, path, or URL-pattern policy changes. `git diff --check` passed.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Manually dispatch the hosted External Link Check with the C2 change, retain the report artifact, and record its URL, revision, counts, C2 absence, remaining unrelated failures, and upload outcome.
  - After that evidence is independently reviewed, restore AC3 and AC5 only if the hosted boundary behavior passes.

### 2026-09-11 16:45 UTC - Task Reviewer

- Invocation scope: Read-only review of the uncommitted C3 docs.rs repair slice for issue #2185: all changed package `README.md` files, `ISSUE.md`, and `external-link-baseline.md`.
- Inputs: Uncommitted diff and status; the C3 inventory in `external-link-baseline.md`; root and package Cargo manifests; resolved `cargo metadata --no-deps --format-version 1`; and live HTTP header checks for the replacement and all replaced docs.rs URLs.
- Evidence: Exactly 14 package README files changed, exactly matching the 14 C3 inventory entries, with no missing or extra paths. Each changes only its Crate documentation URL: the prior individual docs.rs target returns HTTP 404, and the replacement is `https://docs.rs/crate/torrust-tracker/`. Every affected package declares `documentation.workspace = true`; resolved Cargo metadata gives all 14 the inherited `https://docs.rs/crate/torrust-tracker/` value from `[workspace.package]`. The replacement returns HTTP 200 after its expected redirect to `/crate/torrust-tracker/latest`. The five non-C3 package documentation links remain unchanged. `git diff --check` passes.
- Findings:
  - Resolved: The C3 repair is precisely scoped and each replacement target agrees with its package's resolved Cargo documentation metadata.
  - Pending: Hosted External Link Check verification is correctly recorded as pending. Do not mark T2, AC2, M3, or a C3 hosted-verification result complete until a merged hosted run and retained report prove the 14 stale URLs no longer appear while unrelated failures remain visible.
- Verdict: REVIEW WARNED.
- Follow-up actions:
  - Merge the C3 slice, manually dispatch the advisory workflow, retain its report artifact, and record the run URL, revision, counts, absence of all 14 replaced URLs, remaining failures, and upload outcome.
  - Independently review that hosted evidence before completing C3-related issue criteria.
